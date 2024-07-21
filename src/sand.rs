use anyhow::Result;
use crossterm::terminal;

use crate::blocks::Block;

#[derive(Debug)]
pub struct Sand {
    pub columns: u16,
    pub rows: u16,
    pub grid: Vec<Vec<bool>>,
    pub speed: u64,
    pub blocks: Option<Block>,
}

impl Sand {
    pub fn new(fps: u64) -> Result<Self> {
        let block = '█';
        let (columns, rows) = terminal::size()?;
        let grid = vec![vec![false; rows as usize]; columns as usize];
        let speed = 1000 / fps;

        Ok(Self {
            columns,
            rows,
            grid,
            block,
            speed,
        })
    }

    pub fn resize_grid(&mut self, columns: u16, rows: u16) {
        let old_rows = self.rows as usize;
        self.grid
            .resize_with(columns as usize, || vec![false; old_rows]);
        for column in &mut self.grid {
            column.resize(rows as usize, false);
        }
        self.columns = columns;
        self.rows = rows;
    }

    pub fn set_cell(&mut self, column: u16, row: u16) {
        self.grid[column as usize][row as usize] = true;
    }

    pub fn clear_cell(&mut self, column: u16, row: u16) {
        self.grid[column as usize][row as usize] = false;
    }

    pub fn update(&mut self) {
        for x in 0..self.columns as usize {
            for y in (0..self.rows as usize).rev() {
                if self.grid[x][y] {
                    if y + 1 < self.rows as usize && !self.grid[x][y + 1] {
                        self.grid[x][y] = false;
                        self.grid[x][y + 1] = true;
                    }
                }
            }
        }
    }
}
