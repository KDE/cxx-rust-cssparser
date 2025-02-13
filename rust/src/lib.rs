
// struct CssParser {
//
// }

pub mod identifier;
pub mod selectors;
pub mod parser;
pub mod types;

use parser::Parser;
use types::*;

#[cxx::bridge(namespace = "cssparser::rust")]
mod ffi {
    pub enum ValueType {
        Empty,
        String,
        Number,
    }

    #[derive(Debug)]
    pub enum SelectorKind {
        Unknown,
        Type,
        Class,
        Id,
        PseudoClass,
        Attribute,
        DescendantCombinator,
        ChildCombinator,
    }

    extern "Rust" {
        type Value;
        type SelectorPart;
        type Selector;
        type Property;
        type CssRule;

        fn parse(source: &str) -> Vec<CssRule>;

        fn properties(self: &CssRule) -> Vec<Property>;
        fn selectors(self: &CssRule) -> Vec<Selector>;

        fn name(self: &Property) -> &str;
        fn value(self: &Property) -> &Value;

        fn parts(self: &Selector) -> Vec<SelectorPart>;

        fn kind(self: &SelectorPart) -> SelectorKind;
        fn value(self: &SelectorPart) -> &Value;

        fn value_type(self: &Value) -> ValueType;
        fn to_string(self: &Value) -> Result<&str>;
        fn to_number(self: &Value) -> Result<f32>;
    }
}

fn parse(source: &str) -> Vec<CssRule>
{
    Parser::parse(source)
}
