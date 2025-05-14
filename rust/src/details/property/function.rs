// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::sync::{RwLock, OnceLock};
use std::collections::hash_map::HashMap;

use crate::property::property_definition;
use crate::value::Value;

use crate::details::ParseError;

use super::syntax::ParsedPropertySyntax;
use super::value::parse_values;

pub type PropertyFunctionResult<'i> = Result<Vec<Value>, cssparser::ParseError<'i, ParseError>>;
pub type PropertyFunction = for <'a, 'i, 't> fn(&'a mut cssparser::Parser<'i, 't>) -> PropertyFunctionResult<'i>;

fn property_functions() -> &'static RwLock<HashMap<String, PropertyFunction>> {
    static FUNCTIONS: OnceLock<RwLock<HashMap<String, PropertyFunction>>> = OnceLock::new();
    FUNCTIONS.get_or_init(|| {
        let mut map: HashMap<String, PropertyFunction> = HashMap::new();
        map.insert(String::from("var"), var);
        RwLock::new(map)
    })
}

pub fn property_function(name: &str) -> Option<PropertyFunction> {
    if let Ok(functions) = property_functions().read() {
        if let Some(function) = functions.get(name) {
            return Some(*function);
        }
    }

    None{}
}

#[allow(dead_code)]
pub fn add_property_function(name: &str, function: PropertyFunction) -> bool {
    if let Ok(mut functions) = property_functions().write() {
        if functions.get(name).is_some() {
            return false;
        }

        functions.insert(name.to_string(), function);
    }

    true
}

// Parse `var(<custom-property-name>, <declaration-value>?)`
fn var<'i, 't>(parser: &mut cssparser::Parser<'i, 't>) -> PropertyFunctionResult<'i> {
    let var_name = parser.expect_ident()?.to_string();
    let property_definition = property_definition(var_name.as_str());
    if let Some(definition) = property_definition {
        return Ok(definition.initial.clone());
    }

    parser.expect_comma()?;
    parse_values(&ParsedPropertySyntax::Universal, parser)
}
