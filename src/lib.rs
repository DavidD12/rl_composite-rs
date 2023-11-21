#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod model;
pub mod parser;

pub fn load_model(filename: &str) -> Result<model::Model, parser::RlcError> {
    // Parsing
    match parser::parse_file(filename) {
        Ok(mut model) => {
            info!("Parsing OK");
            // RL Duplicate
            match model.rl_model.duplicate() {
                Ok(_) => info!("Duplicate OK"),
                Err(e) => {
                    error!("[RL] {}", e);
                    return Err(e.into());
                }
            }
            // RL Resolve
            match model.rl_model.resolve() {
                Ok(_) => info!("[RL] Resolve OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e.into());
                }
            }
            // RLC Duplicate
            match model.duplicate() {
                Ok(_) => info!("Duplicate OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            // RLC Resolve
            match model.resolve() {
                Ok(_) => info!("Resolve OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            // TODO: check parameters (number)
            // TODO: check type
            // TODO: other check ?
            //
            Ok(model)
        }
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
}
