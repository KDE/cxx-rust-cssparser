// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

#pragma once

#include <filesystem>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "cxx-rust-cssparser-impl.h"

#include "cssparser_export.h"

namespace cssparser
{

using Color = rust::Color;
using Unit = rust::Unit;
using Dimension = rust::Dimension;

using Value = std::variant<std::nullopt_t, std::string, float, int, Color, Dimension>;

using SelectorKind = rust::SelectorKind;

struct CSSPARSER_EXPORT SelectorPart {
    SelectorPart(SelectorKind _kind, const Value &_value)
        : kind(_kind)
        , value(_value)
    {
    }

    inline bool is_combinator() const
    {
        return kind == SelectorKind::DescendantCombinator || kind == SelectorKind::ChildCombinator;
    }

    SelectorKind kind;
    Value value;
};

struct CSSPARSER_EXPORT Selector {
    std::vector<SelectorPart> parts;
};

struct CSSPARSER_EXPORT Property {
    Property(const std::string &_name, const std::vector<Value> &_values)
        : name(_name)
        , values(_values)
    {
    }

    std::string name;
    std::vector<Value> values;
};

struct CSSPARSER_EXPORT CssRule {
    Selector selector;
    std::vector<Property> properties;
};

class CSSPARSER_EXPORT StyleSheet
{
public:
    StyleSheet();
    ~StyleSheet();

    std::vector<CssRule> rules() const;

    void set_root_path(const std::filesystem::path &path);

    void parse_file(const std::string &file_name);
    void parse_string(const std::string &data);
    // std::vector<CssRule> parse(const std::string &source);

    // static void define_property(const std::string &name, const std::string &syntax);
    // static void define_property(const std::string &name, const std::vector<std::string> &syntax);
    // static void define_property(const std::string &name, const std::string &syntax, const Value &initial);

    // rust::Vec<CssRule> parse(rust::Str source);
private:
    struct Private;
    const std::unique_ptr<Private> d;
};

}
