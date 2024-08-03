use crate::conclusion::Conclusion;
use crate::premise::Premise;

#[derive(Debug)]
pub struct Argument {
    pub(crate) vars: Vec<char>,
    pub(crate) premises: Vec<Premise>,
    pub(crate) conclusion: Conclusion,
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
