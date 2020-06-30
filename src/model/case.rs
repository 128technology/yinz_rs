use std::collections::HashMap;
use std::sync::Arc;
use sxd_document::*;

use super::util::*;

#[derive(Debug)]
pub struct Case {
    pub name: String,
    pub children: HashMap<String, Model>,
}

fn parse_children_from_case(el: dom::Element) -> HashMap<String, Model> {
    parse_children(el)
}

fn parse_children_from_implicit_case(el: dom::Element) -> HashMap<String, Model> {
    let child_option = parse_child(el);
    let mut children: HashMap<String, Model> = HashMap::new();

    if let Some(c) = child_option {
        match c {
            Child::Choice(c) => {
                children = c.children;
            }
            Child::Leaf(c) => {
                children.insert(c.name.clone(), Model::Leaf(Arc::new(c)));
            }
            Child::LeafList(c) => {
                children.insert(c.name.clone(), Model::LeafList(Arc::new(c)));
            }
            Child::Container(c) => {
                children.insert(c.name.clone(), Model::Container(Arc::new(c)));
            }
            Child::List(c) => {
                children.insert(c.name.clone(), Model::List(Arc::new(c)));
            }
        }
    }

    children
}

impl Case {
    pub fn new(el: dom::Element) -> Case {
        let name = get_name(el);

        Case {
            name,
            children: if el.name().local_part() == "case" {
                parse_children_from_case(el)
            } else {
                parse_children_from_implicit_case(el)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const EXPLICT_MODEL: &str = r#"<?xml version="1.0"?>
    <yin:case name="explicit" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:leaf name="bar">
            <yin:type name="string"/>
        </yin:leaf>
    </yin:case>"#;

    const IMPLICIT_MODEL: &str = r#"<?xml version="1.0"?>
    <yin:leaf name="bar" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:type name="string"/>
    </yin:leaf>"#;

    #[test]
    fn it_parses_name_explicit() {
        let pkg = get_package(EXPLICT_MODEL);
        let model = super::Case::new(get_root_el(&pkg));
        assert_eq!(model.name, "explicit");
    }

    #[test]
    fn it_parses_name_implicit() {
        let pkg = get_package(IMPLICIT_MODEL);
        let model = super::Case::new(get_root_el(&pkg));
        assert_eq!(model.name, "bar");
    }

    #[test]
    fn it_parses_children_explicit() {
        let pkg = get_package(EXPLICT_MODEL);
        let model = super::Case::new(get_root_el(&pkg));
        assert!(model.children.get("bar").is_some());
    }

    #[test]
    fn it_parses_children_implicit() {
        let pkg = get_package(IMPLICIT_MODEL);
        let model = super::Case::new(get_root_el(&pkg));
        assert!(model.children.get("bar").is_some());
    }
}
