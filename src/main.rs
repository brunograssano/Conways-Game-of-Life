mod cell;
mod board;
mod app;
use std::env;
use crate::app::App;

fn parse_arguments() -> (bool,bool){
    let mut random_start = false;
    let mut colorful_game = false;
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        if arg == "random" || arg == "r" {
            random_start = true;
        } else if arg == "colorful" || arg == "c" {
            colorful_game = true;
        }
    }
    (random_start,colorful_game)
}

fn main() {
    let (random_start,colorful_game) = parse_arguments();
    let mut app = App::new(random_start,colorful_game);
    app.game_loop()
}
