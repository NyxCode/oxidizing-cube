extern crate bresenham;
extern crate cgmath;
extern crate rand;

mod object;
mod screen;

use cgmath::*;
use object::*;
use screen::*;

// TODO: fix comulative floating point arithmetic error
fn main() {
    // let chars = vec!['░', '▒', '▓', '█'];
    // looks great, performs worse in most terminal emulators
    let chars = vec!['.', '~', 'o', '0'];
    let size = 35;
    let mut screen = Screen::new(2 * size, size, chars);

    let init_rotation = create_rotation_matrix(vec3(0., 0., 1.), 35.);
    let rotation = create_rotation_matrix(vec3(0.5, 1., 0.), 1.1);
    let projection = create_projection_matrix();

    let scale: f32 = 12.0;
    let offset: f32 = 2.1;
    let value_scale: f32 = 70.0;
    let value_offset: f32 = 2.1;
    let width = screen.width;
    let height = screen.height;

    let mut cube = mul3(&cube(), init_rotation);

    loop {
        cube = mul3(&cube, rotation);
        let result = mul4(&cube, projection);

        result
            .iter()
            .map(|(v1, v2)| {
                (
                    Pixel::new(
                        ((v1.x + offset) * scale) as usize,
                        ((v1.y + offset) * scale) as usize,
                        ((v1.z + value_offset) * value_scale).min(255.) as u8,
                    ).map(50, width, height),
                    Pixel::new(
                        ((v2.x + offset) * scale) as usize,
                        ((v2.y + offset) * scale) as usize,
                        ((v2.z + value_offset) * value_scale).min(255.) as u8,
                    ).map(50, width, height),
                )
            })
            .for_each(|(px1, px2)| {
                screen.write_line(&px1, &px2);
            });
        screen.flush();
        std::thread::sleep_ms((1000. / 24.) as u32);
    }
}

fn create_rotation_matrix(axis: Vector3<f32>, angle: f32) -> Matrix3<f32> {
    let basis: Basis3<f32> = Rotation3::from_axis_angle(axis, Deg(angle));
    *basis.as_ref()
}

fn create_projection_matrix() -> Matrix4<f32> {
    ortho(-1., 1., -1., 1., -1., 1.)
}
