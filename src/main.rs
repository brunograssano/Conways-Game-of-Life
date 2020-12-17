
mod cell;
mod board;

use crate::board::Board;

extern crate piston_window;
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::*;
use std::env;

fn draw(board: &mut Board, window: &mut PistonWindow,event:Event) {
    let background_color = [1.0; 4];
    let cells = board.get_cells();
    window.draw_2d(&event, |context, graphics, _device| {
        clear(background_color, graphics);
        for i in 0..100 {
            for j in 0..100 {
                if cells[i][j].is_alive() {
                    let color: [f32; 4] = cells[i][j].get_color();
                    let position: [f64; 4] = [(6 * cells[i][j].get_x()) as f64, (6 * cells[i][j].get_y()) as f64, 5.0, 5.0];
                    rectangle(color, position, context.transform, graphics);
                }
            }
        }
    });
}

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
    let mut board = Board::new(random_start,colorful_game);
    game_loop(&mut board)
}

fn game_loop(mut board: &mut Board) {
    let mut window: PistonWindow = WindowSettings::new("Conway's Game of Life!", [600, 600])
                                                    .exit_on_esc(true).build().unwrap();
    let mut mouse_pos = (0.0, 0.0);
    let mut start = false;
    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(_r) = e.render_args() {
            draw(&mut board, &mut window, e);
        } else if let Some(_u) = e.update_args() {
            if start {
                board.update_board();
            }
        } else if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(_button) => {
                    let (x, y) = mouse_pos;
                    board.toggle_cell((x / 6.0) as usize, (y / 6.0) as usize);
                }
                Button::Keyboard(button) => {
                    if button == Key::P {
                        start = false;
                    } else if button == Key::Return {
                        start = true;
                    }
                }
                _ => {}
            }
        } else if let Some(cursor) = e.mouse_cursor_args() {
            mouse_pos = (cursor[0], cursor[1]);
        }
    }
}
