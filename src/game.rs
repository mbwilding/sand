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
    pub radius: f64,
}

impl Game {
    pub fn new(fps: u64, topple: isize) -> Result<Self> {
        let (total_columns, total_rows) = terminal::size()?;
        let selected_column = total_columns / 2;
        let selected_row = total_rows / 2;
        let grid: Vec<Vec<Option<Cell>>> =
            vec![vec![None; total_rows as usize]; total_columns as usize];
        let speed = 1000 / fps;
        let radius = 1.0;

        Ok(Self {
            total_columns,
            total_rows,
            selected_column,
            selected_row,
            grid,
            speed,
            topple,
            radius,
        })
    }

    pub fn set_radius(&mut self, size: u32) {
        self.radius = size as f64;
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

    pub fn create(&mut self) {
        self.apply_to_grid(true);
    }

    pub fn destroy(&mut self) {
        self.apply_to_grid(false);
    }

    fn apply_to_grid(&mut self, state: bool) {
        let center_x = self.selected_column as f64;
        let center_y = self.selected_row as f64;

        for x in (center_x as i32 - self.radius as i32)..=(center_x as i32 + self.radius as i32) {
            for y in (center_y as i32 - self.radius as i32)..=(center_y as i32 + self.radius as i32)
            {
                if ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt()
                    <= self.radius
                    && x >= 0
                    && y >= 0
                    && (x as usize) < self.grid.len()
                    && (y as usize) < self.grid[0].len()
                {
                    self.grid[x as usize][y as usize] =
                        if state { Some(Cell::new()) } else { None };
                }
            }
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
                if self.grid[column][row].is_some()
                    && row + 1 < self.total_rows as usize
                    && self.grid[column][row + 1].is_none()
                {
                    self.grid[column][row + 1] = self.grid[column][row];
                    self.grid[column][row] = None;
                }
            }
        }
    }

    pub fn effect_topple(&mut self, range: isize) {
        for column in 0..self.total_columns as usize {
            for row in (0..self.total_rows as usize).rev() {
                if self.grid[column][row].is_some() {
                    let direction = rand::random::<bool>();
                    if self.check_topple_direction(column, row, range, direction) {
                        continue;
                    }
                    self.check_topple_direction(column, row, range, !direction);
                }
            }
        }
    }

    fn check_topple_direction(
        &mut self,
        column: usize,
        row: usize,
        range: isize,
        direction: bool,
    ) -> bool {
        let new_col = (column as isize + if direction { -1 } else { 1 }) as usize;
        let new_row = (row as isize + range) as usize;
        if new_col < self.total_columns as usize
            && new_row < self.total_rows as usize
            && self.grid[new_col][new_row].is_none()
            && (row == self.total_rows as usize - 1 || self.grid[column][row + 1].is_some())
        {
            self.grid[new_col][new_row] = self.grid[column][row];
            self.grid[column][row] = None;

            return true;
        }

        false
    }
}
