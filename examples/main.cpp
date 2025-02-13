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
        case SelectorKind::Type: return "Type"s;
        case SelectorKind::Class: return "Class"s;
        case SelectorKind::Id: return "Id"s;
        case SelectorKind::Attribute: return "Attribute"s;
        case SelectorKind::PseudoClass: return "PseudoClass"s;
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
    std::ifstream file(path, std::ios::in);
    if (!file.is_open()) {
        std::cerr << "Could not read file!" << std::endl;
        exit(2);
    }

    const auto size = std::filesystem::file_size(path);

    std::string data(size, '\0');
    file.read(data.data(), size);

    CssParser parser;
    auto result = parser.parse(data);

    std::cout << result.size() << " results:" << std::endl;

    for (auto entry : result) {
        // std::cout << entry.selectors;
        for (auto selector : entry.selectors) {
            std::cout << "Selector(" << std::endl;
            for (auto part : selector.parts) {
                std::cout << "  Part(" << kind_to_string(part.kind) << ", " << value_to_string(part.value) << ")" << std::endl;
            }
            std::cout << ")" << std::endl;
        }

        for (auto property : entry.properties) {
            std::cout << "Property(" << std::endl;
            std::cout << "  name: " << property.name << std::endl;
            std::cout << "  value: " << value_to_string(property.value) << std::endl;
            std::cout << ")" << std::endl;
        }

        // std::cout << entry.properties;
    }
}
