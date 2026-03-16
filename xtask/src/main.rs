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

        // Dioxus Development
        "dioxus-run" => {
            run_cargo_command(&["run", "--bin", "dev", "--", "web"]);
        }

        // General Commands
        "check-all" => run_cargo_command(&["check", "--workspace"]),
        "test-all" => run_cargo_command(&["test", "--workspace"]),
        "clean" => run_cargo_command(&["clean"]),

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
    println!();

    println!("Rust Commands:");
    println!("  check-all    # Check all crates");
    println!("  test-all     # Test all crates");
    println!("  clean        # Clean all targets");
    println!();

    println!("Usage:");
    println!("  cargo run -p xtask <command>");
    println!("  Or use the alias: cargo xtask <command>");
}

fn run_cargo_command(args: &[&str]) {
    println!("🔧 Running: cargo {}", args.join(" "));

    let status = Command::new("cargo")
        .args(args)
        .status()
        .expect("Failed to run cargo command");

    if !status.success() {
        exit(1);
    }
}

fn run_flutter_command(command: &str, args: &[String]) {
    dotenvy::dotenv().ok();

    let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let supabase_key = std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY not set");

    run_bash_command(format!("cd app-flutter && flutter {command} --dart-define=SUPABASE_URL={supabase_url} --dart-define=SUPABASE_ANON_KEY={supabase_key} {}", args.join(" ")).as_str());
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
