// SPDX-License-Identifier: LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL
// SPDX-FileCopyrightText: 2025 Arjen Hiemstra <ahiemstra@heimr.nl>

use crate::selector::{Selector, SelectorPart, SelectorKind};
use crate::value::Value;

use crate::details::ParseError;
use crate::details::identifier::Identifier;

use selectors::SelectorList;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PseudoElement(String);

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = SelectorImpl;
}

impl cssparser::ToCss for PseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
    W: std::fmt::Write {
        dest.write_str(self.0.as_str())
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PseudoClass(String);

impl selectors::parser::NonTSPseudoClass for PseudoClass {
    type Impl = SelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        return false;
    }

    fn is_user_action_state(&self) -> bool {
        return false;
    }

    fn visit<V>(&self, _visitor: &mut V) -> bool
    where
        V: selectors::parser::SelectorVisitor<Impl = Self::Impl>, {
        return true;
    }
}

impl cssparser::ToCss for PseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write {
        dest.write_str(self.0.as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectorImpl;

impl ::selectors::SelectorImpl for SelectorImpl {
    type PseudoElement = PseudoElement;
    type NonTSPseudoClass = PseudoClass;

    type Identifier = Identifier;
    type AttrValue = Identifier;
    type LocalName = Identifier;
    type NamespacePrefix = Identifier;
    type NamespaceUrl = Identifier;

    type BorrowedLocalName = String;
    type BorrowedNamespaceUrl = String;

    type ExtraMatchingData<'a> = ();
}

pub enum ParseRelative {
    No,
    Nested,
}

pub struct SelectorParser;

impl SelectorParser {
    pub fn parse<'i, 't>(&self, parser: &mut cssparser::Parser<'i, 't>, relative: ParseRelative) -> Result<Vec<Selector>, cssparser::ParseError<'i, ParseError>> {
        // let relative_selectors = match parent_selector {
        //     None => selectors::parser::ParseRelative::No,
        //     Some(_) => selectors::parser::ParseRelative::ForNesting,
        // };
        let relative_selectors = match relative {
            ParseRelative::No => selectors::parser::ParseRelative::No,
            ParseRelative::Nested => selectors::parser::ParseRelative::ForNesting,
        };
        let result = SelectorList::parse(self, parser, relative_selectors);

        if let Err(error) = result {
            println!("Error parsing selector: {:#?}", error);
            return Err(cssparser::ParseError{kind: cssparser::ParseErrorKind::Custom(ParseError::from(error)), location: parser.current_source_location()})
        }

        let mut selectors = Vec::new();
        for entry in result.unwrap().slice() {
            let mut selector = Selector::new();
            for part in entry.iter_raw_parse_order_from(0) {
                match part {
                    selectors::parser::Component::LocalName(local_name) => selector.push_part(SelectorKind::Type, Value::from(&local_name.name)),
                    selectors::parser::Component::ID(name) => selector.push_part(SelectorKind::Id, Value::from(name)),
                    selectors::parser::Component::Class(name) => selector.push_part(SelectorKind::Class, Value::from(name)),
                    selectors::parser::Component::NonTSPseudoClass(pseudo_class) => selector.push_part(SelectorKind::PseudoClass, Value::from(pseudo_class.0.as_str())),
                    selectors::parser::Component::RelativeSelectorAnchor => println!("relative"), //selector.push_part(SelectorKind::)
                    selectors::parser::Component::Scope => println!("scope"),
                    selectors::parser::Component::ImplicitScope => println!("ImplicitScope"),
                    selectors::parser::Component::ParentSelector => selector.push_part(SelectorKind::RelativeParent, Value::empty()),

                    selectors::parser::Component::Combinator(combinator) => {
                        match combinator {
                            selectors::parser::Combinator::Descendant => selector.parts.push(SelectorPart::new_with_empty(SelectorKind::DescendantCombinator)),
                            selectors::parser::Combinator::Child => selector.parts.push(SelectorPart::new_with_empty(SelectorKind::ChildCombinator)),
                            _ => println!("Warning: Combinator {:#?} not implemented", combinator),
                        }
                    }
                    _ => println!("Warning: Selector part {:#?} not implemented", part),
                }
            }

            selectors.push(selector);
        }

        return Ok(selectors);
    }
}

impl <'i> ::selectors::Parser<'i> for SelectorParser {
    type Impl = SelectorImpl;
    type Error = ::selectors::parser::SelectorParseErrorKind<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        _location: cssparser::SourceLocation,
        name: cssparser::CowRcStr<'i>,
    ) -> Result<<Self::Impl as selectors::SelectorImpl>::NonTSPseudoClass, cssparser::ParseError<'i, Self::Error>> {
        Ok(PseudoClass(name.to_string()))
    }

    fn parse_pseudo_element(
        &self,
       _location: cssparser::SourceLocation,
        name: cssparser::CowRcStr<'i>,
    ) -> Result<<Self::Impl as selectors::SelectorImpl>::PseudoElement, cssparser::ParseError<'i, Self::Error>> {
        Ok(PseudoElement(name.to_string()))
    }

    fn parse_parent_selector(&self) -> bool {
        true
    }
}
