use crate::cell::Cell;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

/// The game struct
pub struct Game {
    pub exit: bool,
    column_total: u16,
    row_total: u16,
    column_current: u16,
    row_current: u16,
    topple: isize,
    brush_max: f64,
    brush_min: f64,
    brush_step: f64,
    brush_current: f64,
    grid: Vec<Vec<Option<Cell>>>,
}

impl Game {
    /// Initializes the game
    pub fn new(columns: u32, rows: u32) -> Self {
        let brush = 0.7;
        Self {
            exit: false,
            column_total: columns as u16,
            row_total: rows as u16,
            column_current: (columns / 2) as u16,
            row_current: (rows / 2) as u16,
            topple: 3,
            brush_max: 60.9,
            brush_min: brush,
            brush_step: brush,
            brush_current: brush,
            grid: vec![vec![None; rows as usize]; columns as usize],
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
            self.brush_current = (self.brush_current + self.brush_step).min(self.brush_max);
        }

        // Mouse scroll down reduces the brush size
        if engine.is_mouse_scrolled_down() {
            self.brush_current = (self.brush_current - self.brush_step).max(self.brush_min);
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
                self.column_current = column as u16;
                self.row_current = row as u16;
                self.apply(button == console_engine::MouseButton::Left);
            }
        }
    }

    /// Resizes the grid
    pub fn resize(&mut self, new_columns: u16, new_rows: u16) {
        let old_rows = self.row_total as usize;
        self.grid
            .resize_with(new_columns as usize, || vec![None; old_rows]);
        for column in &mut self.grid {
            column.resize(new_rows as usize, None);
        }

        self.column_total = new_columns;
        self.row_total = new_rows;
    }

    /// Applies the brush to the grid
    pub fn apply(&mut self, state: bool) {
        let center_x = self.column_current as f64;
        let center_y = self.row_current as f64;

        for x in (center_x as i32 - self.brush_current as i32)
            ..=(center_x as i32 + self.brush_current as i32)
        {
            for y in (center_y as i32 - self.brush_current as i32)
                ..=(center_y as i32 + self.brush_current as i32)
            {
                if ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt()
                    <= self.brush_current
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
        self.grid = vec![vec![None; self.row_total as usize]; self.column_total as usize];
        self.brush_current = self.brush_min;
    }

    /// Drains the last row
    pub fn drain(&mut self) {
        let last_row = self.row_total as usize - 1;
        for column in 0..self.column_total as usize {
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
        engine.print_fbg(1, 0, "Mouse L: Create", Color::Green, Color::Reset);
        engine.print_fbg(1, 1, "Mouse R: Destroy", Color::Yellow, Color::Reset);
        engine.print_fbg(1, 2, "Mouse Wheel: Brush Size", Color::Blue, Color::Reset);
        engine.print_fbg(1, 3, "R: Reset", Color::Cyan, Color::Reset);
        engine.print_fbg(1, 4, "D: Drain", Color::Magenta, Color::Reset);
        engine.print_fbg(1, 5, "Q: Quit", Color::Red, Color::Reset);
        engine.print_fbg(1, 6, "Brush Size:", Color::Grey, Color::Reset);
        engine.print_fbg(
            13,
            6,
            &format!("{:.1}", self.brush_current),
            Color::White,
            Color::Reset,
        );
    }

    /// Updates the game
    pub fn update(&mut self) {
        self.effect_topple(self.topple);
        self.effect_gravity();
    }

    /// Applies the gravity effect
    fn effect_gravity(&mut self) {
        for column in 0..self.column_total as usize {
            for row in (0..self.row_total as usize).rev() {
                if self.grid[column][row].is_some()
                    && row + 1 < self.row_total as usize
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
        for column in 0..self.column_total as usize {
            for row in (0..self.row_total as usize).rev() {
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
        if new_col < self.column_total as usize
            && new_row < self.row_total as usize
            && self.grid[new_col][new_row].is_none()
            && (row == self.row_total as usize - 1 || self.grid[column][row + 1].is_some())
        {
            self.grid[new_col][new_row] = self.grid[column][row];
            self.grid[column][row] = None;

            return true;
        }

        false
    }
}
