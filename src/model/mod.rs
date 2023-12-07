pub mod model;
pub use model::*;

pub mod robot;
pub use robot::*;

pub mod function;
pub use function::*;

pub mod decl_method;
pub use decl_method::*;

pub mod parameter;
pub use parameter::*;

pub mod ros_call;
pub use ros_call::*;

pub mod rlc_type;
pub use rlc_type::*;

pub mod typ;
pub use typ::*;

pub mod expr;
pub use expr::*;

pub mod expression;
pub use expression::*;

use crate::parser::{Position, RlcError};

pub trait ToLang {
    fn to_lang(&self, model: &Model) -> String;
}

//------------------------- Id -------------------------

pub trait Id: Clone + Copy + PartialEq + Eq + core::hash::Hash + std::fmt::Debug + Default {
    fn index(&self) -> usize;
}

pub trait GetFromId<I: Id, T> {
    fn get(&self, i: I) -> Option<&T>;
}

//------------------------- Named -------------------------

pub trait Named<I: Id> {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
    fn name(&self) -> &str;
    fn position(&self) -> Option<Position>;
    fn naming(&self) -> Naming {
        (self.name().into(), self.position())
    }
}

pub type Naming = (String, Option<Position>);

pub fn check_duplicate(names: Vec<Naming>) -> Result<(), RlcError> {
    for (i, (n1, p1)) in names.iter().enumerate() {
        for (n2, p2) in names.iter().skip(i + 1) {
            if n1 == n2 {
                return Err(RlcError::Duplicate {
                    name: n1.clone(),
                    first: p1.clone(),
                    second: p2.clone(),
                });
            }
        }
    }
    Ok(())
}
