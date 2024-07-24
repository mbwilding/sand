use crate::cell::Cell;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

/// The game struct
pub struct Game {
    pub total_columns: u16,
    pub total_rows: u16,
    pub topple: isize,
    pub radius: f64,
    pub brush_min: f64,
    pub brush_max: f64,
    pub brush_step: f64,
    pub grid: Vec<Vec<Option<Cell>>>,
    pub exit: bool,
}

impl Game {
    /// Initializes the game
    pub fn new(total_columns: u32, total_rows: u32) -> Self {
        Self {
            total_columns: total_columns as u16,
            total_rows: total_rows as u16,
            topple: 3,
            radius: 1.0,
            brush_min: 0.0,
            brush_max: 60.0,
            brush_step: 0.5,
            grid: vec![vec![None; total_rows as usize]; total_columns as usize],
            exit: false,
        }
    }

    /// Handles the input
    pub fn input(&mut self, engine: &ConsoleEngine) {
        // Resets the game
        if engine.is_key_pressed(KeyCode::Char('r')) {
            self.reset();
        }

        // Drains the last row
        if engine.is_key_pressed(KeyCode::Char('d')) {
            self.drain();
        }

        // Exits the game
        if engine.is_key_pressed(KeyCode::Char('q')) {
            self.exit = true;
        }

        // Mouse scroll up increases the brush size
        if engine.is_mouse_scrolled_up() {
            self.radius = (self.radius + self.brush_step).min(self.brush_max);
        }

        // Mouse scroll down reduces the brush size
        if engine.is_mouse_scrolled_down() {
            self.radius = (self.radius - self.brush_step).max(self.brush_min);
        }

        // Applies the brush (Left click to draw, Right click to erase)
        for button in [
            console_engine::MouseButton::Left,
            console_engine::MouseButton::Right,
        ] {
            if let Some((column, row)) = engine
                .get_mouse_held(button)
                .or_else(|| engine.get_mouse_press(button))
            {
                self.apply(column, row, button == console_engine::MouseButton::Left);
            }
        }

        // Sets the size of the brush
        for number in 0..=9 {
            if engine.is_key_pressed(KeyCode::Char(char::from_digit(number, 10).unwrap())) {
                self.radius = number as f64;
            }
        }
    }

    /// Resizes the grid
    pub fn resize(&mut self, new_columns: u16, new_rows: u16) {
        let old_rows = self.total_rows as usize;
        self.grid
            .resize_with(new_columns as usize, || vec![None; old_rows]);
        for column in &mut self.grid {
            column.resize(new_rows as usize, None);
        }

        self.total_columns = new_columns;
        self.total_rows = new_rows;
    }

    /// Applies the brush to the grid
    pub fn apply(&mut self, column: u32, row: u32, state: bool) {
        let center_x = column as f64;
        let center_y = row as f64;

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
                    self.grid[x as usize][y as usize] = if state {
                        Some(Cell::new(true, true, true, true))
                    } else {
                        None
                    };
                }
            }
        }
    }

    /// Resets the grid
    pub fn reset(&mut self) {
        self.grid = vec![vec![None; self.total_rows as usize]; self.total_columns as usize];
    }

    /// Drains the last row
    pub fn drain(&mut self) {
        let last_row = self.total_rows as usize - 1;
        for column in 0..self.total_columns as usize {
            self.grid[column][last_row] = None;
        }
    }

    /// Draws the game
    pub fn draw(&self, engine: &mut ConsoleEngine) {
        // Draws the grid
        for (columns, column) in self.grid.iter().enumerate() {
            for (rows, &cell) in column.iter().enumerate() {
                if let Some(cell) = cell {
                    let rgb = cell.color.rgb();
                    let color = Color::Rgb {
                        r: rgb.r,
                        g: rgb.g,
                        b: rgb.b,
                    };
                    engine.set_pxl(
                        columns as i32,
                        rows as i32,
                        pixel::pxl_fg(cell.glyph, color),
                    );
                }
            }
        }

        // Draws the UI
        engine.print(
            0,
            0,
            &format!("Brush: {:.1}", self.radius),
        );
    }

    /// Updates the game
    pub fn update(&mut self) {
        self.effect_topple(self.topple);
        self.effect_gravity();
    }

    /// Applies the gravity effect
    fn effect_gravity(&mut self) {
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

    /// Applies the topple effect
    fn effect_topple(&mut self, range: isize) {
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

    /// Checks if the cell can topple in the given direction and applies the topple effect
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
