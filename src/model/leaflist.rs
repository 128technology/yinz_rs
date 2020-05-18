use sxd_document::*;

use super::util::*;

#[derive(Debug, Clone)]
pub struct LeafList {
    pub name: String,
}

impl LeafList {
    pub fn new(el: dom::Element) -> LeafList {
        LeafList { name: get_name(el) }
    }
}
