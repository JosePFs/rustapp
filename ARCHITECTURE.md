# Architecture

This document describes the architecture of **Eixe**, a physiotherapy clinic management system built with Rust (backend) and Flutter (mobile) / Dioxus (web).

## Overview

Eixe is a **Domain-Driven Design (DDD)** system using **Hexagonal Architecture** (ports and adapters). The codebase is organized into distinct layers that enforce separation of concerns:

- **Domain**: Pure business logic and entities
- **Application**: Use cases and orchestration
- **Infrastructure**: Adapters, persistence, external integrations
- **UI**: Flutter mobile app and Dioxus web backoffice

## Architecture Diagram

```mermaid
graph TB
    subgraph "Mobile Clients"
        APP["📱 Flutter App\n(app-flutter)"]
    end

    subgraph "Web Clients"
        BACKOFFICE["🖥️ Dioxus Backoffice\n(backoffice-dioxus)"]
    end

    subgraph "Rust Bridge"
        FRB["🌉 FFI Bridge\n(mobile-bridge-frb/src/api.rs)"]
    end

    subgraph "Core Layers"
        APP_LAYER["Application Layer\n(application/src)\n\nUse Cases • Orchestration"]
        DOMAIN["Domain Layer\n(domain/src)\n\nEntities • Business Rules"]
    end

    subgraph "Infrastructure"
        INFRA["Infrastructure Layer\n(infrastructure/src)\n\nSupabase Adapter\nPort Implementations"]
    end

    subgraph "External Services"
        DB["🗄️ Supabase\n(PostgreSQL + Auth)"]
    end

    APP -->|FRB Calls| FRB
    BACKOFFICE -->|HTTP Calls| APP_LAYER
    FRB -->|Use Cases| APP_LAYER
    APP_LAYER -->|Domain Rules| DOMAIN
    APP_LAYER -->|Ports| INFRA
    INFRA -->|Adapters| DB
```

## Layer Descriptions

### Domain Layer (`domain/`)

The **heart of the system**. Contains all business logic and domain concepts.

**Responsibilities:**

- Define domain entities and value objects
- Enforce business invariants
- Model domain errors
- Aggregate roots and their rules

**Key Modules:**

- `entities.rs`: Domain entities (Program, Patient, Workout, Exercise, Session, etc.)
- `user.rs`: User abstraction
- `profile.rs`: User profile (Specialist or Patient)
- `session.rs`: Authentication session
- `credentials.rs`: Login credentials
- `role.rs`: User roles (Specialist, Patient)
- `email.rs`: Email value object
- `id.rs`: Identifier value objects
- `error.rs`: Domain errors and Result type

**Design Principles:**

- ✅ Domain is **framework-agnostic**
- ✅ No dependencies on UI or infrastructure
- ✅ Business rules encoded in types and constructors
- ✅ Immutable entities where possible

### Application Layer (`application/`)

**Orchestrates use cases** by composing domain and infrastructure.

**Responsibilities:**

- Implement use cases (application workflows)
- Define ports (trait abstractions)
- Orchestrate domain logic and adapters
- Handle cross-cutting concerns (security, validation)

**Key Components:**

#### Use Cases (`use_cases/`)

Each file represents a specific user workflow:

| Use Case                                                       | Purpose                                  |
| -------------------------------------------------------------- | ---------------------------------------- |
| `login.rs`                                                     | Authenticate specialist/patient          |
| `mobile_login.rs`                                              | Mobile-specific login (with Send bounds) |
| `get_patient_programs.rs`                                      | Retrieve assigned programs for patient   |
| `mobile_get_patient_programs.rs`                               | Mobile variant with Send bounds          |
| `submit_patient_workout_feedback.rs`                           | Log exercise effort/pain feedback        |
| `mobile_submit_patient_workout_feedback.rs`                    | Mobile variant                           |
| `assign_program_to_patient.rs`                                 | Specialist assigns program to patient    |
| `create_program.rs`, `create_workout.rs`, `create_exercise.rs` | Creation workflows                       |
| `patient_progress.rs`                                          | Analytics for specialist dashboard       |
| `agenda_schedule.rs`                                           | Agenda/calendar views                    |

**Use Case Pattern:**

Use cases depend on **domain repository traits** (outbound/driven ports), with the smallest bound that matches their calls—e.g. read-only flows use only `SpecialistCatalogReadRepository`; mutations use `SpecialistCatalogWriteRepository`; patient session flows use `PatientSessionWriteRepository`. A single concrete adapter (`Api` / `NativeApi` → `SupabaseRestRepository`) implements all of them.

