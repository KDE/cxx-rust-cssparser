// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

#include <filesystem>
#include <fstream>
#include <iostream>

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
        }
    }, value);
}

int main(int argc, char **argv)
{
    if (argc != 2) {
        std::cerr << "A file path is required!" << std::endl;
        exit(1);
    }

    std::filesystem::path path(argv[1]);

    StyleSheet sheet;
    sheet.set_root_path(path.parent_path());
    sheet.parse_file(path.filename());

    auto errors = sheet.errors();
    if (!errors.empty()) {
        std::cout << errors.size() << " errors:" << std::endl;

        for (auto error : errors) {
            std::cout << "In file " << error.file << " on line " << error.line << " column " << error.column << ":" << std::endl;
            std::cout << error.message << std::endl;
        }
    }

    auto result = sheet.rules();
    std::cout << result.size() << " results:" << std::endl;

    for (auto entry : result) {
        std::cout << "Selector(" << std::endl;
        for (auto part : entry.selector.parts) {
            std::cout << "  Part(" << kind_to_string(part.kind) << ", " << value_to_string(part.value) << ")" <<    std::endl;
        }
        std::cout << ")" << std::endl;

        for (auto property : entry.properties) {
            std::cout << "Property(" << std::endl;
            std::cout << "  name: " << property.name << std::endl;
            if (property.values.size() == 1) {
                std::cout << "  value: " << value_to_string(property.values.at(0)) << std::endl;
            } else {
                std::cout << "  values:" << std::endl;
                for (auto value : property.values) {
                    std::cout << "    " << value_to_string(value) << std::endl;
                }
            }
            std::cout << ")" << std::endl;
        }
    }
}
