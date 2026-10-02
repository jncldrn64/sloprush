# DESIGN.md: cómo se escribe el código

> **Rol:** normativo sobre el código. Cada principio dice cómo se escribe el código y por qué.
> Manda sobre el código; `docs/REQUIREMENTS.md` manda sobre él. Describe reglas; el
> estado, qué rutas, recursos o funciones existen hoy, va en `docs/ARCHITECTURE.md`.
> **Régimen:** se corrige, y un principio nuevo o cambiado lleva su decisión. **Origen:** plantilla
> 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla 1.0").

Un cambio que rompe un principio de acá se rechaza, lo haya escrito una persona o un modelo. Ante la
duda, se copia la forma del código que ya existe en vez de inventar un idioma nuevo.

Cada principio dice cómo se comprueba, y lo que todavía no se puede comprobar está en la sección 11.
Qué código existe está en `docs/ARCHITECTURE.md`.

## 1. Rust estable

**Regla:** el código compila con el canal estable de Rust.

**Por qué:** el lenguaje lo pidió el autor. El canal estable fue propuesta de la conversación de
diseño, aceptada por el autor, para que el código compile igual en los dos equipos y no dependa de
funciones que cambian. Decisión: `docs/DECISIONS.md`, 2026-10-01 "El lenguaje es Rust estable, con
su motivo".

**Dónde está escrito:** los canales stable, beta y nightly están en la documentación de rustup,
https://rust-lang.github.io/rustup/concepts/channels.html, consultada el 2026-10-01.

**Cómo se comprueba:** `rust-toolchain.toml` fija `channel = "1.97.0"`, y dentro del repo
`rustc --version` da 1.97.0. Corrido el 2026-10-02 en el contenedor de la sesión.

## 2. Toda la salida gráfica pasa por wgpu

**Regla:** todo dibujo pasa por el crate wgpu. Ningún módulo llama directo a Vulkan, OpenGL,
Metal o DirectX.

**Por qué:** usar solo Vulkan con capas de compatibilidad se descartó, porque Vulkan no existe en
el navegador y en la GPU del equipo mínimo su driver libre no es conformante. Decisión:
`docs/DECISIONS.md`, 2026-10-01 "Toda la salida gráfica pasa por wgpu".

**Dónde está escrito:** la tabla "Supported Platforms" del README de wgpu 30.0.1,
https://docs.rs/crate/wgpu/30.0.1/source/README.md, consultada el 2026-10-01.

**Cómo se comprueba:** `cargo tree -e normal --depth 1` lista las dependencias directas, y ninguna
puede ser una API gráfica como `ash`, `glow`, `khronos-egl`, `metal` o `windows`. Corrido el
2026-10-02: lista pollster, wgpu y winit.

## 3. Dos niveles gráficos

**Regla:** el nivel base son los backends que la tabla "Supported Platforms" de wgpu marca como
"Downlevel/Best Effort Support", y el nivel completo, los que marca como "First Class Support".
Toda capacidad del motor funciona en el nivel base. El completo solo agrega opciones visuales o de
rendimiento, y con la misma entrada llega al mismo estado de juego que el base.

**Por qué:** es el medio de A5 de `docs/REQUIREMENTS.md`. Definir los niveles por la marca de la
tabla, y no por una lista de nombres, fue propuesta de la conversación de diseño, aceptada por el
autor. Decisión: `docs/DECISIONS.md`, 2026-10-01 "Dos niveles gráficos definidos por la tabla de
wgpu".

**Dónde está escrito:** la tabla de wgpu 30.0.1, leída de nuevo el 2026-10-01. Con esa tabla:

- nivel base: OpenGL 3.3+ en Windows, OpenGL ES 3.0+ en Linux y Android, WebGL2 en la web;
- nivel completo: Vulkan en Windows y Linux, Metal en macOS, DirectX 12 en Windows, WebGPU en la
  web;
- sin marca de nivel: OpenGL en macOS, que necesita la capa ANGLE, y Vulkan en macOS, que necesita
  MoltenVK (sección 11).

