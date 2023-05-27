use lazy_static::lazy_static;
use std::path::Path;
use tera::Tera;

lazy_static! {
    pub static ref CONF_TEMPLATES: Tera = {
        let mut tera = Tera::new("../examples/conf/*").unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

pub fn populate_placeholders(path: &Path) {
    todo!()
}
