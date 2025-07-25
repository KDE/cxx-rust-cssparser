# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

add_library(cxx-rust-cssparser)

target_sources(cxx-rust-cssparser PRIVATE
    CssParser.cpp
    CssParser.h
)

ecm_generate_export_header(cxx-rust-cssparser
    BASE_NAME cssparser
    VERSION ${PROJECT_VERSION}
    DEPRECATED_BASE_VERSION 0
    EXCLUDE_DEPRECATED_BEFORE_AND_AT ${EXCLUDE_DEPRECATED_BEFORE_AND_AT}
)

target_link_libraries(cxx-rust-cssparser PRIVATE cxx-rust-cssparser-impl)

target_include_directories(cxx-rust-cssparser PUBLIC
    "$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>"
    "$<BUILD_INTERFACE:${CMAKE_BINARY_DIR}/rust>"
    "$<INSTALL_INTERFACE:${KDE_INSTALL_INCLUDEDIR}/cxx-rust-cssparser>"
)

if (CMAKE_SYSTEM_NAME MATCHES "FreeBSD")
    # On FreeBSD, we cannot compile with "-Wl,--no-undefined" because that will
    # fail to find the "environ" symbol.
    string(REPLACE "-Wl,--no-undefined " " " CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS}")
endif()

set_target_properties(cxx-rust-cssparser PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION ${PROJECT_VERSION_MAJOR}
)

install(TARGETS cxx-rust-cssparser EXPORT cxx-rust-cssparserTargets ${KDE_INSTALL_DEFAULT_ARGS})
install(FILES
    CssParser.h
    ${CMAKE_CURRENT_BINARY_DIR}/cssparser_export.h
    DESTINATION ${KDE_INSTALL_INCLUDEDIR}/cxx-rust-cssparser
)
