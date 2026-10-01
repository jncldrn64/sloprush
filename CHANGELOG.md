# CHANGELOG.md: qué cambió y cuándo

> **Rol:** historia. **Régimen:** crece por secciones, lo más nuevo arriba, y una sección publicada
> no se reescribe. Formato [Keep a Changelog](https://keepachangelog.com): cada sección abre con
> `## vX.Y — AAAA-MM-DD`. **Origen:** plantilla 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`,
> 2026-10-01 "Se adopta la plantilla 1.0").

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
