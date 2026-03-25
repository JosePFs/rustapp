# Plan: Mejoras en UI del Backoffice Dioxus

## Objetivo
Identificar y proponer mejoras en la interfaz de usuario del crate `backoffice-dioxus`.

## Target
- **Plataforma**: Web (navegador de escritorio)
- **Dispositivos**: Pantalla normal (desktop 1024px+) y Tablet (768px-1023px)
- **Interacción**: Mouse y teclado (no touch)
- **Soporte**: Navegadores modernos (Chrome, Firefox, Safari, Edge)

## Análisis de la UI Actual

### Vistas existentes
- `LoginView` - Pantalla de inicio de sesión
- `SpecialistPatients` - Dashboard de pacientes
- `SpecialistPrograms` - Gestión de programas
- `ExerciseLibrary` - Biblioteca de ejercicios
- `WorkoutLibrary` - Biblioteca de entrenamientos
- `WorkoutEditor` - Editor de entrenamientos
- `PatientProgress` - Progreso del paciente
- `ProgramEditor` - Editor de programas

### Componentes disponibles
- Button (variantes: primary, secondary, destructive, outline, ghost)
- Input, Textarea, Label
- Card, Progress, Skeleton
- Sheet, Slider, Tooltip
- Login, Backview

### Stack de estilos
- TailwindCSS v4 con tema personalizado
- Colores definidos: primary, surface, text-muted, border, error, success
- Radio: sm (4px), md (8px), lg (12px)

### Breakpoints actuales en uso
- min-w-[280px] - Móvil
- sm:min-w-[320px] - Tablets pequeñas
- md:min-w-[400px] - Tablets grandes
- lg:min-w-2xl - Desktop

## Target
- **Plataforma**: Web (navegador)
- **Dispositivos**: Desktop (≥1024px) y Tablet (768px-1023px)
- **Interacción**: Mouse y teclado
- **Navegadores**: Chrome, Firefox, Safari, Edge (últimas versiones)

## Áreas de Mejora Identificadas

### 1. Consistencia Visual
- Estandarizar uso de espaciados
- Unificar tratamiento de bordes y sombras
- Revisar jerarquía tipográfica

### 2. Accesibilidad
- Añadir atributos ARIA donde faltan
- Mejorar contraste en elementos
- Añadir estados de focus visibles

### 3. UX/Interacción
- Mejorar feedback de carga
- Añadir transiciones suaves
- Optimizar navegación móvil (no es target, pero mantener)

### 4. Componentes
- Expandir variantes de Button
- Melhorar componentes de formulario
- Añadir más estados (empty, error personalizado)

## Plan de Ejecución

### Fase 1: Auditoría UI (Agente reviewer)
- Revisar cada vista contra guidelines de diseño
- Documentar inconsistencias
- Proponer lista priorizada de mejoras

### Fase 2: Implementación (Agente implementer)
- Aplicar mejoras identificadas
- Crear nuevos componentes si es necesario
- Actualizar estilos globales

### Fase 3: Verificación (Agente tester)
- Verificar que cambios compilan
- Revisar que UI funciona correctamente
- Validar accesibilidad

## Agentes a utilizar

1. **reviewer** - Análisis de UI actual
2. **implementer** - Aplicar mejoras
3. **tester** - Verificar cambios
4. **docs** - Documentar cambios si es necesario