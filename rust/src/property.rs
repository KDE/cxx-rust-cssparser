// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::sync::{Arc, RwLock};

use crate::{
    details::property::syntax::{parse_syntax, ParsedPropertySyntax},
    parseerror::{ParseError, SourceLocation},
    value::Value
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PropertyDefinition {
    pub name: String,
    pub syntax: ParsedPropertySyntax,
    pub inherit: bool,
    pub initial: Vec<Value>,
}

/// Property definitions shared by a stylesheet and its imports.
#[derive(Debug, Default)]
pub struct PropertyDefinitions {
    definitions: RwLock<Vec<Arc<PropertyDefinition>>>,
}

impl PropertyDefinitions {
    pub fn get(&self, name: &str) -> Option<Arc<PropertyDefinition>> {
        if let Ok(definitions) = self.definitions.read() {
            return definitions
                .iter()
                .find(|definition| definition.name == name)
                .cloned();
        }
        None
    }

    pub fn add(&self, definition: PropertyDefinition) -> bool {
        if let Ok(mut definitions) = self.definitions.write() {
            if definitions
                .iter()
                .any(|existing| existing.name == definition.name)
            {
                return false;
            }
            definitions.push(Arc::new(definition));
            return true;
        }
        false
    }
}

impl PropertyDefinition {
    pub fn empty() -> PropertyDefinition {
        PropertyDefinition {
            name: String::new(),
            syntax: ParsedPropertySyntax::Empty,
            inherit: false,
            initial: Vec::new(),
        }
    }

    pub fn from_name_syntax(name: &str, syntax: &str, file: &str, line: u32, column: u32) -> Result<PropertyDefinition, ParseError> {
        let result = parse_syntax(syntax, SourceLocation { file: file.to_string(), line, column });
        if let Ok(parsed_syntax) = result {
            Ok(
                PropertyDefinition {
                    name: String::from(name),
                    syntax: parsed_syntax,
                    inherit: false,
                    initial: Vec::new(),
                }
            )
        } else {
            Err(result.err().unwrap())
        }
    }

    pub fn from_name_syntax_initial(name: &str, syntax: &str, initial: &[Value], file: &str, line: u32, column: u32) -> Result<PropertyDefinition, ParseError> {
        let mut pd = Self::from_name_syntax(name, syntax, file, line, column)?;
        pd.initial = Vec::from(initial);
        Ok(pd)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Property {
    pub name: String,
    pub definition: Arc<PropertyDefinition>,
    pub values: Vec<Value>,
}
