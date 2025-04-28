cmake_minimum_required(VERSION 3.20)

project(cxx-rust-cssparser VERSION 1.0.0)

set(KF_DEP_VERSION 6.10.0)

include(FeatureSummary)
find_package(ECM ${KF_DEP_VERSION} NO_MODULE)
set_package_properties(ECM PROPERTIES TYPE REQUIRED DESCRIPTION "Extra CMake Modules." URL "https://commits.kde.org/extra-cmake-modules")
feature_summary(WHAT REQUIRED_PACKAGES_NOT_FOUND FATAL_ON_MISSING_REQUIRED_PACKAGES)

set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

include(KDEInstallDirs)
include(KDECompilerSettings NO_POLICY_SCOPE)
include(KDECMakeSettings)
include(KDEGitCommitHooks)
include(ECMGenerateExportHeader)
include(ECMSetupVersion)
include(ECMGenerateQDoc)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_AUTOMOC FALSE)
set(CMAKE_AUTORCC FALSE)

set(CMAKE_CXX_VISIBILITY_PRESET "default")

kde_enable_exceptions()

add_subdirectory(rust)
add_subdirectory(cpp)

add_subdirectory(examples)

ecm_setup_version(
    PROJECT
    VARIABLE_PREFIX cssparser
    VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparser_version.h"
    PACKAGE_VERSION_FILE "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparserConfigVersion.cmake"
    SOVERSION 1
)

configure_package_config_file(
    "cxx-rust-cssparserConfig.cmake.in"
    "cxx-rust-cssparserConfig.cmake"
    INSTALL_DESTINATION ${KDE_INSTALL_CMAKEPACKAGEDIR}/cxx-rust-cssparser
)

install(FILES
    "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparserConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/cxx-rust-cssparserConfigVersion.cmake"
    DESTINATION ${KDE_INSTALL_CMAKEPACKAGEDIR}/cxx-rust-cssparser
    COMPONENT Devel
)

install(EXPORT cxx-rust-cssparserTargets
    DESTINATION ${KDE_INSTALL_CMAKEPACKAGEDIR}/cxx-rust-cssparser
    FILE cxx-rust-cssparserTargets.cmake
    # NAMESPACE K::
)

feature_summary(WHAT ALL FATAL_ON_MISSING_REQUIRED_PACKAGES)
