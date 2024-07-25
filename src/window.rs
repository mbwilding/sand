use console_engine::{pixel::Pixel, rect_style::BorderStyle, Color, ConsoleEngine};

/// Draw a window
pub fn draw_window(
    engine: &mut ConsoleEngine,
    title: &str,
    content: &str,
    fg: Color,
    bg: Color,
    padding: u16,
    (center_column, center_row): (u16, u16),
) {
    let padding = (padding + 1) as i32;
    let half_padding = padding / 2;
    let content_lines: Vec<&str> = content.lines().collect();
    let content_line_count = content_lines.len() as i32;
    let max_line_length = content_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as i32;

    // Positions
    let start_column = center_column as i32 - (padding + max_line_length / 2);
    let start_row = center_row as i32 - (half_padding + content_line_count / 2);
    let start_content_column = start_column + padding;
    let start_content_row = start_row + half_padding;
    let end_column = start_content_column + max_line_length + padding - 1;
    let end_row = start_content_row + half_padding - 1 + content_line_count;

    // Border
    engine.fill_rect(
        start_column,
        start_row,
        end_column,
        end_row,
        Pixel { fg, bg, chr: ' ' },
    );
    engine.rect_border(
        start_column,
        start_row,
        end_column,
        end_row,
        BorderStyle::new_heavy().with_colors(fg, bg),
    );

    // Title
    let title = &format!(" {title} ");
    let title_x = start_column + (end_column - start_column - title.len() as i32) / 2;
    engine.print_fbg(title_x, start_row, title, bg, fg);

    // Content
    engine.print_fbg(start_content_column, start_content_row, content, fg, bg);
}
