use super::*;
use crate::parser::Position;
use rl_model::model::Named as RlNamed;

#[derive(Clone, Debug)]
pub struct Expr {
    expression: Expression,
    position: Option<Position>,
}

impl Expr {
    pub fn new(expression: Expression, position: Option<Position>) -> Self {
        Self {
            expression,
            position,
        }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

    //

    pub fn new_unresolved<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        Self {
            expression: Expression::Unresolved(name.into()),
            position,
        }
    }

    pub fn resolve_expression(&mut self, model: &Model) -> Result<(), RlcError> {
        match self.expression.clone() {
            Expression::Unresolved(_) => {
                print!("resolving: {}", self.expression.to_lang(model))
            }
            Expression::UnresolvedFunctionCall(name, params) => {
                print!("  - resolv fn call\n");
                for func in model.functions().iter() {
                    if name == func.name() {
                        print!("  - found match with {}\n", func.name());
                        self.expression = Expression::FunctionCall(func.id(), params.to_vec());
                        // TODO: params check
                        // for p1 in params.iter_mut() {
                        //     for p2 in func.parameters().iter() {
                        //         if p1 != p2 {
                        //             return Err(RlcError::Resolve {
                        //                 element: format!("type '{}'", name),
                        //                 position: pos.clone(),
                        //             });
                        //         }
                        //     }
                        // }
                    }
                }
                match self.expression.clone() {
                    Expression::UnresolvedFunctionCall(name, _params) => {
                        return Err(RlcError::Resolve {
                            element: format!("function '{}'", name),
                            position: self.position.clone(),
                        });
                    }
                    _ => (),
                }
            }
            Expression::UnresolvedSkillCall(skillset, skill, params) => {
                print!("  - resolv skill call\n");
                // FALSE FOR NOW
                // Should compare with parent function parameters, not skillset names
                for set in model.rl_model.skillsets().iter() {
                    if skillset == set.to_string() {
                        print!(" -- found matching skillset {}", set.to_string());
                        for s in set.skills().iter() {
                            if skill == s.name() {
                                self.expression = Expression::SkillCall(
                                    RlNamed::id(set),
                                    RlNamed::id(s),
                                    params.clone(),
                                );
                            }
                        }
                        match self.expression.clone() {
                            Expression::UnresolvedSkillCall(skillset, skill, _params) => {
                                return Err(RlcError::Resolve {
                                    element: format!(
                                        "skill '{}' not part of skillset '{}'",
                                        skill, skillset
                                    ),
                                    position: self.position.clone(),
                                });
                            }
                            _ => (),
                        }
                    }
                }
                match self.expression.clone() {
                    Expression::UnresolvedSkillCall(skillset, _skill, _params) => {
                        return Err(RlcError::Resolve {
                            element: format!("skillset '{}'", skillset),
                            position: self.position.clone(),
                        });
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        Ok(())
    }
}

impl ToLang for Expr {
    fn to_lang(&self, model: &Model) -> String {
        self.expression.to_lang(model)
    }
}
