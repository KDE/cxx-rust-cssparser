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

Value convert_value(const rust::Value &input)
{
    switch (input.value_type()) {
        case rust::ValueType::Empty:
            return Value(std::nullopt);
        case rust::ValueType::Length:
            return Value(input.to_length());
        case rust::ValueType::String:
            return Value(std::string(input.to_string()));
        case rust::ValueType::Number:
            return Value(input.to_number());
        case rust::ValueType::Color: {
            return Value(input.to_color());
        default:
            break;
        }
    }

    return Value(std::nullopt);
}

struct StyleSheet::Private
{
    void update_rules();

    rust::StyleSheet *stylesheet;
    std::vector<CssRule> rules;
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

void StyleSheet::set_root_path(const std::filesystem::path &path)
{
    d->stylesheet->set_root_path(path.string());
}

void StyleSheet::parse_file(const std::string &file)
{
    d->stylesheet->parse_file(file);
    d->update_rules();
}

void StyleSheet::parse_string(const std::string &source)
{
    d->stylesheet->parse_string(source);
    d->update_rules();
}

void StyleSheet::Private::update_rules()
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
            s.parts.emplace_back(part.kind(), convert_value(part.value()));
        }
        rule.selector = s;

        rules.push_back(rule);
    }
}

}
