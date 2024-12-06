// HTML Parser Module: The Digital HTML Translator
// 
// This module is like a skilled interpreter that converts raw HTML text into a structured tree
// It breaks down HTML into meaningful, computer-friendly components
// Think of it like taking a complex, handwritten recipe and turning it into a precise, step-by-step cooking guide
// 
// Key Responsibilities:
// - Parse raw HTML text
// - Create a structured Document Object Model (DOM)
// - Handle nested elements, attributes, and text content
// - Provide a robust, flexible parsing mechanism

use std::collections::HashMap;
use crate::dom;

/// HTML Parser: The Text Navigation Expert
/// 
/// This struct is like a skilled tour guide moving through the landscape of HTML text
/// It keeps track of the current position, remembers the entire text, and knows how to navigate
/// 
/// Imagine it as a finger moving across a page, carefully reading and understanding each character
pub struct Parser {
    /// Current reading position in the text
    /// Like a bookmark that shows exactly where we are in the document
    pos: usize,
    
    /// The entire HTML text to be parsed
    /// Like the complete book we're reading through
    input: String,
}

impl Parser {
    /// Peek at the Next Character: Look Ahead Without Moving
    /// 
    /// Like glancing at the next word without actually turning the page
    /// Provides a preview of what's coming next without consuming the character
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check Text Prefix: Matching the Start of a Sequence
    /// 
    /// Like checking if a sentence begins with a specific phrase
    /// Useful for identifying tags, special characters, or specific patterns
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// End of Text Detection: Journey's End
    /// 
    /// Like knowing when you've reached the last page of a book
    /// Determines if we've consumed all available text
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Character Consumption: Moving Forward in the Text
    /// 
    /// Like taking a bite out of a piece of text
    /// Advances the reading position and returns the current character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Conditional Character Consumption: Selective Text Eating
    /// 
    /// Like eating all the chocolate chips in a cookie that match a specific criteria
    /// Continues consuming characters as long as they satisfy a given condition
    fn consume_while<F>(&mut self, test: F) -> String 
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Whitespace Skipping: Smoothing Out the Text
    /// 
    /// Like ironing out wrinkles in a piece of paper
    /// Removes unnecessary spaces, tabs, and newlines
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Tag Name Extraction: Reading Element Labels
    /// 
    /// Like reading the label on a box or a name tag
    /// Identifies valid tag names using alphanumeric characters
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    /// Parse a Single Node: Identifying the Next Piece of the Puzzle
    /// 
    /// Like identifying whether something is a heading or just a sentence
    /// Determines the type of node (element or text) and parses it accordingly
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    /// Parse Plain Text Content: Reading the Words Between Tags
    /// 
    /// Like reading the words between HTML tags
    /// Consumes text until it encounters a tag or the end of the text
    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c != '<'))
    }

    /// Parse an HTML Element: Unpacking a Nested Russian Doll
    /// 
    /// Like unpacking a nested Russian doll
    /// Handles both opening and closing tags, attributes, and child nodes
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

    /// Parse a Single Attribute: Reading a Name Tag
    /// 
    /// Like reading a name tag at a conference
    /// Identifies the attribute name and value
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parse the Value of an Attribute: Reading What's Written on the Name Tag
    /// 
    /// Like reading what's written on the name tag
    /// Consumes the attribute value until it encounters the closing quote
    fn parse_attr_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '"' || quote == '\'');
        let value = self.consume_while(|c| c != quote);
        assert!(self.consume_char() == quote);
        value
    }

    /// Parse All Attributes of an HTML Element: Collecting Details on a Name Tag
    /// 
    /// Like collecting all the details on a name tag
    /// Continues parsing attributes until it encounters the closing tag
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

    /// Parse Multiple Nodes: Reading Multiple Paragraphs in a Document
    /// 
    /// Like reading multiple paragraphs in a document
    /// Continues parsing nodes until it encounters the end of the text or a closing tag
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

/// Main Parsing Function: Converting HTML Text into a Structured Tree
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
