use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct DeclMethodId(pub usize);

impl Default for DeclMethodId {
    fn default() -> Self {
        Self(0)
    }
}

impl Id for DeclMethodId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct DeclMethod {
    id: DeclMethodId,
    name: String,
    parameters: Vec<Parameter>,
    position: Option<Position>,
}

impl DeclMethod {
    pub fn new<S: Into<String>>(
        name: S,
        parameters: Vec<Parameter>,
        position: Option<Position>,
    ) -> Self {
        Self {
            id: Default::default(),
            name: name.into(),
            parameters,
            position,
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }
}

impl Named<DeclMethodId> for DeclMethod {
    fn id(&self) -> DeclMethodId {
        self.id
    }

    fn set_id(&mut self, id: DeclMethodId) {
        self.id = id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for DeclMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLang for DeclMethod {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("decl {}(", self.name);
        if let Some((first, others)) = self.parameters.split_first() {
            s += &first.to_lang(model);
            for x in others.iter() {
                s += &format!(", {}", x.to_lang(model))
            }
        }
        s += ")\n";
        s
    }
}
