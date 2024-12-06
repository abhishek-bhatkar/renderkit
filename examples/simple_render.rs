use web_browser::dom::{Node, NodeType, ElementData};
use web_browser::css::{Color, Value};
use web_browser::style::StyledNode;
use web_browser::layout::{LayoutBox, BoxType, Rect, EdgeSizes};
use web_browser::painting::{paint};
use std::collections::HashMap;

// Struct to hold owned nodes and styled nodes
struct RenderBox {
    _node: Box<Node>,
    styled_node: StyledNode<'static>,
    rect: Rect,
}

// Modify the function to return a RenderBox
fn create_styled_node(tag: &str, background: Color, width: f32, height: f32) -> RenderBox {
    let mut attrs = HashMap::new();
    attrs.insert("class".to_string(), tag.to_string());

    let elem = ElementData {
        tag_name: tag.to_string(),
        attrs,
    };

    let node = Box::new(Node {
        children: vec![],
        node_type: NodeType::Element(elem),
    });

    let mut specified_values = HashMap::new();
    specified_values.insert("display".to_string(), Value::Keyword("block".to_string()));
    specified_values.insert("background".to_string(), 
        Value::ColorValue(background)); 

    // Use Box::leak to create a static reference
    let styled_node = StyledNode {
        node: Box::leak(node.clone()),
        specified_values,
        children: vec![],
    };

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };

    RenderBox {
        _node: node,
        styled_node,
        rect,
    }
}

fn main() {
    // Create multiple render boxes with different colors
    let mut render_boxes = vec![
        create_styled_node("red-box", 
            Color { r: 255, g: 0, b: 0, a: 255 },  // Red
            300.0, 100.0
        ),
        create_styled_node("green-box", 
            Color { r: 0, g: 255, b: 0, a: 255 },  // Green
            300.0, 100.0
        ),
    ];

    // Create layout boxes
    let layout_boxes: Vec<LayoutBox> = render_boxes.iter_mut().enumerate().map(|(i, render_box)| {
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&render_box.styled_node));
        
        // Set dimensions and vertical positioning
        layout_box.dimensions.content = Rect {
            x: 0.0,
            y: (i as f32) * 100.0,
            width: render_box.rect.width,
            height: render_box.rect.height,
        };
        layout_box.dimensions.padding = EdgeSizes::zero();
        layout_box.dimensions.border = EdgeSizes::zero();
        layout_box.dimensions.margin = EdgeSizes::zero();

        layout_box
    }).collect();

    // Paint the layout boxes
    let canvas = paint(&layout_boxes[0], Rect { 
        x: 0.0, 
        y: 0.0, 
        width: 300.0, 
        height: 200.0 
    });

    // Print canvas details
    println!("Canvas created:");
    println!("Width: {} pixels", canvas.width);
    println!("Height: {} pixels", canvas.height);
    println!("First pixel color: {:?}", canvas.pixels[0]);
    println!("Pixel at row 100: {:?}", canvas.pixels[100 * canvas.width]);
    println!("Total pixels: {}", canvas.pixels.len());

    // Optional: Save canvas as a simple PPM image
    save_canvas_as_ppm(&canvas, "output.ppm");
}

fn save_canvas_as_ppm(canvas: &web_browser::painting::Canvas, filename: &str) {
    use std::fs::File;
    use std::io::Write;

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
