use crate::board::Board;
extern crate piston_window;
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::*;

const MIN_SCALE_FACTOR : f64 = 6.0;
const MAX_SCALE_FACTOR : f64 = 60.0;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const PADDING: f64 = 1.0;
const NO_PADDING: f64 = 0.0;

#[derive(Copy, Clone)]
struct Offset{
    x:i32,
    y:i32
}

pub struct App{
    padding : f64,
    start : bool,
    board : Board,
    scale_factor : f64,
    window_offset : Offset,
    window : PistonWindow,
}

impl App{

    pub fn new(random_start:bool,colorful_game:bool,padding:bool)->App{
        App{
            padding : if padding {PADDING} else {NO_PADDING},
            start : false,
            board : Board::new(random_start,colorful_game),
            scale_factor : 6.0,
            window_offset : Offset{ x: 0, y: 0 },
            window: WindowSettings::new("Conway's Game of Life!", [WIDTH, HEIGHT])
                        .exit_on_esc(true).resizable(false).build().unwrap(),
        }
    }

    fn draw(&mut self,event:Event) {
        let scale_factor = self.scale_factor;
        let window_offset = self.window_offset;
        let padding = self.padding;
        let background_color = [1.0; 4];
        let cells = self.board.get_cells();
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear(background_color, graphics);
            for i in 0..100 {
                for j in 0..100 {
                    if cells[i][j].is_alive() {
                        let color: [f32; 4] = cells[i][j].get_color();
                        let position: [f64; 4] = [scale_factor * (cells[i][j].get_x() + window_offset.x) as f64,
                                                  scale_factor * (cells[i][j].get_y() + window_offset.y) as f64,
                                                  scale_factor - padding,
                                                  scale_factor - padding];
                        rectangle(color, position, context.transform, graphics);
                    }
                }
            }
        });
    }

    pub fn game_loop(&mut self) {
        let mut mouse_pos = (0.0, 0.0);
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(_r) = e.render_args() {
                self.draw(e);
            } else if let Some(_u) = e.update_args() {
                if self.start {
                    self.board.update_board();
                }
            } else if let Some(button) = e.press_args() {
                self.manage_buttons(&mut mouse_pos, button);
            } else if let Some(cursor) = e.mouse_cursor_args() {
                mouse_pos = (cursor[0], cursor[1]);
            }
            else if let Some(s) = e.mouse_scroll_args() {
                self.manage_zoom(s);
            }

        }
    }

    fn manage_buttons(&mut self, mouse_pos: &mut (f64, f64), button: Button) {
        match button {
            Button::Mouse(_button) => {
                let (x, y) = mouse_pos;
                self.board.toggle_cell(((*x / self.scale_factor) as i32 - self.window_offset.x) as usize,
                                       ((*y / self.scale_factor) as i32 - self.window_offset.y) as usize);
            }
            Button::Keyboard(button) => {
                if button == Key::P {
                    self.start = false;
                } else if button == Key::Return {
                    self.start = true;
                } else if button == Key::Left {
                    if self.window_offset.x < 0 {
                        self.window_offset.x += 1;
                    }
                } else if button == Key::Right {
                    if (WIDTH as i32 / self.scale_factor as i32) < 100 && -self.window_offset.x < 100 - (WIDTH as i32 / self.scale_factor as i32) {
                        self.window_offset.x -= 1;
                    }
                } else if button == Key::Up {
                    if self.window_offset.y < 0 {
                        self.window_offset.y += 1;
                    }
                } else if button == Key::Down {
                    if (HEIGHT as i32 / self.scale_factor as i32) < 100 && -self.window_offset.y < 100 - (HEIGHT as i32 / self.scale_factor as i32) {
                        self.window_offset.y -= 1;
                    }
                } else if button == Key::R {
                    self.start = false;
                    self.board.restart();
                }
            }
            _ => {}
        }
    }

    fn manage_zoom(&mut self, s: [f64; 2]) {
        self.scale_factor += s[1];
        if self.scale_factor < MIN_SCALE_FACTOR {
            self.scale_factor = MIN_SCALE_FACTOR;
            self.window_offset.x = 0;
            self.window_offset.y = 0;
        } else if self.scale_factor > MAX_SCALE_FACTOR{
            self.scale_factor = MAX_SCALE_FACTOR;
        }
    }
}