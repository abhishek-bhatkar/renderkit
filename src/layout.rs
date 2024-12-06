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
