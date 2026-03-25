# Plan: Correcciones de Layout del Sidebar

## Errores a corregir:

1. **Icono del sidebar y "Eixe" se solapan** - En pantalla grande (desktop)
2. **Contenido fijo en el medio** - No se muestra completo en tablets < 1240px
3. **Contenido muy estrecho** - En pantallas grandes

---

## Análisis de las causas:

### Error 1: Solapamiento Trigger + Header
El `SidebarTrigger` tiene `class="fixed top-4 left-4 z-50"` y el SidebarHeader también está en posición izquierda. En desktop ambos compiten por el mismo espacio.

### Error 2: Contenido con margin-left fijo
```rust
div { class: "ml-16 md:ml-64 p-4 md:p-6 min-h-screen"
```
- `ml-16` = 4rem (64px) - siempre activo
- `md:ml-64` = 16rem (256px) - solo en breakpoints >= 768px

El problema: El sidebar usa clases CSS personalizadas (`--sidebar-width: 16rem`, `--sidebar-width-icon: 3rem`) pero el contenido usa valores fijos de Tailwind que no se sincronizan correctamente con el estado del sidebar.

### Error 3: Contenido estrecho
El contenedor de las vistas usa:
```rust
div { class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl"
```
Esto limita el ancho máximo a `2xl` (~1536px), pero con el sidebar 占用了 espacio, el contenido efectivo es menor.

---

## Plan de correcciones:

### 1. Separar el trigger del contenido (Evitar solapamiento)

**Opción A**: Mover el trigger al interior del sidebar (cuando está expandido) y fuera cuando está colapsado
**Opción B**: Agregar margen al header para evitar overlap
**Opción C**: Ocultar el trigger cuando el sidebar está expandido y usar un botón interno

**Recomendación**: Opción B - agregar `pl-16` o `pl-20` al SidebarTrigger cuando está visible, y ajustar el SidebarHeader para que tenga su propio margen.

### 2. Sincronizar el contenido con el estado del sidebar

El SidebarProvider tiene estado que podemos usar. Necesitamos:
- Cuando sidebar expandido: `ml-64` (256px)
- Cuando sidebar colapsado (icon mode): `ml-16` (64px)

El problema es que no tenemos acceso directo al estado del sidebar desde fuera del componente.

**Solución**: Eliminar el margin-left manual y usar el layout natural del sidebar. El CSS del sidebar ya maneja el `.sidebar-gap` que reserva el espacio.

**Cambio propuesto**:
```rust
// Eliminar ml-16 md:ml-64
div { class: "flex-1 p-4 md:p-6 min-h-screen overflow-auto"
```

### 3. Mejorar el ancho del contenido en desktop

- Quitar límites de ancho máximo (`lg:min-w-2xl`)
- Usar `w-full` y `max-w-full`
- Permitir que el contenido use el espacio disponible

---

## Orden de implementación:

1. **Lib.rs** - Ajustar el layout del AppLayout:
   - Mover el SidebarTrigger a una posición que no se solape
   - Quitar los márgenes manuales del contenido
   - Dejar que el CSS del sidebar maneje el spacing

2. **Vistas** - Ajustar el contenedor de contenido:
   - Cambiar `min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl` por `w-full`
   - Asegurar que el contenido sea fluido

3. **Verificar** - Compilación y tests

---

## Notas adicionales:

El CSS del sidebar ya tiene:
- `.sidebar-gap` que reserva el espacio correcto
- Queries de media para desktop (≥768px)
- Soporte para collapsible="icon" que cambia el ancho

El layout debería funcionar automáticamente si no agregamos márgenes manuales que contradigan el CSS.