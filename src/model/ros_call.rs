#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RosCallType {
    Publish,
    Service,
    Action,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct RosCall {
    topic: String,
    typ: RosCallType,
}

impl RosCall {
    pub fn new<S: Into<String>>(topic: S, typ: RosCallType) -> Self {
        Self {
            topic: topic.into(),
            typ: typ.into(),
        }
    }

    pub fn topic(&self) -> &String {
        &self.topic
    }

    pub fn typ(&self) -> &RosCallType {
        &self.typ
    }
}
