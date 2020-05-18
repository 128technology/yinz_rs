use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use super::containerinstance::ContainerInstance;
use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::{ListData, ListInstance};
use super::util::*;
use crate::model::list::List;
use crate::model::util::Model;

pub struct ListChildData<'a> {
    pub parent: Weak<RefCell<ListData<'a>>>,
    pub model: &'a List,
    pub children: Option<Rc<RefCell<HashMap<String, Child<'a>>>>>,
    pub path: String,
    pub key_value: String,
}

type Link<'a> = Rc<RefCell<ListChildData<'a>>>;

pub struct ListChildInstance<'a>(Link<'a>);

impl<'a> Clone for ListChildInstance<'a> {
    fn clone(&self) -> Self {
        ListChildInstance(Rc::clone(&self.0))
    }
}

impl<'a> PartialEq for ListChildInstance<'a> {
    fn eq(&self, other: &ListChildInstance<'a>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children<'a>(
    model: &'a List,
    value: &Value,
    parent_path: String,
    parent: &Link<'a>,
) -> HashMap<String, Child<'a>> {
    let mut children: HashMap<String, Child> = HashMap::new();
    // TODO: Remove these clones
    let child_path = parent_path.clone();

    match value {
        Value::Object(x) => {
            for (k, v) in x.iter() {
                let children_parent = Parent::ListChildData(Rc::downgrade(parent));
                let child_model = model.children.get(k).unwrap();

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

pub fn get_key_value<'a>(model: &'a List, value: &Value) -> String {
    let mut key_values: Vec<String> = Vec::new();

    for key in &model.keys {
        let key_value = &value[key];
        let key_value_string = match key_value {
            Value::String(x) => x,
            _ => panic!("Key value must be a string."),
        };

        key_values.push(key_value_string.to_string());
    }

    key_values.join(",")
}

impl<'a> ListChildInstance<'a> {
    pub fn new(
        model: &'a List,
        value: &Value,
        parent_path: String,
        parent: Weak<RefCell<ListData<'a>>>,
    ) -> ListChildInstance<'a> {
        let key_value = get_key_value(model, value);
        let path = format!("{}={}", parent_path, key_value);
        let child_path = path.clone();

        let instance = ListChildInstance(Rc::new(RefCell::new(ListChildData {
            model,
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

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
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

impl<'a> ListChildData<'a> {
    pub fn is_generated(&self) -> bool {
        for child in self.children.as_ref().unwrap().borrow().values() {
            match child {
                Child::LeafInstance(c) => {
                    if c.model.name == "generated" && c.value == "true" {
                        return true;
                    }
                }
                _ => (),
            }
        }

        false
    }
}
