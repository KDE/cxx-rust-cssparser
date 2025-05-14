// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::sync::Arc;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::details::{ParseError, ParseErrorKind, SourceLocation};
use crate::details::rulesparser::*;

use crate::property::add_property_definition;
use crate::stylerule::*;

#[derive(Debug)]
pub struct StyleSheet {
    pub rules: Vec<StyleRule>,
    pub errors: Vec<ParseError>,

    pub root_path: PathBuf,
}

impl StyleSheet {
    pub fn new() -> StyleSheet {
        StyleSheet {
            rules: Vec::new(),
            errors: Vec::new(),
            root_path: PathBuf::new(),
        }
    }

    pub fn parse_file(&mut self, file_name: &str) -> Result<(), ParseError> {
        let path = self.root_path.join(file_name);
        let file = File::open(&path);
        if let Err(error) = file {
            return Err(ParseError{ kind: ParseErrorKind::FileError, message: format!("{}", error), location: SourceLocation{ file: path.to_string_lossy().to_string(), line: 0, column: 0 } });
        }

        let mut data = String::new();
        let result = file.unwrap().read_to_string(&mut data);
        if let Err(error) = result {
            return Err(ParseError{ kind: ParseErrorKind::FileError, message: format!("{}", error), location: SourceLocation{ file: path.to_string_lossy().to_string(), line: 0, column: 0 } });
        }

        self.parse_string(data.as_str(), path.to_string_lossy().as_ref())
    }

    pub fn parse_string(&mut self, input: &str, origin: &str) -> Result<(), ParseError> {
        let prefix_input = format!("/*# sourceURL={} */\n{}", origin, input);
        let mut parser_input = cssparser::ParserInput::new(prefix_input.as_str());
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let mut rules_parser = TopLevelParser{};
        let style_sheet_parser = cssparser::StyleSheetParser::new(&mut parser, &mut rules_parser);

        let mut rules: Vec<StyleRule> = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();
        for entry in style_sheet_parser {
            match entry {
                Ok(entry_contents) => {
                    match entry_contents {
                        ParseResult::Rule(rule) => {
                            let mut parsed_rules = StyleRule::from_parsed_rule(&rule);
                            rules.append(&mut parsed_rules);
                        },
                        ParseResult::PropertyDefinition(definition) => {
                            let arc = Arc::new(definition);
                            add_property_definition(&arc);
                        },
                        ParseResult::Import(name) => {
                            self.parse_file(name.as_str())?;
                        }
                        ParseResult::Property(_) => {
                            panic!("Received property at toplevel!");
                        }
                    }
                }
                Err(error) => {
                    if let cssparser::ParseErrorKind::Custom(parse_error) = error.0.kind {
                        errors.push(parse_error)
                    } else {
                        panic!("Unexpected error type: {:#?}", error);
                    }
                }
            }
        }

        self.rules.extend(rules);
        self.errors.extend(errors);

        if self.errors.is_empty() {
            Ok(())
        } else {
            let mut message;
            if self.errors.len() == 1 {
                message = self.errors.first().unwrap().message.clone();
            } else {
                message = String::from("Multiple errors:\n");
                for error in &self.errors {
                    message.push_str(format!("{}\n", error).as_str());
                }
            }

            Err(ParseError { kind: ParseErrorKind::StyleSheetParseError, message, location: SourceLocation { file: origin.to_string(), line: 0, column: 0 } })
        }
    }
}
