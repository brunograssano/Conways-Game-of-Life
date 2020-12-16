
mod cell;
mod board;
use crate::cell::Cell;
use crate::board::Board;

extern crate piston_window;
use piston_window::*;

fn main() {

    let mut board = Board::new();
    let cells = board.get_cells();

    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life!", [600, 600])
            .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for i in 0..100 {
                for j in 0..100{
                    rectangle([1.0, 0.0, 0.0, 1.0], // red <- determinar color a partir del estado
                              [(6 * cells[i][j].get_x()) as f64,(6 *  cells[i][j].get_y())  as f64, 5.0, 5.0],
                              context.transform,
                              graphics);
                }
            }

        });

    }

}
