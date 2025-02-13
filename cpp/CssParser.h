/**
 */

#pragma once

#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

// #include <rust/cxx.h>

namespace cssparser
{

// struct CssRule;

using Value = std::variant<std::nullopt_t, std::string, float>;

enum class SelectorKind {
    Unknown,
    Type,
    Class,
    Id,
    PseudoClass,
    Attribute,
    DescendantCombinator,
    ChildCombinator,
};

struct SelectorPart {
    SelectorPart(SelectorKind _kind, const Value &_value)
        : kind(_kind)
        , value(_value)
    {

    }

    SelectorKind kind;
    Value value;
};

struct Selector {
    std::vector<SelectorPart> parts;
};

struct Property {
    Property(const std::string &_name, const Value &_value)
        : name(_name)
        , value(_value)
    {

    }

    std::string name;
    Value value;
};

struct CssRule {
    std::vector<Selector> selectors;
    std::vector<Property> properties;
};

class CssParser
{
public:
    CssParser();

    std::vector<CssRule> parse(const std::string &source);

    // rust::Vec<CssRule> parse(rust::Str source);
};

// std::unique_ptr<CssParser> new_cssparser();
}