**Cómo se comprueba:** se corre la misma escena con la misma entrada en cada nivel y se compara
el resultado. `cargo test` corre las pruebas de arranque y de dibujo con GL y con Vulkan, y
`tests/caida.sh`, sin argumento, compara el estado de juego entre los dos niveles: lo que baja el
cubo tiene que ser la misma cifra. Con Mesa, el backend GL de wgpu 30.0.1 abre OpenGL de
escritorio, así que `tests/gles.sh` corre las pruebas con GL otra vez sobre OpenGL ES 3.0
(`docs/DECISIONS.md`, 2026-10-02 "El nivel base también se prueba sobre OpenGL ES"). Las tres
dieron sí el 2026-10-02 sobre llvmpipe.

## 4. La simulación avanza a paso fijo de frecuencia configurable

**Regla:** la simulación avanza en pasos fijos, separada del dibujo, y cuántos pasos da no depende
de cuántos cuadros se dibujan. La frecuencia del paso es configurable y vale 60 Hz por defecto.
Dentro de una partida no cambia, y todos los participantes usan la misma.

**Por qué:** que el mismo juego dé el mismo resultado en el equipo mínimo y en el de desarrollo,
sea cual sea la tasa de dibujo. La frecuencia configurable la pidió el autor, pensando en un
multijugador con servidor a 144 Hz. La regla fue propuesta de la conversación de diseño, aceptada
por el autor. Decisión: `docs/DECISIONS.md`, 2026-10-01 "La simulación avanza a paso fijo de
frecuencia configurable".

**Cómo se comprueba:** un cuerpo se suelta en reposo con gravedad 9,81 m/s² y cae durante 1 s de
simulación. La prueba pide dos cosas:

- con el dibujo limitado a 30 y a 240 cuadros por segundo, lo que baja es exactamente igual;
- a 60 Hz, lo que baja se aleja menos de 2 % de 4,905 m, la caída de la fórmula continua.

Con n pasos por segundo, un integrador de Euler se aleja de 4,905 m en una fracción 1/n, que a 60 Hz
es 1,67 %. El comando de abajo da 4,98675 m con Euler semiimplícito y 4,82325 m con Euler explícito.
`cargo test --test caida` comprueba las dos cosas sin GPU, y `tests/caida.sh` comprueba la igualdad
con la ventana, en GL y en Vulkan. La misma suite comprueba además que 2 s de tiempo real den los
mismos pasos con cuadros de 1/30 s y de 1/240 s, con un paso de diferencia como mucho, porque los
cuadros, redondeados a nanosegundos, no suman justo 2 s. Las dos dieron sí el 2026-10-02 sobre
llvmpipe.

```sh
python3 -c "g,h,n=9.81,1/60,60; print(g*h*h*n*(n+1)/2, g*h*h*n*(n-1)/2)"
```

## 5. Nativo contra script

**Regla:** es nativo lo que toca GPU, sistema operativo o hardware, o corre en cada cuadro sobre
muchos objetos. Lo demás se intenta primero como script, y pasa a nativo solo con una medición
registrada.

**Por qué:** A11 de `docs/REQUIREMENTS.md` pide ver qué es parte nativa del motor y qué puede ser
una función documentada de script. Lo de intentar primero como script fue propuesta de la
conversación de diseño, aceptada por el autor, para que lo nativo crezca solo con una medición.
Decisión: `docs/DECISIONS.md`, 2026-10-01 "Nativo contra script, con su motivo".

**Cómo se comprueba:** hoy no se puede. No hay lenguaje de script, así que todo el mínimo viable es
nativo, y "muchos objetos" no tiene número (sección 11).

## 6. Licencias de las dependencias

**Regla:** toda dependencia, directa o transitiva, tiene una licencia del escalón 1, o una excepción
que el autor autorizó para ese crate. Una dependencia se busca en esta escalera, de arriba hacia
abajo:

1. Permisivas: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, Zlib, ISC y Unicode-3.0. Entran sin
   autorización.
