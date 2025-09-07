// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::fmt::Display;

use ffi::ValueConversionError;

use crate::selector::{Selector, SelectorPart, SelectorKind, SelectorValue};
use crate::property::Property;
use crate::stylerule::StyleRule;
use crate::stylesheet::StyleSheet;
use crate::value;

use crate::value::Value;

#[cxx::bridge(namespace = "cssparser::rust")]
mod ffi {
    #[derive(Debug, Clone, Copy)]
    pub enum Unit {
        Unknown,
        Unsupported,
        Number,
        Px,
        Em,
        Rem,
        Pt,
        Percent,
        Degrees,
        Radians,
        Seconds,
        Milliseconds,
    }

    pub enum ValueType {
        Empty,
        Dimension,
        String,
        Color,
        Image,
        Url,
        Integer,
    }

    pub enum AttributeOperator {
        None,
        Exists,
        Equals,
        Includes,
        Prefixed,
        Suffixed,
        Substring,
        DashMatch,
    }

    #[derive(Debug)]
    pub enum SelectorKind {
        Unknown,
        AnyElement,
        Type,
        Class,
        Id,
        PseudoClass,
        Attribute,
        RelativeParent,
        DocumentRoot,
        DescendantCombinator,
        ChildCombinator,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Dimension {
        value: f32,
        unit: Unit,
    }

    pub struct ValueConversionError {
        message: String,
    }

    pub struct StyleSheetError {
        file: String,
        line: u32,
        column: u32,
        message: String,
    }

    extern "Rust" {
        fn to_string(self: &Color) -> String;
        fn to_string(self: &Dimension) -> String;

        type Value;
        fn value_type(self: &Value) -> ValueType;
        fn to_dimension(self: &Value) -> Result<Dimension>;
        fn to_string(self: &Value) -> Result<&str>;
        fn to_color(self: &Value) -> Result<Color>;
        fn to_image(self: &Value) -> Result<&str>;
        fn to_url(self: &Value) -> Result<&str>;
        fn to_integer(self: &Value) -> Result<i32>;

        type SelectorPart;
        fn kind(self: &SelectorPart) -> SelectorKind;
        fn value(self: &SelectorPart) -> &Value;
        fn attribute_name(self: &SelectorPart) -> String;
        fn attribute_operator(self: &SelectorPart) -> AttributeOperator;
        fn attribute_value(self: &SelectorPart) -> &Value;

        type Selector;
        fn parts(self: &Selector) -> Vec<SelectorPart>;

        type Property;
        fn name(self: &Property) -> String;
        fn values(self: &Property) -> Vec<Value>;

        type StyleRule;
        fn selector(self: &StyleRule) -> &Selector;
        fn properties(self: &StyleRule) -> Vec<Property>;

        type StyleSheet;
        fn rules(self: &StyleSheet) -> Vec<StyleRule>;
        fn errors(self: &StyleSheet) -> Vec<StyleSheetError>;
        fn set_root_path(self: &mut StyleSheet, root_path: &str);
        fn parse_file(self: &mut StyleSheet, file_name: &str) -> Result<()>;
        fn parse_string(self: &mut StyleSheet, data: &str, origin: &str) -> Result<()>;

        fn create_stylesheet() -> Box<StyleSheet>;
    }
}

macro_rules! convert_enum {
    ($source:path, $target:path, { $($source_variant:pat => $target_variant:ident),* $(,)? }) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                match value {
                    $(
                        $source_variant => <$target>::$target_variant,
                    )*
                }
            }
        }
    };
}

convert_enum!(value::ValueData, ffi::ValueType, {
    value::ValueData::Empty => Empty,
    value::ValueData::Dimension(_) => Dimension,
    value::ValueData::String(_) => String,
    value::ValueData::Color(_) => Color,
    value::ValueData::Image(_) => Image,
    value::ValueData::Url(_) => Url,
    value::ValueData::Integer(_) => Integer,
});

convert_enum!(value::Unit, ffi::Unit, {
    value::Unit::Unknown => Unknown,
    value::Unit::Unsupported => Unsupported,
    value::Unit::Number => Number,
    value::Unit::Px => Px,
    value::Unit::Em => Em,
    value::Unit::Rem => Rem,
    value::Unit::Pt => Pt,
    value::Unit::Percent => Percent,
    value::Unit::Degrees => Degrees,
    value::Unit::Radians => Radians,
    value::Unit::Seconds => Seconds,
    value::Unit::Milliseconds => Milliseconds,
});

convert_enum!(crate::selector::AttributeOperator, ffi::AttributeOperator, {
    crate::selector::AttributeOperator::None => None,
    crate::selector::AttributeOperator::Exists => Exists,
    crate::selector::AttributeOperator::Equals => Equals,
    crate::selector::AttributeOperator::Includes => Includes,
    crate::selector::AttributeOperator::Prefixed => Prefixed,
    crate::selector::AttributeOperator::Suffixed => Suffixed,
    crate::selector::AttributeOperator::Substring => Substring,
    crate::selector::AttributeOperator::DashMatch => DashMatch,
});

