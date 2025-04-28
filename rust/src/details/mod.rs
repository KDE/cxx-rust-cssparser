// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::io;

pub mod identifier;
pub mod rulesparser;
pub mod selectorparser;

pub mod propertysyntax;
pub mod propertyparser;

#[derive(Debug)]
pub enum ParseError {
    Unspecified,
    Unknown(String),
    UnknownProperty(String),
    InvalidSelectors(String),
    InvalidPropertySyntax(String),
    InvalidPropertyValue(String),
    InvalidPropertyDefinition(String),
    UnsupportedAtRule(String),
    FileError(io::Error),
}

#[macro_export]
macro_rules! parse_error {
    ($p:ident, $i:ident, $m:expr) => {
        Err($p.new_custom_error(ParseError::$i($m)))
    };
}

impl std::error::Error for ParseError {
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unspecified => write!(f, "{}", "Unspecified error"),
            ParseError::Unknown(message) => write!(f, "Unknown error: {}", message),
            ParseError::UnknownProperty(message) => write!(f, "Unknown Property: {}", message),
            ParseError::InvalidSelectors(message) => write!(f, "Invalid Selectors: {}", message),
            ParseError::InvalidPropertySyntax(message) => write!(f, "Invalid property syntax: {}", message),
            ParseError::InvalidPropertyValue(message) => write!(f, "Invalid property value: {}", message),
            ParseError::InvalidPropertyDefinition(message) => write!(f, "Invalid property definition: {}", message),
            ParseError::UnsupportedAtRule(message) => write!(f, "Unsupported @-rule: {}", message),
            ParseError::FileError(error) => write!(f, "{}", error),
        }
    }
}

impl From<selectors::parser::SelectorParseErrorKind<'_>> for ParseError {
    fn from(value: selectors::parser::SelectorParseErrorKind<'_>) -> Self {
        match value {
            selectors::parser::SelectorParseErrorKind::NoQualifiedNameInAttributeSelector(_) => ParseError::InvalidSelectors(String::from("No qualified name in attribute selector")),
            selectors::parser::SelectorParseErrorKind::EmptySelector => ParseError::InvalidSelectors(String::from("Empty Selector")),
            selectors::parser::SelectorParseErrorKind::DanglingCombinator => ParseError::InvalidSelectors(String::from("Dangling Combinator")),
            selectors::parser::SelectorParseErrorKind::NonCompoundSelector => ParseError::InvalidSelectors(String::from("Non-compound Selector")),
            _ => ParseError::InvalidSelectors(String::from("Selectors failed to parse")),
        }
    }
}

impl From<cssparser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>> for ParseError {
    fn from(value: cssparser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>) -> Self {
        if let cssparser::ParseErrorKind::Custom(selector_error) = value.kind {
            ParseError::from(selector_error)
        } else {
            ParseError::Unknown(String::from("Unknown parse error"))
        }
    }
}
