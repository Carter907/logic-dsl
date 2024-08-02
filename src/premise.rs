pub struct Premise {
    content: String,
}

impl Premise {
    pub fn bool_value(&self, atomics: &Vec<(char, bool)>) -> bool {
        false
    }
}
