# Plan: Mejoras en UI del Backoffice Dioxus

## Objetivo
Identificar y proponer mejoras en la interfaz de usuario del crate `backoffice-dioxus` con foco en **accesibilidad**.

## Target
- **Plataforma**: Web (navegador)
- **Dispositivos**: Desktop (≥1024px) - Prioridad principal
- **Dispositivos**: Tablet (768px-1023px) - Soporte secundario
- **Interacción**: Mouse y teclado
- **Navegadores**: Chrome, Firefox, Safari, Edge (últimas versiones)

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

### Breakpoints en uso
- min-w-[280px], sm:min-w-[320px], md:min-w-[400px], lg:min-w-2xl

## Prioridades del usuario
1. **Prioridad**: Accesibilidad
2. **Dispositivo principal**: Desktop
3. **Componentes**: Todos

## Áreas de Mejora (Foco: Accesibilidad)

### 1. Accesibilidad (PRIORITARIO)
- **Focus states**: Añadir indicadores de foco visibles en todos los elementos interactivos
- **ARIA labels**: Añadir atributos aria-label, aria-describedby donde falten
- **Contrast**: Revisar contraste de texto contra fondos (WCAG AA mínimo 4.5:1)
- **Keyboard navigation**: Asegurar que todos los elementos sean accesibles por teclado
- **Skip links**: Añadir enlace para saltar navegación
- **Form labels**: Garantizar que todos los inputs tengan labels asociados

### 2. Consistencia Visual
- Unificar espaciados y bordes
- Estandarizar jerarquía tipográfica
- Unificar sombras

### 3. UX/Interacción
- Mejorar feedback visual de interacciones
- Añadir transiciones suaves para hover/focus

## Plan de Ejecución

### Fase 1: Auditoría de Accesibilidad (Agente reviewer)
- Revisar cada vista contra checklist WCAG AA
- Documentar problemas de contraste, focus, aria
- Proponer lista priorizada de correcciones

### Fase 2: Implementación (Agente implementer)
- Aplicar mejoras de accesibilidad
- Añadir estados de focus visibles
- Corregir contrastes insuficientes
- Añadir labels a formularios

### Fase 3: Verificación (Agente tester)
- Verificar que cambios compilan
- Validar accesibilidad básica

## Agentes a utilizar

1. **reviewer** - Auditoría de accesibilidad
2. **implementer** - Aplicar mejoras
3. **tester** - Verificar cambios