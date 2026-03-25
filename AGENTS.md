# AGENTS.md - Agentic Coding Guidelines

This file provides context for agents operating in this repository.

## Project Overview

- **Name**: Eixe - Physiotherapy Clinic MVP
- **Stack**: Rust (DDD + Hexagonal), Dioxus (backoffice), Flutter (frontoffice)
- **Architecture**: Domain-Driven Design + Hexagonal Architecture
- **Database**: Supabase (PostgreSQL + RLS)

## Directory Structure

```
domain/          # Business entities, value objects, aggregates, domain events
application/     # Use cases and application services (ports)
infrastructure/  # Adapters for Supabase, Flutter, Dioxus
backoffice-dioxus/  # Web backoffice (Dioxus 0.7)
app-flutter/        # Mobile app (Flutter)
mobile-bridge-frb/ # Rust FFI bridge for Flutter
supabase/           # SQL migrations
xtask/              # Development CLI tasks
```

## Build/Lint/Test Commands

```bash
# Check all crates (clippy + cargo check)
cargo xtask check-all

# Run Dioxus backoffice
cargo xtask dioxus-run

# Run Flutter app
cargo xtask flutter-run

# Run Flutter on Linux
cargo xtask flutter-linux

# Run unit tests (all packages, no infrastructure integration tests)
cargo xtask test-unit-all

# Run all tests (requires local Supabase)
cargo xtask test-all

# Run all tests in Docker
cargo xtask test-all-docker

# Run single test (unit tests)
cargo test -p <package> --lib -- <test-name>

# Run single integration test
cargo test -p <package> --test <test-file> -- <test-name>
```

## Code Style Guidelines

### General Principles
- **Language**: English for everything (names, comments, error messages)
- **DDD**: Use ubiquitous language consistently
- **Clean Code + SOLID**: Keep functions small, single responsibility
- **Prefer composition over inheritance**

### Rust Conventions
- **Naming**: `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_CASE` for consts
- **No `get_` prefix**: Use `fn name()` not `fn get_name()`
- **Iterators**: `iter()` / `iter_mut()` / `into_iter()`
- **Conversions**: `as_` (cheap ref), `to_` (expensive), `into_` (ownership)
- **Domain entities**: Prefer immutable when possible
- **Invariants**: Enforced in constructors

### Architecture Rules
1. **Domain** must not depend on infrastructure or UI
2. **Application** orchestrates use cases, no business rules
3. **Infrastructure** implements ports defined in domain/application
4. **UI** (Dioxus) only calls application services
5. **Never**: access DB from domain, access infra from UI, place business rules in UI

### Anti-Patterns to Avoid
- Placing business rules in UI
- Using database models as domain entities
- Creating god services
- Bypassing domain aggregates

### Testing
- Code must be easy to test
- Prefer dependency injection through ports
- Use `cargo xtask test-unit-all` for quick unit test runs

### Imports & Formatting
- Use `rustfmt` for formatting (default Rust style)
- Group imports: `std` → `external` → `project`
- Prefer explicit paths over wildcards
- Use newtypes: `struct Email(String)` for domain semantics

### Error Handling
- Use `?` for propagation (not `try!()` macro)
- Use `expect()` over `unwrap()` when value is guaranteed
- Use assertions for invariants at function entry

### Available Skills
Use these when tasks match their keywords:
- `rust-best-practices` - Idiomatic Rust code patterns
- `m09-domain` - Domain modeling, DDD patterns
- `m06-error-handling` - Result/Option patterns
- `m04-zero-cost` - Generics and traits
- `flutter-bridge-patterns` - Flutter-Rust integration (FRB)
- `domain-cli` - CLI tools with clap

## Cursor Rules Integration

Before editing code, check `.cursor/rules/` for project-specific rules:
- `coding.mdc` - Language and workflow rules
- `architecture.mdc` - Architecture constraints
- `rust-ddd.mdc` - Rust DDD conventions
- `anti-patterns.mdc` - Anti-patterns to avoid
- `agents.mdc` - Agent delegation priorities

## Environment Configuration

Create `.env` in project root:
```
SUPABASE_URL=https://YOUR_PROJECT.supabase.co
SUPABASE_ANON_KEY=your_anon_key
```

For testing, use `.env.test.local` with template from `.env.test.example`.

## Notes

- Domain layer contains: entities, value objects, aggregates, domain events, repository traits
- Infrastructure provides implementations for ports
- Tests requiring Supabase need local Supabase running (`cargo xtask smoke-test` first)
- Flutter-Rust bridge needs regeneration after API changes: `cargo xtask frb-generate`