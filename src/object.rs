use cgmath::*;
use std::ops::Mul;

pub struct Object(Vec<(Vector3<f32>, Vector3<f32>)>);

pub fn mul3(obj: &Object, matrix: Matrix3<f32>) -> Object {
    Object(
        obj.0
            .iter()
            .map(|(v1, v2)| (matrix * *v1, matrix * *v2))
            .collect(),
    )
}

pub fn mul4(obj: &Object, matrix: Matrix4<f32>) -> Vec<(Vector3<f32>, Vector3<f32>)> {
    obj.0
        .iter()
        .map(|(v1, v2)| (vec4(v1.x, v1.y, v1.z, 1.), vec4(v2.x, v2.y, v2.z, 1.)))
        .map(|(v1, v2)| (matrix * v1, matrix * v2))
        .map(|(v1, v2)| (vec3(v1.x, v1.y, v1.z), vec3(v2.x, v2.y, v2.z)))
        .collect()
}

pub fn cube() -> Object {
    Object(vec![
        (vec3(-1., -1., -1.), vec3(1., -1., -1.)),
        (vec3(-1., -1., -1.), vec3(-1., 1., -1.)),
        (vec3(1., -1., -1.), vec3(1., 1., -1.)),
        (vec3(1., 1., -1.), vec3(-1., 1., -1.)),
        (vec3(-1., -1., 1.), vec3(1., -1., 1.)),
        (vec3(-1., -1., 1.), vec3(-1., 1., 1.)),
        (vec3(1., -1., 1.), vec3(1., 1., 1.)),
        (vec3(1., 1., 1.), vec3(-1., 1., 1.)),
        (vec3(-1., -1., -1.), vec3(-1., -1., 1.)),
        (vec3(1., -1., -1.), vec3(1., -1., 1.)),
        (vec3(-1., 1., -1.), vec3(-1., 1., 1.)),
        (vec3(1., 1., -1.), vec3(1., 1., 1.)),
    ])
}
