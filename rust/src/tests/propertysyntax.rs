// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::details::propertysyntax::*;

fn check_syntax(input: &str, expected: ParsedPropertySyntax) {
    let result = parse_syntax(input);

    match result {
        Ok(syntax) => assert_eq!(syntax, expected),
        Err(error) => panic!("{}", error),
    }
}

test_cases! {
    single_datatype:
        check_syntax "<color>",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Single(
                SyntaxComponent::DataType(DataType::Color)
            )
        ]);

    space_separated_list:
        check_syntax "<length>+",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Single(
                SyntaxComponent::SpaceSeparatedList(DataType::Length)
            )
        ]);

    comma_separated_list:
        check_syntax "<url>#",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Single(
                SyntaxComponent::CommaSeparatedList(DataType::Url)
            )
        ]);

    multiple:
        check_syntax "<percentage> <angle>",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Single(
                SyntaxComponent::DataType(DataType::Percentage),
            ),
            SyntaxComponentAlternatives::Single(
                SyntaxComponent::DataType(DataType::Angle)
            )
        ]);

    keywords_list:
        check_syntax "one | two | three",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Alternatives(vec![
                SyntaxComponent::Keyword(String::from("one")),
                SyntaxComponent::Keyword(String::from("two")),
                SyntaxComponent::Keyword(String::from("three")),
            ])
        ]);

    any:
        check_syntax "*",
        ParsedPropertySyntax::Universal;

    type_or_keyword:
        check_syntax "<time> | auto",
        ParsedPropertySyntax::Components(vec![
            SyntaxComponentAlternatives::Alternatives(vec![
                SyntaxComponent::DataType(DataType::Time),
                SyntaxComponent::Keyword(String::from("auto"))
            ])
        ])
}

#[test]
fn invalid_datatype() {
    let result = parse_syntax("<invalid>");
    assert!(result.is_err());

    let result = parse_syntax("<invalid> | <length>");
    assert!(result.is_err());
}
