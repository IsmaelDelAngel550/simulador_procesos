# Mejores Prácticas — KERNEL_OS Simulator

Documento de referencia para mantener el código modular, legible y mantenible a lo largo del desarrollo del simulador.

---

## 1. Principios de Diseño

### 1.1 Separación Estricta de Capas

```
┌─────────────────────────────────────────────────────┐
│  UI Layer (Slint .slint files)                       │
│  • Solo layout, estilos, animaciones, bindings       │
│  • CERO lógica de negocio                            │
│  • Comunica con Rust solo via properties y callbacks │
├─────────────────────────────────────────────────────┤
│  Bridge Layer (main.rs)                              │
│  • Conecta UI ↔ Logic                                │
│  • Convierte structs Rust → Slint structs            │
│  • Maneja Timer loop                                 │
│  • NO contiene lógica de simulación                  │
├─────────────────────────────────────────────────────┤
│  Logic Layer (scheduler/, simulation.rs, process.rs) │
│  • Toda la lógica de negocio                         │
│  • Independiente de UI (testeable sin Slint)         │
│  • Structs puros de Rust, sin dependencia de Slint   │
└─────────────────────────────────────────────────────┘
```

> [!IMPORTANT]
> La capa de lógica **nunca** debe importar `slint::*`. La conversión de tipos se hace exclusivamente en `main.rs`.

### 1.2 Un Archivo, Una Responsabilidad

Cada archivo debe tener una responsabilidad clara y única:

| ❌ Incorrecto | ✅ Correcto |
|-------------|-----------|
| `scheduler.rs` con 800 líneas y todos los algoritmos | `scheduler/mod.rs` + un archivo por algoritmo |
| `ui.slint` con todas las pantallas | Un archivo `.slint` por componente visual |
| `main.rs` con lógica de simulación embebida | `main.rs` solo como bridge, lógica en `simulation.rs` |

### 1.3 Límite de Líneas por Archivo

| Tipo de archivo | Máximo recomendado | Acción si se excede |
|----------------|-------------------|---------------------|
| `.rs` (lógica) | 200 líneas | Extraer a sub-módulo |
| `.rs` (main.rs bridge) | 300 líneas | Extraer helpers a módulo `bridge.rs` |
| `.slint` (componente) | 150 líneas | Extraer sub-componentes |
| `.slint` (pantalla) | 250 líneas | Descomponer en componentes hijos |

---

## 2. Estructura Rust — Backend

### 2.1 Patrón Trait para Algoritmos de Planificación

Cada algoritmo implementa un trait común. Esto permite agregar nuevos algoritmos sin modificar el Scheduler:

```rust
// scheduler/mod.rs

/// Trait que todos los algoritmos de planificación deben implementar.
/// 
/// El método `select_next` recibe la cola de listos inmutable y devuelve
/// el índice del proceso que debe ejecutarse a continuación.
/// Devuelve `None` si la cola está vacía.
pub trait SchedulingAlgorithm {
    /// Selecciona el siguiente proceso a ejecutar de la ready queue.
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize>;
    
    /// Determina si el proceso actual debe ser preemptado.
    /// Solo relevante para algoritmos preemptivos.
    /// `current` es el proceso actualmente en CPU.
    /// `ready_queue` es la cola de procesos listos.
    fn should_preempt(
        &self,
        current: &PCB,
        ready_queue: &VecDeque<PCB>,
    ) -> bool {
        // Default: no preemption
        false
    }
    
    /// Indica si este algoritmo usa quantum (solo Round Robin).
    fn uses_quantum(&self) -> bool {
        false
    }
    
    /// Nombre del algoritmo para logging.
    fn name(&self) -> &'static str;
}
```

Cada implementación vive en su propio archivo:

```rust
// scheduler/fcfs.rs
pub struct Fcfs;

impl SchedulingAlgorithm for Fcfs {
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize> {
        if ready_queue.is_empty() { None } else { Some(0) }
    }
    
    fn name(&self) -> &'static str { "FCFS" }
}
```

```rust
// scheduler/round_robin.rs
pub struct RoundRobin;

impl SchedulingAlgorithm for RoundRobin {
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize> {
        if ready_queue.is_empty() { None } else { Some(0) }
    }
    
    fn uses_quantum(&self) -> bool { true }
    
    fn name(&self) -> &'static str { "Round Robin" }
}
```

### 2.2 Convención para el Scheduler

El `Scheduler` es agnóstico del algoritmo específico. Usa un `Box<dyn SchedulingAlgorithm>`:

