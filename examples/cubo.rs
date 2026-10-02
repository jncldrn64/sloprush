//! Fase 3: dibuja un cubo 3D en perspectiva. W, A, S, D, las flechas, R y F mueven la cámara.
//!
//! Uso: `cargo run --example cubo -- [--backend todos|gl|vulkan] [--cuadros N]`

use std::process::ExitCode;

use sloprush::camara::Camara;
use sloprush::cubo::Cubo;
use sloprush::dibujo::Destino;
use sloprush::gpu::Gpu;
use sloprush::ventana::{self, Escena, Opciones};
use sloprush::winit::keyboard::KeyCode;
use sloprush::{VERSION, registro, wgpu};

struct EscenaCubo {
    cubo: Option<Cubo>,
    camara: Camara,
}

impl Escena for EscenaCubo {
    fn preparar(&mut self, gpu: &Gpu, formato: wgpu::TextureFormat) {
        self.cubo = Some(Cubo::nuevo(gpu, formato));
    }

    fn tecla(&mut self, tecla: KeyCode) {
        if self.camara.mover(tecla) {
            let [x, y, z] = self.camara.posicion;
            registro::resultado(format!("cámara en ({x:.2}, {y:.2}, {z:.2})"));
        }
    }

    fn dibujar(&mut self, gpu: &Gpu, destino: &wgpu::TextureView, ancho: u32, alto: u32) {
        if let Some(cubo) = self.cubo.as_mut() {
            let fondo = wgpu::Color {
                r: 0.05,
                g: 0.07,
                b: 0.2,
                a: 1.0,
            };
            let destino = Destino {
                vista: destino,
                ancho,
                alto,
            };
            cubo.dibujar(gpu, destino, &self.camara, [0.0; 3], fondo);
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let resultado = Opciones::desde_args("sloprush: cubo", &args).and_then(|opciones| {
        registro::arranque(format!("sloprush {VERSION}, ejemplo cubo"));
        let mut escena = EscenaCubo {
            cubo: None,
            // De arriba y a un costado, para que se vean tres caras.
            camara: Camara::mirando([1.6, 1.2, 3.2], [0.0; 3]),
        };
        ventana::correr(opciones, &mut escena)
    });
    match resultado {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            registro::error(e);
            ExitCode::FAILURE
        }
    }
}
