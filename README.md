# MVP Phase 1 — Clinica Fisioterapia

Aplicación de clínica de fisioterapia (MVP Fase 1): frontend en **Dioxus (Rust)** y datos en **Supabase** (Auth + PostgreSQL). Sin backend dedicado en esta fase.

## Stack

- **Frontend:** Dioxus 0.7 (web)
- **Auth y base de datos:** Supabase (PostgreSQL + RLS)
- **Arquitectura:** Capas separadas (UI, servicios) para futura migración a backend Rust (Axum)

## Requisitos

- Rust (1.79+)
- Cuenta y proyecto en [Supabase](https://supabase.com)

## Configuración

1. **Variables de entorno**

   Crea un `.env` en la raíz del proyecto (o configura en tu entorno):

   ```env
   SUPABASE_URL=https://TU_PROYECTO.supabase.co
   SUPABASE_ANON_KEY=tu_anon_key
   ```

   Para desarrollo web con `dx serve`, puedes usar `Dioxus.toml` o inyectar estas variables en el build (p. ej. en el script de arranque).

2. **Base de datos**

   Ejecuta la migración SQL en el SQL Editor de Supabase:

   ```bash
   # Contenido en:
   supabase/migrations/001_initial_schema.sql
   ```

   Incluye tablas (`profiles`, `specialist_patients`, `programs`, `exercises`, `patient_programs`, `workout_sessions`), RLS y trigger de perfil en signup.

3. **Usuarios de prueba y asignación de rol**

   El trigger crea el perfil en `profiles` leyendo el rol de **User Metadata**. Valores válidos: `"specialist"` o `"patient"`. Si no pones nada, el rol por defecto es `"patient"`.

   **Desde el Dashboard de Supabase (Auth → Users → Add user):**
   - Rellena email y contraseña.
   - En **User Metadata** (campo JSON) escribe uno de estos:
     - Especialista: `{"role": "specialist"}`
     - Paciente: `{"role": "patient"}` o déjalo vacío / `{}`.
   - Guarda. El trigger creará la fila en `profiles` con ese rol.

   **Desde la API (signUp en tu app o con curl):**  
   En el cuerpo de la petición incluye `user_metadata`:
   - Especialista: `"user_metadata": { "role": "specialist" }`
   - Paciente: `"user_metadata": { "role": "patient" }` o omite `role`.

## Ejecución

```bash
# Instalar CLI de Dioxus (si no lo tienes)
cargo install dioxus-cli

# Añadir target wasm
rustup target add wasm32-unknown-unknown

# Servir la app web
dx serve
```

Abre la URL que indique el CLI (p. ej. `http://127.0.0.1:8080`).

## Estructura del proyecto

```
src/
  main.rs           # Entrada, router, contexto (config + sesión)
  pages/            # Pantallas (login, dashboards, editor de programa)
  components/       # Componentes reutilizables
  services/         # Acceso a datos (Supabase client, data)
supabase/
  migrations/       # SQL del esquema y RLS
docs/               # Especificaciones (p. ej. MVP_Phase_1_PROMPT.md)
```

## Funcionalidades (MVP Phase 1)

- **Login** con email/contraseña (Supabase Auth). Tras login, redirección por rol (especialista o paciente).
- **Especialista:** listar y añadir pacientes (por email), crear programas, añadir ejercicios, asignar programa a paciente, ver cumplimiento básico.
- **Paciente:** ver programa activo y ejercicios, marcar sesión del día como completada y enviar feedback (esfuerzo 1–10, dolor 0–10, comentario libre).

## Licencia

Privado / según el proyecto.
