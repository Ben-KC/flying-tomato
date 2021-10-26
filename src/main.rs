mod commands;
mod font;
mod interval;
mod render;

use commands::{process_command_event, Command, Event};
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use font::TextMapper;
use interval::Interval;
use render::{cleanup, render_page};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    enable_raw_mode().expect("Could not enable raw mode");

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
    let mut terminal = Terminal::new(backend).expect("Could not create terminal");

    terminal.clear().expect("Could not clear terminal");

    let mapper = TextMapper::new();

    let mut current_interval = Interval::Work;

    'outer: for _ in 0..(NUM_INTERVALS * 2) {
        let (length, msg) = match current_interval {
            Interval::Work => (WORK_LENGTH, "Work Interval"),
            Interval::Break => (BREAK_LENGTH, "Break Interval"),
        };

        for i in (0..=length).rev() {
            render_page(&mut terminal, &i, &mapper, Some(msg)).expect("Could not render page");

            match process_command_event(&rx) {
                Command::Quit => {
                    cleanup(&mut terminal).expect("Could not clean up terminal");
                    break 'outer;
                }
                Command::QuitWithError(_) => {
                    cleanup(&mut terminal).expect("Could not clean up terminal");
                    break 'outer;
                }
                Command::None => {}
            }
        }

        current_interval.switch();
    }

    cleanup(&mut terminal).expect("Could not clean up terminal");

    Ok(())
}
