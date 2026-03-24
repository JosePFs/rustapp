use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return;
    }

    match args[0].as_str() {
        // Flutter Development
        "flutter-run" => {
            run_flutter_command("run", &args[1..]);
        }
        "flutter-linux" => {
            run_flutter_command("run -d linux", &args[1..]);
        }
        "flutter-build" => {
            run_flutter_command("build", &args[1..]);
        }
        "flutter-check" => {
            run_flutter_command("check", &args[1..]);
        }
        "flutter-test" => {
            run_flutter_command("test", &args[1..]);
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
            run_cargo_command(&["run", "--bin", "dev", "--", "web"]);
        }
        "dioxus-check" => {
            run_cargo_command(&["check"]);
        }
        "dioxus-test" => {
            run_cargo_command(&["test"]);
        }
        "dioxus-build" => {
            run_cargo_command(&["build", "--release"]);
        }

        // Domain Commands
        "domain-check" => run_cargo_command(&["check", "--package", "domain"]),
        "domain-test" => run_cargo_command(&["test", "--lib", "--package", "domain"]),
        "domain-build" => run_cargo_command(&["build", "--release", "--package", "domain"]),

        // Application Commands
        "application-check" => run_cargo_command(&["check", "--package", "application"]),
        "application-test" => run_cargo_command(&["test", "--lib", "--package", "application"]),
        "application-build" => {
            run_cargo_command(&["build", "--release", "--package", "application"])
        }

        // Infrastructure Commands
        "infrastructure-check" => run_cargo_command(&["check", "--package", "infrastructure"]),
        "infrastructure-test" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(&[
                "test",
                "--tests",
                &args[1..].join(" "),
                "--package",
                "infrastructure",
            ]);
        }

        // Testing Commands
        "test-all-unit" => run_cargo_command(&[
            "test",
            "--tests",
            &args[1..].join(" "),
            "--lib",
            "--workspace",
        ]),
        "test-all-integration" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(&["test", "--tests", &args[1..].join(" "), "--workspace"]);
        }
        "test-all" => {
            run_cargo_command(&["test", "--lib", "--workspace"]);
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            run_cargo_command(&[
                "test",
                "--tests",
                &args[1..].join(" "),
                "--package",
                "infrastructure",
            ]);
        }
        "test-all-docker" => {
            run_supabase_command(&["start"]);
            run_supabase_command(&["db", "reset"]);
            let test_args = args[1..].join(" ");
            run_bash_command(&format!(
                "docker compose -f docker-compose.test.yml --env-file .env.test.local run --rm runner cargo test --tests {} --workspace -- --nocapture",
                test_args
            ));
        }

        // General Commands
        "check-all" => run_cargo_command(&["check", "--workspace"]),
        "clean" => run_cargo_command(&["clean", "--workspace"]),

        "help" | "--help" | "-h" => show_help(),

        cmd => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("Run 'cargo run -p cli help' for available commands");
            exit(1);
        }
    }
}

fn show_help() {
    println!("XTask Development CLI");
    println!();

    println!("Flutter Commands:");
    println!("  flutter-run-linux # Run the Flutter app on Linux");
    println!("  flutter-run       # Run the Flutter app");
    println!("  flutter-build     # Build the Flutter app");
    println!("  flutter-check     # Check the Flutter app");
    println!("  flutter-test      # Test the Flutter app");
    println!();

    println!("Dioxus Commands:");
    println!("  dioxus-run        # Run the Dioxus app");
    println!("  dioxus-check      # Check the Dioxus app");
    println!("  dioxus-test       # Test the Dioxus app");
    println!("  dioxus-build      # Build the Dioxus app");
    println!();

    println!("Testing Commands:");
    println!("  test-all-unit         [--test-name] # Test all units");
    println!("  test-all-integration  [--test-name] # Test all integrations");
    println!("  test-all-docker       [--test-name] # Test all in Docker");
    println!("  test-all              [--test-name] # Test all");
    println!();

    println!("General Commands:");
    println!("  check-all         # Check all crates");
    println!("  clean             # Clean all targets");
    println!();

    println!("Usage:");
    println!("  cargo xtask <command>");
}

fn run_cargo_command(args: &[&str]) {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .expect("Failed to run cargo command");

    if !status.success() {
        exit(1);
    }
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
