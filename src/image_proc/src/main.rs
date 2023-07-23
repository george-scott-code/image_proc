use image::{DynamicImage, ImageFormat};
use image::imageops::blur;
use open;

fn main() {
    // Path to the image file you want to display
    let image_path = "../images/new_medieval.jpg";

    // Load the image using the image crate
    let img = image::open(image_path).expect("Failed to load image");

    let blurred = blur(&img, 2.5);
    
    let temp_path = "temp_image.png";
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