2. Implementar desde cero, si cabe en la fase.
3. Copyleft débil: MPL-2.0 y LGPL-3.0. Solo con autorización del autor, crate por crate.
4. GPL-3.0 y AGPL-3.0. Igual que el escalón 3, y solo después de él.

Nunca entran GPL-2.0 sin la cláusula "o posterior", código sin licencia, licencias propietarias ni
licencias no comerciales. Para bajar del escalón 2, el agente se detiene y presenta al autor lo que
`CLAUDE.md`, sección 8, pide cuando un umbral se dispara.

La matemática del motor, la de la cámara y el cubo, se escribe en el motor aunque haya un crate del
escalón 1 que la traiga (`docs/DECISIONS.md`, 2026-10-02 "La matemática del cubo y la cámara se
escribe en el motor"). Queda a confirmar por el autor.

**Por qué:** el criterio es del autor, que quiere licencias parecidas a la MIT y deja las demás como
última opción. La escalera fue propuesta de la conversación de diseño, aceptada por el autor.
Decisión: `docs/DECISIONS.md`, 2026-10-01 "Escalera de licencias para las dependencias".

**Dónde está escrito:** Unicode-3.0 figura aprobada por la OSI desde el 2023-11-17,
https://opensource.org/license/unicode-3-0, consultada el 2026-10-01. Las listas `allow` y
`exceptions` de cargo-deny están en
https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html, consultada el 2026-10-01.

**Cómo se comprueba:** `cargo deny check licenses`, con el escalón 1 en `allow` y cada excepción
de los escalones 3 y 4 en `exceptions`, una por crate y con su entrada en `docs/DECISIONS.md`. El
árbol que revisa es el de los objetivos `x86_64-unknown-linux-gnu` y `aarch64-unknown-linux-gnu`
(`docs/DECISIONS.md`, 2026-10-02 "cargo-deny revisa los dos objetivos Linux"). Corrido el
2026-10-02, con `exceptions` vacío: `licenses ok`.

## 7. Versión exacta de cada dependencia

**Regla:** cada dependencia de `Cargo.toml` lleva su versión exacta, con el operador `=`, y
`Cargo.lock` está versionado.

**Por qué:** wgpu cambia su API entre versiones, y un modelo que escribe la llamada de memoria
mezcla versiones. Propuesta de la conversación de diseño, aceptada por el autor. Decisión:
`docs/DECISIONS.md`, 2026-10-01 "Versiones exactas y documentación de la versión, con su motivo".

**Dónde está escrito:** la referencia de Cargo, "Specifying Dependencies",
https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html, consultada el 2026-10-01.
Ahí `"1.2.3"` sin operador equivale a `^1.2.3`, que admite versiones compatibles, y `= 1.2.3` es la
versión exacta.

**Cómo se comprueba:** `git ls-files Cargo.lock` tiene que listarlo, y este comando no tiene que
imprimir nada, porque muestra cada dependencia sin `=`. Corridos los dos el 2026-10-02.

```sh
awk '/^\[dependencies\]/{d=1;next} /^\[/{d=0} d && /=/ && !/"=/' Cargo.toml
```

## 8. Seguridad de las dependencias

**Regla:**

1. Cero avisos en todo el árbol: vulnerabilidades, crates declarados sin mantenimiento y versiones
   retiradas. El árbol es el de los objetivos Linux que se compilan (`docs/DECISIONS.md`,
   2026-10-02 "cargo-deny revisa los dos objetivos Linux").
2. Solo crates del registro crates.io, nunca desde un repositorio git.
3. Cada dependencia directa tiene su primer release hace 1 año o más, y su último release dentro de
   los últimos 12 meses.
4. Un aviso sin arreglo sobre un crate ya incorporado no se silencia: va a los huecos conocidos de
   `docs/ARCHITECTURE.md` y decide el autor.

**Por qué:** que no entre un crate sin soporte con una vulnerabilidad conocida, que es el motivo del
autor. La regla fue propuesta de la conversación de diseño, aceptada por el autor. Decisión:
`docs/DECISIONS.md`, 2026-10-01 "Seguridad de las dependencias".

**Dónde está escrito:** la documentación de cargo-deny, consultada el 2026-10-01.
https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html dice que toda vulnerabilidad
da error, que `unmaintained` vale `"all"` por defecto y que `yanked` vale `"warn"` por defecto.
https://embarkstudios.github.io/cargo-deny/checks/sources/cfg.html dice que `allow-registry` trae
solo crates.io por defecto, y que `unknown-git` y `unknown-registry` valen `"warn"` por defecto.

**Cómo se comprueba:**

- Regla 1: `cargo deny check advisories`, con `yanked = "deny"` en `[advisories]`.
- Regla 2: `cargo deny check sources`, con `unknown-git = "deny"` y `unknown-registry = "deny"` en
  `[sources]`.
- Regla 3: por cada dependencia directa, la API de crates.io da la primera y la última fecha con
  el comando de abajo, que el 2026-10-01 dio 2019-01-24 y 2026-08-22 para wgpu.
- Regla 4: el campo `ignore` de `[advisories]` queda vacío salvo decisión del autor.

```sh
curl -sS -A sloprush https://crates.io/api/v1/crates/wgpu | python3 -c '
import json, sys
v = json.load(sys.stdin)["versions"]
print(min(x["created_at"] for x in v)[:10], max(x["created_at"] for x in v)[:10])'
```

Corridas el 2026-10-02 sobre el `Cargo.lock` de la fase 0: `cargo deny check` dio `advisories ok`
y `sources ok`, con `ignore` vacío, y la regla 3 dio 2020-04-07 y 2026-07-10 para pollster.

## 9. Los tests

Cómo se escriben los tests está en la sección 11. Una suite que alguna vez falló de forma
intermitente se corre N veces, no una, y se queda en esa lista después del arreglo. Qué suites hay
y cómo se corren está en `tests/README.md`.

## 10. Presentación y registro

**Regla:** el motor imprime cada paso de arranque, carga y cierre, y al iniciar, el adaptador
gráfico y el backend elegido. Si el adaptador es llvmpipe u otro renderizador por software, lo
advierte.

**Por qué:** sin driver de GPU, el motor corre sobre llvmpipe sin avisar, y se mediría la CPU
creyendo medir la GPU. Una medición así no cuenta como medición de GPU (`CLAUDE.md`, sección 8).
Propuesta de la conversación de diseño, aceptada por el autor. Decisión: `docs/DECISIONS.md`,
2026-10-01 "El motor imprime su arranque, su adaptador y su backend, con su motivo".

**Dónde está escrito:** llvmpipe es el rasterizador por software de Mesa,
https://docs.mesa3d.org/drivers/llvmpipe.html, consultada el 2026-10-01.

**Cómo se comprueba:** el ejemplo `arranque` de la fase 0 imprime esas líneas. Corrido el
2026-10-02 sobre llvmpipe, con Vulkan y con GL: las dos veces imprimió la advertencia.

El motor todavía no tiene interfaz, así que no hay reglas de iconos ni colores.

## 11. Sin escribir todavía

- **Físicas propias o delegadas a un crate para detectar colisiones.** Sin decidir. La gravedad de
  la fase 4 usa un integrador propio, a confirmar por el autor (`docs/DECISIONS.md`, 2026-10-02
  "La gravedad del cubo usa un integrador propio"). Rapier se nombró como candidato; su estado
  está en `docs/ARCHITECTURE.md`, hueco "Rapier".
- **El umbral de "muchos objetos" del principio 5**, y el lenguaje de script.
- **El nivel base en macOS.** La tabla de wgpu 30.0.1 no marca ningún backend de macOS como
  "Downlevel/Best Effort": OpenGL ahí necesita ANGLE. Falta decidir si ese camino cuenta como nivel
  base.
- **El idioma de los identificadores del código.**
- **Cómo se escriben los tests.**
- **Dónde imprime el registro y con qué formato.**
