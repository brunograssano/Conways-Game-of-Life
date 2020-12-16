
#[derive(Copy, Clone)]
struct Position{
    x : u16,
    y : u16,
}

#[derive(Copy, Clone)]
pub struct Cell{
    alive_current_state: bool,
    alive_next_state: bool,
    position : Position,
}

impl Cell{

    pub fn new(alive:bool, x:u16, y:u16) ->Cell{
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
}