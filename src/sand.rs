use crate::block::Block;
use anyhow::Result;
use crossterm::terminal;

#[derive(Debug)]
pub struct Sand {
    pub columns: u16,
    pub rows: u16,
    pub grid: Vec<Vec<Option<Block>>>,
    pub speed: u64,
}

impl Sand {
    pub fn new(fps: u64) -> Result<Self> {
        let (columns, rows) = terminal::size()?;
        let grid: Vec<Vec<Option<Block>>> = vec![vec![None; rows as usize]; columns as usize];
        let speed = 1000 / fps;

        Ok(Self {
            columns,
            rows,
            grid,
            speed,
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

    pub fn set_cell(&mut self, column: u16, row: u16, block: Block) {
        self.grid[column as usize][row as usize] = Some(block);
    }

    pub fn clear_cell(&mut self, column: u16, row: u16) {
        self.grid[column as usize][row as usize] = None;
    }

    pub fn update(&mut self) {
        for x in 0..self.columns as usize {
            for y in (0..self.rows as usize).rev() {
                if self.grid[x][y].is_some() {
                    if y + 1 < self.rows as usize && self.grid[x][y + 1].is_none() {
                        self.grid[x][y + 1] = self.grid[x][y];
                        self.grid[x][y] = None;
                    }
                }
            }
        }
    }
}
