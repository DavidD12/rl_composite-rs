use crate::model::Model;
use crate::parser::error::*;
use line_col::LineColLookup;
use std::fs;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub struct Parser {
    file: String,
    pub rl_parser: rl_model::parser::Parser,
    pub model: Model,
}

impl Parser {
    pub fn new<S: Into<String>>(file: S) -> Self {
        let file = file.into();
        Self {
            file,
            rl_parser: Default::default(),
            model: Default::default(),
        }
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn parse(&mut self) -> Result<(), RlcError> {
        match fs::read_to_string(&self.file) {
            Ok(input) => {
                let lookup = LineColLookup::new(&input);
                match grammar::ModelParser::new().parse(&lookup, self, &input) {
                    Ok(_) => match self.rl_parser.parse() {
                        Ok(_) => Ok(()),
                        Err(err) => Err(RlcError::RL(err)),
                    },
                    Err(e) => return Err(RlcError::new_parse(&self.file, &lookup, e)),
                }
            }
            Err(e) => {
                let e = RlcError::File {
                    filename: self.file.clone(),
                    message: format!("{:?}", e),
                };
                return Err(e);
            }
        }
    }
}
