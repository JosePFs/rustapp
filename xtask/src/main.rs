use std::env;
use std::process::{Command, exit};

use owo_colors::{OwoColorize, Style};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return;
    }

    let (cmd_args, binary_args) =
        arguments(&args[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>());

    match args[0].as_str() {
        // Flutter Development
        "flutter-run" => {
            run_flutter_command("run", &cmd_args);
        }
        "flutter-linux" => {
            run_flutter_command("run -d linux", &cmd_args);
        }
        "flutter-build" => {
            run_flutter_command("build", &cmd_args);
        }
        "flutter-check" => {
            run_flutter_command("check", &cmd_args);
        }
        "flutter-test" => {
            run_flutter_command("test", &cmd_args);
        }

        // Flutter Bridge Development
        "frb-generate" => {
            run_bash_command(
                "flutter_rust_bridge_codegen generate \
                        --rust-root mobile-bridge-frb \
                        --rust-input crate::api \
                        --dart-root app-flutter \
                        --dart-output app-flutter/lib/src/rust",
            );
        }

        // Dioxus Development
        "dioxus-run" => {
            run_cargo_command("run", &["--bin", "dev", "--", "web"]);
        }
        "dioxus-check" => {
            run_dx_command("check", &cmd_args);
        }
        "dioxus-build" => {
            run_dx_command("build", &cmd_args);
        }

        // Domain Commands
        "domain-check" => run_cargo_command("check", &["--package", "domain"]),
        "domain-test" => run_cargo_command("test", &["--lib", "--package", "domain"]),
        "domain-build" => run_cargo_command("build", &["--release", "--package", "domain"]),

        // Application Commands
        "application-check" => run_cargo_command("check", &["--package", "application"]),
        "application-test" => run_cargo_command("test", &["--lib", "--package", "application"]),
        "application-build" => {
            run_cargo_command("build", &["--release", "--package", "application"])
        }

        // Infrastructure Commands
        "infrastructure-check" => run_cargo_command("check", &["--package", "infrastructure"]),
        "infrastructure-unit-test" => {
            run_cargo_command(
                "test",
                &[
                    "--lib",
                    &cmd_args.join(" "),
                    "--package",
                    "infrastructure",
                    "--",
                    &binary_args.join(" "),
                ],
            );
        }
        "infrastructure-test" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(
                "test",
                &[
                    "--tests",
                    &cmd_args.join(" "),
                    "--package",
                    "infrastructure",
                    "--",
                    &binary_args.join(" "),
                ],
            );
        }

        // Testing Commands
        "smoke-test" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(
                "test",
                &[
                    "--tests",
                    &cmd_args.join(" "),
                    "--package",
                    "infrastructure",
                    "--",
                    &binary_args.join(" "),
                ],
            );
        }
        "test-unit-all" => {
            run_cargo_command(
                "test",
                &[
                    "--tests",
                    &cmd_args.join(" "),
                    "--lib",
                    "--workspace",
                    "--",
                    &binary_args.join(" "),
                ],
            );
        }
        "test-all" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(
                "test",
                &[
                    "test",
                    "--tests",
                    &cmd_args.join(" "),
                    "--workspace",
                    "--",
                    &binary_args.join(" "),
                ],
            );
        }
        "test-all-docker" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_bash_command(&format!(
                "docker compose -f docker-compose.test.yml --env-file .env.test.local run --rm runner cargo test --tests {} --workspace -- {} --nocapture",
                cmd_args.join(" "),
                binary_args.join(" "),
            ));
        }

        // General Commands
        "check-all" => run_cargo_command("check", &["--workspace"]),
        "clean" => run_cargo_command("clean", &["--workspace"]),
        "help" | "--help" | "-h" => show_help(),

        cmd => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("Run 'cargo xtask [help|--help|-h]' for available commands");
            exit(1);
        }
    }
}

