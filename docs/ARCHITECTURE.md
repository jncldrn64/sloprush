# ARCHITECTURE.md: qué es el código hoy

> **Rol:** descriptivo. Dice qué es el código hoy, y cada sección nombra la fecha en que se
> comprobó contra él. Si contradice al código, el código gana y este archivo es el que está mal.
> Cómo tiene que escribirse el código es `docs/DESIGN.md`. **Régimen:** se corrige, y un hueco se
> borra en el PR que lo cierra. **Origen:** plantilla 1.0, sembrada el 2026-10-01
> (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla 1.0").

Una sección se describe cuando un PR de código toca su área, no antes. Lo que falta describir se
lista en la sección 5.

## 1. El repo, archivo por archivo

Comprobado el 2026-10-01 contra `git ls-files`. No hay código.

- `AGENTS.md`: la puerta para las herramientas que buscan ese nombre. Apunta a `CLAUDE.md`.
- `CHANGELOG.md`: qué cambió y cuándo.
- `CLAUDE.md`: el método de trabajo, la jerarquía de los documentos y el piso de prosa.
- `LICENSE`: el texto de la GNU Affero General Public License, versión 3.
- `docs/ARCHITECTURE.md`: este archivo.
- `docs/DECISIONS.md`: por qué el repo es como es.
- `docs/DESIGN.md`: cómo se escribe el código.
- `docs/GLOSSARY.md`: qué significa cada palabra.
- `docs/REQUIREMENTS.md`: qué tiene que ser verdad, y para quién.
- `docs/ROADMAP.md`: las fases hasta el mínimo viable y el Backlog.
- `docs/TEMPORARY-CONTEXT.md`: lo que se perdería si no se anota.
- `tests/README.md`: cómo se corren los tests. Todavía no hay tests.

## 2. Dónde vive cada cosa

Sin describir todavía: no hay código (sección 5).

## 3. Recursos y cómo se reinicia cada uno

Sin describir todavía: no hay código (sección 5).

## 4. Tamaño

Sin describir todavía: no hay código (sección 5).

## 5. Sin describir todavía

Todo el código, porque todavía no hay. Lo lista `git ls-files '*.rs' Cargo.toml`, que el
2026-10-01 devolvió cero archivos.

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
  una ventana; su neofetch no mostró sesión gráfica. En la conversación de diseño se propuso cage,
  un compositor Wayland de una sola aplicación que se instala por apt. Nunca se instaló ni se
  probó. Lo muestra abrir un programa con ventana en el equipo mínimo, con un monitor conectado.
  Bloquea la fase 0.
- **2026-10-01: Qué pide la biblioteca de ventanas.** Sin verificar si la biblioteca de ventanas que
  se elija necesita X11 o Wayland, o si puede dibujar directo sobre la salida de video. Todavía no
  hay biblioteca elegida. Lo muestran la lista de plataformas de su documentación y una ventana
  abierta con ella en el equipo mínimo sin compositor.
- **2026-10-01: glibc de bookworm.** Sin verificar que un binario aarch64 compilado en el equipo de
  desarrollo arranque con la glibc de Debian bookworm del equipo mínimo. Lo muestran
  `ldd --version` en los dos equipos y arrancar ese binario en el equipo mínimo. Bloquea la fase 0.
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
- **2026-10-01: Licencias del árbol de wgpu.** Sin verificar que las dependencias de wgpu cumplan
  `docs/DESIGN.md`, "Licencias de las dependencias". El manifiesto de unicode-ident 1.0.26 declara
  `(MIT OR Apache-2.0) AND Unicode-3.0`, Unicode-3.0 no está en la lista, y proc-macro2 1.0.107
  depende de él. Lo muestra `cargo deny check licenses` con el primer `Cargo.lock` que traiga wgpu.
- **2026-10-01: Rapier.** Candidato para las físicas, sin comprobar contra la lista de licencias ni
  contra el criterio de antigüedad de crates, que está sin decidir. Lo muestra el campo `license`
  de su manifiesto publicado.
