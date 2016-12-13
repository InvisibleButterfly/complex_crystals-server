use super::sampleobject::ObjectType;

pub enum Event {
    Move(MoveEvent),
    Fire(FireEvent),
    Build(BuildEvent),
}

pub struct MoveEvent {
    pub name: String,
    pub owner: String,
    pub dest_x: f64,
    pub dest_y: f64,
}

pub struct FireEvent {
    pub name: String,
    pub owner: String,
    pub dest_x: f64,
    pub dest_y: f64,
}

pub struct BuildEvent {
    pub name: String,
    pub owner: String,
    pub b_type: ObjectType,
    pub b_name: String,
}