//! Fase 0: el motor consigue adaptador y dispositivo en cada nivel gráfico, sin ventana.
//!
//! Necesitan un adaptador de cada backend. Si falta uno, la prueba falla con el motivo.

use sloprush::gpu::{self, Eleccion};
use sloprush::wgpu;

fn arranca(eleccion: Eleccion, esperado: wgpu::Backend) {
    let instancia = gpu::crear_instancia(eleccion, None);
    let gpu =
        gpu::iniciar(instancia, None).unwrap_or_else(|e| panic!("sin GPU con {eleccion:?}: {e}"));
    assert_eq!(gpu.info.backend, esperado);
}

#[test]
fn arranca_en_el_nivel_base_con_gl() {
    arranca(Eleccion::Gl, wgpu::Backend::Gl);
}

#[test]
fn arranca_en_el_nivel_completo_con_vulkan() {
    arranca(Eleccion::Vulkan, wgpu::Backend::Vulkan);
}
