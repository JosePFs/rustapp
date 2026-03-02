//! Carga .env y expone SUPABASE_* en tiempo de compilación para que
//! option_env!() en el código (incl. WASM) tenga los valores al compilar.

use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=.env");

    let path = Path::new(".env");
    if !path.exists() {
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            if key == "SUPABASE_URL" || key == "SUPABASE_ANON_KEY" {
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    }
}
