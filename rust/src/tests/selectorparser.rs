// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::selector::*;
use crate::value::Value;
use crate::details::selectorparser::{SelectorParser, ParseRelative};

fn check_selector(input: &str, expected: Vec<Selector>, relative: ParseRelative) {
    let parser = SelectorParser{};

    let mut parser_input = cssparser::ParserInput::new(input);
    let mut css_parser = cssparser::Parser::new(&mut parser_input);

    let result = parser.parse(&mut css_parser, relative);
    assert_eq!(result.ok().unwrap(), expected)
}

fn check_selector_toplevel(input: &str, expected: Vec<Selector>) {
    check_selector(input, expected, ParseRelative::No);
}

fn check_selector_nested(input: &str, expected: Vec<Selector>) {
    check_selector(input, expected, ParseRelative::Nested);
}

test_cases! {
    simple:
        check_selector_toplevel "type", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
            ])
        ];

    descendant:
        check_selector_toplevel "type .class #id", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Id, Value::from("id")),
            ])
        ];

    child:
        check_selector_toplevel "type > .class", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
                SelectorPart::new_with_empty(SelectorKind::ChildCombinator),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
            ])
        ];

    pseudoclass:
        check_selector_toplevel "type:hovered", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
            ])
        ];

    multiple:
        check_selector_toplevel "type1 .class, type2 .class", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type1")),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
            ]),
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type2")),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
            ]),
        ];

    pseudoclass_child:
        check_selector_toplevel "type:hovered > .class", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
                SelectorPart::new_with_empty(SelectorKind::ChildCombinator),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
            ])
        ];

    nested:
        check_selector_nested "type", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_empty(SelectorKind::RelativeParent),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
            ])
        ];

    parent_descendant:
        check_selector_nested "& type", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_empty(SelectorKind::RelativeParent),
                SelectorPart::new_with_empty(SelectorKind::DescendantCombinator),
                SelectorPart::new_with_value(SelectorKind::Type, Value::from("type")),
            ])
        ];

    parent_same:
        check_selector_nested "&.class:hovered", vec![
            Selector::from_parts(&[
                SelectorPart::new_with_value(SelectorKind::PseudoClass, Value::from("hovered")),
                SelectorPart::new_with_value(SelectorKind::Class, Value::from("class")),
                SelectorPart::new_with_empty(SelectorKind::RelativeParent),
            ])
        ];
}
