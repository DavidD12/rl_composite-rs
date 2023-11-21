use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use line_col::LineColLookup;
use rl_model::parser::error::RlError;

use crate::parser::Position;

pub enum RlcError {
    RL(RlError),
    File {
        filename: String,
        message: String,
    },
    Parse {
        message: String,
        position: Option<Position>,
        expected: Vec<String>,
    },
    Duplicate {
        name: String,
        first: Option<Position>,
        second: Option<Position>,
    },
    Resolve {
        element: String,
        position: Option<Position>,
    },
    Other(String),
}

impl RlcError {
    pub fn new_parse(
        file: &str,
        lookup: &LineColLookup,
        error: ParseError<usize, Token, &str>,
    ) -> Self {
        match error {
            ParseError::InvalidToken { location } => Self::Parse {
                message: "invalid token".into(),
                position: Some(Position::new(file, lookup, location)),
                expected: Vec::new(),
            },
            ParseError::UnrecognizedEOF { location, expected } => Self::Parse {
                message: "unreconized EOF".into(),
                position: Some(Position::new(file, lookup, location)),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::Parse {
                message: format!("unreconized token '{}'", token.1),
                position: Some(Position::new(file, lookup, token.0)),
                expected,
            },
            ParseError::ExtraToken { token } => Self::Parse {
                message: format!("extra token '{}'", token.1),
                position: Some(Position::new(file, lookup, token.0)),
                expected: Vec::new(),
            },
            ParseError::User { error } => Self::Parse {
                message: format!("parse error '{}'", error),
                position: None,
                expected: Vec::new(),
            },
        }
    }
}

impl std::fmt::Display for RlcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RlcError::RL(error) => write!(f, "[RL] {}", error),
            RlcError::File { filename, message } => {
                write!(f, "cannot read file {} {}", filename, message)
            }
            RlcError::Parse {
                message,
                position,
                expected,
            } => match position {
                Some(position) => write!(
                    f,
                    "parse error '{}' at {}, expecting: {:?}",
                    message, position, expected
                ),
                None => write!(f, "parse error '{}', expecting: {:?}", message, expected),
            },
            RlcError::Other(msg) => write!(f, "error: {}", msg),
            RlcError::Resolve { element, position } => {
                if let Some(position) = position {
                    write!(f, "unresolved {} at {}", element, position)
                } else {
                    write!(f, "unresolved {}", element)
                }
            }
            RlcError::Duplicate {
                name,
                first,
                second,
            } => match (first, second) {
                (None, None) => write!(f, "duplicate '{}'", name),
                (None, Some(p)) => write!(f, "duplicate '{}' at {}", name, p),
                (Some(p), None) => write!(f, "duplicate '{}' at {}", name, p),
                (Some(p1), Some(p2)) => write!(f, "duplicate '{}' at {} and {}", name, p1, p2),
            },
        }
    }
}

impl From<RlError> for RlcError {
    fn from(value: RlError) -> Self {
        RlcError::RL(value)
    }
}
