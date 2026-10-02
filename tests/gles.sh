#!/bin/sh
# El nivel base por OpenGL ES, sin pantalla. Con EGL, el backend GL de wgpu 30.0.1 pide OpenGL 3.3
# de escritorio si el driver lo ofrece, y Mesa le da 4.5; si no, abre OpenGL ES.
# MESA_GL_VERSION_OVERRIDE=3.1 hace que Mesa no lo ofrezca, y MESA_GLES_VERSION_OVERRIDE=3.0 hace
# que el OpenGL ES se anuncie como 3.0, el piso de la tabla de wgpu. Se parece a un OpenGL ES 3.0
# sin serlo: docs/DECISIONS.md, 2026-10-02 "El nivel base también se prueba sobre OpenGL ES".
# Corre las pruebas que terminan en _con_gl y comprueba que todas abrieron OpenGL ES 3.0.
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
echo "sí: las $total pruebas con GL corrieron sobre OpenGL ES 3.0"