```rust
pub struct ExampleReadUseCase<R: SpecialistCatalogReadRepository> {
    catalog_read: Arc<R>,
}

pub struct ExampleWriteUseCase<W: SpecialistCatalogWriteRepository> {
    catalog_write: Arc<W>,
}
```

#### Ports (`ports/`)

Application **ports** split into **driven (outbound)** and **driving (inbound)** concerns:

| Kind                      | Where defined                                                        | Who implements                                   | Who consumes                                                   |
| ------------------------- | -------------------------------------------------------------------- | ------------------------------------------------ | -------------------------------------------------------------- |
| **Driven**                | `domain::repositories::*` (read/write/patient session)               | `infrastructure` (e.g. `SupabaseRestRepository`) | **Use cases**                                                  |
| **Driving (inbound API)** | `application/src/ports/api.rs` — traits `BackofficeApi`, `MobileApi` | **`application`** via facades                    | **Inbound adapters**: `backoffice-dioxus`, `mobile-bridge-frb` |

**Inbound traits (`api.rs`):** Each async method mirrors a use case’s `execute` surface (same `*Args` / result types from `use_cases::…`). Dioxus and the FRB bridge, and any new, call into the application through these APIs (or equivalently through the same operations on a facade type).

**`ports/auth/`:** Session/credentials types and the **`AuthService`** driven port (sign-in / refresh). Infrastructure provides the Supabase-backed implementation.

#### Facades (`facade/`)

**`BackofficeFacade<D, A>`** and **`MobileFacade<D, A>`** hold `Arc` of each relevant use case and **`impl BackofficeApi` / `impl MobileApi`** by delegating to `execute`. Generic bounds use only domain repository traits + `AuthService` (e.g. backoffice: read + write catalog; mobile: read catalog + patient session write where needed).

**Composition roots** wire concrete `D` / `A` (e.g. `SupabaseRestRepository`, `SupabaseAuth`), build one `Arc<BackofficeFacade<…>>` or `Arc<MobileFacade<…>>`, and pass it to the UI layer:

- [`backoffice-dioxus/src/app_context.rs`](backoffice-dioxus/src/app_context.rs): `AppContext` stores `BackofficeFacadeHandle`; hooks use `backoffice_facade()` and **`BackofficeApi`** in scope for trait methods.
- [`mobile-bridge-frb/src/api.rs`](mobile-bridge-frb/src/api.rs): a lazy `MobileFacade` drives the exported async FFI functions.

```mermaid
flowchart LR
  subgraph inbound [Inbound adapters]
    Dioxus[backoffice_dioxus]
    FRB[mobile_bridge_frb]
  end
  subgraph app [application]
    ApiTraits[BackofficeApi MobileApi]
    Facades[BackofficeFacade MobileFacade]
    UC[Use cases]
    ApiTraits --> Facades
    Facades --> UC
  end
  subgraph driven [Driven ports]
    Repo[domain repositories]
    AuthP[AuthService]
  end
  subgraph infra [infrastructure]
    Supa[SupabaseRestRepository etc]
  end
  Dioxus --> ApiTraits
  FRB --> ApiTraits
  UC --> Repo
  UC --> AuthP
  Supa --> Repo
```

### Infrastructure Layer (`infrastructure/`)

**Implements ports** with concrete adapters and external integrations.

**Responsibilities:**

- Map domain models to storage (DTO serialization)
- Implement authentication
- Query and persist data
- Integrate external services

**Key Components:**

#### Supabase Adapter (`supabase/`)

Domain repository traits are implemented by **`SupabaseRestRepository`** in `repositories/supabase_rest_repository.rs` (REST + RPC). `Api` and `NativeApi` are type aliases to that type; builders live in `api.rs` and `native_api.rs`. DTOs remain in `infrastructure/src/api/dtos.rs`.

| Module          | Purpose                                |
| --------------- | -------------------------------------- |
| `client.rs`     | HTTP client for Supabase REST API      |
| `config.rs`     | Configuration (URL, anon key)          |
| `repositories/` | `SupabaseRestRepository` (trait impls) |
| `api.rs`        | `Api` alias + `ApiBuilder`             |
| `native_api.rs` | `NativeApi` alias + `NativeApiBuilder` |

