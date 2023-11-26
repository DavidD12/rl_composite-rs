use super::*;

#[derive(Debug, Clone)]
pub struct Robot {
    name: String,
    skillset_type: Type,
    position: Option<Position>,
}

impl Robot {
    pub fn new<S: Into<String>>(name: S, skillset_type: Type, position: Option<Position>) -> Self {
        Self {
            name: name.into(),
            skillset_type,
            position,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn skillset(&self) -> &Type {
        &self.skillset_type
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLang for Robot {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("robot {}:", self.name);
        s += &format!("{}", self.skillset_type.to_lang(model));
        s += "\n";
        s
    }
}
