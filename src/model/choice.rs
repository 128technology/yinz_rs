use std::collections::HashMap;
use sxd_document::*;

use super::case::Case;
use super::util::*;

#[derive(Debug)]
pub struct Choice {
    pub name: String,
    pub children: HashMap<String, Model>,
    cases: Vec<Case>,
}

fn parse_cases(el: dom::Element) -> Vec<Case> {
    let mut cases: Vec<Case> = Vec::new();

    for child in el.children() {
        if let dom::ChildOfElement::Element(e) = child {
            let model_type = e.name().local_part();

            match model_type {
                "case" | "leaf" | "container" | "list" | "leaf-list" => {
                    let case = Case::new(e);
                    cases.push(case);
                }
                _ => (),
            }
        }
    }

    cases
}

impl Choice {
    pub fn new(el: dom::Element) -> Choice {
        let cases = parse_cases(el);
        let mut children = HashMap::new();

        for case in &cases {
            children.extend(case.children.clone());
        }

        Choice {
            name: get_name(el),
            cases,
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const MODEL: &str = r#"<?xml version="1.0"?>
    <yin:choice name="foo" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:leaf name="implicit">
            <yin:type name="string"/>
        </yin:leaf>
        <yin:case name="explicit">
            <yin:leaf name="bar">
                <yin:type name="string"/>
            </yin:leaf>>
        </yin:case>
    </yin:choice>"#;

    #[test]
    fn it_parses_name() {
        let pkg = get_package(MODEL);
        let model = super::Choice::new(get_root_el(&pkg));
        assert_eq!(model.name, "foo");
    }

    #[test]
    fn it_parses_implicit_child() {
        let pkg = get_package(MODEL);
        let model = super::Choice::new(get_root_el(&pkg));
        assert!(model.children.get("implicit").is_some());
    }

    #[test]
    fn it_parses_explicit_child() {
        let pkg = get_package(MODEL);
        let model = super::Choice::new(get_root_el(&pkg));
        assert!(model.children.get("bar").is_some());
    }

    #[test]
    fn it_parses_cases() {
        let pkg = get_package(MODEL);
        let model = super::Choice::new(get_root_el(&pkg));
        assert_eq!(model.cases.len(), 2);
    }
}
