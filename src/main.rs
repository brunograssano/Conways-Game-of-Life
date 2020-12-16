
mod cell;
mod board;
use crate::cell::Cell;
use crate::board::Board;

extern crate piston_window;
use piston_window::*;

fn main() {

    let mut board = Board::new();
    //let cells = board.get_cells();

    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life!", [600, 600])
            .exit_on_esc(true).build().unwrap();

    board.toggle_cell(2,2);
    board.toggle_cell(2,3);
    board.toggle_cell(3,2);
    board.toggle_cell(3,3);

    while let Some(event) = window.next() {
        let cells = board.get_cells();
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for i in 0..100 {
                for j in 0..100{
                    //[red, green, blue, alpha]
                    // All values are between 0.0 and 1.0. For example, black is [0.0, 0.0, 0.0, 1.0] and white is [1.0, 1.0, 1.0, 1.0].
                    let color : [f32;4];
                    if cells[i][j].is_alive() { color = [0.0, 0.0, 0.0, 1.0]} else { color = [1.0, 1.0, 1.0, 1.0] }
                    //let color = [1.0, 0.0, 0.0, 1.0];
                    let position : [f64;4] = [(6 * cells[i][j].get_x()) as f64,(6 *  cells[i][j].get_y())  as f64, 5.0, 5.0];
                    rectangle(color, position, context.transform, graphics);
                }
            }
        });

        board.update();

    }

}
