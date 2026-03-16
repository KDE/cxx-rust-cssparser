// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

#pragma once

#include <filesystem>

#include "Selector.h"

#include "cssparser_export.h"

namespace cssparser
{
namespace rust
{
struct Property;
struct StyleRule;
}

class CSSPARSER_EXPORT Property
{
public:
    Property(const std::string &name, const std::vector<Value> &values);
    inline std::string name() const
    {
        return m_name;
    }
    inline std::span<const Value> values() const
    {
        return std::span<const Value>(m_values.cbegin(), m_values.cend());
    }
    template <typename T>
    inline T value(std::size_t index = 0) const
    {
        return m_values.at(index).get<T>();
    }

    inline Value value(std::size_t index = 0) const
    {
        return m_values.at(index);
    }

    // Internal. Convert from a rust Property to a C++ Property.
    static Property fromRust(const rust::Property &rustData);

private:
    std::string m_name;
    std::vector<Value> m_values;
};

class CSSPARSER_EXPORT Rule
{
public:
    Rule();
    Rule(const Selector &selector, const std::vector<Property> &properties);
    inline Selector selector() const
    {
        return m_selector;
    }
    inline std::span<const Property> properties() const
    {
        return std::span<const Property>(m_properties.cbegin(), m_properties.cend());
    }

    // Internal. Convert from a rust StyleRule to a C++ Rule.
    static Rule fromRust(const rust::StyleRule &rustData);

private:
    Selector m_selector;
    std::vector<Property> m_properties;
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

    std::span<const Rule> rules() const;
    std::span<const Error> errors() const;
    void setRootPath(const std::filesystem::path &path);
    void parseFile(const std::string &file_name);
    void parseString(const std::string &data, const std::string &origin);

private:
    struct Private;
    const std::unique_ptr<Private> d;
};

}
