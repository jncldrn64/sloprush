# GLOSSARY.md: qué significa cada palabra hoy

> **Rol:** descriptivo. Dice qué significa un término hoy; por qué cambió vive en
> `docs/DECISIONS.md`. Cada línea cita su fuente por fecha y título de la decisión, o la sección
> del CHANGELOG que trajo el término. Una palabra que nombra algo que todavía no existe lo dice.
> **Régimen:** se corrige. **Origen:** plantilla 1.0, sembrada el 2026-10-01 (`docs/DECISIONS.md`,
> 2026-10-01 "Se adopta la plantilla 1.0").

Una línea por término, en orden alfabético dentro de cada grupo. Un grupo se abre cuando hay
suficientes términos para que el orden alfabético solo ya no alcance.

## 1. Método

Fuente de las líneas de este grupo que no citan otra: 2026-10-01 "Se adopta la plantilla 1.0".

- **borrador**: el estado en que se abre un PR. El autor lo marca listo y lo mergea.
- **cita textual**: palabras del autor copiadas tal cual, en una viñeta propia que abre con
  comillas o con su etiqueta, como `**A13.**`. No se reescribe ni se resume, y no cuenta para las
  reglas 1 y 2 de prosa. Fuente: 2026-10-02 "Las citas textuales del autor no cuentan para las
  reglas 1 y 2".
- **contexto temporal**: lo que se observó y se perdería si nadie lo escribe, antes de saber si se
  quiere. Vive en `docs/TEMPORARY-CONTEXT.md`, y cada línea se coloca o se descarta.
- **decisión**: una entrada de `docs/DECISIONS.md` que dice por qué algo es como es. No se edita
  nunca.
- **desfase**: la distancia entre la última versión del CHANGELOG y la que muestra el programa. La
  abre un PR doc-only y la cierra el próximo PR de código.
- **descriptivo**: el documento que dice qué es. Pierde contra el código.
- **Entró**: la línea con que nace un ítem del Backlog, con fecha y PR sacados de `git log -S`.
- **Hipótesis**: el marcador de una inferencia hecha al reconstruir contexto, con su base a la
  vista. Su contraparte es **Sin origen recuperable**.
- **hueco conocido**: una entrada fechada de la sección "Huecos conocidos" de
  `docs/ARCHITECTURE.md`, con la corrida que la muestra. Se borra cuando se cierra.
- **línea base**: un conteo de prosa medido en una fecha con un comando escrito. No puede subir.
- **lista para verificación**: el estado en que el agente deja una fase. Solo el autor la pasa a
  `cerrada`, después de correr sus ejemplos en los dos equipos. Fuente: 2026-10-01 "Estado de fase
  lista para verificación, con su motivo".
- **normativo**: el documento que dice qué tiene que ser. Si el código lo contradice, el código
  está en falta.
- **PR doc-only**: un PR que no toca código ni tests. Abre su propia sección del CHANGELOG y no sube
  la versión.
- **terna plana**: tres oraciones seguidas cuyos largos caen dentro de 3 palabras. La unidad de la
  regla 8 de prosa.
- **tipo**: la palabra que abre un commit o el título de un PR: `add`, `chg`, `fix`, `rmv` o `doc`.

## 2. Motor

- **API de bloques**: la interfaz con que el desarrollador arma sus cosas con bloques. Todavía no
  existe, y es inestable hasta que el autor la congele. Fuente: 2026-10-01 "La API de bloques es
  inestable hasta que el autor la congele, con su motivo".
- **aviso**: lo que el chequeo de seguridad marca sobre un crate: una vulnerabilidad, un crate
  declarado sin mantenimiento o una versión retirada. La regla es cero avisos en todo el árbol.
  Fuente: 2026-10-01 "Seguridad de las dependencias".
- **backend**: la API gráfica que wgpu usa por debajo en una corrida, como Vulkan u OpenGL ES. El
  motor la imprime al iniciar. Fuente: 2026-10-01 "El motor imprime su arranque, su adaptador y su
  backend, con su motivo".
