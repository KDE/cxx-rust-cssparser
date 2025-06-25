// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::details::property::function::property_function;
use crate::value::{Color, Value};

fn check_value(input: &str, expected: Vec<Value>) {
    let mut parser_input = cssparser::ParserInput::new(input);
    let mut parser = cssparser::Parser::new(&mut parser_input);

    let function_name = parser.expect_function().unwrap().as_ref();
    let function = property_function(function_name).unwrap();

    let result = parser.parse_nested_block(|parser| {
        let output = function(parser);
        if let Ok(output_ok) = output {
            Ok(output_ok)
        } else {
            return output;
        }
    });

    match result {
        Ok(values) => assert_eq!(values, expected),
        Err(error) => panic!("{}", error),
    }
}

test_cases! {
    mix:
        check_value "mix(black, white, 0.5)", vec![
            Value::from(Color { r: 127, g: 127, b: 127, a: 255 })
        ];
    mix_alpha:
        check_value "mix(rgba(255, 0, 255, 0.25), rgba(255, 255, 0, 0.75), 0.25)", vec![
            Value::from(Color { r: 255, g: 63, b: 191, a: 95})
        ]
}
