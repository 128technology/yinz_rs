use inflector::cases::camelcase::to_camel_case;
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
    let key_split = key_attr_value.split(' ');

    key_split.map(|s| to_camel_case(s)).collect()
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

impl WithChildren for List {
    fn get_children(&self) -> &HashMap<String, Model> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const MODEL: &str = r#"<?xml version="1.0"?>
    <yin:list name="peer" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:key value="name"/>
        <yin:leaf name="name">
            <yin:type name="string"/>
        </yin:leaf>
    </yin:list>"#;

    #[test]
    fn it_parses_name() {
        let pkg = get_package(MODEL);
        let model = super::List::new(get_root_el(&pkg));
        assert_eq!(model.name, "peer");
    }

    #[test]
    fn it_parses_keys() {
        let pkg = get_package(MODEL);
        let model = super::List::new(get_root_el(&pkg));
        assert_eq!(model.keys, vec!["name"]);
    }

    #[test]
    fn it_parses_children() {
        let pkg = get_package(MODEL);
        let model = super::List::new(get_root_el(&pkg));
        assert_eq!(model.children.len(), 1);
    }
}
