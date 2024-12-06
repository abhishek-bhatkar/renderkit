
use std::collections::HashMap;
use crate::dom;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String 
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::Node::elem(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '"' || quote == '\'');
        let value = self.consume_while(|c| c != quote);
        assert!(self.consume_char() == quote);
        value
    }

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

/// Parse an HTML document and return the root element
pub fn parse(source: String) -> dom::Node {
    let mut parser = Parser {
        pos: 0,
        input: source
    };
    
    let mut nodes = parser.parse_nodes();

    // If the document contains a root element, just return it.
    // Otherwise, create one and wrap all nodes in it.
    if nodes.len() == 1 && matches!(nodes[0].node_type, dom::NodeType::Element(_)) {
        nodes.remove(0)
    } else {
        dom::Node::elem("html".to_string(), HashMap::new(), nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::NodeType;

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

    #[test]
    fn test_parse_element() {
        let html = String::from("<div>Hello</div>");
        let node = parse(html);
        assert!(matches!(node.node_type, NodeType::Element(_)));
        if let NodeType::Element(data) = &node.node_type {
            assert_eq!(data.tag_name, "div");
        }
    }

    #[test]
    fn test_parse_attributes() {
        let html = String::from(r#"<div class="greeting" id="message">Hello</div>"#);
        let node = parse(html);
        if let NodeType::Element(data) = &node.node_type {
            assert_eq!(data.attrs.get("class").unwrap(), "greeting");
            assert_eq!(data.attrs.get("id").unwrap(), "message");
        }
    }

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
