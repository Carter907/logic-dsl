use std::str::FromStr;
use crate::model::argument::Argument;

impl FromStr for Argument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        Ok(
            Argument::new()
        )
    }
}

