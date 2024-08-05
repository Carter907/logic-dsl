use std::collections::HashMap;
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
        let mut table: HashMap<&char, bool> = HashMap::new();

        // initial values
        for char in &self.vars {
            table.insert(char, true);
        }

        // create a loop in which the table


        println!("{:?}", table);

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
    use crate::model::argument::Argument;

    #[test]
    fn test_argument() {

        let argument = "let P, Q
        P -> Q
        P
        --
        Q".parse::<Argument>().unwrap();
        let validity = argument.is_valid_argument();

        println!("{:?}", validity)
    }
}
