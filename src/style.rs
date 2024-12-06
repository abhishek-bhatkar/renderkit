// Style Module: The Fashion Designer of Web Rendering
//
// This module is like a stylist for web elements
// It transforms raw HTML elements into styled, visually rich components
// Think of it as turning basic clothing into a fashionable outfit

use std::collections::{HashMap, HashSet};
use crate::dom::{Node, NodeType, ElementData};
use crate::css::{Stylesheet, Rule, Selector, SimpleSelector, Specificity, Value};

/// Display Behavior: How Elements Appear and Flow
/// 
/// Like different clothing styles that determine how a garment looks and fits
#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    /// Inline elements: Flow within text, like a small accessory
    Inline,
    /// Block elements: Take full width, like a full-length coat
    Block,
    /// Hidden elements: Completely invisible, like a garment in a closed wardrobe
    None,
}

/// Style Property Map: A Wardrobe of Design Choices
/// 
/// Stores CSS properties and their corresponding values
pub type PropertyMap = HashMap<String, Value>;

/// Styled Node: A Fashionably Dressed HTML Element
/// 
/// Like a mannequin with carefully chosen clothing and accessories
#[derive(Clone)]
pub struct StyledNode<'a> {
    /// Original HTML node
    pub node: &'a Node,
    
    /// Specific style properties applied to this node
    pub specified_values: PropertyMap,
    
    /// Styled child nodes
    pub children: Vec<StyledNode<'a>>,
}

// Custom Debug implementation to avoid complex nested printing
impl<'a> std::fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StyledNode")
            .field("node", &"Node")  // Simplified representation of node
            .field("specified_values", &self.specified_values)
            .field("children_count", &self.children.len())
            .finish()
    }
}

impl<'a> StyledNode<'a> {
    /// Retrieve a specific style property
    /// 
    /// Like checking a specific detail of an outfit
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    /// Determine how the element should be displayed
    /// 
    /// Like choosing the overall style of an outfit
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline
        }
    }

    /// Flexible property lookup with fallback options
    /// 
    /// Like having multiple outfit choices if the first isn't available
    pub fn lookup(&self, primary: &str, fallback: &str, default: &Value) -> Value {
        self.value(primary)
            .or_else(|| self.value(fallback))
            .unwrap_or_else(|| default.clone())
    }
}

impl ElementData {
    /// Retrieve the ID attribute of an element
    /// 
    /// Like finding a unique identifier tag on a piece of clothing
    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    /// Retrieve the CSS classes of an element
    /// 
    /// Like checking all the style tags on a garment
    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

/// Selector Matching: Finding the Right Style
/// 
/// Like determining if a specific outfit matches a person's style

/// Check if a selector matches an HTML element
/// 
/// Like trying an outfit on a mannequin to see if it fits
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(s) => matches_simple_selector(elem, s)
    }
}

/// Check if a simple selector matches an element's characteristics
/// 
/// Like checking if an outfit matches specific criteria
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check tag name (like checking the type of garment)
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID (like checking a unique identifier)
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check classes (like checking style tags)
    if selector.class.iter().any(|class| !elem.classes().contains(class.as_str())) {
        return false;
    }

    true
}

/// Matched Rule: A Styled Outfit with Its Complexity
/// 
/// Represents a CSS rule that matches an element, along with its specificity
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find all CSS rules that match an element
/// 
/// Like searching through a wardrobe to find matching outfits
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// Match a single rule to an element
/// 
/// Like trying on a single outfit to see if it fits
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter()
        .find(|selector| matches(elem, selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Compute Specified Style Values
/// 
/// Like assembling the perfect outfit from multiple style sources
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Sort rules by specificity (most specific last)
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    // Apply declarations from matched rules
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

/// Build Style Tree: Transforming Raw HTML into Styled Elements
/// 
/// Like turning a basic mannequin into a fashion model
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

// Test Module: Fashion Quality Control
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Test selector matching logic
    /// 
    /// Like checking if outfits match different style criteria
    #[test]
    fn test_matches_simple_selector() {
        // Create a test element with classes and ID
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "test-class".to_string());
        attrs.insert("id".to_string(), "test-id".to_string());

        let elem = ElementData {
            tag_name: "div".to_string(),
            attrs,
        };

        // Test various selector scenarios
        // Like trying different outfit matching rules

        // Tag selector test
        let tag_selector = Selector::Simple(SimpleSelector {
            tag_name: Some("div".to_string()),
            id: None,
            class: vec![],
        });
        assert!(matches(&elem, &tag_selector));

        // Class selector test
        let class_selector = Selector::Simple(SimpleSelector {
            tag_name: None,
            id: None,
            class: vec!["test-class".to_string()],
        });
        assert!(matches(&elem, &class_selector));

        // ID selector test
        let id_selector = Selector::Simple(SimpleSelector {
            tag_name: None,
            id: Some("test-id".to_string()),
            class: vec![],
        });
        assert!(matches(&elem, &id_selector));

        // Non-matching selector test
        let non_match_selector = Selector::Simple(SimpleSelector {
            tag_name: Some("span".to_string()),
            id: None,
            class: vec![],
        });
        assert!(!matches(&elem, &non_match_selector));
    }
}
