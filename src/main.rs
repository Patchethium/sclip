//! a small tool to read out the clipboard content
//! using `xclip -o` and `espeak`
use chrono;
use clap::Parser;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};
#[derive(Parser, Debug)]
struct Args {
    /// Time interval for checking clipboard content, in milliseconds
    #[arg(short, long)]
    time: Option<u64>,
    // The selection to read from, primary or clipboard 
}

fn main() {
    let args = Args::parse();
    let time = args.time.unwrap_or(300);
    let memory = Arc::new(Mutex::new(String::new()));
    loop {
        let output = std::process::Command::new("xclip")
            .arg("-o")
            .arg("-selection")
            .arg("clipboard")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        let mut memory = memory.lock().unwrap();
        if output != *memory {
            *memory = output.to_string();
            let trimmed = output.trim();
            let datetime = chrono::Local::now();
            println!(
                "{} {}",
                datetime.format("%Y-%m-%d %H:%M:%S").to_string().purple(),
                "Speaking".green()
            );
            if trimmed.len() > 0 {
                println!("{}", trimmed);
                std::process::Command::new("espeak")
                    .arg(trimmed)
                    .output()
                    .expect("failed to execute process");
            }
        } else {
            // do nothing
        }
        std::thread::sleep(std::time::Duration::from_millis(time));
    }
}
