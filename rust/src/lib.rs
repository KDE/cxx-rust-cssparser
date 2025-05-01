// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

mod details;

pub mod value;
pub mod selector;
pub mod property;
pub mod stylerule;
pub mod stylesheet;

use selector::{Selector, SelectorPart};
use property::Property;
use stylerule::StyleRule;
use stylesheet::StyleSheet;
use value::{Value, ValueData};

#[cfg(test)]
mod tests;

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

#[cxx::bridge(namespace = "cssparser::rust")]
mod ffi {
    pub enum ValueType {
        Empty,
        Length,
        Number,
        Percentage,
        String,
        Color,
        Image,
        Url,
        Integer,
        Angle,
        Time,
    }

    #[derive(Debug)]
    pub enum SelectorKind {
        Unknown,
        Type,
        Class,
        Id,
        PseudoClass,
        Attribute,
        RelativeParent,
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
    pub enum Unit {
        Unknown,
        Unsupported,
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

    #[derive(Debug, Clone, Copy)]
    pub struct Dimension {
        value: f32,
        unit: Unit,
    }

    extern "Rust" {
        fn to_string(self: &Color) -> String;
        fn to_string(self: &Dimension) -> String;

        type Value;
        fn value_type(self: &Value) -> ValueType;
        fn to_length(self: &Value) -> Result<Dimension>;
        fn to_number(self: &Value) -> Result<f32>;
        fn to_percentage(self: &Value) -> Result<f32>;
        fn to_string(self: &Value) -> Result<&str>;
        fn to_color(self: &Value) -> Result<Color>;
        fn to_integer(self: &Value) -> Result<i32>;
        fn to_angle(self: &Value) -> Result<Dimension>;
        fn to_time(self: &Value) -> Result<Dimension>;

        type SelectorPart;
        fn kind(self: &SelectorPart) -> SelectorKind;
        fn value(self: &SelectorPart) -> &Value;

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
        fn set_root_path(self: &mut StyleSheet, root_path: &str);
        fn parse_file(self: &mut StyleSheet, file_name: &str) -> Result<()>;
        fn parse_string(self: &mut StyleSheet, data: &str) -> Result<()>;

        fn create_stylesheet() -> Box<StyleSheet>;
    }
}

convert_enum!(value::ValueData, ffi::ValueType, {
    ValueData::Empty => Empty,
    ValueData::Length(_) => Length,
    ValueData::Number(_) => Number,
    ValueData::Percentage(_) => Percentage,
    ValueData::String(_) => String,
    ValueData::Color(_) => Color,
    ValueData::Image(_) => Image,
    ValueData::Url(_) => Url,
    ValueData::Integer(_) => Integer,
    ValueData::Angle(_) => Angle,
    ValueData::Time(_) => Time,
});

convert_enum!(value::Unit, ffi::Unit, {
    value::Unit::Unknown => Unknown,
    value::Unit::Unsupported => Unsupported,
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

convert_enum!(selector::SelectorKind, ffi::SelectorKind, {
    selector::SelectorKind::Unknown => Unknown,
    selector::SelectorKind::Type => Type,
    selector::SelectorKind::Class => Class,
    selector::SelectorKind::Id => Id,
    selector::SelectorKind::PseudoClass => PseudoClass,
    selector::SelectorKind::Attribute => Attribute,
    selector::SelectorKind::RelativeParent => RelativeParent,
    selector::SelectorKind::DescendantCombinator => DescendantCombinator,
    selector::SelectorKind::ChildCombinator => ChildCombinator,
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

impl ffi::Color {
    fn to_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}

impl ffi::Dimension {
    fn to_string(&self) -> String {
        format!("{}{:?}", self.value, self.unit)
    }
}

impl Value {
    fn value_type(&self) -> ffi::ValueType {
        self.data.clone().into()
    }

    fn to_length(&self) -> Result<ffi::Dimension, details::ParseError> {
        if let ValueData::Length(dimension) = &self.data {
            Ok(dimension.into())
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a length value")))
        }
    }

    fn to_number(&self) -> Result<f32, details::ParseError> {
        if let ValueData::Number(number) = self.data {
            Ok(number)
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a number value")))
        }
    }

    fn to_percentage(&self) -> Result<f32, details::ParseError> {
        if let ValueData::Percentage(percentage) = self.data {
            Ok(percentage)
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a percentage value")))
        }
    }

    fn to_string(&self) -> Result<&str, details::ParseError> {
        if let ValueData::String(string) = &self.data {
            Ok(string.as_str())
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a string value")))
        }
    }

    fn to_color(&self) -> Result<ffi::Color, details::ParseError> {
        if let ValueData::Color(color) = &self.data {
            Ok(color.into())
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a color value")))
        }
    }

    fn to_integer(&self) -> Result<i32, details::ParseError> {
        if let ValueData::Integer(integer) = self.data {
            Ok(integer)
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not an integer value")))
        }
    }

    fn to_angle(&self) -> Result<ffi::Dimension, details::ParseError> {
        if let ValueData::Angle(dimension) = &self.data {
            Ok(dimension.into())
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not an angle value")))
        }
    }

    fn to_time(&self) -> Result<ffi::Dimension, details::ParseError> {
        if let ValueData::Time(dimension) = &self.data {
            Ok(dimension.into())
        } else {
            Err(details::ParseError::InvalidPropertyValue(String::from("Not a time value")))
        }
    }
}

impl SelectorPart {
    fn kind(&self) -> ffi::SelectorKind {
        self.kind.into()
    }

    fn value(&self) -> &Value {
        &self.value
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

    fn values(&self) -> Vec<Value> {
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

    fn set_root_path(&mut self, path: &str) {
        self.root_path = std::path::PathBuf::from(path);
    }
}

fn create_stylesheet() -> Box<StyleSheet> {
    Box::new(StyleSheet::new())
}
