use crate::cell::Cell;

const N: usize = 100;

#[derive(Copy, Clone)]
pub struct Board{
    cells : [[Cell; N] ; N],
}

impl Board {

    pub fn new(random_start : bool,colorful_game:bool)->Board{
        let mut board = Board{
            cells : [[ Cell::new(false, 0, 0,[0.0;4]); N]; N]
        };

        for i in 0..N {
            for j in 0..N {
                board.cells[i][j] = Cell::new(if random_start {rand::random()}
                                                    else { false },
                                              i as i32,
                                              j as i32,
                                              if colorful_game {[rand::random(), rand::random(),rand::random(), 1.0]}
                                                    else { [0.0, 0.0, 0.0, 1.0] });
            }
        }
        return board
    }

    pub fn get_cells(self)->[[Cell; N] ; N]{
        self.cells
    }

    pub fn toggle_cell(&mut self,i:usize,j:usize){
        if i>=N || j>=N{
            return;
        }
        self.cells[i][j].toggle_state();
    }

    fn get_neighbours(&mut self, row: usize, col: usize) -> usize {
        let mut neighbors: usize = 0;
        for x in 0..3 {
            for y in 0..3 {
                if is_inside_grid(row as i32, col as i32, x as i32, y as i32) {
                    neighbors += self.cells[row + x - 1][col + y - 1].affects_cell();
                }
            }
        }

        neighbors -= self.cells[row][col].affects_cell();
        neighbors
    }

    fn swap_states(&mut self) {
        for i in 0..N {
            for j in 0..N {
                self.cells[i][j].update_state();
            }
        }
    }

    pub fn update_board(&mut self){
        for i  in 0..N {
            for j in 0..N {
                let neighbors = self.get_neighbours(i, j);

                if self.cells[i][j].is_alive() && (neighbors == 2 || neighbors == 3) { self.cells[i][j].survives() }
                else if ! self.cells[i][j].is_alive() && neighbors == 3 { self.cells[i][j].survives() }
                else { self.cells[i][j].dies() }
            }
        }

        self.swap_states()
    }

    pub fn restart(&mut self){
        for i  in 0..N {
            for j in 0..N {
                self.cells[i][j].restart();
            }
        }
    }

}

fn is_inside_grid(row:i32, col:i32, x:i32, y:i32)->bool{
    let j = col + y - 1;
    let i = row + x - 1;
    (0 <= i) && (i < N as i32) &&(0 <= j) && (j < N as i32)
}