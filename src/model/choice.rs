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
        match child {
            dom::ChildOfElement::Element(e) => {
                let model_type = e.name().local_part();

                match model_type {
                    "case" | "leaf" | "container" | "list" | "leaf-list" => {
                        let case = Case::new(e);
                        cases.push(case);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    return cases;
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
