# REQUIREMENTS.md: qué tiene que ser verdad, y para quién

> **Rol:** normativo, el más alto sobre el producto. Dice qué tiene que ser verdad para que el
> programa esté bien hecho, y para quién. No lleva fechas ni orden de trabajo, que van en
> `docs/ROADMAP.md`. No dice por qué ganó una forma, que va en `docs/DECISIONS.md`. **Régimen:** se
> corrige, y cada cambio lleva su decisión. **Origen:** plantilla 1.0, sembrada el 2026-10-01
> (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla 1.0").

## 1. Para quién es

Las frases son del autor, dichas el 2026-10-01, con la ortografía corregida y nada más. Van
ordenadas por tema. La etiqueta de cada una sirve para citarla desde otros documentos.

Qué es:

- **A1.** "No quiero un mod, quiero un motor donde yo hacer mis cosas propias mediante mis
  bloques."
- **A10.** "Un motor genérico dicta el techo, y de él salen las cosas que pueden o no hacerse."
- **A11.** "Extender las capacidades del motor exigiendo genéricos para ver qué es parte nativa del
  motor, y qué puede ser solo una función documentada como scripting (al estilo MTA)."
- **A8.** "Quiero que tenga una interfaz que permita incluso la ejecución de código arbitrario
  mediante una API interna e instrucciones personalizadas, para que el desarrollador codifique
  nuevas funciones en la propia GUI del motor."

Dónde corre:

- **A3.** "El motor debe correr en Linux, FreeBSD, Windows, macOS, todos en al menos los OS o
  kernels soportados de los últimos 5 años."
- **A4.** "Quiero poder moverlo incluso en una board H618."
- **A2.** "Yo quiero el más puro agnosticismo tanto como sea posible, para que solo haya una
  implementación posible que pueda ser transferible."
- **A5.** "Si hay discrepancias entre funcionalidad que existe en un lado u otro, se tomará como
  límite la implementación que tenga menos, y se podrán usar opciones avanzadas adicionales para
  que, si el desarrollador que esté usando el motor quiere, implemente algo visual de una
  implementación concreta (que no afecte al posible gameplay que haga)."

Qué hace:

- **A6.** "El motor debe soportar 3D y 2D, así mismo como fondos prerrenderizados, skyboxes o
  similares."
- **A7.** "Quiero poder llevar el motor a más de 240 fps."
- **A9.** "Exportar los juegos bajo los mismos parámetros que el propio motor."

Referencias que el autor nombró, sin frase textual: MTA: San Andreas, como Lua que extiende un
juego, y Garry's Mod.

## 2. Qué tiene que ser verdad

Cada requisito cita la frase de la sección 1 de la que sale. Si todavía no tiene forma de
comprobarse, lo dice, y el dato que falta está en la sección 5.

1. **Es un motor, y lo que se hace con él se arma con bloques** (A1). Qué es un bloque sale del
   catálogo, que se extrae después del mínimo viable.
2. **Lo que el motor no trae como nativo se agrega como función documentada de script** (A10,
   A11). Se comprueba cuando exista el scripting, que está en el Backlog de `docs/ROADMAP.md`.
3. **Dibuja en 2D y en 3D** (A6). Se comprueba con los ejemplos del mínimo viable: un sprite 2D y
   un cubo 3D con cámara. Los fondos prerrenderizados y los skyboxes están en el Backlog.
4. **El desarrollador programa funciones nuevas desde la interfaz del motor** (A8). Está en el
   Backlog.
5. **Los juegos se exportan** (A9). Está en el Backlog, y qué son "los mismos parámetros" está en
   la sección 5.

## 3. Qué no es

- Un mod de un juego existente (A1).
- Ni un juego de conducción ni uno de construcción de bloques. El autor nombró un mundo abierto de
  conducción con tráfico y físicas, en 3D, y un juego de construcción de bloques con físicas y
  gravedad, en 2D, como fuente futura de escenas de prueba. Ninguno es un entregable.
- Una API de bloques estable. Es inestable hasta que el autor la congele por escrito
  (`docs/DECISIONS.md`, 2026-10-01 "La API de bloques es inestable hasta que el autor la
  congele").

## 4. Requisitos no funcionales

1. **Plataformas** (A3). Hoy se comprueba solo Linux, en los dos equipos de `docs/GLOSSARY.md`.
   FreeBSD, Windows, macOS y el navegador están en el Backlog como plataformas por verificar.
2. **Equipo mínimo** (A4). Todo el 2D y el 3D básico corren en el equipo mínimo, una Orange Pi
   Zero 3. Se comprueba corriendo ahí los ejemplos de cada fase.
3. **Una sola implementación del juego** (A2, A5). Con la misma entrada, el estado de juego es el
   mismo sobre cualquier implementación gráfica, y lo que una tiene y otra no solo puede ser
   visual. Los medios están en `docs/DESIGN.md`: "Dos niveles gráficos" y "La simulación avanza a
   paso fijo de 60 Hz".
4. **Rendimiento** (A7). Más de 240 cuadros por segundo. Sin escena definida: sección 5.
5. **Licencia del motor.** AGPL-3.0, con el archivo `LICENSE` (`docs/DECISIONS.md`, 2026-10-01
   "La licencia del motor es AGPL-3.0"). Qué licencias se admiten en las dependencias es un medio:
   `docs/DESIGN.md`, "Licencias de las dependencias".
6. **Avisos de licencia en lo exportado.** Todo juego exportado incluye los avisos de licencia de
   las dependencias (`docs/DECISIONS.md`, 2026-10-01 "Los juegos exportados llevan los avisos de
   licencia"). Se comprueba cuando exista la exportación.

## 5. Sin escribir todavía

Mientras esta lista tenga algo, el archivo es una semilla.

- Qué pasa si el motor falla, y para quién más es. La sección 1 lo pide y ninguna frase del autor
  lo dice.
- Qué versiones de cada sistema entran en "los OS o kernels soportados de los últimos 5 años"
  (A3).
- La escena de los 240 fps (A7): escena, cantidad de objetos, equipo, resolución, backend y
  sincronización vertical, como pide `CLAUDE.md`, sección 8. Los juegos de la sección 3 son una
  fuente posible.
- Qué obliga la AGPL-3.0 a quien exporta un juego. Pregunta abierta del autor, sin contestar.
- Qué son "los mismos parámetros que el propio motor" de A9.
- Qué es un bloque (A1) y qué es un genérico (A11). Salen del catálogo de bloques.
