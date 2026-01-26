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
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "npm"]);
        cmd
    } else {
        Command::new("npm")
    }
}

fn build_wasm(manifest_dir: &Path, frontend_dir: &Path) {
    let wasm_crate_dir = manifest_dir.join("../rlqt-ql-wasm");
    let wasm_output_dir = frontend_dir.join("src/wasm/pkg");
    let wasm_file = wasm_output_dir.join("rlqt_ql_wasm_bg.wasm");

    for file in ["Cargo.toml", "src/lib.rs"] {
        println!(
            "cargo:rerun-if-changed={}",
            wasm_crate_dir.join(file).display()
        );
    }

    let ql_core_dir = manifest_dir.join("../rlqt-ql-core/src");
    if ql_core_dir.exists() {
        for entry in fs::read_dir(&ql_core_dir).into_iter().flatten().flatten() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    // Skip WASM build if the output already exists
    // Users need to run `wasm-pack build` manually before `cargo build` to create the initial WASM output,
    // or re-run it after changing rlqt-ql-core or rlqt-ql-wasm sources.
    // Running wasm-pack from within a cargo build script causes lock conflicts.
    if wasm_file.exists() {
        return;
    }

    if !wasm_crate_dir.exists() {
        println!(
            "cargo:warning=WASM crate not found at {:?}, skipping WASM build",
            wasm_crate_dir
        );
        return;
    }

    let wasm_pack = which_wasm_pack();
    if wasm_pack.is_none() {
        println!(
            "cargo:warning=wasm-pack not found, skipping WASM build. Install with: cargo install wasm-pack"
        );
        return;
    }

    // Use a separate target directory to avoid Cargo.lock conflicts
    let wasm_target_dir = manifest_dir.join("../target-wasm");

    // Clear environment variables that interfere with wasm-pack's build process
    // when running from within a cargo build script
    let status = Command::new(wasm_pack.unwrap())
        .args(["build", "--target", "web", "--out-dir"])
        .arg(&wasm_output_dir)
        .arg(&wasm_crate_dir)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_BUILD_RUSTFLAGS")
        .env("CARGO_TARGET_DIR", &wasm_target_dir)
        .status()
        .expect("Failed to run wasm-pack");

    if !status.success() {
        panic!("wasm-pack build failed");
    }
}

fn which_wasm_pack() -> Option<String> {
    let home = env::var("HOME").ok()?;
    let cargo_bin = Path::new(&home).join(".cargo/bin/wasm-pack");
    if cargo_bin.exists() {
        return Some(cargo_bin.to_string_lossy().to_string());
    }

    let output = Command::new("which").arg("wasm-pack").output().ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            return Some(path);
        }
    }

    None
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
    let manifest_path = Path::new(&manifest_dir);
    let frontend_dir = manifest_path.join("frontend");

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

    // Skip frontend build if pre-built assets exist (for crates.io publishing and CI)
    let dist_dir = frontend_dir.join("dist");
    if dist_dir.exists() && dist_dir.join("index.html").exists() {
        // Also check WASM is present
        let wasm_pkg = frontend_dir.join("src/wasm/pkg/rlqt_ql_wasm_bg.wasm");
        if wasm_pkg.exists() {
            return;
        }
    }

    // RLQT_SKIP_FRONTEND_BUILD=1 forces skip even without pre-built assets (CI artifact download)
    if env::var("RLQT_SKIP_FRONTEND_BUILD").is_ok() {
        if dist_dir.exists() && dist_dir.join("index.html").exists() {
            println!("cargo:warning=RLQT_SKIP_FRONTEND_BUILD set, using pre-built frontend");
            return;
        }
        println!("cargo:warning=RLQT_SKIP_FRONTEND_BUILD set but dist/ not found, building anyway");
    }

    build_wasm(manifest_path, &frontend_dir);

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
