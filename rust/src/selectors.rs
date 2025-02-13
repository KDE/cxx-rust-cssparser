use cssparser::*;
// use selectors::parser::SelectorParseErrorKind;

use crate::identifier::Identifier;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PseudoElement(String);

impl ::selectors::parser::PseudoElement for PseudoElement {
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

impl ::selectors::parser::NonTSPseudoClass for PseudoClass {
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

impl ToCss for PseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write {
        dest.write_str(self.0.as_str())
    }
}

impl std::fmt::Display for PseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

pub struct SelectorParser;

impl <'i> ::selectors::Parser<'i> for SelectorParser {
    type Impl = SelectorImpl;
    type Error = ::selectors::parser::SelectorParseErrorKind<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        _location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<<Self::Impl as selectors::SelectorImpl>::NonTSPseudoClass, ParseError<'i, Self::Error>> {
        Ok(PseudoClass(name.to_string()))
    }

    fn parse_pseudo_element(
        &self,
       _location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<<Self::Impl as selectors::SelectorImpl>::PseudoElement, ParseError<'i, Self::Error>> {
        Ok(PseudoElement(name.to_string()))
    }
}
