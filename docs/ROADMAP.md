# ROADMAP.md: qué sigue, en qué orden

> **Rol:** plan. Ordena el trabajo en fases y no cambia una norma: una fase que necesita otra la
> cambia por la regla 2 de la jerarquía de `CLAUDE.md`. **Régimen:** se corrige. Una fase cerrada
> no se borra. **Origen:** plantilla 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`, 2026-10-01
> "Se adopta la plantilla 1.0").

## Principio de orden

Primero va la fase 0, que es infraestructura y cierra los huecos que bloquean todo lo demás. Esa
fase no se corre de lugar. Después viene el mínimo viable, una capacidad por fase y nada más.

Dentro del mínimo viable el agente decide la estructura interna. La gravedad va última porque
necesita el cubo. El catálogo de bloques se extrae de lo construido y lo congela el autor, así que
no tiene fase mientras el mínimo viable no esté cerrado. Fuentes: `docs/DECISIONS.md`, 2026-10-01
"Primero el mínimo viable, después el catálogo de bloques" y "Alcance del mínimo viable".

## Reglas de fase

**Estados:** `pendiente`, `en curso`, `lista para verificación`, `cerrada (AAAA-MM-DD)`.

**Quién mueve el estado:** el agente lleva una fase hasta `lista para verificación`. Solo el autor
la pasa a `cerrada`, después de correr sus ejemplos en los dos equipos (`docs/DECISIONS.md`,
2026-10-01 "Estado de fase lista para verificación").

**Entrada a una fase en curso:** un ítem parqueado entra a una fase que ya arrancó solo si dejarlo
afuera hace imposible un incremento pendiente, o si obliga a rehacer trabajo ya entregado. Lo
demás espera a la fase siguiente.

**Una fase trae** un objetivo de una línea, su alcance, un criterio de aceptación verificable, qué
bloquea y qué la bloquea, y su estado.

**Cada fase termina con un ejemplo** en `examples/`, por lo menos, que corre con
`cargo run --example` y el nombre del ejemplo. El criterio de aceptación nombra ese ejemplo y el
equipo donde corre (`docs/DECISIONS.md`, 2026-10-01 "Cada fase cierra con un ejemplo").

**Qué prueba cada equipo:** el equipo mínimo, todo el 2D y el 3D básico. El equipo de desarrollo,
lo mismo, más el 3D pesado y las mediciones (`docs/DECISIONS.md`, 2026-10-01 "Dos equipos de
prueba").

## Fase 0: Infraestructura

**Estado:** `pendiente`

**Objetivo:** dejar el proyecto Cargo listo y cerrar los huecos que bloquean todo lo demás.

**Alcance:**
- Proyecto Cargo con el toolchain fijado.
- cargo deny y cargo audit.
- Hooks de git.
- Cierre de cuatro huecos de `docs/ARCHITECTURE.md`: "Panfrost en el equipo mínimo", "Una ventana
  en el equipo mínimo", "glibc de bookworm" y "wgpu sobre Panfrost".
- El ejemplo `arranque`, que imprime los pasos de arranque y cierre, el adaptador y el backend
  (`docs/DESIGN.md`, "Presentación y registro").

**Criterio de aceptación:** `cargo run --example arranque` corre en los dos equipos. En el equipo
mínimo imprime un adaptador Mali con backend OpenGL ES y ninguna advertencia de renderizador por
software; en el de desarrollo, un adaptador de hardware. En el equipo de desarrollo, cargo deny y
cargo audit terminan sin hallazgos. Los cuatro huecos quedan borrados de `docs/ARCHITECTURE.md`
con la corrida que cerró cada uno.

**Bloquea:** fases 1, 2, 3 y 4.

**Bloqueada por:** ninguna fase. Antes de la primera dependencia hace falta el criterio de
antigüedad de crates, sin decidir (`docs/DESIGN.md`, "Sin escribir todavía").

## Fase 1: Ventana y teclado

**Estado:** `pendiente`

**Objetivo:** abrir una ventana y leer el teclado.

**Alcance:**
- Ventana.
- Entrada de teclado.
- El ejemplo `ventana`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example ventana` abre una ventana,
imprime cada tecla que se aprieta y termina sin error al cerrar la ventana.

**Bloquea:** fases 2, 3 y 4.

**Bloqueada por:** fase 0.

## Fase 2: Sprite 2D

**Estado:** `pendiente`

**Objetivo:** dibujar un sprite 2D.

**Alcance:**
- Un sprite 2D en la ventana.
- El ejemplo `sprite`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example sprite` dibuja un sprite en la
ventana, y su registro no advierte renderizador por software.

**Bloquea:** nada.

**Bloqueada por:** fase 1.

## Fase 3: Cubo 3D con cámara

**Estado:** `pendiente`

**Objetivo:** dibujar un cubo 3D visto desde una cámara que se mueve con el teclado.

**Alcance:**
- Un cubo 3D.
- Una cámara que se mueve con el teclado.
- El ejemplo `cubo`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example cubo` dibuja un cubo en
perspectiva, las teclas mueven la cámara, y el registro no advierte renderizador por software.

**Bloquea:** fase 4.

**Bloqueada por:** fase 1.

## Fase 4: Gravedad sobre el cubo

**Estado:** `pendiente`

**Objetivo:** que el cubo caiga con gravedad, con la simulación a paso fijo de 60 Hz.

**Alcance:**
- Gravedad sobre el cubo.
- Simulación a paso fijo de 60 Hz, separada del dibujo (`docs/DESIGN.md`, "La simulación avanza a
  paso fijo de 60 Hz").
- El ejemplo `caida`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example caida` suelta el cubo en
reposo con gravedad 9,81 m/s² e imprime cuánto bajó tras 1 s de simulación. Con el dibujo limitado
a 30 y a 240 cuadros por segundo imprime la misma cifra. Compararla con 4,905 m necesita la
tolerancia que falta en `docs/DESIGN.md`, "Sin escribir todavía".

**Bloquea:** nada.

**Bloqueada por:** fase 3, y la decisión entre físicas propias o delegadas a un crate, sin tomar
(`docs/DESIGN.md`, "Sin escribir todavía").

## Backlog

Lo que ya se sabe que se quiere y todavía no tiene fase. Cada ítem nace con su línea de entrada.

- Scripting y catálogo de bloques, con la API de bloques. Su documentación propia va en una carpeta
  de `docs/` cuando el módulo exista.
  **Entró:** 2026-10-01, #2.
- GUI del motor con ejecución de código (A8 de `docs/REQUIREMENTS.md`).
  **Entró:** 2026-10-01, #2.
- Exportación de juegos (A9), con los avisos de licencia de las dependencias.
  **Entró:** 2026-10-01, #2.
- Carga por zonas.
  **Entró:** 2026-10-01, #2.
- Fondos prerrenderizados y skybox (A6).
  **Entró:** 2026-10-01, #2.
- Red.
  **Entró:** 2026-10-01, #2.
- Audio.
  **Entró:** 2026-10-01, #2.
- FreeBSD, Windows, macOS y navegador, como plataformas por verificar (A3).
  **Entró:** 2026-10-01, #2.
- La escena de medición de los 240 fps (A7), con las condiciones de `CLAUDE.md`, sección 8.
  **Entró:** 2026-10-01, #2.