**RPC example:** migration `014_rpc_session_feedback_by_patient_program.sql` exposes `list_session_exercise_feedback_for_patient_program` so the app loads program-wide feedback in one call instead of two round-trips.

#### Database Schema (`../supabase/migrations/`)

PostgreSQL schema defined as versioned migrations:

- `001_initial_schema.sql`: Core tables (users, profiles, programs, workouts, exercises)
- `002_*.sql`: Incremental updates

**Core Tables:**

```
auth.users ───┬─→ public.profiles (user_id, role)
              │
              ├─→ public.specialist_patients (specialist_id, patient_id)
              │
              └─→ public.patient_programs (program_id, patient_id)
                  └─→ public.workout_sessions (feedback, completion)
                      └─→ public.session_exercise_feedback (effort, pain)

public.programs ───→ public.program_schedules ───→ public.workouts
public.workouts ───→ public.workout_exercises ───→ public.exercises
```

### Bridge Layer (`mobile-bridge-frb/`)

**FFI (Foreign Function Interface)** exposing Rust to Flutter.

**Generated by `flutter_rust_bridge`**, this crate bridges Rust and Dart via:

- C FFI bindings
- Serialization/deserialization (serde)
- Async channel support

**Entrypoint: `src/api.rs`**

Public async functions callable from Flutter:

```rust
// Authentication
pub async fn login(request: LoginRequest, config: BridgeConfig) -> Result<LoginResponse, String>
pub async fn refresh_session(refresh_token: String, config: BridgeConfig) -> Result<LoginResponse, String>

// Data Fetching
pub async fn get_patient_programs(token: String, config: BridgeConfig) -> Result<Vec<PatientProgramSummary>, String>

// Data Submission
pub async fn submit_day_feedback(token: String, request: SubmitDayFeedbackRequest, config: BridgeConfig) -> Result<(), String>
pub async fn update_day_completion(token: String, request: UpdateDayCompletionRequest, config: BridgeConfig) -> Result<(), String>
```

**Data Structures:**

- `BridgeConfig`: Supabase URL + anon_key (configured at runtime)
- `LoginRequest`: email + password
- `LoginResponse`: access_token, user_id, user_profile_type
- `PatientProgramSummary`: Program details with days and exercises
- `SubmitDayFeedbackRequest`: Workout feedback (effort, pain, comments)

**Key Pattern:**

1. Receive Flutter request
2. Instantiate `NativeApi` with config
3. Create use case (e.g., `MobileLoginUseCase<NativeApi>`, `MobileRefreshSessionUseCase<NativeApi, SupabaseAuth>` for token refresh)
4. Execute use case and map domain result to Dart-friendly DTO
5. Return `Result<T, String>` (Dart cannot represent Rust error enums)

## UI Layers

### Flutter Mobile (`app-flutter/`)

**Mobile frontoffice** for patients and specialists.

**Architecture:**

- **Pages** (container widgets): Handle navigation and page state
- **Widgets** (presentation): Dumb UI components, no logic
- **Pages** call Rust bridge functions directly via generated bindings
- **FutureBuilder/StreamBuilder**: Handle async bridge calls

**Key Entry Point:** `lib/main.rs`

```dart
void main() {
  runApp(EixeApp(bridgeConfig: BridgeRuntimeConfig.fromEnvironment()));
}
```

**Design:**

- All business logic in Rust
- Flutter is purely UI/UX
- No Dart-side repositories or use cases
- Error messages sourced from Rust via bridge

**Localization:** Supports English (en), Spanish (es), Galician (gl) via `l10n/app_*.arb`

### Dioxus Backoffice (`backoffice-dioxus/`)

**Web backoffice** for specialist management.

**Architecture:**

- **Views** (container components): Pages with routing
- **Hooks** (custom): Wrapper around use cases
- **Components** (presentation): Reusable UI elements
- **app_context.rs**: Dependency container; wires `Arc<Api>` into each use case (the API type implements the domain repository traits)

**Key Entry Point:** `src/main.rs`

```rust
fn main() {
    backoffice_dioxus::launch();
}
```

Calls `backoffice_dioxus::lib.rs`:

```rust
pub fn launch() {
    init_logging();
    dioxus::launch(App);  // Start router and render App component
}
```

**Routing:**

