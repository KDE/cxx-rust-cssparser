use cssparser::{
    Parser as CssParser,
    ParserInput,
    ParseError,
    ParserState,
    Token,
    QualifiedRuleParser,
    AtRuleParser,
    StyleSheetParser,
};
use selectors::parser::{Combinator, Component, ParseRelative, SelectorParseErrorKind};

use crate::selectors::SelectorParser;
use crate::ffi::SelectorKind;
use crate::types::*;

// #[derive(Debug, Default)]
// pub struct Property {
//     pub name: String,
//     pub value: Value,
// }

// #[derive(Debug)]
// pub struct CssRule {
//     pub selectors: Vec<Selector>,
//     pub properties: Vec<Property>,
// }

struct RulesParser;

impl<'i> QualifiedRuleParser<'i> for RulesParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = CssRule;
    type Error = SelectorParseErrorKind<'i>;

    fn parse_prelude<'t>(&mut self, parser: &mut CssParser<'i, 't>) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let selector_parser = SelectorParser{};
        let result = selectors::SelectorList::parse(&selector_parser, parser, ParseRelative::No);

        let mut selectors = Vec::new();
        for entry in result.clone().unwrap().slice() {
            let mut selector = Selector::new();
            for part in entry.iter_raw_parse_order_from(0) {
                match part {
                    Component::LocalName(local_name) => selector.parts.push(SelectorPart::new(SelectorKind::Type, Value::from(local_name.name.to_string()))),
                    Component::ID(name) => selector.parts.push(SelectorPart::new(SelectorKind::Id, Value::from(name.to_string()))),
                    Component::Class(name) => selector.parts.push(SelectorPart::new(SelectorKind::Class, Value::from(name.to_string()))),
                    Component::NonTSPseudoClass(pseudo_class) => selector.parts.push(SelectorPart::new(SelectorKind::PseudoClass, Value::from(pseudo_class.to_string()))),

                    Component::Combinator(combinator) => {
                        match combinator {
                            Combinator::Descendant => selector.parts.push(SelectorPart::new(SelectorKind::DescendantCombinator, Value::empty())),
                            Combinator::Child => selector.parts.push(SelectorPart::new(SelectorKind::ChildCombinator, Value::empty())),
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }

            selectors.push(selector);
        }

        return Ok(selectors);
    }

    fn parse_block<'t>(&mut self, prelude: Self::Prelude, _location: &ParserState, parser: &mut CssParser<'i, 't>) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut properties = Vec::new();
        while !parser.is_exhausted() {
            // println!("{:#?}", parser.next());
            let mut tokens = Vec::new();

            let mut token = parser.next().unwrap();
            while *token != Token::Semicolon {
                tokens.push(token.clone());
                token = parser.next().unwrap();
            }

            let mut p = Property::default();

            let mut iter = tokens.iter();

            match &iter.next().unwrap() {
                Token::Ident(name) => p.name = name.to_string(),
                _ => continue,
            }

            match &iter.next().unwrap() {
                Token::Colon => (),
                _ => continue,
            }

            match &iter.next().unwrap() {
                Token::Ident(value) => p.value = Value::from(value.to_string()),
                Token::Hash(value) => p.value = Value::from(value.to_string()),
                Token::IDHash(value) => p.value = Value::from(value.to_string()),
                Token::QuotedString(value) => p.value = Value::from(value.to_string()),
                Token::Number{has_sign: _, value, int_value: _} => p.value = Value::from(*value),
                Token::Dimension{has_sign: _, value, int_value: _, unit: _} => p.value = Value::from(*value),
                // value => println!("{:#?}", value),
                _ => p.value = Value::empty(),
            }

            // if !matches!(tokens[0], Token::Ident(_)) {
            //     continue;
            // }
            //
            // let Token::Ident(name) = tokens[0];
            // p.name = tokens[0].;
            // let token = parser.next();
            // p.name = token.unwrap();

            properties.push(p);
        }
        Ok(CssRule {
            selectors: prelude,
            properties,
        })
    }
}

impl<'i> AtRuleParser<'i> for RulesParser {
    type Prelude = ();
    type AtRule = CssRule;
    type Error = SelectorParseErrorKind<'i>;
}


pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Vec<CssRule> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = CssParser::new(&mut parser_input);
        let mut rules_parser = RulesParser{};
        let style_sheet_parser = StyleSheetParser::new(&mut parser, &mut rules_parser);

        let mut result = Vec::new();
        for entry in style_sheet_parser {
            match entry {
                Ok(rule) => result.push(rule),
                Err(error) => println!("{:#?}",error),
            }
        }
        result
    }
}
