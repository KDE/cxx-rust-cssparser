use precomputed_hash::PrecomputedHash;
use cssparser::ToCss;

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Identifier(String);

impl PrecomputedHash for Identifier {
    fn precomputed_hash(&self) -> u32 {
        // let Identifier(contents) = self;
        return 0;
    }
}

impl ToCss for Identifier {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
    W: std::fmt::Write {
        let Identifier(contents) = self;
        return dest.write_str(contents);
    }
}

impl <'a> From<&'a str> for Identifier {
    fn from(value: &'a str) -> Self {
        return Identifier(value.to_string());
    }
}

impl std::borrow::Borrow<String> for Identifier {
    fn borrow(&self) -> &String {
        let Identifier(contents) = self;
        return contents;
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
