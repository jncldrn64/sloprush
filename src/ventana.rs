//! Ventana y bucle de eventos con winit.
//!
//! En Linux, winit 0.30.13 abre la ventana por X11 o por Wayland; no dibuja directo sobre la
//! salida de video. Cada escena del motor se dibuja acá a través de [`Escena`].

use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::gpu::{self, Eleccion, Gpu};
use crate::registro;

/// Lo que se le pide a una corrida con ventana.
#[derive(Clone, Debug, PartialEq)]
pub struct Opciones {
    /// Título de la ventana.
    pub titulo: String,
    /// Backends que se le piden a wgpu.
    pub eleccion: Eleccion,
    /// Modo de cuadros fijos: dibuja esta cantidad de cuadros y termina con código 0.
    pub cuadros: Option<u64>,
    /// Tope de cuadros dibujados por segundo. Con tope, la presentación usa un modo sin
    /// sincronización vertical si la superficie lo ofrece. En Linux, la superficie GL de wgpu
    /// 30.0.1 solo ofrece `Fifo`, así que ahí el tope no pasa la frecuencia del monitor.
    pub limite_fps: Option<u32>,
}

impl Opciones {
    /// Lee `--backend`, `--cuadros N` y `--limite-fps N`. Cualquier otra opción es error.
    pub fn desde_args(titulo: &str, args: &[String]) -> Result<Self, String> {
        let (eleccion, resto) = gpu::separar_backend(args)?;
        let mut opciones = Opciones {
            titulo: titulo.to_string(),
            eleccion,
            cuadros: None,
            limite_fps: None,
        };
        let mut i = 0;
        while i < resto.len() {
            let nombre = resto[i].as_str();
            let valor = resto.get(i + 1);
            match nombre {
                "--cuadros" => opciones.cuadros = Some(numero_positivo(nombre, valor)?),
                "--limite-fps" => {
                    let n = numero_positivo(nombre, valor)?;
                    let n =
                        u32::try_from(n).map_err(|_| format!("{nombre} es demasiado grande"))?;
                    opciones.limite_fps = Some(n);
                }
                otro => return Err(format!("opción desconocida: {otro}")),
            }
            i += 2;
        }
        Ok(opciones)
    }
}

fn numero_positivo(nombre: &str, valor: Option<&String>) -> Result<u64, String> {
    let texto = valor.ok_or_else(|| format!("a {nombre} le falta el valor"))?;
    match texto.parse::<u64>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!(
            "{nombre} pide un entero mayor que cero, no {texto}"
        )),
    }
}

/// Lo que se dibuja en la ventana.
pub trait Escena {
    /// Se llama una vez, con la GPU lista y el formato de la superficie.
    fn preparar(&mut self, _gpu: &Gpu, _formato: wgpu::TextureFormat) {}

    /// Una tecla apretada, por su posición en el teclado.
    fn tecla(&mut self, _tecla: KeyCode) {}

    /// Avanza con el tiempo real desde el cuadro anterior. Si devuelve `false`, la ventana cierra.
    fn avanzar(&mut self, _transcurrido: Duration) -> bool {
        true
    }

    /// Dibuja un cuadro en `destino`, de `ancho` por `alto` píxeles.
    fn dibujar(&mut self, gpu: &Gpu, destino: &wgpu::TextureView, ancho: u32, alto: u32);
}

