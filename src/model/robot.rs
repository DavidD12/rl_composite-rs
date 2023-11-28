use super::*;
use rl_model::model::SkillsetId;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RobotId(pub usize);

impl Default for RobotId {
    fn default() -> Self {
        Self(0)
    }
}

impl Id for RobotId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    id: RobotId,
    name: String,
    skillset: SkillsetId,
    position: Option<Position>,
}

impl Robot {
    pub fn new<S: Into<String>>(name: S, skillset_type: Type, position: Option<Position>) -> Self {
        Self {
            id: RobotId::default(),
            name: name.into(),
            skillset_type,
            position,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn skillset_type(&self) -> &Type {
        &self.skillset_type
    }
    pub fn set_skillset_type(&mut self, id: SkillsetId) -> () {
        self.skillset_type = Type::Skillset(id);
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl Named<RobotId> for Robot {
    fn id(&self) -> RobotId {
        self.id
    }

    fn set_id(&mut self, id: RobotId) {
        self.id = id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLang for Robot {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("robot {}:", self.name);
        s += &format!("{}", self.skillset_type.to_lang(model));
        s += "\n";
        s
    }
}
