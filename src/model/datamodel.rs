use sxd_document::*;

use super::container::Container;

#[derive(Debug)]
pub struct DataModel {
    pub root: Container,
}

impl DataModel {
    pub fn new(root_el: dom::Element) -> DataModel {
        DataModel {
            root: Container::new(root_el),
        }
    }
}
