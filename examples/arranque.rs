//! Fase 0: arranca la GPU, imprime los adaptadores, el elegido y su backend, y cierra.
//!
//! Uso: `cargo run --example arranque -- [--backend todos|gl|vulkan]`

use std::process::ExitCode;

use sloprush::{VERSION, gpu, registro};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match correr(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            registro::error(e);
            ExitCode::FAILURE
        }
    }
}

fn correr(args: &[String]) -> Result<(), String> {
    let (eleccion, resto) = gpu::separar_backend(args)?;
    if let Some(sobrante) = resto.first() {
        return Err(format!("opción desconocida: {sobrante}"));
    }
    registro::arranque(format!("sloprush {VERSION}, ejemplo arranque"));
    let instancia = gpu::crear_instancia(eleccion, None);
    gpu::listar_adaptadores(&instancia, eleccion);
    let gpu = gpu::iniciar(instancia, None).map_err(|e| e.to_string())?;
    drop(gpu);
    registro::cierre("dispositivo liberado, fin del ejemplo arranque");
    Ok(())
}
