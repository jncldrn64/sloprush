//! Pasos de dibujo que comparten las escenas.

use crate::gpu::Gpu;

/// Limpia `destino` con un color y envía el trabajo a la cola.
pub fn limpiar(gpu: &Gpu, destino: &wgpu::TextureView, color: wgpu::Color) {
    let mut codificador = gpu
        .dispositivo
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("limpiar"),
        });
    {
        let _pasada = codificador.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("limpiar"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: destino,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
    gpu.cola.submit([codificador.finish()]);
}
