# Plan: Iconos y Navegación - Backoffice Dioxus

## Resumen
Mejorar la UI del backoffice-dioxus añadiendo iconos ionicons y usando el componente sidebar existente de dioxus-primitives para desktop y tablet.

---

## Componente Sidebar EXISTENTE

El proyecto ya tiene un componente `Sidebar` completo en `components/sidebar/` que incluye:
- `SidebarProvider` - Provider para estado
- `Sidebar` - Componente principal
- `SidebarHeader`, `SidebarContent`, `SidebarFooter`
- `SidebarMenu`, `SidebarMenuItem`, `SidebarMenuButton`
- `SidebarTrigger`, `SidebarRail`
- Responsive: desktop expandido, tablet colapsado como offcanvas
- Keyboard shortcut: `Ctrl+b` o `Cmd+b` para toggle

---

## Fase 1: Integrar Sidebar en AppLayout

### Modificar `lib.rs`
```rust
use crate::components::sidebar::{
    SidebarProvider, Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuButton, SidebarTrigger,
    SidebarCollapsible, SidebarVariant,
};
use dioxus_free_icons::icons::io_icons::{IoPeople, IoFolderOpen, IoBarbell, IoFitness, IoLogOut};

// En AppLayout:
SidebarProvider {
    default_open: true,
    Sidebar {
        collapsible: SidebarCollapsible::Icon,
        variant: SidebarVariant::Sidebar,
        SidebarHeader { "Eixe" }
        SidebarContent {
            SidebarMenu {
                SidebarMenuItem {
                    SidebarMenuButton {
                        Icon { icon: IoPeople, width: 20, height: 20 }
                        span { "Pacientes" }
                    }
                }
                // ... otras opciones
            }
        }
        SidebarFooter {
            SidebarMenuButton {
                Icon { icon: IoLogOut, width: 20, height: 20 }
                span { "Cerrar sesión" }
            }
        }
    }
    SidebarTrigger {}
    // Contenido principal
}
```

---

## Fase 2: Iconos en Botones y Acciones

### 2.1 Imports necesarios en cada vista
```rust
use dioxus_free_icons::icons::io_icons::{
    IoAdd, IoPencil, IoTrash, IoSave, IoClose,
    IoSearch, IoCheckmark, IoChevronDown, IoChevronUp,
    IoArrowBack, IoRefresh, IoEye, IoMenu
};
use dioxus_free_icons::Icon;
```

### 2.2 Botones por vista

**exercise_library.rs:**
| Acción | Icono | Texto | Tooltip |
|--------|-------|-------|---------|
| Crear ejercicio | IoAdd | "Crear" | Sí |
| Editar | IoPencil | - | Sí |
| Eliminar | IoTrash | - | Sí |
| Guardar | IoSave | "Guardar" | No |
| Cancelar | IoClose | "Cancelar" | No |

**workout_library.rs:**
| Acción | Icono | Texto | Tooltip |
|--------|-------|-------|---------|
| Crear | IoAdd | "Crear" | Sí |
| Ver ejercicios | IoEye | - | Sí |
| Editar | IoPencil | - | Sí |
| Eliminar | IoTrash | - | Sí |
| Guardar | IoSave | "Guardar" | No |
| Cancelar | IoClose | "Cancelar" | No |

**specialist_dashboard.rs:**
| Acción | Icono | Texto |
|--------|-------|-------|
| Añadir paciente | IoAdd | "Añadir" |

**specialist_programs.rs:**
| Acción | Icono | Texto |
|--------|-------|-------|
| Crear programa | IoAdd | "Crear" |
| Asignar | IoCheckmark | "Asignar" |
| Limpiar | IoClose | "Limpiar" |

### 2.3 Reemplazar textos de navegación
- "▲/▼" → `IoChevronDown` / `IoChevronUp`
- Título del navbar → `IoMenu`

---

## Fase 3: Responsive

El sidebar existente ya maneja:
- Desktop (≥768px): Sidebar visible
- Mobile (<768px): Offcanvas/sheet

Para tablet (768-1024px): Verificar que el comportamiento sea adecuado.

---

## Orden de Implementación

1. **lib.rs** - Integrar sidebar provider y componentes
2. **specialist_dashboard.rs** - Añadir iconos
3. **specialist_programs.rs** - Añadir iconos
4. **exercise_library.rs** - Añadir iconos
5. **workout_library.rs** - Añadir iconos
6. **Verificar** - Compilación y tests

---

## Archivos a Modificar

### Existentes (5)
- `lib.rs`
- `specialist_dashboard.rs`
- `specialist_programs.rs`
- `exercise_library.rs`
- `workout_library.rs`

### No nuevos (sidebar ya existe)

---

## Notas
- El componente Sidebar ya está exportado en `components/mod.rs`
- dioxus-free-icons con feature `ionicons` ya está en Cargo.toml
- Focus rings implementados previamente se mantienen
- El sidebar soporta tooltips automáticamente para items colapsados