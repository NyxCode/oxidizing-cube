use bresenham::Bresenham;
use cgmath::*;
use std::cmp::max;
use std::mem;

pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub value: u8,
}

impl Pixel {
    pub fn new(x: usize, y: usize, value: u8) -> Self {
        Pixel { x, y, value }
    }

    pub fn map(&self, gamma: usize, width: usize, height: usize) -> Pixel {
        fn do_map(gamma: usize, value: usize, max: usize) -> usize {
            (value as f32 / gamma as f32 * max as f32) as usize
        }

        Pixel::new(
            do_map(gamma, self.x, width),
            do_map(gamma, self.y, height),
            self.value,
        )
    }
}

type ScreenBuffer = Vec<Vec<u8>>;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pixel: Vec<char>,
    buffer: ScreenBuffer,
}

impl Screen {
    pub fn new(width: usize, height: usize, pixel: Vec<char>) -> Self {
        Screen {
            width,
            height,
            pixel,
            buffer: new_buffer(width, height),
        }
    }

    #[inline]
    pub fn write_pixel(&mut self, pixel: &Pixel) {
        if pixel.x < self.width && pixel.y < self.height {
            let current_pixel = self.buffer[pixel.y][pixel.x];
            if current_pixel < pixel.value {
                self.buffer[pixel.y][pixel.x] = pixel.value;
            }
        }
    }

    pub fn write_line(&mut self, from: &Pixel, to: &Pixel) {
        let pixel_count = max(
            (to.x as isize - from.x as isize + 1).abs(),
            (to.y as isize - from.y as isize + 1).abs(),
        );

        let value_increase = to.value as isize - from.value as isize;

        let (from_x, from_y) = (from.x as isize, from.y as isize);
        let (to_x, to_y) = {
            let x = if to.x > from.x {
                to.x as isize + 1
            } else {
                to.x as isize - 1
            };
            let y = if to.y > from.y {
                to.y as isize + 1
            } else {
                to.y as isize - 1
            };
            (x, y)
        };

        let mut counter = 0;
        for (x, y) in Bresenham::new((from_x, from_y), (to_x, to_y)) {
            let progress = counter as f32 / (pixel_count - 1) as f32;
            let value = from.value as f32 + (progress as f32 * value_increase as f32);
            counter += 1;

            self.write_pixel(&Pixel {
                x: x as usize,
                y: y as usize,
                value: value as u8,
            });
        }
    }

    pub fn flush(&mut self) {
        let buffer = mem::replace(&mut self.buffer, new_buffer(self.width, self.height));

        buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| match *value {
                        0 => ' ',
                        value => {
                            let index = (value as f32 / 255.) * ((self.pixel.len() - 1) as f32);
                            self.pixel[index as usize]
                        }
                    })
                    .collect::<String>()
            })
            .for_each(|row| println!("{}", row));
    }
}

fn new_buffer(width: usize, height: usize) -> ScreenBuffer {
    let mut row = Vec::new();
    row.resize(width, 0);

    let mut buffer = Vec::new();
    buffer.resize(height, row);

    buffer
}
