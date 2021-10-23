mod font;

use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use font::TextMapper;
use std::io;
use std::mem;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("unable to poll for events") {
                if let CEvent::Key(key) = event::read().expect("unable to read event") {
                    tx.send(Event::KeyInput(key)).expect("unable to send event");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    const WORK_LENGTH: u32 = 25 * 60;
    const BREAK_LENGTH: u32 = 5 * 60;
    const NUM_INTERVALS: u32 = 5;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mapper = TextMapper::new();

    let mut current_interval = Interval::Work;

    'outer: for _ in 0..(NUM_INTERVALS * 2) {
        let (length, msg) = match current_interval {
            Interval::Work => (WORK_LENGTH, "Work Interval"),
            Interval::Break => (BREAK_LENGTH, "Break Interval"),
        };

        for i in (0..=length).rev() {
            render_page(&mut terminal, &i, &mapper, Some(msg))?;

            match process_command_event(&rx) {
                Command::Quit => {
                    cleanup(&mut terminal)?;
                    break 'outer;
                }
                Command::QuitWithError(_) => {
                    cleanup(&mut terminal)?;
                    break 'outer;
                }
                Command::None => {}
            }
        }

        current_interval.switch();
    }

    cleanup(&mut terminal)?;

    Ok(())
}

fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    terminal.clear()?;

    Ok(())
}

fn process_command_event(rx: &mpsc::Receiver<Event>) -> Command {
    match rx.try_recv() {
        Ok(e) => match e {
            Event::KeyInput(event) => match (event.code, event.modifiers) {
                (KeyCode::Char('q'), m) if m.is_empty() => Command::Quit,
                (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => Command::Quit,
                _ => Command::None,
            },
            Event::Tick => Command::None,
        },
        Err(err) => {
            if err == mpsc::TryRecvError::Disconnected {
                Command::QuitWithError("mpsc channel disconnected")
            } else {
                Command::None
            }
        }
    }
}

fn render_clock<'a>(num_seconds: &u32, mapper: &TextMapper, header: Option<&'a str>) -> Text<'a> {
    let time = format!("{:0>2}:{:0>2}", num_seconds / 60, num_seconds % 60);
    let mut text = Text::default();

    for i in 0..=5 {
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

fn render_page(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    num_seconds: &u32,
    mapper: &TextMapper,
    header: Option<&str>,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let mut text_height = 6;

        if header.is_some() {
            text_height += 3;
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

enum Interval {
    Work,
    Break,
}

impl Interval {
    fn switch(&mut self) {
        match self {
            Self::Work => {
                let _ = mem::replace(self, Self::Break);
            }
            Self::Break => {
                let _ = mem::replace(self, Self::Work);
            }
        }
    }
}

enum Event {
    KeyInput(KeyEvent),
    Tick,
}

enum Command<'a> {
    None,
    QuitWithError(&'a str),
    Quit,
}
