# Auditoría de Accesibilidad - Backoffice Dioxus

## Resumen
Análisis de accesibilidad WCAG AA para el crate `backoffice-dioxus`. Target: Desktop (≥1024px).

---

## Hallazgos de Accesibilidad

### ✅ Existente - BUENO
1. **Focus states en Button** (style.css línea 10-12): `focus-visible` con box-shadow
2. **Transiciones suaves**: Buttons tienen `transition: 0.2s ease`
3. **Contraste acceptable**: Colores primarios tienen buen contraste
4. **Labels en checkboxes**: Usan `<label>` envolviendo `<input>`

### ❌ PROBLEMAS ENCONTRADOS

#### 1. Focus States Faltantes en Vistas
**Archivos afectados**: specialist_dashboard.rs, exercise_library.rs, workout_library.rs, specialist_programs.rs

**Problema**: Elementos `<a>` (Link de dioxus-router) y `<button>`inline no tienen indicadores de foco visibles.

```rust
// PROBLEMA: No hay estilo de focus
Link { to: Route::PatientProgress { id: link.patient_id.clone() }, ... }
button { class: "min-h-11 px-2 ...", ... }
```

**Solución**: Añadir `focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary` en Tailwind.

#### 2. Formularios sin Labels Visibles
**Archivos**: specialist_programs.rs (líneas 100-111, 158-163, 239-243)

**Problema**: Inputs de "Nombre del programa" y "Descripción" solo tienen `placeholder`, no hay `<label>`.

```rust
// PROBLEMA
input { placeholder: t!("program_name"), ... }
```

**Solución**: Añadir `<label>` antes de cada input o usar aria-label.

#### 3. Tooltip sin ARIA
**Archivos**: specialist_dashboard.rs, specialist_programs.rs

**Problema**: Tooltips carecen de atributos ARIA para lectores de pantalla.

```rust
// ACTUAL
Tooltip { ... }
```

**Solución**: Añadir `role="tooltip"` y `aria-describedby`.

#### 4. Navegación sin Skip Link
**Problema**: No hay forma de saltar la navegación para usuarios de teclado.

**Solución**: Añadir "Skip to main content" link al inicio del layout.

#### 5. Contraste de text-text-muted
**Problema**: `text-text-muted` (oklch 0.5) puede no cumplir 4.5:1 en algunos contextos.

**Solución**: Usar solo para iconos decorative, no para texto importante.

---

## Lista de Correcciones Priorizadas

### Prioridad ALTA
1. Añadir focus-visible rings a todos los Links y Buttons inline
2. Añadir labels a todos los inputs de formularios
3. Añadir skip link al layout

### Prioridad MEDIA
4. Mejorar tooltip con ARIA
5. Revisar contraste de text-muted
6. Añadir aria-describedby a inputs con errores

---

## Archivos a Modificar

1. `tailwind.css` - Añadir utility para focus-visible
2. `views/specialist_dashboard.rs` - Focus rings + labels
3. `views/specialist_programs.rs` - Labels formularios
4. `views/exercise_library.rs` - Focus rings
5. `views/workout_library.rs` - Focus rings
6. `components/tooltip/component.rs` - ARIA
7. `lib.rs` - Añadir skip link

---

## Recomendaciones Adicionales

- Crear componente `Label` existente pero no se usa consistentemente
- Componente `Input` ya soporta eventos de focus/blur
- Considerar añadir `role="alert"` para mensajes de error