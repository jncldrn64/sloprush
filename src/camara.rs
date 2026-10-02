//! Cámara en perspectiva que se mueve con el teclado.
//!
//! Se traslada sin girar: la dirección en que mira queda fija. W y S, o arriba y abajo, avanzan y
//! retroceden; A y D, o izquierda y derecha, van de costado; R y F suben y bajan.

use winit::keyboard::KeyCode;

use crate::matematica::{self, Mat4, Vec3};

/// Cuánto avanza la cámara por tecla, en unidades del mundo.
pub const PASO: f32 = 0.25;

/// Una cámara en `posicion` que mira hacia `posicion + direccion`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camara {
    pub posicion: Vec3,
    pub direccion: Vec3,
}

const ARRIBA: Vec3 = [0.0, 1.0, 0.0];
const CAMPO_VERTICAL: f32 = std::f32::consts::FRAC_PI_4;
const CERCA: f32 = 0.1;
const LEJOS: f32 = 100.0;

impl Camara {
    /// Una cámara en `posicion` que mira hacia `objetivo`.
    pub fn mirando(posicion: Vec3, objetivo: Vec3) -> Self {
        Camara {
            posicion,
            direccion: matematica::normalizar(matematica::restar(objetivo, posicion)),
        }
    }

    /// Mueve la cámara un paso según la tecla. Devuelve si la tecla movía la cámara.
    pub fn mover(&mut self, tecla: KeyCode) -> bool {
        let costado = matematica::normalizar(matematica::cruz(self.direccion, ARRIBA));
        let desplazamiento = match tecla {
            KeyCode::KeyW | KeyCode::ArrowUp => self.direccion,
            KeyCode::KeyS | KeyCode::ArrowDown => matematica::escalar(self.direccion, -1.0),
            KeyCode::KeyD | KeyCode::ArrowRight => costado,
            KeyCode::KeyA | KeyCode::ArrowLeft => matematica::escalar(costado, -1.0),
            KeyCode::KeyR => ARRIBA,
            KeyCode::KeyF => matematica::escalar(ARRIBA, -1.0),
            _ => return false,
        };
        self.posicion = matematica::sumar(self.posicion, matematica::escalar(desplazamiento, PASO));
        true
    }

    /// Vista y perspectiva juntas, para un destino de `ancho` por `alto`.
    pub fn matriz(&self, ancho: u32, alto: u32) -> Mat4 {
        let aspecto = ancho.max(1) as f32 / alto.max(1) as f32;
        let objetivo = matematica::sumar(self.posicion, self.direccion);
        let vista = matematica::mirar(self.posicion, objetivo, ARRIBA);
        let proyeccion = matematica::perspectiva(CAMPO_VERTICAL, aspecto, CERCA, LEJOS);
        matematica::multiplicar(&proyeccion, &vista)
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn w_avanza_hacia_donde_mira() {
        let mut c = Camara::mirando([0.0, 0.0, 3.0], [0.0, 0.0, 0.0]);
        assert!(c.mover(KeyCode::KeyW));
        assert_eq!(c.posicion, [0.0, 0.0, 3.0 - PASO]);
    }

    #[test]
    fn d_va_a_la_derecha_y_a_a_la_izquierda() {
        let mut c = Camara::mirando([0.0, 0.0, 3.0], [0.0, 0.0, 0.0]);
        c.mover(KeyCode::KeyD);
        assert_eq!(c.posicion, [PASO, 0.0, 3.0]);
        c.mover(KeyCode::ArrowLeft);
        c.mover(KeyCode::ArrowLeft);
        assert_eq!(c.posicion, [-PASO, 0.0, 3.0]);
    }

    #[test]
    fn r_sube_y_otra_tecla_no_mueve() {
        let mut c = Camara::mirando([0.0, 0.0, 3.0], [0.0, 0.0, 0.0]);
        c.mover(KeyCode::KeyR);
        assert_eq!(c.posicion, [0.0, PASO, 3.0]);
        assert!(!c.mover(KeyCode::KeyX));
        assert_eq!(c.posicion, [0.0, PASO, 3.0]);
    }
}
