
use class::Class;
use section::Section;

pub struct Page {
    classes: HashMap<String, Class>,
    body: Option<Section>,
}

impl Page {
    pub fn add_class(&mut self, class: Class) {
        match class.id {
            Some(id) => self.classes.insert(id.clone(), class),
            None => println!("Class had no id"),
        }
    }

    pub fn set_body<S: Into<Option<Section>>(&mut self, body: S) {
        self.body = body.into();
    }
}

