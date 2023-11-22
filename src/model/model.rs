use crate::parser::error::*;
use rl_model::model::Model as RlModel;

use super::*;

pub struct Model {
    pub rl_model: RlModel,
    includes: Vec<String>,
    functions: Vec<Function>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            rl_model: RlModel::default(),
            includes: vec![],
            functions: vec![],
        }
    }
}

impl Model {
    // ----- Include -----
    pub fn add_include<S: Into<String>>(&mut self, file: S) {
        self.includes.push(file.into());
    }

    pub fn includes(&self) -> &Vec<String> {
        &self.includes
    }

    // ----- Function -----

    pub fn add_function(&mut self, mut fun: Function) -> FunctionId {
        let id = FunctionId(self.functions.len());
        fun.set_id(id);
        self.functions.push(fun);
        id
    }

    pub fn get_function(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(id.index())
    }

    // ---------- ----------
    pub fn duplicate(&self) -> Result<(), RlcError> {
        // TODO: check duplicate RLC and with RL model
        //
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), RlcError> {
        // TODO
        Ok(())
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rl_model)?;
        writeln!(
            f,
            "\n//--------------------------------------------------\n"
        )?;
        for x in self.includes.iter() {
            writeln!(f, "include \"{}\"", x)?;
        }
        for x in self.functions.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<FunctionId, Function> for Model {
    fn get(&self, id: FunctionId) -> Option<&Function> {
        self.get_function(id)
    }
}