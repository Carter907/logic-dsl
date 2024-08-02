pub struct Conclusion {
    content: String,
}

impl Conclusion {
    pub fn new() -> Conclusion {
        Conclusion {
            content: String::new(),
        }
    }
    pub fn from(str: &str) -> Conclusion {
        Conclusion {
            content: str.to_string(),
        }
    }

    pub fn bool_value(&self, atomics: &Vec<(char, bool)>) -> bool {
        false
    }
}
