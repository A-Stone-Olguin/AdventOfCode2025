use ndarray::Array2;

use crate::utils::parse_input_data::parse_input_to_grid;

pub struct Grid {
    grid: Array2<char>,
}

impl Grid {
    pub fn new(file_data: &str) -> Self {
        let grid = parse_input_to_grid(file_data);
        Self { grid }
    }

    pub fn left(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match x <= 0 {
            true => None,
            false => Some((x - 1, y)),
        }
    }

    pub fn right(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match x >= self.grid.dim().0 - 1 {
            true => None,
            false => Some((x + 1, y)),
        }
    }

    pub fn down(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match y >= self.grid.dim().1 - 1 {
            true => None,
            false => Some((x, y + 1)),
        }
    }

    pub fn up(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match y <= 0 {
            true => None,
            false => Some((x, y - 1)),
        }
    }
}
