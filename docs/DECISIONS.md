# DECISIONS.md: por qué el repo es como es

> **Rol:** historia. Dice por qué el repo es como es. Una entrada vigente gana sobre un normativo
> que la contradice: el normativo quedó viejo y se corrige. **Régimen:** append-only. Una entrada no
> se edita ni se borra aunque quede obsoleta; una nueva la reemplaza y la nombra por fecha y título.
> **Origen:** plantilla 1.0, sembrada el 2026-10-01 (2026-10-01 "Se adopta la plantilla 1.0").

Cada entrada abre con `## AAAA-MM-DD: título` y lleva estos campos:

- **Contexto:** qué problema o pregunta la motivó.
- **Decisión:** qué se decidió.
- **Alternativas:** qué se descartó y por qué, si hubo.
- **Estado:** `vigente`, o `reemplazada por AAAA-MM-DD "título"`.

Una entrada que introduce o refina un término escribe su línea en `docs/GLOSSARY.md` en el mismo
PR.

## 2026-10-01: Se adopta la plantilla 1.0

**Contexto:** El repo arranca con el método de otros dos repos del autor, ya probado en uso. La
plantilla, como conjunto, se usa acá por primera vez.

**Decisión:** Se siembran los documentos canónicos con su jerarquía (`CLAUDE.md`, sección 2), su
formato común (`CLAUDE.md`, sección 4) y el piso de prosa sin líneas base. Las líneas base se miden
en este mismo PR. Con permiso del autor se sacan de la raíz las copias de los documentos de `docs/`
y de `tests/README.md`, idénticas según `cmp`. También se saca `PLANTILLA.md`, que explica la
plantilla y dice que no se copia al repo nuevo.

**Estado:** `vigente`

## 2026-10-01: Ideas descartadas

**Contexto:** Esta entrada existe para que una sesión no vuelva a proponer lo que ya se pesó y se
descartó. `CLAUDE.md` obliga a leerla antes de proponer un cambio de estructura o de método.

**Decisión:** Ninguna se retoma sin una razón nueva.

- Un repo maestro de reglas compartido entre proyectos. Cada repo es autosuficiente y no depende
  de otro para entenderse.
- Que `CLAUDE.md` guarde convenciones del código del producto. Van en `docs/DESIGN.md`.
- Confiar en la memoria del modelo entre sesiones. Lo que tiene que sobrevivir vive en el repo.

**Estado:** `vigente`

## 2026-10-01: La licencia del motor es AGPL-3.0

**Contexto:** El commit inicial, `1448ea3` del 2026-10-01, trajo un `LICENSE` con la GNU Affero
General Public License, versión 3. El material de la siembra decía que la licencia estaba sin
decidir, y el autor lo corrigió en la sesión de la siembra.

**Decisión:** El motor se licencia bajo AGPL-3.0, con el `LICENSE` de ese commit, que es el origen
de la decisión. Qué obliga la AGPL-3.0 a quien exporta un juego queda abierto en
`docs/REQUIREMENTS.md`, "Sin escribir todavía".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: El lenguaje es Rust estable

**Contexto:** Decisión de la conversación de diseño del autor, del 2026-10-01. Qué la motivó no
quedó escrito: **Sin origen recuperable.**

**Decisión:** El motor se escribe en Rust, canal estable. Norma en `docs/DESIGN.md`, "Rust
estable".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Toda la salida gráfica pasa por wgpu

