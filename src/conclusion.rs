#[derive(Debug)]
pub struct Conclusion {
    content: String,
}

impl Conclusion {
    pub fn new() -> Conclusion {
        Conclusion {
            content: String::new(),
        }
    }
    pub fn from(content: String) -> Conclusion {
        Conclusion {
            content,
        }
    }

    pub fn bool_value(&self, atomics: &Vec<(char, bool)>) -> bool {
        false
    }
}
