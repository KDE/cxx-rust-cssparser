// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

// Implements the parts of cssparser that are required to parse things.

use cssparser::{CowRcStr, RuleBodyParser};

use crate::property::{Property, PropertyDefinition, property_definition};
use crate::selector::Selector;

use super::{parse_error, ParseError, ParseErrorKind};
use super::selectorparser::{SelectorParser, ParseRelative};
use super::propertysyntax::parse_values;
use super::propertyparser::parse_property_definition;

#[derive(Debug)]
pub struct ParsedRule {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
    pub nested_rules: Vec<Self>,
}

#[derive(Debug)]
pub enum ParseResult {
    Property(Property),
    Rule(ParsedRule),
    PropertyDefinition(PropertyDefinition),
    Import(String),
}

#[derive(Debug)]
pub enum AtRulePrelude {
    Property(String),
    Import(String),
}

pub struct RulesParser<const TOP_LEVEL: bool>;
pub type TopLevelParser = RulesParser<true>;
pub type NestedParser = RulesParser<false>;

impl<'i, const TOP_LEVEL: bool> cssparser::QualifiedRuleParser<'i> for RulesParser<TOP_LEVEL> {
    type Prelude = Vec<Selector>;
    type QualifiedRule = ParseResult;
    type Error = ParseError;

    fn parse_prelude<'t>(&mut self, parser: &mut cssparser::Parser<'i, 't>) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        let selector_parser = SelectorParser{};
        let relative = if TOP_LEVEL { ParseRelative::No } else { ParseRelative::Nested };
        let result = selector_parser.parse(parser, relative);
        if let Ok(selectors) = result {
            Ok(selectors)
        } else {
            parse_error(parser, ParseErrorKind::InvalidSelectors, result.err().unwrap().to_string())
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _location: &cssparser::ParserState,
        parser: &mut cssparser::Parser<'i, 't>) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>>
    {
        let mut nested_parser = NestedParser{};
        let mut body_parser = RuleBodyParser::<NestedParser, Self::QualifiedRule, Self::Error>::new(parser, &mut nested_parser);

        let mut properties = Vec::new();
        let mut nested = Vec::new();
        while let Some(entry) = body_parser.next() {
            if let Ok(entry_contents) = entry {
                if let ParseResult::Property(property) = entry_contents {
                    properties.push(property);
                } else if let ParseResult::Rule(rule) = entry_contents {
                    nested.push(rule);
                }
            }
        }

        Ok(ParseResult::Rule(ParsedRule {
            selectors: prelude,
            properties,
            nested_rules: nested,
        }))
    }
}

impl<'i, const TOP_LEVEL: bool> cssparser::AtRuleParser<'i> for RulesParser<TOP_LEVEL> {
    type Prelude = AtRulePrelude;
    type AtRule = ParseResult;
    type Error = ParseError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        let name_string = name.to_string();
        match name_string.as_str() {
            "property" => {
                return Ok(AtRulePrelude::Property(input.expect_ident()?.to_string()));
            },
            "import" => {
                let url = input.expect_url_or_string()?.to_string();
                return Ok(AtRulePrelude::Import(url));
            }
            _ => parse_error(input, ParseErrorKind::UnsupportedAtRule, format!("Unsupported @-rule {}", name)),
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::AtRule, cssparser::ParseError<'i, Self::Error>> {
        match prelude {
            AtRulePrelude::Property(name) => {
                let result = parse_property_definition(input, name.to_string());
                match result {
                    Ok(definition) => return Ok(ParseResult::PropertyDefinition(definition)),
                    Err(error) => return parse_error(input, ParseErrorKind::InvalidPropertyDefinition, error.to_string())
                }
            },
            _ => {
                return parse_error(input, ParseErrorKind::UnsupportedAtRule, format!("Got @-rule: {:?}", prelude));
            }
        }
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
    ) -> Result<Self::AtRule, ()> {
        match prelude {
            AtRulePrelude::Import(url) => {
                return Ok(ParseResult::Import(url))
            },
            _ => {
                return Err(())
            }
        }
    }
}

impl<'i, const TOP_LEVEL: bool> cssparser::DeclarationParser<'i> for RulesParser<TOP_LEVEL> {
    type Declaration = ParseResult;
    type Error = ParseError;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        let property_definition = property_definition(name.to_string().as_str());
        if property_definition.is_none() {
            return parse_error!(input, UnknownProperty, format!("No definition for property {}", name));
        }

        let pd = property_definition.unwrap();

        let values_result = parse_values(&pd.syntax, input);
        if let Ok(values) = values_result {
            Ok(ParseResult::Property(Property {
                name: name.to_string(),
                definition: pd.clone(),
                values,
            }))
        } else {
            Err(values_result.err().unwrap())
        }
    }
}

impl<'i, const TOP_LEVEL: bool> cssparser::RuleBodyItemParser<'i, ParseResult, ParseError>
    for RulesParser<TOP_LEVEL>
{
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}
