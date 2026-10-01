# GLOSSARY.md: qué significa cada palabra hoy

> **Rol:** descriptivo. Dice qué significa un término hoy; por qué cambió vive en
> `docs/DECISIONS.md`. Cada línea cita su fuente por fecha y título de la decisión, o la sección
> del CHANGELOG que trajo el término. Una palabra que nombra algo que todavía no existe lo dice.
> **Régimen:** se corrige. **Origen:** plantilla 1.0.

Una línea por término, en orden alfabético dentro de cada grupo. Un grupo se abre cuando hay
suficientes términos para que el orden alfabético solo ya no alcance.

## Método

- **borrador**: el estado en que se abre un PR. El autor lo marca listo y lo mergea.
- **contexto temporal**: lo que se observó y se perdería si nadie lo escribe, antes de saber si se
  quiere. Vive en `docs/TEMPORARY-CONTEXT.md`, y cada línea se coloca o se descarta.
- **decisión**: una entrada de `docs/DECISIONS.md` que dice por qué algo es como es. No se edita
  nunca.
- **desfase**: la distancia entre la última versión del CHANGELOG y la que muestra el programa. La
  abre un PR doc-only y la cierra el próximo PR de código.
- **descriptivo**: el documento que dice qué es. Pierde contra el código.
- **Entró**: la línea con que nace un ítem del Backlog, con fecha y PR sacados de `git log -S`.
- **hueco conocido**: una entrada fechada de la sección "Huecos conocidos" de
  `docs/ARCHITECTURE.md`, con la corrida que la muestra. Se borra cuando se cierra.
- **Hipótesis**: el marcador de una inferencia hecha al reconstruir contexto, con su base a la
  vista. Su contraparte es **Sin origen recuperable**.
- **línea base**: un conteo de prosa medido en una fecha con un comando escrito. No puede subir.
- **normativo**: el documento que dice qué tiene que ser. Si el código lo contradice, el código
  está en falta.
- **PR doc-only**: un PR que no toca código ni tests. Abre su propia sección del CHANGELOG y no sube
  la versión.
- **terna plana**: tres oraciones seguidas cuyos largos caen dentro de 3 palabras. La unidad de la
  regla 8 de prosa.
- **tipo**: la palabra que abre un commit o el título de un PR: `add`, `chg`, `fix`, `rmv` o `doc`.
