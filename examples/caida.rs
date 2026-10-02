//! Fase 4: el cubo cae con gravedad, con la simulación a paso fijo, separada del dibujo. Tras 1 s
//! simulado imprime cuánto bajó y termina con código 0.
//!
//! Uso: `cargo run --example caida -- [--backend todos|gl|vulkan] [--hz N] [--limite-fps N]`
//!
//! `--hz` es la frecuencia del paso, 60 por defecto. `--limite-fps` topea los cuadros dibujados por
//! segundo. Con cualquier tope, lo que baja el cubo tiene que ser la misma cifra.

use std::process::ExitCode;
use std::time::Duration;

use sloprush::camara::Camara;
use sloprush::cubo::Cubo;
use sloprush::dibujo::Destino;
use sloprush::gpu::Gpu;
use sloprush::simulacion::{self, Cuerpo, GRAVEDAD, Simulacion};
use sloprush::ventana::{self, Escena, Opciones};
use sloprush::{VERSION, registro, wgpu};

const ALTURA_INICIAL: f32 = 2.0;

struct EscenaCaida {
    cubo: Option<Cubo>,
    camara: Camara,
    simulacion: Simulacion,
    cuadros: u64,
}

impl Escena for EscenaCaida {
    fn preparar(&mut self, gpu: &Gpu, formato: wgpu::TextureFormat) {
        self.cubo = Some(Cubo::nuevo(gpu, formato));
    }

    fn avanzar(&mut self, transcurrido: Duration) -> bool {
        let objetivo = u64::from(self.simulacion.frecuencia());
        self.simulacion.avanzar(transcurrido, objetivo);
        if self.simulacion.pasos() < objetivo {
            return true;
        }
        let bajada = ALTURA_INICIAL - self.simulacion.cuerpo.altura;
        let continua = GRAVEDAD / 2.0;
        registro::resultado(format!(
            "bajó {bajada:.6} m en {} pasos, 1 s simulado a {} Hz",
            self.simulacion.pasos(),
            self.simulacion.frecuencia()
        ));
        registro::resultado(format!(
            "desvío de {continua:.3} m: {:.2} %",
            100.0 * (bajada - continua).abs() / continua
        ));
        registro::resultado(format!("{} cuadros dibujados", self.cuadros));
        false
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
            let centro = [0.0, self.simulacion.cuerpo.altura, 0.0];
            cubo.dibujar(gpu, destino, &self.camara, centro, fondo);
            self.cuadros += 1;
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let resultado = simulacion::separar_frecuencia(&args).and_then(|(hz, resto)| {
        let opciones = Opciones::desde_args("sloprush: caida", &resto)?;
        registro::arranque(format!("sloprush {VERSION}, ejemplo caida"));
        registro::arranque(format!(
            "simulación a paso fijo de {hz} Hz; tope de dibujo: {}",
            opciones
                .limite_fps
                .map_or("sin tope".to_string(), |n| format!(
                    "{n} cuadros por segundo"
                ))
        ));
        let mut escena = EscenaCaida {
            cubo: None,
            // De frente y lejos, para ver toda la caída.
            camara: Camara::mirando([0.0, -0.5, 9.0], [0.0, -0.5, 0.0]),
            simulacion: Simulacion::nueva(hz, Cuerpo::en_reposo(ALTURA_INICIAL)),
            cuadros: 0,
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
