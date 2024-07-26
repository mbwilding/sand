use crate::cell::Cell;
use crate::fps_counter::FpsCounter;
use crate::window::draw_window;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use rayon::prelude::*;

/// The game struct
pub struct Game {
    pub exit: bool,
    pub current_column: u16,
    pub current_row: u16,
    gravity: bool,
    topple: bool,
    topple_range: u8,
    topple_min: u8,
    topple_max: u8,
    topple_range_default: u8,
    brush_max: f64,
    brush_min: f64,
    brush_step: f64,
    brush_current: f64,
    column_total: u16,
    row_total: u16,
    grid: Vec<Vec<Option<Cell>>>,
    window_padding: u16,
    window_help: Window,
    fps_counter: FpsCounter,
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
        let topple_range = 3;

        Self {
            exit: false,
            current_column: (columns / 2) as u16,
            current_row: (rows / 2) as u16,
            gravity: true,
            topple: true,
            topple_range,
            topple_min: 1,
            topple_max: 100,
            topple_range_default: topple_range,
            brush_max: 60.9,
            brush_min: brush,
            brush_step: brush,
            brush_current: brush,
            column_total: columns as u16,
            row_total: rows as u16,
            grid: vec![vec![None; rows as usize]; columns as usize],
            window_padding: 3,
            window_help: Window {
                state: false,
                fg: Color::Green,
                bg: Color::Black,
            },
            fps_counter: FpsCounter::new(),
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
    pub fn brush_apply(&mut self, additive: bool) {
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

    /// Gravity toggle
    pub fn toggle_gravity(&mut self) {
        self.gravity = !self.gravity;
    }

    /// Topple toggle
    pub fn toggle_topple(&mut self) {
        self.topple = !self.topple;
    }

    /// Increases the brush size
    pub fn brush_increase(&mut self) {
        self.brush_current = (self.brush_current + self.brush_step).min(self.brush_max);
    }

    /// Decreases the brush size
    pub fn brush_decrease(&mut self) {
        self.brush_current = (self.brush_current - self.brush_step).max(self.brush_min);
    }

    /// Increases the topple range
    pub fn topple_range_increase(&mut self) {
        self.topple_range = (self.topple_range + 1).min(self.topple_max);
    }

    /// Decreases the topple range
    pub fn topple_range_decrease(&mut self) {
        self.topple_range = (self.topple_range - 1).max(self.topple_min);
    }

    /// Resets the grid
    pub fn reset(&mut self) {
        self.grid = vec![vec![None; self.row_total as usize]; self.column_total as usize];
        self.brush_current = self.brush_min;
        self.topple_range = self.topple_range_default;
        self.gravity = true;
        self.topple = true;
    }

    /// Drains the last row, when gravity is enabled
    pub fn drain(&mut self) {
        if !self.gravity {
            return;
        }

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
brush_apply  ┃ mouse_l/mouse_r
brush_size   ┃ mouse_wheel, -/=
topple_range ┃ [/]
reset        ┃ r
drain        ┃ d
gravity      ┃ g
topple       ┃ t
help         ┃ h
quit         ┃ q

state
━━━━━
gravity: {},
topple: {},
topple_range: {},
brush_size: {:.1}
current_pos: ({}, {})
fps: {:.1}
frame: {}",
                    self.gravity,
                    self.topple,
                    self.topple_range,
                    self.brush_current,
                    self.current_column,
                    self.current_row,
                    self.fps_counter.fps,
                    engine.frame_count
                ),
                self.window_help.fg,
                self.window_help.bg,
                self.window_padding,
                (self.column_total / 2, self.row_total / 2),
            );
        }
    }

    /// Updates the game
    pub fn tick(&mut self) {
        if self.topple {
            self.effect_topple(self.topple_range);
        }
        if self.gravity {
            self.effect_gravity();
        }

        self.fps_counter.tick();
    }

    /// Applies the gravity effect
    fn effect_gravity(&mut self) {
        self.grid.par_iter_mut().for_each(|column| {
            let column_total = column.len();
            for row in (0..column_total).rev() {
                if column[row].is_some() && row + 1 < column_total && column[row + 1].is_none() {
                    column[row + 1] = column[row];
                    column[row] = None;
                }
            }
        });
    }

    /// Applies the topple effect
    fn effect_topple(&mut self, range: u8) {
        for column in 0..self.column_total as usize {
            for row in 0..self.row_total as usize {
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
        range: u8,
        direction: bool,
    ) -> bool {
        let direction = if direction { -1 } else { 1 };
        let new_col = (column as isize + direction) as usize;
        let row_check = row + range as usize;
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
