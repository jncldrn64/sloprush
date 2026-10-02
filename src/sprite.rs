//! Sprite 2D: un rectángulo con una textura, dibujado con un solo pipeline de wgpu.
//!
//! La textura es un damero de 4 por 4 que se arma en el código, sin leer archivos.

use crate::gpu::Gpu;
use crate::registro;

/// Color de las casillas pares del damero, en sRGB.
pub const COLOR_A: [u8; 4] = [242, 140, 40, 255];
/// Color de las casillas impares del damero, en sRGB.
pub const COLOR_B: [u8; 4] = [250, 240, 220, 255];
/// Casillas por lado del damero.
pub const LADO: u32 = 4;

/// Los bytes RGBA del damero, fila por fila.
pub fn damero() -> Vec<u8> {
    let mut bytes = Vec::with_capacity((LADO * LADO * 4) as usize);
    for fila in 0..LADO {
        for columna in 0..LADO {
            let color = if (fila + columna) % 2 == 0 {
                COLOR_A
            } else {
                COLOR_B
            };
            bytes.extend_from_slice(&color);
        }
    }
    bytes
}

/// Un sprite listo para dibujar en destinos de un formato.
pub struct Sprite {
    pipeline: wgpu::RenderPipeline,
    grupo: wgpu::BindGroup,
    rectangulo: wgpu::Buffer,
}

impl Sprite {
    /// Carga sombreador, textura y pipeline para dibujar en destinos de `formato`.
    pub fn nuevo(gpu: &Gpu, formato: wgpu::TextureFormat) -> Self {
        let disp = &gpu.dispositivo;
        let modulo = disp.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sprite.wgsl").into()),
        });
        registro::carga("sombreador del sprite");

        let tamano = wgpu::Extent3d {
            width: LADO,
            height: LADO,
            depth_or_array_layers: 1,
        };
        let textura = disp.create_texture(&wgpu::TextureDescriptor {
            label: Some("damero"),
            size: tamano,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        gpu.cola.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &textura,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &damero(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(LADO * 4),
                rows_per_image: Some(LADO),
            },
            tamano,
        );
        registro::carga(format!("textura del sprite, damero de {LADO}x{LADO}"));

        let muestreador = disp.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sprite"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let rectangulo = disp.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rectángulo del sprite"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let disposicion = disp.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let vista = textura.create_view(&wgpu::TextureViewDescriptor::default());
        let grupo = disp.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite"),
            layout: &disposicion,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: rectangulo.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&vista),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&muestreador),
                },
            ],
        });
        let disposicion_pipeline = disp.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite"),
            bind_group_layouts: &[Some(&disposicion)],
            immediate_size: 0,
        });
        let pipeline = disp.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sprite"),
            layout: Some(&disposicion_pipeline),
            vertex: wgpu::VertexState {
                module: &modulo,
                entry_point: Some("vertice"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &modulo,
                entry_point: Some("fragmento"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: formato,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        registro::carga(format!("pipeline del sprite para {formato:?}"));

        let sprite = Sprite {
            pipeline,
            grupo,
            rectangulo,
        };
        sprite.ubicar(gpu, [0.0, 0.0], [0.5, 0.5]);
        sprite
    }

    /// Ubica el sprite con su `centro` y su `medio` tamaño, en coordenadas de recorte: de -1 a 1
    /// en cada eje, con el y hacia arriba.
    pub fn ubicar(&self, gpu: &Gpu, centro: [f32; 2], medio: [f32; 2]) {
        let mut bytes = Vec::with_capacity(16);
        for v in [centro[0], centro[1], medio[0], medio[1]] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        gpu.cola.write_buffer(&self.rectangulo, 0, &bytes);
    }

    /// Limpia `destino` con `fondo` y dibuja el sprite encima.
    pub fn dibujar(&self, gpu: &Gpu, destino: &wgpu::TextureView, fondo: wgpu::Color) {
        let mut codificador =
            gpu.dispositivo
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("sprite"),
                });
        {
            let mut pasada = codificador.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sprite"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: destino,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(fondo),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pasada.set_pipeline(&self.pipeline);
            pasada.set_bind_group(0, &self.grupo, &[]);
            pasada.draw(0..6, 0..1);
        }
        gpu.cola.submit([codificador.finish()]);
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn el_damero_alterna_colores_por_casilla() {
        let bytes = damero();
        assert_eq!(bytes.len(), (LADO * LADO * 4) as usize);
        assert_eq!(&bytes[0..4], &COLOR_A);
        assert_eq!(&bytes[4..8], &COLOR_B);
        let segunda_fila = (LADO * 4) as usize;
        assert_eq!(&bytes[segunda_fila..segunda_fila + 4], &COLOR_B);
    }
}
