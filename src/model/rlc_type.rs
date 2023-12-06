use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RlcTypeId(pub usize);
impl Default for RlcTypeId {
    fn default() -> Self {
        Self(0)
    }
}

impl Id for RlcTypeId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct RlcType {
    id: RlcTypeId,
    name: String,
    position: Option<Position>,
}

impl RlcType {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = RlcTypeId::default();
        let name = name.into();
        Self { id, name, position }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> RlcTypeId {
        self.id
    }
}

impl Named<RlcTypeId> for RlcType {
    fn id(&self) -> RlcTypeId {
        self.id
    }
    fn set_id(&mut self, id: RlcTypeId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for RlcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
