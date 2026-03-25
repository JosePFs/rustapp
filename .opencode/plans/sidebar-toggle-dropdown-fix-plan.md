# Plan: Correcciones - Sidebar Toggle y Dropdowns

## Problemas identificados:

### Problema 1: Botón de toggle del sidebar no visible
**Causa**: El `SidebarTrigger` fue eliminado en el cambio anterior. El sidebar de dioxus usa:
- Desktop (≥768px): Sidebar fijo visible
- Mobile (<768px): Se convierte en Sheet/OffcanvasEl trigger original era para **colapsar** el sidebar (hacerlo más estrecho), no para mostrarlo/ocultarlo. En móvil el sidebar ya se maneja automáticamente.**Solución propuesta**: Añadir un botón toggle visible en el SidebarHeader para colapsar/expandir el sidebar, usando el componente `use_sidebar().toggle()` o añadir el `SidebarRail` que ya existe.

### Problema 2: Dropdowns en las vistas (especialmente specialist_programs)
**Causa**: En el cambio anterior solo se eliminaron los dropdowns de algunas vistas (`specialist_dashboard`, `exercise_library`, `workout_library`), pero otras vistas todavía tienen el código de dropdown: `specialist_programs`, `program_editor`, `workout_editor`, `patient_progress`.**Solución**: Eliminar el código de dropdown de todas las vistas que todavía lo tengan.

---

## Plan de implementación:

### 1. Añadir botón de toggle del sidebar en lib.rs

**Cambio en lib.rs AppLayout**:```rust
SidebarHeader { class: "p-4 border-b border-border flex items-center justify-between",
    div { class: "flex items-center gap-2",
        h1 { class: "text-xl font-semibold text-text", "Eixe" }
    }
    // Añadir botón para colapsar/expandier usando use_sidebar    button { 
        class: "p-2 hover:bg-gray-100 rounded-md",
        onclick: move |_| { /* toggle sidebar */ },
        Icon { width: 20, height: 20, icon: IoMenu }
    }
}
```El sidebar ya tiene el comportamiento de colapsar automáticamente. Solo necesitamos un trigger visible.

### 2. Eliminar dropdowns de las vistas

**Vistas a modificar**:- `specialist_programs.rs` - tiene dropdown completo- `program_editor.rs` - tiene dropdown
- `workout_editor.rs` - tiene dropdown
- `patient_progress.rs` - tiene dropdown

**Cambio en cada vista**: Eliminar el bloque:
```rust
{    let mut nav_open = use_signal(|| false);
    rsx! {
        nav { class: "relative mb-6",
            button { ... }            if nav_open() { ... }
        }
    }}
```

---

## Orden de implementación:

1. **lib.rs** - Añadir botón de toggle visible en SidebarHeader
2. **specialist_programs.rs** - Eliminar dropdown
3. **program_editor.rs** - Eliminar dropdown
4. **workout_editor.rs** - Eliminar dropdown
5. **patient_progress.rs** - Eliminar dropdown
6. **Verificar** - Compilación y tests

---

## Notas:
- El sidebar de dioxus ya maneja automáticamente mobile (<768px) como Sheet
- El toggle es principalmente para desktop (colapsar el sidebar a modo icon)
- Los dropdowns en las vistas son redundantes ahora que hay un sidebar global