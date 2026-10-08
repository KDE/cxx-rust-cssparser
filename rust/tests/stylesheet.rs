// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use std::path::PathBuf;

use cxx_rust_cssparser_impl::parseerror::{ParseError, ParseErrorKind, SourceLocation};
use cxx_rust_cssparser_impl::{
    property::{Property, PropertyDefinition},
    selector::*,
    stylerule::StyleRule,
    stylesheet::StyleSheet,
    value::{Color, Dimension, Value, Unit},
};

fn setup(stylesheet: &StyleSheet) {
    let definition =
        PropertyDefinition::from_name_syntax("test", "<color>", "Test Input", 0, 0).unwrap();
    stylesheet.definitions.add(definition);
}

#[test]
fn minimal() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());

    let result = stylesheet.parse_string("test { }");
    assert!(result.is_ok());

    assert_eq!(stylesheet.rules, Vec::from([
        StyleRule {
            selector: Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("test")),
            ]),
            properties: Vec::new(),
        }
    ]));
}

#[test]
fn property_registration() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    setup(&stylesheet);
    let property_definition = stylesheet.definitions.get("test").unwrap();

    let result = stylesheet.parse_string("example { test: red; }");
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    assert_eq!(
        stylesheet.rules,
        vec![
            StyleRule {
                selector: Selector::from_parts(&[
                    SelectorPart::new_with_value(SelectorKind::Type, Value::from("example"))
                ]),
                properties: vec![
                    Property {
                        name: String::from("test"),
                        definition: property_definition.clone(),
                        values: Vec::from([
                            Value::from(Color::rgba(255, 0, 0, 255))
                        ])
                    }
                ],
            }
        ]
    );
}

#[test]
fn custom_properties() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    setup(&stylesheet);

    let result = stylesheet.parse_string(
        ":root {
            --test-color: #ff0000;
            --test-length: 24px;
        }

        example {
            test: var(--test-color);
        }");
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    let color_definition = stylesheet.definitions.get("--test-color").unwrap();
    assert_eq!(*color_definition, PropertyDefinition::from_name_syntax_initial("--test-color", "*", &[Value::from(Color::rgba(255, 0, 0, 255))], "Test Input", 0, 0).unwrap());

    let length_definition = stylesheet.definitions.get("--test-length").unwrap();
    assert_eq!(*length_definition, PropertyDefinition::from_name_syntax_initial("--test-length", "*", &[Value::from(Dimension{value: 24.0, unit: Unit::Px})], "Test Input", 0, 0).unwrap());

    assert_eq!(
        stylesheet.rules,
        vec![
            StyleRule {
                selector: Selector::from_parts(&[
                    SelectorPart::new_with_empty(SelectorKind::DocumentRoot),
                ]),
                properties: Vec::new(),
            },
            StyleRule {
                selector: Selector::from_parts(&[
                    SelectorPart::new_with_value(SelectorKind::Type, Value::from("example")),
                ]),
                properties: vec![
                    Property {
                        name: String::from("test"),
                        definition: stylesheet.definitions.get("test").unwrap().clone(),
                        values: vec![
                            Value::from(Color::rgba(255, 0, 0, 255))
                        ]
                    }
                ]
            }
        ]
    );
}

#[test]
fn nested_block() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    setup(&stylesheet);
    let property_definition = stylesheet.definitions.get("test").unwrap();

    let result = stylesheet.parse_string(
        "example {
            test: red;

            nested {
                test: blue;
            }
        }");
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    let expected = Vec::from([
        StyleRule {
            selector: Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("example"))
            ]),
            properties: Vec::from([
                Property {
                    name: String::from("test"),
                    definition: property_definition.clone(),
                    values: Vec::from([
                        Value::from(Color::rgba(255, 0, 0, 255))
                    ]),
                }
            ]),
        },
        StyleRule {
            selector: Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("example")),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("nested")),
            ]),
            properties: Vec::from([
                Property {
                    name: String::from("test"),
                    definition: property_definition.clone(),
                    values: Vec::from([
                        Value::from(Color::rgba(0, 0, 255, 255))
                    ]),
                }
            ]),
        },
    ]);

    let rules = &stylesheet.rules;
    assert_eq!(rules.len(), expected.len());
    assert_eq!(rules, &expected);

    stylesheet = StyleSheet::new(PathBuf::new());
    setup(&stylesheet);
    let result = stylesheet.parse_string(
    "example {
        test: red;

        & nested {
            test: blue;
        }
    }");
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    let rules = &stylesheet.rules;
    assert_eq!(rules.len(), expected.len());
    assert_eq!(rules, &expected);
}

