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
advertencias, `cargo deny check` dio `advisories ok, bans ok, licenses ok, sources ok` y
`cargo audit` revisó 126 crates sin avisos. El binario aarch64 pide como máximo `GLIBC_2.34`. Nada
de esto corrió en los equipos del autor.

Con GL, el campo `driver` dijo `4.5 (Core Profile)`: Mesa le dio a wgpu OpenGL de escritorio y no
OpenGL ES. Con OpenGL ES forzado, `sh tests/gles.sh` dio sí (`docs/DECISIONS.md`, 2026-10-02 "El
nivel base también se prueba sobre OpenGL ES"). `cargo deny check` imprime además seis advertencias
`duplicate` y una `license-not-encountered`, que no son avisos de `docs/DESIGN.md`, "Seguridad de
las dependencias", y no hacen fallar el chequeo.

**Para cerrarla, en el equipo de desarrollo,** con rustup instalado, que lee `rust-toolchain.toml` y
baja Rust 1.97.0 y el objetivo aarch64 la primera vez:

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

Tiene que elegir la Radeon, sin la línea `[advertencia]`, en los dos backends. Con `--backend gl`,
el campo `driver` de la línea `adaptador elegido` dice si abrió OpenGL de escritorio u OpenGL ES;
con Mesa se espera el de escritorio, y `sh tests/gles.sh` corre las pruebas con GL sobre OpenGL ES.

`cargo deny check` puede imprimir advertencias `duplicate` y `license-not-encountered` y terminar en
`ok`: eso no es un hallazgo. `vulkaninfo` y `xrandr` cierran los huecos "Vulkan en el equipo de
desarrollo" y "Frecuencia del monitor del equipo de desarrollo".

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

Tiene que elegir la Mali con backend `Gl` y sin la línea `[advertencia]`, y el campo `driver` tiene
que decir `OpenGL ES`, que es lo que pide el criterio. Si `/dev/dri/renderD128` no deja leer al
usuario, falta agregarlo al grupo `render`. Esas corridas cierran los huecos "Panfrost en el equipo
mínimo", "glibc de bookworm" y "wgpu sobre Panfrost". El hueco "Una ventana en el equipo mínimo"
se cierra con los bloques "Para llevar el binario al equipo mínimo" y "Para cerrarla, en el equipo
mínimo" de la fase 1. Ya se pueden correr, porque el ejemplo `ventana` existe, y no hace falta
cerrar antes la fase 1.

## Fase 1: Ventana y teclado

**Estado:** `lista para verificación`

**Objetivo:** abrir una ventana y leer el teclado.

**Alcance:**
- Ventana.
- Entrada de teclado.
- El ejemplo `ventana`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example ventana` abre una ventana,
imprime cada tecla que se aprieta y termina sin error al cerrar la ventana.

**Bloquea:** fases 2, 3 y 4.

**Bloqueada por:** fase 0.

**Corrida del agente, el 2026-10-02:** en el mismo contenedor, dentro de un Xvfb, sobre llvmpipe.
`tests/ventana.sh vulkan` y `tests/ventana.sh gl` dieron sí: el ejemplo abrió la ventana, imprimió
`[entrada] tecla Code(KeyA)` y `Code(KeyB)` para las teclas que mandó xdotool, y al recibir el
pedido de cierre de un gestor de ventanas salió con código 0. `--cuadros 5` también salió con
código 0 en los dos backends.

Ninguna corrida pasó por una pantalla real, y todas fueron por X11. El camino por Wayland, el que
usa cage en el equipo mínimo, no corrió.

En el commit de la fase, el hook de pre-commit corrió `cargo fmt --check`, `cargo clippy` sin
advertencias y `cargo test`, y el de pre-push corrió `cargo deny check`, con los cuatro chequeos en
`ok`, y `cargo audit`, que revisó 247 crates sin avisos. El binario aarch64 de `ventana` pide como
máximo `GLIBC_2.34`.

**Para cerrarla, en el equipo de desarrollo:**

```sh
cargo run --example ventana -- --backend vulkan; echo $?
cargo run --example ventana -- --backend gl; echo $?
```

Con cada uno: se ve una ventana azul oscura, cada tecla apretada sale como una línea
`[entrada] tecla`, y al cerrar la ventana el comando termina sin error. Cada `echo $?` da 0.

**Para llevar el binario al equipo mínimo,** desde el equipo de desarrollo, con la variable `PI` de
la fase 0. Las fases 2 a 4 usan el mismo bloque con su ejemplo en `EJEMPLO`:

```sh
EJEMPLO=ventana
cargo build --release --target aarch64-unknown-linux-gnu --example "$EJEMPLO"
aarch64-linux-gnu-objdump -T "target/aarch64-unknown-linux-gnu/release/examples/$EJEMPLO" \
  | grep -o 'GLIBC_[0-9.]*' | sort -uV | tail -1
scp "target/aarch64-unknown-linux-gnu/release/examples/$EJEMPLO" "$PI":
```

La versión de glibc que imprime `objdump` tiene que ser 2.36 o menor.

**Para cerrarla, en el equipo mínimo,** desde la consola de la placa, con un monitor y un teclado
conectados:

```sh
sudo apt install cage libwayland-egl1
rm -f ventana.log
cage -- sh -c './ventana --backend gl --cuadros 600 > ventana.log 2>&1
  echo "código $?" >> ventana.log'
cat ventana.log
```

cage no deja cerrar la ventana a mano, así que el ejemplo termina solo tras 600 cuadros, unos 10
segundos si la pantalla va a 60 Hz. Mientras tanto se ve la ventana azul a pantalla completa y se
aprietan algunas teclas. Después, `ventana.log` tiene que terminar con `código 0` y traer una línea
`[entrada] tecla` por cada tecla. Eso cierra el hueco "Una ventana en el equipo mínimo".

El código se anota desde adentro porque cage 0.1.4, la de bookworm, sale con 0 aunque el ejemplo
falle. El registro viejo se borra antes, para que una corrida en la que cage no arranca no deje leer
el de la corrida anterior. libwayland-egl1 hace falta porque bajo cage wgpu abre la superficie GL
por Wayland con esa biblioteca, y ninguno de los otros paquetes la trae. Fuentes:
`docs/DECISIONS.md`, 2026-10-02 "Fuentes consultadas en la revisión de las fases 0 a 4".

## Fase 2: Sprite 2D

**Estado:** `lista para verificación`

**Objetivo:** dibujar un sprite 2D.

**Alcance:**
- Un sprite 2D en la ventana.
- El ejemplo `sprite`.

**Criterio de aceptación:** en los dos equipos, `cargo run --example sprite` dibuja un número fijo
de cuadros y termina con código 0, y su registro no advierte renderizador por software. Eso da sí
o no sin mirar la pantalla. Que el sprite se vea en la ventana lo comprueba el autor.

**Bloquea:** nada.

**Bloqueada por:** fase 1.

**Corrida del agente, el 2026-10-02:** en el mismo contenedor, sobre llvmpipe.
`cargo test --test sprite` dibujó el sprite en un lienzo de 64 por 64 sin ventana, con GL y con
Vulkan, y leyó los píxeles de vuelta: el fondo negro en cuatro puntos y cuatro de las 16 casillas
del damero con su color. Con el sprite corrido un cuarto de pantalla, la prueba falla.

`cargo test --test lienzo` leyó entero un lienzo de 50 por 30, cuyas filas llevan relleno, y falla
si `Lienzo::leer` no salta el relleno. `sh tests/gles.sh` corrió las dos sobre OpenGL ES 3.0 y dio
sí.

Dentro de un Xvfb, el ejemplo `sprite` con `--cuadros 30` salió con código 0 en los dos backends, y
advirtió el renderizador por software, como corresponde a llvmpipe. En el commit de la fase, el
hook de pre-commit corrió `cargo fmt --check`, `cargo clippy` sin advertencias y `cargo test`. El
binario aarch64 de `sprite` pide como máximo `GLIBC_2.34`.

**Para cerrarla, en el equipo de desarrollo:**

```sh
cargo test --test sprite
cargo run --example sprite -- --backend vulkan --cuadros 300; echo $?
cargo run --example sprite -- --backend gl --cuadros 300; echo $?
```

Cada `echo $?` tiene que dar 0, sin líneas `[advertencia]`. A la vista: un cuadrado con damero
naranja y crema en el centro de una ventana azul oscura.

**Para cerrarla, en el equipo mínimo,** con el binario llevado como en la fase 1, con
`EJEMPLO=sprite`:

```sh
rm -f sprite.log
cage -- sh -c './sprite --backend gl --cuadros 300 > sprite.log 2>&1
  echo "código $?" >> sprite.log'
grep código sprite.log; grep -c advertencia sprite.log
```

El primer `grep` tiene que dar `código 0` y el segundo, 0. A la vista, el mismo damero.

## Fase 3: Cubo 3D con cámara

**Estado:** `lista para verificación`

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

**Corrida del agente, el 2026-10-02:** en el mismo contenedor, sobre llvmpipe.
`cargo test --test cubo` dibujó el cubo en un lienzo de 64 por 64 con GL y con Vulkan y leyó los
píxeles: de frente se ve la cara +z, desde la derecha la +x, tras cuatro pasos con R el cubo queda
abajo con la cara +y encima, y tras cuatro pasos con D el centro queda vacío y el cubo a la
izquierda. Sin la prueba de profundidad, o con la perspectiva dada vuelta en vertical, la prueba
falla. `sh tests/gles.sh` la corrió sobre OpenGL ES 3.0 y dio sí.

Dentro de un Xvfb, el ejemplo `cubo` con `--cuadros 30` salió con código 0 en los dos backends. En
otra corrida, con `--cuadros 600`, las teclas W, D, D y R que mandó xdotool movieron la cámara un
paso de 0,25 cada una, y salió con código 0. En el commit de la fase, el hook de pre-commit corrió
`cargo fmt --check`, `cargo clippy` sin advertencias y `cargo test`. El binario aarch64 de `cubo`
pide como máximo `GLIBC_2.34`.

**Para cerrarla, en el equipo de desarrollo:**

```sh
cargo test --test cubo
cargo run --example cubo -- --backend vulkan --cuadros 300; echo $?
cargo run --example cubo -- --backend gl --cuadros 300; echo $?
cargo run --example cubo -- --backend vulkan; echo $?
cargo run --example cubo -- --backend gl; echo $?
```

Las dos primeras corridas terminan solas: cada `echo $?` tiene que dar 0, sin líneas
`[advertencia]`. En las dos últimas se ve el cubo con tres caras de colores distintos. W, A, S, D,
las flechas, R y F mueven la cámara, y cada paso sale como una línea `[resultado] cámara en`. Al
cerrar la ventana, `echo $?` da 0.

La matemática de la cámara y del cubo se escribió en el motor y no con un crate del escalón 1.
Antes de pasar la fase a `cerrada`, el autor confirma o reemplaza la decisión 2026-10-02 "La
matemática del cubo y la cámara se escribe en el motor".

**Para cerrarla, en el equipo mínimo,** con el binario llevado como en la fase 1, con
`EJEMPLO=cubo`:

```sh
rm -f cubo.log
cage -- sh -c './cubo --backend gl --cuadros 600 > cubo.log 2>&1
  echo "código $?" >> cubo.log'
grep código cubo.log; grep -c advertencia cubo.log; grep 'cámara en' cubo.log
```

Mientras corre se aprietan algunas teclas de movimiento. El primer `grep` tiene que dar `código 0`,
el segundo 0, y el tercero una línea `[resultado] cámara en` por cada tecla.

## Fase 4: Gravedad sobre el cubo

**Estado:** `lista para verificación`

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

**Bloqueada por:** fase 3. La gravedad usa un integrador propio, a confirmar por el autor
(`docs/DECISIONS.md`, 2026-10-02 "La gravedad del cubo usa un integrador propio").

**Corrida del agente, el 2026-10-02:** en el mismo contenedor, sobre llvmpipe.
`cargo test --test caida` simuló la caída sin GPU con cuadros de 1/30 s, de 1/240 s y de largos
irregulares: los tres dieron la misma bajada, bit a bit, y a 60 Hz se aleja 1,67 % de 4,905 m.

2 s de tiempo real dieron 120 pasos con cuadros de 1/240 s y 119 con cuadros de 1/30 s, porque
los cuadros, redondeados a nanosegundos, no suman justo 2 s; el comando de abajo da las sumas y lo
que piden 120 pasos. Con un paso por cuadro, esa prueba falla. Sin el tope de pasos, falla la de 30
contra 240.

```sh
python3 -c "r=lambda s: round(s*1e9); print(60*r(1/30), 480*r(1/240), 120*r(1/60))"
```

La misma suite dibujó el cubo en un lienzo, con GL y con Vulkan, a la altura inicial y tras 1 s
simulado, y lo encontró arriba y después abajo. `sh tests/gles.sh` corrió esa parte sobre OpenGL ES
3.0 y dio sí.

Dentro de un Xvfb, `tests/caida.sh` corrió el ejemplo con tope de 30 y de 240 cuadros por segundo,
en GL y en Vulkan. Las cuatro corridas imprimieron `bajó 4.986750 m en 60 pasos` y salieron con
código 0, que el script comprueba. En el commit de la fase, el hook de pre-commit corrió
`cargo fmt --check`, `cargo clippy` sin advertencias y `cargo test`, y el de pre-push corrió
`cargo deny check`, con los cuatro chequeos en `ok`, y `cargo audit`, que revisó 247 crates sin
avisos. El binario aarch64 de `caida` pide como máximo `GLIBC_2.34`.

Con GL, la superficie de wgpu 30.0.1 solo ofrece la presentación `Fifo`, y el ejemplo lo avisa en
una línea `[arranque]`. En una pantalla real se espera que el tope de 240 con GL quede en la
frecuencia del monitor. No verificado: ninguna corrida pasó por una pantalla real, wgpu 30.0.1 no
fija el intervalo de intercambio de EGL, y en Xvfb `Fifo` no esperó. Aun así, la comparación sigue
siendo entre dos ritmos de dibujo distintos.

**Para cerrarla, en el equipo de desarrollo:**

```sh
cargo test --test caida
cargo run --release --example caida -- --backend vulkan --limite-fps 30; echo $?
cargo run --release --example caida -- --backend vulkan --limite-fps 240; echo $?
cargo run --release --example caida -- --backend gl --limite-fps 240; echo $?
```

Las tres corridas tienen que imprimir la misma línea `[resultado] bajó`, y cada `echo $?` tiene
que dar 0. A la vista, el cubo cae y la ventana se cierra sola tras 1 s simulado. Antes de pasar la
fase a `cerrada`, el autor confirma o reemplaza la decisión 2026-10-02 "La gravedad del cubo usa un
integrador propio".

**Para cerrarla, en el equipo mínimo,** con el binario llevado como en la fase 1, con
`EJEMPLO=caida`:

```sh
rm -f caida-30.log caida-240.log
cage -- sh -c './caida --backend gl --limite-fps 30 > caida-30.log 2>&1
  echo "código $?" >> caida-30.log'
cage -- sh -c './caida --backend gl --limite-fps 240 > caida-240.log 2>&1
  echo "código $?" >> caida-240.log'
grep -h código caida-30.log caida-240.log; grep -h bajó caida-30.log caida-240.log
```

El primer `grep` tiene que dar dos veces `código 0`. El segundo, dos líneas con la misma cifra, y
la misma que en el equipo de desarrollo. Ahí se espera que el tope de 240 quede en la frecuencia
del monitor, porque el equipo mínimo dibuja con GL. No verificado.

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
