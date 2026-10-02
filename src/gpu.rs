//! Arranque de la GPU: instancia, adaptador y dispositivo de wgpu.
//!
//! Todo el dibujo del motor pasa por wgpu (`docs/DESIGN.md`, "Toda la salida gráfica pasa por
//! wgpu"). El dispositivo se pide con los límites de WebGL2, los más bajos de wgpu, para que lo que
//! corre en el nivel completo corra igual en el nivel base (`docs/DESIGN.md`, "Dos niveles
//! gráficos").

use std::fmt;

use crate::registro;

/// Qué backends se le piden a wgpu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eleccion {
    /// Los que wgpu encuentre; en Linux, Vulkan y OpenGL ES.
    Todos,
    /// Solo OpenGL ES, del nivel base.
    Gl,
    /// Solo Vulkan, del nivel completo.
    Vulkan,
}

impl Eleccion {
    /// Lee el valor de `--backend`: `todos`, `gl` o `vulkan`.
    pub fn desde_texto(texto: &str) -> Result<Self, String> {
        match texto {
            "todos" => Ok(Self::Todos),
            "gl" => Ok(Self::Gl),
            "vulkan" => Ok(Self::Vulkan),
            otro => Err(format!(
                "backend desconocido: {otro}; los válidos son todos, gl y vulkan"
            )),
        }
    }

    /// Los backends de wgpu que corresponden a esta elección.
    pub fn backends(self) -> wgpu::Backends {
        match self {
            Self::Todos => wgpu::Backends::VULKAN | wgpu::Backends::GL,
            Self::Gl => wgpu::Backends::GL,
            Self::Vulkan => wgpu::Backends::VULKAN,
        }
    }
}

/// Saca `--backend <valor>` de los argumentos y devuelve el resto. Sin la opción, `Todos`.
pub fn separar_backend(args: &[String]) -> Result<(Eleccion, Vec<String>), String> {
    let mut eleccion = Eleccion::Todos;
    let mut resto = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--backend" {
            let valor = args
                .get(i + 1)
                .ok_or_else(|| "a --backend le falta el valor".to_string())?;
            eleccion = Eleccion::desde_texto(valor)?;
            i += 2;
        } else {
            resto.push(args[i].clone());
            i += 1;
        }
    }
    Ok((eleccion, resto))
}

/// Nombres que delatan un renderizador por software aunque el driver no lo declare como CPU.
const NOMBRES_DE_SOFTWARE: [&str; 5] =
    ["llvmpipe", "lavapipe", "softpipe", "swiftshader", "swrast"];

/// Dice si un adaptador dibuja con la CPU, por su tipo o por su nombre.
pub fn es_por_software(tipo: wgpu::DeviceType, nombre: &str, driver: &str) -> bool {
    if tipo == wgpu::DeviceType::Cpu {
        return true;
    }
    let texto = format!("{nombre} {driver}").to_lowercase();
    NOMBRES_DE_SOFTWARE.iter().any(|n| texto.contains(n))
}

/// Una línea legible con los datos de un adaptador.
pub fn describir(info: &wgpu::AdapterInfo) -> String {
    format!(
        "{} | backend {:?} | tipo {:?} | driver {} {}",
        info.name, info.backend, info.device_type, info.driver, info.driver_info
    )
}

/// Por qué no se pudo arrancar la GPU.
#[derive(Debug)]
pub enum ErrorArranque {
    /// Ningún adaptador de los backends pedidos.
    SinAdaptador(String),
    /// Hubo adaptador, pero no dio un dispositivo con los límites del nivel base.
    SinDispositivo(String),
}

impl fmt::Display for ErrorArranque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SinAdaptador(detalle) => write!(f, "no hay adaptador gráfico: {detalle}"),
            Self::SinDispositivo(detalle) => {
                write!(f, "el adaptador no dio dispositivo: {detalle}")
            }
        }
    }
}

impl std::error::Error for ErrorArranque {}

/// La GPU lista para dibujar.
pub struct Gpu {
    pub instancia: wgpu::Instance,
    pub adaptador: wgpu::Adapter,
    pub dispositivo: wgpu::Device,
    pub cola: wgpu::Queue,
    pub info: wgpu::AdapterInfo,
}

impl Gpu {
    /// Si el adaptador elegido dibuja con la CPU.
    pub fn por_software(&self) -> bool {
        es_por_software(self.info.device_type, &self.info.name, &self.info.driver)
    }
}

