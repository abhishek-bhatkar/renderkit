use crate::style::{StyledNode, Display};

/// Represents a rectangular area with position and size
#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Represents edge sizes for padding, border, and margin
#[derive(Debug, Default, Clone, Copy)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl EdgeSizes {
    /// Create an EdgeSizes with all values set to zero
    pub fn zero() -> Self {
        EdgeSizes {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

impl Rect {
    /// Expand a rectangle by adding edge sizes
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

/// Represents the complete dimensions of a box
#[derive(Debug, Default, Clone)]
pub struct Dimensions {
    /// Position of the content area relative to the document origin
    pub content: Rect,

    /// Surrounding edges
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    /// Get the content box rectangle
    pub fn content_box(&self) -> Rect {
        self.content
    }

    /// Get the padding box rectangle
    pub fn padding_box(&self) -> Rect {
        self.content_box().expanded_by(self.padding)
    }

    /// Get the border box rectangle
    pub fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    /// Get the margin box rectangle
    pub fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }

    /// Get the content rectangle
    pub fn content_rect(&self) -> Rect {
        Rect {
            x: self.content.x,
            y: self.content.y,
            width: self.content.width,
            height: self.content.height,
        }
    }

    /// Get the padding rectangle
    pub fn padding_rect(&self) -> Rect {
        Rect {
            x: self.content.x - self.padding.left,
            y: self.content.y - self.padding.top,
            width: self.content.width + self.padding.left + self.padding.right,
            height: self.content.height + self.padding.top + self.padding.bottom,
        }
    }

    /// Get the border rectangle
    pub fn border_rect(&self) -> Rect {
        Rect {
            x: self.content.x - self.padding.left - self.border.left,
            y: self.content.y - self.padding.top - self.border.top,
            width: self.content.width + self.padding.left + self.padding.right 
                   + self.border.left + self.border.right,
            height: self.content.height + self.padding.top + self.padding.bottom 
                    + self.border.top + self.border.bottom,
        }
    }

    /// Get the margin rectangle
    pub fn margin_rect(&self) -> Rect {
        Rect {
            x: self.content.x - self.padding.left - self.border.left - self.margin.left,
            y: self.content.y - self.padding.top - self.border.top - self.margin.top,
            width: self.content.width + self.padding.left + self.padding.right 
                   + self.border.left + self.border.right 
                   + self.margin.left + self.margin.right,
            height: self.content.height + self.padding.top + self.padding.bottom 
                    + self.border.top + self.border.bottom 
                    + self.margin.top + self.margin.bottom,
        }
    }
}

/// Type of layout box
#[derive(Debug)]
pub enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

/// Represents a box in the layout tree
#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    /// Create a new layout box
    pub fn new(box_type: BoxType<'a>) -> LayoutBox<'a> {
        LayoutBox {
            box_type,
            dimensions: Dimensions::default(),
            children: Vec::new(),
        }
    }

    /// Get the style node associated with this layout box
    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block has no style node"),
        }
    }

    /// Lay out a box and its descendants
    pub fn layout(&mut self, containing_block: &Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => {}, // TODO: Implement inline layout
            BoxType::AnonymousBlock => self.layout_block(containing_block),
        }
    }

    /// Layout a block-level box
    fn layout_block(&mut self, containing_block: &Dimensions) {
        // Child width can depend on parent width, so calculate this box's width first
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container
        self.calculate_block_position(containing_block);

        // Recursively lay out the children of this box
        self.layout_block_children();

        // Parent height can depend on child height, so calculate height after children are laid out
        self.calculate_block_height();
    }

    /// Calculate the width of a block-level box with precise CSS spec compliance
    fn calculate_block_width(&mut self, containing_block: &Dimensions) {
        let style = self.get_style_node();

        // Default values
        let auto = crate::css::Value::Keyword("auto".to_string());
        let zero = crate::css::Value::Length(0.0, crate::css::Unit::Px);

        // Retrieve width and margin values with fallback to shorthand properties
        let mut width = style.value("width").unwrap_or(auto.clone());
        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        // Calculate total width of non-auto dimensions
        let total = [
            &margin_left, 
            &margin_right, 
            &border_left, 
            &border_right,
            &padding_left, 
            &padding_right, 
            &width
        ].iter().map(|v| v.to_px()).sum::<f32>();

        // Width constraint handling
        let underflow = containing_block.content.width - total;

        // CSS width calculation algorithm
        match (width == auto, margin_left == auto, margin_right == auto) {
            // Overconstrained: adjust right margin
            (false, false, false) => {
                margin_right = crate::css::Value::Length(
                    margin_right.to_px() + underflow, 
                    crate::css::Unit::Px
                );
            },

            // Exactly one size is auto: adjust that size
            (false, false, true) => { 
                margin_right = crate::css::Value::Length(underflow, crate::css::Unit::Px); 
            },
            (false, true, false) => { 
                margin_left = crate::css::Value::Length(underflow, crate::css::Unit::Px); 
            },

            // Width is auto: handle auto margins
            (true, _, _) => {
                // Reset auto margins to 0
                if margin_left == auto { margin_left = zero.clone(); }
                if margin_right == auto { margin_right = zero.clone(); }

                if underflow >= 0.0 {
                    // Expand width to fill underflow
                    width = crate::css::Value::Length(underflow, crate::css::Unit::Px);
                } else {
                    // Width can't be negative, adjust right margin
                    width = zero.clone();
                    margin_right = crate::css::Value::Length(
                        margin_right.to_px() + underflow, 
                        crate::css::Unit::Px
                    );
                }
            },

            // Both margins auto: center the box
            (false, true, true) => {
                margin_left = crate::css::Value::Length(underflow / 2.0, crate::css::Unit::Px);
                margin_right = crate::css::Value::Length(underflow / 2.0, crate::css::Unit::Px);
            }
        }

        // Store calculated dimensions
        self.dimensions.content.width = width.to_px();
        self.dimensions.margin.left = margin_left.to_px();
        self.dimensions.margin.right = margin_right.to_px();
        self.dimensions.border.left = border_left.to_px();
        self.dimensions.border.right = border_right.to_px();
        self.dimensions.padding.left = padding_left.to_px();
        self.dimensions.padding.right = padding_right.to_px();
    }

    /// Calculate the position of a block-level box
    fn calculate_block_position(&mut self, containing_block: &Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        // Default to zero
        let zero = crate::css::Value::Length(0.0, crate::css::Unit::Px);

        // Set margins, borders, and padding
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();
        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();
        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        // Position the box
        d.content.x = containing_block.content.x + 
                      d.margin.left + 
                      d.border.left + 
                      d.padding.left;

        // Position below previous boxes in the container
        d.content.y = containing_block.content.height + 
                      containing_block.content.y + 
                      d.margin.top + 
                      d.border.top + 
                      d.padding.top;
    }

    /// Layout the children of a block-level box
    fn layout_block_children(&mut self) {
        for child in &mut self.children {
            child.layout(&self.dimensions);
            
            // Increment the height so each child is laid out below the previous one
            self.dimensions.content.height += child.dimensions.margin_box().height;
        }
    }

    /// Calculate the height of a block-level box
    fn calculate_block_height(&mut self) {
        // If height is explicitly set, use that
        if let Some(crate::css::Value::Length(h, crate::css::Unit::Px)) = 
            self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
        // Otherwise, keep the height set by layout_block_children
    }
}

