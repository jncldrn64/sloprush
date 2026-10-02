//! Vectores y matrices de 4 por 4 para la cámara y el cubo.
//!
//! Las matrices van por columnas, como las lee WGSL. El sistema es de mano derecha, con la cámara
//! mirando hacia z negativa, y la profundidad de recorte va de 0 a 1, como la pide wgpu.

/// Un vector de tres componentes.
pub type Vec3 = [f32; 3];

/// Una matriz de 4 por 4, columna por columna.
pub type Mat4 = [[f32; 4]; 4];

pub fn sumar(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn restar(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn escalar(a: Vec3, k: f32) -> Vec3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

pub fn punto(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn cruz(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn normalizar(a: Vec3) -> Vec3 {
    let largo = punto(a, a).sqrt();
    escalar(a, 1.0 / largo)
}

pub const IDENTIDAD: Mat4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

/// Una traslación por `v`.
pub fn trasladar(v: Vec3) -> Mat4 {
    let mut m = IDENTIDAD;
    m[3] = [v[0], v[1], v[2], 1.0];
    m
}

/// El producto `a * b`: aplicar `b` y después `a`.
pub fn multiplicar(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut r = [[0.0; 4]; 4];
    for (columna, salida) in r.iter_mut().enumerate() {
        for (fila, valor) in salida.iter_mut().enumerate() {
            *valor = (0..4).map(|k| a[k][fila] * b[columna][k]).sum();
        }
    }
    r
}

/// Aplica `m` al punto `p` y divide por w.
pub fn transformar_punto(m: &Mat4, p: Vec3) -> Vec3 {
    let v = [p[0], p[1], p[2], 1.0];
    let mut r = [0.0; 4];
    for (fila, salida) in r.iter_mut().enumerate() {
        *salida = (0..4).map(|k| m[k][fila] * v[k]).sum();
    }
    [r[0] / r[3], r[1] / r[3], r[2] / r[3]]
}

/// Perspectiva con campo vertical `fov_y` en radianes. Lleva z = -cerca a profundidad 0 y
/// z = -lejos a profundidad 1.
pub fn perspectiva(fov_y: f32, aspecto: f32, cerca: f32, lejos: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    let rango = cerca - lejos;
    [
        [f / aspecto, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, lejos / rango, -1.0],
        [0.0, 0.0, cerca * lejos / rango, 0.0],
    ]
}

/// La vista de una cámara en `ojo` que mira hacia `objetivo`, con `arriba` como vertical.
pub fn mirar(ojo: Vec3, objetivo: Vec3, arriba: Vec3) -> Mat4 {
    let f = normalizar(restar(objetivo, ojo));
    let s = normalizar(cruz(f, arriba));
    let u = cruz(s, f);
    [
        [s[0], u[0], -f[0], 0.0],
        [s[1], u[1], -f[1], 0.0],
        [s[2], u[2], -f[2], 0.0],
        [-punto(s, ojo), -punto(u, ojo), punto(f, ojo), 1.0],
    ]
}

/// La matriz como 64 bytes en little endian, para un buffer uniforme.
pub fn a_bytes(m: &Mat4) -> Vec<u8> {
    m.iter().flatten().flat_map(|v| v.to_le_bytes()).collect()
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn cerca_de(a: Vec3, b: Vec3) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    #[test]
    fn identidad_no_cambia_nada() {
        let m = mirar([1.0, 2.0, 3.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(multiplicar(&IDENTIDAD, &m), m);
        assert_eq!(multiplicar(&m, &IDENTIDAD), m);
    }

    #[test]
    fn la_vista_lleva_el_ojo_al_origen_y_el_objetivo_al_frente() {
        let m = mirar([0.0, 0.0, 3.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(cerca_de(
            transformar_punto(&m, [0.0, 0.0, 3.0]),
            [0.0, 0.0, 0.0]
        ));
        assert!(cerca_de(
            transformar_punto(&m, [0.0, 0.0, 0.0]),
            [0.0, 0.0, -3.0]
        ));
        assert!(cerca_de(
            transformar_punto(&m, [1.0, 0.0, 3.0]),
            [1.0, 0.0, 0.0]
        ));
    }

    #[test]
    fn trasladar_mueve_el_punto() {
        let m = trasladar([1.0, 2.0, 3.0]);
        assert!(cerca_de(
            transformar_punto(&m, [1.0, 1.0, 1.0]),
            [2.0, 3.0, 4.0]
        ));
    }

    #[test]
    fn la_perspectiva_lleva_cerca_a_0_y_lejos_a_1() {
        let p = perspectiva(1.0, 1.0, 0.1, 100.0);
        assert!((transformar_punto(&p, [0.0, 0.0, -0.1])[2]).abs() < 1e-5);
        assert!((transformar_punto(&p, [0.0, 0.0, -100.0])[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn lo_lejano_se_ve_mas_chico() {
        let p = perspectiva(1.0, 1.0, 0.1, 100.0);
        let cerca = transformar_punto(&p, [1.0, 0.0, -2.0])[0];
        let lejos = transformar_punto(&p, [1.0, 0.0, -8.0])[0];
        assert!(cerca > lejos && lejos > 0.0);
    }
}
