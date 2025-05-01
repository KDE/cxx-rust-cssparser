// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::recognize,
    character::complete::{char, satisfy, space0},
    error::ErrorKind,
    multi::{many0_count, many1},
    sequence::{delimited, pair, preceded, terminated},
};


use cssparser::color::{parse_hash_color, parse_named_color};

use super::ParseError;
use crate::{parse_error, value::{Color, Dimension, Value, Unit}};

struct SyntaxParseError<I>(I, String);

impl<I> std::fmt::Debug for SyntaxParseError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxParseError")
         .field("message", &self.1)
         .finish()
    }
}

impl<I> nom::error::ParseError<I> for SyntaxParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::Tag => SyntaxParseError(input, String::from("Input did not match a tag")),
            _ => SyntaxParseError(input, String::from(kind.description()))
        }
    }

    fn append(input: I, kind: ErrorKind, _other: Self) -> Self {
        match kind {
            ErrorKind::Alt => SyntaxParseError(input, String::from("Input matched none of the alternatives")),
            _ => SyntaxParseError(input, String::from("Multiple errors"))
        }
    }
}

type SyntaxParseResult<I, O> = IResult<I, O, SyntaxParseError<I>>;

fn make_error<I, O>(input: I, message: String) -> SyntaxParseResult<I, O> {
    Err(nom::Err::Error(SyntaxParseError(input, message)))
}

