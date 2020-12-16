
use crate::cell::Cell;

const N: usize = 100;

#[derive(Copy, Clone)]
pub struct Board{
    cells : [[Cell; N] ; N],
}

impl Board {

    pub fn new()->Board{
        let mut board = Board{
            cells : [[ Cell::new(false, 0, 0); N]; N]
        };

        for i in 0..N {
            for j in 0..N {
                board.cells[i][j] = Cell::new(false, i as i32, j as i32);
            }
        }
        return board
    }

    pub fn get_cells(self)->[[Cell; N] ; N]{
        self.cells
    }

    pub fn toggle_cell(&mut self,i:usize,j:usize){
        self.cells[i][j].toggle_state();
    }

    pub fn update(&mut self){
        for i  in 1..(N-1) {
            for j in 1..(N - 1) {

                //get neighbours for each cell
                let mut neighbors: usize = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        neighbors += self.cells[i + x - 1][j + y - 1].affects_cell();
                    }
                }

                neighbors -= self.cells[i][j].affects_cell(); //you don't affect yourself

                if self.cells[i][j].is_alive() && (neighbors == 2 || neighbors == 3) { self.cells[i][j].survives() }
                else if ! self.cells[i][j].is_alive() && neighbors == 3 { self.cells[i][j].survives() }
                else { self.cells[i][j].dies() }
            }
        }


        for i  in 1..(N-1) {
            for j in 1..(N - 1) {
                self.cells[i][j].update();
            }
        }
    }

}