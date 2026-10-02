//! Fase 4: la caída del cubo no depende de cuántos cuadros se dibujan, y a 60 Hz se aleja menos de
//! 2 % de la fórmula continua.
//!
//! Las pruebas de la simulación corren sin GPU: simulan la sucesión de tiempos de cuadro que da un
//! dibujo limitado a 30 y a 240 cuadros por segundo. Las dos últimas dibujan el cubo a la altura
//! de la simulación en un lienzo de 64 por 64, con la cámara del ejemplo `caida`, y leen los
//! píxeles de vuelta. Prueban las piezas de la biblioteca que usa el ejemplo, no el ejemplo.

use std::time::Duration;

use sloprush::camara::Camara;
use sloprush::cubo::{CARA_MAS_Z, COLORES, Cubo};
use sloprush::dibujo::Destino;
use sloprush::gpu::{self, Eleccion};
use sloprush::lienzo::{Lienzo, parecido, pixel};
use sloprush::simulacion::{Cuerpo, FRECUENCIA_POR_DEFECTO, GRAVEDAD, Simulacion};
use sloprush::wgpu;

/// Lo que baja el cuerpo en 1 s simulado a `hz`, con un cuadro dibujado cada `periodo`.
fn caida(hz: u32, periodo: Duration) -> f32 {
    let mut s = Simulacion::nueva(hz, Cuerpo::en_reposo(0.0));
    let objetivo = u64::from(hz);
    while s.pasos() < objetivo {
        s.avanzar(periodo, objetivo);
    }
    -s.cuerpo.altura
}

fn desvio(bajada: f32) -> f32 {
    let continua = GRAVEDAD / 2.0;
    (bajada - continua).abs() / continua
}

#[test]
fn con_el_dibujo_a_30_y_a_240_baja_exactamente_lo_mismo() {
    let a_30 = caida(FRECUENCIA_POR_DEFECTO, Duration::from_secs_f64(1.0 / 30.0));
    let a_240 = caida(FRECUENCIA_POR_DEFECTO, Duration::from_secs_f64(1.0 / 240.0));
    assert_eq!(
        a_30.to_bits(),
        a_240.to_bits(),
        "a 30: {a_30}, a 240: {a_240}"
    );
}

#[test]
fn el_mismo_tiempo_real_da_los_mismos_pasos_a_30_y_a_240() {
    // 2 s de tiempo real en cuadros de 1/30 s y de 1/240 s, sin tope de pasos.
    let pasos = |periodo: Duration, cuadros: u32| {
        let mut s = Simulacion::nueva(FRECUENCIA_POR_DEFECTO, Cuerpo::en_reposo(0.0));
        for _ in 0..cuadros {
            s.avanzar(periodo, u64::MAX);
        }
        s.pasos()
    };
    let a_30 = pasos(Duration::from_secs_f64(1.0 / 30.0), 60);
    let a_240 = pasos(Duration::from_secs_f64(1.0 / 240.0), 480);
    // Los cuadros redondeados a nanosegundos no suman justo 2 s: 1 999 999 980 ns a 30 y
    // 2 000 000 160 ns a 240, y 120 pasos piden 2 000 000 040 ns. Por eso dan 119 y 120.
    assert!(
        a_30.abs_diff(a_240) <= 1 && a_30.abs_diff(120) <= 1,
        "a 30: {a_30} pasos, a 240: {a_240}"
    );
}

#[test]
fn con_cuadros_irregulares_tambien_baja_lo_mismo() {
    let regular = caida(FRECUENCIA_POR_DEFECTO, Duration::from_millis(16));
    let mut s = Simulacion::nueva(FRECUENCIA_POR_DEFECTO, Cuerpo::en_reposo(0.0));
    let cuadros_ms = [3, 41, 7, 16, 90, 1, 25];
    let mut i = 0;
    while s.pasos() < 60 {
        s.avanzar(Duration::from_millis(cuadros_ms[i % cuadros_ms.len()]), 60);
        i += 1;
    }
    assert_eq!((-s.cuerpo.altura).to_bits(), regular.to_bits());
}

