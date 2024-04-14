use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub power: i64,
}


#[derive(Clone)]
pub struct CreateSpell {
    pub name: String,
    pub power: i64,
}