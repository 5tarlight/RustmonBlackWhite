mod player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
}

pub use player::Player;
