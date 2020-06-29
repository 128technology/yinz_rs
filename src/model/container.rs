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

impl WithChildren for Container {
    fn get_children(&self) -> &HashMap<String, Model> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const MODEL: &str = r#"<?xml version="1.0"?>
    <yin:container name="bfd" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:leaf name="state">
            <yin:type name="enumeration">
                <yin:enum name="enabled"/>
                <yin:enum name="disabled"/>
            </yin:type>
            <yin:default value="enabled"/>
        </yin:leaf>
        <yin:leaf name="desired-tx-interval">
            <yin:type name="uint32"/>
        </yin:leaf>
    </yin:container>"#;

    #[test]
    fn it_parses_name() {
        let pkg = get_package(MODEL);
        let model = super::Container::new(get_root_el(&pkg));
        assert_eq!(model.name, "bfd");
    }

    #[test]
    fn it_parses_children() {
        let pkg = get_package(MODEL);
        let model = super::Container::new(get_root_el(&pkg));
        assert_eq!(model.children.len(), 2);
    }
}