convert_enum!(SelectorKind, ffi::SelectorKind, {
    SelectorKind::Unknown => Unknown,
    SelectorKind::AnyElement => AnyElement,
    SelectorKind::Type => Type,
    SelectorKind::Class => Class,
    SelectorKind::Id => Id,
    SelectorKind::PseudoClass => PseudoClass,
    SelectorKind::Attribute => Attribute,
    SelectorKind::RelativeParent => RelativeParent,
    SelectorKind::DocumentRoot => DocumentRoot,
    SelectorKind::DescendantCombinator => DescendantCombinator,
    SelectorKind::ChildCombinator => ChildCombinator,
});

impl From<&value::Dimension> for ffi::Dimension {
    fn from(value: &value::Dimension) -> Self {
        ffi::Dimension{
            value: value.value,
            unit: value.unit.clone().into(),
        }
    }
}

impl From<&value::Color> for ffi::Color {
    fn from(value: &value::Color) -> Self {
        ffi::Color { r: value.r, g: value.g, b: value.b, a: value.a }
    }
}

impl Display for ffi::Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a))
    }
}

impl Display for ffi::Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{:?}", self.value, self.unit))
    }
}

impl value::Value {
    fn value_type(&self) -> ffi::ValueType {
        self.data.clone().into()
    }

    fn to_dimension(&self) -> Result<ffi::Dimension, ffi::ValueConversionError> {
        if let value::ValueData::Dimension(dimension) = &self.data {
            Ok(dimension.into())
        } else {
            Err(ValueConversionError{ message: String::from("Not a length value") })
        }
    }

    fn to_string(&self) -> Result<&str, ffi::ValueConversionError> {
        if let value::ValueData::String(string) = &self.data {
            Ok(string.as_str())
        } else {
            Err(ValueConversionError{ message: String::from("Not a string value") })
        }
    }

    fn to_color(&self) -> Result<ffi::Color, ffi::ValueConversionError> {
        if let value::ValueData::Color(color) = &self.data {
            Ok(color.into())
        } else {
            Err(ffi::ValueConversionError{ message: String::from("Not a color value") })
        }
    }

    fn to_integer(&self) -> Result<i32, ffi::ValueConversionError> {
        if let value::ValueData::Integer(integer) = self.data {
            Ok(integer)
        } else {
            Err(ffi::ValueConversionError{ message: String::from("Not an integer value") })
        }
    }

    fn to_image(&self) -> Result<&str, ffi::ValueConversionError> {
        Err(ffi::ValueConversionError{ message: String::from("Unimplemented") })
    }

    fn to_url(&self) -> Result<&str, ffi::ValueConversionError> {
        if let value::ValueData::Url(url) = &self.data {
            Ok(url.as_str())
        } else {
            Err(ffi::ValueConversionError{ message: String::from("Not a URL") })
        }
    }
}

impl SelectorPart {
    fn kind(&self) -> ffi::SelectorKind {
        self.kind.into()
    }

    fn value(&self) -> &value::Value {
        if let SelectorValue::Value(value) = &self.value {
            value
        } else {
            Value::empty_ref()
        }
    }

    fn attribute_name(&self) -> String {
        if let SelectorValue::Attribute { name, operator: _, value: _ } = &self.value {
            name.clone()
        } else {
            String::new()
        }
    }

    fn attribute_operator(&self) -> ffi::AttributeOperator {
        if let SelectorValue::Attribute { name: _, operator, value: _ } = self.value {
            ffi::AttributeOperator::from(operator)
        } else {
            ffi::AttributeOperator::None
        }
    }

    fn attribute_value(&self) -> &Value {
        if let SelectorValue::Attribute { name: _, operator: _, value } = &self.value {
            value
        } else {
            Value::empty_ref()
        }
    }
}

impl Selector {
    fn parts(&self) -> Vec<SelectorPart> {
        self.parts.clone()
    }
}

impl Property {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn values(&self) -> Vec<value::Value> {
        self.values.clone()
    }
}

impl StyleRule {
    fn selector(&self) -> &Selector {
        &self.selector
    }

    fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }
}

impl StyleSheet {
    fn rules(&self) -> Vec<StyleRule> {
        self.rules.clone()
    }

    fn errors(&self) -> Vec<ffi::StyleSheetError> {
        let mut result = Vec::new();
        for error in &self.errors {
            result.push(ffi::StyleSheetError{
                file: String::from("Unknown"),
                line: 0,
                column: 0,
                message: format!("{}", error),
            })
        }
        result
    }

    fn set_root_path(&mut self, path: &str) {
        self.root_path = std::path::PathBuf::from(path);
    }
}

fn create_stylesheet() -> Box<StyleSheet> {
    Box::new(StyleSheet::new())
}

impl std::fmt::Display for ValueConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value could not be converted: {}", self.message)
    }
}
