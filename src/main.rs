use renderkit::RenderKit;

fn main() {
    let engine = RenderKit::new();
    
    // Example of rendering a simple rectangle
    let canvas = engine.render_rectangle(300.0, 100.0, renderkit::Color { 
        r: 255, 
        g: 0, 
        b: 0, 
        a: 255 
    });

    println!("Rendered canvas:");
    println!("Width: {} pixels", canvas.width);
    println!("Height: {} pixels", canvas.height);
    println!("Total pixels: {}", canvas.pixels.len());

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
        Ok(html_canvas) => {
            println!("\nHTML rendering successful!");
            println!("HTML Canvas Width: {} pixels", html_canvas.width);
            println!("HTML Canvas Height: {} pixels", html_canvas.height);
        },
        Err(e) => println!("Error rendering HTML: {}", e),
    }
}
