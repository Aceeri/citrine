
pub struct Section {
    pub id: Option<String>, 
    pub classes: Vec<Class>,
    pub child: Node<Section>,
}

impl Default for Section {
    fn default() -> Self {
        Section {
            id: None,
            classes: Vec::new(),
            children: None,
        }
    }
}

