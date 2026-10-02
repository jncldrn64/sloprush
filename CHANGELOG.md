# CHANGELOG.md: qué cambió y cuándo

> **Rol:** historia. **Régimen:** crece por secciones, lo más nuevo arriba, y una sección publicada
> no se reescribe. Formato [Keep a Changelog](https://keepachangelog.com): cada sección abre con
> `## vX.Y — AAAA-MM-DD`. **Origen:** plantilla 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`,
> 2026-10-01 "Se adopta la plantilla 1.0").

## v0.2 — 2026-10-02

Primera versión con código. La versión que muestra el motor pasa a 0.2.0.

### Added
- Fase 0: el proyecto Cargo `sloprush` con Rust 1.97.0 fijado, wgpu 30.0.1 y pollster 1.0.1 a
  versión exacta, y su `Cargo.lock`.
- Fase 0: el ejemplo `arranque`, que imprime los adaptadores, el elegido y su backend, y advierte
  si dibuja por software.
- Fase 0: `deny.toml`, los hooks de git de `.githooks/` y el linker para compilar hacia aarch64.

### Changed
- `CLAUDE.md`, sección 9: las citas textuales del autor no cuentan para las reglas 1 y 2 de
  prosa, y sus líneas base se midieron de nuevo.

### Removed
- El hueco "Licencias del árbol de wgpu": `cargo deny check` pasó con el árbol de la fase 0.

## v0.1 — 2026-10-01

Solo documentación. Corrige y completa la siembra del mismo día con los motivos de las decisiones y
cuatro cambios de regla.

### Added
- `docs/DECISIONS.md`: diecinueve entradas. Diecisiete reemplazan a decisiones de la siembra, con
  su motivo o con su regla nueva. "Seguridad de las dependencias" y las fuentes de esta corrección
  son nuevas.
- `docs/DESIGN.md`: el principio "Seguridad de las dependencias", con sus comandos de cargo-deny.
- `docs/REQUIREMENTS.md`: la frase A13 del autor, y la pregunta de si la AGPL-3.0 sirve a su fin.
- `docs/ROADMAP.md`: al Backlog, multijugador con servidor, detección continua de colisiones,
  sandbox para scripts de terceros y tráfico escrito como script.
- `docs/GLOSSARY.md`: los términos aviso y escalera de licencias.

### Changed
- Licencias de las dependencias: una escalera de cuatro escalones, con Unicode-3.0 en el primero.
- Simulación: paso fijo de frecuencia configurable, 60 Hz por defecto. La prueba de caída pide la
  misma cifra con el dibujo a 30 y a 240 cuadros por segundo, y menos de 2 % de desvío a 60 Hz.
- Niveles gráficos: los define la marca de la tabla de wgpu, y no una lista de nombres.
- `docs/ROADMAP.md`: la fase 0 sin el bloqueo por antigüedad de crates y con el alcance repartido
  entre agente y autor. Las fases 2 y 3 suman un criterio que da sí o no sin mirar la pantalla.
- `docs/DESIGN.md`: cada "Por qué" dice qué pidió el autor y qué propuso la conversación de diseño.
- `CLAUDE.md`, sección 10: se consultan solo proyectos del escalón 1. Sección 9: líneas base
  medidas de nuevo.
- `docs/ARCHITECTURE.md`: los huecos "Licencias del árbol de wgpu" y "Rapier".

### Removed
- `docs/modulo/` y su renglón de `docs/ARCHITECTURE.md`, que contradecían la decisión sobre la API
  de bloques.
- De "Sin escribir todavía": la antigüedad de crates, la tolerancia de la prueba de caída y el nivel
  base en Windows, en `docs/DESIGN.md`, y qué pasa si el motor falla, en `docs/REQUIREMENTS.md`.

## v0.1 — 2026-10-01

Solo documentación. Siembra los documentos de la plantilla 1.0 con el material que el autor dio el
2026-10-01.

### Added
- `docs/REQUIREMENTS.md`: las frases del autor, citadas tal cual, y los requisitos que salen de
  ellas.
- `docs/DESIGN.md`: siete principios, más presentación y registro, y una sección "Sin escribir
  todavía" con lo que falta decidir.
- `docs/DECISIONS.md`: por qué se adopta la plantilla, la licencia AGPL-3.0, cada decisión de la
  conversación de diseño y las fuentes consultadas.
- `docs/ROADMAP.md`: el estado `lista para verificación`, las fases 0 a 4 hasta el mínimo viable y
  el Backlog.
- `docs/ARCHITECTURE.md`: el repo archivo por archivo y once huecos conocidos.
- `docs/GLOSSARY.md`: el grupo "Motor", con los términos del producto.
- `CLAUDE.md`: tres reglas de método, sobre cifras de rendimiento, documentación de la versión y
  código externo, y las líneas base de prosa.
- Comprobadas contra sus paquetes publicados las licencias de Bevy, macroquad y ggez. El hueco
  nació y se cerró en esta versión.

### Removed
- Las copias en la raíz de los documentos de `docs/` y de `tests/README.md`, idénticas según
  `cmp`.
- `PLANTILLA.md`, que explicaba la plantilla y no forma parte del repo.
