use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

use clap::{Parser, Subcommand};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    Web,
    Android {
        #[arg(long, default_value = "ebecad4a")]
        device: String,
    },
}

fn main() {
    let cli = Cli::parse();

    build_app_css();

    start_css_watcher();

    match cli.command {
        CommandKind::Web => {
            let mut cmd = Command::new("dx");
            cmd.arg("serve");
            run_or_panic("dx serve (web)", &mut cmd);
        }
        CommandKind::Android { device } => {
            let mut cmd = Command::new("dx");
            cmd.arg("serve")
                .arg("--platform")
                .arg("android")
                .arg("--device")
                .arg(device);
            run_or_panic("dx serve (android)", &mut cmd);
        }
    }
}

fn run_or_panic(context: &str, cmd: &mut Command) {
    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("Error running {context}: {e}"));
    if !status.success() {
        panic!("{context} finished with code {:?}", status.code());
    }
}

fn build_app_css() {
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(
        "cat assets/dx-components-theme.css \
             assets/tailwind.css \
             assets/styling/main.css \
             > assets/app.css",
    );
    if let Err(e) = cmd.status() {
        eprintln!("Error generating app.css: {e}");
    }
}

fn start_css_watcher() {
    let (tx, rx) = channel();

    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("failed to create CSS watcher");

    for path in [
        "assets/dx-components-theme.css",
        "assets/tailwind.css",
        "assets/styling/main.css",
    ] {
        if let Err(e) = watcher.watch(Path::new(path), RecursiveMode::NonRecursive) {
            eprintln!("Failed to watch {path}: {e}");
        }
    }

    thread::spawn(move || {
        let _watcher = watcher;

        let mut last_rebuild = Instant::now();
        let debounce = Duration::from_millis(300);

        while let Ok(res) = rx.recv() {
            match res {
                Ok(event) => {
                    if !matches!(event.kind, EventKind::Modify(_)) {
                        continue;
                    }

                    let now = Instant::now();
                    if now.duration_since(last_rebuild) < debounce {
                        continue;
                    }

                    build_app_css();
                    last_rebuild = now;
                }
                Err(e) => {
                    eprintln!("CSS watcher error: {e}");
                }
            }
        }
    });
}
