// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2024 Arjen Hiemstra <ahiemstra@heimr.nl>

#include "CssParser.h"

#include <filesystem>
#include <iostream>
#include <fstream>
#include <format>

using namespace std::string_literals;

namespace cssparser
{

Color::Color convert_color(const ::rust::Box<rust::Color> &color)
{
    Color::Color result;
    result.type = color->color_type();
    switch (color->color_type()) {
    case rust::ColorType::Empty:
        break;
    case rust::ColorType::Rgba:
        result.data = color->to_rgba();
        break;
    case rust::ColorType::Custom: {
        auto custom = color->to_custom();

        std::vector<std::string> arguments;
        std::ranges::transform(custom.arguments, std::back_inserter(arguments), [](auto arg) {
            return std::string(arg);
        });

        result.data = Color::CustomColor {
            .source = std::string(custom.source),
            .arguments = arguments,
        };

        break;
    }
    case rust::ColorType::Modified: {
        auto modified = color->to_modified();

        auto result_color = Color::ModifiedColor{};
        result_color.color = std::make_shared<Color::Color>(convert_color(modified.color));
        result_color.operation = modified.operation_type();

        switch (result_color.operation) {
            case rust::ColorOperationType::Add:
            case rust::ColorOperationType::Subtract:
            case rust::ColorOperationType::Multiply:
                result_color.data = std::make_shared<Color::Color>(convert_color(modified.color_value()));
                break;
            case rust::ColorOperationType::Set: {
                auto set_data = modified.set_values();
                auto result_data = Color::SetOperationData{};
                if (set_data.r < 0) {
                    result_data.r = std::nullopt;
                } else {
                    result_data.r = uint8_t(set_data.r);
                }
                if (set_data.g < 0) {
                    result_data.g = std::nullopt;
                } else {
                    result_data.g = uint8_t(set_data.g);
                }
                if (set_data.b < 0) {
                    result_data.b = std::nullopt;
                } else {
                    result_data.b = uint8_t(set_data.b);
                }
                if (set_data.a < 0) {
                    result_data.a = std::nullopt;
                } else {
                    result_data.a = uint8_t(set_data.a);
                }
                result_color.data = result_data;
                break;
            }
            case rust::ColorOperationType::Mix: {
                auto mix_data = modified.mix_values();
                result_color.data = Color::MixOperationData{
                    .other = std::make_shared<Color::Color>(convert_color(mix_data.other)),
                    .amount = mix_data.amount,
                };
                break;
            }
        }

        result.data = result_color;
        break;
    }
    }

    return result;
}

Value convert_value(const rust::Value &input)
{
    switch (input.value_type()) {
        case rust::ValueType::Empty:
            return Value(std::nullopt);
        case rust::ValueType::Dimension: {
            auto dim = input.to_dimension();
            return Value(Dimension{.value = dim.value, .unit = dim.unit});
        }
        case rust::ValueType::String:
            return Value(std::string(input.to_string()));
        case rust::ValueType::Color: {
            return Value(convert_color(input.to_color()));
        }
        case rust::ValueType::Integer: {
            return Value(input.to_integer());
        }
        case rust::ValueType::Url: {
            return Value(Url{ .data = std::string(input.to_url())});
        }
        default:
            break;
    }

    return Value(std::nullopt);
}

struct StyleSheet::Private
{
    void update();

    rust::StyleSheet *stylesheet;
    std::vector<CssRule> rules;
    std::vector<Error> errors;
};

StyleSheet::StyleSheet()
    : d(std::make_unique<Private>())
{
    auto sheet = rust::create_stylesheet();
    d->stylesheet = sheet.into_raw();
}

StyleSheet::~StyleSheet() = default;

std::vector<CssRule> StyleSheet::rules() const
{
    return d->rules;
}

std::vector<Error> StyleSheet::errors() const
{
    return d->errors;
}

void StyleSheet::set_root_path(const std::filesystem::path &path)
{
    d->stylesheet->set_root_path(path.string());
}

void StyleSheet::parse_file(const std::string &file)
{
    try {
        d->stylesheet->parse_file(file);
    } catch (const std::exception &e) {
        d->errors.push_back(Error {
            .file = file,
            .line = 0,
            .column = 0,
            .message = e.what(),
        });

        return;
    }

    d->update();
}

void StyleSheet::parse_string(const std::string &source, const std::string &origin)
{
    d->stylesheet->parse_string(source, origin);
    d->update();
}

void StyleSheet::Private::update()
{
    rules.clear();

    for (const auto &entry : stylesheet->rules()) {
        CssRule rule;

        const auto &properties = entry.properties();
        for (const auto &property : properties) {
            std::vector<Value> vals;
            const auto values = property.values();
            for (const auto &value : values) {
                vals.push_back(convert_value(value));
            }
            rule.properties.emplace_back(std::string(property.name()), vals);
        }

        Selector s;
        const auto &parts = entry.selector().parts();
        for (const auto &part : parts) {
            if (part.kind() == rust::SelectorKind::Attribute) {
                auto new_part = SelectorPart{SelectorKind::Attribute, std::nullopt};
                new_part.attributeMatch = AttributeMatch {
                    .name = std::string(part.attribute_name()),
                    .op = part.attribute_operator(),
                    .value = convert_value(part.attribute_value()),
                };
                s.parts.push_back(new_part);
            } else {
                s.parts.emplace_back(part.kind(), convert_value(part.value()));
            }
        }
        rule.selector = s;

        rules.push_back(rule);
    }

    for (const auto &entry : stylesheet->errors()) {
        errors.push_back(Error{
            .file = std::string(entry.file),
            .line = entry.line,
            .column = entry.column,
            .message = std::string(entry.message),
        });
    }
}

}
