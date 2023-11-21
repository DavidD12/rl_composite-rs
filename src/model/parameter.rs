use super::*;
use crate::parser::Position;

//------------------------- Parameter -------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Parameter {
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Parameter {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let name = name.into();
        Self {
            name,
            typ,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_same(&self, other: &Parameter) -> bool {
        self.name == other.name && self.typ == other.typ
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLang for Parameter {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name, self.typ.to_lang(model))
    }
}
