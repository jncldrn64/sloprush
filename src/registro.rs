//! Registro del motor: cada paso de arranque, carga y cierre sale por la salida estándar con la
//! etapa entre corchetes, para que una corrida se pueda leer y comparar sin pantalla.

/// Un paso del arranque: instancia, adaptador, dispositivo, ventana.
pub fn arranque(mensaje: impl AsRef<str>) {
    println!("[arranque] {}", mensaje.as_ref());
}

/// Un paso de carga: sombreadores, texturas, mallas.
pub fn carga(mensaje: impl AsRef<str>) {
    println!("[carga] {}", mensaje.as_ref());
}

/// Un paso del cierre.
pub fn cierre(mensaje: impl AsRef<str>) {
    println!("[cierre] {}", mensaje.as_ref());
}

/// Una entrada del usuario, como una tecla apretada.
pub fn entrada(mensaje: impl AsRef<str>) {
    println!("[entrada] {}", mensaje.as_ref());
}

/// Algo que el usuario tiene que saber, como un renderizador por software.
pub fn advertencia(mensaje: impl AsRef<str>) {
    println!("[advertencia] {}", mensaje.as_ref());
}

/// Un resultado que un ejemplo imprime para que se lo compare entre corridas.
pub fn resultado(mensaje: impl AsRef<str>) {
    println!("[resultado] {}", mensaje.as_ref());
}

/// Un error que corta la corrida. Va a la salida de error.
pub fn error(mensaje: impl AsRef<str>) {
    eprintln!("[error] {}", mensaje.as_ref());
}
