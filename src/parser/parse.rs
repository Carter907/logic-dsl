use std::str::FromStr;
use crate::model::argument::Argument;
use crate::model::conclusion::Conclusion;
use crate::model::premise::Premise;

impl FromStr for Argument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n")
            .map(|e| { e.trim() })
            .collect::<Vec<&str>>().into_iter();
        let conclusion: Conclusion;
        let mut premises: Vec<Premise> = vec![];
        let mut vars: Vec<char> = vec![];
        if let Some(declaration) = lines.next() {
            if &declaration[0..3] == "let" {
                let declaration = &declaration[3..];
                vars = declaration.split(",")
                    .map(|e| { e.trim().to_string().pop().unwrap() })
                    .map(|e| {
                        if !e.is_alphabetic() {
                            panic!("cannot have none alphabetic variables")
                        }
                        e
                    })
                    .collect::<Vec<char>>();
            } else {
                panic!("no let found")
            }

        } else {
            panic!("no declaration found")
        }
        while let Some(premise) = lines.next() {
            if premise.trim() == "--" {
                break;
            }
            premises.push(Premise::from(premise.to_string()))
        }

        conclusion = Conclusion::from(lines.next().unwrap().to_string());

        Ok(Argument {
            vars,
            premises,
            conclusion,
        })
    }
}

#[cfg(test)]
mod parsing_test {
    use crate::model::argument::Argument;

    #[test]
    fn test_parse() {
        let argument = "let P, Q
        P -> Q
        P
        --
        Q".parse::<Argument>().unwrap();

        println!("{:?}", argument)

    }
}
