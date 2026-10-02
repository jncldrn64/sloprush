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
"Primero el mínimo viable, después el catálogo de bloques, con su motivo" y "Alcance del mínimo
viable, con su motivo".

## Reglas de fase

**Estados:** `pendiente`, `en curso`, `lista para verificación`, `cerrada (AAAA-MM-DD)`.

**Quién mueve el estado:** el agente lleva una fase hasta `lista para verificación`. Solo el autor
la pasa a `cerrada`, después de correr sus ejemplos en los dos equipos (`docs/DECISIONS.md`,
2026-10-01 "Estado de fase lista para verificación, con su motivo").

**Entrada a una fase en curso:** un ítem parqueado entra a una fase que ya arrancó solo si dejarlo
afuera hace imposible un incremento pendiente, o si obliga a rehacer trabajo ya entregado. Lo
demás espera a la fase siguiente.

**Una fase trae** un objetivo de una línea, su alcance, un criterio de aceptación verificable, qué
bloquea y qué la bloquea, y su estado.

**Cada fase termina con un ejemplo** en `examples/`, por lo menos, que corre con
`cargo run --example` y el nombre del ejemplo. El criterio de aceptación nombra ese ejemplo y el
equipo donde corre (`docs/DECISIONS.md`, 2026-10-01 "Cada fase cierra con un ejemplo, con su
motivo").

**Qué prueba cada equipo:** el equipo mínimo, todo el 2D y el 3D básico. El equipo de desarrollo,
lo mismo, más el 3D pesado y las mediciones (`docs/DECISIONS.md`, 2026-10-01 "Dos equipos de
prueba, con su motivo").

## Fase 0: Infraestructura

**Estado:** `lista para verificación`

**Objetivo:** dejar el proyecto Cargo listo y cerrar los huecos que bloquean todo lo demás.

**Alcance a cargo del agente:**
- Proyecto Cargo con el toolchain fijado.
- cargo deny y cargo audit, configurados según `docs/DESIGN.md`, "Licencias de las dependencias"
  y "Seguridad de las dependencias".
- Hooks de git.
- El ejemplo `arranque`, que imprime los pasos de arranque y cierre, el adaptador y el backend
  (`docs/DESIGN.md`, "Presentación y registro"). Corrido en el equipo mínimo, cierra el hueco
  "wgpu sobre Panfrost" de `docs/ARCHITECTURE.md`.

**Alcance a cargo del autor**, porque piden sus manos en el equipo mínimo: cerrar los huecos
"Panfrost en el equipo mínimo", "Una ventana en el equipo mínimo" y "glibc de bookworm" de
`docs/ARCHITECTURE.md`.

**Criterio de aceptación:** `cargo run --example arranque` corre en los dos equipos. En el equipo
mínimo imprime un adaptador Mali con backend OpenGL ES y ninguna advertencia de renderizador por
software; en el de desarrollo, un adaptador de hardware. En el equipo de desarrollo,
`cargo deny check` y cargo audit terminan sin hallazgos. Los cuatro huecos quedan borrados de
`docs/ARCHITECTURE.md` con la corrida que cerró cada uno.

**Bloquea:** fases 1, 2, 3 y 4.

**Bloqueada por:** nada.

**Corrida del agente, el 2026-10-02:** en un contenedor x86_64 sin GPU, con Ubuntu 24.04, Rust
1.97.0 y los adaptadores por software de Mesa 25.2.8. `cargo run --example arranque` salió con
código 0 con `--backend vulkan` (lavapipe) y con `--backend gl` (llvmpipe por EGL, sin pantalla), y
las dos veces advirtió el renderizador por software. `cargo test` pasó 7 de 7, `cargo clippy` no dio
advertencias, `cargo deny check` dio `advisories ok, bans ok, licenses ok, sources ok` y `cargo
audit` revisó 126 crates sin avisos. El binario aarch64 pide como máximo `GLIBC_2.34`. Nada de esto
corrió en los equipos del autor.

**Para cerrarla, en el equipo de desarrollo:**

```sh
git clone https://github.com/jncldrn64/sloprush && cd sloprush
git config core.hooksPath .githooks
cargo run --example arranque -- --backend vulkan
cargo run --example arranque -- --backend gl
cargo test
cargo install --locked cargo-deny@0.20.2 cargo-audit@0.22.2
cargo deny check && cargo audit
vulkaninfo --summary
xrandr
```

Tiene que elegir la Radeon, sin la línea `[advertencia]`, en los dos backends. `vulkaninfo` y
`xrandr` cierran los huecos "Vulkan en el equipo de desarrollo" y "Frecuencia del monitor del equipo
de desarrollo".

**Para llevar el binario al equipo mínimo,** desde el equipo de desarrollo:

```sh
sudo apt install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --example arranque
aarch64-linux-gnu-objdump -T target/aarch64-unknown-linux-gnu/release/examples/arranque \
  | grep -o 'GLIBC_[0-9.]*' | sort -uV | tail -1
PI=usuario@direccion-de-la-orange-pi
scp target/aarch64-unknown-linux-gnu/release/examples/arranque "$PI":
```

La versión de glibc que imprime `objdump` tiene que ser 2.36 o menor, la de bookworm.

**Para cerrarla, en el equipo mínimo:**

```sh
lsmod | grep panfrost
dmesg | grep -i -E "panfrost|mali"
ls -l /dev/dri/
ldd --version
sudo apt install libegl1 libegl-mesa0 libgl1-mesa-dri
./arranque --backend gl
./arranque
```

Tiene que elegir la Mali con backend `Gl` y sin la línea `[advertencia]`. Si `/dev/dri/renderD128`
no deja leer al usuario, falta agregarlo al grupo `render`. Esas corridas cierran los huecos
"Panfrost en el equipo mínimo", "glibc de bookworm" y "wgpu sobre Panfrost". El hueco "Una ventana
en el equipo mínimo" se cierra con el ejemplo `ventana`, con los comandos de la fase 1.

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

**Criterio de aceptación:** en los dos equipos, `cargo run --example sprite` dibuja un número fijo
de cuadros y termina con código 0, y su registro no advierte renderizador por software. Eso da sí
o no sin mirar la pantalla. Que el sprite se vea en la ventana lo comprueba el autor.

**Bloquea:** nada.

**Bloqueada por:** fase 1.

## Fase 3: Cubo 3D con cámara

**Estado:** `pendiente`

**Objetivo:** dibujar un cubo 3D visto desde una cámara que se mueve con el teclado.

**Alcance:**
- Un cubo 3D.
- Una cámara que se mueve con el teclado.
- El ejemplo `cubo`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example cubo` dibuja un número fijo de
cuadros y termina con código 0, y su registro no advierte renderizador por software. Eso da sí o
no sin mirar la pantalla. Que el cubo se vea en perspectiva y que las teclas muevan la cámara lo
comprueba el autor.

**Bloquea:** fase 4.

**Bloqueada por:** fase 1.

## Fase 4: Gravedad sobre el cubo

**Estado:** `pendiente`

**Objetivo:** que el cubo caiga con gravedad, con la simulación a paso fijo de 60 Hz por defecto.

**Alcance:**
- Gravedad sobre el cubo.
- Simulación a paso fijo, separada del dibujo, con la frecuencia configurable y 60 Hz por defecto
  (`docs/DESIGN.md`, "La simulación avanza a paso fijo de frecuencia configurable").
- El ejemplo `caida`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example caida` suelta el cubo en
reposo con gravedad 9,81 m/s² e imprime cuánto bajó tras 1 s de simulación. Con el dibujo limitado
a 30 y a 240 cuadros por segundo imprime exactamente la misma cifra, y a 60 Hz esa cifra se aleja
menos de 2 % de 4,905 m.

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
- Multijugador con servidor, que puede subir la frecuencia de la simulación.
  **Entró:** 2026-10-01, #3.
- Detección continua de colisiones: más frecuencia reduce las colisiones perdidas y no las elimina.
  **Entró:** 2026-10-01, #3.
- Sandbox para scripts de terceros, sin acceso a disco ni a red salvo permiso (A8).
  **Entró:** 2026-10-01, #3.
- El tráfico escrito como script, como prueba de que el catálogo de bloques alcanza.
  **Entró:** 2026-10-01, #3.
