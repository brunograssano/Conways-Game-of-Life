
mod cell;
mod board;

use crate::board::Board;

extern crate piston_window;
use piston_window::*;

fn draw(board: &mut Board, window: &mut PistonWindow, event: Event) {
    let cells = board.get_cells();
    window.draw_2d(&event, |context, graphics, _device| {
        clear([1.0; 4], graphics);
        for i in 0..100 {
            for j in 0..100 {
                if cells[i][j].is_alive() {
                    let color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
                    let position: [f64; 4] = [(6 * cells[i][j].get_x()) as f64, (6 * cells[i][j].get_y()) as f64, 5.0, 5.0];
                    rectangle(color, position, context.transform, graphics);
                }
            }
        }
    });
}

fn main() {

    let mut board = Board::new();

    let mut window: PistonWindow = WindowSettings::new("Conway's Game of Life!", [600, 600])
                                                    .exit_on_esc(true).build().unwrap();

    window.set_max_fps(30);

    let mut mouse_pos = (0.0,0.0);
    let mut start = false;
    while !start {
        if let Some(event) = window.next(){
            if let Some(args) = event.button_args() {
                if args.scancode == Some(28){
                    start = true;
                }
            }

            if let Some(Button::Mouse(_button)) = event.press_args() {
                let (x,y) = mouse_pos;
                board.toggle_cell((x/6.0) as usize,(y/6.0) as usize);
            }
            if let Some(cursor) = event.mouse_cursor_args(){
                mouse_pos = (cursor[0], cursor[1]);
            }

            draw(&mut board, &mut window, event);
        }
    }

    while let Some(event) = window.next() {
        board.update();
        draw(&mut board, &mut window, event);
    }

}
