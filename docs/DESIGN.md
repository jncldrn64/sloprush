# DESIGN.md: cómo se escribe el código

> **Rol:** normativo sobre el código. Cada principio dice cómo se escribe el código y por qué.
> Manda sobre el código; `docs/REQUIREMENTS.md` manda sobre él. Describe reglas; el
> estado, qué rutas, recursos o funciones existen hoy, va en `docs/ARCHITECTURE.md`.
> **Régimen:** se corrige, y un principio nuevo o cambiado lleva su decisión. **Origen:** plantilla
> 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla 1.0").

Un cambio que rompe un principio de acá se rechaza, lo haya escrito una persona o un modelo. Ante la
duda, se copia la forma del código que ya existe en vez de inventar un idioma nuevo.

Hoy no hay código. Cada principio dice cómo se comprobará, y lo que todavía no se puede comprobar
está en la sección 10.

## 1. Rust estable

**Regla:** el código compila con el canal estable de Rust.

**Por qué:** **Sin origen recuperable.** Lo decidió el autor en `docs/DECISIONS.md`,
2026-10-01 "El lenguaje es Rust estable", sin dejar el motivo escrito.

**Dónde está escrito:** los canales stable, beta y nightly están en la documentación de rustup,
https://rust-lang.github.io/rustup/concepts/channels.html, consultada el 2026-10-01.

**Cómo se comprueba:** sin comando todavía. No hay proyecto Cargo, y fijar el toolchain es
alcance de la fase 0 de `docs/ROADMAP.md`.

## 2. Toda la salida gráfica pasa por wgpu

**Regla:** todo dibujo pasa por el crate wgpu. Ningún módulo llama directo a Vulkan, OpenGL,
Metal o DirectX.

**Por qué:** usar solo Vulkan con capas de compatibilidad se descartó, porque Vulkan no existe en
el navegador y en la GPU del equipo mínimo su driver libre no es conformante. Decisión:
`docs/DECISIONS.md`, 2026-10-01 "Toda la salida gráfica pasa por wgpu".

**Dónde está escrito:** la tabla "Supported Platforms" del README de wgpu 30.0.1,
https://docs.rs/crate/wgpu/30.0.1/source/README.md, consultada el 2026-10-01.

**Cómo se comprueba:** sin comando todavía (sección 10).

## 3. Dos niveles gráficos

**Regla:** toda capacidad del motor funciona en el nivel base, OpenGL ES 3.0 y WebGL2. El nivel
completo, Vulkan y Metal, solo agrega opciones visuales o de rendimiento, y con la misma entrada
llega al mismo estado de juego que el base.

**Por qué:** es el medio de A5 de `docs/REQUIREMENTS.md`. Un piso único se descartó porque dejaba
sin técnicas de rendimiento al nivel alto. Decisión: `docs/DECISIONS.md`, 2026-10-01 "Dos niveles
gráficos".

**Dónde está escrito:** la misma tabla de wgpu 30.0.1. Ahí OpenGL ES 3.0+ en Linux y WebGL2 en la
web figuran como "Downlevel/Best Effort Support", y Vulkan y Metal como "First Class Support".

**Cómo se comprueba:** se corre el mismo ejemplo con la misma entrada en cada nivel y se compara
el estado de juego. El comando no existe todavía (sección 10).

## 4. La simulación avanza a paso fijo de 60 Hz

**Regla:** la simulación avanza en pasos de 1/60 s, y cuántos pasos da no depende de cuántos
cuadros se dibujan.

**Por qué:** **Sin origen recuperable.** Lo decidió el autor en `docs/DECISIONS.md`,
2026-10-01 "La simulación avanza a paso fijo de 60 Hz", sin dejar el motivo escrito.

**Cómo se comprueba:** con la prueba prevista en esa decisión. Un cuerpo soltado en reposo con
gravedad 9,81 m/s² baja 4,905 m en 1 s de simulación, y baja lo mismo con el dibujo limitado a 30
y a 240 cuadros por segundo. La prueba no existe todavía; es el criterio de la fase 4 de
`docs/ROADMAP.md`. Le falta una tolerancia (sección 10).

## 5. Nativo contra script

**Regla:** es nativo lo que toca GPU, sistema operativo o hardware, o corre en cada cuadro sobre
muchos objetos. Lo demás se intenta primero como script, y pasa a nativo solo con una medición
registrada.

**Por qué:** A11 de `docs/REQUIREMENTS.md` pide ver qué es parte nativa del motor y qué puede ser
una función documentada de script. Decisión: `docs/DECISIONS.md`, 2026-10-01 "Nativo contra
script".

