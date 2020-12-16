
#[derive(Copy, Clone)]
struct Position{
    x : i32,
    y : i32,
}

#[derive(Copy, Clone)]
pub struct Cell{
    alive_current_state: bool,
    alive_next_state: bool,
    position : Position,
}

impl Cell{

    pub fn new(alive:bool, x:i32, y:i32) ->Cell{
        Cell{
            alive_current_state: alive,
            alive_next_state: false,
            position : Position{ x, y }
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

    pub fn update(&mut self){
        self.alive_current_state = self.alive_next_state;
    }

    pub fn get_x(self)->i32{
        self.position.x
    }
    pub fn get_y(self)->i32{
        self.position.y
    }

    pub fn toggle_state(&mut self){
        self.alive_current_state = !self.alive_current_state;
    }
}