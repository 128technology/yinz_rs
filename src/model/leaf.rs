use sxd_document::*;

use super::util::*;

#[derive(Debug, Clone)]
pub struct Leaf {
    pub name: String,
}

impl Leaf {
    pub fn new(el: dom::Element) -> Leaf {
        Leaf { name: get_name(el) }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const MODEL: &str = r#"<?xml version="1.0"?>
    <yin:leaf name="foo" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:type name="string"/>
    </yin:leaf>"#;

    #[test]
    fn it_parses_name() {
        let pkg = get_package(MODEL);
        let model = super::Leaf::new(get_root_el(&pkg));
        assert_eq!(model.name, "foo");
    }
}
