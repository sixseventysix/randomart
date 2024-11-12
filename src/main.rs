use image::{ImageBuffer, RgbImage};

const HEIGHT: u32 = 400;
const WIDTH: u32 = 400;

struct PixelCoordinates {
    x: f32,
    y: f32
}

struct Colour {
    r: f32,
    g: f32,
    b: f32
}

fn fmod(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        0.0 
    } else {
        x - (x / y).trunc() * y
    }
}

fn what_do_i_even_call_this(pixel_coordinates: PixelCoordinates) -> Colour {
    let x = pixel_coordinates.x;
    let y = pixel_coordinates.y;

    if x * y > 0.0 {
        Colour { r: x, g: y, b: 1.0 }
    } else {
        let value = fmod(x, y);
        Colour {
            r: value,
            g: value,
            b: value,
        }
    }
} 

fn gray_gradient(pixel_coordinates: PixelCoordinates) -> Colour {
    Colour { r: pixel_coordinates.x, g: pixel_coordinates.x, b: pixel_coordinates.x }
}

fn render_pixels<F>(function: F) -> RgbImage 
where
    F: Fn(PixelCoordinates) -> Colour 
{
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (px, py, pixel) in img.enumerate_pixels_mut() {
        let x = (px as f32 / (WIDTH - 1) as f32) * 2.0 - 1.0;
        let y = (py as f32 / (HEIGHT - 1) as f32) * 2.0 - 1.0;

        let colour = function(PixelCoordinates { x, y });

        let r = ((colour.r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        let g = ((colour.g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        let b = ((colour.b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

        *pixel = image::Rgb([r, g, b]);
    }
    img
}

fn simple_hash(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; 
    let prime: u64 = 0x100000001b3;

    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(prime); 
    }

    hash
}

fn main() {
    let img = render_pixels(what_do_i_even_call_this);
    img.save("data/output.png").expect("failed to save the image");
}