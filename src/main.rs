//! a little tool to read out the clipboard content
//! using `xclip -o` and `espeak`
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

use chrono;
use colored::Colorize;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Time interval for checking clipboard content in milliseconds.
    #[arg(short, long, default_value = "300")]
    time: u64,
    /// The selection of `xclip` to read from.
    #[arg(short, long, default_value = "clipboard")]
    selection: Selection,
    /// If true, when the clipboard is not empty, whether to read it out immediately.
    #[arg(short, long, default_value = "false")]
    initial: bool,
}

// "primary", "secondary", "clipboard" or "buffer-cut"
#[derive(Debug, Clone, clap::ValueEnum)]
enum Selection {
    Primary,
    Secondary,
    Clipboard,
    BufferCut,
}

impl ToString for Selection {
    fn to_string(&self) -> String {
        match self {
            Selection::Primary => "primary".to_string(),
            Selection::Secondary => "secondary".to_string(),
            Selection::Clipboard => "clipboard".to_string(),
            Selection::BufferCut => "buffer-cut".to_string(),
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    let time = args.time;
    let selection = args.selection.to_string();
    let initial = args.initial;

    let mut memory = Box::new(String::new());
    let mut child: Option<Child> = None;

    println!("{} Start listening to the clipboard, press {} to exit.", get_datetime().green(), "Ctrl+C".yellow());

    loop {
        let stdout = Command::new("xclip")
            .arg("-o")
            .args(&["-selection", selection.as_str()])
            .output()
            .expect("failed to execute process xclip");
        let input = String::from_utf8_lossy(&stdout.stdout)
            .to_string()
            .trim()
            .to_string();

        // skip the first loop if the clipboard is not empty and initial is false
        if !initial && !input.is_empty() && memory.len() == 0 {
            *memory = input.clone();
            thread::sleep(Duration::from_millis(time));
            continue;
        }

        if input != *memory {
            *memory = input.clone();
            if let Some(child) = &mut child {
                if child.try_wait().unwrap().is_none() {
                    child.kill().expect("failed to kill child");
                    println!(
                        "{} {}",
                        get_datetime().green(),
                        "Interrupted.".blue(),
                    );
                }
            }
            println!(
                "{} {} {}",
                get_datetime().green(),
                "Speaking:".yellow().bold(),
                input
            );
            child = Some(
                Command::new("espeak")
                    .arg(&input)
                    .spawn()
                    .expect("failed to execute process"),
            );
        }
        thread::sleep(Duration::from_millis(time));
    }
}

fn get_datetime() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}