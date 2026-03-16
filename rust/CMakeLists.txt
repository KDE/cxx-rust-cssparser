# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_add_cxxbridge(cxx-rust-cssparser-impl-bridge
    CRATE cxx_rust_cssparser_impl
    FILES ffi.rs
)

# Ensure we build the bridge target with PIC otherwise linking fails
set_target_properties(cxx-rust-cssparser-impl-bridge PROPERTIES POSITION_INDEPENDENT_CODE TRUE)

if (BUILD_TESTING)
    add_test(
        NAME cargo-test
        COMMAND $<TARGET_FILE:Rust::Cargo> test --manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()
