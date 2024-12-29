extern crate bad_apple;
extern crate crossterm;
extern crate image;
extern crate ratatui;

use crossterm::event::{self, KeyCode};
use crossterm::terminal::{self};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, io};

use bad_apple::{generate_ascii, get_img, grayscale_and_resize};
fn main() -> Result<(), io::Error> {
    let mut framerate: u128 = 60;
    let chars: [&str; 6] = [" ", ".", ",", "*", "@", "#"];
    let mut current_frame: u16 = 1;
    let mut size = terminal::size();
    let args: Vec<String> = env::args().collect();

    let arg = args.iter().find(|x| x.len() < 3);
    match arg {
        None => {}
        Some(arg) => {
            if let Ok(fps) = arg.parse() {
                framerate = fps;
            }
        }
    }
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut scroll_offset = 0;
    let mut ascii = String::new();
    let mut t2 = Instant::now();
    let mut duration = Duration::new(0, 0);
    let mut duration2 = Duration::new(0, 0);
    loop {
        let t = Instant::now();
        match terminal.size() {
            Ok(mut size) => {
                size.width = size.width - 2;
                size.height = size.height - 2;
                ascii = generate_ascii(
                    grayscale_and_resize(
                        get_img(&mut current_frame).unwrap(),
                        size.width as u32,
                        size.height as u32,
                    ),
                    size.width as u32,
                    size.height as u32,
                    chars,
                );
                let elapsed_string = format!(
                    "{} {}{}{}{} {}",
                    "-----",
                    duration.as_millis().to_string(),
                    "+",
                    { duration2.as_millis().to_string() },
                    { "ms" },
                    "-----"
                );
                ascii.replace_range(
                    ascii.len() - (elapsed_string.len() / 2) - 1 - (size.width / 2) as usize
                        ..ascii.len() - 1 - (size.width / 2) as usize,
                    elapsed_string.as_str(),
                );
            }
            Err(_) => {
                println!("error")
            }
        }

        duration = t.elapsed();

        t2 = Instant::now();
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let paragraph = Paragraph::new(&*ascii)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::White))
                .scroll((scroll_offset, 0)); // Scroll only vertically

            f.render_widget(paragraph, chunks[0]);
        })?;
        duration2 = t2.elapsed();

        if let Ok(true) = event::poll(Duration::from_millis(0)) {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if scroll_offset > 0 {
                            scroll_offset -= 1;
                        }
                    }
                    KeyCode::Down => {
                        scroll_offset += 1;
                    }
                    _ => {}
                }
            }
        }
        let time_to_sleep: f32 = ((1f32 / framerate as f32) * 1000f32
            - (duration.as_millis() as f32 + duration2.as_millis() as f32));
        if time_to_sleep > 0f32 {
            sleep(Duration::from_millis(time_to_sleep as u64));
        }
    }

    // Restore terminal settings
    disable_raw_mode()?;
    Ok(())
}
