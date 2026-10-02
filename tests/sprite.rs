//! Fase 2: el sprite se dibuja donde tiene que estar, con su textura, en cada nivel gráfico.
//!
//! Dibuja en un lienzo de 64 por 64 sin ventana y lee los píxeles de vuelta de la GPU.

use sloprush::gpu::{self, Eleccion};
use sloprush::lienzo::{Lienzo, parecido, pixel};
use sloprush::sprite::{COLOR_A, COLOR_B, Sprite};
use sloprush::wgpu;

const LADO: u32 = 64;
const NEGRO: [u8; 4] = [0, 0, 0, 255];
const TOLERANCIA: u8 = 2;

fn dibujar_y_leer(eleccion: Eleccion) -> Vec<[u8; 4]> {
    let instancia = gpu::crear_instancia(eleccion, None);
    let gpu =
        gpu::iniciar(instancia, None).unwrap_or_else(|e| panic!("sin GPU con {eleccion:?}: {e}"));
    let formato = wgpu::TextureFormat::Rgba8UnormSrgb;
    let lienzo = Lienzo::nuevo(&gpu, LADO, LADO, formato);
    let sprite = Sprite::nuevo(&gpu, formato);
    // Centrado y con medio tamaño 0,5: ocupa las columnas y filas de 16 a 47.
    sprite.ubicar(&gpu, [0.0, 0.0], [0.5, 0.5]);
    sprite.dibujar(&gpu, &lienzo.vista, wgpu::Color::BLACK);
    lienzo.leer(&gpu).expect("lectura del lienzo")
}

fn comprobar(pixeles: &[[u8; 4]]) {
    let en = |x, y| pixel(pixeles, LADO, x, y);
    for (x, y) in [(0, 0), (63, 63), (14, 32), (32, 14)] {
        assert!(
            parecido(en(x, y), NEGRO, TOLERANCIA),
            "fondo en ({x},{y}): {:?}",
            en(x, y)
        );
    }
    // Cada casilla del damero ocupa 8 píxeles: la columna 20 cae en la casilla 0 y la 28 en la 1.
    assert!(
        parecido(en(20, 20), COLOR_A, TOLERANCIA),
        "casilla 0: {:?}",
        en(20, 20)
    );
    assert!(
        parecido(en(28, 20), COLOR_B, TOLERANCIA),
        "casilla 1: {:?}",
        en(28, 20)
    );
    assert!(
        parecido(en(20, 28), COLOR_B, TOLERANCIA),
        "fila 1: {:?}",
        en(20, 28)
    );
    assert!(
        parecido(en(44, 44), COLOR_A, TOLERANCIA),
        "esquina opuesta: {:?}",
        en(44, 44)
    );
}

#[test]
fn sprite_en_el_nivel_base_con_gl() {
    comprobar(&dibujar_y_leer(Eleccion::Gl));
}

#[test]
fn sprite_en_el_nivel_completo_con_vulkan() {
    comprobar(&dibujar_y_leer(Eleccion::Vulkan));
}
