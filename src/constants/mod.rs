/// Global constants for the simulation.

// ─── Simulation Limits ───────────────────────────────────────────────────────

pub const SYS_KERNEL_PID: u32 = 0x00A1;

pub const MIN_BURST: u32 = 5;
pub const MAX_BURST: u32 = 50;

pub const MIN_PRIORITY: u8 = 1;
pub const MAX_PRIORITY: u8 = 10;

pub const MIN_MEMORY: f32 = 16.0;
pub const MAX_MEMORY: f32 = 512.0;

// ─── I/O & Interrupts ────────────────────────────────────────────────────────

pub const IO_PROBABILITY: f64 = 0.15;
pub const MIN_IO_BURST: u32 = 5;
pub const MAX_IO_BURST: u32 = 20;

// ─── Timer Speeds ────────────────────────────────────────────────────────────

pub const SPEED_1X_MS: u64 = 1000;
pub const SPEED_2X_MS: u64 = 500;
pub const SPEED_5X_MS: u64 = 200;
pub const SPEED_10X_MS: u64 = 100;
