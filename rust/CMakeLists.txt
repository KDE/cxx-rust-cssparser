# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

string(SHA1 _cargo_path_hash ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)
string(SUBSTRING "${_cargo_path_hash}" 0 5 _cargo_path_hash)
message(STATUS ${CMAKE_BINARY_DIR}/cargo/rust_${_cargo_path_hash}/${Rust_CARGO_TARGET}/cxxbridge)
target_include_directories(cxx_rust_cssparser_impl INTERFACE
    ${CMAKE_BINARY_DIR}/cargo/rust_${_cargo_path_hash}/${Rust_CARGO_TARGET}/cxxbridge
)

if (BUILD_TESTING)
    add_test(
        NAME cargo-test
        COMMAND $<TARGET_FILE:Rust::Cargo> test --manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()
