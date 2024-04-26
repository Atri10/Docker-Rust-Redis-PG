use diesel::{Insertable, Queryable};
use crate::domain::models::Spell::{CreateSpell, Spell};
use crate::infrastructure::schema::spells;

#[derive(Queryable)]
pub struct SpellDiesel {
    pub id: i32,
    pub name: String,
    pub power: i64,
}

impl From<Spell> for SpellDiesel {
    fn from(value: Spell) -> Self {
        SpellDiesel {
            id: value.id,
            name: value.name,
            power: value.power,
        }
    }
}



#[derive(Insertable)]
#[diesel(table_name = spells)]
pub struct CreateSpellDiesel {
    pub name: String,
    pub power: i64,
}

impl Into<Spell> for SpellDiesel {
    fn into(self) -> Spell {
        Spell {
            id: self.id,
            name: self.name,
            power: self.power,
        }
    }
}

impl From<CreateSpell> for CreateSpellDiesel {
    fn from(value: CreateSpell) -> Self {
        CreateSpellDiesel {
            name: value.name,
            power: value.power,
        }
    }
}


impl Into<Spell> for CreateSpellDiesel {
    fn into(self) -> Spell {
        Spell {
            id: 0,
            name: self.name,
            power: self.power,
        }
    }
}