/// Crea la instancia de wgpu. Con ventana, `pantalla` es la conexión con el sistema de ventanas,
/// que OpenGL ES necesita para presentar; sin ventana, `None`.
pub fn crear_instancia(
    eleccion: Eleccion,
    pantalla: Option<Box<dyn wgpu::wgt::WgpuHasDisplayHandle>>,
) -> wgpu::Instance {
    let mut descriptor = match pantalla {
        Some(p) => wgpu::InstanceDescriptor::new_with_display_handle(p),
        None => wgpu::InstanceDescriptor::new_without_display_handle(),
    };
    descriptor.backends = eleccion.backends();
    registro::arranque(format!(
        "instancia de wgpu con backends {:?}",
        descriptor.backends
    ));
    wgpu::Instance::new(descriptor)
}

/// Imprime todos los adaptadores que la instancia ve.
pub fn listar_adaptadores(instancia: &wgpu::Instance, eleccion: Eleccion) {
    let adaptadores = pollster::block_on(instancia.enumerate_adapters(eleccion.backends()));
    if adaptadores.is_empty() {
        registro::arranque("ningún adaptador disponible");
    }
    for adaptador in adaptadores {
        registro::arranque(format!(
            "adaptador disponible: {}",
            describir(&adaptador.get_info())
        ));
    }
}

/// Elige un adaptador, imprime cuál y con qué backend, avisa si es por software y pide el
/// dispositivo con los límites de WebGL2.
pub fn iniciar(
    instancia: wgpu::Instance,
    superficie: Option<&wgpu::Surface<'_>>,
) -> Result<Gpu, ErrorArranque> {
    let opciones = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: superficie,
        apply_limit_buckets: false,
    };
    let adaptador = pollster::block_on(instancia.request_adapter(&opciones))
        .map_err(|e| ErrorArranque::SinAdaptador(e.to_string()))?;
    let info = adaptador.get_info();
    registro::arranque(format!("adaptador elegido: {}", describir(&info)));
    registro::arranque(format!("backend elegido: {:?}", info.backend));
    if es_por_software(info.device_type, &info.name, &info.driver) {
        registro::advertencia(format!(
            "{} es un renderizador por software: una medición hecha así no cuenta como \
             medición de GPU",
            info.name
        ));
    }

    let limites = wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adaptador.limits());
    let descriptor = wgpu::DeviceDescriptor {
        label: Some("sloprush"),
        required_features: wgpu::Features::empty(),
        required_limits: limites,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    };
    let (dispositivo, cola) = pollster::block_on(adaptador.request_device(&descriptor))
        .map_err(|e| ErrorArranque::SinDispositivo(e.to_string()))?;
    registro::arranque("dispositivo listo con los límites de WebGL2");

    Ok(Gpu {
        instancia,
        adaptador,
        dispositivo,
        cola,
        info,
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn un_adaptador_de_tipo_cpu_es_por_software() {
        assert!(es_por_software(wgpu::DeviceType::Cpu, "cualquiera", ""));
    }

    #[test]
    fn llvmpipe_es_por_software_aunque_no_diga_cpu() {
        assert!(es_por_software(
            wgpu::DeviceType::Other,
            "llvmpipe (LLVM 20.1.2, 256 bits)",
            ""
        ));
    }

    #[test]
    fn una_gpu_discreta_no_es_por_software() {
        assert!(!es_por_software(
            wgpu::DeviceType::DiscreteGpu,
            "AMD Radeon RX 580 Series (RADV POLARIS10)",
            "radv"
        ));
    }

    #[test]
    fn backend_por_defecto_y_por_opcion() {
        let args: Vec<String> = vec![];
        assert_eq!(separar_backend(&args).unwrap().0, Eleccion::Todos);
        let args = vec!["--backend".to_string(), "gl".to_string(), "x".to_string()];
        let (eleccion, resto) = separar_backend(&args).unwrap();
        assert_eq!(eleccion, Eleccion::Gl);
        assert_eq!(resto, vec!["x".to_string()]);
    }

    #[test]
    fn backend_desconocido_o_sin_valor_es_error() {
        assert!(separar_backend(&["--backend".to_string(), "metal".to_string()]).is_err());
        assert!(separar_backend(&["--backend".to_string()]).is_err());
    }
}
