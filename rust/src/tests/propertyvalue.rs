// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::details::propertysyntax::{parse_syntax, parse_values};
use crate::value::{Color, Dimension, Value, Unit};

fn check_value(input: (&str, &str), expected: Vec<Value>) {
    let mut parser_input = cssparser::ParserInput::new(input.1);
    let mut parser = cssparser::Parser::new(&mut parser_input);

    let parsed_syntax = parse_syntax(input.0).unwrap();

    let result = parse_values(&parsed_syntax, &mut parser);
    match result {
        Ok(values) => assert_eq!(values, expected),
        Err(error) => panic!("{}", error),
    }
}

test_cases! {
    length_px:
        check_value ("<length>", "24px"), vec![
            Value::from(Dimension{value: 24.0, unit: Unit::Px})
        ];
    length_em:
        check_value ("<length>", "3em"), vec![
            Value::from(Dimension{value: 3.0, unit: Unit::Em})
        ];
    length_list:
        check_value ("<length>+", "1px 2px 3px"), vec![
            Value::from(Dimension{value: 1.0, unit: Unit::Px}),
            Value::from(Dimension{value: 2.0, unit: Unit::Px}),
            Value::from(Dimension{value: 3.0, unit: Unit::Px}),
        ];
    number:
        check_value ("<number>", "66.6"), vec![
            Value::from(66.6)
        ];
    color_hex:
        check_value ("<color>", "#ff0000"), vec![
            Value::from(Color{r: 255, g: 0, b: 0, a: 255})
        ];
    color_hex_short:
        check_value ("<color>", "#0f0"), vec![
            Value::from(Color{r: 0, g: 255, b: 0, a: 255})
        ];
    color_named:
        check_value ("<color>", "blue"), vec![
            Value::from(Color{r: 0, g: 0, b: 255, a: 255})
        ];
    color_comma_list:
        check_value ("<color>#", "red, green, blue"), vec![
            Value::from(Color{r: 255, g: 0, b: 0, a: 255}),
            Value::from(Color{r: 0, g: 128, b: 0, a: 255}),
            Value::from(Color{r: 0, g: 0, b: 255, a: 255}),
        ];
}
