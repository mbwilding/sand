use console_engine::{ConsoleEngine, KeyCode};

use crate::game::Game;

/// Handles the input
pub fn check(game: &mut Game, engine: &ConsoleEngine) {
    // Resets the game
    if engine.is_key_pressed(KeyCode::Char('r')) {
        game.reset();
    }

    // Drains the last row
    if engine.is_key_pressed(KeyCode::Char('d')) {
        game.drain();
    }

    // Toggles the help UI
    if engine.is_key_pressed(KeyCode::Char('h')) {
        game.toggle_help_window();
    }

    // Toggle gravity
    if engine.is_key_pressed(KeyCode::Char('g')) {
        game.toggle_gravity();
    }

    // Toggle topple
    if engine.is_key_pressed(KeyCode::Char('t')) {
        game.toggle_topple();
    }

    // Exits the game
    if engine.is_key_pressed(KeyCode::Char('q')) {
        game.exit = true;
    }

    // Increases the brush size
    if engine.is_key_pressed(KeyCode::Char('=')) {
        game.brush_increase();
    }

    // Reduces the brush size
    if engine.is_key_pressed(KeyCode::Char('-')) {
        game.brush_decrease()
    }

    // Increases the topple range
    if engine.is_key_pressed(KeyCode::Char(']')) {
        game.topple_range_increase();
    }

    // Decreases the topple range
    if engine.is_key_pressed(KeyCode::Char('[')) {
        game.topple_range_decrease();
    }

    // Mouse scroll up increases the brush size
    if engine.is_mouse_scrolled_up() {
        game.brush_increase();
    }

    // Mouse scroll down reduces the brush size
    if engine.is_mouse_scrolled_down() {
        game.brush_decrease()
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
            game.current_column = column as u16;
            game.current_row = row as u16;
            game.brush_apply(button == console_engine::MouseButton::Left);
        }
    }
}
