# tests/: cómo se corren los tests

Este README solo describe esta carpeta, así que no es un documento canónico ni pide permiso. Cómo
se escriben los tests es `docs/DESIGN.md`.

## 1. Correr todo

`cargo test`, desde la raíz. Al final de cada suite cargo imprime `test result: ok`, o el nombre
de la prueba que falló.

Las pruebas que tocan la GPU piden un adaptador OpenGL ES y uno Vulkan, uno por nivel gráfico. En un
equipo sin GPU alcanzan los de Mesa por software, que en Ubuntu 24.04 vienen en los paquetes
libegl1, libegl-mesa0 y mesa-vulkan-drivers. Si falta un adaptador, la prueba que lo pide falla y
dice cuál.

## 2. Las suites

- Pruebas unitarias de `src/gpu.rs`: la detección de un renderizador por software y la opción
  `--backend`. Salen de `docs/DESIGN.md`, "Presentación y registro".
- `tests/arranque.rs`: el motor consigue dispositivo con GL y con Vulkan, sin ventana. Sale del
  criterio de aceptación de la fase 0.
