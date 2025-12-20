// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::TempDir;

use crate::errors::CommandRunError;

type Result<T> = std::result::Result<T, CommandRunError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveType {
    PlainLog,
    GzipLog,
    XzLog,
    TarGz,
    TarXz,
}

impl ArchiveType {
    pub fn from_path(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            ArchiveType::TarGz
        } else if file_name.ends_with(".tar.xz") || file_name.ends_with(".txz") {
            ArchiveType::TarXz
        } else if file_name.ends_with(".log.gz") || file_name.ends_with(".gz") {
            ArchiveType::GzipLog
        } else if file_name.ends_with(".log.xz") || file_name.ends_with(".xz") {
            ArchiveType::XzLog
        } else {
            ArchiveType::PlainLog
        }
    }

    pub fn is_tar_archive(&self) -> bool {
        matches!(self, ArchiveType::TarGz | ArchiveType::TarXz)
    }
}

pub enum LogReader {
    Plain(BufReader<File>),
    Gzip(BufReader<GzDecoder<File>>),
    Xz(BufReader<XzReader>),
}

impl Read for LogReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            LogReader::Plain(r) => r.read(buf),
            LogReader::Gzip(r) => r.read(buf),
            LogReader::Xz(r) => r.read(buf),
        }
    }
}

impl BufRead for LogReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            LogReader::Plain(r) => r.fill_buf(),
            LogReader::Gzip(r) => r.fill_buf(),
            LogReader::Xz(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            LogReader::Plain(r) => r.consume(amt),
            LogReader::Gzip(r) => r.consume(amt),
            LogReader::Xz(r) => r.consume(amt),
        }
    }
}

pub struct XzReader {
    decompressed: Vec<u8>,
    position: usize,
}

impl XzReader {
    pub fn new(mut file: File) -> std::io::Result<Self> {
        let mut compressed = Vec::new();
        file.read_to_end(&mut compressed)?;

        let mut decompressed = Vec::new();
        lzma_rs::xz_decompress(&mut compressed.as_slice(), &mut decompressed).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("XZ error: {}", e))
        })?;

        Ok(Self {
            decompressed,
            position: 0,
        })
    }
}

impl Read for XzReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = &self.decompressed[self.position..];
        let to_read = std::cmp::min(buf.len(), remaining.len());
        buf[..to_read].copy_from_slice(&remaining[..to_read]);
        self.position += to_read;
        Ok(to_read)
    }
}

pub fn open_log_reader(path: &Path) -> Result<LogReader> {
    let archive_type = ArchiveType::from_path(path);
    let file = File::open(path).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to open file '{}': {}", path.display(), e),
        )))
    })?;

    match archive_type {
        ArchiveType::PlainLog => Ok(LogReader::Plain(BufReader::new(file))),
        ArchiveType::GzipLog => {
            let decoder = GzDecoder::new(file);
            Ok(LogReader::Gzip(BufReader::new(decoder)))
        }
        ArchiveType::XzLog => {
            let xz_reader = XzReader::new(file).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                    e.kind(),
                    format!("Failed to decompress XZ file '{}': {}", path.display(), e),
                )))
            })?;
            Ok(LogReader::Xz(BufReader::new(xz_reader)))
        }
        ArchiveType::TarGz | ArchiveType::TarXz => Err(CommandRunError::Library(
            rlqt_lib::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Cannot open tar archive as a single file: {}. Use extract_tar_archive instead.",
                    path.display()
                ),
            )),
        )),
    }
}

pub struct ExtractedArchive {
    // Field exists to keep the temp directory alive until dropped
    pub _temp_dir: TempDir,
    pub log_files: Vec<PathBuf>,
}

pub fn extract_tar_archive(path: &Path) -> Result<ExtractedArchive> {
    let archive_type = ArchiveType::from_path(path);
    let file = File::open(path).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to open archive '{}': {}", path.display(), e),
        )))
    })?;

    let temp_dir = TempDir::new().map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to create temp directory: {}", e),
        )))
    })?;

    match archive_type {
        ArchiveType::TarGz => {
            let decoder = GzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.set_preserve_permissions(false);
            archive.set_preserve_mtime(false);
            archive.unpack(temp_dir.path()).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                    e.kind(),
                    format!(
                        "Failed to extract tar.gz archive '{}': {}",
                        path.display(),
                        e
                    ),
                )))
            })?;
        }
        ArchiveType::TarXz => {
            let mut compressed = Vec::new();
            let mut file = file;
            file.read_to_end(&mut compressed).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                    e.kind(),
                    format!("Failed to read archive '{}': {}", path.display(), e),
                )))
            })?;

            let mut decompressed = Vec::new();
            lzma_rs::xz_decompress(&mut compressed.as_slice(), &mut decompressed).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Failed to decompress XZ archive '{}': {}",
                        path.display(),
                        e
                    ),
                )))
            })?;

            let mut archive = Archive::new(decompressed.as_slice());
            archive.set_preserve_permissions(false);
            archive.set_preserve_mtime(false);
            archive.unpack(temp_dir.path()).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                    e.kind(),
                    format!(
                        "Failed to extract tar.xz archive '{}': {}",
                        path.display(),
                        e
                    ),
                )))
            })?;
        }
        _ => {
            return Err(CommandRunError::Library(rlqt_lib::Error::Io(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Not a tar archive: {}", path.display()),
                ),
            )));
        }
    }

    let log_files = find_log_files_in_dir(temp_dir.path())?;

    Ok(ExtractedArchive {
        _temp_dir: temp_dir,
        log_files,
    })
}

fn find_log_files_in_dir(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut log_files = Vec::new();
    collect_log_files_recursive(dir, &mut log_files, 0)?;
    log_files.sort();
    Ok(log_files)
}

fn collect_log_files_recursive(
    dir: &Path,
    log_files: &mut Vec<PathBuf>,
    depth: usize,
) -> Result<()> {
    const MAX_DEPTH: usize = 5;
    if depth > MAX_DEPTH {
        return Ok(());
    }

    let entries = std::fs::read_dir(dir).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read directory '{}': {}", dir.display(), e),
        )))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            CommandRunError::Library(rlqt_lib::Error::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to read directory entry: {}", e),
            )))
        })?;

        let path = entry.path();
        if path.is_dir() {
            collect_log_files_recursive(&path, log_files, depth + 1)?;
        } else if path.is_file() && is_log_file(&path) {
            log_files.push(path);
        }
    }

    Ok(())
}

fn is_log_file(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    file_name.ends_with(".log") || file_name.ends_with(".log.gz") || file_name.ends_with(".log.xz")
}

pub fn strip_compression_suffix(file_name: &str) -> &str {
    let suffixes = [
        ".log.tar.gz",
        ".log.tgz",
        ".log.tar.xz",
        ".log.txz",
        ".log.gz",
        ".log.xz",
        ".tar.gz",
        ".tgz",
        ".tar.xz",
        ".txz",
        ".log",
    ];

    let lower = file_name.to_lowercase();
    for suffix in suffixes {
        if lower.ends_with(suffix) {
            return &file_name[..file_name.len() - suffix.len()];
        }
    }

    file_name
}

pub fn is_supported_log_file(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    file_name.ends_with(".log")
        || file_name.ends_with(".log.gz")
        || file_name.ends_with(".log.xz")
        || file_name.ends_with(".log.tar.gz")
        || file_name.ends_with(".log.tgz")
        || file_name.ends_with(".log.tar.xz")
        || file_name.ends_with(".log.txz")
}