**Contexto:** El motor tiene que correr en el equipo mínimo, y el navegador está entre las
plataformas por verificar. La GPU del equipo mínimo es una Mali-G31 según el material de la
siembra, sin comprobar en el equipo (`docs/ARCHITECTURE.md`, hueco "Panfrost en el equipo
mínimo").

**Decisión:** Toda la salida gráfica pasa por el crate wgpu, y ningún módulo llama directo a
Vulkan, OpenGL, Metal o DirectX. Norma en `docs/DESIGN.md`, "Toda la salida gráfica pasa por
wgpu".

**Alternativas:** Usar solo Vulkan con capas de compatibilidad. Se descartó porque Vulkan no existe
en el navegador y porque en la Mali-G31 el driver Vulkan libre no es conformante. La página de
Panfrost de Mesa, leída de nuevo el 2026-10-01, dice que PanVK es conformante solo en Mali-G610, y
que donde su soporte es experimental no carga salvo con `PAN_I_WANT_A_BROKEN_VULKAN_DRIVER=1`. La
misma página da OpenGL ES 3.1 para la G31, de arquitectura Bifrost v7, y dice que el OpenGL ES de
Panfrost es conformante solo en Mali-G52, G57 y G610.

**Estado:** `vigente`

## 2026-10-01: Dos niveles gráficos

**Contexto:** El fin es A5 de `docs/REQUIREMENTS.md`, sección 1.

**Decisión:** Hay dos niveles gráficos en lugar de un piso único. El nivel base es OpenGL ES 3.0 y
WebGL2, y el completo es Vulkan y Metal. Toda capacidad funciona en el base, y el completo solo
agrega opciones visuales o de rendimiento. Con la misma entrada, los dos llegan al mismo estado de
juego. Norma en `docs/DESIGN.md`, "Dos niveles gráficos".

**Alternativas:** Un piso único. Se descartó porque dejaba sin técnicas de rendimiento al nivel
alto.

**Estado:** `vigente`

## 2026-10-01: La simulación avanza a paso fijo de 60 Hz

**Contexto:** Decisión de la conversación de diseño. Qué la motivó no quedó escrito: **Sin origen
recuperable.**

**Decisión:** La simulación avanza a paso fijo de 60 Hz, separada del dibujo. La prueba prevista
es que un cuerpo soltado en reposo con gravedad 9,81 m/s² baje 4,905 m en 1 s de simulación, igual
con el dibujo limitado a 30 y a 240 cuadros por segundo. Norma en `docs/DESIGN.md`, "La simulación
avanza a paso fijo de 60 Hz".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Nativo contra script

**Contexto:** A10 y A11 de `docs/REQUIREMENTS.md` piden ver qué es parte nativa del motor y qué
puede ser una función documentada de script.

**Decisión:** Es nativo lo que toca GPU, sistema operativo o hardware, o corre en cada cuadro sobre
muchos objetos. Lo demás se intenta primero como script, y pasa a nativo solo con una medición
registrada. Norma en `docs/DESIGN.md`, "Nativo contra script".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Primero el mínimo viable, después el catálogo de bloques

**Contexto:** Lo que se hace con el motor se arma con bloques (A1), y había que elegir si el
catálogo se escribe antes o después del código.

**Decisión:** Primero se hace un mínimo viable, donde el agente decide la estructura interna. El
catálogo de bloques se extrae después de lo construido, y lo congela el autor. El orden está en
`docs/ROADMAP.md`, "Principio de orden".

**Alternativas:** Especificar el catálogo antes de tener código. Se descartó, y el porqué no quedó
escrito: **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Alcance del mínimo viable

**Contexto:** La entrada "Primero el mínimo viable, después el catálogo de bloques" pide un mínimo
viable, y había que decir qué entra.

**Decisión:** El mínimo viable es una ventana, un sprite 2D, un cubo 3D con cámara, entrada de
teclado y gravedad sobre el cubo, y nada más. Va al Backlog de `docs/ROADMAP.md`, sin fase:

- scripting y catálogo de bloques;
- GUI del motor con ejecución de código;
- exportación de juegos;
- carga por zonas;
- fondos prerrenderizados y skybox;
- red y audio;
- FreeBSD, Windows, macOS y navegador, como plataformas por verificar.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: La API de bloques es inestable hasta que el autor la congele

**Contexto:** El catálogo de bloques sale del mínimo viable, según la entrada "Primero el mínimo
viable, después el catálogo de bloques".

**Decisión:** La API de bloques es inestable hasta que el autor la congele por escrito. Mientras
el módulo no exista, no tiene carpeta propia en `docs/`.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Licencias admitidas en las dependencias

**Contexto:** Decisión de la conversación de diseño. Qué la motivó no quedó escrito: **Sin origen
recuperable.**

**Decisión:** Una dependencia solo entra con licencia MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause,
Zlib o ISC. La verificación prevista es con cargo deny y cargo audit, que entran en la fase 0 de
`docs/ROADMAP.md`. Norma en `docs/DESIGN.md`, "Licencias de las dependencias".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Versiones exactas y documentación de la versión

**Contexto:** Decisión de la conversación de diseño. Qué la motivó no quedó escrito: **Sin origen
recuperable.**

**Decisión:** Cada crate lleva su versión exacta en `Cargo.toml`, `Cargo.lock` está versionado, y
antes de usar una API se lee la documentación de esa versión. Las dos primeras son norma del código
en `docs/DESIGN.md`, "Versión exacta de cada dependencia". La lectura es regla de método, en
`CLAUDE.md`, sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: No se copia código de proyectos externos

**Contexto:** `CLAUDE.md`, sección 10, admitía copiar de terceros si lo copiado viajaba con su
LICENSE y su atribución. Para el código, esta decisión lo cierra.

**Decisión:** No se copia ni se traduce código de proyectos externos. Se pueden consultar proyectos
con las licencias de "Licencias admitidas en las dependencias", y cada consulta se registra como
entrada de este archivo con URL, licencia, fecha e idea tomada. No se abre código GPL, LGPL o AGPL,
ni descompilaciones de juegos. Cambia `CLAUDE.md`, sección 10.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: El motor imprime su arranque, su adaptador y su backend

**Contexto:** **Hipótesis:** que una medición hecha sin GPU no pase por una de GPU. La base es la
propia decisión, que dice que una medición con renderizador por software no cuenta.

**Decisión:** El motor imprime cada paso de arranque, carga y cierre. Al iniciar imprime el
adaptador gráfico y el backend elegido. Si el adaptador es llvmpipe u otro renderizador por
software, lo advierte, y una medición hecha así no cuenta como medición de GPU. La impresión es
norma en `docs/DESIGN.md`, "Presentación y registro"; la regla sobre la medición, en `CLAUDE.md`,
sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Toda cifra de rendimiento lleva sus condiciones

**Contexto:** A7 de `docs/REQUIREMENTS.md` pide más de 240 fps, y ese número todavía no tiene
escena.

**Decisión:** Toda cifra de rendimiento lleva escena, cantidad de objetos, equipo, resolución,
backend, sincronización vertical y el comando que la produjo. Los 240 fps de A7 son un objetivo
sin escena definida todavía. Cambia `CLAUDE.md`, sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Estado de fase lista para verificación

**Contexto:** Las reglas de fase de la plantilla 1.0 tenían tres estados, `pendiente`, `en curso`
y `cerrada`, y no decían quién cierra una fase.

**Decisión:** Se agrega el estado `lista para verificación`. El agente lleva una fase hasta ahí.
Solo el autor la pasa a `cerrada`, después de correr los ejemplos en los dos equipos. Cambia
`docs/ROADMAP.md`, "Reglas de fase".

**Alternativas:** Quedarse con los tres estados de la plantilla. Por qué se descartó no quedó
escrito: **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Los juegos exportados llevan los avisos de licencia

**Contexto:** Decisión de la conversación de diseño. Qué la motivó no quedó escrito: **Sin origen
recuperable.**

**Decisión:** Todo juego exportado incluye los avisos de licencia de las dependencias. Requisito
en `docs/REQUIREMENTS.md`, sección 4.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Dos equipos de prueba

**Contexto:** A4 de `docs/REQUIREMENTS.md` pide que el motor corra en una placa H618. El autor
tiene dos equipos, comprobados con neofetch el 2026-10-01.

**Decisión:** El equipo mínimo prueba todo el 2D y el 3D básico. El equipo de desarrollo prueba lo
mismo, más el 3D pesado y las mediciones.

- **Equipo de desarrollo:** Linux Mint 22 x86_64, kernel 6.8.0-138, Intel i7-7700, GPU AMD Radeon
  RX 470/480/570/580, 15945 MiB de RAM, resolución 1920x1080.
- **Equipo mínimo:** Orange Pi Zero 3, Armbian 26.11.0-trunk.46 bookworm aarch64, kernel
  6.18.51-current-sunxi64, 4 núcleos, 1973 MiB de RAM, 464 paquetes, sin sesión gráfica a la
  vista.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Cada fase cierra con un ejemplo

**Contexto:** La plantilla pide que cada fase traiga un criterio de aceptación verificable
(`docs/ROADMAP.md`, "Reglas de fase").

**Decisión:** Cada fase termina con al menos un ejemplo en `examples/` que corre con
`cargo run --example` y el nombre del ejemplo. La opción está documentada en
https://doc.rust-lang.org/cargo/commands/cargo-run.html, consultada el 2026-10-01. Cambia
`docs/ROADMAP.md`, "Reglas de fase".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Fuentes consultadas en la siembra

**Contexto:** La entrada "No se copia código de proyectos externos" pide registrar cada fuente con
URL, licencia, fecha e idea tomada, y no hay archivo de créditos.

**Decisión:** Se registran las fuentes leídas el 2026-10-01 para sembrar los documentos. Ninguna
aportó código, y de ninguna se leyó código.

- Mesa, página de Panfrost, https://docs.mesa3d.org/drivers/panfrost.html. Se tomó el soporte de
  la Mali-G31 y el estado de PanVK. Licencia de la página: no verificada.
- Mesa, página de llvmpipe, https://docs.mesa3d.org/drivers/llvmpipe.html. Se tomó que llvmpipe es
  un rasterizador por software. Licencia de la página: no verificada.
- README publicado de wgpu 30.0.1, https://docs.rs/crate/wgpu/30.0.1/source/README.md, crate con
  licencia MIT OR Apache-2.0. Se tomó la tabla "Supported Platforms".
- Referencia de Cargo, "Specifying Dependencies",
  https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html. Se tomó el operador `=`.
- Libro de Cargo, "cargo run", https://doc.rust-lang.org/cargo/commands/cargo-run.html. Se tomó
  la opción `--example`.
- Documentación de rustup, "Channels", https://rust-lang.github.io/rustup/concepts/channels.html.
  Se tomaron los canales stable, beta y nightly.
- Documentación de cargo-deny, "licenses",
  https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html. Se tomó el chequeo
  `cargo deny check licenses`.
- Manifiestos publicados en la API de crates.io, https://crates.io/api/v1/crates/ más el nombre
  del crate: la descripción de cargo-audit 0.22.2, la licencia de unicode-ident 1.0.26 y las
  dependencias de proc-macro2 1.0.107.
- Licencias de cuatro motores Rust nombrados como posible consulta, leídas en el `Cargo.toml.orig`
  y en los archivos de licencia de cada paquete publicado, https://docs.rs/crate/bevy/0.19.1/source/
  y sus equivalentes. Bevy 0.19.1 y macroquad 0.4.16: MIT OR Apache-2.0, con `LICENSE-MIT` y
  `LICENSE-APACHE`. ggez 0.10.0: MIT, con `LICENSE`. Fyrox 1.0.1: MIT en el manifiesto, sin archivo
  de licencia en el paquete. Las cuatro están en la lista admitida. Idea tomada: ninguna.

**Estado:** `vigente`
