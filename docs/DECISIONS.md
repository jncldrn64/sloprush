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

## 2026-10-01: La licencia del motor es AGPL-3.0, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "La licencia del motor es AGPL-3.0", que no traía el motivo.
El autor lo dio en la conversación de diseño:

- "Lo que salga de este repo no quiero que termine en ciertas manos, aunque realmente la licencia
  es nada sin nadie que la proteja."

**Decisión:** El motor se licencia bajo AGPL-3.0, con el `LICENSE` del commit inicial `1448ea3`.
Si la AGPL-3.0 sirve a ese fin queda como pregunta abierta en `docs/REQUIREMENTS.md`, "Sin escribir
todavía", junto a la de qué obliga a quien exporta un juego.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: El lenguaje es Rust estable, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "El lenguaje es Rust estable", que no traía el motivo. El
lenguaje lo pidió el autor:

- "Dicho motor gráfico debe estar enteramente escrito desde 0 en Rust."

**Decisión:** El motor se escribe en Rust estable, sin nightly. El canal estable fue propuesta de
la conversación de diseño, aceptada por el autor, para que el código compile igual en los dos
equipos y no dependa de funciones que cambian. Norma en `docs/DESIGN.md`, "Rust estable".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Dos niveles gráficos definidos por la tabla de wgpu

**Contexto:** Reemplaza a 2026-10-01 "Dos niveles gráficos", cuya lista de backends dejaba sin
clasificar a DirectX 12 en Windows y a WebGPU en el navegador. La regla nueva fue propuesta de la
conversación de diseño, aceptada por el autor.

**Decisión:** El nivel base son los backends que la tabla "Supported Platforms" de wgpu marca como
"Downlevel/Best Effort Support", la familia OpenGL. El nivel completo son los que marca como "First
Class Support". Toda capacidad funciona en el base, y el completo solo agrega opciones visuales o de
rendimiento. Con la misma entrada, los dos llegan al mismo estado de juego. Norma en
`docs/DESIGN.md`, "Dos niveles gráficos".

**Alternativas:** Nombrar los backends uno por uno, descartado porque dejaba backends sin
nivel. Un piso único, descartado en la entrada reemplazada porque dejaba sin técnicas de
rendimiento al nivel alto.

**Estado:** `vigente`

## 2026-10-01: La simulación avanza a paso fijo de frecuencia configurable

**Contexto:** Reemplaza a 2026-10-01 "La simulación avanza a paso fijo de 60 Hz". El autor pidió
poder cambiar la frecuencia:

- "Me gustaría que el motor se pudiera actualizar a placer, en caso que llegue un multijugador y se
  pueda delegar esa carga al servidor y poner 144 Hz para una detección perfecta."

La forma de abajo es propuesta de la conversación de diseño, aceptada por el autor. Más frecuencia
reduce las colisiones perdidas y no las elimina, así que el multijugador con servidor y la
detección continua de colisiones van al Backlog de `docs/ROADMAP.md`.

**Decisión:** La simulación avanza a paso fijo, separada del dibujo. La frecuencia del paso es
configurable, vale 60 Hz por defecto, no cambia dentro de una partida y todos los participantes
usan la misma. El paso es fijo para que el mismo juego dé el mismo resultado en el equipo mínimo y
en el de desarrollo, sea cual sea la tasa de dibujo. Norma y prueba en `docs/DESIGN.md`, "La
simulación avanza a paso fijo de frecuencia configurable".

**Alternativas:** El paso fijo de 60 Hz de la entrada reemplazada, que no dejaba subir la
frecuencia. Su prueba también estaba mal planteada: esperaba 4,905 m, la caída de la fórmula
continua, y un integrador por pasos se aleja de esa cifra en 1/n con n pasos por segundo.

**Estado:** `vigente`

## 2026-10-01: Nativo contra script, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Nativo contra script", que no traía el motivo. Sale de A11
de `docs/REQUIREMENTS.md`, y la forma fue propuesta de la conversación de diseño, aceptada por el
autor.

**Decisión:** Es nativo lo que toca GPU, sistema operativo o hardware, o corre en cada cuadro sobre
muchos objetos. Lo demás se intenta primero como script, para que lo nativo crezca solo con una
medición registrada. Norma en `docs/DESIGN.md`, "Nativo contra script".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Primero el mínimo viable, después el catálogo de bloques, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Primero el mínimo viable, después el catálogo de bloques",
que no traía el motivo. El autor lo dijo así:

