use std::{rc::Rc, thread, time::Duration};

use ::entity::load_player;
use rustmon::{game::main_title::create_player, logger::Logger};

fn main() {
    let (recv, _log_handle) = Logger::new();
    let recv = Rc::new(recv);

    if let Some(player) = load_player() {
        // println!("Hi {}", player.name);
        recv.send(format!("Hi {}", player.name)).unwrap();

        let copy = Rc::clone(&recv);
        thread::sleep(Duration::from_secs(1));
        copy.send("Welcome back!".to_string()).unwrap();
    } else {
        create_player();
    }

    // log_handle.join().unwrap();
}
