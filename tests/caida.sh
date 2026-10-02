#!/bin/sh
# Fase 4 con ventana: corre el ejemplo caida en un Xvfb con el dibujo limitado a 30 y a 240 cuadros
# por segundo y comprueba que todas las corridas impriman la misma bajada y terminen con código 0.
# Sin argumento corre GL y Vulkan, y así compara el estado de juego entre los dos niveles.
# Pide Xvfb. Uso: tests/caida.sh [gl|vulkan]. Sale con 0 si todo dio sí.
set -eu
cargo build -q --example caida
bajada() {
  xvfb-run -a -s '-screen 0 1280x720x24' ./target/debug/examples/caida --backend "$1" \
    --limite-fps "$2" | grep '^\[resultado\] bajó'
}
referencia=""
for backend in ${1:-gl vulkan}; do
  for tope in 30 240; do
    linea="$(bajada "$backend" "$tope")"
    if [ -z "$linea" ]; then
      echo "no: $backend con tope $tope no imprimió la bajada"
      exit 1
    fi
    if [ -z "$referencia" ]; then
      referencia="$linea"
    elif [ "$linea" != "$referencia" ]; then
      echo "no: $backend con tope $tope dio \"$linea\", y antes \"$referencia\""
      exit 1
    fi
  done
done
echo "sí: ${1:-gl y vulkan}, topes 30 y 240: $referencia"
