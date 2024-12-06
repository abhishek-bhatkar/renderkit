// RenderKit: Command-Line Rendering Demonstration

// Import the RenderKit library we've created
use renderkit::RenderKit;

/// Main function - the entry point of our program
/// This is like a showcase of what RenderKit can do
fn main() {
    // Create a new RenderKit rendering engine
    // Think of this like unboxing a new digital art toolkit
    let engine = RenderKit::new();
    
    // Example 1: Render a simple red rectangle
    // This is like drawing a basic shape on a digital canvas
    let canvas = engine.render_rectangle(300.0, 100.0, renderkit::Color { 
        r: 255,   // Full red intensity
        g: 0,     // No green
        b: 0,     // No blue
        a: 255    // Fully opaque
    });

    // Print out some information about our canvas
    println!("Rendered Rectangle Canvas:");
    println!("Width: {} pixels", canvas.width);
    println!("Height: {} pixels", canvas.height);
    println!("Total pixels: {}", canvas.pixels.len());

    // Example 2: Render a simple HTML/CSS example
    // This shows how we can render more complex content
    let html = r#"
        <div class="box">Hello RenderKit!</div>
    "#;
    
    let css = r#"
        .box {
            background: red;   // Red background
            width: 300px;      // 300 pixels wide
            height: 100px;     // 100 pixels tall
        }
    "#;

    // Try to render the HTML with CSS
    match engine.render(html, css) {
        Ok(html_canvas) => {
            println!("\nHTML Rendering Successful!");
            println!("HTML Canvas Width: {} pixels", html_canvas.width);
            println!("HTML Canvas Height: {} pixels", html_canvas.height);
        },
        Err(e) => println!("Error rendering HTML: {}", e),
    }
}

// A few notes about this program:
// 1. It demonstrates two ways of rendering: 
//    - A simple colored rectangle
//    - A more complex HTML+CSS rendering
// 2. It shows how easy it is to use the RenderKit library
// 3. It provides some basic error handling
// 4. It prints out information about the rendered canvases
