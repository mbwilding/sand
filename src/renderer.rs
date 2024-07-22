use crate::game::Game;
use anyhow::Result;
use crossterm::{
    cursor::{self},
    event::{self, poll, Event, KeyCode},
    execute, queue, style, terminal,
};
use std::io::Write;
use std::time::Duration;

pub fn render(mut game: Game) -> Result<()> {
    let mut w = std::io::stdout();

    execute!(
        w,
        terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    terminal::enable_raw_mode()?;

    loop {
        queue!(w, cursor::Hide)?;

        if poll(Duration::from_millis(game.speed))? {
            match crossterm::event::read()? {
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(k) => match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => game.reset(),
                    KeyCode::Char('d') => game.drain(),
                    KeyCode::Char('h') | KeyCode::Left => game.move_left(),
                    KeyCode::Char('l') | KeyCode::Right => game.move_right(),
                    KeyCode::Char('k') | KeyCode::Up => game.move_up(),
                    KeyCode::Char('j') | KeyCode::Down => game.move_down(),
                    KeyCode::Char(' ') => game.cell_add(),
                    KeyCode::Delete => game.cell_remove(),
                    _ => {}
                },
                Event::Mouse(m) => {
                    let (column, row) = (m.column, m.row);
                    game.selected_column = column;
                    game.selected_row = row;
                    match m.kind {
                        event::MouseEventKind::Down(event::MouseButton::Left)
                        | event::MouseEventKind::Drag(event::MouseButton::Left) => {
                            game.cell_add();
                        }
                        event::MouseEventKind::Down(event::MouseButton::Right)
                        | event::MouseEventKind::Drag(event::MouseButton::Right) => {
                            game.cell_remove();
                        }
                        _ => {}
                    }
                }
                Event::Paste(_) => {}
                Event::Resize(columns, rows) => game.resize_grid(columns, rows),
            };
        }

        game.update();

        queue!(w, terminal::Clear(terminal::ClearType::All))?;
        for (columns, column) in game.grid.iter().enumerate() {
            for (rows, &cell) in column.iter().enumerate() {
                if let Some(block) = cell {
                    let color = block.color.rgb();
                    queue!(
                        w,
                        cursor::MoveTo(columns as u16, rows as u16),
                        style::SetForegroundColor(style::Color::Rgb {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                        }),
                        style::Print(block.glyph),
                        style::ResetColor
                    )?;
                }
            }
        }

        queue!(
            w,
            cursor::MoveTo(0, 0),
            style::SetBackgroundColor(style::Color::Black),
            style::Print(" ".repeat(game.total_columns.into())),
            cursor::MoveTo(0, 0),
            style::Print("paint: left_mouse or space | erase: right_mouse or ctrl | drain: d | reset: r | quit: q"),
            cursor::MoveTo(game.selected_column, game.selected_row),
            style::SetBackgroundColor(style::Color::Black),
            style::SetForegroundColor(style::Color::White),
            style::Print("X"),
            style::ResetColor
        )?;

        w.flush()?;
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

// TODO: Below
// Fix blocking loop
