//! Fase 2: dibuja un sprite 2D con textura en el centro de la ventana.
//!
//! Uso: `cargo run --example sprite -- [--backend todos|gl|vulkan] [--cuadros N]`

use std::process::ExitCode;

use sloprush::gpu::Gpu;
use sloprush::sprite::Sprite;
use sloprush::ventana::{self, Escena, Opciones};
use sloprush::{VERSION, registro, wgpu};

#[derive(Default)]
struct EscenaSprite {
    sprite: Option<Sprite>,
}

impl Escena for EscenaSprite {
    fn preparar(&mut self, gpu: &Gpu, formato: wgpu::TextureFormat) {
        self.sprite = Some(Sprite::nuevo(gpu, formato));
    }

    fn dibujar(&mut self, gpu: &Gpu, destino: &wgpu::TextureView, ancho: u32, alto: u32) {
        if let Some(sprite) = &self.sprite {
            // Un cuadrado de la mitad del alto de la ventana, sin deformarse con su proporción.
            let medio_alto = 0.5;
            let medio_ancho = medio_alto * alto as f32 / ancho.max(1) as f32;
            sprite.ubicar(gpu, [0.0, 0.0], [medio_ancho, medio_alto]);
            let fondo = wgpu::Color {
                r: 0.05,
                g: 0.07,
                b: 0.2,
                a: 1.0,
            };
            sprite.dibujar(gpu, destino, fondo);
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let resultado = Opciones::desde_args("sloprush: sprite", &args).and_then(|opciones| {
        registro::arranque(format!("sloprush {VERSION}, ejemplo sprite"));
        ventana::correr(opciones, &mut EscenaSprite::default())
    });
    match resultado {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            registro::error(e);
            ExitCode::FAILURE
        }
    }
}
