use rl_model::model::{skill, SkillId, SkillsetId};

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
    UnresolvedSkillCall(String, String, Vec<Expr>),
    //
    Nary(NaryOperator, Vec<Expr>),
    //
    Composition(CompositionOperator, Vec<Expr>),
    //
    IfThenElse(Box<Expr>, Box<Expr>, Vec<(Expr, Expr)>, Box<Expr>),
    //
    // RobotRef(RobotId, SkillsetId),
    FunctionCall(FunctionId, Vec<Expr>),
    SkillCall(SkillsetId, SkillId, Vec<Expr>),
}

impl Expression {
    pub fn get_type(&self) -> Type {
        match self {
            Expression::Unresolved(_) => todo!(),
            Expression::UnresolvedFunctionCall(_, _) => todo!(),
            Expression::Nary(_, _) => todo!(),
            Expression::Composition(_, _) => todo!(),
            Expression::IfThenElse(_, _, _, _) => todo!(),
            Expression::FunctionCall(_, _) => todo!(),
            Expression::UnresolvedSkillCall(_, _, _) => todo!(),
            Expression::SkillCall(_, _, _) => todo!(),
        }
    }

    pub fn check_type(&self) -> Result<(), RlcError> {
        match self {
            Expression::Unresolved(_) => todo!(),
            Expression::UnresolvedFunctionCall(_, _) => todo!(),
            Expression::Nary(_, _) => todo!(),
            Expression::Composition(_, _) => todo!(),
            Expression::IfThenElse(_, _, _, _) => todo!(),
            Expression::FunctionCall(_, _) => todo!(),
            Expression::UnresolvedSkillCall(_, _, _) => todo!(),
            Expression::SkillCall(_, _, _) => todo!(),
        }
    }
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
            Expression::FunctionCall(id, params) => {
                let func = model.get_function(id.clone()).expect("ERROR");
                let mut s = format!("{}(", func.name());
                if let Some((first, others)) = params.split_first() {
                    s += &first.to_lang(model);
                    for x in others.iter() {
                        s += &format!(", {}", x.to_lang(model));
                    }
                }
                s + ")"
            }
            Expression::UnresolvedSkillCall(skillset, skill, params) => {
                let mut s = format!("{}.{}?(", skillset, skill);
                if let Some((first, others)) = params.split_first() {
                    s += &first.to_lang(model);
                    for x in others.iter() {
                        s += &format!(", {}", x.to_lang(model));
                    }
                }
                s + ")"
            }
            Expression::SkillCall(skillset_id, skill_id, params) => {
                let skillset = model
                    .rl_model
                    .get_skillset(skillset_id.clone())
                    .expect("ERROR");
                let skill = skillset.get_skill(skill_id.clone()).expect("ERROR");
                let mut s = format!("{}.{}(", skillset.to_string(), skill.to_string());
                if let Some((first, others)) = params.split_first() {
                    s += &first.to_lang(model);
                    for x in others.iter() {
                        s += &format!(", {}", x.to_lang(model));
                    }
                }
                s + ")"
            }
        }
    }
}
