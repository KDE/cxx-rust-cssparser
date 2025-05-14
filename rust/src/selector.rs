// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectorKind {
    Unknown,
    AnyElement,
    Type,
    Class,
    Id,
    PseudoClass,
    Attribute,
    RelativeParent,
    DocumentRoot,
    DescendantCombinator,
    ChildCombinator,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectorPart {
    pub kind: SelectorKind,
    pub value: Value,
}

impl SelectorPart {
    pub fn new_with_empty(kind: SelectorKind) -> SelectorPart {
        SelectorPart { kind, value: Value::empty() }
    }

    pub fn new_with_value(kind: SelectorKind, value: Value) -> SelectorPart {
        SelectorPart { kind, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl Selector {
    pub fn new() -> Selector {
        Selector { parts: Vec::new() }
    }

    pub fn from_parts(parts: &[SelectorPart]) -> Selector {
        Selector {
            parts: Vec::from(parts),
        }
    }

    pub fn combine(first: &Selector, second: &Selector) -> Selector {
        let mut parts = first.parts.clone();

        let second_parts = second.parts.clone();
        if !second_parts.is_empty() {
            let parent = parts.iter().position(|part| part.kind == SelectorKind::RelativeParent);
            if let Some(index) = parent {
                parts.remove(index);
                parts.splice(index..index, second_parts);
            } else {
                parts.extend(second_parts);
            }
        }

        Selector { parts }
    }

    pub fn push_part(&mut self, kind: SelectorKind, value: Value) {
        self.parts.push(SelectorPart::new_with_value(kind, value));
    }
}
