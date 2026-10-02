#!/bin/sh
# El nivel base por OpenGL ES, sin pantalla. Con Mesa, el backend GL de wgpu 30.0.1 abre OpenGL 3.3
# de escritorio si el driver lo ofrece, y OpenGL ES si no. MESA_GL_VERSION_OVERRIDE=3.1 hace que
# no lo ofrezca, y MESA_GLES_VERSION_OVERRIDE=3.0 hace que el OpenGL ES se anuncie como 3.0, el
# piso de la tabla de wgpu. wgpu se limita a lo que anuncia esa versión; Mesa no le quita nada.
# Corre las pruebas que dibujan con GL y comprueba que todas abrieron OpenGL ES 3.0.
# Pide Mesa. Uso: tests/gles.sh. Sale con 0 si todo dio sí.
set -eu
salida="$(mktemp)"
trap 'rm -f "$salida"' EXIT
export MESA_GL_VERSION_OVERRIDE=3.1 MESA_GLES_VERSION_OVERRIDE=3.0
if ! cargo test -q -- con_gl --nocapture --test-threads=1 > "$salida" 2>&1; then
  echo "no: alguna prueba con GL falló sobre OpenGL ES"
  cat "$salida"
  exit 1
fi
total="$(grep -c 'adaptador elegido: .*backend Gl ' "$salida" || true)"
es="$(grep -c 'adaptador elegido: .*backend Gl .*driver .*OpenGL ES 3\.0 ' "$salida" || true)"
if [ "$total" -eq 0 ] || [ "$es" -ne "$total" ]; then
  echo "no: $es de $total adaptadores GL abrieron OpenGL ES 3.0"
  grep 'adaptador elegido' "$salida" || true
  exit 1
fi
echo "sí: las $total pruebas con GL dibujaron sobre OpenGL ES 3.0"
