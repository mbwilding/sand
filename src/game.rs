use crate::cell::Cell;
use crate::window::draw_window;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;

/// The game struct
pub struct Game {
    pub exit: bool,
    pub current_column: u16,
    pub current_row: u16,
    brush_max: f64,
    brush_min: f64,
    brush_step: f64,
    brush_current: f64,
    column_total: u16,
    row_total: u16,
    topple: isize,
    grid: Vec<Vec<Option<Cell>>>,
    window_padding: u16,
    window_help: Window,
}

struct Window {
    state: bool,
    fg: Color,
    bg: Color,
}

impl Game {
    /// Initializes the game
    pub fn new(columns: u32, rows: u32) -> Self {
        let brush = 0.7;
        Self {
            exit: false,
            current_column: (columns / 2) as u16,
            current_row: (rows / 2) as u16,
            brush_max: 60.9,
            brush_min: brush,
            brush_step: brush,
            brush_current: brush,
            column_total: columns as u16,
            row_total: rows as u16,
            topple: 3,
            grid: vec![vec![None; rows as usize]; columns as usize],
            window_padding: 3,
            window_help: Window {
                state: false,
                fg: Color::Green,
                bg: Color::Black,
            },
        }
    }

    /// Toggles the help UI
    pub fn toggle_help_window(&mut self) {
        self.window_help.state = !self.window_help.state;
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
    pub fn apply(&mut self, additive: bool) {
        let center_column = self.current_column as f64;
        let center_row = self.current_row as f64;

        for column in (center_column as i32 - self.brush_current as i32)
            ..=(center_column as i32 + self.brush_current as i32)
        {
            for row in (center_row as i32 - self.brush_current as i32)
                ..=(center_row as i32 + self.brush_current as i32)
            {
                if ((column as f64 - center_column).powi(2) + (row as f64 - center_row).powi(2))
                    .sqrt()
                    <= self.brush_current
                    && column >= 0
                    && row >= 0
                    && (column as usize) < self.grid.len()
                    && (row as usize) < self.grid[0].len()
                {
                    let cell = self.grid[column as usize][row as usize];

                    if additive && cell.is_none() {
                        let cell = Cell::new(true, true, true, true);
                        self.grid[column as usize][row as usize] = Some(cell);
                    } else if !additive && cell.is_some() {
                        self.grid[column as usize][row as usize] = None;
                    }
                }
            }
        }
    }

    /// Increases the brush size
    pub fn brush_increase(&mut self) {
        self.brush_current = (self.brush_current + self.brush_step).min(self.brush_max);
    }

    /// Decreases the brush size
    pub fn brush_decrease(&mut self) {
        self.brush_current = (self.brush_current - self.brush_step).max(self.brush_min);
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
        if self.window_help.state {
            draw_window(
                engine,
                "HELP",
                &format!(
                    r"bindings
━━━━━━━━
add        ┃ mouse_l
remove     ┃ mouse_r
brush_size ┃ mouse_wheel, -/=
reset      ┃ r
drain      ┃ d
help       ┃ h
quit       ┃ q

state
━━━━━
brush_size: {:.1}
current_pos: ({}, {})",
                    self.brush_current, self.current_column, self.current_row,
                ),
                self.window_help.fg,
                self.window_help.bg,
                self.window_padding,
                (self.column_total / 2, self.row_total / 2),
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
