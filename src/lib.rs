// RenderKit: A Simple Web Rendering Engine

// Import core modules that handle different aspects of web rendering
// Think of these as specialized departments in a web rendering factory
pub mod dom;        // Handles the structure of web content
pub mod html;       // Parses HTML into a usable format
pub mod css;        // Understands styling rules
pub mod style;      // Applies styles to HTML elements
pub mod layout;     // Figures out how elements are positioned
pub mod painting;   // Actually draws the content on a canvas

// Re-export commonly used types
// This is like creating a convenient toolbox for users of the library
pub use dom::{Node, NodeType, ElementData};
pub use css::{Color, Value, Stylesheet};
pub use style::StyledNode;
pub use layout::{LayoutBox, BoxType, Rect, EdgeSizes};
pub use painting::{Canvas, DisplayCommand, paint};

use std::collections::HashMap;

/// RenderKit: Your Friendly Neighborhood Web Rendering Engine
///
/// Imagine this as a digital artist that takes HTML and CSS and turns them 
/// into a beautiful, rendered image. It breaks down the process into simple steps:
/// 1. Parse HTML (understand the content)
/// 2. Parse CSS (understand the styling)
/// 3. Create a styled document
/// 4. Layout the elements
/// 5. Paint the final image
pub struct RenderKit;

impl RenderKit {
    /// Create a new RenderKit - like unboxing a new rendering toolkit
    pub fn new() -> Self {
        RenderKit
    }

    /// Render HTML with CSS - the main magic happens here!
    ///
    /// # What this does:
    /// - Takes raw HTML and CSS as input
    /// - Transforms them into a visual representation
    /// - Returns a Canvas (think of it like a digital painting)
    ///
    /// # Example
    /// ```
    /// let engine = RenderKit::new();
    /// let result = engine.render("<div>Hello World</div>", "div { background: red; }");
    /// ```
    pub fn render(&self, html: &str, css: &str) -> Result<Canvas, String> {
        // Step 1: Parse HTML into a tree-like structure (DOM)
        let dom = html::parse(html.to_string());
        
        // Step 2: Parse CSS rules
        let stylesheet = css::parse(css.to_string());
        
        // Step 3: Apply CSS styles to HTML elements
        let styled_node = style::style_tree(&dom, &stylesheet);
        
        // Step 4: Calculate layout (where things should be positioned)
        let layout_root = layout::build_layout_tree(&styled_node);
        
        // Step 5: Paint the final image
        let canvas = painting::paint(&layout_root, layout_root.dimensions.content);
        
        Ok(canvas)
    }

    /// Create a simple colored rectangle - perfect for testing or simple graphics
    ///
    /// # What this does:
    /// - Creates a rectangular canvas with a specific color
    /// - Useful for understanding basic rendering
    ///
    /// # Parameters
    /// - `width`: How wide the rectangle should be
    /// - `height`: How tall the rectangle should be
    /// - `color`: What color to fill the rectangle with
    pub fn render_rectangle(&self, width: f32, height: f32, color: Color) -> Canvas {
        // Create a dummy HTML-like node to represent our rectangle
        let node = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attrs: HashMap::new(),
            }),
        };

        // Apply the color as a style
        let styled_node = StyledNode {
            node: &node,
            specified_values: {
                let mut map = HashMap::new();
                map.insert("background".to_string(), Value::ColorValue(color));
                map
            },
            children: vec![],
        };

        // Prepare the layout for our rectangle
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&styled_node));
        
        // Set the exact dimensions
        layout_box.dimensions.content = Rect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };

        // Paint and return the canvas
        painting::paint(&layout_box, layout_box.dimensions.content)
    }
}

// Test module - this is like a quality control department
#[cfg(test)]
mod tests {
    use super::*;

    /// Test creating a simple red rectangle
    #[test]
    fn test_render_rectangle() {
        let engine = RenderKit::new();
        let canvas = engine.render_rectangle(100.0, 100.0, Color { r: 255, g: 0, b: 0, a: 255 });
        
        // Verify the canvas matches our expectations
        assert_eq!(canvas.width, 100);
        assert_eq!(canvas.height, 100);
        assert_eq!(canvas.pixels[0], Color { r: 255, g: 0, b: 0, a: 255 });
    }

    /// Test rendering a simple HTML div
    #[test]
    fn test_render_simple_html() {
        let engine = RenderKit::new();
        let html = r#"<div>Hello</div>"#;
        let css = "div { background: red; }";
        let result = engine.render(html, css);
        assert!(result.is_ok());
    }
}