- **bloque**: la palabra del autor en A1 de `docs/REQUIREMENTS.md`. Todavía no tiene definición:
  sale del catálogo de bloques. Fuente: 2026-10-01 "Primero el mínimo viable, después el catálogo
  de bloques, con su motivo".
- **catálogo de bloques**: la lista de bloques que se extrae de lo construido en el mínimo viable y
  que congela el autor. Todavía no existe. Fuente: la misma entrada que **bloque**.
- **congelar**: fijar una API por escrito, cosa que solo hace el autor. Antes de eso la API es
  inestable. Fuente: la misma entrada que **API de bloques**.
- **equipo de desarrollo**: el PC del autor, con Linux Mint 22, un i7-7700 y una Radeon RX
  470/480/570/580. Prueba lo mismo que el equipo mínimo, más el 3D pesado y las mediciones. Fuente:
  2026-10-01 "Dos equipos de prueba, con su motivo".
- **equipo mínimo**: la Orange Pi Zero 3 del autor, con Armbian bookworm aarch64 y 1973 MiB de RAM.
  Prueba todo el 2D y el 3D básico. Fuente: 2026-10-01 "Dos equipos de prueba, con su motivo".
- **escalera de licencias**: el orden en que se busca una dependencia. Escalón 1, licencias
  permisivas, sin autorización; 2, implementar desde cero; 3, copyleft débil, con autorización del
  autor; 4, GPL-3.0 y AGPL-3.0, con autorización y después del 3. Fuente: 2026-10-01 "Escalera de
  licencias para las dependencias".
- **modo de cuadros fijos**: la corrida de un ejemplo con `--cuadros N`, que dibuja N cuadros y
  termina con código 0. Da sí o no sin mirar la pantalla. Fuente: CHANGELOG, v0.2.
- **lienzo**: una textura fuera de pantalla donde se dibuja un cuadro para leer sus píxeles de
  vuelta. Es como prueban las fases 2 y 3 lo que se dibuja. Fuente: 2026-10-02 "Las pruebas de
  dibujo leen un lienzo fuera de pantalla".
- **mínimo viable**: una ventana, un sprite 2D, un cubo 3D con cámara, entrada de teclado y gravedad
  sobre el cubo, y nada más. Todavía no existe. Fuente: 2026-10-01 "Alcance del mínimo viable, con
  su motivo".
- **nativo**: lo que toca GPU, sistema operativo o hardware, o corre en cada cuadro sobre muchos
  objetos, más lo que pasó de script a nativo con una medición registrada. Fuente: 2026-10-01
  "Nativo contra script, con su motivo".
- **nivel base**: los backends que la tabla "Supported Platforms" de wgpu marca como
  "Downlevel/Best Effort Support", la familia OpenGL. Toda capacidad del motor funciona ahí. Fuente:
  2026-10-01 "Dos niveles gráficos definidos por la tabla de wgpu".
- **nivel completo**: los backends que esa tabla marca como "First Class Support". Solo agrega
  opciones visuales o de rendimiento al nivel base. Fuente: 2026-10-01 "Dos niveles gráficos
  definidos por la tabla de wgpu".
- **paso fijo**: el avance de la simulación en pasos de igual duración, que no depende de cuántos
  cuadros se dibujan. Su frecuencia es configurable, vale 60 Hz por defecto y no cambia dentro de
  una partida. Fuente: 2026-10-01 "La simulación avanza a paso fijo de frecuencia configurable".
- **renderizador por software**: un adaptador que dibuja con la CPU, como llvmpipe. Una medición
  hecha con uno no cuenta como medición de GPU. Fuente: 2026-10-01 "El motor imprime su arranque,
  su adaptador y su backend, con su motivo".
- **script**: una función documentada que extiende el motor sin ser nativa, al estilo MTA de A11 en
  `docs/REQUIREMENTS.md`. Todavía no existe, y su lenguaje no está elegido. Fuente: 2026-10-01
  "Nativo contra script, con su motivo".
- **sprite**: un rectángulo 2D con una textura. Hoy, el damero de `src/sprite.rs`. Fuente:
  2026-10-02 "La textura del sprite se arma en el código".