```rust
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")] LoginView {},
    #[route("/specialist")] SpecialistPatients {},
    #[route("/specialist/programs")] SpecialistPrograms {},
    #[route("/specialist/exercises")] ExerciseLibrary {},
    #[route("/specialist/workouts")] WorkoutLibrary {},
    #[route("/specialist/workouts/:id")] WorkoutEditor { id: String },
    #[route("/specialist/patient/:id")] PatientProgress { id: String },
    #[route("/patient/program/:patient_program_id/day/:day_index")] PatientWorkoutSessionView { ... },
    #[route("/programs/:id/edit")] ProgramEditor { id: String },
}
```

**Use Case Example: Fetching Programs**

1. `WorkoutLibrary` view calls `use_workout_library()` hook
2. Hook calls `ListWorkoutLibrary::execute()`
3. Use case orchestrates `backend.get_workouts()`
4. `NativeApi` (Supabase adapter) fetches from DB
5. Result bubbles back to component via Dioxus signal

## Data Flow Examples

### Sequence 1: Mobile Patient Login

```mermaid
sequenceDiagram
    actor User
    participant Flutter as Flutter App
    participant Bridge as mobile-bridge-frb
    participant UseCase as MobileLoginUseCase
    participant Adapter as NativeApi
    participant Supabase as Supabase REST API
    participant DB as PostgreSQL

    User->>Flutter: Enter email/password
    Flutter->>Bridge: login(email, password, config)
    Bridge->>UseCase: execute(credentials)
    UseCase->>Adapter: sign_in(credentials)
    Adapter->>Supabase: POST /auth/v1/token
    Supabase->>DB: Verify user
    DB-->>Supabase: User found + profile role
    Supabase-->>Adapter: access_token, refresh_token
    Adapter-->>UseCase: Session { user_id, access_token, ... }
    UseCase-->>Bridge: LoginResponse
    Bridge-->>Flutter: LoginResponse
    Flutter->>User: Show dashboard
```

### Sequence 2: Specialist Fetches Programs (Dioxus Web)

```mermaid
sequenceDiagram
    actor Specialist
    participant Dioxus as Dioxus View
    participant Hook as use_list_programs Hook
    participant UseCase as ListProgramsUseCase
    participant Adapter as NativeApi
    participant Supabase as Supabase REST API
    participant DB as PostgreSQL

    Specialist->>Dioxus: Navigate to /specialist/programs
    Dioxus->>Hook: use_resource(fetch_programs)
    Hook->>UseCase: execute(token)
    UseCase->>Adapter: get_programs_by_specialist(specialist_id, token)
    Adapter->>Supabase: GET /rest/v1/programs?specialist_id=eq.{id}
    Supabase->>DB: SELECT * FROM programs WHERE specialist_id = ?
    DB-->>Supabase: [Program...]
    Supabase-->>Adapter: [ProgramDto...]
    Adapter-->>UseCase: [Program...] (domain entities)
    UseCase-->>Hook: [Program...]
    Hook-->>Dioxus: Signal<Option<Result<Vec<Program>>>>
    Dioxus->>Dioxus: Render program list
    Dioxus->>Specialist: Display programs
```

## Dependency Injection

### Web (Dioxus)

```rust
// backoffice-dioxus/src/app_context.rs (simplified)
pub fn build_app_context() -> Result<AppContext> {
    let backend = Arc::new(ApiBuilder::new().build());
    // Arc::new(SomeUseCase::<Api>::new(backend.clone(), …))
    // …
}
```

Injected into app as a context provider; hooks extract it via `use_context()`.

### Mobile (Flutter)

```rust
// mobile-bridge-frb/src/api.rs
fn backend(config: BridgeConfig) -> Arc<NativeApi> {
    let config = SupabaseConfig {
        url: config.url,
        anon_key: config.anon_key,
    };
    Arc::new(NativeApi::new(SupabaseClient::new(config)))
}
```

Config passed at call time from Flutter; backend instantiated per request (simple and safe for FFI).

## Key Design Decisions

### 1. DDD Boundaries by Crate

```
domain/              → Business logic (framework-agnostic)
application/         → Use cases + ports (orchestration)
infrastructure/      → Adapters (Supabase REST)
mobile-bridge-frb/   → FFI bridge (Dart calling Rust)
app-flutter/         → Mobile UI (pure presentation)
backoffice-dioxus/   → Web UI (pure presentation)
```

**Benefit:** Clear separation; infrastructure cannot leak into domain.

### 2. Ports for Abstraction

