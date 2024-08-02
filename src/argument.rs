use crate::conclusion::Conclusion;
use crate::premise::Premise;

pub struct Argument {
    vars: Vec<char>,
    premises: Vec<Premise>,
    conclusion: Conclusion,
}

impl Argument {
    pub fn is_valid_argument(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod logic_test {
    use crate::argument::Argument;

    #[test]
    fn test_argument() {
        let source = "let P, Q
P -> Q
P
--
Q";

    }
}