/// Build the layout tree from a style tree
pub fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // Create the root box
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has display: none."),
    });

    // Create descendant boxes
    for child in &style_node.children {
        match child.display() {
            Display::Block => {
                // If the parent is inline but a child is block, create an anonymous block
                if matches!(root.box_type, BoxType::InlineNode(_)) {
                    let mut anon_block = LayoutBox::new(BoxType::AnonymousBlock);
                    anon_block.children.push(build_layout_tree(child));
                    root.children.push(anon_block);
                } else {
                    root.children.push(build_layout_tree(child));
                }
            },
            Display::Inline => {
                // If the parent is a block node, create an anonymous inline container
                if matches!(root.box_type, BoxType::BlockNode(_)) {
                    let mut anon_inline = LayoutBox::new(BoxType::AnonymousBlock);
                    anon_inline.children.push(build_layout_tree(child));
                    root.children.push(anon_inline);
                } else {
                    root.children.push(build_layout_tree(child));
                }
            },
            Display::None => {} // Skip nodes with display: none
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{Node, NodeType, ElementData};
    use crate::style::{StyledNode};
    use crate::css::{Value, Unit};
    use std::collections::HashMap;

    /// Create a test styled node with specific display and width properties
    fn create_test_styled_node(tag: &str, display: &str, width: Option<f32>) -> StyledNode<'static> {
        let mut attrs = HashMap::new();
        attrs.insert("display".to_string(), display.to_string());
        
        let elem = ElementData {
            tag_name: tag.to_string(),
            attrs,
        };

        let node = Node {
            children: vec![],
            node_type: NodeType::Element(elem),
        };

        let mut specified_values = HashMap::new();
        specified_values.insert("display".to_string(), Value::Keyword(display.to_string()));
        
        if let Some(w) = width {
            specified_values.insert("width".to_string(), Value::Length(w, Unit::Px));
        }

        StyledNode {
            node: Box::leak(Box::new(node)),
            specified_values,
            children: vec![],
        }
    }

    /// Create a test dimensions with specific content width and height
    fn create_test_dimensions(width: f32, height: f32) -> Dimensions {
        Dimensions {
            content: Rect { x: 0.0, y: 0.0, width, height },
            padding: EdgeSizes::default(),
            border: EdgeSizes::default(),
            margin: EdgeSizes::default(),
        }
    }

    #[test]
    fn test_layout_box_creation() {
        let style_node = create_test_styled_node("div", "block", Some(100.0));
        let layout_box = LayoutBox::new(BoxType::BlockNode(&style_node));

        assert!(matches!(layout_box.box_type, BoxType::BlockNode(_)));
        assert_eq!(layout_box.children.len(), 0);
    }

    #[test]
    fn test_block_width_calculation() {
        let style_node = create_test_styled_node("div", "block", Some(200.0));
        let containing_block = create_test_dimensions(300.0, 200.0);
        
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&style_node));
        layout_box.calculate_block_width(&containing_block);

        // Check that the width is set correctly
        assert_eq!(layout_box.dimensions.content.width, 200.0);
    }

    #[test]
    fn test_block_width_auto() {
        let style_node = create_test_styled_node("div", "block", None);
        let containing_block = create_test_dimensions(300.0, 200.0);
        
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&style_node));
        layout_box.calculate_block_width(&containing_block);

        // Check that the width fills the containing block
        assert_eq!(layout_box.dimensions.content.width, 300.0);
    }

    #[test]
    fn test_block_position_calculation() {
        let style_node = create_test_styled_node("div", "block", Some(200.0));
        let mut containing_block = create_test_dimensions(300.0, 200.0);
        containing_block.content.height = 100.0; // Simulate previous content height
        
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&style_node));
        layout_box.calculate_block_width(&containing_block);
        layout_box.calculate_block_position(&containing_block);

        // Check x position (should be at the left of containing block)
        assert_eq!(layout_box.dimensions.content.x, 0.0);

        // Check y position (should be below previous content)
        assert_eq!(layout_box.dimensions.content.y, 100.0);
    }

    #[test]
    fn test_nested_block_layout() {
        // Create a parent block
        let parent_style_node = create_test_styled_node("div", "block", Some(300.0));
        let containing_block = create_test_dimensions(400.0, 200.0);
        
        // Create a child block
        let child_style_node = create_test_styled_node("p", "block", Some(250.0));
        
        // Create layout boxes
        let mut parent_layout_box = LayoutBox::new(BoxType::BlockNode(&parent_style_node));
        let child_layout_box = LayoutBox::new(BoxType::BlockNode(&child_style_node));
        
        // Add child to parent
        parent_layout_box.children.push(child_layout_box);
        
        // Layout the parent box
        parent_layout_box.layout(&containing_block);

        // Check parent dimensions
        assert_eq!(parent_layout_box.dimensions.content.width, 300.0);
        
        // Check child dimensions
        let child = &parent_layout_box.children[0];
        assert_eq!(child.dimensions.content.width, 250.0);
    }

    #[test]
    fn test_margin_handling() {
        // Create a style node with margin properties
        let mut attrs = HashMap::new();
        attrs.insert("display".to_string(), "block".to_string());
        attrs.insert("margin-left".to_string(), "20px".to_string());
        attrs.insert("margin-right".to_string(), "auto".to_string());

        let elem = ElementData {
            tag_name: "div".to_string(),
            attrs,
        };

        let node = Node {
            children: vec![],
            node_type: NodeType::Element(elem),
        };

        let mut specified_values = HashMap::new();
        specified_values.insert("display".to_string(), Value::Keyword("block".to_string()));
        specified_values.insert("margin-left".to_string(), Value::Length(20.0, Unit::Px));
        specified_values.insert("margin-right".to_string(), Value::Keyword("auto".to_string()));
        specified_values.insert("width".to_string(), Value::Length(200.0, Unit::Px));

        let style_node = StyledNode {
            node: Box::leak(Box::new(node)),
            specified_values,
            children: vec![],
        };

        let containing_block = create_test_dimensions(300.0, 200.0);
        
        let mut layout_box = LayoutBox::new(BoxType::BlockNode(&style_node));
        layout_box.calculate_block_width(&containing_block);

        // Check margin calculations
        assert_eq!(layout_box.dimensions.margin.left, 20.0);
        assert_eq!(layout_box.dimensions.margin.right, 80.0); // Remaining space
    }
}
