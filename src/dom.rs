use std::collections::HashMap;

/// A Node in the DOM tree.
#[derive(Debug)]
pub struct Node {
    /// Data common to all nodes: list of children
    pub children: Vec<Node>,
    /// Data specific to each node type
    pub node_type: NodeType,
}

/// Different types of DOM nodes we support
#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

/// Data specific to element nodes
#[derive(Debug)]
pub struct ElementData {
    /// Tag name of the element
    pub tag_name: String,
    /// Element attributes stored as name-value pairs
    pub attrs: AttrMap,
}

/// Type alias for attribute map
pub type AttrMap = HashMap<String, String>;

impl Node {
    /// Create a new text node
    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    /// Create a new element node
    pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name,
                attrs,
            }),
        }
    }

    /// Pretty print the DOM tree
    pub fn pretty_print(&self, indent: usize) -> String {
        match &self.node_type {
            NodeType::Text(text) => format!("{:indent$}{}\n", "", text, indent = indent),
            NodeType::Element(elem_data) => {
                let mut result = String::new();
                
                // Print opening tag with attributes
                result.push_str(&format!("{:indent$}<{}", "", elem_data.tag_name, indent = indent));
                for (name, value) in &elem_data.attrs {
                    result.push_str(&format!(" {}=\"{}\"", name, value));
                }
                result.push_str(">\n");

                // Print children
                for child in &self.children {
                    result.push_str(&child.pretty_print(indent + 2));
                }

                // Print closing tag
                result.push_str(&format!("{:indent$}</{}>\n", "", elem_data.tag_name, indent = indent));
                
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_node() {
        let node = Node::text("Hello, world!".to_string());
        assert!(matches!(node.node_type, NodeType::Text(_)));
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_element_node() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "greeting".to_string());
        
        let text_node = Node::text("Hello, world!".to_string());
        let elem = Node::elem(
            "div".to_string(),
            attrs,
            vec![text_node],
        );

        assert!(matches!(elem.node_type, NodeType::Element(_)));
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn test_pretty_print() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "greeting".to_string());
        
        let text_node = Node::text("Hello, world!".to_string());
        let elem = Node::elem(
            "div".to_string(),
            attrs,
            vec![text_node],
        );

        let output = elem.pretty_print(0);
        assert!(output.contains("<div"));
        assert!(output.contains("class=\"greeting\""));
        assert!(output.contains("Hello, world!"));
        assert!(output.contains("</div>"));
    }
}
