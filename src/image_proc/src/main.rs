use image::{DynamicImage, ImageFormat};
use image::imageops::blur;
use image::{Rgba, RgbaImage, Pixel};
use image::imageops::overlay;
use open;

fn main() {
    // Path to the image file you want to display
    let image_path = "../images/new_medieval.jpg";

    // Load the image using the image crate
    let img = image::open(image_path).expect("Failed to load image");

    let mut blurred = blur(&img, 2.5);
    
    let temp_path = "temp_image.png";
    // blurred.save(temp_path).expect("Failed to save temporary image");


    let mut img2 = RgbaImage::new(1000, 1000);
    let start = Rgba::from_slice(&[255, 73, 108, 0]);
    let end = Rgba::from_slice(&[0, 0, 0, 255]);

    image::imageops::horizontal_gradient(&mut img2, start, end);

    overlay(&mut blurred, &img2, 0, 0);

    blurred.save(temp_path).expect("Failed to save temporary image");

    if let Err(e) = open::that(temp_path) {
        println!("Error opening the image viewer: {}", e);
    }
    // Display the image using an external viewer (default image viewer on your system)
    // display_image(blurred);
}

fn display_image(image: DynamicImage) {
    // Create a temporary file to save the image
    let temp_path = "temp_image.png";
    image.save(temp_path).expect("Failed to save temporary image");

    // Try opening the default image viewer to display the image
    if let Err(e) = open::that(temp_path) {
        println!("Error opening the image viewer: {}", e);
    }
}
