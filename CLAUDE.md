# CLAUDE.md: método de trabajo

> **Rol:** normativo sobre el método. Dice cómo se lee, se escribe, se mide y se entrega en este
> repo. No dice qué hace el producto ni cómo se escribe su código: eso es `docs/REQUIREMENTS.md` y
> `docs/DESIGN.md`. **Régimen:** se corrige. Un cambio de regla lleva su entrada en
> `docs/DECISIONS.md` en el mismo PR. **Origen:** plantilla 1.0, sacada de dos repos que ya usaban
> este método, y sembrada el 2026-10-01 (`docs/DECISIONS.md`, 2026-10-01 "Se adopta la plantilla
> 1.0").

## 1. Orden de lectura

Antes de tocar nada se lee, en este orden:

1. `docs/REQUIREMENTS.md`: qué tiene que ser verdad, y para quién.
2. `docs/ARCHITECTURE.md`: qué es el código hoy y qué huecos quedan.
3. `docs/DESIGN.md`: cómo se escribe el código. Obligatorio antes de escribir código.
4. `docs/ROADMAP.md`: cuando el trabajo pertenece a una fase.
5. `docs/DECISIONS.md`, por sus títulos: `grep '^## ' docs/DECISIONS.md` y se leen las entradas
   que toca el trabajo. Antes de proponer un cambio de estructura o de método se lee además la
   entrada "Ideas descartadas", que existe para que nadie las vuelva a proponer.
6. `docs/TEMPORARY-CONTEXT.md`, último: lo más volátil. Su estado normal es vacío, y se lee igual,
   porque una línea de ahí puede contradecir lo que uno estaba por proponer.

## 2. Jerarquía: qué manda cuando dos dicen cosas distintas

Cada documento es de un tipo, y el tipo decide qué pasa cuando choca con el código o con otro
documento. El tipo va en la primera línea del bloque de cita de cada archivo, después de **Rol:**.

| Tipo | Archivos | Contra el código |
|---|---|---|
| Normativo | `REQUIREMENTS.md`, `DESIGN.md`, `CLAUDE.md`, el estándar de un módulo | El código está en falta |
| Descriptivo | `ARCHITECTURE.md`, `GLOSSARY.md`, la superficie de un módulo | Gana el código |
| Historia | `DECISIONS.md`, `CHANGELOG.md` | No compite: registra. Una decisión vigente sí gana sobre un normativo viejo |
| Plan | `ROADMAP.md` | No compite: ordena el trabajo |
| Tránsito | `TEMPORARY-CONTEXT.md` | Nunca manda |
| Puerta | `AGENTS.md` | Nunca manda: apunta |

Las reglas:

1. **Un descriptivo que contradice al código está mal**, y se corrige en el mismo PR que lo
   encuentra.
2. **Un código que contradice a un normativo está en falta.** O se arregla el código, o se escribe
   una decisión que cambia la norma y la norma se edita en el mismo PR. Nunca se deja la
   contradicción escrita.
3. **Entre normativos, el fin manda sobre el medio:** `REQUIREMENTS.md` manda sobre `DESIGN.md`, y
   los dos mandan sobre el estándar de un módulo. `CLAUDE.md` gobierna el método y no el producto,
   así que no compite con ellos.
4. **Una decisión vigente gana sobre un normativo que la contradice.** El normativo quedó viejo y
   se corrige en el PR que lo encuentra. Una entrada reemplazada no gana nada, y entre dos
   decisiones manda la más nueva, que nombra a la que reemplaza.
5. **`ROADMAP.md` no cambia una norma.** Una fase que necesita otra norma la cambia por la regla 2.
6. **`TEMPORARY-CONTEXT.md` y `AGENTS.md` no mandan nunca.**

**Las convenciones del código del producto viven en `docs/DESIGN.md` y en ningún otro lado**, ni
siquiera acá. Este archivo dice cómo se trabaja; `DESIGN.md` dice cómo se escribe el código. Mezclar
los dos fue la discrepancia que esta plantilla corrige.

## 3. Documentación

Los documentos canónicos son los de la tabla de la sección 2, más `CHANGELOG.md`. En la raíz:
`AGENTS.md`, `CLAUDE.md` y `CHANGELOG.md`. El resto en `docs/`, y un módulo con documentación propia
en `docs/<módulo>/`, con un par: un estándar normativo y una superficie descriptiva.

- **No se crea un documento nuevo sin preguntar al autor.** La excepción es un `README.md` de
  subcarpeta que solo explica esa carpeta, como `tests/README.md`.
- **Los nombres de archivo van en inglés** y el contenido en el idioma del repo. Los nombres son
  convención de herramientas; el idioma, del autor.
- **Este repo no describe otros repos.** El nombre de otro puede aparecer como procedencia, nunca
  como información operativa.

## 4. Formato común de los documentos

Todos los `.md` canónicos tienen la misma forma, para que un modelo sepa dónde mirar sin leer
entero:

1. **Línea 1:** `# NOMBRE.md: qué guarda`. Dice el rol, no el nombre del proyecto.
2. **Bloque de cita** inmediato, con tres campos en negrita: **Rol:** (tipo de la sección 2 y qué
   guarda), **Régimen:** (se corrige, append-only, crece por sección o tiende a cero) y **Origen:**
   (cuándo nació y la entrada de `DECISIONS.md` que lo explica). Si dice qué no guarda, dice
   también dónde va eso.
3. **Secciones `##` numeradas** (`## 1. Título`) en normativos y descriptivos, para poder citarlas
   como "sección 3". En historia, plan y tránsito, las secciones son entradas fechadas o fases y no
   se numeran.
4. **Campos en negrita con dos puntos:** `**Estado:**`, `**Contexto:**`. El valor de un estado va
   entre acentos graves: `` `pendiente` ``.
5. **Una entrada fechada abre con** `## AAAA-MM-DD: título`, con dos puntos y sin guion largo.
6. **Un `###` solo dentro de un descriptivo largo.** Si un normativo necesita `###`, probablemente
   necesita partirse.
7. **Ancho de línea:** hasta 100 columnas.
8. **Una referencia a otro documento** cita ruta y sección por título, o fecha y título si es una
   decisión. Nunca un número de línea.

## 5. Decisiones, huecos y contexto temporal

- **Una decisión va en `docs/DECISIONS.md`**, append-only, una entrada por decisión. Una nueva
  reemplaza a una vieja nombrándola por fecha y título.
- **Un hueco abierto va en la sección "Huecos conocidos" de `docs/ARCHITECTURE.md`**, fechado y con
  el comando o la corrida que lo muestra. Se borra en el PR que lo cierra; el CHANGELOG guarda el
  rastro.
- **Un puntero a una decisión** se agrega donde una línea se leería arbitraria o contradictoria sin
  él, y cita fecha y título.
- **Una decisión que introduce o refina un término** escribe su línea en `docs/GLOSSARY.md` en el
  mismo PR.
- **Lo que se observó y se perdería** va a `docs/TEMPORARY-CONTEXT.md`, sin pedir permiso. Sus
  reglas viven en ese archivo.

## 6. CHANGELOG y versión mostrada

- [Keep a Changelog](https://keepachangelog.com). Un solo archivo que crece por secciones, lo más
  nuevo arriba. Cada sección abre con `## vX.Y — AAAA-MM-DD` y lleva `### Added`, `### Changed`,
  `### Fixed` o `### Removed`.
- **Un PR solo de documentación abre su propia sección fechada** y no sube la versión. Una sección
  publicada es historia y no se reescribe.
- **La versión que muestra el programa es la última del CHANGELOG.** Se sube en el PR de código que
  la amerita. Un PR doc-only puede dejar el CHANGELOG adelante; ese desfase lo cierra el próximo PR
  de código.

## 7. Commits y PR

- **Commit:** `<tipo>: <resumen imperativo corto>`, con tipo en `{add, chg, fix, rmv, doc}`. `add`
  es capacidad nueva, `chg` cambio de comportamiento, `fix` corrección, `rmv` algo que se saca,
  `doc` solo documentación. El cuerpo no vuelve a narrar el cambio: una o dos líneas y la sección
  del CHANGELOG.
- **Título del PR:** el mismo formato y el mismo tipo que el commit.
- **Cuerpo del PR:** la tabla de archivos se copia de `git diff --numstat origin/main...HEAD`,
  corrido como último paso antes de escribirla.
- **Se trabaja vía PR, abierto como borrador.** Si `push`, crear una rama o abrir el PR devuelve
  `403`, se para y se avisa que falta permiso de escritura.
- **Fechas:** siempre ISO 8601. Un ítem del Backlog nace con una línea `**Entró:**`, con la fecha
  y el PR que lo trajo, sacados de `git log -S` sobre el archivo.

## 8. Honestidad de estado y promesas

- **Nada se declara "funciona" o "probado" sin una corrida real.** Si no se verificó, se dice con
  esas palabras.
- **Una instrucción de comprobación trae sus condiciones**, no solo sus pasos.
- **Al reconstruir contexto perdido, una inferencia se marca:** `**Hipótesis:**` con su base a la
  vista, `**Por qué se anotó:**` para lo que tiene cita y `**Sin origen recuperable.**` para lo que
  no se sabe. Ante la duda entre inferir y declarar el vacío, se declara el vacío.
- **Ninguna regla prescribe un mecanismo futuro.** Un umbral puede obligar a decidir, no decidir
  por adelantado. Una frase que nombra una sintaxis, un protocolo o una API concretos va con su
  corrida, o con un puntero a donde está escrita.
- **Cuando un umbral se dispara**, la decisión contesta tres cosas antes de planear nada: qué se
  está volviendo difícil, con el número que lo muestra; qué opciones hay y cuánto cuesta cada una;
  y qué corrida descarta las que no funcionan.
- **Una idea se descarta** cuando su complejidad supera un beneficio que se pueda medir, nunca
  porque suene riesgosa, y después de comprobar qué quiso decir quien la propuso.
- **Toda cifra de rendimiento lleva sus condiciones:** escena, cantidad de objetos, equipo,
  resolución, backend, sincronización vertical y el comando que la produjo. Una medición hecha con
  llvmpipe u otro renderizador por software no cuenta como medición de GPU (`docs/DESIGN.md`,
  "Presentación y registro"). Decisiones: 2026-10-01 "Toda cifra de rendimiento lleva sus
  condiciones" y "El motor imprime su arranque, su adaptador y su backend".
- **Antes de usar una API de una dependencia se lee la documentación de la versión fijada** en
  `Cargo.toml`. Decisión: 2026-10-01 "Versiones exactas y documentación de la versión".

## 9. Prosa

Docs y comentarios en el idioma del repo, aplicando las skills `no-ai-slop-writing-rules:no-ai-slop`
y `no-ai-slop-writing-rules:rossmann-voice`. Vienen del plugin externo `no-ai-slop-writing-rules`
(realrossmanngroup, https://github.com/realrossmanngroup/no_ai_slop_writing_rules), que se instala
por sesión con `/plugin marketplace add realrossmanngroup/no_ai_slop_writing_rules` y después
`/plugin install no-ai-slop-writing-rules`. No se vendorea porque no trae licencia.

**Si el texto original del plugin está al alcance en la sesión, se lee y manda sobre este piso.**
El disparador es tenerlo, instalado o adjunto, no la forma en que llegó.

**Guion largo (`—`):** prohibido en la prosa. Solo se permite como token de formato en los
encabezados de sección del CHANGELOG.

**El piso.** Ocho reglas. Cada una tiene una línea base medida con su comando, y **ninguna línea
base puede subir**. La primera medición se hace en el PR que siembra los documentos, y desde ahí
cada PR la compara.

1. **Intensificadores.** Antes de entregar se buscan las palabras de la lista del comando de la
   regla 1. Con el plugin instalado, valen además sus listas completas.
2. **Paralelismo contrastivo** ("no es X, es Y"): como máximo uno cada 500 palabras por archivo.
3. **Viñeta del CHANGELOG:** hasta 60 palabras. Si no entra, son dos.
4. **Encabezado con paréntesis:** solo cuando el paréntesis trae un dato, como un número o un estado
   de verificación.
5. **"No verificado"** sobre el código es obligatorio. Narrar qué se buscó y no se encontró, no.
6. **Una afirmación sobre el código se ancla en algo que sobreviva a un refactor**, un nombre de
   función o una cita que se pueda grepear, nunca en un número de línea. Un número que describe el
   código va con el comando que lo recalcula, o no va. Escribir el comando no es correrlo.
7. **Un párrafo de prosa corrida:** hasta cinco oraciones. Listas, tablas y líneas de glosario
   quedan fuera.
8. **Tres oraciones seguidas de largo parecido** son la señal de que el texto se alisa. La medida es
   qué porcentaje de las ternas consecutivas cae dentro de 3 palabras.

El corpus son todos los `.md` versionados menos este archivo, que movería sus propios números, y
`docs/TEMPORARY-CONTEXT.md`, exento por diseño. Los comandos, desde la raíz:

```sh
C=$(git ls-files '*.md' ':!:CLAUDE.md' ':!:*TEMPORARY-CONTEXT.md')

# Palabras del corpus.
wc -w $C | tail -1

# Regla 1. Lista en español; en un repo en inglés: very|absolutely|clearly|simply|probably|actually|really
grep -niwE "muy|absolutamente|claramente|simplemente|probablemente|realmente" $C

# Regla 2, por archivo: palabras y casos. Dividir el primero por el segundo.
for f in $C; do printf '%s %s %s\n' "$f" "$(wc -w < "$f")" "$(grep -ciE \
  "no (es|son|era|fue) [^,.;]{2,45}[,;] (es|sino|son)|[a-zá-úñ]+, no (un|una|el|la|de|por|lo|a|con) [a-zá-úñ]" \
  "$f")"; done

# Regla 3, viñetas del CHANGELOG de más de 60 palabras, con sus líneas de continuación.
awk '/^- /{if(b)print w; b=1; w=NF; next} /^  [^ ]/ && b{w+=NF; next}
     {if(b)print w; b=0} END{if(b)print w}' CHANGELOG.md | awk '$1>60' | wc -l

# Regla 4, encabezados con paréntesis, por archivo.
grep -cE "^#{1,4} .*\(.*\)" $C

# Regla 6, anclas a número de línea.
grep -oE "\b[A-Za-z_./-]+\.[a-z]{1,4}:[0-9]+" $C | cut -d: -f1 | sort | uniq -c

# Reglas 7 y 8, un solo extractor. Descarta bloques de código, tablas, encabezados, citas,
# viñetas y sus continuaciones, y se queda con párrafos de más de 15 palabras.
python3 - $C <<'EOF'
import io,re,sys
P,tot=[],0
for f in sys.argv[1:]:
    out,inc=[],False
    for l in io.open(f,encoding='utf-8').read().split('\n'):
        if l.strip().startswith('```'): inc=not inc; out.append(''); continue
        if inc or re.match(r'^\s*[|#>]',l) or re.match(r'^\s*[-*+] ',l) \
           or re.match(r'^\s{2,}\S',l) or re.match(r'^\s*\d+\. ',l):
            out.append(''); continue
        out.append(l)
    ps=[p for p in re.split(r'\n\s*\n','\n'.join(out)) if len(p.split())>15]
    for p in ps:
        o=[len(x.split()) for x in re.split(r'(?<=[.:;!?])\s+',p) if x.strip()]
        if len(o)>5: tot+=1; print(f"regla 7: {f}: {len(o)} oraciones")
        if len(o)>=3: P.append(o)
t=sum(len(o)-2 for o in P)
pl=sum(1 for o in P for i in range(len(o)-2) if max(o[i:i+3])-min(o[i:i+3])<=3)
print(f"regla 7: {tot} párrafos de más de cinco oraciones")
print(f"regla 8: {pl} planas de {t} ternas = {100*pl/max(t,1):.1f}%")
EOF
```

**Líneas base, medidas el 2026-10-01** con el bloque de arriba, sobre 11 archivos y 7891 palabras.
Se midieron en el PR que siembra los documentos.

| Regla | Línea base |
|---|---|
| 1. Intensificadores | 0 apariciones |
| 2. Paralelismo contrastivo | 0 casos en cada uno de los 11 archivos |
| 3. Viñetas del CHANGELOG de más de 60 palabras | 0 |
| 4. Encabezados con paréntesis | 0 en cada uno de los 11 archivos |
| 5. "No verificado" | sin comando: se revisa a mano |
| 6. Anclas a número de línea | 0 |
| 7. Párrafos de más de cinco oraciones | 0 |
| 8. Ternas planas | 0 de 54 ternas, 0,0 % |

El conteo de la regla 2 depende de su expresión regular. Si se ajusta, se recalculan todos los
archivos de una vez y se reescribe la línea base con su fecha nueva.

## 10. Alcance de escritura y vendoreo

- **Este repo es el único destino de escritura.** Otro repo clonado en la sesión es solo lectura:
  se copia desde él, nunca se escribe en él. Ante la duda, se para y se pregunta.
- **Lo que se copia de terceros**, fuera del código, viaja con su LICENSE y su atribución en la
  misma carpeta. Si la fuente no la trae, se para y se avisa antes del commit.
- **No se copia ni se traduce código de proyectos externos.** Se pueden consultar proyectos con una
  licencia de `docs/DESIGN.md`, "Licencias de las dependencias", y cada consulta se registra como
  entrada de `docs/DECISIONS.md` con URL, licencia, fecha e idea tomada. No se abre código GPL,
  LGPL o AGPL, ni descompilaciones de juegos. Decisión: 2026-10-01 "No se copia código de
  proyectos externos".
