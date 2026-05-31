use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Func {
    Log,
    Sin,
    Cos,
    Tan,
    Sec,
    Cosec,
    Cotan,
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Func::Log => write!(f, "log"),
            Func::Sin => write!(f, "sin"),
            Func::Cos => write!(f, "cos"),
            Func::Tan => write!(f, "tan"),
            Func::Sec => write!(f, "sec"),
            Func::Cosec => write!(f, "cosec"),
            Func::Cotan => write!(f, "cot"),

        }
    }
}

impl FromStr for Func {
    // we really don't need any errors here
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
            "tan" => Ok(Func::Tan),
            "log" => Ok(Func::Log),
            "sec" => Ok(Func::Sec),
            "cosec" => Ok(Func::Cosec),
            "cotan" => Ok(Func::Cotan),
            _ => Err(()),
        }
    }
}
