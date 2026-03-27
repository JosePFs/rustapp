# MVP Phase 1 - Eixe: Physiotherapy Clinic App

Application is divided into:

- A web-based back office for patient management, scheduling routines, and tracking exercise performance, effort, and progress.
- A mobile application for patients where they can view videos of their scheduled routines, follow exercise instructions, and provide feedback on their performance.

## Stack

- **Frontend:** Dioxus 0.7 (web) for the backoffice used by specialists, and Flutter for the mobile app used by patients to perform their exercises.
- **Auth and database:** Supabase (PostgreSQL + RLS).
- **Architecture:** DDD + hexagonal architecture, with support for a future migration to a Rust backend (Axum).

## Requirements

- Rust (1.79+)
- Dioxus 0.7
- Flutter 3.41
- Have an account and project in [Supabase](https://supabase.com) and supabase [cli](https://supabase.com/docs/guides/local-development/cli/install)

## Configuration

1. **Environment variables**

   Create a `.env` file in the project root (or configure them in your environment):

   ```env
   SUPABASE_URL=https://YOUR_PROJECT.supabase.co
   SUPABASE_ANON_KEY=your_anon_key
   ```

   For testing, a `.env.test.local` file. `.env.test.example` is provided as template.

2. **Database**

   Run the SQL migration in Supabase’s SQL Editor:

   ```bash
   # Content in:
   supabase/migrations/20260324120000_init.sql
   ```

3. **Test users and role assignment**

   For the time being, the Supabase dashboard is used to create users and roles.

   A trigger creates the profile in `profiles` by reading the role from **User Metadata**.
   Valid values: `"specialist"` or `"patient"`. If you don’t set anything, the default role is `"patient"`.

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

# Launch app in mobile (-d <DEVICE_ID> when several devices available)
cargo xtask flutter-run

# Open devtools
flutter devtools

# Also: VS Code: Ctrl+Shift+P -> “Dart: Open DevTools”.
```

## Running API

```bash
# Launch server
cargo xtask api-run
```

```bash
# To hot-reload development
cargo install cargo-watch

# Run the server with hot-reload
cargo xtask api-watch

# Run the tests with hot-reload
cargo xtask api-watch-test
```

## Running tests

```bash
# To check whether supabase is running or not
cargo xtask smoke-test [-- --extra-test-arguments]

# By default runs all the tests of a specific type, but it can be passed as a second argument
cargo xtask [test-all|test-all-unit|test-all-integration|test-all-docker|api-unit-test|api-test] [test-name] [-- --extra-test-arguments]
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

For more detail, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Features (MVP Phase 1)

- **Login** with email/password (Supabase Auth).
- **Specialist:** list and add patients (by email), create programs, add exercises, assign a program to a patient, view basic adherence/compliance.
- **Patient:** view active program and exercises, mark the day’s session as completed, and send feedback (effort 1–10, pain 0–10, free-text comment).

## License

Dual-licensed under **AGPL-3.0** and a **commercial license**.

- See [LICENSE](LICENSE) for the AGPL-3.0 open-source terms.
- See [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) for commercial licensing terms.
