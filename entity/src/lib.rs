mod gender;
mod player;

use std::fs;

pub use gender::Gender;
pub use player::Player;

pub fn load_player() -> Option<Player> {
    if let Ok(data) = fs::read_to_string("./data/player.json") {
        let player: Player = serde_json::from_str(&data).unwrap();
        Some(player)
    } else {
        None
    }
}
