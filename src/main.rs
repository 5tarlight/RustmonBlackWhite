use rustmon::{entity, game::main_title::create_player};

fn main() {
    if let Some(player) = entity::load_player() {
        println!("Hi {}", player.name);
    } else {
        create_player();
    }
}
