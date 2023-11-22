use super::*;

//-------------------------------------------------- Nary Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NaryOperator {
    And,
    Or,
}

impl NaryOperator {
    pub fn new(&self, v: Vec<Expr>) -> Expression {
        Expression::Nary(*self, v)
    }
}

impl std::fmt::Display for NaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}

//-------------------------------------------------- Composition --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CompositionOperator {
    Sequence,
    Choice,
    Parallel,
    Race,
}

impl CompositionOperator {
    pub fn new(&self, v: Vec<Expr>) -> Expression {
        Expression::Composition(*self, v)
    }
}

impl std::fmt::Display for CompositionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sequence => write!(f, ">"),
            Self::Choice => write!(f, "+"),
            Self::Parallel => write!(f, "|"),
            Self::Race => write!(f, "!"),
        }
    }
}

//-------------------------------------------------- Expression --------------------------------------------------

#[derive(Clone, Debug)]
pub enum Expression {
    Unresolved(String),
    UnresolvedFunctionCall(String, Vec<Expr>),
    //
    Nary(NaryOperator, Vec<Expr>),
    //
    Composition(CompositionOperator, Vec<Expr>),
    //
    IfThenElse(Box<Expr>, Box<Expr>, Vec<(Expr, Expr)>, Box<Expr>),
    //
    // RobotRef(RobotId, SkillsetId),
    FunctionCall(FunctionId, Vec<Expr>),
}

impl ToLang for Expression {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Expression::Unresolved(name) => format!("{}?", name),
            Expression::UnresolvedFunctionCall(name, params) => {
                let mut s = format!("{}?(", name);
                if let Some((first, others)) = params.split_first() {
                    s += &first.to_lang(model);
                    for x in others.iter() {
                        s += &format!(", {}", x.to_lang(model));
                    }
                }
                s + ")"
            }
            Expression::Nary(op, exprs) => {
                if let Some((first, others)) = exprs.split_first() {
                    format!(
                        "({})",
                        others.iter().fold(first.to_lang(model), |prev, x| format!(
                            "{} {} {}",
                            prev,
                            *op,
                            x.to_lang(model)
                        ))
                    )
                } else {
                    panic!("empty exprs in {:?}", self)
                }
            }
            Expression::Composition(op, exprs) => {
                if let Some((first, others)) = exprs.split_first() {
                    format!(
                        "({})",
                        others.iter().fold(first.to_lang(model), |prev, x| format!(
                            "{} {} {}",
                            prev,
                            *op,
                            x.to_lang(model)
                        ))
                    )
                } else {
                    panic!("empty exprs in {:?}", self)
                }
            }
            Expression::IfThenElse(c, t, l, e) => {
                let mut s = format!("if {} then {}", c.to_lang(model), t.to_lang(model));
                for (x, y) in l.iter() {
                    s.push_str(&format!(
                        " elif {} then {}",
                        x.to_lang(model),
                        y.to_lang(model)
                    ));
                }
                s.push_str(&format!(" else {} end", e.to_lang(model)));
                s
            }
            Expression::FunctionCall(_, _) => todo!(),
        }
    }
}
