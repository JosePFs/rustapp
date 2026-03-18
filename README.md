# MVP Phase 1 — Physiotherapy Clinic

Physiotherapy clinic application (MVP Phase 1): web backoffice frontend in **Dioxus (Rust)**, mobile app frontend in **Flutter**, and data in **Supabase** (Auth + PostgreSQL). No dedicated backend in this phase.

## Stack

- **Frontend:** Dioxus 0.7 (web) for the backoffice used by specialists, and Flutter for the mobile app used by patients to perform their exercises.
- **Auth and database:** Supabase (PostgreSQL + RLS).
- **Architecture:** DDD + hexagonal architecture, with support for a future migration to a Rust backend (Axum).

## Requirements

- Rust (1.79+)
- Dioxus 0.7
- Flutter 3.41
- Account and project in [Supabase](https://supabase.com)

## Configuration

1. **Environment variables**

   Create a `.env` file in the project root (or configure them in your environment):

   ```env
   SUPABASE_URL=https://YOUR_PROJECT.supabase.co
   SUPABASE_ANON_KEY=your_anon_key
   ```

   For web development with `dx serve`, you can use `Dioxus.toml` or inject these variables into the build (e.g. in the startup script).

2. **Database**

   Run the SQL migration in Supabase’s SQL Editor:

   ```bash
   # Content in:
   supabase/migrations/001_initial_schema.sql
   ```

   It includes tables (`profiles`, `specialist_patients`, `programs`, `exercises`, `patient_programs`, `workout_sessions`), RLS, and the profile trigger on signup.

3. **Test users and role assignment**

   The trigger creates the profile in `profiles` by reading the role from **User Metadata**. Valid values: `"specialist"` or `"patient"`. If you don’t set anything, the default role is `"patient"`.

   **From the Supabase Dashboard (Auth → Users → Add user):**
   - Fill in email and password.
   - In **User Metadata** (JSON field) enter one of:
     - Specialist: `{"role": "specialist"}`
     - Patient: `{"role": "patient"}` or leave it empty / `{}`.
   - Save. The trigger will create the row in `profiles` with that role.

   **From the API (signUp in your app or with curl):**  
   In the request body include `user_metadata`:
   - Specialist: `"user_metadata": { "role": "specialist" }`
   - Patient: `"user_metadata": { "role": "patient" }` or omit `role`.

## Running Backoffice Dioxus

```bash
# Install Dioxus CLI (if you don't have it)
cargo install dioxus-cli

# Add wasm target
rustup target add wasm32-unknown-unknown

# Serve the backoffice web app
cargo xtask dioxus-run
```

Open the URL shown by the CLI (e.g. `http://127.0.0.1:8080`).

## Running Flutter

```bash
# Launch app in linux
cargo xtask flutter-linux

# Launch app in mobile (-d <ID_DEL_DISPOSITIVO> when several devices available)
cargo xtask flutter-run

# Open devtools
flutter devtools

# Also: VS Code: Ctrl+Shift+P -> “Dart: Open DevTools”.
```

## Project layout

- `domain`: business entities and domain logic
- `application`: use cases and application services, orchestrating domain and ports
- `infrastructure`: adapters for Supabase, Flutter, Dioxus, and other I/O
- `backoffice-dioxus`: web backoffice frontend in Dioxus
- `app-flutter`: mobile app frontend in Flutter
- `mobile-bridge-frb`: Rust FFI bridge compiled as a native library for Flutter
- `supabase`: SQL migrations and database schema
- `xtask`: helper CLI tasks for local development (running frontends, building bridges, etc.)
- [ARCHITECTURE.md](ARCHITECTURE.md): architecture overview with component and sequence diagrams

## Features (MVP Phase 1)

- **Login** with email/password (Supabase Auth). After login, redirect by role (specialist or patient).
- **Specialist:** list and add patients (by email), create programs, add exercises, assign a program to a patient, view basic adherence/compliance.
- **Patient:** view active program and exercises, mark the day’s session as completed, and send feedback (effort 1–10, pain 0–10, free-text comment).

## License

Dual-licensed under **AGPL-3.0** and a **commercial license**.

- See [LICENSE](LICENSE) for the AGPL-3.0 open-source terms.
- See [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) for commercial licensing terms.
