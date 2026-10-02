//! Lienzo: una textura fuera de pantalla donde se dibuja un cuadro para leerlo de vuelta.
//!
//! Las pruebas comprueban lo que se dibuja leyendo los píxeles del lienzo, sin mirar una pantalla.

use std::sync::mpsc;

use crate::gpu::Gpu;

/// Una textura de destino que se puede copiar a memoria.
pub struct Lienzo {
    pub textura: wgpu::Texture,
    pub vista: wgpu::TextureView,
    pub ancho: u32,
    pub alto: u32,
}

impl Lienzo {
    /// Crea un lienzo de `ancho` por `alto` en `formato`, que tiene que ser de 4 bytes por píxel.
    pub fn nuevo(gpu: &Gpu, ancho: u32, alto: u32, formato: wgpu::TextureFormat) -> Self {
        let textura = gpu.dispositivo.create_texture(&wgpu::TextureDescriptor {
            label: Some("lienzo"),
            size: wgpu::Extent3d {
                width: ancho,
                height: alto,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: formato,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let vista = textura.create_view(&wgpu::TextureViewDescriptor::default());
        Lienzo {
            textura,
            vista,
            ancho,
            alto,
        }
    }

    /// Copia el lienzo a memoria y devuelve sus píxeles fila por fila, de arriba hacia abajo.
    pub fn leer(&self, gpu: &Gpu) -> Result<Vec<[u8; 4]>, String> {
        let alineacion = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let bytes_por_fila = self.ancho * 4;
        let fila_alineada = bytes_por_fila.div_ceil(alineacion) * alineacion;
        let memoria = gpu.dispositivo.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lectura del lienzo"),
            size: u64::from(fila_alineada) * u64::from(self.alto),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut codificador =
            gpu.dispositivo
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("lectura del lienzo"),
                });
        codificador.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.textura,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &memoria,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(fila_alineada),
                    rows_per_image: Some(self.alto),
                },
            },
            wgpu::Extent3d {
                width: self.ancho,
                height: self.alto,
                depth_or_array_layers: 1,
            },
        );
        gpu.cola.submit([codificador.finish()]);

        let (enviar, recibir) = mpsc::channel();
        memoria.map_async(wgpu::MapMode::Read, .., move |r| {
            let _ = enviar.send(r);
        });
        gpu.dispositivo
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| format!("la GPU no terminó la copia: {e}"))?;
        recibir
            .recv()
            .map_err(|e| format!("la copia no avisó: {e}"))?
            .map_err(|e| format!("no se pudo leer la copia: {e}"))?;

        let mut pixeles = Vec::with_capacity((self.ancho * self.alto) as usize);
        {
            let datos = memoria
                .get_mapped_range(..)
                .map_err(|e| format!("no se pudo ver la copia: {e}"))?;
            for fila in datos.chunks(fila_alineada as usize) {
                for p in fila[..bytes_por_fila as usize].chunks_exact(4) {
                    pixeles.push([p[0], p[1], p[2], p[3]]);
                }
            }
        }
        memoria.unmap();
        Ok(pixeles)
    }
}

/// El píxel en la columna `x` y la fila `y` de una lectura de `ancho` píxeles por fila.
pub fn pixel(pixeles: &[[u8; 4]], ancho: u32, x: u32, y: u32) -> [u8; 4] {
    pixeles[(y * ancho + x) as usize]
}

/// Si dos colores difieren en a lo sumo `tolerancia` en cada canal.
pub fn parecido(a: [u8; 4], b: [u8; 4], tolerancia: u8) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= tolerancia)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn parecido_respeta_la_tolerancia() {
        assert!(parecido([10, 20, 30, 255], [12, 18, 30, 255], 2));
        assert!(!parecido([10, 20, 30, 255], [13, 20, 30, 255], 2));
    }

    #[test]
    fn pixel_cuenta_filas_de_arriba_hacia_abajo() {
        let pixeles = vec![[0, 0, 0, 0], [1, 0, 0, 0], [2, 0, 0, 0], [3, 0, 0, 0]];
        assert_eq!(pixel(&pixeles, 2, 1, 1), [3, 0, 0, 0]);
    }
}
