// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

#include <filesystem>
#include <fstream>
#include <iostream>

#include <getopt.h>

#include "CssParser.h"

using namespace cssparser;
using namespace std::string_literals;

std::string kind_to_string(SelectorKind kind)
{
    switch (kind) {
        case SelectorKind::Unknown: return "Unknown"s;
        case SelectorKind::AnyElement: return "AnyElement"s;
        case SelectorKind::Type: return "Type"s;
        case SelectorKind::Class: return "Class"s;
        case SelectorKind::Id: return "Id"s;
        case SelectorKind::Attribute: return "Attribute"s;
        case SelectorKind::RelativeParent: return "RelativeParent"s;
        case SelectorKind::PseudoClass: return "PseudoClass"s;
        case SelectorKind::DocumentRoot: return "DocumentRoot"s;
        case SelectorKind::DescendantCombinator: return "DescendantCombinator";
        case SelectorKind::ChildCombinator: return "ChildCombinator";
    }

    return "Unknown"s;
}

std::string value_to_string(const Value &value)
{
    return std::visit([](auto &&arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, std::nullopt_t>) {
            return "Empty"s;
        } else if constexpr (std::is_same_v<T, std::string>) {
            return arg;
        } else if constexpr (std::is_same_v<T, float>) {
            return std::to_string(arg);
        } else if constexpr (std::is_same_v<T, int>) {
            return std::to_string(arg);
        } else if constexpr (std::is_same_v<T, Color>) {
            return std::string(arg.to_string());
        } else if constexpr (std::is_same_v<T, Dimension>) {
            return std::string(arg.to_string());
        } else if constexpr (std::is_same_v<T, Url>) {
            return arg.data;
        }
    }, value);
}

std::string selector_part_to_string(const SelectorPart &part)
{
    auto kind = kind_to_string(part.kind);
    switch (part.kind) {
        case SelectorKind::Unknown:
        case SelectorKind::AnyElement:
        case SelectorKind::DocumentRoot:
        case SelectorKind::DescendantCombinator:
        case SelectorKind::ChildCombinator:
            return kind;
        default:
            return kind + ": " + value_to_string(part.value);
    }
}

int main(int argc, char **argv)
{
    std::vector<std::filesystem::path> prepend_files;
    std::vector<std::filesystem::path> append_files;

    int show_help = 0;

    auto options = std::array{
        option { .name = "prepend", .has_arg = required_argument, .flag = nullptr, .val = 'p' },
        option { .name = "append", .has_arg = required_argument, .flag = nullptr, .val = 'a' },
        option { .name = "help", .has_arg = no_argument, .flag = nullptr, .val = 'h' },
    };

    int c = 0;
    int i = 0;
    while ((c = getopt_long(argc, argv, "", options.data(), &i)) != -1) {
        switch (c) {
        case 'p':
            prepend_files.push_back(std::string(optarg));
            break;
        case 'a':
            append_files.push_back(std::string(optarg));
            break;
        case 'h':
            show_help = 1;
            break;
        default:
            std::cout << "Unrecognized option: " << c;
            show_help = 1;
        }
    }

    if (optind >= argc && show_help == 0) {
        std::cerr << "A file path is required!" << std::endl;
        show_help = 1;
    }

    if (show_help != 0) {
        std::cout << "Usage: " << argv[0] << "[options] <filename>\n\n";
        std::cout << "Options:\n";
        std::cout << "--prepend <filename> Add and parse <filename> before parsing the main file.\n";
        std::cout << "--append <filename> Add and parse <filename> after parsing the main file.\n";
        exit(1);
    }

    std::filesystem::path path(argv[optind]);

    StyleSheet sheet;

    for (auto file : prepend_files) {
        if (file.has_parent_path()) {
            sheet.set_root_path(file.parent_path());
            sheet.parse_file(file.filename());
        } else{
            sheet.set_root_path(path.parent_path());
            sheet.parse_file(file);
        }
    }

    sheet.set_root_path(path.parent_path());
    sheet.parse_file(path.filename());

    for (auto file : append_files) {
        if (file.has_parent_path()) {
            sheet.set_root_path(file.parent_path());
            sheet.parse_file(file.filename());
        } else{
            sheet.set_root_path(path.parent_path());
            sheet.parse_file(file);
        }
    }

    auto errors = sheet.errors();
    if (!errors.empty()) {
        std::cout << errors.size() << " errors:" << std::endl;

        for (auto error : errors) {
            std::cout << error.message << std::endl;
        }

        exit(2);
    }

    auto result = sheet.rules();
    std::cout << result.size() << " results:" << std::endl;

    for (auto entry : result) {
        std::cout << "StyleRule {\n";
        std::cout << "  selector:\n";
        for (auto part : entry.selector.parts) {
            std::cout << "    " << selector_part_to_string(part) << "\n";
        }
        std::cout << "\n";

        for (auto property : entry.properties) {
            // std::cout << "  Property(";
            if (property.values.size() == 1) {
                std::cout << "  " << property.name << ": " << value_to_string(property.values.at(0)) << "\n";
            } else {
                std::cout << "\n";
                std::cout << "  " << property.name << ":\n";
                for (auto value : property.values) {
                    std::cout << "    " << value_to_string(value) << "\n";
                }
                // std::cout << "  )\n";
            }
        }

        std::cout << "}\n\n";
    }
}
