use crate::cell::Cell;
use anyhow::Result;
use crossterm::terminal;

#[derive(Debug)]
pub struct Game {
    pub total_columns: u16,
    pub total_rows: u16,
    pub selected_column: u16,
    pub selected_row: u16,
    pub speed: u64,
    pub grid: Vec<Vec<Option<Cell>>>,
    pub topple: isize,
}

impl Game {
    pub fn new(fps: u64, topple: isize) -> Result<Self> {
        let (columns, rows) = terminal::size()?;
        let selected_column = columns / 2;
        let selected_row = rows / 2;
        let grid: Vec<Vec<Option<Cell>>> = vec![vec![None; rows as usize]; columns as usize];
        let speed = 1000 / fps;

        Ok(Self {
            total_columns: columns,
            total_rows: rows,
            selected_column,
            selected_row,
            grid,
            speed,
            topple,
        })
    }

    pub fn move_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
        } else {
            self.selected_column = self.total_columns - 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.selected_column < self.total_columns - 1 {
            self.selected_column += 1;
        } else {
            self.selected_column = 0;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        } else {
            self.selected_row = self.total_rows - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_row < self.total_rows - 1 {
            self.selected_row += 1;
        } else {
            self.selected_row = 0;
        }
    }

    pub fn resize_grid(&mut self, new_columns: u16, new_rows: u16) {
        let old_rows = self.total_rows as usize;
        self.grid
            .resize_with(new_columns as usize, || vec![None; old_rows]);
        for column in &mut self.grid {
            column.resize(new_rows as usize, None);
        }

        self.selected_column = self.selected_column.min(new_columns - 1);
        self.selected_row = self.selected_row.min(new_rows - 1);

        self.total_columns = new_columns;
        self.total_rows = new_rows;
    }

    pub fn is_current_cell_set(&mut self) -> bool {
        self.grid[self.selected_column as usize][self.selected_row as usize].is_some()
    }

    pub fn cell_add(&mut self) {
        if !self.is_current_cell_set() {
            self.grid[self.selected_column as usize][self.selected_row as usize] =
                Some(Cell::new());
        }
    }

    pub fn cell_remove(&mut self) {
        if self.is_current_cell_set() {
            self.grid[self.selected_column as usize][self.selected_row as usize] = None;
        }
    }

    pub fn reset(&mut self) {
        self.grid = vec![vec![None; self.total_rows as usize]; self.total_columns as usize];
    }

    pub fn drain(&mut self) {
        let last_row = self.total_rows as usize - 1;
        for column in 0..self.total_columns as usize {
            self.grid[column][last_row] = None;
        }
    }

    pub fn update(&mut self) {
        self.effect_topple(self.topple);
        self.effect_gravity();
    }

    pub fn effect_gravity(&mut self) {
        for column in 0..self.total_columns as usize {
            for row in (0..self.total_rows as usize).rev() {
                if self.grid[column][row].is_some() {
                    if row + 1 < self.total_rows as usize && self.grid[column][row + 1].is_none() {
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

        for column in 0..self.total_columns as usize {
            for row in (0..self.total_rows as usize).rev() {
                if self.grid[column][row].is_some() {
                    let mut directions = [(-1, range), (1, range)];
                    directions.shuffle(&mut rng);
                    for (dc, dr) in directions.iter() {
                        let new_col = (column as isize + dc) as usize;
                        let new_row = (row as isize + dr) as usize;
                        if new_col < self.total_columns as usize
                            && new_row < self.total_rows as usize
                            && self.grid[new_col][new_row].is_none()
                            && (row == self.total_rows as usize - 1
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
