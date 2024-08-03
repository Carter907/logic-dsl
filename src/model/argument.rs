use crate::model::conclusion::Conclusion;
use crate::model::premise::Premise;

#[derive(Debug)]
pub struct Argument {
    pub(crate) vars: Vec<char>,
    pub(crate) premises: Vec<Premise>,
    pub(crate) conclusion: Conclusion,
}

impl Argument {
    pub fn is_valid_argument(&self) -> bool {
        let first_premise_str = &self.premises[0];


        false
    }
    pub fn new() -> Argument {

        Argument {
            vars: vec![],
            premises: vec![],
            conclusion: Conclusion::new(),
        }
    }
}

#[cfg(test)]
mod logic_test {

    #[test]
    fn test_argument() {

    }
}
