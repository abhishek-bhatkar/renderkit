use renderkit::{RenderKit, Color};
use std::fs::File;
use std::io::Write;

fn main() {
    // Create a new RenderKit instance
    let engine = RenderKit::new();

    // Render a red rectangle
    let canvas = engine.render_rectangle(300.0, 100.0, Color { r: 255, g: 0, b: 0, a: 255 });

    // Print canvas details
    println!("Canvas created:");
    println!("Width: {} pixels", canvas.width);
    println!("Height: {} pixels", canvas.height);
    println!("First pixel color: {:?}", canvas.pixels[0]);
    println!("Total pixels: {}", canvas.pixels.len());

    // Save as PPM image
    save_canvas_as_ppm(&canvas, "output.ppm");

    // Example of rendering HTML/CSS
    let html = r#"
        <div class="box">Hello RenderKit!</div>
    "#;
    
    let css = r#"
        .box {
            background: red;
            width: 300px;
            height: 100px;
        }
    "#;

    match engine.render(html, css) {
        Ok(canvas) => {
            println!("\nHTML rendering successful!");
            save_canvas_as_ppm(&canvas, "output_html.ppm");
        },
        Err(e) => println!("Error rendering HTML: {}", e),
    }
}

fn save_canvas_as_ppm(canvas: &renderkit::Canvas, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    
    // PPM header
    writeln!(file, "P3").expect("Unable to write");
    writeln!(file, "{} {}", canvas.width, canvas.height).expect("Unable to write");
    writeln!(file, "255").expect("Unable to write");

    // Write pixel data
    for pixel in &canvas.pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).expect("Unable to write pixel");
    }

    println!("Canvas saved as {}", filename);
}