- "Podremos dejar que decida en las primeras instancias hasta que tenga un mínimo viable."

**Decisión:** Primero se hace un mínimo viable, donde el agente decide la estructura interna. El
catálogo de bloques se extrae después de lo construido, y lo congela el autor. El orden está en
`docs/ROADMAP.md`, "Principio de orden".

**Alternativas:** Especificar el catálogo antes de tener código. El autor lo descartó con esta
frase:

- "Pedirle desde ya el techo, me frena."

**Estado:** `vigente`

## 2026-10-01: Alcance del mínimo viable, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Alcance del mínimo viable", que no traía su origen. El
alcance fue propuesta de la conversación de diseño, y el autor lo aceptó:

- "Bastante simple la verdad."

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

## 2026-10-01: La API de bloques es inestable hasta que el autor la congele, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "La API de bloques es inestable hasta que el autor la
congele", que no traía el motivo. Propuesta de la conversación de diseño, aceptada por el autor.

**Decisión:** La API de bloques es inestable hasta que el autor la congele por escrito. El
catálogo se extrae después del mínimo viable, y congelar la API antes obligaría a rehacerla.
Mientras el módulo no exista, no tiene carpeta propia en `docs/`. Por eso se borra `docs/modulo/`,
la carpeta de la plantilla que contradecía esta regla.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Escalera de licencias para las dependencias

**Contexto:** Reemplaza a 2026-10-01 "Licencias admitidas en las dependencias". El criterio es del
autor:

- "En cuanto a las dependencias sí me importa, no por la licencia en sí, sino por ver si el modelo
  no encuentra nada, que lo tenga que implementar desde 0 si hace falta... es decir licencias
  similares a la MIT sí o sí."
- "Incluir otros tipos de licencias si no se encuentra nada en las MIT y la otra mencionada, siendo
  última opción las AGPL, Mozilla y demás, suponiendo que sea muy costosa la implementación."

La forma operativa de abajo es propuesta de la conversación de diseño, aceptada por el autor.

**Decisión:** Una dependencia se busca en esta escalera, de arriba hacia abajo:

1. Licencias permisivas: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, Zlib, ISC y Unicode-3.0.
   Entran sin autorización.
2. Implementar desde cero, si cabe en la fase.
3. Copyleft débil: MPL-2.0 y LGPL-3.0. Solo con autorización del autor, crate por crate.
4. GPL-3.0 y AGPL-3.0. Igual que el escalón 3, y solo después de él.

Nunca entran GPL-2.0 sin la cláusula "o posterior", código sin licencia, licencias propietarias ni
licencias no comerciales. Para bajar del escalón 2, el agente se detiene y presenta al autor lo que
`CLAUDE.md`, sección 8, pide cuando un umbral se dispara. Cada excepción es una entrada de este
archivo y una excepción por crate en la configuración de cargo deny. Norma en `docs/DESIGN.md`,
"Licencias de las dependencias".

**Alternativas:** La lista cerrada de seis licencias de la entrada reemplazada. No tenía camino
para cuando no hay crate permisivo, y rechazaba a unicode-ident 1.0.26, del que depende
proc-macro2, porque declara `(MIT OR Apache-2.0) AND Unicode-3.0`. La OSI aprobó Unicode-3.0 el
2023-11-17.

**Estado:** `vigente`

## 2026-10-01: Seguridad de las dependencias

**Contexto:** El autor dio el motivo de su criterio de antigüedad:

- "Tampoco quiero obtener algo sin soporte que tenga una CVE, por eso mi límite tan raro en años."

Su criterio pedía 1 año de antigüedad y 1 actualización en el último mes. Lo segundo excluía a
wgpu, cuyo último release, 30.0.1, es del 2026-08-22. La regla de abajo es propuesta de la
conversación de diseño, y el autor la aceptó:

- "podemos poner salvaguardas a los tipos de vulnerabilidades"

**Decisión:** Cuatro reglas para las dependencias:

1. Cero avisos en todo el árbol: vulnerabilidades, crates declarados sin mantenimiento y versiones
   retiradas.
2. Solo crates del registro crates.io, nunca desde un repositorio git.
3. Cada dependencia directa tiene su primer release hace 1 año o más, y su último release dentro de
   los últimos 12 meses.
