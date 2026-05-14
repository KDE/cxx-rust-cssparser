# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

if (USE_CORROSION_CXXBRIDGE)
    corrosion_add_cxxbridge(cxx-rust-cssparser-impl-bridge
        CRATE cxx_rust_cssparser_impl
        FILES ffi.rs
    )

    set_target_properties(cxx-rust-cssparser-impl-bridge PROPERTIES POSITION_INDEPENDENT_CODE TRUE)
else()
    # Flatpak: use cxx_build output directly instead of corrosion_add_cxxbridge.
    string(SHA1 _cargo_path_hash ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)
    string(SUBSTRING "${_cargo_path_hash}" 0 5 _cargo_path_hash)
    set(_cxxbridge_dir ${CMAKE_BINARY_DIR}/cargo/rust_${_cargo_path_hash}/${Rust_CARGO_TARGET}/cxxbridge)

    # Forward the cxx_build header to the path corrosion_add_cxxbridge would use.
    set(_bridge_include_dir ${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated/cxxbridge)
    file(MAKE_DIRECTORY ${_bridge_include_dir}/cxx-rust-cssparser-impl-bridge)
    file(WRITE ${_bridge_include_dir}/cxx-rust-cssparser-impl-bridge/ffi.h
        "#pragma once\n#include \"${_cxxbridge_dir}/cxx_rust_cssparser_impl/src/ffi.rs.h\"\n"
    )

    target_include_directories(cxx_rust_cssparser_impl INTERFACE ${_bridge_include_dir})
endif()

if (BUILD_TESTING)
    add_test(
        NAME cargo-test
        COMMAND $<TARGET_FILE:Rust::Cargo> test --manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()
