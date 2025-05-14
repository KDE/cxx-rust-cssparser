// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::property::Property;
use crate::selector::Selector;

use crate::details::rulesparser::ParsedRule;

#[derive(Clone, Debug, PartialEq)]
pub struct StyleRule {
    pub selector: Selector,
    pub properties: Vec<Property>,
}

impl StyleRule {
    pub fn from_parsed_rule(parsed: &ParsedRule) -> Vec<StyleRule> {
        let mut result = Vec::new();

        for selector in &parsed.selectors {
            if selector.parts.is_empty() && parsed.properties.is_empty() {
                continue;
            }

            result.push(StyleRule {
                selector: selector.clone(),
                properties: parsed.properties.clone(),
            });

            for nested_rule in &parsed.nested_rules {
                for nested_result in StyleRule::from_parsed_rule(nested_rule) {
                    result.push(Self {
                        selector: Selector::combine(&nested_result.selector, &selector),
                        properties: nested_result.properties,
                    });
                }
            }
        }

        result
    }
}
