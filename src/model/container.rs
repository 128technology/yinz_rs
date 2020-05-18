use std::collections::HashMap;
use sxd_document::*;

use super::util::*;

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub children: HashMap<String, Model>,
}

impl Container {
    pub fn new(el: dom::Element) -> Container {
        Container {
            name: get_name(el),
            children: parse_children(el),
        }
    }
}
