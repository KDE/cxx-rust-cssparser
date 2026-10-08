// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2026 Ilya Bizyaev <me@ilyabiz.com>

use std::path::PathBuf;

use cxx_rust_cssparser_impl::{
    selector::{Selector, SelectorKind, SelectorPart},
    stylerule::StyleRule,
    stylesheet::StyleSheet,
    value::{Color, Dimension, Value},
};

fn collect_widths(rules: impl IntoIterator<Item = StyleRule>) -> Vec<Value> {
    rules
        .into_iter()
        .flat_map(|rule| rule.properties)
        .filter(|property| property.name == "width")
        .flat_map(|property| property.values)
        .collect()
}

fn widths(stylesheet: &StyleSheet) -> Vec<Value> {
    collect_widths(stylesheet.all_rules())
}

fn widths_for_selector(stylesheet: &StyleSheet, element: &str) -> Vec<Value> {
    let selector = Selector::from_parts(&[SelectorPart::new_with_value(
        SelectorKind::Type,
        Value::from(element),
    )]);
    collect_widths(
        stylesheet
            .all_rules()
            .into_iter()
            .filter(|rule| rule.selector == selector),
    )
}

fn parse_width(size: u32) -> StyleSheet {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    stylesheet
        .parse_string(&format!(
            "@property width {{ syntax: \"<length>\"; }}
         :root {{ --size: {size}px; }}
         button {{ width: var(--size); }}"
        ))
        .unwrap();
    stylesheet
}

#[test]
fn independent_stylesheets_keep_their_own_variables() {
    let mut first = parse_width(10);
    let second = parse_width(20);
    assert_eq!(widths(&first), vec![Value::from(Dimension::px(10.0))]);
    assert_eq!(widths(&second), vec![Value::from(Dimension::px(20.0))]);

    first
        .parse_string("button { width: var(--size); }")
        .unwrap();
    assert_eq!(widths(&first), vec![Value::from(Dimension::px(10.0)); 2]);
    drop(first);
    drop(second);
    assert_eq!(
        widths(&parse_width(30)),
        vec![Value::from(Dimension::px(30.0))]
    );
}

#[test]
fn independent_stylesheets_can_define_different_property_syntax() {
    let _first = parse_width(10);
    let mut second = StyleSheet::new(PathBuf::new());
    second
        .parse_string("@property width { syntax: \"<color>\"; } button { width: blue; }")
        .unwrap();
    assert_eq!(
        widths(&second),
        vec![Value::from(Color::rgba(0, 0, 255, 255))]
    );
}

#[test]
fn variables_from_other_stylesheets_do_not_suppress_fallbacks() {
    let _first = parse_width(10);
    let mut second = StyleSheet::new(PathBuf::new());
    second
        .parse_string(
            "@property width { syntax: \"<length>\"; } button { width: var(--size, 42px); }",
        )
        .unwrap();
    assert_eq!(widths(&second), vec![Value::from(Dimension::px(42.0))]);
}

#[test]
fn imports_share_definitions_with_their_parent_and_siblings() {
    let path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/property_definitions/parent.css"
    ));
    let mut stylesheet = StyleSheet::new(path);
    stylesheet
        .import(PathBuf::from("api-imported-definitions.css"))
        .unwrap();
    stylesheet.parse().unwrap();
    for (element, variable, expected_px) in [
        ("nested", "--small", 5.0),
        ("sibling", "--from-child", 12.0),
        ("button", "--from-child", 12.0),
    ] {
        assert_eq!(
            widths_for_selector(&stylesheet, element),
            vec![Value::from(Dimension::px(expected_px))],
            "{element} {{ width: var({variable}); }}"
        );
    }
}

#[test]
fn property_descriptors_resolve_variables() {
    let mut stylesheet = StyleSheet::new(PathBuf::new());
    stylesheet
        .parse_string(
            r#":root { --length-syntax: "<length>"; --size: 10px; }
               @property width {
                   syntax: var(--length-syntax);
                   initial-value: var(--size);
               }
               button { width: 20px; }"#,
        )
        .unwrap();

    assert_eq!(
        stylesheet.definitions.get("width").unwrap().initial,
        vec![Value::from(Dimension::px(10.0))]
    );
    assert_eq!(widths(&stylesheet), vec![Value::from(Dimension::px(20.0))]);
}

#[test]
fn nested_fallbacks_resolve_variables() {
    for (expression, expected_px) in [
        ("var(--missing, var(--size))", 10.0),
        ("var(--missing, var(--also-missing, 30px))", 30.0),
    ] {
        let mut stylesheet = StyleSheet::new(PathBuf::new());
        stylesheet
            .parse_string(&format!(
                r#"@property width {{ syntax: "<length>"; }}
                   :root {{ --size: 10px; }}
                   button {{ width: {expression}; }}"#,
            ))
            .unwrap();
        assert_eq!(
            widths(&stylesheet),
            vec![Value::from(Dimension::px(expected_px))],
            "{expression}"
        );
    }
}
