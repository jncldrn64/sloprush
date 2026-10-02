#!/bin/sh
# Fase 1 sin mirar la pantalla: abre el ejemplo ventana en un Xvfb, le aprieta a y b con XTEST,
# pide el cierre como un gestor de ventanas y revisa el código de salida y lo impreso.
# Pide Xvfb, xdotool y python3. Uso: tests/ventana.sh [gl|vulkan]. Sale con 0 si todo dio sí.
set -eu
backend="${1:-vulkan}"
salida="$(mktemp)"
trap 'rm -f "$salida"' EXIT
cargo build -q --example ventana
xvfb-run -a -s '-screen 0 1280x720x24' sh -c '
  ./target/debug/examples/ventana --backend "$1" > "$2" 2>&1 &
  pid=$!
  id=$(xdotool search --sync --name "sloprush: ventana" | head -1)
  sleep 1
  xdotool windowfocus --sync "$id"
  xdotool key a b
  sleep 1
  python3 tests/cerrar_ventana.py "$id"
  wait "$pid"' sh "$backend" "$salida"
for esperado in 'tecla Code(KeyA)' 'tecla Code(KeyB)' '[cierre] ventana cerrada' 'backend elegido'; do
  if ! grep -qF "$esperado" "$salida"; then
    echo "no: falta \"$esperado\" en la salida"
    cat "$salida"
    exit 1
  fi
done
echo "sí: ventana con $backend, teclas impresas y cierre con código 0"
