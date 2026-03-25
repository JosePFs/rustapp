# Plan: Añadir Botón Toggle del Sidebar en Mobile

## Problema actual
El sidebar en dispositivos ≤767px solo se puede mostrar usando `Ctrl+b` (o `Cmd+b` en Mac). No hay ningún botón visible para togglear el sidebar en móvil/tablet.

## Análisis
El componente `SidebarTrigger` ya existe en `components/sidebar/component.rs`:
- Usa `use_sidebar()` internamente para hacer toggle
- Incluye un botón con icono SVG de hamburguesa
- El componente requiere estar dentro del `SidebarProvider` para funcionar

## Solución propuesta

### Opción A: Usar SidebarTrigger existente
Añadir el componente `SidebarTrigger` al layout, con CSS para mostrarlo solo en móvil.

**Cambio en lib.rs AppLayout:**
```rust
SidebarTrigger { 
    class: "fixed top-4 left-4 z-50 md:hidden"  // Solo visible en móvil
}
```

### Opción B: Crear trigger manual con use_sidebar
Si Option A no funciona (por el contexto), crear un trigger manual que llame a `ctx.toggle()`.

**Problema identificado**: El error anterior era porque `use_sidebar()` no puede llamarse dentro del propio componente Sidebar. Necesita estar en un hijo del Provider.

**Solución**: Poner el trigger como hijo directo del SidebarProvider, no dentro de Sidebar.

---

## Plan de implementación

### Paso 1: Añadir SidebarTrigger a lib.rs
- Importar `SidebarTrigger` 
- Añadir fuera del componente Sidebar, como hijo directo de SidebarProvider
- Añadir clase `md:hidden` para ocultar en desktop

### Paso 2: Verificar funcionamiento
- Compilar y probar
- En móvil (≤767px): el botón debe abrir el sidebar como Sheet
- En desktop (≥768px): el botón está oculto (el sidebar ya está visible)

---

## Notas adicionales
- El SidebarTrigger ya está incluido en el CSS del sidebar (`sidebar-trigger`)
- El breakpoint de 768px ya está definido en el CSS del sidebar como `MOBILE_BREAKPOINT`
- Usar la clase `md:hidden` de Tailwind para mostrar solo en móvil coincide con el CSS del sidebar