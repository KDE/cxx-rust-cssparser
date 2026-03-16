# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_add_cxxbridge(cxx-rust-cssparser-impl-bridge
    CRATE cxx_rust_cssparser_impl
    FILES ffi.rs
)

# Ensure we build the bridge target with PIC otherwise linking fails
set_target_properties(cxx-rust-cssparser-impl-bridge PROPERTIES POSITION_INDEPENDENT_CODE TRUE)

# Make the generated header file available at a more easily accessed location.
# Use a proper OUTPUT-based custom command instead of POST_BUILD to ensure
# Ninja can track the dependency correctly and avoid race conditions where
# dependent targets try to compile before ffi.h is copied.
set(_ffi_h_generated "${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated/cxxbridge/cxx-rust-cssparser-impl-bridge/include/cxx-rust-cssparser-impl-bridge/ffi.h")
set(_ffi_h_copied "${CMAKE_CURRENT_BINARY_DIR}/ffi.h")

add_custom_command(
    OUTPUT "${_ffi_h_copied}"
    COMMAND ${CMAKE_COMMAND} -E copy "${_ffi_h_generated}" "${_ffi_h_copied}"
    DEPENDS "${_ffi_h_generated}"
    COMMENT "Copying ffi.h to ${_ffi_h_copied}"
)

add_custom_target(cxx-rust-cssparser-copy-ffi-h DEPENDS "${_ffi_h_copied}")
add_dependencies(cxx-rust-cssparser-copy-ffi-h cxx-rust-cssparser-impl-bridge)

corrosion_install(TARGETS cxx_rust_cssparser_impl EXPORT cxx-rust-cssparserTargets)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/ffi.h DESTINATION ${KDE_INSTALL_INCLUDEDIR}/cxx-rust-cssparser)

if (BUILD_TESTING)
    add_test(
        NAME cargo-test
        COMMAND $<TARGET_FILE:Rust::Cargo> test --manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()
