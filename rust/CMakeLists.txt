if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

set(RUST_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libcxx_rust_cssparser.a")
set(RUST_CXX "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparser-impl.cpp")
set(RUST_HEADER "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparser-impl.h")

add_library(cxx-rust-cssparser-impl STATIC ${RUST_CXX})
add_custom_command(
    OUTPUT ${RUST_LIB} ${RUST_CXX}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/cxx-rust-cssparser/src/ffi.rs.cc ${RUST_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/cxx-rust-cssparser/src/ffi.rs.h ${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparser-impl.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}

    DEPENDS
    build.rs
    Cargo.toml

    src/ffi.rs
    src/lib.rs
    src/property.rs
    src/selector.rs
    src/stylerule.rs
    src/stylesheet.rs
    src/value.rs

    src/details/identifier.rs
    src/details/mod.rs
    src/details/rulesparser.rs
    src/details/selectorparser.rs
)

set_target_properties(cxx-rust-cssparser-impl PROPERTIES POSITION_INDEPENDENT_CODE TRUE)
target_include_directories(cxx-rust-cssparser-impl PUBLIC ${CMAKE_CURRENT_BINARY_DIR})

target_link_libraries(cxx-rust-cssparser-impl pthread dl ${RUST_LIB})

install(FILES ${RUST_HEADER} DESTINATION ${KDE_INSTALL_INCLUDEDIR}/cxx-rust-cssparser)
