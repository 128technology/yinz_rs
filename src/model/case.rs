use std::collections::HashMap;
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
                children.insert(c.name.clone(), Model::Leaf(c));
            }
            Child::LeafList(c) => {
                children.insert(c.name.clone(), Model::LeafList(c));
            }
            Child::Container(c) => {
                children.insert(c.name.clone(), Model::Container(c));
            }
            Child::List(c) => {
                children.insert(c.name.clone(), Model::List(c));
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
