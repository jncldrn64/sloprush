// Cubo 3D: cada vértice trae su posición y el color de su cara.

struct Transformacion {
    matriz: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> transformacion: Transformacion;

struct Entrada {
    @location(0) posicion: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct Salida {
    @builtin(position) posicion: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertice(entrada: Entrada) -> Salida {
    var salida: Salida;
    salida.posicion = transformacion.matriz * vec4<f32>(entrada.posicion, 1.0);
    salida.color = entrada.color;
    return salida;
}

@fragment
fn fragmento(entrada: Salida) -> @location(0) vec4<f32> {
    return vec4<f32>(entrada.color, 1.0);
}
