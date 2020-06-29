use std::error::Error;
use std::fmt;
use std::path::Path;
use sxd_document::parser;

use super::datamodel::DataModel;
use super::util::*;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse data model XML.")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Could not parse data model XML."
    }

    fn cause(&self) -> Option<&(dyn Error)> {
        None
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<DataModel, Box<dyn Error>> {
    let model_xml = read_xml_from_file(path).unwrap();
    let package = parser::parse(&model_xml)?;
    let root_el = get_root_el(&package);

    let root = evaluate_get_yin_xpath("//yin:container[@name=\"authority\"]", &root_el)?;

    Ok(DataModel::new(root))
}