#[test]
fn a_60_hz_se_aleja_menos_de_2_por_ciento_de_4_905_m() {
    let bajada = caida(60, Duration::from_millis(16));
    assert!(
        desvio(bajada) < 0.02,
        "bajó {bajada} m, desvío {}",
        desvio(bajada)
    );
}

#[test]
fn el_desvio_es_1_sobre_n_con_n_pasos_por_segundo() {
    for hz in [30, 60, 144, 240] {
        let bajada = caida(hz, Duration::from_millis(1));
        let esperado = 1.0 / hz as f32;
        assert!(
            (desvio(bajada) - esperado).abs() < 1e-4,
            "a {hz} Hz: desvío {}, esperado {esperado}",
            desvio(bajada)
        );
    }
}

const LADO: u32 = 64;
const NEGRO: [u8; 4] = [0, 0, 0, 255];
const TOLERANCIA: u8 = 2;

/// Dibuja el cubo a la altura inicial del ejemplo `caida` y después de 1 s simulado, y comprueba
/// que bajó en la imagen: arriba al principio, abajo al final.
fn comprobar_en_pantalla(eleccion: Eleccion) {
    let instancia = gpu::crear_instancia(eleccion, None);
    let gpu =
        gpu::iniciar(instancia, None).unwrap_or_else(|e| panic!("sin GPU con {eleccion:?}: {e}"));
    let formato = wgpu::TextureFormat::Rgba8UnormSrgb;
    let lienzo = Lienzo::nuevo(&gpu, LADO, LADO, formato);
    let mut cubo = Cubo::nuevo(&gpu, formato);
    // La misma cámara que el ejemplo `caida`.
    let camara = Camara::mirando([0.0, -0.5, 9.0], [0.0, -0.5, 0.0]);
    let mut simulacion = Simulacion::nueva(FRECUENCIA_POR_DEFECTO, Cuerpo::en_reposo(2.0));
    let mut ver = |altura: f32| {
        let destino = Destino {
            vista: &lienzo.vista,
            ancho: LADO,
            alto: LADO,
        };
        cubo.dibujar(
            &gpu,
            destino,
            &camara,
            [0.0, altura, 0.0],
            wgpu::Color::BLACK,
        );
        lienzo.leer(&gpu).expect("lectura del lienzo")
    };
    // A 2 m, la cara +z cae cerca de la fila 9; a -2,99 m, cerca de la 54.
    let (arriba, abajo) = ((32, 9), (32, 54));
    let antes = ver(simulacion.cuerpo.altura);
    simulacion.avanzar(Duration::from_secs(2), u64::from(FRECUENCIA_POR_DEFECTO));
    let despues = ver(simulacion.cuerpo.altura);
    for (nombre, p, cubo_en, vacio_en) in [
        ("antes", &antes, arriba, abajo),
        ("después", &despues, abajo, arriba),
    ] {
        let color = pixel(p, LADO, cubo_en.0, cubo_en.1);
        assert!(
            parecido(color, COLORES[CARA_MAS_Z], TOLERANCIA),
            "{nombre}, cubo en {cubo_en:?}: {color:?}"
        );
        let color = pixel(p, LADO, vacio_en.0, vacio_en.1);
        assert!(
            parecido(color, NEGRO, TOLERANCIA),
            "{nombre}, fondo en {vacio_en:?}: {color:?}"
        );
    }
}

#[test]
fn cae_en_pantalla_en_el_nivel_base_con_gl() {
    comprobar_en_pantalla(Eleccion::Gl);
}

#[test]
fn cae_en_pantalla_en_el_nivel_completo_con_vulkan() {
    comprobar_en_pantalla(Eleccion::Vulkan);
}
