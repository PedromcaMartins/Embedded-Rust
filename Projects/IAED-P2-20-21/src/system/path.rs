#[derive(Debug)]
pub struct Component {
    path: Vec<String>, 
    value: String,
}

impl Component {
    pub fn new(path: Vec<String>, value: String) -> Self {
        Self {
            path,
            value,
        }
    }

    pub fn get_path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn eq_path(&self, other: &[String]) -> bool {
        self.path.eq(other)
    }

    pub fn ne_path(&self, other: &[String]) -> bool {
        self.path.ne(other)
    }

    pub fn eq_value(&self, other: &String) -> bool {
        self.value.eq(other)
    }

    pub fn has_value(&self) -> bool {
        !self.value.is_empty()
    }

    pub fn has_no_value(&self) -> bool {
        self.value.is_empty()
    }

    pub fn is_component_of_directory(&self, path: &[String]) -> bool {
        self.path[..self.path.len() - 1].eq(path)
    }

    pub fn is_sub_component_of_directory(&self, path: &[String]) -> bool {
        self.path.len() >= path.len() && self.path[..path.len()].eq(path)
    }

    pub fn is_root_component(&self) -> bool {
        self.path.len() == 1
    }
}