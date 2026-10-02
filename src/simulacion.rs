//! Simulación a paso fijo, separada del dibujo.
//!
//! La frecuencia del paso se elige al crear la simulación, vale 60 Hz por defecto y no cambia
//! después (`docs/DESIGN.md`, "La simulación avanza a paso fijo de frecuencia configurable"). El
//! tiempo real de cada cuadro se acumula, y la simulación da los pasos enteros que caben en él.
//! Cuántos pasos se dan no depende de cuántos cuadros se dibujan.

use std::time::Duration;

/// Aceleración de la gravedad, en m/s², hacia y negativa.
pub const GRAVEDAD: f32 = 9.81;

/// Frecuencia del paso si no se elige otra, en Hz.
pub const FRECUENCIA_POR_DEFECTO: u32 = 60;

/// Un cuerpo que se mueve en vertical.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cuerpo {
    /// Altura, en metros.
    pub altura: f32,
    /// Velocidad vertical, en m/s; positiva hacia arriba.
    pub velocidad: f32,
}

impl Cuerpo {
    /// Un cuerpo quieto a `altura` metros.
    pub fn en_reposo(altura: f32) -> Self {
        Cuerpo {
            altura,
            velocidad: 0.0,
        }
    }
}

/// La simulación de un cuerpo que cae con gravedad.
#[derive(Clone, Debug)]
pub struct Simulacion {
    frecuencia: u32,
    paso_real: Duration,
    acumulado: Duration,
    pasos: u64,
    pub cuerpo: Cuerpo,
}

impl Simulacion {
    /// Una simulación de paso fijo a `frecuencia` Hz. La frecuencia no cambia durante la partida.
    pub fn nueva(frecuencia: u32, cuerpo: Cuerpo) -> Self {
        let paso_real = paso_real(frecuencia)
            .expect("la frecuencia del paso tiene que estar entre 1 y FRECUENCIA_MAXIMA");
        Simulacion {
            frecuencia,
            paso_real,
            acumulado: Duration::ZERO,
            pasos: 0,
            cuerpo,
        }
    }

    pub fn frecuencia(&self) -> u32 {
        self.frecuencia
    }

    /// Pasos dados desde el comienzo.
    pub fn pasos(&self) -> u64 {
        self.pasos
    }

    /// Da un paso de 1/frecuencia segundos con Euler semiimplícito: primero la velocidad, después
    /// la posición con la velocidad nueva.
    pub fn dar_paso(&mut self) {
        let h = 1.0 / self.frecuencia as f32;
        self.cuerpo.velocidad -= GRAVEDAD * h;
        self.cuerpo.altura += self.cuerpo.velocidad * h;
        self.pasos += 1;
    }

    /// Suma `transcurrido` de tiempo real y da los pasos enteros que caben, sin pasar de `tope`
    /// pasos en total. Devuelve cuántos pasos dio.
    pub fn avanzar(&mut self, transcurrido: Duration, tope: u64) -> u64 {
        self.acumulado += transcurrido;
        let mut dados = 0;
        while self.acumulado >= self.paso_real && self.pasos < tope {
            self.acumulado -= self.paso_real;
            self.dar_paso();
            dados += 1;
        }
        if self.pasos >= tope {
            self.acumulado = Duration::ZERO;
        }
        dados
    }
}

/// Frecuencia más alta que se acepta: la de un paso de 1 ns.
pub const FRECUENCIA_MAXIMA: u32 = 1_000_000_000;

/// Cuánto tiempo real dura un paso a `frecuencia` Hz. `None` si la frecuencia es cero o más alta
/// que [`FRECUENCIA_MAXIMA`], porque el paso bajaría de 1 ns y `Duration` lo redondearía.
fn paso_real(frecuencia: u32) -> Option<Duration> {
    if frecuencia == 0 || frecuencia > FRECUENCIA_MAXIMA {
        return None;
    }
    Some(Duration::from_secs_f64(1.0 / f64::from(frecuencia)))
}

/// Saca `--hz <valor>` de los argumentos y devuelve la frecuencia y el resto. Sin la opción,
/// [`FRECUENCIA_POR_DEFECTO`].
pub fn separar_frecuencia(args: &[String]) -> Result<(u32, Vec<String>), String> {
    let mut frecuencia = FRECUENCIA_POR_DEFECTO;
    let mut resto = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--hz" {
            let texto = args
                .get(i + 1)
                .ok_or_else(|| "a --hz le falta el valor".to_string())?;
            frecuencia = match texto.parse::<u32>() {
                Ok(n) if paso_real(n).is_some() => n,
                _ => {
                    return Err(format!(
                        "--hz pide un entero entre 1 y {FRECUENCIA_MAXIMA}, no {texto}"
                    ));
                }
            };
            i += 2;
        } else {
            resto.push(args[i].clone());
            i += 1;
        }
    }
    Ok((frecuencia, resto))
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn un_paso_cambia_velocidad_y_altura() {
        let mut s = Simulacion::nueva(10, Cuerpo::en_reposo(0.0));
        s.dar_paso();
        assert_eq!(s.pasos(), 1);
        assert!((s.cuerpo.velocidad + 0.981).abs() < 1e-6);
        assert!((s.cuerpo.altura + 0.0981).abs() < 1e-6);
    }

    #[test]
    fn avanzar_da_solo_los_pasos_enteros_que_caben() {
        let mut s = Simulacion::nueva(60, Cuerpo::en_reposo(0.0));
        assert_eq!(s.avanzar(Duration::from_millis(10), u64::MAX), 0);
        assert_eq!(s.avanzar(Duration::from_millis(10), u64::MAX), 1);
        assert_eq!(s.pasos(), 1);
    }

    #[test]
    fn avanzar_respeta_el_tope() {
        let mut s = Simulacion::nueva(60, Cuerpo::en_reposo(0.0));
        assert_eq!(s.avanzar(Duration::from_secs(5), 60), 60);
        assert_eq!(s.avanzar(Duration::from_secs(5), 60), 0);
        assert_eq!(s.pasos(), 60);
    }

    #[test]
    fn frecuencia_por_defecto_y_por_opcion() {
        assert_eq!(separar_frecuencia(&[]).unwrap().0, 60);
        let args = vec!["--hz".to_string(), "144".to_string(), "x".to_string()];
        let (hz, resto) = separar_frecuencia(&args).unwrap();
        assert_eq!(hz, 144);
        assert_eq!(resto, vec!["x".to_string()]);
        assert!(separar_frecuencia(&["--hz".to_string(), "0".to_string()]).is_err());
    }

    #[test]
    fn una_frecuencia_con_paso_de_menos_de_1_ns_es_error() {
        let hz = |n: u32| separar_frecuencia(&["--hz".to_string(), n.to_string()]);
        assert!(hz(FRECUENCIA_MAXIMA + 1).is_err());
        assert_eq!(hz(FRECUENCIA_MAXIMA).unwrap().0, FRECUENCIA_MAXIMA);
        assert_eq!(paso_real(FRECUENCIA_MAXIMA), Some(Duration::from_nanos(1)));
    }
}
