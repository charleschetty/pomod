use clap::Parser;
use notify_rust::Notification;
use rodio::{source::Source, Decoder, OutputStream};
use serde::Deserialize;
use std::{fs::File, io::BufReader};

use std::io::Write;
#[derive(Parser)]
/// A simple timer
pub struct Cli {
    /// config file
    #[arg(long, short, default_value = "default")]
    pub config: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub time: Time,
    pub display: Display,
    pub audio: Audio,
    pub notify: Notify,
    pub cache: bool,
}

#[derive(Deserialize)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

#[derive(Deserialize)]
pub enum Display {
    Txt,
    Tui,
    None,
}

#[derive(Deserialize)]
pub struct Audio {
    pub play: bool,
    pub path: String,
    pub time: u8,
}

#[derive(Deserialize)]
pub struct Notify {
    pub notify: bool,
    pub message: String,
}

pub fn play_audio(path: &String, time: &u8) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening media file ! {}", e);
            std::process::exit(1);
        }
    };
    let file = BufReader::new(file);
    let source = Decoder::new(file).unwrap();
    let _source = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs((*time).into()));
}

pub fn notify(message: &str) {
    match Notification::new().summary(&message).show() {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending notification: {}", e);
            std::process::exit(1);
        }
    }
}
pub fn process(cfg: &Config) {
    if cfg.notify.notify == true {
        notify(&cfg.notify.message);
    }
    if cfg.audio.play == true {
        play_audio(&cfg.audio.path, &cfg.audio.time);
    }
}

pub fn cache(time_to_second: &u32) {
    let cache_file = match dirs::cache_dir() {
        Some(dir) => dir.join(".pomod"),
        None => {
            println!("Error finding cache dir");
            std::process::exit(1)
        }
    };

    let mut file = match std::fs::File::create(cache_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            std::process::exit(1)
        }
    };

    let time_now = chrono::Utc::now().timestamp();
    let time_end = time_now + *time_to_second as i64;

    let str_time = format!("{} {}", time_now, time_end);

    match file.write(str_time.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error writing file: {}", e);
            std::process::exit(1)
        }
    }

    file.flush().unwrap();
}
