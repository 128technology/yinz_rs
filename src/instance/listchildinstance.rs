use inflector::cases::kebabcase::to_kebab_case;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use super::containerinstance::ContainerInstance;
use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::{ListData, ListInstance};
use super::util::*;
use crate::model::list::List;
use crate::model::util::{Model, WithChildren};

pub struct ListChildData {
    pub parent: Weak<RefCell<ListData>>,
    pub model: Arc<List>,
    pub children: Option<Rc<RefCell<HashMap<String, Child>>>>,
    pub path: String,
    pub key_value: String,
}

type Link = Rc<RefCell<ListChildData>>;

pub struct ListChildInstance(Link);

impl Clone for ListChildInstance {
    fn clone(&self) -> Self {
        ListChildInstance(Rc::clone(&self.0))
    }
}

impl PartialEq for ListChildInstance {
    fn eq(&self, other: &ListChildInstance) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children(
    model: Arc<List>,
    value: &Value,
    parent_path: String,
    parent: &Link,
) -> HashMap<String, Child> {
    let mut children: HashMap<String, Child> = HashMap::new();
    let child_path = parent_path;

    if let Value::Object(x) = value {
        for (k, v) in x.iter() {
            let children_parent = Parent::ListChildData(Rc::downgrade(parent));
            let child_model = model.get_child(k).unwrap();

            match child_model {
                Model::Leaf(m) => {
                    children.insert(
                        k.to_string(),
                        Child::LeafInstance(LeafInstance::new(m.clone(), &v, children_parent)),
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

pub fn get_key_value(model: Arc<List>, value: &Value) -> String {
    let mut key_values: Vec<String> = Vec::new();

    for key in &model.keys {
        let key_value = match &value[key] {
            Value::Null => &value[to_kebab_case(key)],
            _ => &value[key],
        };
        let key_value_string = match key_value {
            Value::String(x) => x.clone(),
            Value::Number(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            _ => panic!("Key value must be a string."),
        };

        key_values.push(key_value_string);
    }

    key_values.join(",")
}

impl ListChildInstance {
    pub fn new(
        model: Arc<List>,
        value: &Value,
        parent_path: String,
        parent: Weak<RefCell<ListData>>,
    ) -> ListChildInstance {
        let key_value = get_key_value(model.clone(), value);
        let path = format!("{}={}", parent_path, key_value);
        let child_path = path.clone();

        let instance = ListChildInstance(Rc::new(RefCell::new(ListChildData {
            model: model.clone(),
            children: None,
            path,
            parent,
            key_value,
        })));

        instance.0.borrow_mut().children = Some(Rc::new(RefCell::new(parse_children(
            model,
            value,
            child_path,
            &instance.0,
        ))));

        instance
    }

    pub fn get_key(&self) -> String {
        self.0.borrow().key_value.to_string()
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        for child in self.0.borrow().children.as_ref().unwrap().borrow().values() {
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

impl ListChildData {
    pub fn is_generated(&self) -> bool {
        for child in self.children.as_ref().unwrap().borrow().values() {
            if let Child::LeafInstance(c) = child {
                if c.model.name == "generated" && c.value == "true" {
                    return true;
                }
            }
        }

        false
    }
}