4. Un aviso sin arreglo sobre un crate ya incorporado no se silencia: va a los huecos conocidos de
   `docs/ARCHITECTURE.md` y decide el autor.

Los comandos que las comprueban están en `docs/DESIGN.md`, "Seguridad de las dependencias".

**Alternativas:** El criterio original, con 1 actualización en el último mes. Se descartó porque
excluía a wgpu, y porque un crate maduro puede pasar meses sin cambios.

**Estado:** `vigente`

## 2026-10-01: Versiones exactas y documentación de la versión, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Versiones exactas y documentación de la versión", que no
traía el motivo. Propuesta de la conversación de diseño, aceptada por el autor: wgpu cambia su API
entre versiones, y un modelo que escribe la llamada de memoria mezcla versiones.

**Decisión:** Cada crate lleva su versión exacta en `Cargo.toml`, `Cargo.lock` está versionado, y
antes de usar una API se lee la documentación de esa versión. Las dos primeras son norma del código
en `docs/DESIGN.md`, "Versión exacta de cada dependencia". La lectura es regla de método, en
`CLAUDE.md`, sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: No se copia código de proyectos externos, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "No se copia código de proyectos externos", que no traía su
origen. La idea es del autor:

- "Que únicamente se inspire de proyectos MIT."
- "Daríamos crédito."

La forma es propuesta de la conversación de diseño, aceptada por el autor. Las ideas no se protegen
y la forma escrita sí, así que consultar es libre y traducir un archivo es hacer una adaptación.

**Decisión:** No se copia ni se traduce código de proyectos externos. Se pueden consultar proyectos
con una licencia del escalón 1 de "Escalera de licencias para las dependencias", y cada consulta se
registra como entrada de este archivo con URL, licencia, fecha e idea tomada. No se abre código
GPL, LGPL o AGPL, ni descompilaciones de juegos. Esa prohibición se escribió cuando la licencia del
motor estaba sin decidir, y el autor puede revisarla. Cambia `CLAUDE.md`, sección 10.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: El motor imprime su arranque, su adaptador y su backend, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "El motor imprime su arranque, su adaptador y su backend",
cuyo contexto era una hipótesis. Propuesta de la conversación de diseño, aceptada por el autor, que
confirma esa hipótesis: sin driver de GPU, el motor corre sobre llvmpipe sin avisar, y se mediría
la CPU creyendo medir la GPU.

**Decisión:** El motor imprime cada paso de arranque, carga y cierre. Al iniciar imprime el
adaptador gráfico y el backend elegido. Si el adaptador es llvmpipe u otro renderizador por
software, lo advierte, y una medición hecha así no cuenta como medición de GPU. La impresión es
norma en `docs/DESIGN.md`, "Presentación y registro"; la regla sobre la medición, en `CLAUDE.md`,
sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Toda cifra de rendimiento lleva sus condiciones, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Toda cifra de rendimiento lleva sus condiciones", que no
traía el motivo. Propuesta de la conversación de diseño, aceptada por el autor. 240 fps son 4,17 ms
por cuadro, lo recalcula `python3 -c "print(1000/240)"`, y ese margen depende de la escena y del
equipo.

**Decisión:** Toda cifra de rendimiento lleva escena, cantidad de objetos, equipo, resolución,
backend, sincronización vertical y el comando que la produjo. Los 240 fps de A7 son un objetivo
sin escena definida todavía. Cambia `CLAUDE.md`, sección 8.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Estado de fase lista para verificación, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Estado de fase lista para verificación", que no traía el
motivo. Propuesta de la conversación de diseño, aceptada por el autor. El agente no ve la pantalla
de ninguno de los dos equipos, y `CLAUDE.md`, sección 8, prohíbe declarar algo probado sin una
corrida.

**Decisión:** Se agrega el estado `lista para verificación`. El agente lleva una fase hasta ahí.
Solo el autor la pasa a `cerrada`, después de correr los ejemplos en los dos equipos. Cambia
`docs/ROADMAP.md`, "Reglas de fase".

**Alternativas:** Quedarse con los tres estados de la plantilla. Se descartó por el motivo del
contexto.

**Estado:** `vigente`

