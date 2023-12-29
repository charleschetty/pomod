use chrono::{Duration, NaiveDateTime, Utc};
use std::fs;
use std::process;
use std::thread::sleep;

fn get() {
    let cache_file = match dirs::cache_dir() {
        Some(dir) => dir.join(".pomod"),
        None => {
            println!("Error finding cache dir");
            std::process::exit(1)
        }
    };
    let contents = fs::read_to_string(&cache_file);

    match contents {
        Ok(val) => {
            let content: Vec<&str> = val.split(' ').collect();
            let stamp = content.get(0).unwrap().parse::<i64>().unwrap();
            let till_stamp = content.get(1).unwrap().parse::<i64>().unwrap();
            let diff_sec = till_stamp - stamp;

            let s = NaiveDateTime::from_timestamp_opt(stamp, 0).unwrap();
            let now = Utc::now().naive_utc();
            let duration = now.signed_duration_since(s);

            if duration.num_seconds() >= diff_sec {
                print!("00:00\n");
                sleep(std::time::Duration::from_secs(1));
                return;
            }

            let remaining = Duration::seconds(diff_sec - duration.num_seconds());

            print!(
                "{:0>2}:{:0>2}\n",
                remaining.num_minutes(),
                remaining.num_seconds() % 60,
            );
            sleep(std::time::Duration::from_secs(1));
        }
        Err(err) => {
            println!("Error reading file: {}", err);
            process::exit(1)
        }
    }
}
fn main() {
    get();
}
