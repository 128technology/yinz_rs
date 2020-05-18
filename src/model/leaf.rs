use sxd_document::*;

use super::util::*;

#[derive(Debug, Clone)]
pub struct Leaf {
    pub name: String,
}

impl Leaf {
    pub fn new(el: dom::Element) -> Leaf {
        Leaf { name: get_name(el) }
    }
}
