use crate::parser::error::*;
use rl_model::model::Model as RlModel;

use super::*;

pub struct Model {
    pub rl_model: RlModel,
    includes: Vec<String>,
    robots: Vec<Robot>,
    types: Vec<RlcType>,
    functions: Vec<Function>,
    declared_methods: Vec<DeclMethod>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            rl_model: RlModel::default(),
            includes: vec![],
            robots: vec![],
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

    // ----- Robots -----
    pub fn add_robot(&mut self, r: Robot) {
        print!("{}", r.name());
        self.robots.push(r);
    }

    pub fn robots(&self) -> &Vec<Robot> {
        &self.robots
    }

    // ----- Types -----
    pub fn add_type(&mut self, rlc_type: RlcType) {
        self.types.push(rlc_type);
    }

    pub fn types(&self) -> &Vec<RlcType> {
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

        // Robots
        for (i, r1) in self.robots.iter().enumerate() {
            for r2 in self.robots.iter().skip(i + 1) {
                if r1.name() == r2.name() {
                    return Err(RlcError::Duplicate {
                        name: r1.name().to_string().clone(),
                        first: r1.position().clone(),
                        second: r2.position().clone(),
                    });
                }
            }
        }

        // Types within self
        for (i, t1) in self.types().iter().enumerate() {
            for t2 in self.types().iter().skip(i + 1) {
                if t1.name() == t2.name() {
                    return Err(RlcError::Duplicate {
                        name: t1.name().to_string().clone(),
                        first: t1.position().clone(),
                        second: t2.position().clone(),
                    });
                }
            }
        }
        // Types within included skillsets
        for t1 in self.types().iter() {
            for t2 in self.rl_model.types().iter() {
                if t1.to_string() == t2.to_string() {
                    return Err(RlcError::DuplType {
                        name: t1.name().to_string().clone(),
                    });
                }
            }
        }

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
        for (i, decl1) in self.declared_methods.iter().enumerate() {
            for decl2 in self.declared_methods.iter().skip(i + 1) {
                if decl1.name() == decl2.name() {
                    return Err(RlcError::Duplicate {
                        name: (decl1.name().to_string().clone()),
                        first: (decl1.position().clone()),
                        second: (decl2.position().clone()),
                    });
                }
            }
        }

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
        for x in self.robots.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        for x in self.types.iter() {
            writeln!(f, "type {}", x)?;
        }
        write!(f, "\n")?;
        for x in self.functions.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        for x in self.declared_methods.iter() {
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