**Cómo se comprueba:** hoy no se puede. No hay lenguaje de script, así que todo el mínimo viable es
nativo, y "muchos objetos" no tiene número (sección 10).

## 6. Licencias de las dependencias

**Regla:** toda dependencia, directa o transitiva, tiene licencia MIT, Apache-2.0, BSD-2-Clause,
BSD-3-Clause, Zlib o ISC.

**Por qué:** **Sin origen recuperable.** Lo decidió el autor en `docs/DECISIONS.md`,
2026-10-01 "Licencias admitidas en las dependencias", sin dejar el motivo escrito.

**Cómo se comprueba:** `cargo deny check licenses`, documentado en
https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html, consultado el 2026-10-01.
Sin correr: no hay dependencias, y la herramienta entra en la fase 0. La decisión prevé además
cargo audit, que su manifiesto en crates.io describe como auditoría de Cargo.lock por
vulnerabilidades de seguridad.

## 7. Versión exacta de cada dependencia

**Regla:** cada dependencia de `Cargo.toml` lleva su versión exacta, con el operador `=`, y
`Cargo.lock` está versionado.

**Por qué:** **Sin origen recuperable.** Lo decidió el autor en `docs/DECISIONS.md`,
2026-10-01 "Versiones exactas y documentación de la versión", sin dejar el motivo escrito.

**Dónde está escrito:** la referencia de Cargo, "Specifying Dependencies",
https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html, consultada el 2026-10-01.
Ahí `"1.2.3"` sin operador equivale a `^1.2.3`, que admite versiones compatibles, y `= 1.2.3` es la
versión exacta.

**Cómo se comprueba:** `git ls-files Cargo.lock` tiene que listarlo. Sin correr: no hay proyecto
Cargo todavía.

## 8. Los tests

Todavía no hay tests, y cómo se escriben está en la sección 10. Una suite que alguna vez falló de
forma intermitente se corre N veces, no una, y se queda en esa lista después del arreglo. Cómo se
corren está en `tests/README.md`.

## 9. Presentación y registro

**Regla:** el motor imprime cada paso de arranque, carga y cierre, y al iniciar, el adaptador
gráfico y el backend elegido. Si el adaptador es llvmpipe u otro renderizador por software, lo
advierte.

**Por qué:** una medición hecha con un renderizador por software no cuenta como medición de GPU
(`CLAUDE.md`, sección 8), y la advertencia es lo que lo deja a la vista. Decisión:
`docs/DECISIONS.md`, 2026-10-01 "El motor imprime su arranque, su adaptador y su backend".

**Dónde está escrito:** llvmpipe es el rasterizador por software de Mesa,
https://docs.mesa3d.org/drivers/llvmpipe.html, consultada el 2026-10-01.

**Cómo se comprueba:** el ejemplo de la fase 0 de `docs/ROADMAP.md` imprime esas líneas. No existe
todavía.

El motor todavía no tiene interfaz, así que no hay reglas de iconos ni colores.

## 10. Sin escribir todavía

- **Antigüedad de los crates.** Sin decidir. El autor pidió un mínimo de 1 año de antigüedad y 1
  actualización en el último mes. En la conversación de diseño se propuso cambiar lo segundo a
  "último release dentro de los últimos 12 meses", porque un crate maduro puede pasar meses sin
  cambios. El autor no eligió. La fase 0 lo necesita antes de sumar la primera dependencia.
- **Físicas propias o delegadas a un crate.** Sin decidir. Rapier se nombró como candidato, sin
  comprobar contra el principio 6 ni contra el criterio de antigüedad. La fase 4 lo necesita.
- **Tolerancia de la prueba del principio 4.** 4,905 m es la caída exacta, g·t²/2. Un integrador
  de pasos de 1/60 s da otra cifra: 4,98675 m con Euler semiimplícito y 4,82325 m con Euler
  explícito. Lo recalcula
  `python3 -c "g,h,n=9.81,1/60,60; print(g*h*h*n*(n+1)/2, g*h*h*n*(n-1)/2)"`. Falta elegir el
  integrador o la tolerancia.
- **El umbral de "muchos objetos" del principio 5**, y el lenguaje de script.
- **El nivel base en Windows y macOS.** La tabla de wgpu 30.0.1 da GL 3.3+ en Windows y GL con la
  capa ANGLE en macOS. El nivel completo tampoco nombra DirectX 12, que esa tabla da como soporte
  de primera clase en Windows.
- **Los comandos que comprueban los principios 2 y 3.**
- **Cómo se escriben los tests.**
- **Dónde imprime el registro y con qué formato.**
