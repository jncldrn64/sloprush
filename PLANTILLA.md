# PLANTILLA.md: cómo usar la plantilla 1.0

Este archivo no se copia al repo nuevo. Explica la plantilla y qué la separa de los dos repos de los
que sale.

## 1. Qué es

Diez documentos genéricos, más un par opcional para un módulo y un `tests/README.md`. Salen de las
reglas que hoy rigen en MIDI-Scale-Trainer y en TL-FCCU. Cuando los dos repos discrepaban, decidió
MIDI, salvo en tres puntos que la sección 3 justifica.

```
AGENTS.md              puerta, no manda
CLAUDE.md              método, jerarquía y formato común
CHANGELOG.md           historia: qué y cuándo
docs/REQUIREMENTS.md   normativo: qué tiene que ser verdad
docs/DESIGN.md         normativo: cómo se escribe el código
docs/ARCHITECTURE.md   descriptivo: qué es el código hoy, y los huecos
docs/GLOSSARY.md       descriptivo: qué significa cada palabra
docs/ROADMAP.md        plan: fases y Backlog
docs/DECISIONS.md      historia: por qué, append-only
docs/TEMPORARY-CONTEXT.md  tránsito: lo que se perdería
docs/modulo/standard.md + surface.md   opcional, un módulo con superficie propia
tests/README.md        opcional, solo esa carpeta
```

## 2. Cómo se aplica

1. Se copian los archivos al repo nuevo, menos este.
2. Se reemplaza cada `<...>` y cada `AAAA-MM-DD`. Lo que todavía no se sabe se deja como semilla, y
   el archivo lo dice en su sección "Sin escribir todavía".
3. Si el repo se escribe en inglés, se traduce el contenido y se cambia la lista de la regla 1 de
   prosa. Los nombres de archivo ya están en inglés.
4. En el mismo PR se corre el bloque de comandos de `CLAUDE.md`, sección 9, y las líneas base se
   escriben ahí con su fecha.
5. El primer PR es `doc` y abre la sección `v0.1` del CHANGELOG.

## 3. Qué cambia respecto de los repos actuales

Ninguno de los dos repos se tocó. Lo que sigue es lo que cada uno tendría que cambiar para quedar en
sintonía con la plantilla, si alguna vez se quiere.

| Punto | MIDI hoy | FCCU hoy | Plantilla |
|---|---|---|---|
| Dónde está la jerarquía | `AGENTS.md`, 5 niveles | No hay ("no document hierarchy for now") | `CLAUDE.md`, sección 2, por tipo de documento |
| Convenciones del código | En `CLAUDE.md`: iconos, colores, verbosidad | En `docs/DESIGN.md` | Solo en `docs/DESIGN.md` |
| Orden de lectura | Empieza por ARCHITECTURE, DECISIONS completo en segundo lugar | Empieza por REQUIREMENTS, DECISIONS por títulos | Empieza por REQUIREMENTS, DECISIONS por títulos |
| Nombres de archivo | Mezcla: `REQUISITOS`, `GLOSARIO`, `CONTEXTO-TEMPORAL` | Inglés | Inglés |
| Encabezado de una decisión | `## fecha — título` | `## fecha: título` | `## fecha: título` |
| Línea 1 de un documento | A veces el nombre del proyecto | A veces un título libre (DESIGN, ROADMAP, CLI) | Siempre `# NOMBRE.md: qué guarda` |
| Bloque de cita con Rol, Régimen y Origen | Parcial, sin esos campos | Parcial, sin esos campos | En todos |
| Campos de fase | `**Estado:**` en negrita | `Status:` plano | En negrita con dos puntos |
| Marcadores de inferencia | Tres | Dos, falta "Por qué se anotó" | Tres |
| Glosario | Agrupado por tema | Una lista | Lista, y se agrupa cuando crece |
| CLAUDE.md por encima de REQUIREMENTS | Sí, nivel 3 contra 4 | Sin jerarquía | No compiten: método contra producto |

**Los tres puntos donde no decidió MIDI**, y por qué:

1. **Las convenciones del código fuera de `CLAUDE.md`.** Es la discrepancia que pediste cerrar. En
   MIDI, un modelo que busca cómo escribir el código tiene que leerlo en el archivo del método; en
   FCCU, `DESIGN.md` existía y no tenía lugar en ninguna jerarquía. La plantilla le da uno:
   normativo, debajo de REQUIREMENTS.
2. **DECISIONS por títulos y no completo.** MIDI lo hace obligatorio y completo, con 39.262
   palabras. FCCU midió el costo el 2026-10-01: leerlo entero duplicaba la lectura inicial.
3. **Nombres en inglés.** MIDI ya mezcla `ARCHITECTURE`, `ROADMAP` y `DECISIONS` en inglés con tres
   en español. Fijarlo en un idioma termina la mezcla y no depende del idioma del repo.

## 4. Qué no está verificado

La plantilla nunca se usó. Las reglas salen de dos repos donde sí rigen, pero que este conjunto
funcione junto se comprueba recién con el primer repo que lo adopte. La jerarquía de la sección 2
de `CLAUDE.md` es nueva en esta forma: MIDI tiene una parecida y FCCU ninguna.
