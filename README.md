# MVP Phase 1 — Physiotherapy Clinic

Physiotherapy clinic application (MVP Phase 1): backoffice in **Dioxus (Rust)**, frontoffice mobile app in **Flutter**, and data in **Supabase** (Auth + PostgreSQL). No dedicated backend in this phase.

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

## Running

```bash
# Install Dioxus CLI (if you don't have it)
cargo install dioxus-cli

# Add wasm target
rustup target add wasm32-unknown-unknown

# Serve the backoffice web app
cargo dev-web
```

Open the URL shown by the CLI (e.g. `http://127.0.0.1:8080`).

## Project structure in virtual workspace

```
application    # Core
domain         # Core
infrastructure # Adapters for Supabase, Flutter and Dioxus
```

## Features (MVP Phase 1)

- **Login** with email/password (Supabase Auth). After login, redirect by role (specialist or patient).
- **Specialist:** list and add patients (by email), create programs, add exercises, assign a program to a patient, view basic adherence/compliance.
- **Patient:** view active program and exercises, mark the day’s session as completed, and send feedback (effort 1–10, pain 0–10, free-text comment).

## License

Private / project-specific.
