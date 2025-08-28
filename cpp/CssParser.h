// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

#pragma once

#include <filesystem>
#include <format>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "cxx-rust-cssparser-impl.h"

#include "cssparser_export.h"

namespace cssparser
{

namespace Color
{

using ColorType = rust::ColorType;
struct Color;

using Rgba = rust::Rgba;

struct CustomColor {
    std::string source;
    std::vector<std::string> arguments;
};

struct MixedColor {
    std::shared_ptr<Color> first;
    std::shared_ptr<Color> second;
    float amount;
};

struct Color {
    ColorType type;
    std::variant<std::nullopt_t, Rgba, CustomColor, MixedColor> data = std::nullopt;

    inline std::string to_string() const {
        switch (type) {
        case ColorType::Empty:
            return "Empty";
        case ColorType::Rgba: {
            auto rgba = std::get<Rgba>(data);
            return std::format("RGBA({}, {}, {}, {})", rgba.r, rgba.g, rgba.b, rgba.a);
        }
        case ColorType::Custom: {
            auto custom = std::get<CustomColor>(data);
            auto args = std::string{};

            for (auto arg : custom.arguments) {
                if (!args.empty()) {
                    args += ", ";
                }
                args += arg;
            }

            return std::format("CustomColor(source: {}, arguments: {})", std::string(custom.source), args);
        }
        case ColorType::Mix: {
            auto mix = std::get<MixedColor>(data);
            return std::format("MixColor(first: {}, second: {}, amount: {:.1f})", mix.first->to_string(), mix.second->to_string(), mix.amount);
        }
        }
        return std::string();
    }
};

}

using Unit = rust::Unit;

struct Dimension {
    float value;
    Unit unit;

    inline operator float() const {
        return value;
    }

    inline std::string to_string() const {
        switch (unit) {
        case Unit::Px:
            return std::format("{} px", value);
        case Unit::Em:
            return std::format("{} em", value);
        case Unit::Rem:
            return std::format("{} rem", value);
        case Unit::Pt:
            return std::format("{} pt", value);
        case Unit::Percent:
            return std::format("{} %", value);
        default:
            return std::format("{} (Unknown unit)", value);
        }
    }
};

struct Url {
    std::string data;
};

using AttributeOperator = rust::AttributeOperator;

using Value = std::variant<std::nullopt_t, std::string, int, Color::Color, Dimension, Url>;

struct AttributeMatch {
    std::string name;
    AttributeOperator op;
    Value value;
};

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
    std::optional<AttributeMatch> attributeMatch;
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

    template <typename T>
    inline T value(std::size_t index = 0) const
    {
        return std::get<T>(values.at(index));
    }

    inline Value value(std::size_t index = 0) const
    {
        return values.at(index);
    }

    std::string name;
    std::vector<Value> values;
};

struct CSSPARSER_EXPORT CssRule {
    Selector selector;
    std::vector<Property> properties;
};

struct CSSPARSER_EXPORT Error {
    std::string file;
    uint32_t line = 0;
    uint32_t column = 0;
    std::string message;
};

class CSSPARSER_EXPORT StyleSheet
{
public:
    StyleSheet();
    ~StyleSheet();

    std::vector<CssRule> rules() const;
    std::vector<Error> errors() const;

    void set_root_path(const std::filesystem::path &path);

    void parse_file(const std::string &file_name);
    void parse_string(const std::string &data, const std::string &origin);

private:
    struct Private;
    const std::unique_ptr<Private> d;
};

}
