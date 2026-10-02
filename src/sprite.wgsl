// Sprite 2D: un rectángulo con textura, ubicado en coordenadas de recorte.

struct Rectangulo {
    // xy: centro; zw: medio ancho y media altura.
    centro_y_medio: vec4<f32>,
};

@group(0) @binding(0) var<uniform> rectangulo: Rectangulo;
@group(0) @binding(1) var textura: texture_2d<f32>;
@group(0) @binding(2) var muestreador: sampler;

struct Salida {
    @builtin(position) posicion: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertice(@builtin(vertex_index) indice: u32) -> Salida {
    // Dos triángulos que cubren el rectángulo.
    var esquinas = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0),
    );
    let esquina = esquinas[indice];
    var salida: Salida;
    let centro = rectangulo.centro_y_medio.xy;
    let medio = rectangulo.centro_y_medio.zw;
    salida.posicion = vec4<f32>(centro + esquina * medio, 0.0, 1.0);
    salida.uv = vec2<f32>(esquina.x * 0.5 + 0.5, 0.5 - esquina.y * 0.5);
    return salida;
}

@fragment
fn fragmento(entrada: Salida) -> @location(0) vec4<f32> {
    return textureSample(textura, muestreador, entrada.uv);
}
