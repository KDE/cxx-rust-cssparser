// use std::error::Error;

use crate::ffi::{SelectorKind, ValueType};

#[derive(Debug)]
pub struct Error(&'static str);

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone)]
pub enum ValueData {
    #[default] Empty,
    String(String),
    Number(f32),
}

#[derive(Debug, Default, Clone)]
pub struct Value {
    data: ValueData
}

impl Value {
    pub fn empty() -> Value {
        Value{data: ValueData::Empty}
    }

    pub fn value_type(&self) -> ValueType {
        match self.data {
            ValueData::Empty => ValueType::Empty,
            ValueData::String(_) => ValueType::String,
            ValueData::Number(_) => ValueType::Number,
        }
    }

    pub fn to_string(&self) -> Result<&str, Error> {
        if let ValueData::String(contents) = &self.data {
            return Ok(contents);
        } else {
            return Err(Error("Not a string"));
        }
    }

    pub fn to_number(&self) -> Result<f32, Error> {
        if let ValueData::Number(contents) = &self.data {
            return Ok(*contents);
        } else {
            return Err(Error("Not a number"));
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value{data: ValueData::String(value)}
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value{data: ValueData::Number(value)}
    }
}

#[derive(Debug, Clone)]
pub struct SelectorPart {
    pub kind: SelectorKind,
    pub value: Value,
}

impl SelectorPart {
    pub fn new(kind: SelectorKind, value: Value) -> SelectorPart {
        SelectorPart{kind, value}
    }

    pub fn kind(&self) -> SelectorKind {
        self.kind
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl Selector {
    pub fn new() -> Selector {
        Selector{parts: Vec::new()}
    }

    pub fn parts(&self) -> Vec<SelectorPart> {
        self.parts.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

impl Property {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Debug)]
pub struct CssRule {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

impl CssRule {
    pub fn selectors(&self) -> Vec<Selector> {
        self.selectors.clone()
    }

    pub fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }
}
