# ARCHITECTURE.md: qué es el código hoy

> **Rol:** descriptivo. Dice qué es el código hoy, y cada sección nombra la fecha en que se
> comprobó contra él. Si contradice al código, el código gana y este archivo es el que está mal.
> Cómo tiene que escribirse el código es `docs/DESIGN.md`. **Régimen:** se corrige, y un hueco se
> borra en el PR que lo cierra. **Origen:** plantilla 1.0.

Una sección se describe cuando un PR de código toca su área, no antes. Lo que falta describir se
lista en la sección 5.

## 1. El repo, archivo por archivo

Comprobado el AAAA-MM-DD contra `git ls-files`.

<Un renglón por archivo o carpeta: qué es.>

## 2. Dónde vive cada cosa

<Rutas en disco, almacenamiento, configuración. Una tabla: qué, variable o símbolo, ruta.>

## 3. Recursos y cómo se reinicia cada uno

<Todo lo que el programa crea, quién lo crea y cómo se limpia o se regenera.>

## 4. Tamaño

<Líneas, funciones, módulos. Cada número con el comando que lo recalcula, y la fecha.>

## 5. Sin describir todavía

<Las áreas del código que nadie describió. Se listan con un comando, para que la lista no
envejezca.>

## 6. Huecos conocidos

Lo que está abierto o no se verificó contra datos reales. Cada hueco abre con su fecha y trae el
comando o la corrida que lo muestra, para que el próximo lector compruebe si sigue abierto. Se borra
en el PR que lo cierra, y el CHANGELOG guarda el rastro. Un PR que anuncia algo en el CHANGELOG mira
si eso cierra un hueco de esta lista.

<AAAA-MM-DD: qué está abierto. Comando o corrida que lo muestra.>
