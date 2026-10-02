//! Cubo 3D de lado 1, con un color plano por cara y búfer de profundidad.

use crate::camara::Camara;
use crate::dibujo::Destino;
use crate::gpu::Gpu;
use crate::matematica::{self, Vec3};
use crate::registro;

/// Color de cada cara en sRGB, en el orden +x, -x, +y, -y, +z, -z.
pub const COLORES: [[u8; 4]; 6] = [
    [220, 60, 60, 255],
    [60, 180, 75, 255],
    [240, 210, 60, 255],
    [145, 80, 200, 255],
    [60, 120, 230, 255],
    [245, 130, 50, 255],
];

/// Índice en [`COLORES`] de la cara que mira hacia +z.
pub const CARA_MAS_Z: usize = 4;
/// Índice en [`COLORES`] de la cara que mira hacia +x.
pub const CARA_MAS_X: usize = 0;

const FORMATO_PROFUNDIDAD: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const BYTES_POR_VERTICE: u64 = 24;

/// Pasa un canal de sRGB en bytes a valor lineal, como lo espera un destino sRGB.
pub fn srgb_a_lineal(canal: u8) -> f32 {
    let c = f32::from(canal) / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Los 36 vértices del cubo, seis por cara, cada uno con posición y color lineal.
pub fn vertices() -> Vec<f32> {
    // Normal de cada cara y dos ejes del plano de la cara.
    let caras: [(Vec3, Vec3, Vec3); 6] = [
        ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        ([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        ([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ([0.0, 0.0, -1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    ];
    let mut datos = Vec::with_capacity(36 * 6);
    for ((normal, u, v), color) in caras.iter().zip(COLORES) {
        let centro = matematica::escalar(*normal, 0.5);
        let esquina = |su: f32, sv: f32| {
            matematica::sumar(
                centro,
                matematica::sumar(
                    matematica::escalar(*u, su * 0.5),
                    matematica::escalar(*v, sv * 0.5),
                ),
            )
        };
        let lineal = [
            srgb_a_lineal(color[0]),
            srgb_a_lineal(color[1]),
            srgb_a_lineal(color[2]),
        ];
        for (su, sv) in [
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 1.0),
            (-1.0, -1.0),
            (1.0, 1.0),
            (-1.0, 1.0),
        ] {
            datos.extend_from_slice(&esquina(su, sv));
            datos.extend_from_slice(&lineal);
        }
    }
    datos
}

struct Profundidad {
    vista: wgpu::TextureView,
    ancho: u32,
    alto: u32,
}

/// Un cubo listo para dibujar en destinos de un formato.
pub struct Cubo {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    transformacion: wgpu::Buffer,
    grupo: wgpu::BindGroup,
    profundidad: Option<Profundidad>,
}

impl Cubo {
    /// Carga sombreador, vértices y pipeline para dibujar en destinos de `formato`.
    pub fn nuevo(gpu: &Gpu, formato: wgpu::TextureFormat) -> Self {
        let disp = &gpu.dispositivo;
        let modulo = disp.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cubo"),
            source: wgpu::ShaderSource::Wgsl(include_str!("cubo.wgsl").into()),
        });
        registro::carga("sombreador del cubo");

        let datos: Vec<u8> = vertices().iter().flat_map(|v| v.to_le_bytes()).collect();
        let vertices = disp.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vértices del cubo"),
            size: datos.len() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.cola.write_buffer(&vertices, 0, &datos);
        registro::carga("malla del cubo, 36 vértices");

        let transformacion = disp.create_buffer(&wgpu::BufferDescriptor {
            label: Some("transformación del cubo"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let disposicion = disp.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cubo"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let grupo = disp.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cubo"),
            layout: &disposicion,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transformacion.as_entire_binding(),
            }],
        });
        let disposicion_pipeline = disp.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cubo"),
            bind_group_layouts: &[Some(&disposicion)],
            immediate_size: 0,
        });
        let atributos = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 12,
                shader_location: 1,
            },
        ];
        let pipeline = disp.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cubo"),
            layout: Some(&disposicion_pipeline),
            vertex: wgpu::VertexState {
                module: &modulo,
                entry_point: Some("vertice"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: BYTES_POR_VERTICE,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &atributos,
                })],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: FORMATO_PROFUNDIDAD,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &modulo,
                entry_point: Some("fragmento"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: formato,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        registro::carga(format!("pipeline del cubo para {formato:?}"));

        Cubo {
            pipeline,
            vertices,
            transformacion,
            grupo,
            profundidad: None,
        }
    }

    fn profundidad(&mut self, gpu: &Gpu, ancho: u32, alto: u32) -> &wgpu::TextureView {
        let vigente = matches!(&self.profundidad, Some(p) if p.ancho == ancho && p.alto == alto);
        if !vigente {
            let textura = gpu.dispositivo.create_texture(&wgpu::TextureDescriptor {
                label: Some("profundidad"),
                size: wgpu::Extent3d {
                    width: ancho,
                    height: alto,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: FORMATO_PROFUNDIDAD,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let vista = textura.create_view(&wgpu::TextureViewDescriptor::default());
            self.profundidad = Some(Profundidad { vista, ancho, alto });
        }
        &self.profundidad.as_ref().expect("recién creada").vista
    }

    /// Limpia `destino` con `fondo` y dibuja el cubo con su centro en `centro`, visto desde
    /// `camara`.
    pub fn dibujar(
        &mut self,
        gpu: &Gpu,
        destino: Destino<'_>,
        camara: &Camara,
        centro: Vec3,
        fondo: wgpu::Color,
    ) {
        let modelo = matematica::trasladar(centro);
        let matriz = matematica::multiplicar(&camara.matriz(destino.ancho, destino.alto), &modelo);
        gpu.cola
            .write_buffer(&self.transformacion, 0, &matematica::a_bytes(&matriz));
        let mut codificador =
            gpu.dispositivo
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("cubo"),
                });
        let profundidad = self.profundidad(gpu, destino.ancho, destino.alto).clone();
        {
            let mut pasada = codificador.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cubo"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: destino.vista,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(fondo),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &profundidad,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pasada.set_pipeline(&self.pipeline);
            pasada.set_bind_group(0, &self.grupo, &[]);
            pasada.set_vertex_buffer(0, self.vertices.slice(..));
            pasada.draw(0..36, 0..1);
        }
        gpu.cola.submit([codificador.finish()]);
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn treinta_y_seis_vertices_de_seis_floats() {
        assert_eq!(vertices().len(), 36 * 6);
    }

    #[test]
    fn todos_los_vertices_estan_en_la_superficie_del_cubo() {
        for v in vertices().chunks_exact(6) {
            let mayor = v[..3].iter().fold(0.0_f32, |m, c| m.max(c.abs()));
            assert!((mayor - 0.5).abs() < 1e-6, "vértice fuera del cubo: {v:?}");
        }
    }

    #[test]
    fn srgb_a_lineal_en_los_extremos() {
        assert_eq!(srgb_a_lineal(0), 0.0);
        assert!((srgb_a_lineal(255) - 1.0).abs() < 1e-6);
    }
}
