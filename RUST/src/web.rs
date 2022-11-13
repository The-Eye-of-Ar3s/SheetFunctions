extern crate sxd_document;
extern crate sxd_xpath;

use sxd_document::parser;
use sxd_xpath::{evaluate_xpath};
use urlencoding::encode;

#[allow(non_snake_case, dead_code)]
pub fn ENCODEURL(text: String) -> Result<String, String> {
    return Ok(encode(&text).to_string().replace("~", "%7E"));
}
#[allow(non_snake_case, dead_code)]
pub fn FILTERXML(xml: String, xpath: String) -> Result<String, String> { //TODO: IF MULTIPLE RETURN VECTOR OF STRING IF SINGLE RETURN VECTOR OF SINGLE
    let package = match parser::parse(xml.as_str()) {
        Ok(d) => d,
        Err(_) => return Err("#VALUE!".to_owned())
    };

    let document = package.as_document();

    match evaluate_xpath(&document.clone(), &xpath) {
        Ok(r) => return Ok(r.string()),
        Err(_) => return Err("#VALUE!".to_owned())
    };
}

#[allow(non_snake_case, dead_code)]
pub fn WEBSERVICE(url: String) -> Result<String, String> {
    let resp = reqwest::blocking::get(url);
    let ret_t = match resp {
        Ok(r) => r.text(),
        Err(_) => return Err("#VALUE!".to_owned())
    };

    match ret_t {
        Ok(r) => return Ok(r),
        Err(_) => return Err("#VALUE!".to_owned())
    };
}