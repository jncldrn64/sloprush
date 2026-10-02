//! Fase 3: el cubo se dibuja en perspectiva, con profundidad, y la cámara lo mueve con el teclado.
//!
//! Dibuja en un lienzo de 64 por 64 sin ventana y lee los píxeles de vuelta de la GPU.

use sloprush::camara::Camara;
use sloprush::cubo::{CARA_MAS_X, CARA_MAS_Z, COLORES, Cubo};
use sloprush::dibujo::Destino;
use sloprush::gpu::{self, Eleccion, Gpu};
use sloprush::lienzo::{Lienzo, parecido, pixel};
use sloprush::wgpu;
use sloprush::winit::keyboard::KeyCode;

const LADO: u32 = 64;
const NEGRO: [u8; 4] = [0, 0, 0, 255];
const TOLERANCIA: u8 = 2;
const ORIGEN: [f32; 3] = [0.0, 0.0, 0.0];

struct Banco {
    gpu: Gpu,
    lienzo: Lienzo,
    cubo: Cubo,
}

impl Banco {
    fn nuevo(eleccion: Eleccion) -> Self {
        let instancia = gpu::crear_instancia(eleccion, None);
        let gpu = gpu::iniciar(instancia, None)
            .unwrap_or_else(|e| panic!("sin GPU con {eleccion:?}: {e}"));
        let formato = wgpu::TextureFormat::Rgba8UnormSrgb;
        let lienzo = Lienzo::nuevo(&gpu, LADO, LADO, formato);
        let cubo = Cubo::nuevo(&gpu, formato);
        Banco { gpu, lienzo, cubo }
    }

    fn ver(&mut self, camara: &Camara) -> Vec<[u8; 4]> {
        let destino = Destino {
            vista: &self.lienzo.vista,
            ancho: LADO,
            alto: LADO,
        };
        self.cubo
            .dibujar(&self.gpu, destino, camara, ORIGEN, wgpu::Color::BLACK);
        self.lienzo.leer(&self.gpu).expect("lectura del lienzo")
    }
}

fn comprobar(eleccion: Eleccion) {
    let mut banco = Banco::nuevo(eleccion);
    let en = |p: &[[u8; 4]], x, y| pixel(p, LADO, x, y);

    // De frente: se ve la cara +z. La -z se dibuja después; sin profundidad, taparía a la +z.
    let mut camara = Camara::mirando([0.0, 0.0, 3.0], ORIGEN);
    let p = banco.ver(&camara);
    assert!(
        parecido(en(&p, 32, 32), COLORES[CARA_MAS_Z], TOLERANCIA),
        "de frente: {:?}",
        en(&p, 32, 32)
    );
    for (x, y) in [(0, 0), (63, 0), (0, 63), (63, 63)] {
        assert!(
            parecido(en(&p, x, y), NEGRO, TOLERANCIA),
            "fondo en ({x},{y}): {:?}",
            en(&p, x, y)
        );
    }

    // Desde la derecha: se ve la cara +x.
    let p = banco.ver(&Camara::mirando([3.0, 0.0, 0.0], ORIGEN));
    assert!(
        parecido(en(&p, 32, 32), COLORES[CARA_MAS_X], TOLERANCIA),
        "de costado: {:?}",
        en(&p, 32, 32)
    );

    // Cuatro pasos a la derecha con D: el cubo queda a la izquierda y el centro, vacío.
    for _ in 0..4 {
        assert!(camara.mover(KeyCode::KeyD));
    }
    let p = banco.ver(&camara);
    assert!(
        parecido(en(&p, 32, 32), NEGRO, TOLERANCIA),
        "centro tras D: {:?}",
        en(&p, 32, 32)
    );
    assert!(
        parecido(en(&p, 10, 32), COLORES[CARA_MAS_Z], TOLERANCIA),
        "izquierda tras D: {:?}",
        en(&p, 10, 32)
    );

    // Ocho pasos más: el cubo sale de la vista.
    for _ in 0..8 {
        camara.mover(KeyCode::ArrowRight);
    }
    let p = banco.ver(&camara);
    assert!(
        p.iter().all(|c| parecido(*c, NEGRO, TOLERANCIA)),
        "el cubo sigue a la vista"
    );
}

#[test]
fn cubo_en_el_nivel_base_con_gl() {
    comprobar(Eleccion::Gl);
}

#[test]
fn cubo_en_el_nivel_completo_con_vulkan() {
    comprobar(Eleccion::Vulkan);
}
