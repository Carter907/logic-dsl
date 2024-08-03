#[derive(Debug)]
pub struct Premise {
    content: String,
}

impl Premise {
    pub fn new() -> Premise {
       Premise {
           content: String::new()
       }
    }
    pub fn from(content: String) -> Premise {
        Premise {
            content
        }
    }
    pub fn bool_value(&self, atomics: &Vec<(char, bool)>) -> bool {
        false
    }
}
