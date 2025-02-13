add_library(cxx-rust-cssparser)

target_sources(cxx-rust-cssparser PRIVATE
    CssParser.cpp
)

target_link_libraries(cxx-rust-cssparser PRIVATE cxx-rust-cssparser-impl)
