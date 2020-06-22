use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::ListInstance;
use super::util::*;
use crate::model::container::Container;
use crate::model::util::{Model, WithChildren};

pub struct ContainerData {
    pub parent: Option<Parent>,
    pub model: Arc<Container>,
    pub children: Option<Arc<RwLock<HashMap<String, Child>>>>,
    pub path: String,
}

type Link = Arc<RwLock<ContainerData>>;

pub struct ContainerInstance(Link);

impl Clone for ContainerInstance {
    fn clone(&self) -> Self {
        ContainerInstance(Arc::clone(&self.0))
    }
}

impl PartialEq for ContainerInstance {
    fn eq(&self, other: &ContainerInstance) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children(
    model: Arc<Container>,
    value: &Value,
    parent_path: String,
    parent: &Link,
) -> HashMap<String, Child> {
    let mut children: HashMap<String, Child> = HashMap::new();
    let child_path = parent_path;

    if let Value::Object(x) = value {
        for (k, v) in x.iter() {
            let child_model = model.get_child(k).unwrap();
            let children_parent = Parent::ContainerData(Arc::downgrade(parent));

            match child_model {
                Model::Leaf(m) => {
                    children.insert(
                        k.to_string(),
                        Child::LeafInstance(LeafInstance::new(
                            m.clone(),
                            &v,
                            child_path.clone(),
                            children_parent,
                        )),
                    );
                }
                Model::Container(m) => {
                    children.insert(
                        k.to_string(),
                        Child::ContainerInstance(ContainerInstance::new(
                            m.clone(),
                            &v,
                            child_path.clone(),
                            Some(children_parent),
                        )),
                    );
                }
                Model::LeafList(m) => {
                    children.insert(
                        k.to_string(),
                        Child::LeafListInstance(LeafListInstance::new(
                            m.clone(),
                            &v,
                            child_path.clone(),
                            children_parent,
                        )),
                    );
                }
                Model::List(m) => {
                    children.insert(
                        k.to_string(),
                        Child::ListInstance(ListInstance::new(
                            m.clone(),
                            &v,
                            child_path.clone(),
                            children_parent,
                        )),
                    );
                }
            }
        }
    }

    children
}

impl ContainerInstance {
    pub fn new(
        model: Arc<Container>,
        value: &Value,
        parent_path: String,
        parent: Option<Parent>,
    ) -> ContainerInstance {
        let path = format!("{}/{}", parent_path, model.name);
        let child_path = path.clone();

        let instance = ContainerInstance(Arc::new(RwLock::new(ContainerData {
            model: model.clone(),
            children: None,
            path,
            parent,
        })));

        instance.0.write().unwrap().children = Some(Arc::new(RwLock::new(parse_children(
            model,
            value,
            child_path,
            &instance.0,
        ))));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        for child in self
            .0
            .read()
            .unwrap()
            .children
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .values()
        {
            match child {
                Child::ContainerInstance(c) => {
                    c.visit(f);
                }
                Child::ListInstance(c) => {
                    c.visit(f);
                }
                Child::LeafInstance(c) => {
                    c.visit(f);
                }
                Child::LeafListInstance(c) => {
                    c.visit(f);
                }
            }
        }
    }
}
