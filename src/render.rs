//! # Render
//!
//! Contains functions for rendering to and cleaning up the terminal

use crate::font::TextMapper;
use crossterm::terminal::disable_raw_mode;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

/// Perform cleanup operations before exiting
///
/// # Arguments
///
/// * `terminal` - The crossterm terminal to clean up
pub fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    terminal.clear()?;

    Ok(())
}

/// Render the clock based on the number of seconds
///
/// # Attributes
///
/// * `num_seconds` - u32 representing the number of seconds on the clock
/// * `mapper` - The [TextMapper](font::TextMapper) to use for the rendering
/// * `header` - Optional header to display below the clock
///
/// # Returns
///
/// The clock display as a [Text](tui::text::Text) object
pub fn render_clock<'a>(
    num_seconds: &u32,
    mapper: &TextMapper,
    header: Option<&'a str>,
) -> Text<'a> {
    let time = format!("{:0>2}:{:0>2}", num_seconds / 60, num_seconds % 60);
    let mut text = Text::default();

    for i in 0..mapper.get_num_scans() {
        text.extend(Text::from(mapper.get_string_scan(&time, i)));
    }

    if let Some(h) = header {
        text.extend(Text::styled(
            format!("\n{}\n", h),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        ));
    }

    text
}

/// Render the screen
///
/// # Arguments
///
/// * `terminal` - The crossterm terminal in which to render
/// * `num_seconds` - u32 representing the number of seconds on the clock (will be passed to [render_clock])
/// * `mapper` - The [TextMapper](font::TextMapper) to use for the rendering (will be passed to [render_clock])
/// * `header` - Optional header to display below the clock (will be passed to [render_clock])
pub fn render_page(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    num_seconds: &u32,
    mapper: &TextMapper,
    header: Option<&str>,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let mut text_height = mapper.get_num_scans().try_into().unwrap();

        if header.is_some() {
            text_height += 3;
        }

        if f.size().height < text_height {
            f.render_widget(
                Paragraph::new(Text::from("What is this, a terminal for ants?")),
                f.size(),
            );
            return;
        }

        let space = (f.size().height - text_height) / 2;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(space),
                    Constraint::Length(text_height),
                    Constraint::Length(space),
                ]
                .as_ref(),
            )
            .split(f.size());

        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Flying Tomato",
                    Style::default().fg(Color::White),
                ))
                .border_style(Style::default().fg(Color::Red)),
            f.size(),
        );

        f.render_widget(
            Paragraph::new(render_clock(num_seconds, mapper, header)).alignment(Alignment::Center),
            chunks[1],
        );

        sleep(Duration::from_millis(10));
    })?;

    Ok(())
}
