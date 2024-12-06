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
    /// The area covered by the content area plus its padding
    fn padding_box(&self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    /// The area covered by the content area plus padding and borders
    fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    /// The area covered by the content area plus padding, borders, and margin
    fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(self.margin)
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
    use crate::css::Stylesheet;
    use std::collections::HashMap;

    // Helper function to print layout tree details
    fn print_layout_tree(layout_box: &LayoutBox, indent: usize) {
        let indent_str = " ".repeat(indent * 2);
        match &layout_box.box_type {
            BoxType::BlockNode(_) => println!("{}BlockNode", indent_str),
            BoxType::InlineNode(_) => println!("{}InlineNode", indent_str),
            BoxType::AnonymousBlock => println!("{}AnonymousBlock", indent_str),
        }
        for child in &layout_box.children {
            print_layout_tree(child, indent + 1);
        }
    }

    #[test]
    fn test_build_layout_tree() {
        // Create a simple DOM tree
        let mut root_attrs = HashMap::new();
        root_attrs.insert("display".to_string(), "block".to_string());
        
        let root_elem = ElementData {
            tag_name: "div".to_string(),
            attrs: root_attrs.clone(),
        };

        let child1_elem = ElementData {
            tag_name: "span".to_string(),
            attrs: {
                let mut child_attrs = HashMap::new();
                child_attrs.insert("display".to_string(), "inline".to_string());
                child_attrs
            },
        };

        let child2_elem = ElementData {
            tag_name: "p".to_string(),
            attrs: {
                let mut child_attrs = HashMap::new();
                child_attrs.insert("display".to_string(), "block".to_string());
                child_attrs
            },
        };

        let child1_node = Node {
            children: vec![],
            node_type: NodeType::Element(child1_elem.clone()),
        };

        let child2_node = Node {
            children: vec![],
            node_type: NodeType::Element(child2_elem.clone()),
        };

        let root_node = Node {
            children: vec![child1_node.clone(), child2_node.clone()],
            node_type: NodeType::Element(root_elem.clone()),
        };

        // Create a dummy stylesheet
        let _stylesheet = Stylesheet { rules: vec![] };

        // Manually create StyledNode with display property
        let style_node = crate::style::StyledNode {
            node: &root_node,
            specified_values: {
                let mut values = HashMap::new();
                values.insert("display".to_string(), crate::css::Value::Keyword("block".to_string()));
                values
            },
            children: vec![
                crate::style::StyledNode {
                    node: &child1_node,
                    specified_values: {
                        let mut values = HashMap::new();
                        values.insert("display".to_string(), crate::css::Value::Keyword("inline".to_string()));
                        values
                    },
                    children: vec![],
                },
                crate::style::StyledNode {
                    node: &child2_node,
                    specified_values: {
                        let mut values = HashMap::new();
                        values.insert("display".to_string(), crate::css::Value::Keyword("block".to_string()));
                        values
                    },
                    children: vec![],
                }
            ],
        };

        // Debug print the style node
        println!("Style Node Display: {:?}", style_node.display());
        println!("Style Node Specified Values: {:?}", style_node.specified_values);
        println!("Number of Children: {}", style_node.children.len());

        // Build layout tree
        let layout_tree = build_layout_tree(&style_node);

        // Print layout tree structure
        println!("Layout Tree Structure:");
        print_layout_tree(&layout_tree, 0);

        // Verify root node
        assert!(matches!(layout_tree.box_type, BoxType::BlockNode(_)), "Root node should be a block node");
        assert_eq!(layout_tree.children.len(), 2, "Root node should have 2 children");

        // Verify first child (inline)
        match layout_tree.children[0].box_type {
            BoxType::InlineNode(_) => {},
            BoxType::AnonymousBlock => {
                // If an anonymous block was created, check its first child
                assert!(matches!(layout_tree.children[0].children[0].box_type, BoxType::InlineNode(_)), 
                        "First child should be an inline node");
            },
            _ => panic!("First child should be an inline node or in an anonymous block"),
        }

        // Verify second child (block)
        assert!(matches!(layout_tree.children[1].box_type, BoxType::BlockNode(_)), "Second child should be a block node");
    }
}
