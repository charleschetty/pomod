use cfonts::{render, Align, Options};
use chrono::{DateTime, Duration, Local};
use crossterm::{
    cursor,
    style::Print,
    terminal::{self},
    QueueableCommand,
};
use std::{fmt::Write as _, io::Write};
pub struct Timer {
    total: Duration,
    started_at: DateTime<Local>,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            total: duration,
            started_at: Local::now(),
        }
    }

    pub fn get_remaining_time(&self) -> String {
        let time_passed = Local::now() - self.started_at;
        let time_remain = self.total - time_passed;

        let time_days = time_remain.num_days();
        let time_hours = time_remain.num_hours();
        let time_minutes = time_remain.num_minutes();
        let time_seconds = time_remain.num_seconds();

        let mut result = String::new();

        fn append_number(s: &mut String, num: i64) {
            if s.is_empty() {
                let _ = write!(s, "{}", num);
            } else {
                let _ = write!(s, "{:02}", num);
            }
        }

        if time_days > 0 {
            let _ = write!(result, "{}:", time_days);
        }
        if time_hours > 0 {
            append_number(&mut result, time_hours % 24);
            result.push(':');
        }
        append_number(&mut result, time_minutes % 60);
        result.push(':');
        let _ = write!(result, "{:02}", time_seconds % 60);
        result
    }
    fn get_time_str(&self) -> String {
        let result = self.get_remaining_time();
        let time_str = render(Options {
            text: result,
            align: Align::Center,
            font: cfonts::Fonts::FontBlock,
            gradient: Vec::new(),
            colors: vec![cfonts::Colors::System],
            ..Options::default()
        });
        time_str.text
    }

    pub fn print_time(&self) {
        let time_str: Vec<String> = self
            .get_time_str()
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect();

        let highest_col = time_str.len();
        let (width, height) = terminal::size().unwrap();
        let (width, height) = (width as usize, height as usize);

        if width < 20 || height < 5 {
            panic!("Window is too small")
        };
        let mut stdout = std::io::stdout();

        crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All),).unwrap();
        for (line_nr, line) in time_str.iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(
                    0,
                    (line_nr + (height - highest_col) / 2).try_into().unwrap(),
                ))
                .unwrap();
            crossterm::queue!(stdout, Print(line),).unwrap();
        }
        stdout.flush().unwrap();
    }
}
