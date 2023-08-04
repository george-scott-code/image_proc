use image::Pixel;
use open;

fn main() {
    let image_path = "../images/new_medieval.jpg";

    let img = image::open(image_path).expect("Failed to load image");
    let temp_path = "temp_image.png";
    let mut rgba = img.to_rgba8();

    const CLEARPIXEL: image::Rgba<u8> = image::Rgba([0,0,0,40]);

    for pixel in rgba.enumerate_pixels_mut() {
        if pixel.1 % 3 != 0 {
            pixel.2.blend(&CLEARPIXEL);
        }
    } 

    rgba.save(temp_path).expect("Failed to save temporary image");

    if let Err(e) = open::that(temp_path) {
        println!("Error opening the image viewer: {}", e);
    }
}
