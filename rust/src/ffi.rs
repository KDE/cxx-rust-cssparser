// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use ffi::ValueConversionError;

use crate::selector::{Selector, SelectorPart, SelectorKind, SelectorValue};
use crate::property::Property;
use crate::stylerule::StyleRule;
use crate::stylesheet::StyleSheet;
use crate::value;

use crate::value::Value;
use crate::value::Color;

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

    pub enum ColorType {
        Empty,
        Rgba,
        Custom,
        Mix,
    }

    pub struct Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    pub struct CustomColor {
        source: String,
        arguments: Vec<String>,
    }

    pub struct MixedColor {
        first: Box<Color>,
        second: Box<Color>,
        amount: f32,
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
        fn to_string(self: &Dimension) -> String;

        type Color;
        fn color_type(self: &Color) -> ColorType;
        fn to_string(self: &Color) -> String;
        fn to_rgba(self: &Color) -> Result<Rgba>;
        fn to_custom(self: &Color) -> Result<CustomColor>;
        fn to_mix(self: &Color) -> Result<MixedColor>;

        type Value;
        fn value_type(self: &Value) -> ValueType;
        fn to_dimension(self: &Value) -> Result<Dimension>;
        fn to_string(self: &Value) -> String;
        fn to_color(self: &Value) -> Result<Box<Color>>;
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

convert_enum!(value::ColorData, ffi::ColorType, {
    value::ColorData::Empty => Empty,
    value::ColorData::Rgba{ r: _, g: _, b: _, a: _ } => Rgba,
    value::ColorData::Custom{ source: _, arguments: _ } => Custom,
    value::ColorData::Mix{ first: _, second: _, amount: _ } => Mix,
});

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

impl ffi::Dimension {
    fn to_string(&self) -> String {
        format!("{}{:?}", self.value, self.unit)
    }
}

impl value::Color {
    fn color_type(&self) -> ffi::ColorType {
        self.data.clone().into()
    }

    fn to_string(&self) -> String {
        match &self.data {
            value::ColorData::Empty => format!("Empty"),
            value::ColorData::Rgba{r, g, b, a} => format!("RGBA({}, {}, {}, {})", r, g, b, a),
            value::ColorData::Custom{source, arguments} => format!("Custom({}, {:?})", source, arguments),
            value::ColorData::Mix{first, second, amount} => format!("Mix({}, {}, {})", first.to_string(), second.to_string(), amount),
        }
    }

    fn to_rgba(&self) -> Result<ffi::Rgba, ffi::ValueConversionError> {
        if let value::ColorData::Rgba{r ,g , b, a} = &self.data {
            Ok(ffi::Rgba{r: *r, g: *g, b: *b, a: *a})
        } else {
            Err(ValueConversionError{ message: String::from("Not an RGBA color") })
        }
    }

    fn to_custom(&self) -> Result<ffi::CustomColor, ffi::ValueConversionError> {
        if let value::ColorData::Custom{source, arguments} = &self.data {
            Ok(ffi::CustomColor{source: source.clone(), arguments: arguments.clone()})
        } else {
            Err(ValueConversionError{ message: String::from("Not an RGBA color") })
        }
    }

    fn to_mix(&self) -> Result<ffi::MixedColor, ffi::ValueConversionError> {
        if let value::ColorData::Mix{first, second, amount} = &self.data {
            Ok(ffi::MixedColor{first: first.clone(), second: second.clone(), amount: *amount})
        } else {
            Err(ValueConversionError{ message: String::from("Not an RGBA color") })
        }
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

    fn to_color(&self) -> Result<Box<Color>, ffi::ValueConversionError> {
        if let value::ValueData::Color(color) = &self.data {
            Ok(Box::new(color.clone()))
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
