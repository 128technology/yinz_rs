use std::sync::Arc;
use sxd_document::*;

use super::container::Container;

#[derive(Debug)]
pub struct DataModel {
    pub root: Arc<Container>,
}

impl DataModel {
    pub fn new(root_el: dom::Element) -> DataModel {
        DataModel {
            root: Arc::new(Container::new(root_el)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::util::*;

    const DATA_MODEL: &str = r#"<?xml version="1.0"?>
    <yin:container name="root" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
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
    fn it_parses() {
        let pkg = get_package(DATA_MODEL);
        let model = super::DataModel::new(get_root_el(&pkg));
        assert_eq!(model.root.name, "root");
    }
}
