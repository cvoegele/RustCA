use std::cell::Cell;
use ndarray::{Array, Array3, ArrayView, ArrayView3, Dim, s};

pub struct CellularAutomata {
    side_length: usize,
    field: Array3<i32>,
}

impl CellularAutomata {
    pub fn new(side_length: usize) -> CellularAutomata {
        CellularAutomata {
            side_length,
            field: Array3::<i32>::zeros((side_length, side_length, side_length)),
        }
    }

    pub fn get_size(&self) -> usize {
        self.side_length
    }

    pub fn set_value(&mut self, value: i32, index: (usize, usize, usize)) {
        let (x, y, z) = index;
        self.field[[x, y, z]] = value
    }

    pub fn get_value(&self, index: (usize, usize, usize)) -> i32 {
        let (x, y, z) = index;
        self.field[[x, y, z]]
    }

    pub fn get_neighbours(&self, index: (usize, usize, usize)) -> ArrayView3<'_, i32> {
        let (x, y, z) = index;
        self.field.slice(s![x-1..x+2, y-1..y+2, z-1..z+2])
    }

    pub fn update(&mut self, update_function: (&dyn Fn(&CellularAutomata, i32, (usize, usize, usize)) -> i32)) {
        for x in 0..self.side_length {
            for y in 0..self.side_length {
                for z in 0..self.side_length {
                    let index = (x, y, z);
                    let value = self.field[index];
                    let result = update_function(&self, value, index);
                    self.field[index] = result
                }
            }
        }
    }
}
