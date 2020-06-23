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