## 2026-10-01: Los juegos exportados llevan los avisos de licencia, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Los juegos exportados llevan los avisos de licencia", que no
traía el motivo. Propuesta de la conversación de diseño, aceptada por el autor. Las licencias
permisivas exigen conservar su aviso en cada copia, y un juego exportado lleva los crates adentro.

**Decisión:** Todo juego exportado incluye los avisos de licencia de las dependencias. Requisito
en `docs/REQUIREMENTS.md`, sección 4.

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Dos equipos de prueba, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Dos equipos de prueba", que no traía el motivo. El autor
repartió los equipos así:

- "Ese podría ser el mínimo para el videojuego en 2D, y el SUS para el 3D."

La conversación de diseño propuso un ajuste, y el autor lo aceptó. El 3D básico también corre en el
equipo mínimo, porque su piso gráfico es casi el del navegador y sirve de sustituto barato.

**Decisión:** El equipo mínimo prueba todo el 2D y el 3D básico. El equipo de desarrollo prueba lo
mismo, más el 3D pesado y las mediciones.

- **Equipo de desarrollo:** Linux Mint 22 x86_64, kernel 6.8.0-138, Intel i7-7700, GPU AMD Radeon
  RX 470/480/570/580, 15945 MiB de RAM, resolución 1920x1080.
- **Equipo mínimo:** Orange Pi Zero 3, Armbian 26.11.0-trunk.46 bookworm aarch64, kernel
  6.18.51-current-sunxi64, 4 núcleos, 1973 MiB de RAM, 464 paquetes, sin sesión gráfica a la
  vista.

**Alternativas:** El reparto tal como lo dijo el autor, con el equipo mínimo solo para el 2D. Se
ajustó por el motivo del contexto.

**Estado:** `vigente`

## 2026-10-01: Cada fase cierra con un ejemplo, con su motivo

**Contexto:** Reemplaza a 2026-10-01 "Cada fase cierra con un ejemplo", que no traía el motivo.
Propuesta de la conversación de diseño, aceptada por el autor: un ejemplo que corre es el criterio
de terminado, para no rehacer.

**Decisión:** Cada fase termina con al menos un ejemplo en `examples/` que corre con
`cargo run --example` y el nombre del ejemplo. La opción está documentada en
https://doc.rust-lang.org/cargo/commands/cargo-run.html, consultada el 2026-10-01. Cambia
`docs/ROADMAP.md`, "Reglas de fase".

**Alternativas:** **Sin origen recuperable.**

**Estado:** `vigente`

## 2026-10-01: Fuentes consultadas en la corrección de la siembra

**Contexto:** La entrada "No se copia código de proyectos externos, con su motivo" pide registrar
cada fuente con URL, licencia, fecha e idea tomada.

**Decisión:** Se registran las fuentes leídas el 2026-10-01 para esta corrección. Ninguna aportó
código, y de ninguna se leyó código.

- API de crates.io, https://crates.io/api/v1/crates/ más el nombre del crate. Se tomaron la
  licencia de unicode-ident 1.0.26, `(MIT OR Apache-2.0) AND Unicode-3.0`; las fechas de wgpu,
  primer release 2019-01-24 y último, 30.0.1, el 2026-08-22; y rapier3d 0.36.0, Apache-2.0, primer
  release 2020-08-19 y último el 2026-09-25. Licencia de los datos: no verificada.
- OSI, https://opensource.org/license/unicode-3-0. Se tomó que Unicode-3.0 está aprobada desde el
  2023-11-17, en la categoría "Special Purpose". Licencia de la página: no verificada.
- README publicado de wgpu 30.0.1, https://docs.rs/crate/wgpu/30.0.1/source/README.md, leído de
  nuevo. Se tomó la tabla "Supported Platforms".
- Documentación de cargo-deny, https://embarkstudios.github.io/cargo-deny/, páginas de los chequeos
  advisories, sources y licenses y de su configuración. Se tomaron los comandos
  `cargo deny check advisories`, `cargo deny check sources` y `cargo deny check licenses`, y los
  campos `yanked`, `unmaintained`, `unknown-git`, `unknown-registry` y `exceptions`. Licencia de
  la página: no verificada.
- README publicado de cargo-audit 0.22.2, https://docs.rs/crate/cargo-audit/0.22.2/source/README.md.
  Se tomó que audita vulnerabilidades de la RustSec Advisory Database. No documenta cómo trata los
  crates sin mantenimiento ni las versiones retiradas.

**Estado:** `vigente`
