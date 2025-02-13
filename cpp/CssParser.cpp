#include "CssParser.h"

#include <filesystem>
#include <iostream>
#include <fstream>

#include "cxx-rust-cssparser-impl.h"

using namespace std::string_literals;

namespace cssparser
{

Value convert_value(const rust::Value &input)
{
    switch (input.value_type()) {
        case rust::ValueType::String:
            return Value(std::string(input.to_string()));
        case rust::ValueType::Number:
            return Value(input.to_number());
        default:
            break;
    }

    return Value(std::nullopt);
}

SelectorKind convert_selector_kind(rust::SelectorKind input)
{
    switch (input) {
        case rust::SelectorKind::Unknown: return SelectorKind::Unknown;
        case rust::SelectorKind::Type: return SelectorKind::Type;
        case rust::SelectorKind::Class: return SelectorKind::Class;
        case rust::SelectorKind::Id: return SelectorKind::Id;
        case rust::SelectorKind::PseudoClass: return SelectorKind::PseudoClass;
        case rust::SelectorKind::Attribute: return SelectorKind::Attribute;
        case rust::SelectorKind::DescendantCombinator: return SelectorKind::DescendantCombinator;
        case rust::SelectorKind::ChildCombinator: return SelectorKind::ChildCombinator;
    }

    return SelectorKind::Unknown;
}

CssParser::CssParser()
{
}

std::vector<CssRule> CssParser::parse(const std::string &source)
{
    const auto result = rust::parse(source);

    std::vector<CssRule> output;
    for (const auto &entry : result) {
        CssRule rule;

        const auto properties = entry.properties();
        for (const auto &property : properties) {
            rule.properties.emplace_back(std::string(property.name()), convert_value(property.value()));
        }

        const auto selectors = entry.selectors();
        for (const auto &selector : selectors) {
            Selector s;

            const auto parts = selector.parts();
            for (const auto &part : parts) {
                s.parts.emplace_back(convert_selector_kind(part.kind()), convert_value(part.value()));
            }

            rule.selectors.push_back(s);
        }

        output.push_back(rule);
    }

    return output;
}

}
