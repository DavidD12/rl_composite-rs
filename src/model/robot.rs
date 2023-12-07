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
    pub fn new<S: Into<String>>(name: S, skillset: SkillsetId, position: Option<Position>) -> Self {
        Self {
            id: RobotId::default(),
            name: name.into(),
            skillset,
            position,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn skillset(&self) -> &SkillsetId {
        &self.skillset
    }
    pub fn set_skillset_id(&mut self, id: SkillsetId) -> () {
        self.skillset = id;
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
    fn to_lang(&self, _model: &Model) -> String {
        let mut s: String = format!("robot {}:", self.name);
        // s += &format!("{}", model.rl_model.get_skillset(self.skillset));
        s += "\n";
        s
    }
}
