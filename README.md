# KERNEL_OS — Simulador de Gestión de Procesos

Aplicación de escritorio nativa que simula la gestión de procesos de un sistema operativo con interfaz visual estilo cyberpunk/terminal de kernel.

## Características

- **6 algoritmos de planificación**: FCFS, SJF, SRTF, Round Robin, Prioridad, Prioridad Preemptiva
- **Simulación en tiempo real** con velocidad ajustable (1x, 2x, 5x, 10x)
- **Dashboard interactivo** con colas de procesos, CPU panel y logs
- **Diagrama de Gantt** con métricas post-simulación
- **Gestión de procesos** con tabla, edición y acciones de lote
- **Eventos de I/O aleatorios** con 15% de probabilidad por tick
- **Ejecutable portable** sin dependencias en tiempo de ejecución

## Requisitos de Compilación

### Windows

1. Instalar [Rust](https://rustup.rs/) (rustup)
2. Desde la terminal:
```powershell
cargo build --release
```
3. El ejecutable estará en `target/release/simulador-procesos.exe`

### Linux

1. Instalar Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Instalar dependencias del sistema (para el backend de renderizado):
```bash
# Ubuntu/Debian
sudo apt install libfontconfig1-dev libfreetype6-dev

# Fedora
sudo dnf install fontconfig-devel freetype-devel
```
3. Compilar:
```bash
cargo build --release
```
4. El ejecutable estará en `target/release/simulador-procesos`

## Ejecución en Modo Desarrollo

```bash
cargo run
```

## Stack Técnico

| Capa | Tecnología |
|------|-----------|
| Backend / Lógica | Rust (Edition 2021) |
| GUI | Slint 1.16 |
| RNG | rand 0.8 |

## Estructura del Proyecto

```
simuladorprocesos/
├── Cargo.toml          # Dependencias y configuración
├── build.rs            # Compilador de archivos .slint
├── src/                # Código Rust
│   ├── main.rs         # Entry point, Timer, bridge UI↔Logic
│   ├── process.rs      # PCB, ProcessState, generación
│   ├── metrics.rs      # Cálculos de métricas
│   ├── simulation.rs   # Motor de simulación
│   └── scheduler/      # Algoritmos de planificación
│       ├── mod.rs      # Trait + Scheduler
│       ├── fcfs.rs
│       ├── sjf.rs
│       ├── srtf.rs
│       ├── round_robin.rs
│       ├── priority.rs
│       └── priority_preemptive.rs
└── ui/                 # Interfaz Slint
    ├── app.slint       # Ventana principal
    ├── theme/          # Paleta y tipografía
    ├── structs.slint   # Structs compartidos
    ├── globals.slint   # Estado global
    └── components/     # Componentes visuales
```

## Licencia

Uso educativo.
