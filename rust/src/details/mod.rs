// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

pub mod identifier;
pub mod rulesparser;
pub mod selectorparser;

pub mod property;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    Unspecified,
    Unimplemented,
    UnexpectedEndOfInput,
    Unknown,
    UnknownProperty,
    UnexpectedToken,
    InvalidSelectors,
    InvalidPropertySyntax,
    InvalidPropertyValue,
    UnknownFunction,
    InvalidPropertyDefinition,
    PropertyValueDoesNotMatchSyntax,
    UnsupportedAtRule,
    InvalidAtRule,
    InvalidQualifiedRule,
    FileError,
    StyleSheetParseError,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl SourceLocation {
    pub fn from_file_location(file: String, location: cssparser::SourceLocation) -> SourceLocation {
        SourceLocation { file, line: location.line, column: location.column }
    }

    pub fn from_file(file: &str) -> SourceLocation {
        SourceLocation { file: file.to_string(), line: 0, column: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub location: SourceLocation,
}

pub fn parse_error<'i, 't, R>(parser: &cssparser::Parser<'i, 't>, kind: ParseErrorKind, message: String) -> Result<R, cssparser::ParseError<'i, ParseError>> {
    Err(parser.new_custom_error(ParseError{ kind, message, location: SourceLocation::from_file_location(parser.current_source_url().unwrap_or("").to_string(), parser.current_source_location())}))
}

pub fn unwrap_parse_error<'i, 't, R>(error: &'t Result<R, cssparser::ParseError<'i, ParseError>>) -> Option<&'t ParseError> {
    if let Err(parse_error) = error {
        if let cssparser::ParseErrorKind::Custom(custom_error) = &parse_error.kind {
            Some(custom_error)
        } else {
            None
        }
    } else {
        None
    }
}

impl std::error::Error for ParseError {
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "In file \"{}\" at line {} column {}: ", self.location.file, self.location.line, self.location.column)?;
        match self.kind {
            ParseErrorKind::Unspecified => write!(f, "Unspecified error"),
            ParseErrorKind::Unimplemented => write!(f, "Feature is not yet implemented"),
            ParseErrorKind::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseErrorKind::Unknown => write!(f, "Unknown error: {}", self.message),
            ParseErrorKind::UnknownProperty => write!(f, "Unknown Property: {}", self.message),
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token: {}", self.message),
            ParseErrorKind::InvalidSelectors => write!(f, "Invalid Selectors: {}", self.message),
            ParseErrorKind::InvalidPropertySyntax => write!(f, "Invalid property syntax: {}", self.message),
            ParseErrorKind::InvalidPropertyValue => write!(f, "Invalid property value: {}", self.message),
            ParseErrorKind::UnknownFunction => write!(f, "Unknown function: {}", self.message),
            ParseErrorKind::InvalidPropertyDefinition => write!(f, "Invalid property definition: {}", self.message),
            ParseErrorKind::PropertyValueDoesNotMatchSyntax => write!(f, "Property value does not match syntax: {}", self.message),
            ParseErrorKind::UnsupportedAtRule => write!(f, "Unsupported @-rule: {}", self.message),
            ParseErrorKind::InvalidAtRule => write!(f, "Invalid @-rule: {}", self.message),
            ParseErrorKind::InvalidQualifiedRule => write!(f, "Invalid qualified rule"),
            ParseErrorKind::FileError => write!(f, "IO Error: {}", self.message),
            ParseErrorKind::StyleSheetParseError => write!(f, "Stylesheet failed to parse: {}", self.message),
        }
    }
}

impl ParseError {
    fn from_cssparser_error<'i, E>(value: cssparser::ParseError<'i, E>, file: String) -> Self
        where E: ToParseError,
    {
        match value.kind {
            cssparser::ParseErrorKind::Basic(basic_error) => basic_error.to_parse_error(file, value.location),
            cssparser::ParseErrorKind::Custom(custom_error) => custom_error.to_parse_error(file, value.location),
        }
    }
}

trait ToParseError {
    fn to_parse_error(&self, file: String, location: cssparser::SourceLocation) -> ParseError;
}

impl ToParseError for ParseError {
    fn to_parse_error(&self, _file: String, _location: cssparser::SourceLocation) -> ParseError {
        self.clone()
    }
}

impl ToParseError for cssparser::BasicParseErrorKind<'_> {
    fn to_parse_error(&self, file: String, location: cssparser::SourceLocation) -> ParseError {
        let location = SourceLocation::from_file_location(file, location);
        match self {
            cssparser::BasicParseErrorKind::UnexpectedToken(token) => ParseError{ kind: ParseErrorKind::UnexpectedToken, message: format!("{:?}", token), location },
            cssparser::BasicParseErrorKind::EndOfInput => ParseError{ kind: ParseErrorKind::UnexpectedEndOfInput, message: String::new(), location },
            cssparser::BasicParseErrorKind::AtRuleInvalid(at_rule) => ParseError{ kind: ParseErrorKind::InvalidAtRule, message: at_rule.to_string(), location },
            cssparser::BasicParseErrorKind::AtRuleBodyInvalid => ParseError{ kind: ParseErrorKind::InvalidAtRule, message: String::from("Invalid @-rule body"), location },
            cssparser::BasicParseErrorKind::QualifiedRuleInvalid => ParseError{ kind:ParseErrorKind::InvalidQualifiedRule, message: String::new(), location },
        }
    }
}

impl ToParseError for selectors::parser::SelectorParseErrorKind<'_>
{
    fn to_parse_error(&self, file: String, location: cssparser::SourceLocation) -> ParseError {
        let location = SourceLocation::from_file_location(file, location);
        match self {
            selectors::parser::SelectorParseErrorKind::NoQualifiedNameInAttributeSelector(_) =>
                ParseError{ kind: ParseErrorKind::InvalidSelectors, message: String::from("No qualified name in attribute selector"), location },
            selectors::parser::SelectorParseErrorKind::EmptySelector => ParseError{ kind: ParseErrorKind::InvalidSelectors, message: String::from("Empty Selector"), location },
            selectors::parser::SelectorParseErrorKind::DanglingCombinator => ParseError{ kind: ParseErrorKind::InvalidSelectors, message: String::from("Dangling Combinator"), location },
            selectors::parser::SelectorParseErrorKind::NonCompoundSelector => ParseError{ kind: ParseErrorKind::InvalidSelectors, message: String::from("Non-compound Selector"), location },
            _ => ParseError{ kind: ParseErrorKind::InvalidSelectors, message: String::from("Selectors failed to parse"), location },
        }
    }
}
