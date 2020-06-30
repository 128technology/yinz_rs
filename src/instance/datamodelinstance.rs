use serde_json::Value;
use std::sync::Arc;

use super::containerinstance::ContainerInstance;
use super::util::*;
use crate::model::datamodel::DataModel;

pub struct DataModelInstance {
    pub root: ContainerInstance,
}

impl DataModelInstance {
    pub fn new(model: Arc<DataModel>, value: &Value) -> DataModelInstance {
        let root_name = &model.root.name;
        let root =
            ContainerInstance::new(model.root.clone(), &value[root_name], "".to_string(), None);

        DataModelInstance { root }
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        self.root.visit(f);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::*;
    use std::cell::Cell;
    use std::sync::Arc;

    use crate::instance::util::*;
    use crate::model::datamodel::DataModel;
    use crate::model::util::*;

    const DATA_MODEL: &str = r#"<?xml version="1.0"?>
    <yin:container name="root" xmlns:yin="urn:ietf:params:xml:ns:yang:yin:1">
        <yin:leaf name="foo">
            <yin:type name="string"/>
        </yin:leaf>
        <yin:container name="a-container">
            <yin:leaf name="leaf-in-container">
                <yin:type name="string"/>
            </yin:leaf>
            <yin:leaf-list name="leaf-list-in-container">
                <yin:type name="string"/>
            </yin:leaf-list>
        </yin:container>
        <yin:list name="a-list">
            <yin:key value="leaf-in-list"/>
            <yin:leaf name="leaf-in-list">
                <yin:type name="string"/>
            </yin:leaf>
            <yin:leaf-list name="leaf-list-in-list">
                <yin:type name="string"/>
            </yin:leaf-list>
        </yin:list>
    </yin:container>"#;

    const INSTANCE: &str = r#"
    {
        "root": {
            "foo": "bar",
            "a-container": { "leaf-in-container": "fizz", "leaf-list-in-container": ["buzz"] },
            "a-list": [{ "leaf-in-list": "fizz", "leaf-list-in-list": ["buzz"] }]
        }
    }"#;

    #[test]
    fn it_visits_all_nodes() {
        let pkg = get_package(DATA_MODEL);
        let data_model = Arc::new(DataModel::new(get_root_el(&pkg)));
        let v: Value = from_str(INSTANCE).unwrap();
        let instance = super::DataModelInstance::new(data_model, &v);

        let count = Cell::new(0);
        let visitor = |node: NodeToVisit| match node {
            NodeToVisit::LeafInstance(_) | NodeToVisit::LeafListInstance(_) => {
                count.set(count.get() + 1);
            }
        };

        instance.visit(&visitor);

        assert_eq!(count.get(), 5);
    }
}
