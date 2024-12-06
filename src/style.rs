use std::collections::{HashMap, HashSet};
use crate::dom::{Node, NodeType, ElementData};
use crate::css::{Stylesheet, Rule, Selector, SimpleSelector, Specificity, Value};

/// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

/// A node with associated style data.
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

impl ElementData {
    /// Get the ID attribute of an element
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    /// Get the classes of an element
    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

/// Check if a selector matches an element
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(s) => matches_simple_selector(elem, s)
    }
}

/// Check if a simple selector matches an element
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    if selector.class.iter().any(|class| !elem.classes().contains(class.as_str())) {
        return false;
    }

    true
}

/// Represents a matched rule with its specificity
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find matching rules for an element
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// Match a single rule to an element
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter()
        .find(|selector| matches(elem, selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Compute specified values for an element
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Sort rules by specificity (lowest to highest)
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    // Apply declarations from matched rules
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

/// Build a style tree from a DOM tree and stylesheet
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{Node, NodeType, ElementData};
    use crate::css::{Stylesheet, Rule, Selector, SimpleSelector, Value};
    use std::collections::HashMap;

    #[test]
    fn test_matches_simple_selector() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "test-class".to_string());
        attrs.insert("id".to_string(), "test-id".to_string());

        let elem = ElementData {
            tag_name: "div".to_string(),
            attributes: attrs,
        };

        // Test tag selector
        let tag_selector = Selector::Simple(SimpleSelector {
            tag_name: Some("div".to_string()),
            id: None,
            class: vec![],
        });
        assert!(matches(&elem, &tag_selector));

        // Test class selector
        let class_selector = Selector::Simple(SimpleSelector {
            tag_name: None,
            id: None,
            class: vec!["test-class".to_string()],
        });
        assert!(matches(&elem, &class_selector));

        // Test ID selector
        let id_selector = Selector::Simple(SimpleSelector {
            tag_name: None,
            id: Some("test-id".to_string()),
            class: vec![],
        });
        assert!(matches(&elem, &id_selector));

        // Test non-matching selector
        let non_match_selector = Selector::Simple(SimpleSelector {
            tag_name: Some("span".to_string()),
            id: None,
            class: vec![],
        });
        assert!(!matches(&elem, &non_match_selector));
    }
}
