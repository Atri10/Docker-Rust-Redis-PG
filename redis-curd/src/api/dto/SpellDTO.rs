use serde::{Deserialize, Serialize};
use crate::domain::models::Spell::{CreateSpell, Spell};

#[derive(Deserialize, Serialize)]
pub struct CreateSpellDTO {
    pub name: String,
    pub power: i64,
}


#[derive(Deserialize, Serialize)]
pub struct SpellDTO {
    pub id: i32,
    pub name: String,
    pub power: i64,
}

impl Into<SpellDTO> for Spell {
    fn into(self) -> SpellDTO {
        SpellDTO {
            id: self.id,
            name: self.name,
            power: self.power,
        }
    }
}


impl Into<Spell> for SpellDTO {
    fn into(self) -> Spell {
        Spell {
            id: self.id,
            name: self.name,
            power: self.power,
        }
    }
}


impl Into<CreateSpellDTO> for CreateSpell {
    fn into(self) -> CreateSpellDTO {
        CreateSpellDTO {
            name: self.name,
            power: self.power,
        }
    }
}


impl Into<CreateSpell> for CreateSpellDTO {
    fn into(self) -> CreateSpell {
        CreateSpell {
            name: self.name,
            power: self.power,
        }
    }
}
