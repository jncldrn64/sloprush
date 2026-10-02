//! Motor sloprush.
//!
//! Lo que el motor exige está en `docs/REQUIREMENTS.md`, y cómo se escribe su código, en
//! `docs/DESIGN.md`. Qué hay hoy en cada módulo lo describe `docs/ARCHITECTURE.md`.

pub mod gpu;
pub mod registro;

/// wgpu, en la versión exacta que fija `Cargo.toml`, para que los ejemplos y las pruebas usen la
/// misma.
pub use wgpu;

/// Versión del motor, la última del CHANGELOG.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
