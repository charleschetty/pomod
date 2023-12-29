use chrono::Duration;
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fs, io::Write, thread::sleep};
use pomod::timer;
use pomod::app;

//
fn main() {
    let args = app::Cli::parse();

    let config = {
        if args.config == "default" {
            let binding = match dirs::config_dir() {
                Some(dir) => dir.join("pomod/pomod.toml"),
                None => {
                    println!("Error finding config file !");
                    std::process::exit(1)
                }
            };
            let binding = binding.to_str().unwrap();
            binding.to_owned()
        } else {
            args.config
        }
    };
    let toml_str = match fs::read_to_string(config) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Error reading config file ! {}", e);
            std::process::exit(1);
        }
    };
    let toml_val = match toml::from_str::<app::Config>(&toml_str) {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing ! {}", e);
            std::process::exit(1);
        }
    };

    let time_to_second = toml_val.time.hours as u32 * 3600
        + toml_val.time.minutes as u32 * 60
        + toml_val.time.seconds as u32;

    match toml_val.display {
        app::Display::Txt => {
            let timer = timer::Timer::new(Duration::seconds(time_to_second.into()));
            if toml_val.cache == true {
                app::cache(&time_to_second);
            }
            for _i in 0..time_to_second {
                let time_str = timer.get_remaining_time();
                let mut stdout = std::io::stdout();
                write!(stdout, "\r{}", time_str).unwrap();
                stdout.flush().unwrap();
                sleep(std::time::Duration::from_secs(1))
            }
            app::process(&toml_val);
        }
        app::Display::Tui => {
            if toml_val.cache == true {
                app::cache(&time_to_second);
            }
            display_tui(&time_to_second, &toml_val);
        }
        app::Display::None => {
            if toml_val.cache == true {
                app::cache(&time_to_second);
            }
            sleep(std::time::Duration::from_secs(time_to_second.into()));
            app::process(&toml_val);
        }
    }
}

fn display_tui(time_to_second: &u32, cfg: &app::Config) {
    let timer = timer::Timer::new(Duration::seconds((*time_to_second).into()));
    enable_raw_mode().unwrap();
    crossterm::execute! {
        std::io::stdout(),
        EnterAlternateScreen,
        cursor::Hide
    }
    .unwrap();
    let mut breakout = false;

    for _i in 0..*time_to_second {
        timer.print_time();
        if event::poll(std::time::Duration::from_secs(1)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => {
                        breakout = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    if breakout == false {
        if cfg.notify.notify == true {
            app::notify(&cfg.notify.message);
        }
        if cfg.audio.play == true {
            app::play_audio(&cfg.audio.path, &cfg.audio.time);
        }
    }
    disable_raw_mode().unwrap();
    crossterm::execute! {
        std::io::stdout(),
        LeaveAlternateScreen,
        cursor::Show
    }
    .unwrap();
}
