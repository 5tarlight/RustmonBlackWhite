use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::Gender;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub gender: Gender,
    pub money: f64,
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

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn save(&self) {
        let file = "./data/player.json";
        let path = Path::new(&file);

        match fs::create_dir_all("./data") {
            Ok(_) => {}
            Err(err) => {
                println!("{:}", err);
            }
        }

        match File::create(path) {
            Ok(_) => {}
            Err(err) => {
                println!("{:}", err);
            }
        } // TODO : Send to logger

        match fs::write(path, self.to_json()) {
            Ok(_) => {
                println!("Success!");
            }
            Err(err) => {
                println!("{:}", err);
            }
        }
    }
}
