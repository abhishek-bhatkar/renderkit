pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod painting;

// Re-export commonly used types
pub use dom::{Node, NodeType, ElementData};
pub use css::{Color, Value, Stylesheet};
pub use style::StyledNode;
pub use layout::{LayoutBox, BoxType, Rect, EdgeSizes};
pub use painting::{Canvas, DisplayCommand, paint};

use std::collections::HashMap;

/// RenderKit is a minimal web rendering engine library written in Rust.
/// It provides basic building blocks for parsing HTML/CSS and rendering web content.
pub struct RenderKit;

impl RenderKit {
    /// Creates a new RenderKit instance
    pub fn new() -> Self {
        RenderKit
    }

    /// Renders HTML content with given CSS styles
    pub fn render(&self, html: &str, css: &str) -> Result<Canvas, String> {
        // Parse HTML into DOM
        let dom = html::parse(html.to_string());
        
        // Parse CSS
        let stylesheet = css::parse(css.to_string());
        
        // Apply styles to DOM
        let styled_node = style::style_tree(&dom, &stylesheet);
        
        // Create layout tree
        let layout_root = layout::build_layout_tree(&styled_node);
        
        // Paint the layout tree
        let canvas = painting::paint(&layout_root, layout_root.dimensions.content);
        
        Ok(canvas)
    }

    /// Renders a simple colored rectangle
    pub fn render_rectangle(&self, width: f32, height: f32, color: Color) -> Canvas {
        // Create a owned Node
        let node = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attrs: HashMap::new(),
            }),
        };

        // Create a owned StyledNode
        let styled_node = StyledNode {
            node: &node,
            specified_values: {
                let mut map = HashMap::new();
                map.insert("background".to_string(), Value::ColorValue(color));
                map
            },
            children: vec![],
        };

        // Create a LayoutBox
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&styled_node));
        
        // Set dimensions
        layout_box.dimensions.content = Rect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };

        // Paint and return canvas
        painting::paint(&layout_box, layout_box.dimensions.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_rectangle() {
        let engine = RenderKit::new();
        let canvas = engine.render_rectangle(100.0, 100.0, Color { r: 255, g: 0, b: 0, a: 255 });
        assert_eq!(canvas.width, 100);
        assert_eq!(canvas.height, 100);
        assert_eq!(canvas.pixels[0], Color { r: 255, g: 0, b: 0, a: 255 });
    }

    #[test]
    fn test_render_simple_html() {
        let engine = RenderKit::new();
        let html = r#"<div>Hello</div>"#;
        let css = "div { background: red; }";
        let result = engine.render(html, css);
        assert!(result.is_ok());
    }
}