```rust
pub struct Scheduler {
    algorithm: Box<dyn SchedulingAlgorithm>,
    quantum: u32,
    quantum_remaining: u32,
    ready_queue: VecDeque<PCB>,
    blocked_queue: VecDeque<PCB>,
    terminated: Vec<PCB>,
    current_process: Option<PCB>,
    clock: u32,
    gantt_log: Vec<GanttEntry>,
    sys_log: Vec<LogEntry>,
}

impl Scheduler {
    pub fn new(algo: Algorithm, quantum: u32) -> Self {
        let algorithm: Box<dyn SchedulingAlgorithm> = match algo {
            Algorithm::FCFS => Box::new(fcfs::Fcfs),
            Algorithm::SJF => Box::new(sjf::Sjf),
            Algorithm::SRTF => Box::new(srtf::Srtf),
            Algorithm::RoundRobin => Box::new(round_robin::RoundRobin),
            Algorithm::Priority => Box::new(priority::PriorityNonPreemptive),
            Algorithm::PriorityPreemptive => Box::new(priority_preemptive::PriorityPreemptive),
        };
        // ...
    }
}
```

### 2.3 Naming Conventions — Rust

| Elemento | Convención | Ejemplo |
|----------|-----------|---------|
| Structs | PascalCase | `ProcessState`, `GanttEntry` |
| Enums | PascalCase | `Algorithm::RoundRobin` |
| Funciones | snake_case | `calculate_metrics()` |
| Constantes | SCREAMING_SNAKE | `MAX_PRIORITY`, `IO_PROBABILITY` |
| Módulos | snake_case | `scheduler`, `round_robin` |
| Archivos | snake_case | `priority_preemptive.rs` |
| Variables | snake_case | `ready_queue`, `remaining_time` |

### 2.4 Constantes Globales

Todas las constantes van en un archivo dedicado o como `const` en el módulo correspondiente:

```rust
// process.rs
pub const SYS_KERNEL_PID: u32 = 0x00A1;
pub const MIN_BURST: u32 = 5;
pub const MAX_BURST: u32 = 50;
pub const MIN_PRIORITY: u8 = 1;
pub const MAX_PRIORITY: u8 = 10;
pub const MIN_MEMORY: f32 = 16.0;
pub const MAX_MEMORY: f32 = 512.0;
pub const IO_PROBABILITY: f64 = 0.15;    // 15%
pub const MIN_IO_BURST: u32 = 5;
pub const MAX_IO_BURST: u32 = 20;
```

### 2.5 Documentación de Código

```rust
/// Avanza la simulación un tick.
///
/// Este método es el corazón del simulador. En cada tick:
/// 1. Verifica llegadas de nuevos procesos
/// 2. Maneja completions de I/O
/// 3. Ejecuta el proceso actual (decrementa remaining_time)
/// 4. Evalúa preemption según el algoritmo
/// 5. Registra eventos en gantt_log y sys_log
///
/// # Panics
/// No debe causar panic en operación normal.
pub fn tick(&mut self) {
    // ...
}
```

### 2.6 Manejo de Errores

- Usar `Result<T, E>` para operaciones que pueden fallar (inicialización, I/O)
- Usar `Option<T>` para valores que pueden no existir (current_process, finish_time)
- **Nunca** usar `.unwrap()` en código de producción excepto en aserciones de invariantes documentadas
- En `main.rs`, propagar errores con `?` y manejar el error final con un mensaje descriptivo

```rust
// ✅ Correcto
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.run()
}

// ❌ Incorrecto
fn main() {
    let ui = AppWindow::new().unwrap();  // No en producción
    ui.run().unwrap();
}
```

---

## 3. Estructura Slint — Frontend

### 3.1 Jerarquía de Componentes

```
app.slint (Ventana raíz)
├── import theme/colors.slint
├── import theme/typography.slint
├── import structs.slint
├── import globals.slint
│
├── InitModal (component)
│   └── usa: HudButton, HudInput, HudCard
│
├── Dashboard (component)
│   ├── HeaderBar
│   │   └── usa: HudCard
│   ├── ReadyQueue
│   │   └── usa: HudCard, HudBadge
│   ├── BlockedQueue
│   │   └── usa: HudCard, HudBadge
│   ├── CpuPanel
│   │   └── usa: HudCard, HudBadge
│   ├── TerminatedList
│   │   └── usa: HudCard
│   ├── SysLog
│   │   └── usa: HudCard
│   └── Footer
│
├── Sidebar (component) — shared across screens
│
├── ProcessTable (component)
│   ├── BatchActions
│   │   └── usa: HudButton
│   ├── TableRow (repeated)
│   │   └── usa: HudBadge, HudButton
│   └── Pagination
│       └── usa: HudButton
│
├── EditModal (component)
│   └── usa: HudCard, HudInput, HudSlider, HudButton
│
└── GanttView (component)
    ├── GanttMetrics
    │   └── usa: HudCard
    ├── GanttChart (custom drawing)
    └── GanttTable
```

