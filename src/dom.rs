// DOM (Document Object Model) Module
//
// This module is like a digital blueprint of a web page
// It breaks down HTML into a tree-like structure that computers can understand
// Think of it like turning a recipe into a step-by-step cooking guide

use std::collections::HashMap;

/// A Node in the DOM tree
/// 
/// Imagine this as a single piece in a complex LEGO structure
/// Each node can be either text or an HTML element, and can have child nodes
#[derive(Debug, Clone)]
pub struct Node {
    /// Children of this node - like smaller LEGO pieces attached to a main piece
    pub children: Vec<Node>,
    
    /// The specific type of this node (text or element)
    /// Like a label that tells you what kind of LEGO piece this is
    pub node_type: NodeType,
}

/// Different types of nodes a web page can have
/// 
/// This is like having different types of LEGO pieces:
/// - Text: Just words
/// - Element: A full HTML tag with potential attributes
#[derive(Debug, Clone)]
pub enum NodeType {
    /// Plain text content
    Text(String),
    
    /// HTML element with a tag name and attributes
    Element(ElementData),
}

/// Detailed information about an HTML element
/// 
/// Think of this like a detailed instruction card for a specific LEGO piece
#[derive(Debug, Clone)]
pub struct ElementData {
    /// The name of the HTML tag (like "div", "p", "span")
    pub tag_name: String,
    
    /// Attributes of the element (like class, id, style)
    /// Like additional stickers or special features on a LEGO piece
    pub attrs: AttrMap,
}

/// A convenient way to store attributes as key-value pairs
/// Like a name tag with multiple details
pub type AttrMap = HashMap<String, String>;

impl Node {
    /// Create a new text node - like making a simple text LEGO piece
    /// 
    /// # Example
    /// ```
    /// let text_node = Node::text("Hello, world!".to_string());
    /// ```
    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    /// Create a new element node - like assembling a complex LEGO structure
    /// 
    /// # Parameters
    /// - `tag_name`: The type of HTML element
    /// - `attrs`: Attributes for the element
    /// - `children`: Child nodes inside this element
    /// 
    /// # Example
    /// ```
    /// let div = Node::elem(
    ///     "div".to_string(), 
    ///     HashMap::new(), 
    ///     vec![Node::text("Hello".to_string())]
    /// );
    /// ```
    pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name,
                attrs,
            }),
        }
    }

    /// Pretty print the DOM tree - like creating an instruction manual
    /// 
    /// This method converts the DOM tree into a human-readable format
    /// It shows the structure with proper indentation and all details
    /// 
    /// # Parameters
    /// - `indent`: How many spaces to indent each level
    /// 
    /// # Returns
    /// A string representation of the DOM tree
    pub fn pretty_print(&self, indent: usize) -> String {
        match &self.node_type {
            NodeType::Text(text) => format!("{:indent$}{}\n", "", text, indent = indent),
            NodeType::Element(elem_data) => {
                let mut result = String::new();
                
                // Print opening tag with attributes
                // Like writing the first line of a LEGO instruction
                result.push_str(&format!("{:indent$}<{}", "", elem_data.tag_name, indent = indent));
                for (name, value) in &elem_data.attrs {
                    result.push_str(&format!(" {}=\"{}\"", name, value));
                }
                result.push_str(">\n");

                // Print children
                // Like showing how smaller LEGO pieces connect
                for child in &self.children {
                    result.push_str(&child.pretty_print(indent + 2));
                }

                // Print closing tag
                // Like finishing the LEGO instruction
                result.push_str(&format!("{:indent$}</{}>\n", "", elem_data.tag_name, indent = indent));
                
                result
            }
        }
    }
}

// Test Module: Quality Control for our DOM Builder
// These tests are like checking that our LEGO instructions work correctly
#[cfg(test)]
mod tests {
    use super::*;

    /// Test creating a simple text node
    /// Like checking if we can make a basic LEGO piece
    #[test]
    fn test_text_node() {
        let node = Node::text("Hello, world!".to_string());
        assert!(matches!(node.node_type, NodeType::Text(_)));
        assert_eq!(node.children.len(), 0);
    }

    /// Test creating an element node with a child
    /// Like assembling a more complex LEGO structure
    #[test]
    fn test_element_node() {
        // Create attributes for our element
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "greeting".to_string());
        
        // Create a text node to use as a child
        let text_node = Node::text("Hello, world!".to_string());
        
        // Create a div element with the text node
        let elem = Node::elem(
            "div".to_string(),
            attrs,
            vec![text_node],
        );

        assert!(matches!(elem.node_type, NodeType::Element(_)));
        assert_eq!(elem.children.len(), 1);
    }

    /// Test the pretty print functionality
    /// Like checking if our LEGO instruction manual looks correct
    #[test]
    fn test_pretty_print() {
        // Create attributes for our element
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "greeting".to_string());
        
        // Create a text node to use as a child
        let text_node = Node::text("Hello, world!".to_string());
        
        // Create a div element with the text node
        let elem = Node::elem(
            "div".to_string(),
            attrs,
            vec![text_node],
        );

        // Generate the pretty-printed output
        let output = elem.pretty_print(0);
        
        // Check that all expected parts are in the output
        assert!(output.contains("<div"));
        assert!(output.contains("class=\"greeting\""));
        assert!(output.contains("Hello, world!"));
        assert!(output.contains("</div>"));
    }
}
