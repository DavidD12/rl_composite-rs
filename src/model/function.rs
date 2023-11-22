use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FunctionId(pub usize);

impl Default for FunctionId {
    fn default() -> Self {
        Self(0)
    }
}

impl Id for FunctionId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    id: FunctionId,
    name: String,
    parameters: Vec<Parameter>,
    expr: Expr,
    position: Option<Position>,
}

impl Function {
    pub fn new<S: Into<String>>(
        name: S,
        parameters: Vec<Parameter>,
        expr: Expr,
        position: Option<Position>,
    ) -> Self {
        Self {
            id: Default::default(),
            name: name.into(),
            parameters,
            expr,
            position,
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn set_expr(&mut self, expr: Expr) {
        self.expr = expr
    }
}

impl Named<FunctionId> for Function {
    fn id(&self) -> FunctionId {
        self.id
    }

    fn set_id(&mut self, id: FunctionId) {
        self.id = id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLang for Function {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("def {}(", self.name);
        if let Some((first, others)) = self.parameters.split_first() {
            s += &first.to_lang(model);
            for x in others.iter() {
                s += &format!(", {}", x.to_lang(model))
            }
        }
        s += ") {\n";
        s += &format!("  {}\n", self.expr.to_lang(model));
        s + "}\n"
    }
}
