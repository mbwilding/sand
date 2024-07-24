use crate::game::Game;
use anyhow::{Ok, Result};
use crossterm::event::{self, poll, Event, KeyCode};

pub fn check(game: &mut Game) -> Result<()> {
    if poll(std::time::Duration::from_millis(0))? {
        match crossterm::event::read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(k) => match k.code {
                // KeyCode::Char('h') | KeyCode::Left => game.move_left(),
                // KeyCode::Char('l') | KeyCode::Right => game.move_right(),
                // KeyCode::Char('k') | KeyCode::Up => game.move_up(),
                // KeyCode::Char('j') | KeyCode::Down => game.move_down(),
                KeyCode::Char('r') => game.reset(),
                KeyCode::Char('d') => game.drain(),
                KeyCode::Char(' ') => game.apply(true),
                KeyCode::Char('x') => game.apply(false),
                KeyCode::Char('q') => game.quit(),
                KeyCode::Char(c) if c.is_ascii_digit() => game.set_radius(c.to_digit(10).unwrap()),
                _ => {}
            },
            Event::Mouse(m) => {
                let (column, row) = (m.column, m.row);
                game.selected_column = column;
                game.selected_row = row;
                match m.kind {
                    event::MouseEventKind::Down(event::MouseButton::Left)
                    | event::MouseEventKind::Drag(event::MouseButton::Left) => {
                        game.apply(true);
                    }
                    event::MouseEventKind::Down(event::MouseButton::Right)
                    | event::MouseEventKind::Drag(event::MouseButton::Right) => {
                        game.apply(false);
                    }
                    _ => {}
                }
            }
            Event::Paste(_) => {}
            Event::Resize(columns, rows) => game.resize_grid(columns, rows),
        };
    }

    Ok(())
}