#[test]
fn complex() {
    let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/complex.css"));
    let mut stylesheet = StyleSheet::new(path.clone());

    let result = stylesheet.parse();
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    let rules = &stylesheet.rules;
    assert_eq!(rules.len(), 9);

    let expected_selectors = [
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
            SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
            SelectorPart {
                kind: SelectorKind::Attribute,
                value: SelectorValue::Attribute {
                    name: String::from("display"),
                             operator: AttributeOperator::Equals,
                             value: Value::from("something"),
                }
            },
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("toolbutton")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("toolbutton")),
            SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("toolbutton")),
            SelectorPart {
                kind: SelectorKind::Attribute,
                value: SelectorValue::Attribute {
                    name: String::from("display"),
                    operator: AttributeOperator::Equals,
                    value: Value::from("something"),
                }
            },
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
            SelectorPart::new_with_value(SelectorKind::Class, Value::from("primary")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
            SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("indicator")),
        ]),
        Selector::from_parts(&[
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("button")),
            SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
            SelectorPart::new_with_value(SelectorKind::Type, Value::from("indicator")),
            SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
        ]),
    ];

    let selectors: Vec<Selector> = rules.iter().map(|item| item.selector.clone()).collect();
    for (index, selector) in selectors.iter().enumerate() {
        assert_eq!(selector, &expected_selectors[index]);
    }

    let expected_properties = vec![
        Property {
            name: String::from("width"),
            definition: stylesheet.definitions.get("width").unwrap(),
            values: vec![Value::from(Dimension{value: 32.0, unit: Unit::Px})],
        },
        Property {
            name: String::from("height"),
            definition: stylesheet.definitions.get("height").unwrap(),
            values: vec![Value::from(Dimension{value: 32.0, unit: Unit::Px})],
        },
        Property {
            name: String::from("color"),
            definition: stylesheet.definitions.get("color").unwrap(),
            values: vec![Value::from(Color::rgba(255, 0, 0, 255))]
        },
        Property {
            name: String::from("padding"),
            definition: stylesheet.definitions.get("padding").unwrap(),
            values: vec![
                Value::from(Dimension{value: 4.0, unit: Unit::Px}),
                Value::from(Dimension{value: 4.0, unit: Unit::Px}),
                Value::from(Dimension{value: 4.0, unit: Unit::Px}),
                Value::from(Dimension{value: 4.0, unit: Unit::Px}),
            ]
        },
        Property {
            name: String::from("padding-top"),
            definition: stylesheet.definitions.get("padding-top").unwrap(),
            values: vec![
                Value::from(Dimension{value: 2.0, unit: Unit::Rem}),
            ]
        },
        Property {
            name: String::from("background-image"),
            definition: stylesheet.definitions.get("background-image").unwrap(),
            values: vec![
                Value::new_url(path.parent().unwrap().join("background.svg").to_string_lossy().as_ref()),
            ]
        }
    ];
    let properties: Vec<Property> = rules.first().unwrap().properties.clone();
    assert_eq!(properties, expected_properties);
}

#[test]
fn import() {
    let mut stylesheet = StyleSheet::new(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/import.css")));

    let result = stylesheet.parse();
    assert!(result.is_ok(), "Parsing stylesheet failed with error: {}", result.err().unwrap().to_string());

    assert_eq!(stylesheet.imported_sheets.len(), 2);

    let rules = stylesheet.all_rules();
    assert_eq!(rules.len(), 4);
}

#[test]
fn errors() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    let result = stylesheet.parse_string("this is not css");
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, vec![
        ParseError {
            kind: ParseErrorKind::UnexpectedEndOfInput,
            message: String::new(),
            location: SourceLocation {
                file: String::new(),
                line: 1,
                column: 16,
            },
        }
    ]);

    stylesheet = StyleSheet::new(PathBuf::new());
    let result = stylesheet.parse_string("example { unknown-property: somevalue; }");
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, vec![
        ParseError {
            kind: ParseErrorKind::UnknownProperty,
            message: String::from("No definition for property unknown-property"),
            location: SourceLocation {
                file: String::new(),
                line: 1,
                column: 28,
            }
        }
    ]);

    stylesheet = StyleSheet::new(PathBuf::new());
    setup(&stylesheet);
    let result = stylesheet.parse_string("example { test: url(somevalue); }");
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, vec![
        ParseError {
            kind: ParseErrorKind::InvalidPropertyValue,
            message: String::from("Parsing values for property test failed: Expected Color, got Url(somevalue)"),
            location: SourceLocation {
                file: String::new(),
                line: 1,
                column: 31,
            },
        }
    ]);

    stylesheet = StyleSheet::new(PathBuf::new());
    let result = stylesheet.parse_string("invalid - selector { }");
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, vec![
        ParseError {
            kind: ParseErrorKind::InvalidSelectors,
            message: String::from("Invalid Selectors: Dangling Combinator"),
            location: SourceLocation {
                file: String::new(),
                line: 1,
                column: 20
            }
        }
    ]);

    let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/file_with_errors.css"));
    let file_errors = vec![
        ParseError {
            kind: ParseErrorKind::UnknownProperty,
            message: String::from("No definition for property unknown-property"),
            location: SourceLocation {
                file: path.to_string_lossy().to_string(),
                line: 38,
                column: 22,
            }
        },
        ParseError {
            kind: ParseErrorKind::InvalidSelectors,
            message: String::from("Invalid Selectors: Dangling Combinator"),
            location: SourceLocation {
                file: path.to_string_lossy().to_string(),
                line: 41,
                column: 19,
            }
        },
        ParseError {
            kind: ParseErrorKind::InvalidPropertyValue,
            message: String::from("Parsing values for property padding-top failed: Expected Length, got String(value)"),
            location: SourceLocation {
                file: path.to_string_lossy().to_string(),
                line: 46,
                column: 23,
            }
        }
    ];

    let mut stylesheet = StyleSheet::new(path.clone());

    let result = stylesheet.parse();
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, file_errors);

    let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/import_errors.css"));
    let mut stylesheet = StyleSheet::new(path.clone());

    let result = stylesheet.parse();
    assert!(result.is_ok());
    assert_eq!(stylesheet.errors, vec![]);
    assert_eq!(stylesheet.all_errors(), file_errors);
}
