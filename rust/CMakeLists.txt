# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_add_cxxbridge(cxx-rust-cssparser-impl-bridge
    CRATE cxx_rust_cssparser_impl
    FILES ffi.rs
)

# Ensure we build the bridge target with PIC otherwise linking fails
set_target_properties(cxx-rust-cssparser-impl-bridge PROPERTIES POSITION_INDEPENDENT_CODE TRUE)

# Make the generated header file available at a more easily accessed location
add_custom_command(
    TARGET cxx-rust-cssparser-impl-bridge
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated/cxxbridge/cxx-rust-cssparser-impl-bridge/include/cxx-rust-cssparser-impl-bridge/ffi.h
        ${CMAKE_CURRENT_BINARY_DIR}/ffi.h
    BYPRODUCTS ffi.h
)

corrosion_install(TARGETS cxx_rust_cssparser_impl EXPORT cxx-rust-cssparserTargets)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/ffi.h DESTINATION ${KDE_INSTALL_INCLUDEDIR}/cxx-rust-cssparser)

if (BUILD_TESTING)
    add_test(
        NAME cargo-test
        COMMAND $<TARGET_FILE:Rust::Cargo> test --manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()