### 3.2 Convención de Archivos .slint

Cada archivo `.slint` sigue esta estructura:

```slint
// ============================================
// Componente: ReadyQueue
// Pantalla: Dashboard (Panel izquierdo)
// Descripción: Muestra la cola de procesos listos
// ============================================

import { KernelColors } from "../theme/colors.slint";
import { ProcessItem } from "../structs.slint";
import { HudCard } from "../common/hud_card.slint";
import { HudBadge } from "../common/hud_badge.slint";

export component ReadyQueue inherits Rectangle {
    // === Properties ===
    in property <[ProcessItem]> processes: [];
    in property <int> count: 0;
    
    // === Callbacks ===
    callback process-clicked(int);  // PID
    
    // === Layout ===
    background: KernelColors.bg-card;
    border-radius: 4px;
    border-width: 1px;
    border-color: KernelColors.border-default;
    
    VerticalLayout {
        padding: 16px;
        spacing: 8px;
        
        // Header
        HorizontalLayout {
            Text { text: "● COLA LISTOS"; /* ... */ }
            Text { text: "Q:" + count; /* ... */ }
        }
        
        // List items
        for process in processes: Rectangle {
            // Item rendering...
        }
    }
}
```

### 3.3 Naming Conventions — Slint

| Elemento | Convención | Ejemplo |
|----------|-----------|---------|
| Componentes | PascalCase | `ReadyQueue`, `HudBadge` |
| Propiedades | kebab-case | `burst-time`, `is-running` |
| Callbacks | kebab-case | `process-clicked`, `save-changes` |
| Archivos | snake_case | `ready_queue.slint`, `hud_badge.slint` |
| Globals | PascalCase | `KernelColors`, `SimState` |
| Structs | PascalCase | `ProcessItem`, `LogItem` |
| Colores (vars) | kebab-case | `bg-deep`, `accent-cyan` |

### 3.4 Componentes Reutilizables (common/)

Cada componente en `common/` es un bloque base usado por componentes de nivel superior:

#### HudCard — Card con esquinas decorativas

```slint
export component HudCard inherits Rectangle {
    in property <string> title: "";
    in property <bool> show-corner-decorations: true;
    
    background: KernelColors.bg-card;
    border-radius: 4px;
    border-width: 1px;
    border-color: KernelColors.border-default;
    
    // Corner decorations (L-shaped brackets)
    if show-corner-decorations: Rectangle {
        // Top-left corner bracket
        // Top-right corner bracket
        // etc.
    }
}
```

#### HudButton — Botón estilizado

```slint
export component HudButton inherits Rectangle {
    in property <string> text: "";
    in property <bool> primary: false;
    in property <bool> enabled: true;
    callback clicked();
    
    // Hover glow effect
    animate background { duration: 150ms; easing: ease-in-out; }
    animate border-color { duration: 150ms; easing: ease-in-out; }
}
```

#### HudBadge — Badge de estado

```slint
export component HudBadge inherits Rectangle {
    in property <int> state: 0;  // 0-4 maps to ProcessState
    
    // Pulsating dot for RUNNING and BLOCKED states
    // Color mapped from state
}
```

### 3.5 Animaciones — Guía de Uso

```slint
// ✅ CORRECTO: Animaciones sutiles y con propósito
Rectangle {
    x: is-active ? 0px : -50px;
    opacity: is-active ? 1.0 : 0.0;
    animate x { duration: 300ms; easing: ease-out; }
    animate opacity { duration: 200ms; easing: ease-in; }
}

// ❌ INCORRECTO: Animaciones que no aportan
Rectangle {
    animate width { duration: 1000ms; }  // Demasiado lento
    animate height { duration: 1000ms; } // No aporta a UX
}
```

Reglas para animaciones:
1. **Duración**: 150-400ms para transiciones, 600-1000ms para loops (pulsación)
2. **Easing**: `ease-out` para entradas, `ease-in` para salidas, `ease-in-out` para loops
3. **Propósito**: Cada animación debe comunicar un cambio de estado o guiar la atención

---

## 4. Comunicación UI ↔ Rust

### 4.1 Patrón de Properties

