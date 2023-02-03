use super::Gender;

pub struct Player {
    name: String,
    gender: Gender,
    money: f64,
}

impl Player {
    pub fn new(name: String, gender: Gender) -> Self {
        Self {
            name,
            gender,
            money: 1000.0,
        }
    }

    pub fn clone(&self) -> Player {
        Player {
            name: self.name.clone(),
            gender: self.gender,
            money: self.money,
        }
    }
}
