use crate::cell::Cell;
use console_engine::pixel;
use console_engine::rect_style::BorderStyle;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

/// The game struct
pub struct Game {
    pub exit: bool,
    help: bool,
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
            help: false,
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

        // Toggles the help UI
        if engine.is_key_pressed(KeyCode::Char('h')) {
            self.toggle_help();
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

    /// Toggles the help UI
    pub fn toggle_help(&mut self) {
        self.help = !self.help;
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

        // Conditionally draws the help UI
        if self.help {
            let offset = 1;

            engine.print_fbg(1, offset - 1, "sand-tui:", Color::Black, Color::Green);
            engine.print_fbg(10, offset - 1, " help", Color::Green, Color::Black);

            engine.rect_border(0, offset + 0, 24, offset + 9, BorderStyle::new_heavy().with_colors(Color::Red, Color::Black));

            engine.print_fbg(1, offset + 1, "mouse_l: add", Color::Green, Color::Reset);
            engine.print_fbg(1, offset + 2, "mouse_r: remove", Color::Yellow, Color::Reset);
            engine.print_fbg(1, offset + 3, "mouse_wheel: brush_size", Color::Blue, Color::Reset);
            engine.print_fbg(1, offset + 4, "r: reset", Color::Cyan, Color::Reset);
            engine.print_fbg(1, offset + 5, "d: drain", Color::Magenta, Color::Reset);
            engine.print_fbg(1, offset + 6, "q: quit", Color::Red, Color::Reset);
            engine.print_fbg(1, offset + 8, "brush_size:", Color::Grey, Color::Reset);
            engine.print_fbg(
                13,
                offset + 8,
                &format!("{:.1}", self.brush_current),
                Color::White,
                Color::Reset,
            );
        }
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
                    if !self.check_topple_direction(column, row, range, direction) {
                        self.check_topple_direction(column, row, range, !direction);
                    }
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
        let row_check = (row as isize + range) as usize;
        if new_col < self.column_total as usize
            && row_check < self.row_total as usize
            && self.grid[new_col][row_check].is_none()
            && (row == self.row_total as usize - 1 || self.grid[column][row + 1].is_some())
        {
            self.grid[new_col][row] = self.grid[column][row];
            self.grid[column][row] = None;

            return true;
        }

        false
    }
}
