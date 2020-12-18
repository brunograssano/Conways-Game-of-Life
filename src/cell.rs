
#[derive(Copy, Clone)]
struct Position{
    row: i32,
    col: i32,
}

#[derive(Copy, Clone)]
pub struct Cell{
    original_state: bool,
    alive_current_state: bool,
    alive_next_state: bool,
    position : Position,
    color : [f32;4],
}

impl Cell{

    pub fn new(alive:bool, row:i32, col:i32, color:[f32;4]) ->Cell{
        Cell{
            original_state: alive,
            alive_current_state: alive,
            alive_next_state: false,
            position : Position{ row, col },
            color
        }
    }

    pub fn is_alive(self)->bool{
        self.alive_current_state
    }

    pub fn dies(&mut self){
        self.alive_next_state = false;
    }

    pub fn survives(&mut self){
        self.alive_next_state = true;
    }

    pub fn affects_cell(self) ->usize{
        if self.alive_current_state { 1 } else { 0 }
    }

    pub fn update_state(&mut self){
        self.alive_current_state = self.alive_next_state;
    }

    pub fn get_x(self)->i32{
        self.position.row
    }

    pub fn get_y(self)->i32{
        self.position.col
    }

    pub fn get_color(self) -> [f32;4]{
        self.color
    }

    pub fn toggle_state(&mut self){
        self.alive_current_state = !self.alive_current_state;
    }

    pub fn restart(&mut self){
        self.alive_current_state = self.original_state;
    }
}