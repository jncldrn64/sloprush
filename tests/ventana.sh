#!/bin/sh
# Fase 1 sin mirar la pantalla: abre el ejemplo ventana en un Xvfb, le aprieta a y b con XTEST,
# pide el cierre como un gestor de ventanas y revisa el código de salida y lo impreso.
# Pide Xvfb, xdotool y python3. Uso: tests/ventana.sh [gl|vulkan]. Sale con 0 si todo dio sí.
set -eu
backend="${1:-vulkan}"
salida="$(mktemp)"
trap 'rm -f "$salida"' EXIT
cargo build -q --example ventana
# Adentro del Xvfb: si la ventana no aparece en 20 s, o si el ejemplo ya terminó, es no.
if ! xvfb-run -a -s '-screen 0 1280x720x24' sh -c '
  ./target/debug/examples/ventana --backend "$1" > "$2" 2>&1 &
  pid=$!
  id=$(timeout 20 xdotool search --sync --name "sloprush: ventana" | head -1)
  if [ -z "$id" ] || ! kill -0 "$pid" 2>/dev/null; then
    echo "no: la ventana no apareció"
    exit 1
  fi
  sleep 1
  xdotool windowfocus --sync "$id"
  xdotool key a b
  sleep 1
  python3 tests/cerrar_ventana.py "$id"
  if ! wait "$pid"; then
    echo "no: el ejemplo terminó con un código distinto de 0"
    exit 1
  fi' sh "$backend" "$salida"; then
  cat "$salida"
  exit 1
fi
for esperado in 'tecla Code(KeyA)' 'tecla Code(KeyB)' '[cierre] ventana cerrada' \
  'backend elegido'; do
  if ! grep -qF "$esperado" "$salida"; then
    echo "no: falta \"$esperado\" en la salida"
    cat "$salida"
    exit 1
  fi
done
echo "sí: ventana con $backend, teclas impresas y cierre con código 0"
