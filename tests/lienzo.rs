//! El lienzo se lee entero aunque su ancho no llene una fila de copia de 256 bytes.
//!
//! Las pruebas de las fases 2 a 4 usan lienzos de 64 píxeles de ancho, 256 bytes por fila, justo
//! la alineación de copia de wgpu. Esta usa 50 por 30: cada fila trae relleno y `Lienzo::leer`
//! lo tiene que saltar.

use sloprush::dibujo;
use sloprush::gpu::{self, Eleccion};
use sloprush::lienzo::{Lienzo, parecido, pixel};
use sloprush::wgpu;

const ANCHO: u32 = 50;
const ALTO: u32 = 30;
const ROJO: [u8; 4] = [255, 0, 0, 255];

fn comprobar(eleccion: Eleccion) {
    let instancia = gpu::crear_instancia(eleccion, None);
    let gpu =
        gpu::iniciar(instancia, None).unwrap_or_else(|e| panic!("sin GPU con {eleccion:?}: {e}"));
    let lienzo = Lienzo::nuevo(&gpu, ANCHO, ALTO, wgpu::TextureFormat::Rgba8UnormSrgb);
    dibujo::limpiar(&gpu, &lienzo.vista, wgpu::Color::RED);
    let p = lienzo.leer(&gpu).expect("lectura del lienzo");
    assert_eq!(p.len(), (ANCHO * ALTO) as usize);
    for y in 0..ALTO {
        for x in 0..ANCHO {
            let color = pixel(&p, ANCHO, x, y);
            assert!(parecido(color, ROJO, 0), "({x},{y}): {color:?}");
        }
    }
}

#[test]
fn lienzo_sin_alinear_en_el_nivel_base_con_gl() {
    comprobar(Eleccion::Gl);
}

#[test]
fn lienzo_sin_alinear_en_el_nivel_completo_con_vulkan() {
    comprobar(Eleccion::Vulkan);
}
