mod player;

#[derive(Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

pub use player::Player;
