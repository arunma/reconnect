use once_cell::sync::Lazy;
use std::path::Path;
use tera::Tera;

pub static CONF_TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::new("../examples/conf/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
});

pub fn populate_placeholders(_path: &Path) {
    todo!()
}
