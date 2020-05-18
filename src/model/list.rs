use std::collections::HashMap;
use sxd_document::*;

use super::util::*;

#[derive(Debug, Clone)]
pub struct List {
    pub name: String,
    pub children: HashMap<String, Model>,
    pub keys: Vec<String>,
}

fn parse_keys(el: dom::Element) -> Vec<String> {
    let key_el = evaluate_get_yin_xpath("./yin:key", &el).expect("A list must have keys defined.");
    let key_attr_value = key_el.attribute("value").unwrap().value().to_string();
    let key_split = key_attr_value.split(" ");

    key_split.map(|s| s.to_string()).collect()
}

impl List {
    pub fn new(el: dom::Element) -> List {
        List {
            name: get_name(el),
            children: parse_children(el),
            keys: parse_keys(el),
        }
    }
}