fn make_failure<I, O>(input: I, message: String) -> SyntaxParseResult<I, O> {
    Err(nom::Err::Failure(SyntaxParseError(input, message)))
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    CustomIdent,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxComponent {
    DataType(DataType),
    Keyword(String),
    SpaceSeparatedList(DataType),
    CommaSeparatedList(DataType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxComponentAlternatives {
    Single(SyntaxComponent),
    Alternatives(Vec<SyntaxComponent>),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum ParsedPropertySyntax {
    #[default] Empty,
    Universal,
    Components(Vec<SyntaxComponentAlternatives>),
}

fn is_custom_ident_start(input: char) -> bool {
    match input {
        'a'..='z' => true,
        'A'..='Z' => true,
        '_' => true,
        '\u{00B7}' => true,
        '\u{00CD}'..='\u{00D7}' => true,
        '\u{00D8}'..='\u{00F6}' => true,
        '\u{00F8}'..='\u{037D}' => true,
        '\u{037F}'..='\u{1FFF}' => true,
        '\u{200C}' => true,
        '\u{200D}' => true,
        '\u{203F}' => true,
        '\u{2040}' => true,
        '\u{2070}'..='\u{218F}' => true,
        '\u{2C00}'..='\u{2FEF}' => true,
        '\u{3001}'..='\u{D7FF}' => true,
        '\u{F900}'..='\u{FDCF}' => true,
        '\u{FDF0}'..='\u{FFFD}' => true,
        '\u{10000}'..='\u{10FFFF}' => true,
        _ => false,
    }
}

fn is_custom_ident(input: char) -> bool {
    if is_custom_ident_start(input) || input.is_numeric() || input == '-' {
        true
    } else {
        false
    }
}

fn keyword(input: &str) -> SyntaxParseResult<&str, SyntaxComponent> {
    let result = recognize(
        pair(
            satisfy(is_custom_ident_start),
            many0_count::<&str, SyntaxParseError<&str>, _>(satisfy(is_custom_ident))
        )
    ).parse(input);

    if let Ok((remain, keyword)) = result {
        Ok((remain, SyntaxComponent::Keyword(keyword.to_string())))
    } else {
        make_error(input, String::from("Input is not a keyword"))
    }
}

fn data_type_name(input: &str) -> SyntaxParseResult<&str, SyntaxComponent> {
    let result = delimited(
            char::<&str, SyntaxParseError<&str>>('<'),
            alt((
                tag("length-percentage"),
                tag("length"),
                tag("number"),
                tag("percentage"),
                tag("string"),
                tag("color"),
                tag("url"),
                tag("integer"),
                tag("angle"),
                tag("time"),
                tag("resolution"),
                tag("transform-function"),
                tag("custom-ident"),
            )),
            char('>')).parse(input);

    if let Ok((remain, name)) = result {
        let data_type = match name {
            "length-percentage" => DataType::LengthPercentage,
            "length" => DataType::Length,
            "number" => DataType::Number,
            "percentage" => DataType::Percentage,
            "string" => DataType::String,
            "color" => DataType::Color,
            "url" => DataType::Url,
            "integer" => DataType::Integer,
            "angle" => DataType::Angle,
            "time" => DataType::Time,
            "resolution" => DataType::Resolution,
            "transform-function" => DataType::TransformFunction,
            "custom-ident" => DataType::CustomIdent,
            _ => return make_failure(input, String::from("Invalid data type"))
        };
        Ok((remain, SyntaxComponent::DataType(data_type)))
    } else {
        make_error(input, String::from("Input is not a data type"))
    }
}

fn space_separated_list(input: &str) -> SyntaxParseResult<&str, SyntaxComponent> {
    let (remain, data_type_name) = terminated(data_type_name, char('+')).parse(input)?;
    if let SyntaxComponent::DataType(data_type) = data_type_name {
        Ok((remain, SyntaxComponent::SpaceSeparatedList(data_type)))
    } else {
        make_error(input, String::from("Input is not a space separated list"))
    }
}

fn comma_separated_list(input: &str) -> SyntaxParseResult<&str, SyntaxComponent> {
    let (remain, data_type_name) = terminated(data_type_name, char('#')).parse(input)?;
    if let SyntaxComponent::DataType(data_type) = data_type_name {
        Ok((remain, SyntaxComponent::CommaSeparatedList(data_type)))
    } else {
        make_error(input, String::from("Input is not a comma separated list"))
    }
}

fn syntax_component(input: &str) -> SyntaxParseResult<&str, SyntaxComponent> {
    let result = delimited(
        space0,
        alt((
            space_separated_list,
            comma_separated_list,
            data_type_name,
            keyword
        )),
        space0
    ).parse(input);

    if let Ok((remain, component)) = result {
        Ok((remain, component))
    } else {
        make_error(input, String::from("Input did not match any syntax component"))
    }
}

fn syntax_alternatives(input: &str) -> SyntaxParseResult<&str, SyntaxComponentAlternatives> {
    let alternatives = pair(syntax_component, many1(preceded(char('|'), syntax_component))).parse(input);
    if let Ok((remain, result)) = alternatives {
        let mut components = Vec::new();
        components.push(result.0);
        components.extend(result.1);
        return Ok((remain, SyntaxComponentAlternatives::Alternatives(components)));
    }

    let single = syntax_component(input);
    if let Ok((remain, component)) = single {
        return Ok((remain, SyntaxComponentAlternatives::Single(component)));
    }

    make_error(input, String::from("Input did not match a single syntax component or alternative"))
}

fn syntax_components(input: &str) -> SyntaxParseResult<&str, ParsedPropertySyntax> {
    let result = many1(syntax_alternatives).parse(input);

    if let Ok((remain, alternatives)) = result {
        Ok((remain, ParsedPropertySyntax::Components(alternatives)))
    } else {
        make_error(input, String::from("Input did not match a list of components"))
    }
}

fn universal(input: &str) -> SyntaxParseResult<&str, ParsedPropertySyntax> {
    let result = delimited(space0::<&str, SyntaxParseError<&str>>, char('*'), space0).parse(input);

    if let Ok((remain, _)) = result {
        Ok((remain, ParsedPropertySyntax::Universal))
    } else {
        make_error(input, String::from("Input did not match the universal syntax"))
    }
}

pub fn parse_syntax(input: &str) -> Result<ParsedPropertySyntax, ParseError> {
    let result = alt((
        universal,
        syntax_components,
    )).parse(input);

    if let Ok((_, syntax)) = result {
        Ok(syntax)
    } else {
        match result.err().unwrap() {
            nom::Err::Incomplete(_) => Err(ParseError::InvalidPropertySyntax(String::from("Incomplete input"))),
            nom::Err::Error(error) | nom::Err::Failure(error) => {
                let message = format!("Input {} encountered error: {}", error.0, error.1);
                Err(ParseError::InvalidPropertySyntax(message))
            }
        }
    }
}

fn parse_value_component<'i, 't>(component: &SyntaxComponent, parser: &mut cssparser::Parser<'i, 't>) ->Result<Vec<Value>, cssparser::ParseError<'i, ParseError>> {
    match component {
        SyntaxComponent::Keyword(keyword) => {
            if let Ok(_) = parser.expect_ident_matching(keyword) {
                return Ok(vec![Value::from(keyword.as_str())]);
            } else {
                return parse_error!(parser, InvalidPropertySyntax, format!("Expected keyword {}", keyword));
            }
        },
        SyntaxComponent::DataType(data_type) => {
            match data_type {
                DataType::Length => {
                    let token = parser.next()?.clone();
                    if let cssparser::Token::Dimension{has_sign: _, value, int_value: _, unit: unit_string} = token {
                        let unit = Unit::parse(unit_string.to_string().as_str());
                        match unit {
                            Unit::Px | Unit::Em | Unit::Rem | Unit::Pt => {
                                return Ok(vec![Value::from(Dimension{value, unit})]);
                            }
                            _ => {
                                return parse_error!(parser, InvalidPropertyValue, format!("Invalid unit for length: {}", unit_string));
                            }
                        }
                    } else {
                        return parse_error!(parser, InvalidPropertyValue, String::from("Expected a dimension"))
                    }
                },
                DataType::Number => {
                    let number = parser.expect_number()?;
                    return Ok(vec![Value::from(number)]);
                }
                DataType::Color => {
                    let token = parser.next()?.clone();
                    match token {
                        cssparser::Token::IDHash(value) => {
                            if let Ok(color) = parse_hash_color(value.as_bytes()) {
                                return Ok(vec![Value::from(Color::from(color))])
                            } else {
                                return parse_error!(parser, InvalidPropertyValue, format!("{} could not be parsed as color", value));
                            }
                        },
                        cssparser::Token::Hash(value) => {
                            if let Ok(color) = parse_hash_color(value.as_bytes()) {
                                return Ok(vec![Value::from(Color::from(color))])
                            } else {
                                return parse_error!(parser, InvalidPropertyValue, format!("{} could not be parsed as color", value));
                            }
                        },
                        cssparser::Token::Ident(value) => {
                            if let Ok(color) = parse_named_color(&value[..]) {
                                return Ok(vec![Value::from(Color::from(color))]);
                            } else {
                                return parse_error!(parser, InvalidPropertyValue, format!("{} could not be parsed as color", value));
                            }
                        }
                        _ => {
                            return parse_error!(parser, InvalidPropertyValue, format!("Unexpected token {:?}", token));
                        }
                    }
                },
                DataType::Integer => {
                    let integer = parser.expect_integer()?;
                    return Ok(vec![Value::from(integer)]);
                }
                _ => {
                    return parse_error!(parser, InvalidPropertyValue, format!("Data type {:?} is not supported", data_type));
                }
            }
        },
        SyntaxComponent::SpaceSeparatedList(data_type) => {
            let mut values: Vec<Value> = Vec::new();
            let component = SyntaxComponent::DataType(data_type.clone());
            while !parser.is_exhausted() {
                let value = parse_value_component(&component, parser)?;
                values.extend(value)
            }
            return Ok(values);
        },
        SyntaxComponent::CommaSeparatedList(data_type) => {
            let component = SyntaxComponent::DataType(data_type.clone());
            let result = parser.parse_comma_separated(|parser| {
                parse_value_component(&component, parser)
            });

            if let Ok(values) = result {
                return Ok(values.into_iter().flatten().collect())
            } else {
                return Err(result.unwrap_err())
            }
        },
    }
}

pub fn parse_values<'i, 't>(syntax: &ParsedPropertySyntax, parser: &mut cssparser::Parser<'i, 't>) -> Result<Vec<Value>, cssparser::ParseError<'i, ParseError>> {
    match syntax {
        ParsedPropertySyntax::Empty => parse_error!(parser, InvalidPropertyValue, String::from("Properties with empty values are unsupported")),
        ParsedPropertySyntax::Universal => parse_error!(parser, InvalidPropertyValue, String::from("Universal property syntax is unsupported")),
        ParsedPropertySyntax::Components(components) => {
            let mut it = components.iter();

            let mut values: Vec<Value> = Vec::new();
            while let Some(component_alternatives) = it.next() {
                match component_alternatives {
                    SyntaxComponentAlternatives::Single(component) => {
                        let result = parse_value_component(component, parser);
                        if let Ok(value) = result {
                            values.extend(value);
                        } else {
                            return Err(result.err().unwrap());
                        }
                    },
                    SyntaxComponentAlternatives::Alternatives(alternatives) => {
                        let parser_state = parser.state();
                        for component in alternatives {
                            let result = parse_value_component(component, parser);
                            if let Ok(value) = result {
                                values.extend(value);
                                break;
                            } else {
                                parser.reset(&parser_state);
                            }
                        }
                    }
                }
            }

            if !values.is_empty() {
                Ok(values)
            } else {
                parse_error!(parser, InvalidPropertyValue, String::from("No valid values were found"))
            }
        }
    }
}