```slint
// En app.slint
export component AppWindow inherits Window {
    // Properties IN: Rust → UI (solo lectura desde UI)
    in property <[ProcessItem]> ready-queue: [];
    in property <[ProcessItem]> blocked-queue: [];
    in property <[ProcessItem]> terminated: [];
    in property <int> clock: 0;
    in property <float> cpu-load: 0;
    
    // Properties IN-OUT: bidireccionales
    in-out property <int> current-screen: 0;
    in-out property <bool> sim-running: false;
    
    // Callbacks: UI → Rust
    callback start-simulation(int, int, int, int);  // processes, memory, algo, quantum
    callback pause-simulation();
    callback step-simulation();
    callback edit-process(int, string, int, int);    // pid, name, burst, priority
    callback delete-process(int);                     // pid
}
```

### 4.2 Patrón de Actualización desde Rust

```rust
// main.rs — dentro del Timer callback
let ui_handle = ui.as_weak();
timer.start(TimerMode::Repeated, Duration::from_millis(interval), move || {
    let Some(ui) = ui_handle.upgrade() else { return };
    
    // 1. Avanzar simulación
    simulation.borrow_mut().tick();
    
    // 2. Convertir estado a tipos Slint
    let state = simulation.borrow().to_ui_state();
    
    // 3. Actualizar properties (batch update)
    ui.set_ready_queue(state.ready_queue.into());
    ui.set_blocked_queue(state.blocked_queue.into());
    ui.set_terminated(state.terminated.into());
    ui.set_clock(state.clock as i32);
    ui.set_cpu_load(state.cpu_load);
    // ...
});
```

### 4.3 Conversión de Tipos Rust → Slint

```rust
// Función helper en main.rs o módulo bridge
fn pcb_to_slint(pcb: &PCB) -> ProcessItem {
    ProcessItem {
        pid: pcb.pid as i32,
        pid_hex: SharedString::from(format!("0x{:04X}", pcb.pid)),
        name: SharedString::from(&pcb.name),
        state: pcb.state as i32,
        priority: pcb.priority as i32,
        arrival_time: pcb.arrival_time as i32,
        burst_time: pcb.burst_time as i32,
        remaining_time: pcb.remaining_time as i32,
        memory_mb: pcb.memory_mb,
        io_remaining: pcb.io_burst.unwrap_or(0) as i32,
        finish_time: pcb.finish_time.unwrap_or(0) as i32,
        turnaround_time: pcb.turnaround_time.unwrap_or(0) as i32,
        waiting_time: pcb.waiting_time.unwrap_or(0) as i32,
        selected: false,
    }
}
```

---

## 5. Testing

### 5.1 Tests Unitarios para Algoritmos

Cada algoritmo debe tener tests que verifican el orden de selección:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_pcb(pid: u32, burst: u32, priority: u8) -> PCB {
        PCB {
            pid,
            name: format!("P{}", pid),
            state: ProcessState::Ready,
            burst_time: burst,
            remaining_time: burst,
            arrival_time: 0,
            priority,
            memory_mb: 64.0,
            io_burst: None,
            finish_time: None,
            turnaround_time: None,
            waiting_time: None,
        }
    }
    
    #[test]
    fn sjf_selects_shortest_burst() {
        let algo = Sjf;
        let mut queue = VecDeque::new();
        queue.push_back(make_pcb(1, 10, 5));
        queue.push_back(make_pcb(2, 3, 5));
        queue.push_back(make_pcb(3, 7, 5));
        
        assert_eq!(algo.select_next(&queue), Some(1)); // PID 2 (burst=3)
    }
}
```

### 5.2 Tests de Integración del Scheduler

```rust
#[test]
fn fcfs_full_simulation() {
    let mut scheduler = Scheduler::new(Algorithm::FCFS, 0);
    scheduler.add_process(make_pcb(1, 5, 5));
    scheduler.add_process(make_pcb(2, 3, 5));
    
    // Run until completion
    while !scheduler.is_complete() {
        scheduler.tick();
    }
    
    assert_eq!(scheduler.terminated.len(), 2);
    assert_eq!(scheduler.terminated[0].pid, 1); // FCFS: P1 finishes first
    assert_eq!(scheduler.terminated[0].finish_time, Some(5));
    assert_eq!(scheduler.terminated[1].finish_time, Some(8));
}
```

---

## 6. Checklist de Código antes de Commit

- [ ] Cada archivo tiene menos del límite de líneas recomendado
- [ ] No hay `unwrap()` fuera de tests
- [ ] Todos los structs públicos tienen documentación `///`
- [ ] Los componentes `.slint` importan solo lo que necesitan
- [ ] Las animaciones tienen `duration` entre 150-400ms
- [ ] Las constantes están definidas como `const`, no como magic numbers
- [ ] Los nuevos algoritmos implementan `SchedulingAlgorithm` trait
- [ ] La lógica de simulación no importa tipos de Slint
- [ ] Los tests pasan: `cargo test`
