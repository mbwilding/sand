use crate::{game::Game, input};
use anyhow::Result;
use crossterm::{cursor, execute, queue, style, terminal};
use std::io::Write;

pub fn render(mut game: Game) -> Result<()> {
    let mut w = std::io::stdout();

    execute!(
        w,
        terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
        cursor::Hide
    )?;

    terminal::enable_raw_mode()?;

    let mut previous_frame = vec![vec![None; game.grid[0].len()]; game.grid.len()];

    loop {
        input::check(&mut game)?;
        if !game.running {
            break;
        }

        game.update();

        for (columns, column) in game.grid.iter().enumerate() {
            for (rows, &cell) in column.iter().enumerate() {
                if previous_frame[columns][rows] != cell {
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
                    } else {
                        queue!(
                            w,
                            cursor::MoveTo(columns as u16, rows as u16),
                            style::Print(' ')
                        )?;
                    }
                    previous_frame[columns][rows] = cell;
                }
            }
        }

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
