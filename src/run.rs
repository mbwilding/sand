use crate::{block::Block, sand::Sand};
use anyhow::Result;
use crossterm::{
    cursor::{self},
    event::{self, poll, Event, KeyCode},
    execute, queue, style, terminal,
};
use std::io::Write;
use std::time::Duration;

pub fn run(mut sand: Sand) -> Result<()> {
    let mut w = std::io::stdout();

    execute!(
        w,
        terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    terminal::enable_raw_mode()?;

    loop {
        queue!(w, cursor::Hide)?;

        if poll(Duration::from_millis(sand.speed))? {
            match crossterm::event::read()? {
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(k) => match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => sand.reset(),
                    KeyCode::Char('d') => sand.drain(),
                    _ => {}
                },
                Event::Mouse(m) => {
                    let (column, row) = (m.column, m.row);
                    match m.kind {
                        event::MouseEventKind::Down(event::MouseButton::Left)
                        | event::MouseEventKind::Drag(event::MouseButton::Left) => {
                            if sand.is_cell_unset(column, row) {
                                sand.cell_set(column, row, Block::new());
                            }
                        }
                        event::MouseEventKind::Down(event::MouseButton::Right)
                        | event::MouseEventKind::Drag(event::MouseButton::Right) => {
                            if sand.is_cell_unset(column, row) {
                                sand.clear_cell(column, row);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Paste(_) => {}
                Event::Resize(columns, rows) => sand.resize_grid(columns, rows),
            };
        }

        sand.update();

        queue!(w, terminal::Clear(terminal::ClearType::All))?;
        for (columns, column) in sand.grid.iter().enumerate() {
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
                        style::Print(block.character),
                        style::ResetColor
                    )?;
                }
            }
        }

        queue!(
            w,
            cursor::MoveTo(0, 0),
            style::SetBackgroundColor(style::Color::Black),
            style::Print(" ".repeat(sand.columns.into())),
            cursor::MoveTo(0, 0),
            style::Print("paint: left_mouse | erase: right_mouse | drain: d | reset: r | quit: q"),
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
