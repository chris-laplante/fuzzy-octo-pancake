use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    println!("Progress bar");
    let len = 500;
    let style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {human_pos}/{human_len} {msg}")
        .unwrap()
        .progress_chars("##-");
    let bar = ProgressBar::new(len).with_style(style);

    for _ in 0..=len {
        sleep(Duration::from_millis(10));
        bar.inc(1);
    }
    bar.finish_with_message("Done");

    println!("Hello, world!");
}
