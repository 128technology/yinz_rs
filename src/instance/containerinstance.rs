use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::ListInstance;
use super::util::*;
use crate::model::container::Container;
use crate::model::util::{Model, WithChildren};

pub struct ContainerData<'a> {
    pub parent: Option<Parent<'a>>,
    pub model: &'a Container,
    pub children: Option<Arc<RwLock<HashMap<String, Child<'a>>>>>,
    pub path: String,
}

type Link<'a> = Arc<RwLock<ContainerData<'a>>>;

pub struct ContainerInstance<'a>(Link<'a>);

impl<'a> Clone for ContainerInstance<'a> {
    fn clone(&self) -> Self {
        ContainerInstance(Arc::clone(&self.0))
    }
}

impl<'a> PartialEq for ContainerInstance<'a> {
    fn eq(&self, other: &ContainerInstance<'a>) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children<'a>(
    model: &'a Container,
    value: &Value,
    parent_path: String,
    parent: &Link<'a>,
) -> HashMap<String, Child<'a>> {
    let mut children: HashMap<String, Child> = HashMap::new();
    let child_path = parent_path.clone();

    match value {
        Value::Object(x) => {
            for (k, v) in x.iter() {
                let child_model = model.get_child(k).unwrap();
                let children_parent = Parent::ContainerData(Arc::downgrade(parent));

                match child_model {
                    Model::Leaf(m) => {
                        children.insert(
                            k.to_string(),
                            Child::LeafInstance(LeafInstance::new(
                                m,
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
                                m,
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
                                m,
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
                                m,
                                &v,
                                child_path.clone(),
                                children_parent,
                            )),
                        );
                    }
                }
            }
        }
        _ => {}
    }

    return children;
}

impl<'a> ContainerInstance<'a> {
    pub fn new(
        model: &'a Container,
        value: &Value,
        parent_path: String,
        parent: Option<Parent<'a>>,
    ) -> ContainerInstance<'a> {
        let path = format!("{}/{}", parent_path, model.name);
        let child_path = path.clone();

        let instance = ContainerInstance(Arc::new(RwLock::new(ContainerData {
            model,
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

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
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
