// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::details::identifier::Identifier;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self{r: value.0, g: value.1, b: value.2, a: 255}
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from(value: (u8, u8, u8, f32)) -> Self {
        Self{r: value.0, g: value.1, b: value.2, a: (value.3 * 255.0) as u8}
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Unit {
    #[default] Unknown,
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

impl Unit {
    pub fn parse(input: &str) -> Unit {
        match input {
            "px" => Unit::Px,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "pt" => Unit::Pt,
            "%" => Unit::Percent,
            "deg" => Unit::Degrees,
            "rad" => Unit::Radians,
            "s" => Unit::Seconds,
            "ms" => Unit::Milliseconds,
            "mm"
            | "cm"
            | "Q"
            | "in"
            | "pc"
            | "vh"
            | "vw"
            | "lh"
            | "rlh"
            | "grad"
            | "turn" => Unit::Unsupported,
            _ => Unit::Unknown,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dimension {
    pub value: f32,
    pub unit: Unit,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ValueData {
    #[default] Empty,
    Length(Dimension),
    Number(f32),
    Percentage(f32),
    String(String),
    Color(Color),
    Image(String),
    Url(String),
    Integer(i32),
    Angle(Dimension),
    Time(Dimension),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Value {
    pub(crate) data: ValueData
}

impl Value {
    pub fn empty() -> Value {
        Value{data: ValueData::Empty}
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let named_color = cssparser::color::parse_named_color(value);
        if let Ok(color) = named_color {
            return Value{data: ValueData::Color(Color{r: color.0, g: color.1, b: color.2, a: 255})};
        }

        let hashed_color = cssparser::color::parse_hash_color(value.as_bytes());
        if let Ok(color) = hashed_color {
            return Value{data: ValueData::Color(Color{r: color.0, g: color.1, b: color.2, a: cssparser::color::clamp_unit_f32(color.3)})};
        }

        Value{data: ValueData::String(value.to_string())}
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value{data: ValueData::Number(value)}
    }
}

impl From<&Identifier> for Value {
    fn from(value: &Identifier) -> Self {
        let id: String = value.into();
        Value::from(id.as_str())
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Value{data: ValueData::Color(value)}
    }
}

impl From<Dimension> for Value {
    fn from(value: Dimension) -> Self {
        Value{data: ValueData::Length(value)}
    }
}
