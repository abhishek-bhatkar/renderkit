// HTML Parser Module
// 
// This module is like a translator that converts raw HTML text into a structured tree
// It breaks down HTML into meaningful parts that a computer can understand
// Think of it like taking a recipe and turning it into a step-by-step cooking guide

use std::collections::HashMap;
use crate::dom;

/// HTML Parser: The HTML Text Translator
/// 
/// This struct keeps track of where we are while reading the HTML
/// It's like a finger moving across a page, keeping track of which part we're currently reading
pub struct Parser {
    pos: usize,        // Current position in the text
    input: String,     // The entire HTML text
}

impl Parser {
    /// Peek at the next character without moving forward
    /// Like looking ahead one step without actually taking the step
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check if the text starts with a specific string
    /// Like checking if a sentence begins with a certain word
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Check if we've reached the end of the text
    /// Like knowing when you've read the last page of a book
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Move forward and "eat" the next character
    /// Like taking a bite out of a piece of text
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Consume characters as long as they match a certain condition
    /// Like eating all the chocolate chips in a cookie
    fn consume_while<F>(&mut self, test: F) -> String 
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Skip over any whitespace (spaces, tabs, newlines)
    /// Like smoothing out wrinkles in a piece of paper
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Extract a tag name (like "div" or "p")
    /// Like reading the label on a box
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    /// Parse a single node in the HTML
    /// This could be an element (like a div) or just text
    /// Like identifying whether something is a heading or just a sentence
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    /// Parse plain text content
    /// Like reading the words between HTML tags
    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c != '<'))
    }

    /// Parse an HTML element with its attributes and children
    /// This is like unpacking a nested Russian doll
    /// It handles both opening and closing tags
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents (children)
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::Node::elem(tag_name, attrs, children)
    }

    /// Parse a single attribute (like class="example")
    /// Like reading a name tag at a conference
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parse the value of an attribute
    /// Like reading what's written on the name tag
    fn parse_attr_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '"' || quote == '\'');
        let value = self.consume_while(|c| c != quote);
        assert!(self.consume_char() == quote);
        value
    }

    /// Parse all attributes of an HTML element
    /// Like collecting all the details on a name tag
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    /// Parse multiple nodes
    /// Like reading multiple paragraphs in a document
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }
}

/// Main parsing function: Convert HTML text into a structured tree
/// 
/// # What this does:
/// - Takes raw HTML text as input
/// - Breaks it down into a tree-like structure
/// - Ensures there's always a root element
///
/// # Examples
/// ```
/// let html = "<div>Hello World</div>";
/// let parsed_node = parse(html.to_string());
/// ```
pub fn parse(source: String) -> dom::Node {
    let mut parser = Parser {
        pos: 0,
        input: source
    };
    
    let mut nodes = parser.parse_nodes();

    // If there's only one root element, return it
    // Otherwise, wrap everything in an <html> tag
    if nodes.len() == 1 && matches!(nodes[0].node_type, dom::NodeType::Element(_)) {
        nodes.remove(0)
    } else {
        dom::Node::elem("html".to_string(), HashMap::new(), nodes)
    }
}

// Test module: Quality Control for our HTML Parser
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::NodeType;

    /// Test parsing simple text
    #[test]
    fn test_parse_text() {
        let html = String::from("Hello, world!");
        let node = parse(html);
        // Text should be wrapped in an html element
        assert!(matches!(node.node_type, NodeType::Element(_)));
        assert_eq!(node.children.len(), 1);
        if let NodeType::Text(text) = &node.children[0].node_type {
            assert_eq!(text, "Hello, world!");
        } else {
            panic!("Expected text node");
        }
    }

    /// Test parsing a simple HTML element
    #[test]
    fn test_parse_element() {
        let html = String::from("<div>Hello</div>");
        let node = parse(html);
        assert!(matches!(node.node_type, NodeType::Element(_)));
        if let NodeType::Element(data) = &node.node_type {
            assert_eq!(data.tag_name, "div");
        }
    }

    /// Test parsing HTML attributes
    #[test]
    fn test_parse_attributes() {
        let html = String::from(r#"<div class="greeting" id="message">Hello</div>"#);
        let node = parse(html);
        if let NodeType::Element(data) = &node.node_type {
            assert_eq!(data.attrs.get("class").unwrap(), "greeting");
            assert_eq!(data.attrs.get("id").unwrap(), "message");
        }
    }

    /// Test parsing nested HTML elements
    #[test]
    fn test_parse_nested() {
        let html = String::from(r#"
            <html>
                <body>
                    <h1>Title</h1>
                    <div id="main" class="test">
                        <p>Hello <em>world</em>!</p>
                    </div>
                </body>
            </html>
        "#);
        let node = parse(html);
        assert!(matches!(node.node_type, NodeType::Element(_)));
        if let NodeType::Element(data) = &node.node_type {
            assert_eq!(data.tag_name, "html");
        }
    }
}
