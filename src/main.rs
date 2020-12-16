
mod cell;
mod board;

extern crate piston_window;
use piston_window::*;

fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life!", [600, 600])
            .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }

    //lectura de configuracion

    //armado

    //ejecutar


    // cells |||||
    // tienen un estado y vecinos
    // estado de una cell en t = F(estados de los vecinos en t-1)
}
