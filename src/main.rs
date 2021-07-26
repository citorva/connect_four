extern crate text_io;
extern crate getrandom;

mod connect_four;
mod random_bot;

use crate::connect_four::{Interface, Area, State};
use text_io::scan;
use std::fmt::Display;
use std::str::FromStr;
use std::cell::RefCell;

/// Interface en ligne de commande avec un joueur
struct PlayerCLI {
    #[doc(hidden)]
    name : String
}

impl PlayerCLI {
    /// Initialise l’interface avec le nom du joueur
    ///
    /// # Arguments
    ///  * `name` - Le nom du joueur
    pub fn new(name : &str) -> Self {
        Self {
            name: String::from(name)
        }
    }

    /// Renomme l’interface du joueur
    ///
    /// # Arguments
    ///  * `name` - Le nom du joueur
    pub fn rename(&mut self, name : &str) {
        self.name = String::from(name);
    }
}

impl Interface for PlayerCLI {
    fn play(&mut self, area: &Area, token : State) -> usize {
        println!("À {} de jouer ({})", self.name, if token == State::RedToken { "🔴" } else { "🟡" });

        println!("{}", area);

        let columns = area.get_available_columns();

        if columns.len() == 1 {
            println!("Une seule possibilité: {}", columns[0]);

            return columns[0];
        }

        return request("Choisissez une position", columns);
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

fn request<T : Sized + Copy + Display + Eq + FromStr>(req : &str, options : Vec<T>) -> T {
    let option_text = {
        let mut option_text = String::new();

        for i in 0..options.len() {
            if i != 0 {
                option_text += format!("/{}", options[i]).as_str();
            } else {
                option_text += format!("{}", options[i]).as_str();
            }
        }

        option_text
    };

    loop {
        println!("{} [{}]", req, option_text);

        let tmp : String;

        scan!("{}", tmp);

        if let Ok(t) = tmp.parse::<T>() {
            if options.contains(&t) {
                return t;
            }
        }
    }
}

#[doc(hidden)]
fn main() {
    let player1 = RefCell::new(PlayerCLI::new("Joueur 1"));
    let player2 = RefCell::new(PlayerCLI::new("Joueur 2"));
    let rnd_bot = RefCell::new(random_bot::RandomBot::new("Robot aléatoire"));

    let mut game = connect_four::Engine::new(&player1, &player2);

    loop {
        let players = request("Nombre de joueurs", vec![1,2]);

        if players == 1 {
            game.set_player(2, &rnd_bot).unwrap();
        } else {
            game.set_player(2, &player2).unwrap();
        }

        {
            let mut tmp : String;

            println!("Nom du joueur 1");

            scan!("{}", tmp);
            player1.borrow_mut().rename(tmp.as_str());

            if players == 2 {
                println!("Nom du joueur 2");

                scan!("{}", tmp);
                player2.borrow_mut().rename(tmp.as_str());
            }
        }

        if let Ok(v) = game.play() {
            println!("{}", game.get_disposition());

            if let Some(p) = v {
                println!("{} a gagné", p);
            } else {
                println!("Match nul");
            }
        } else {
            break;
        }

        println!("Rejouer? [y/n]");

        let com : String;

        scan!("{}", com);

        if com == "n" {
            break;
        }

        game.reset();
    }
}