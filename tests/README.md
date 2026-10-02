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
- Pruebas unitarias de `src/ventana.rs`: las opciones `--cuadros` y `--limite-fps`.
- `tests/ventana.sh gl` y `tests/ventana.sh vulkan`: abren el ejemplo `ventana` en un Xvfb, le
  aprietan dos teclas con xdotool, piden el cierre con `tests/cerrar_ventana.py` y revisan la
  salida. Piden Xvfb, xdotool y python3, y no corren con `cargo test`. Salen del criterio de
  aceptación de la fase 1.
- Pruebas unitarias de `src/lienzo.rs` y `src/sprite.rs`: la tolerancia de color, el orden de los
  píxeles leídos y el damero.
- `tests/sprite.rs`: dibuja el sprite en un lienzo de 64 por 64 con GL y con Vulkan y comprueba el
  fondo y las casillas del damero, píxel por píxel. Sale del criterio de aceptación de la fase 2.
- Pruebas unitarias de `src/matematica.rs`, `src/camara.rs` y `src/cubo.rs`: vista, perspectiva,
  traslación, el paso de cada tecla y la malla del cubo.
- `tests/cubo.rs`: dibuja el cubo con GL y con Vulkan desde varias posiciones de cámara y comprueba
  qué cara se ve y dónde. Sale del criterio de aceptación de la fase 3.
