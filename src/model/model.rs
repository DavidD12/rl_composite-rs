use crate::parser::error::*;
use rl_model::model::Model as RlModel;
use rl_model::model::RlType;

use super::*;

pub struct Model {
    pub rl_model: RlModel,
    includes: Vec<String>,
    types: Vec<RlType>,
    functions: Vec<Function>,
    declared_methods: Vec<DeclMethod>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            rl_model: RlModel::default(),
            includes: vec![],
            types: vec![],
            functions: vec![],
            declared_methods: vec![],
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

    // ----- Types -----
    pub fn add_type(&mut self, mut rl_type: RlType) {
        self.types.push(rl_type);
    }

    pub fn types(&self) -> &Vec<RlType> {
        &self.types
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

    // ----- Declared Methods -----

    pub fn add_declared_method(&mut self, mut decl: DeclMethod) -> DeclMethodId {
        let id = DeclMethodId(self.declared_methods.len());
        decl.set_id(id);
        self.declared_methods.push(decl);
        id
    }

    pub fn get_declared_method(&self, id: DeclMethodId) -> Option<&DeclMethod> {
        self.declared_methods.get(id.index())
    }

    // ---------- ----------
    pub fn duplicate(&self) -> Result<(), RlcError> {
        // Includes
        for (i, s1) in self.includes.iter().enumerate() {
            for s2 in self.includes.iter().skip(i + 1) {
                if s1 == s2 {
                    return Err(RlcError::DuplInclude { name: s1.clone() });
                }
            }
        }
        // Types

        // Functions
        for (i, fn1) in self.functions.iter().enumerate() {
            for fn2 in self.functions.iter().skip(i + 1) {
                if fn1.name() == fn2.name() {
                    return Err(RlcError::Duplicate {
                        name: (fn1.name().to_string().clone()),
                        first: (fn1.position().clone()),
                        second: (fn2.position().clone()),
                    });
                }
            }
        }
        // Declared Methods

        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), RlcError> {
        // TODO
        Ok(())
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.includes.iter() {
            writeln!(f, "include \"{}\"", x)?;
        }
        write!(f, "\n")?;
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
