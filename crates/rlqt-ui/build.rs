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

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn npm_command() -> Command {
    if cfg!(windows) {
        // On Windows, npm is a script
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "npm"]);
        cmd
    } else {
        Command::new("npm")
    }
}

fn watch_directory(dir: &Path) {
    if !dir.exists() {
        return;
    }

    println!("cargo:rerun-if-changed={}", dir.display());

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());

            if path.is_dir() {
                watch_directory(&path);
            }
        }
    }
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let frontend_dir = Path::new(&manifest_dir).join("frontend");

    watch_directory(&frontend_dir.join("src"));

    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("index.html").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("package.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("package-lock.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("tsconfig.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("vite.config.ts").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("postcss.config.js").display()
    );

    if !frontend_dir.join("node_modules").exists() {
        let status = npm_command()
            .current_dir(&frontend_dir)
            .arg("install")
            .status()
            .expect("Failed to run npm install");

        if !status.success() {
            panic!("npm install failed");
        }
    }

    let status = npm_command()
        .current_dir(&frontend_dir)
        .args(["run", "build"])
        .status()
        .expect("Failed to run npm build");

    if !status.success() {
        panic!("Frontend build failed");
    }
}
