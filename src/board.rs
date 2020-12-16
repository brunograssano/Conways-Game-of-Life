
//use cell::Cell;
use crate::cell::Cell;

const N: usize = 600;

struct Board{
    cells : [[Cell; N] ; N],
}

impl Board {

    pub fn new()->Board{
        let mut board = Board{
            cells : [[ Cell::new(false, 0, 0); N]; N]
        };

        for i in 0..N {
            for j in 0..N {
                board.cells[i][j] = Cell::new(false, i as u16, j as u16);
            }
        }
        return board
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