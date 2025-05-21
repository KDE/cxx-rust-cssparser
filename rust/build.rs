// SPDX-License-Identifier: BSD-2-Clause
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

fn main() {
    let _ = cxx_build::bridge("src/ffi.rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
