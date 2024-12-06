use crate::layout::{LayoutBox, BoxType, Rect as LayoutRect};
use crate::css::{Value, Color};

/// Represents a single drawing command
#[derive(Debug, Clone)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    // TODO: Add more display commands like text, border, etc.
}

/// Display list is a collection of drawing commands
pub type DisplayList = Vec<DisplayCommand>;

/// Represents a rectangular area
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<LayoutRect> for Rect {
    fn from(layout_rect: LayoutRect) -> Self {
        Rect {
            x: layout_rect.x,
            y: layout_rect.y,
            width: layout_rect.width,
            height: layout_rect.height,
        }
    }
}

/// Canvas for rendering pixels
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Clone for Canvas {
    fn clone(&self) -> Self {
        Canvas {
            pixels: self.pixels.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl Canvas {
    /// Create a new blank canvas with white background
    pub fn new(width: usize, height: usize) -> Self {
        let white = Color { r: 255, g: 255, b: 255, a: 255 };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }

    /// Paint a single display command onto the canvas
    pub fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to canvas boundaries
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        // Simple pixel painting (no alpha blending yet)
                        self.pixels[x + y * self.width] = color.clone();
                    }
                }
            }
        }
    }
}

/// Helper function to get color for a specific CSS property
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => {
            match style.value(name) {
                Some(Value::ColorValue(color)) => Some(color.clone()),
                _ => None
            }
        },
        BoxType::AnonymousBlock => None
    }
}

/// Render background for a layout box
fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color, 
            layout_box.dimensions.border_box().into()
        ));
    });
}

/// Render borders for a layout box
fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return // No border color specified
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(color.clone(), Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(DisplayCommand::SolidColor(color.clone(), Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(DisplayCommand::SolidColor(color.clone(), Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}

/// Recursively render a layout box and its children
fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    println!("Rendering layout box: {:?}", layout_box);
    render_background(list, layout_box);
    render_borders(list, layout_box);

    // Recursively render children
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

/// Build a display list from a layout tree
pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

/// Paint a layout tree to a canvas
pub fn paint(layout_root: &LayoutBox, bounds: LayoutRect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);

    for item in display_list {
        println!("Painting item: {:?}", item);
        canvas.paint_item(&item);
    }

    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{Node, NodeType, ElementData};
    use crate::style::StyledNode;
    use std::collections::HashMap;

    /// Create a test styled node with a background color
    fn create_test_styled_node(tag: &str, background: Color) -> StyledNode<'static> {
        let mut attrs = HashMap::new();
        attrs.insert("display".to_string(), "block".to_string());

        let elem = ElementData {
            tag_name: tag.to_string(),
            attrs,
        };

        let node = Node {
            children: vec![],
            node_type: NodeType::Element(elem),
        };

        let mut specified_values = HashMap::new();
        specified_values.insert("display".to_string(), Value::Keyword("block".to_string()));
        specified_values.insert("background".to_string(), Value::ColorValue(background.clone()));

        StyledNode {
            node: Box::leak(Box::new(node)),
            specified_values,
            children: vec![],
        }
    }

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(100, 100);
        assert_eq!(canvas.width, 100);
        assert_eq!(canvas.height, 100);
        assert_eq!(canvas.pixels.len(), 10000);
    }

    #[test]
    fn test_display_list_generation() {
        let red = Color { r: 255, g: 0, b: 0, a: 255 };
        let style_node = create_test_styled_node("div", red.clone());
        
        let mut layout_box = crate::layout::LayoutBox::new(
            crate::layout::BoxType::BlockNode(&style_node)
        );
        
        // Manually set dimensions to ensure painting works
        layout_box.dimensions.content.x = 0.0;
        layout_box.dimensions.content.y = 0.0;
        layout_box.dimensions.content.width = 100.0;
        layout_box.dimensions.content.height = 100.0;
        layout_box.dimensions.padding = crate::layout::EdgeSizes::zero();
        layout_box.dimensions.border = crate::layout::EdgeSizes::zero();
        layout_box.dimensions.margin = crate::layout::EdgeSizes::zero();

        let display_list = build_display_list(&layout_box);
        assert_eq!(display_list.len(), 1);

        match &display_list[0] {
            DisplayCommand::SolidColor(color, _) => {
                assert_eq!(*color, red);
            }
        }
    }

    #[test]
    fn test_paint_single_box() {
        let red = Color { r: 255, g: 0, b: 0, a: 255 };
        let style_node = create_test_styled_node("div", red.clone());
        
        let mut layout_box = crate::layout::LayoutBox::new(
            crate::layout::BoxType::BlockNode(&style_node)
        );
        
        // Manually set dimensions to ensure painting works
        layout_box.dimensions.content.x = 0.0;
        layout_box.dimensions.content.y = 0.0;
        layout_box.dimensions.content.width = 100.0;
        layout_box.dimensions.content.height = 100.0;
        layout_box.dimensions.padding = crate::layout::EdgeSizes::zero();
        layout_box.dimensions.border = crate::layout::EdgeSizes::zero();
        layout_box.dimensions.margin = crate::layout::EdgeSizes::zero();

        let canvas = paint(&layout_box, LayoutRect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 });
        
        // Check that the first pixel is red
        assert_eq!(canvas.pixels[0], red);
    }
}
