use super::element_type::ElementType;

pub struct ElementDescription {
    id: String,
    element_type: ElementType,
}

impl ElementDescription {
    pub fn new(id: &str, element_type: ElementType) -> Self {
        ElementDescription {
            id: id.to_owned(),
            element_type: element_type,
        }
    }

    pub fn parse(uri: &str) -> Option<Self> {
        let splits: Vec<&str> = uri.split("/").collect();
        if splits.len() < 2 {
            return None;
        }

        let id = splits[splits.len() - 1];
        let uri_type_string = splits[splits.len() - 2];

        if let Some(element_t) = parse_uri_type(uri_type_string) {
            Some(Self::new(id, element_t))
        } else {
            None
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_owned()
    }

    pub fn get_type(&self) -> ElementType {
        self.element_type
    }
}

fn parse_uri_type(t: &str) -> Option<ElementType> {
    unimplemented!()
}