/// Abre la ventana y dibuja la escena hasta que se cierra la ventana, se cumple el modo de
/// cuadros fijos o la escena pide terminar.
pub fn correr(opciones: Opciones, escena: &mut dyn Escena) -> Result<(), String> {
    let bucle =
        EventLoop::new().map_err(|e| format!("no se pudo crear el bucle de eventos: {e}"))?;
    let pantalla = bucle.owned_display_handle();
    let mut app = App {
        opciones,
        escena,
        pantalla,
        estado: None,
        error: None,
        cuadros: 0,
        anterior: None,
    };
    bucle
        .run_app(&mut app)
        .map_err(|e| format!("el bucle de eventos terminó con error: {e}"))?;
    match app.error {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

struct Estado {
    ventana: Arc<Window>,
    gpu: Gpu,
    superficie: wgpu::Surface<'static>,
    configuracion: wgpu::SurfaceConfiguration,
}

struct App<'a> {
    opciones: Opciones,
    escena: &'a mut dyn Escena,
    pantalla: OwnedDisplayHandle,
    estado: Option<Estado>,
    error: Option<String>,
    cuadros: u64,
    anterior: Option<Instant>,
}

impl App<'_> {
    fn abrir(&mut self, bucle: &ActiveEventLoop) -> Result<(), String> {
        let atributos = Window::default_attributes()
            .with_title(&self.opciones.titulo)
            .with_inner_size(PhysicalSize::new(640, 480));
        let ventana = Arc::new(
            bucle
                .create_window(atributos)
                .map_err(|e| format!("no se pudo abrir la ventana: {e}"))?,
        );
        registro::arranque(format!("ventana \"{}\" abierta", self.opciones.titulo));

        let instancia = gpu::crear_instancia(
            self.opciones.eleccion,
            Some(Box::new(self.pantalla.clone())),
        );
        let superficie = instancia
            .create_surface(ventana.clone())
            .map_err(|e| format!("no se pudo crear la superficie: {e}"))?;
        let gpu = gpu::iniciar(instancia, Some(&superficie)).map_err(|e| e.to_string())?;

        let capacidades = superficie.get_capabilities(&gpu.adaptador);
        let formato = capacidades
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| capacidades.formats.first().copied())
            .ok_or_else(|| "la superficie no admite ningún formato".to_string())?;
        let sin_espera = [wgpu::PresentMode::Immediate, wgpu::PresentMode::Mailbox]
            .into_iter()
            .find(|m| capacidades.present_modes.contains(m));
        let presentacion = match (self.opciones.limite_fps, sin_espera) {
            (Some(_), Some(modo)) => modo,
            (Some(_), None) => {
                registro::arranque(
                    "la superficie solo presenta con sincronización vertical: el tope de dibujo \
                     no pasa la frecuencia del monitor",
                );
                wgpu::PresentMode::Fifo
            }
            (None, _) => wgpu::PresentMode::Fifo,
        };
        let tamano = ventana.inner_size();
        let configuracion = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: formato,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: tamano.width.max(1),
            height: tamano.height.max(1),
            desired_maximum_frame_latency: 2,
            present_mode: presentacion,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        superficie.configure(&gpu.dispositivo, &configuracion);
        registro::arranque(format!(
            "superficie de {}x{}, formato {:?}, presentación {:?}",
            configuracion.width, configuracion.height, formato, presentacion
        ));

        self.escena.preparar(&gpu, formato);
        ventana.request_redraw();
        self.estado = Some(Estado {
            ventana,
            gpu,
            superficie,
            configuracion,
        });
        Ok(())
    }

    fn cuadro(&mut self, bucle: &ActiveEventLoop) {
        let inicio = Instant::now();
        let transcurrido = self
            .anterior
            .map(|a| inicio.duration_since(a))
            .unwrap_or_default();
        self.anterior = Some(inicio);
        if !self.escena.avanzar(transcurrido) {
            registro::cierre("la escena terminó");
            bucle.exit();
            return;
        }
        let Some(estado) = self.estado.as_mut() else {
            return;
        };

        let textura = match estado.superficie.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Outdated => {
                estado
                    .superficie
                    .configure(&estado.gpu.dispositivo, &estado.configuracion);
                estado.ventana.request_redraw();
                return;
            }
            // Una superficie perdida no se reconfigura: se crea otra para la misma ventana.
            wgpu::CurrentSurfaceTexture::Lost => {
                match estado.gpu.instancia.create_surface(estado.ventana.clone()) {
                    Ok(nueva) => {
                        nueva.configure(&estado.gpu.dispositivo, &estado.configuracion);
                        estado.superficie = nueva;
                        registro::arranque("superficie perdida, creada de nuevo");
                        estado.ventana.request_redraw();
                    }
                    Err(e) => {
                        self.error = Some(format!(
                            "se perdió la superficie y no se pudo crear otra: {e}"
                        ));
                        bucle.exit();
                    }
                }
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                estado.ventana.request_redraw();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                self.error = Some("error de validación al pedir el cuadro".to_string());
                bucle.exit();
                return;
            }
        };
        let vista = textura
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.escena.dibujar(
            &estado.gpu,
            &vista,
            estado.configuracion.width,
            estado.configuracion.height,
        );
        estado.ventana.pre_present_notify();
        estado.gpu.cola.present(textura);
        self.cuadros += 1;

        if self.opciones.cuadros == Some(self.cuadros) {
            registro::cierre(format!(
                "{} cuadros dibujados, modo de cuadros fijos",
                self.cuadros
            ));
            bucle.exit();
            return;
        }
        if let Some(limite) = self.opciones.limite_fps {
            let periodo = Duration::from_secs_f64(1.0 / f64::from(limite));
            if let Some(resto) = periodo.checked_sub(inicio.elapsed()) {
                std::thread::sleep(resto);
            }
        }
        estado.ventana.request_redraw();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, bucle: &ActiveEventLoop) {
        if self.estado.is_some() {
            return;
        }
        if let Err(e) = self.abrir(bucle) {
            self.error = Some(e);
            bucle.exit();
        }
    }

    fn window_event(&mut self, bucle: &ActiveEventLoop, _id: WindowId, evento: WindowEvent) {
        match evento {
            WindowEvent::CloseRequested => {
                registro::cierre(format!("ventana cerrada tras {} cuadros", self.cuadros));
                bucle.exit();
            }
            WindowEvent::Resized(tamano) if tamano.width > 0 && tamano.height > 0 => {
                if let Some(estado) = self.estado.as_mut() {
                    estado.configuracion.width = tamano.width;
                    estado.configuracion.height = tamano.height;
                    estado
                        .superficie
                        .configure(&estado.gpu.dispositivo, &estado.configuracion);
                }
            }
            WindowEvent::KeyboardInput { event, .. }
                if event.state == ElementState::Pressed && !event.repeat =>
            {
                registro::entrada(format!(
                    "tecla {:?}, texto {:?}",
                    event.physical_key, event.logical_key
                ));
                if let PhysicalKey::Code(codigo) = event.physical_key {
                    self.escena.tecla(codigo);
                }
            }
            WindowEvent::RedrawRequested => self.cuadro(bucle),
            _ => {}
        }
    }

    fn exiting(&mut self, _bucle: &ActiveEventLoop) {
        if self.estado.take().is_some() {
            registro::cierre("superficie y dispositivo liberados");
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn args(texto: &str) -> Vec<String> {
        texto.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn sin_opciones_no_hay_tope_ni_cuadros_fijos() {
        let o = Opciones::desde_args("t", &[]).unwrap();
        assert_eq!(o.cuadros, None);
        assert_eq!(o.limite_fps, None);
        assert_eq!(o.eleccion, Eleccion::Todos);
    }

    #[test]
    fn lee_cuadros_limite_y_backend() {
        let o =
            Opciones::desde_args("t", &args("--cuadros 3 --limite-fps 240 --backend gl")).unwrap();
        assert_eq!(o.cuadros, Some(3));
        assert_eq!(o.limite_fps, Some(240));
        assert_eq!(o.eleccion, Eleccion::Gl);
    }

    #[test]
    fn rechaza_cero_texto_y_opciones_desconocidas() {
        assert!(Opciones::desde_args("t", &args("--cuadros 0")).is_err());
        assert!(Opciones::desde_args("t", &args("--cuadros tres")).is_err());
        assert!(Opciones::desde_args("t", &args("--cuadros")).is_err());
        assert!(Opciones::desde_args("t", &args("--pantalla-completa")).is_err());
    }
}
