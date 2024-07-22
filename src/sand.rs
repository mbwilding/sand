use crate::block::Block;
use anyhow::Result;
use crossterm::terminal;

#[derive(Debug)]
pub struct Sand {
    pub columns: u16,
    pub rows: u16,
    pub grid: Vec<Vec<Option<Block>>>,
    pub speed: u64,
    pub topple: isize,
}

impl Sand {
    pub fn new(fps: u64, topple: isize) -> Result<Self> {
        let (columns, rows) = terminal::size()?;
        let grid: Vec<Vec<Option<Block>>> = vec![vec![None; rows as usize]; columns as usize];
        let speed = 1000 / fps;

        Ok(Self {
            columns,
            rows,
            grid,
            speed,
            topple,
        })
    }

    pub fn resize_grid(&mut self, columns: u16, rows: u16) {
        let old_rows = self.rows as usize;
        self.grid
            .resize_with(columns as usize, || vec![None; old_rows]);
        for column in &mut self.grid {
            column.resize(rows as usize, None);
        }
        self.columns = columns;
        self.rows = rows;
    }

    pub fn is_cell_set(&mut self, column: u16, row: u16) -> bool {
        self.grid[column as usize][row as usize].is_some()
    }

    pub fn is_cell_unset(&mut self, column: u16, row: u16) -> bool {
        !self.is_cell_set(column, row)
    }

    pub fn cell_set(&mut self, column: u16, row: u16, block: Block) {
        self.grid[column as usize][row as usize] = Some(block);
    }

    pub fn clear_cell(&mut self, column: u16, row: u16) {
        self.grid[column as usize][row as usize] = None;
    }

    pub fn reset(&mut self) {
        self.grid = vec![vec![None; self.rows as usize]; self.columns as usize];
    }

    pub fn drain(&mut self) {
        let last_row = self.rows as usize - 1;
        for column in 0..self.columns as usize {
            self.grid[column][last_row] = None;
        }
    }

    pub fn update(&mut self) {
        self.effect_topple(self.topple);
        self.effect_gravity();
    }

    pub fn effect_gravity(&mut self) {
        for column in 0..self.columns as usize {
            for row in (0..self.rows as usize).rev() {
                if self.grid[column][row].is_some() {
                    if row + 1 < self.rows as usize && self.grid[column][row + 1].is_none() {
                        self.grid[column][row + 1] = self.grid[column][row];
                        self.grid[column][row] = None;
                    }
                }
            }
        }
    }

    pub fn effect_topple(&mut self, range: isize) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        for column in 0..self.columns as usize {
            for row in (0..self.rows as usize).rev() {
                if self.grid[column][row].is_some() {
                    let mut directions = [(-1, range), (1, range)];
                    directions.shuffle(&mut rng);
                    for (dc, dr) in directions.iter() {
                        let new_col = (column as isize + dc) as usize;
                        let new_row = (row as isize + dr) as usize;
                        if new_col < self.columns as usize
                            && new_row < self.rows as usize
                            && self.grid[new_col][new_row].is_none()
                            && (row == self.rows as usize - 1
                                || self.grid[column][row + 1].is_some())
                        {
                            self.grid[new_col][new_row] = self.grid[column][row];
                            self.grid[column][row] = None;
                            break;
                        }
                    }
                }
            }
        }
    }
}
