// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use super::propertysyntax::ParsedPropertySyntax;
use super::ParseError;
use crate::parse_error;

use crate::details::propertysyntax::{parse_syntax, parse_values};
use crate::property::PropertyDefinition;

struct PropertyDefinitionParser {
    definition: PropertyDefinition,
}

impl<'i> cssparser::AtRuleParser<'i> for PropertyDefinitionParser {
    type Prelude = ();
    type AtRule = ();
    type Error = ParseError;
}

impl<'i> cssparser::QualifiedRuleParser<'i> for PropertyDefinitionParser {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ParseError;
}

impl<'i> cssparser::DeclarationParser<'i> for PropertyDefinitionParser {
    type Declaration = ();
    type Error = ParseError;

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        match name.to_lowercase().as_str() {
            "syntax" => {
                let syntax_string = input.expect_string()?.to_string();
                // println!("syntax string {:#?}", syntax_string);

                let parsed = parse_syntax(syntax_string.as_str());
                if let Ok(syntax) = parsed {
                    self.definition.syntax = syntax;
                } else {
                    return parse_error!(input, InvalidPropertyDefinition, String::from("Expected string for property syntax"));
                }
            },
            "inherits" => {
                let value_string = input.expect_ident()?.to_string();

                match value_string.to_lowercase().as_str() {
                    "true" => self.definition.inherit = true,
                    "false" => self.definition.inherit = false,
                    _ => return parse_error!(input, InvalidPropertyDefinition, String::from("Unexpected value for inherit")),
                }
            },
            "initial-value" => {
                let value_result = parse_values(&self.definition.syntax, input);
                if let Ok(value) = value_result {
                    self.definition.initial = value;
                } else {
                    return Err(value_result.err().unwrap())
                }
            }
            _ => (),
        }

        if self.definition.name.is_empty() {
            parse_error!(input, InvalidPropertyDefinition, String::from("name is required for property definitions"))
        } else if let ParsedPropertySyntax::Empty = self.definition.syntax {
            parse_error!(input, InvalidPropertyDefinition, String::from("syntax is required for property definitions"))
        } else if !input.is_exhausted() {
            Err(input.new_custom_error(ParseError::InvalidPropertyDefinition(String::from("Unexpected trailing characters"))))
        } else {
            Ok(())
        }
    }
}

impl<'i> cssparser::RuleBodyItemParser<'i, (), ParseError> for PropertyDefinitionParser {
    fn parse_qualified(&self) -> bool {
        false
    }

    fn parse_declarations(&self) -> bool {
        true
    }
}

pub fn parse_property_definition<'i, 't>(
    input: &mut cssparser::Parser<'i, 't>,
    name: String,
) -> Result<PropertyDefinition, cssparser::ParseError<'i, ParseError>> {
    let mut parser = PropertyDefinitionParser{
        definition: PropertyDefinition::empty(),
    };
    parser.definition.name = name;
    let mut rule_parser = cssparser::RuleBodyParser::new(input, &mut parser);

    while let Some(item) = rule_parser.next() {
        if let Err(error) = item {
            return Err(error.0)
        }
    }

    Ok(parser.definition)
}
