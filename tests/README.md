# tests/: cómo se corren los tests

Este README solo describe esta carpeta, así que no es un documento canónico ni pide permiso. Cómo
se escriben los tests es `docs/DESIGN.md`.

## 1. Correr todo

`cargo test`, desde la raíz. Al final de cada suite cargo imprime `test result: ok`, o el nombre
de la prueba que falló.

Las pruebas que tocan la GPU piden un adaptador del backend GL de wgpu y uno Vulkan, uno por nivel
gráfico. Con Mesa, el backend GL abre OpenGL de escritorio; `tests/gles.sh` corre las mismas
pruebas sobre OpenGL ES. En un equipo sin GPU alcanzan los adaptadores de Mesa por software, que en
Ubuntu 24.04 vienen en los paquetes libegl1, libegl-mesa0 y mesa-vulkan-drivers. Si falta un
adaptador, la prueba que lo pide falla y dice cuál.

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
- `tests/sprite.rs`: dibuja el sprite en un lienzo de 64 por 64 con GL y con Vulkan y comprueba
  cuatro puntos del fondo y cuatro de las 16 casillas del damero. Sale del criterio de aceptación
  de la fase 2.
- `tests/lienzo.rs`: limpia un lienzo de 50 por 30 con GL y con Vulkan y lo lee entero. Prueba
  el relleno de fila que los lienzos de 64 de ancho no tienen.
- Pruebas unitarias de `src/matematica.rs`, `src/camara.rs` y `src/cubo.rs`: vista, perspectiva,
  traslación, el paso de cada tecla y la malla del cubo.
- `tests/cubo.rs`: dibuja el cubo con GL y con Vulkan desde varias posiciones de cámara, movida con
  R y con D, y comprueba qué cara se ve y dónde, también arriba y abajo. Sale del criterio de
  aceptación de la fase 3.
- Pruebas unitarias de `src/simulacion.rs`: un paso, los pasos que caben en un tiempo, el tope y
  la opción `--hz`.
- `tests/caida.rs`: simula la caída sin GPU con cuadros de 1/30 s, de 1/240 s y de largos
  irregulares, y comprueba la igualdad bit a bit, los mismos pasos para el mismo tiempo real y el
  desvío de 1/n. Además dibuja el cubo con GL y con Vulkan antes y después de la caída y lo
  encuentra más abajo. Sale del criterio de aceptación de la fase 4.
- `tests/caida.sh`: corre el ejemplo `caida` en un Xvfb con tope de 30 y de 240 cuadros por
  segundo. Sin argumento corre GL y Vulkan, y exige código 0 y la misma bajada en las cuatro
  corridas. Pide Xvfb y no corre con `cargo test`.
- `tests/gles.sh`: corre con `cargo test` las pruebas que terminan en `_con_gl`, la de arranque y
  cuatro que dibujan, con OpenGL ES forzado por dos variables de Mesa, y exige que todas hayan
  abierto OpenGL ES 3.0. Pide Mesa y no corre
  con `cargo test`. Sale de `docs/DESIGN.md`, "Dos niveles gráficos".
