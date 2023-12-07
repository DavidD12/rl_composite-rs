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

    fn res_function_call(
        &mut self,
        model: &Model,
        name: String,
        params: Vec<Expr>,
    ) -> Result<(), RlcError> {
        print!("  - resolv fn call\n");
        for func in model.functions().iter() {
            if name == func.name() {
                print!("  - found match with {}\n", func.name());
                let mut resolved_params: Vec<Expr> = params.clone();
                for p in resolved_params.iter_mut() {
                    p.resolve_expression(model)?;
                }
                self.expression = Expression::FunctionCall(func.id(), resolved_params);
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
        Ok(())
    }

    fn res_skill_call(
        &mut self,
        model: &Model,
        skillset: String,
        skill: String,
        params: Vec<Expr>,
    ) -> Result<(), RlcError> {
        print!("  - resolv skill call\n");
        // FALSE FOR NOW
        // Should compare with parent function parameters, not skillset names
        for set in model.rl_model.skillsets().iter() {
            if skillset == set.to_string() {
                print!(" -- found matching skillset {}\n", set.to_string());
                for s in set.skills().iter() {
                    let target_skillset = RlNamed::id(set);
                    let target_skill: rl_model::model::SkillId = RlNamed::id(s);
                    if skill == s.name() {
                        let mut resolved_params: Vec<Expr> = params.clone();
                        for p in resolved_params.iter_mut() {
                            p.resolve_expression(model)?;
                        }
                        self.expression =
                            Expression::SkillCall(target_skillset, target_skill, resolved_params);
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
        Ok(())
    }

    fn res_parameters(&mut self, model: &Model) -> Result<(), RlcError> {
        match self.expression.clone() {
            Expression::SkillCall(set_id, skill_id, params) => {
                // first resolve parameters expressions
                let mut resolved_params = params.clone();
                for p in resolved_params.iter_mut() {
                    p.resolve_expression(model)?;
                }
                let set = model.rl_model.get_skillset(set_id).expect("ERROR");
                let skill = set.get_skill(skill_id).expect("ERROR");
                for (i, input) in skill.inputs().iter().enumerate() {
                    print!("  -- input {}: {}\n", i.to_string(), input.name());
                    print!("  --- {}\n", params[i].expression().to_lang(model))
                }
            }
            _ => (),
        }
        Ok(())
    }

    pub fn resolve_expression(&mut self, model: &Model) -> Result<(), RlcError> {
        match self.expression.clone() {
            Expression::Unresolved(_) => {
                print!("resolving: {}\n", self.expression.to_lang(model))
                // is parent function parameter ?
                // is declared parameter ?
            }
            Expression::UnresolvedFunctionCall(name, params) => {
                self.res_function_call(model, name, params)?;
            }
            Expression::UnresolvedSkillCall(skillset, skill, params) => {
                self.res_skill_call(model, skillset, skill, params)?;
            }
            Expression::RosCall(topic, typ, params) => {
                let mut resolved_params: Vec<Expr> = params.clone();
                for p in resolved_params.iter_mut() {
                    p.resolve_expression(model)?;
                }
                self.expression = Expression::RosCall(topic, typ, resolved_params);
            }
            _ => (),
        }
        self.res_parameters(model)?;
        Ok(())
    }
}

impl ToLang for Expr {
    fn to_lang(&self, model: &Model) -> String {
        self.expression.to_lang(model)
    }
}
