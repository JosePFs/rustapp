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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CommandKind::Web => {
            build_app_css();
            start_css_watcher();

            let mut cmd = Command::new("dx");
            cmd.arg("serve").arg("--port").arg("8080");
            run_or_panic("dx serve (web)", cmd);
        }
    }
}

fn run_or_panic(context: &str, mut cmd: Command) {
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
        "cat backoffice-dioxus/assets/dx-components-theme.css \
             backoffice-dioxus/assets/tailwind.css \
             backoffice-dioxus/assets/styling/main.css \
             > backoffice-dioxus/assets/app.css",
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
        "backoffice-dioxus/assets/dx-components-theme.css",
        "backoffice-dioxus/assets/tailwind.css",
        "backoffice-dioxus/assets/styling/main.css",
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
