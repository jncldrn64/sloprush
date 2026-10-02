# ARCHITECTURE.md: qué es el código hoy

> **Rol:** descriptivo. Dice qué es el código hoy, y cada sección nombra la fecha en que se
> comprobó contra él. Si contradice al código, el código gana y este archivo es el que está mal.
> Cómo tiene que escribirse el código es `docs/DESIGN.md`. **Régimen:** se corrige, y un hueco se
> borra en el PR que lo cierra. **Origen:** plantilla 1.0, sembrada el 2026-10-01
> (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla 1.0").

Una sección se describe cuando un PR de código toca su área, no antes. Lo que falta describir se
lista en la sección 5.

## 1. El repo, archivo por archivo

Comprobado el 2026-10-02 contra `git ls-files`.

- `.cargo/config.toml`: el linker para compilar hacia aarch64 desde x86_64, `aarch64-linux-gnu-gcc`.
- `.githooks/pre-commit`: `cargo fmt --check`, `cargo clippy` sin advertencias y `cargo test` antes
  de cada commit. Se activa con `git config core.hooksPath .githooks`.
- `.githooks/pre-push`: `cargo deny check` y `cargo audit` antes de cada push.
- `.gitignore`: deja afuera `target/`.
- `AGENTS.md`: la puerta para las herramientas que buscan ese nombre. Apunta a `CLAUDE.md`.
- `CHANGELOG.md`: qué cambió y cuándo.
- `CLAUDE.md`: el método de trabajo, la jerarquía de los documentos y el piso de prosa.
- `Cargo.lock`: la versión de cada crate del árbol.
- `Cargo.toml`: el paquete `sloprush` 0.2.0, edición 2024, sin publicar, con sus dependencias a
  versión exacta.
- `LICENSE`: el texto de la GNU Affero General Public License, versión 3.
- `deny.toml`: la configuración de cargo-deny.
- `docs/ARCHITECTURE.md`: este archivo.
- `docs/DECISIONS.md`: por qué el repo es como es.
- `docs/DESIGN.md`: cómo se escribe el código.
- `docs/GLOSSARY.md`: qué significa cada palabra.
- `docs/REQUIREMENTS.md`: qué tiene que ser verdad, y para quién.
- `docs/ROADMAP.md`: las fases hasta el mínimo viable y el Backlog.
- `docs/TEMPORARY-CONTEXT.md`: lo que se perdería si no se anota.
- `examples/arranque.rs`: el ejemplo de la fase 0. Arranca la GPU, imprime los adaptadores, el
  elegido y su backend, y cierra.
- `examples/ventana.rs`: el ejemplo de la fase 1. Abre una ventana, la limpia en cada cuadro e
  imprime cada tecla.
- `rust-toolchain.toml`: fija Rust 1.97.0, con rustfmt, clippy y el objetivo
  `aarch64-unknown-linux-gnu`.
- `src/dibujo.rs`: pasos de dibujo compartidos. Hoy, `limpiar`.
- `src/gpu.rs`: el arranque de wgpu. `crear_instancia`, `listar_adaptadores`, `iniciar` y
  `es_por_software`, y la elección de backend, `Eleccion` y `separar_backend`.
- `src/lib.rs`: la raíz del crate. Expone `dibujo`, `gpu`, `registro`, `ventana`, `wgpu` y
  `VERSION`.
- `src/registro.rs`: `arranque`, `carga`, `cierre`, `entrada`, `advertencia`, `resultado` y
  `error`.
- `src/ventana.rs`: la ventana con winit. `Opciones`, el rasgo `Escena` y `correr`, que abre la
  ventana, conecta la superficie de wgpu y dibuja la escena cuadro a cuadro.
- `tests/README.md`: cómo se corren las pruebas.
- `tests/arranque.rs`: las pruebas de la fase 0.
- `tests/cerrar_ventana.py`: pide el cierre de una ventana X11 como un gestor de ventanas.
- `tests/ventana.sh`: la prueba de la fase 1, dentro de un Xvfb.

## 2. Dónde vive cada cosa

Comprobado el 2026-10-02 contra el código.

| Qué | Símbolo | Ruta |
|---|---|---|
| Versión que muestra el motor | `VERSION`, de `CARGO_PKG_VERSION` | `src/lib.rs` |
| Backend pedido | `Eleccion`, `separar_backend`, opción `--backend` | `src/gpu.rs` |
| Límites del dispositivo | `Limits::downlevel_webgl2_defaults` dentro de `iniciar` | `src/gpu.rs` |
| Detección de software | `es_por_software`, `NOMBRES_DE_SOFTWARE` | `src/gpu.rs` |
| Opciones con ventana | `Opciones::desde_args`: `--cuadros`, `--limite-fps` | `src/ventana.rs` |
| Modo de presentación | `AutoVsync`, o `AutoNoVsync` con `--limite-fps` | `src/ventana.rs` |
| Tamaño inicial de la ventana | 640 por 480, en `App::abrir` | `src/ventana.rs` |
| Formato del registro | `[etapa] mensaje` en stdout; `[error]` en stderr | `src/registro.rs` |
| Versión de Rust | `channel` | `rust-toolchain.toml` |
| Linker de aarch64 | `linker` | `.cargo/config.toml` |
| Licencias y avisos | `[graph]`, `[advisories]`, `[licenses]`, `[sources]` | `deny.toml` |

## 3. Recursos y cómo se reinicia cada uno

Comprobado el 2026-10-02 contra el código.

- `target/`: lo que compila cargo. Se regenera solo; `cargo clean` lo borra.
- Las bases de avisos de RustSec que bajan cargo-deny y cargo-audit, fuera del repo, en
  `~/.cargo/advisory-dbs` y `~/.cargo/advisory-db`. Cada corrida las actualiza.
- La GPU: `iniciar` crea instancia, adaptador, dispositivo y cola dentro de `Gpu`, y se liberan al
  soltar ese valor.
- La ventana y su superficie: las crea `App::abrir` al arrancar el bucle de eventos, y se liberan
  en `exiting`, al terminar.

## 4. Tamaño

Comprobado el 2026-10-02: 766 líneas de Rust, contadas con
`wc -l src/*.rs examples/*.rs tests/*.rs | tail -1`.

## 5. Sin describir todavía

Nada. Cada archivo de `git ls-files` figura en la sección 1, comprobado el 2026-10-02.

## 6. Huecos conocidos

Lo que está abierto o no se verificó contra datos reales. Cada hueco abre con su fecha y trae el
comando o la corrida que lo muestra, para que el próximo lector compruebe si sigue abierto. Se borra
en el PR que lo cierra, y el CHANGELOG guarda el rastro. Un PR que anuncia algo en el CHANGELOG mira
si eso cierra un hueco de esta lista.

- **2026-10-01: Panfrost en el equipo mínimo.** Sin verificar que el driver Panfrost esté activo en
  el kernel del equipo mínimo, ni que su GPU sea la Mali-G31 que da por hecha el material de la
  siembra. En el equipo mínimo: `lsmod | grep panfrost`, `dmesg | grep -i -E "panfrost|mali"` y
  `ls -l /dev/dri/`. Bloquea la fase 0.
- **2026-10-01: Una ventana en el equipo mínimo.** Sin verificar que el equipo mínimo pueda mostrar
  una ventana; su neofetch no mostró sesión gráfica. winit 0.30.13 abre ventanas solo por X11 o por
  Wayland, así que hace falta un servidor o un compositor. En la conversación de diseño se propuso
  cage, un compositor Wayland de una sola aplicación que bookworm trae por apt. Nunca se instaló ni
  se probó. Lo muestra el ejemplo `ventana` corrido con cage en el equipo mínimo, con los comandos
  de la fase 1 de `docs/ROADMAP.md`. Bloquea la fase 0.
- **2026-10-01: glibc de bookworm.** Sin verificar que un binario aarch64 compilado en el equipo de
  desarrollo arranque con la glibc de Debian bookworm del equipo mínimo. El 2026-10-02, compilado
  en un contenedor con Ubuntu 24.04 y glibc 2.39, el ejemplo `arranque` pide como máximo
  `GLIBC_2.34`, y bookworm trae la 2.36. Lo muestran `aarch64-linux-gnu-objdump -T` sobre el binario
  de `target/aarch64-unknown-linux-gnu/release/examples/`, `ldd --version` en el equipo mínimo y
  arrancar el binario ahí. Bloquea la fase 0.
- **2026-10-01: Vulkan en el equipo de desarrollo.** Sin verificar. En el equipo de desarrollo:
  `vulkaninfo --summary`.
- **2026-10-01: Frecuencia del monitor del equipo de desarrollo.** Sin verificar. En el equipo de
  desarrollo: `xrandr`.
- **2026-10-01: wgpu sobre Panfrost.** Sin verificar que wgpu funcione con el backend OpenGL ES
  sobre Panfrost en la Mali-G31. El README de wgpu 30.0.1 marca OpenGL ES 3.0+ en Linux como
  "Downlevel/Best Effort Support". Lo muestra el ejemplo de la fase 0 corrido en el equipo mínimo,
  que imprime adaptador y backend. Bloquea la fase 0.
- **2026-10-01: Licencia de Fyrox.** El manifiesto de Fyrox 1.0.1 declara MIT, y el paquete
  publicado no trae archivo de licencia. Lo muestra el archivo de licencia de su repositorio,
  https://github.com/FyroxEngine/Fyrox. Bevy, macroquad y ggez quedaron comprobados:
  `docs/DECISIONS.md`, 2026-10-01 "Fuentes consultadas en la siembra".
- **2026-10-01: Memoria al compilar.** El equipo de desarrollo mostraba 13649 MiB ocupados de
  15945, y compilar puede agotarla. Sin medir. Lo muestra `free -m` antes y durante la primera
  compilación completa.
- **2026-10-01: Rapier.** Candidato para las físicas. rapier3d 0.36.0 declara Apache-2.0, del
  escalón 1. Su primer release es del 2020-08-19 y el último del 2026-09-25, según la API de
  crates.io consultada el 2026-10-01, así que como dependencia directa cumple la regla 3 de
  `docs/DESIGN.md`, "Seguridad de las dependencias". Sin verificar: su árbol de dependencias
  contra las dos normas. Lo muestra `cargo deny check` con un `Cargo.lock` que lo traiga. La
  decisión entre físicas propias o un crate sigue sin tomar.
