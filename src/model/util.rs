use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use sxd_document::*;
use sxd_xpath::nodeset::Node;
use sxd_xpath::{Context, Factory, Value};

use super::choice::Choice;
use super::container::Container;
use super::leaf::Leaf;
use super::leaflist::LeafList;
use super::list::List;

const YIN_NS: &str = "urn:ietf:params:xml:ns:yang:yin:1";

#[derive(Debug, Clone)]
pub enum Model {
    Leaf(Leaf),
    List(List),
    Container(Container),
    LeafList(LeafList),
}

#[derive(Debug)]
pub enum Child {
    Leaf(Leaf),
    List(List),
    Container(Container),
    LeafList(LeafList),
    Choice(Choice),
}

pub fn read_xml_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
struct NotFound;

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse data model XML.")
    }
}

impl Error for NotFound {
    fn description(&self) -> &str {
        "Could not parse data model XML."
    }

    fn cause(&self) -> Option<&(dyn Error)> {
        None
    }
}

pub fn evaluate_get_yin_xpath<'a>(
    xpath: &str,
    el: &'a dom::Element,
) -> Result<dom::Element<'a>, Box<dyn Error>> {
    let mut context = Context::new();
    context.set_namespace("yin", YIN_NS);

    let factory = Factory::new();
    let xpath = factory.build(xpath)?;
    let result = xpath.ok_or(NotFound)?.evaluate(&context, *el)?;

    let found = match result {
        Value::Nodeset(x) => match x.document_order_first().ok_or(NotFound)? {
            Node::Element(x) => x,
            _ => return Err(NotFound.into()),
        },
        _ => return Err(NotFound.into()),
    };

    Ok(found)
}

pub fn get_name(el: dom::Element) -> String {
    el.attribute("name").unwrap().value().to_string()
}

pub fn parse_children(el: dom::Element) -> HashMap<String, Model> {
    let mut children: HashMap<String, Model> = HashMap::new();

    for child in el.children() {
        match child {
            dom::ChildOfElement::Element(e) => {
                let child = parse_child(e);

                match child {
                    Some(c) => match c {
                        Child::Leaf(x) => {
                            children.insert(x.name.clone(), Model::Leaf(x));
                        }
                        Child::LeafList(x) => {
                            children.insert(x.name.clone(), Model::LeafList(x));
                        }
                        Child::Container(x) => {
                            children.insert(x.name.clone(), Model::Container(x));
                        }
                        Child::List(x) => {
                            children.insert(x.name.clone(), Model::List(x));
                        }
                        Child::Choice(x) => {
                            children.extend(x.children);
                        }
                    },
                    None => (),
                }
            }
            _ => (),
        }
    }

    return children;
}

pub fn parse_child(el: dom::Element) -> Option<Child> {
    let model_type = el.name().local_part();

    match model_type {
        "leaf" => {
            return Some(Child::Leaf(Leaf::new(el)));
        }
        "container" => {
            return Some(Child::Container(Container::new(el)));
        }
        "list" => {
            return Some(Child::List(List::new(el)));
        }
        "leaf-list" => {
            return Some(Child::LeafList(LeafList::new(el)));
        }
        "choice" => {
            return Some(Child::Choice(Choice::new(el)));
        }
        _ => {
            return None;
        }
    }
}
