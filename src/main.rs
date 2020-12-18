mod cell;
mod board;
mod app;
use std::env;
use crate::app::App;

fn parse_arguments() -> (bool,bool,bool){
    let mut random_start = false;
    let mut colorful_game = false;
    let mut padding = true;
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        if arg == "random" || arg == "r" {
            random_start = true;
        } else if arg == "colorful" || arg == "c" {
            colorful_game = true;
        }else if arg == "no_padding" || arg == "p" {
            padding = false;
        }
    }
    (random_start,colorful_game,padding)
}

fn main() {
    let (random_start,colorful_game,padding) = parse_arguments();
    let mut app = App::new(random_start,colorful_game,padding);
    app.game_loop()
}
