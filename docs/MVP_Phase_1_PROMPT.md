# MVP Phase 1 — Prompt

## Project Context

We are building Phase 1 of an MVP for a physiotherapy clinic application.

### Technology Stack

- Supabase (Auth + PostgreSQL)
- Frontend in Dioxus (Rust)
- No dedicated backend in this phase
- Architecture prepared for future migration to a Rust backend using Axum
- Designed with clean architecture principles and future Domain-Driven Design evolution in mind

This document defines exactly what must be generated in this phase.

---

# Phase 1 MVP Objective

The system must allow:

## Specialist

- Create patient
- Create basic program
- Assign program to patient
- View daily completion status

## Patient

- View active training program
- Mark daily session as completed
- Send feedback:
  - Effort (1–10)
  - Pain (0–10)
  - Free-text comment

---

# Part 1 — Database Model (Supabase / PostgreSQL)

Generate a complete SQL script including:

## Required Tables

- profiles
- specialist_patients
- programs
- exercises
- patient_programs
- workout_sessions

## Requirements

- Proper foreign keys
- Necessary constraints
- Unique constraints where required
- Basic indexing where appropriate
- Enable Row Level Security (RLS) on all tables

## Mandatory RLS Policies

- A specialist can only see their own patients
- A patient can only see their own data
- A specialist can only create and modify their own programs
- A patient can only modify their own workout sessions
- No user can access data belonging to other users

⚠️ Only generate the full SQL first.  
Do not generate frontend code until the database is complete.

---

# Part 2 — Dioxus Project Structure

Generate the base structure:

    /src
      /pages
        login.rs
        specialist_dashboard.rs
        patient_dashboard.rs
        program_editor.rs
      /components
      /services
        supabase_client.rs
      main.rs

## Requirements

- Clear separation between:
  - UI layer (pages/components)
  - Data access layer (services)
- No business logic embedded directly inside UI rendering code
- Structured in a way that allows future backend extraction (Axum)

---

# Part 3 — UI Functionalities

Generate incrementally:

1. Supabase authentication (login)
2. Specialist dashboard:
   - List patients
   - Create patient
   - Create program
   - Assign program
   - View basic compliance status
3. Patient dashboard:
   - View active program
   - View exercises
   - Mark session completed
   - Submit feedback

Do not generate everything at once.  
Proceed step by step.

---

# Architectural Decisions

This section defines the intentional architectural constraints for Phase 1.

## 1. No Dedicated Backend (Yet)

All business logic is minimal and handled via:

- Supabase database
- RLS policies
- Direct frontend calls to Supabase

This reduces cost and complexity during MVP validation.

However, the code must be structured so that:

- Data access can later be extracted into a Rust backend (Axum).
- Frontend does not contain critical domain rules.

---

## 2. Database as Temporary Source of Truth

In Phase 1:

- PostgreSQL + RLS acts as the primary security boundary.
- Authorization rules are enforced at the database level.
- Frontend assumes backend trust boundaries are database-enforced.

Future evolution:

- Introduce backend domain layer.
- Move business rules from DB into domain services.
- Keep DB as persistence layer only.

---

## 3. Layer Separation (Even Without Backend)

Even without a backend:

The project must be structured with:

- Presentation layer (pages/components)
- Data access layer (services)
- Clear boundaries between them

This ensures future extraction of a domain layer without full rewrite.

---

## 4. Minimal Domain Complexity

This MVP intentionally avoids:

- Complex workout models
- Advanced progression systems
- Metrics engine
- Event-driven workflows

Complexity will be introduced only after validating real-world usage.

---

## 5. Scalability Strategy

The system is designed to scale in three steps:

1. MVP with Supabase only
2. Add Rust backend (Axum) for domain logic
3. Introduce Domain Events and modular bounded contexts if needed

This avoids premature over-engineering while keeping architectural optionality.

---

# Quality Criteria

Generated code must:

- Be clean and readable
- Follow idiomatic Rust practices
- Avoid tight coupling
- Avoid embedding complex logic in UI
- Be ready for backend extraction
- Avoid premature abstractions

---

# Instructions order

Start by generating only the complete SQL schema with RLS and properly defined policies.

Do not proceed to frontend until the database model is fully defined.

Generate step by step.
