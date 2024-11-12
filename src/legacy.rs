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