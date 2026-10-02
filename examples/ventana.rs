//! Fase 1: abre una ventana, la limpia en cada cuadro e imprime cada tecla que se aprieta.
//!
//! Uso: `cargo run --example ventana -- [--backend todos|gl|vulkan] [--cuadros N]`
//!
//! Termina con código 0 al cerrar la ventana, o tras N cuadros en el modo de cuadros fijos.

use std::process::ExitCode;

use sloprush::gpu::Gpu;
use sloprush::ventana::{self, Escena, Opciones};
use sloprush::{VERSION, dibujo, registro, wgpu};

struct Fondo;

impl Escena for Fondo {
    fn dibujar(&mut self, gpu: &Gpu, destino: &wgpu::TextureView, _ancho: u32, _alto: u32) {
        let azul_oscuro = wgpu::Color {
            r: 0.05,
            g: 0.07,
            b: 0.2,
            a: 1.0,
        };
        dibujo::limpiar(gpu, destino, azul_oscuro);
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let resultado = Opciones::desde_args("sloprush: ventana", &args).and_then(|opciones| {
        registro::arranque(format!("sloprush {VERSION}, ejemplo ventana"));
        ventana::correr(opciones, &mut Fondo)
    });
    match resultado {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            registro::error(e);
            ExitCode::FAILURE
        }
    }
}
