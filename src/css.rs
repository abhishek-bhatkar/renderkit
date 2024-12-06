// CSS Parsing Module
//
// This module is like a CSS translator for web browsers
// It breaks down CSS rules into a structured, computer-friendly format
// Think of it like converting a recipe into precise cooking instructions

// Core CSS Data Structures
// These are like different types of cooking tools in our kitchen

/// A complete CSS stylesheet
/// 
/// Imagine this as a complete cookbook with multiple recipes (rules)
#[derive(Debug)]
pub struct Stylesheet {
    /// Collection of CSS rules in the stylesheet
    pub rules: Vec<Rule>,
}

/// A single CSS rule
/// 
/// Like a single recipe in a cookbook, with specific ingredients (selectors) and instructions (declarations)
#[derive(Debug)]
pub struct Rule {
    /// CSS selectors that determine which HTML elements this rule applies to
    pub selectors: Vec<Selector>,
    
    /// CSS property declarations that define how elements should look
    pub declarations: Vec<Declaration>,
}

/// Types of CSS selectors
/// 
/// Currently supports simple selectors, like choosing specific cooking utensils
#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

/// A simple CSS selector
/// 
/// Think of this like a precise description of which kitchen utensil to use
#[derive(Debug)]
pub struct SimpleSelector {
    /// HTML tag name (like 'div', 'p')
    pub tag_name: Option<String>,
    
    /// Element ID
    pub id: Option<String>,
    
    /// CSS classes
    pub class: Vec<String>,
}

/// A CSS property declaration
/// 
/// Like a specific cooking instruction: what to do and how to do it
#[derive(Debug)]
pub struct Declaration {
    /// Property name (like 'color', 'width')
    pub name: String,
    
    /// Value for the property
    pub value: Value,
}

/// Different types of CSS values
/// 
/// Like different types of measurements in cooking
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Keyword values (like 'bold', 'center')
    Keyword(String),
    
    /// Numeric length values
    Length(f32, Unit),
    
    /// Color values
    ColorValue(Color),
}

/// CSS length units
/// 
/// Like different measuring tools in the kitchen
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    /// Pixels, the most basic unit
    Px,
}

/// RGB Color representation
/// 
/// Like mixing colors for painting
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha (transparency) component (0-255)
    pub a: u8,
}

/// Selector specificity calculation type
/// 
/// Used to determine which CSS rule takes precedence
/// Like deciding which recipe instruction is more important
pub type Specificity = (usize, usize, usize);

impl Selector {
    /// Calculate the specificity of a selector
    /// 
    /// Determines how "strong" or "precise" a selector is
    /// Higher specificity means the rule is more likely to be applied
    pub fn specificity(&self) -> Specificity {
        // Based on W3C selector specificity rules
        let Selector::Simple(ref simple) = self;
        let a = simple.id.iter().count();       // ID selectors
        let b = simple.class.len();             // Class selectors
        let c = simple.tag_name.iter().count(); // Tag name selectors
        (a, b, c)
    }
}

impl Value {
    /// Convert a value to pixels
    /// 
    /// Provides a standard way to convert different value types to pixels
    /// Defaults to 0 for non-length values
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0
        }
    }
}

// CSS Parser: The Kitchen Chef of Our CSS Module
/// Parses raw CSS text into structured data
struct Parser {
    /// Current parsing position
    pos: usize,
    /// Raw CSS input string
    input: String,
}

impl Parser {
    // Parsing Helper Methods
    // Like kitchen prep techniques

    /// Get the next character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check if we've reached the end of the input
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consume and return the next character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Consume characters while a test condition is true
    fn consume_while<F>(&mut self, test: F) -> String 
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Skip over whitespace characters
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parse an identifier (like a property name or class)
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    /// Parse a simple CSS selector
    /// 
    /// Like choosing specific kitchen utensils
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        selector
    }

    /// Parse a CSS rule
    /// 
    /// Like following a recipe in a cookbook
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    /// Parse a list of CSS selectors
    /// 
    /// Like choosing multiple kitchen utensils
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break, // start of declarations
                c   => panic!("Unexpected character {} in selector list", c)
            }
        }
        // Return selectors with highest specificity first, for use in matching
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    /// Parse a list of CSS declarations
    /// 
    /// Like following a list of cooking instructions
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.consume_char() == '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    /// Parse a single CSS declaration
    /// 
    /// Like following a single cooking instruction
    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert!(self.consume_char() == ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert!(self.consume_char() == ';');

        Declaration {
            name: property_name,
            value: value,
        }
    }

    /// Parse a CSS value
    /// 
    /// Like measuring ingredients for a recipe
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier())
        }
    }

    /// Parse a numeric length value
    /// 
    /// Like measuring a specific amount of an ingredient
    fn parse_length(&mut self) -> Value {
        let number = self.parse_float();
        let unit = self.parse_unit();
        Value::Length(number, unit)
    }

    /// Parse a floating-point number
    /// 
    /// Like measuring a precise amount of an ingredient
    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| matches!(c, '0'..='9' | '.'));
        s.parse().unwrap()
    }

    /// Parse a unit (like 'px')
    /// 
    /// Like choosing a specific measuring tool
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit")
        }
    }

    /// Parse a color value
    /// 
    /// Like mixing colors for painting
    fn parse_color(&mut self) -> Value {
        assert!(self.consume_char() == '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    /// Parse a hexadecimal color pair
    /// 
    /// Like mixing a specific shade of color
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }
}

// Utility Functions
// Like kitchen helper tools

/// Check if a character is valid in an identifier
fn valid_identifier_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

/// Main entry point for parsing a CSS stylesheet
pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser { pos: 0, input: source };
    Stylesheet { rules: parser.parse_rules() }
}

impl Parser {
    /// Parse a list of CSS rules
    /// 
    /// Like following a list of recipes in a cookbook
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break }
            rules.push(self.parse_rule());
        }
        rules
    }
}

// Test Module: Quality Control for Our CSS Parser
#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing simple CSS selectors
    #[test]
    fn test_parse_simple_selector() {
        let css = "div.note#title { margin: auto; }".to_string();
        let stylesheet = parse(css);
        let rule = &stylesheet.rules[0];
        
        match &rule.selectors[0] {
            Selector::Simple(selector) => {
                assert_eq!(selector.tag_name, Some("div".to_string()));
                assert_eq!(selector.id, Some("title".to_string()));
                assert_eq!(selector.class, vec!["note".to_string()]);
            }
        }
    }

    /// Test parsing CSS declarations
    #[test]
    fn test_parse_declarations() {
        let css = "div { margin: 10px; color: #cc0000; }".to_string();
        let stylesheet = parse(css);
        let rule = &stylesheet.rules[0];
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.declarations[0].name, "margin");
        assert_eq!(rule.declarations[1].name, "color");
    }

    /// Test selector specificity calculation
    #[test]
    fn test_selector_specificity() {
        let css = "div#main.note { margin: auto; }".to_string();
        let stylesheet = parse(css);
        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selectors[0].specificity(), (1, 1, 1));
    }
}
