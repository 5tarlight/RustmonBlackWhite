use ansi_term::Color;
use console::println;
use entity::{Gender, Player};

pub fn create_player() {
    console::println(format!(
        "안녕 포켓몬 세계에 온 걸 환영해. 내 이름은 {}. 포켓몬을 연구하고 있지.",
        Color::Yellow.paint("주박사")
    ));
    let mut name = console::read_line("이름을 알려줄래? ");

    while console::read_line(format!("{} 이로구나? (Y/N) ", Color::Yellow.paint(&name)).as_str())
        != "Y".to_string()
    {
        name = console::read_line("이름을 알려줄래? ");
    }

    console::println(format!("{}. 멋진 이름이네.", Color::Yellow.paint(&name)));

    let gender: Gender;
    loop {
        let gen = console::read_line("성별은 무엇이니? (M/F) ");
        if gen == "M" {
            gender = Gender::Male;
            break;
        } else if gen == "F" {
            gender = Gender::Female;
            break;
        }
    }

    let player = Player::new(name, gender);
    println(player.to_json());
    player.save();
}