fn show_help() {
    let title_style = Style::new().bold().bright_cyan();
    let cmd_style = Style::new().bold().bright_green();
    let arg_style = Style::new().yellow();
    let comment_style = Style::new().dimmed();
    let header_style = Style::new().bold().bright_white();
    const PADDING: usize = 24;

    println!();
    println!("{}", " XTask Development CLI ".style(title_style));
    println!(
        "{}",
        "─────────────────────────────────────────────────────────".dimmed()
    );
    println!();

    let sections = [
        (
            "Flutter Commands",
            vec![
                ("flutter-run-linux", "Run the Flutter app on Linux"),
                ("flutter-run", "Run the Flutter app"),
                ("flutter-build", "Build the Flutter app"),
                ("flutter-check", "Check the Flutter app"),
                ("flutter-test", "Test the Flutter app"),
            ],
        ),
        (
            "Dioxus Commands",
            vec![
                ("dioxus-run", "Run the Dioxus app"),
                ("dioxus-check", "Check the Dioxus app"),
                ("dioxus-test", "Test the Dioxus app"),
                ("dioxus-build", "Build the Dioxus app"),
            ],
        ),
        (
            "General Commands",
            vec![
                ("check-all", "Check all crates"),
                ("clean", "Clean all targets"),
                ("help", "Show this help message"),
            ],
        ),
    ];

    for (section, commands) in &sections {
        println!("{}", section.style(header_style));
        for (cmd, desc) in commands {
            println!(
                "  {:<PADDING$} {}",
                cmd.style(cmd_style),
                format!("# {desc}").style(comment_style)
            );
        }
        println!();
    }

    println!("{}", "Testing Commands".style(header_style));
    let test_cmds = [
        ("test-unit-all", "Test all unit tests"),
        ("test-all", "Test all tests"),
        ("test-all-docker", "Test all in Docker"),
        ("test-all", "Test all"),
    ];
    let args_hint = "[test-name] [-- --extra-test-arguments]";
    for (cmd, desc) in &test_cmds {
        println!(
            "  {:<PADDING$} {} {}",
            cmd.style(cmd_style),
            args_hint.style(arg_style),
            format!("# {desc}").style(comment_style)
        );
    }
    println!();

    println!("{}", "Usage".style(header_style));
    println!(
        "  cargo xtask {} {}",
        "<command>".style(cmd_style),
        "[command-arguments] [-- --extra-binary-arguments]".style(arg_style),
    );
    println!();
}

fn run_dx_command(command: &str, args: &[String]) {
    run_bash_command(format!("dx {command} {}", args.join(" ")).as_str());
}

fn run_flutter_command(command: &str, args: &[String]) {
    run_bash_command(format!("cd app-flutter && flutter {command} {}", args.join(" ")).as_str());
}

fn run_bash_command(command_line: &str) {
    let mut command = Command::new("bash");
    command.arg("-c").arg(command_line);
    let status = command.status().expect("Failed to run bash command");

    if !status.success() {
        eprintln!("Script failed");
        exit(1);
    }
}

fn run_supabase_command(args: &[&str]) {
    let mut cmd = Command::new("npx");
    cmd.arg("supabase").args(args);

    let status = cmd.status().expect("Failed to run supabase via npx");

    if !status.success() {
        eprintln!("Supabase command failed");
        exit(1);
    }
}

fn run_cargo_command(command: &str, args: &[&str]) {
    let (cargo_args, binary_args) = arguments(args);
    let command = format!(
        "{command} {} -- {}",
        cargo_args.join(" "),
        binary_args.join(" ")
    );
    let status = Command::new("cargo")
        .args(command.split_whitespace().collect::<Vec<&str>>())
        .status()
        .expect("Failed to run cargo command");

    if !status.success() {
        exit(1);
    }
}

fn arguments(args: &[&str]) -> (Vec<String>, Vec<String>) {
    let separator = args.iter().position(|a| *a == "--");
    let (cargo_args, binary_args) = match separator {
        Some(pos) => (&args[..pos], &args[pos + 1..]),
        None => (&args[..], &[] as &[&str]),
    };

    (
        cargo_args.iter().map(|s| s.to_string()).collect(),
        binary_args.iter().map(|s| s.to_string()).collect(),
    )
}
