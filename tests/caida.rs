//! Fase 4: la caída del cubo no depende de cuántos cuadros se dibujan, y a 60 Hz se aleja menos de
//! 2 % de la fórmula continua.
//!
//! Corre sin GPU: simula la sucesión de tiempos de cuadro que da un dibujo limitado a 30 y a 240
//! cuadros por segundo.

use std::time::Duration;

use sloprush::simulacion::{Cuerpo, FRECUENCIA_POR_DEFECTO, GRAVEDAD, Simulacion};

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