Instead of direct database calls, use cases depend on **domain repository traits** and `AuthService` where needed:

- `SpecialistCatalogReadRepository` / `SpecialistCatalogWriteRepository` → catalog and programs
- `PatientSessionWriteRepository` → patient workout sessions and feedback
- `AuthService` → Supabase auth

**Benefit:** Easy to mock or swap adapters without touching use cases; each use case declares only the outbound ports it uses.

### 3. Send-Bounded Variants for Mobile

Mobile use cases (`MobileLoginUseCase`, `MobileRefreshSessionUseCase`, `GetPatientProgramsUseCase`, etc.) use the same domain repository traits; repository traits already require `Send + Sync` where needed because:

- Flutter's Rust bridge runs async calls on a thread pool
- Domain futures must be `Send`
- Dioxus (web) doesn't need this; it runs single-threaded

**Benefit:** No unnecessary `Send` overhead on web; optimal for each platform.

### 4. DTOs for Serialization

Domain entities (e.g., `Program`, `Exercise`) differ from Supabase rows (`ProgramDto`, `ExerciseDto`):

- Domains are type-safe, immutable, validate invariants
- DTOs are flattened, nullable for JSON serialization
- Adapter maps DTO → Domain on retrieval, Domain → DTO on persistence

**Benefit:** Decouples domain model from storage schema.

### 5. Error Handling

Domain errors are `enum` with context:

```rust
pub enum DomainError {
    NotFound(String),
    Unauthorized,
    InvalidInput(String),
    // ...
}
```

When crossing the FFI boundary, errors convert to `String` (Dart cannot represent Rust enums):

```rust
pub async fn login(...) -> Result<LoginResponse, String> {
    use_case.execute(...).await.map_err(|e| e.to_string())
}
```

**Benefit:** Rich error info in Rust; simple JSON string in mobile UI.

## Testing Strategy

### Domain Testing

Pure Rust tests; no external dependencies required.

### Application Testing

Mock the relevant **repository traits** or `AuthService`; test orchestration logic.

### Integration Testing

Run against real Supabase instance; test end-to-end flows.

### UI Testing (Flutter)

Widget tests mock Rust bridge; integration tests use real bridge with test server.

## Deployment

### Mobile

1. Build Rust bridge: `cargo build --release` (generates `.so` for Android, `.a` for iOS)
2. Flutter build: `flutter build apk` / `flutter build ios`
3. Distribute via Play Store or TestFlight

### Web (Dioxus)

1. Build: `dx build --release`
2. Serve static files
3. Or: embed in a Tauri/Electron app

### Backend (Supabase)

1. Migrations applied via `supabase db push`
2. Hosted on Supabase infrastructure
3. RLS policies enforce row-level security

## Crate Dependencies

```
backoffice-dioxus
├── application
│   ├── domain
│   └── infrastructure
│       ├── domain
│       └── application (for trait impls)
│
mobile-bridge-frb
├── application
│   ├── domain
│   └── infrastructure
│       ├── domain
│       └── application
│
app-flutter
└── mobile-bridge-frb (generated bindings)
```

**Invariant:** Domain has zero external dependencies (except std).

## Extending the System

### Adding a New Use Case

1. **Define** in `application/src/use_cases/your_use_case.rs`
2. **Depend on** the relevant **domain repository trait(s)** from `application::ports` (e.g. `SpecialistCatalogReadRepository`, `SpecialistCatalogWriteRepository`, `PatientSessionWriteRepository`) plus `AuthService` if applicable
3. **Implement ports** in `infrastructure/src/supabase/native_api.rs` if needed
4. **Call from UI:**
   - Dioxus: Create hook in `backoffice-dioxus/src/hooks/`, call use case
   - Flutter: Expose in `mobile-bridge-frb/src/api.rs`, generate bindings, call from Dart

### Adding a New Port

1. **Define trait** in `application/src/ports/your_port.rs`
2. **Implement** in `infrastructure/src/supabase/native_api.rs`
3. **Use cases** depend on trait, not concrete type

### Schema Changes

1. **Create migration** in `supabase/migrations/NNN_*.sql`
2. **Update DTOs** in `infrastructure/src/api/dtos.rs` if needed
3. **Update domain entities** in `domain/src/entities.rs` if business logic changes
4. **Update use cases** to work with new domain entities

---

**Last Updated:** 2026-03-18

For more information, see the project README and individual module documentation.
