// Copyright (c) 2025 Kevin Thomas
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

//! Build script for STM32F303RE embedded project
//!
//! This build script handles the linker memory configuration at compile time by:
//! - Copying the `memory.x` linker script to the build output directory
//! - Adding the output directory to the linker search path
//! - Configuring incremental rebuild behavior
//!
//! # Purpose
//! The `memory.x` file defines the memory layout for the STM32F303RE microcontroller,
//! specifying FLASH and RAM regions that the linker uses to place code and data.
//! This script ensures the linker can find this critical configuration file.
//!
//! # Cargo Integration
//! - Runs automatically before each build
//! - Only re-runs when `memory.x` changes (for build efficiency)
//! - Communicates with Cargo via `cargo:` directives

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Build script entry point
///
/// Performs the following tasks:
/// 1. Copies `memory.x` to the build output directory
/// 2. Instructs Cargo to add the output directory to linker search paths
/// 3. Configures rebuild triggers to only watch `memory.x`
///
/// # Panics
/// Panics if:
/// - `OUT_DIR` environment variable is not set (should never happen in normal builds)
/// - Unable to create or write to `memory.x` in the output directory
fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
