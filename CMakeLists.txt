cmake_minimum_required(VERSION 3.20)

project(cxx-rust-cssparser VERSION 1.0.0)

include(FeatureSummary)
find_package(ECM 6.0.0 NO_MODULE)
set_package_properties(ECM PROPERTIES TYPE REQUIRED DESCRIPTION "Extra CMake Modules." URL "https://commits.kde.org/extra-cmake-modules")
feature_summary(WHAT REQUIRED_PACKAGES_NOT_FOUND FATAL_ON_MISSING_REQUIRED_PACKAGES)

set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

include(KDEInstallDirs)
include(KDECompilerSettings NO_POLICY_SCOPE)
include(KDECMakeSettings)
include(KDEGitCommitHooks)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_AUTOMOC FALSE)
set(CMAKE_AUTORCC FALSE)

set(CMAKE_CXX_VISIBILITY_PRESET "default")

kde_enable_exceptions()

add_subdirectory(rust)
add_subdirectory(cpp)

add_subdirectory(examples)
