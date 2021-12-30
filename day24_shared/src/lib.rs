use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, strum::Display, strum::IntoStaticStr)]
pub enum Item {
    Value(i64),
    Variable(Variable),
}

impl Item {
    pub fn resolve(&self, variables: &BTreeMap<Variable, i64>) -> i64 {
        match self {
            Item::Variable(x) => variables.get(x).copied().unwrap_or(0),
            Item::Value(x) => *x,
        }
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = Variable::from_str(s) {
            Ok(Item::Variable(x))
        } else {
            Ok(Item::Value(s.parse::<i64>().map_err(|e| e.to_string())?))
        }
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Clone,
    strum::Display,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum Variable {
    X,
    Y,
    Z,
    W,
}

impl Variable {
    pub fn all() -> Vec<Variable> {
        use strum::IntoEnumIterator;
        Variable::iter().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, strum::Display, strum::IntoStaticStr)]
pub enum Instruction {
    #[strum(to_string = "inp")]
    Input(Item),
    #[strum(to_string = "add")]
    Add(Item, Item),
    #[strum(to_string = "mul")]
    Multiply(Item, Item),
    #[strum(to_string = "div")]
    Divide(Item, Item),
    #[strum(to_string = "mod")]
    Modulo(Item, Item),
    #[strum(to_string = "eql")]
    Equal(Item, Item),
}

impl Instruction {
    pub fn is_input(&self) -> bool {
        match self {
            Instruction::Input(_) => true,
            _ => false,
        }
    }

    pub fn variables<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = &'a Variable> + 'a> {
        use Instruction::*;
        match self {
            Input(Item::Variable(var)) => Box::new(std::iter::once(var)),
            Add(Item::Variable(var_a), Item::Variable(var_b))
            | Multiply(Item::Variable(var_a), Item::Variable(var_b))
            | Divide(Item::Variable(var_a), Item::Variable(var_b))
            | Modulo(Item::Variable(var_a), Item::Variable(var_b))
            | Equal(Item::Variable(var_a), Item::Variable(var_b)) => {
                Box::new([var_a, var_b].into_iter())
            }
            Add(Item::Variable(var), _)
            | Multiply(Item::Variable(var), _)
            | Divide(Item::Variable(var), _)
            | Modulo(Item::Variable(var), _)
            | Equal(Item::Variable(var), _) => Box::new(std::iter::once(var)),
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("inp") => {
                let first_arg = x
                    .trim_start_matches("inp")
                    .trim()
                    .split(' ')
                    .next()
                    .ok_or("invalid input")?;
                Ok(Instruction::Input(first_arg.parse()?))
            }
            x if x.starts_with("add") => {
                let mut args = x.trim_start_matches("add").trim().split(' ');
                let first_arg = args.next().ok_or("invalid input")?;
                let second_arg = args.next().ok_or("invalid input")?;
                Ok(Instruction::Add(first_arg.parse()?, second_arg.parse()?))
            }
            x if x.starts_with("mul") => {
                let mut args = x.trim_start_matches("mul").trim().split(' ');
                let first_arg = args.next().ok_or("invalid input")?;
                let second_arg = args.next().ok_or("invalid input")?;
                Ok(Instruction::Multiply(
                    first_arg.parse()?,
                    second_arg.parse()?,
                ))
            }
            x if x.starts_with("div") => {
                let mut args = x.trim_start_matches("div").trim().split(' ');
                let first_arg = args.next().ok_or("invalid input")?;
                let second_arg = args.next().ok_or("invalid input")?;
                Ok(Instruction::Divide(first_arg.parse()?, second_arg.parse()?))
            }
            x if x.starts_with("mod") => {
                let mut args = x.trim_start_matches("mod").trim().split(' ');
                let first_arg = args.next().ok_or("invalid input")?;
                let second_arg = args.next().ok_or("invalid input")?;
                Ok(Instruction::Modulo(first_arg.parse()?, second_arg.parse()?))
            }
            x if x.starts_with("eql") => {
                let mut args = x.trim_start_matches("eql").trim().split(' ');
                let first_arg = args.next().ok_or("invalid input")?;
                let second_arg = args.next().ok_or("invalid input")?;
                Ok(Instruction::Equal(first_arg.parse()?, second_arg.parse()?))
            }
            _ => Err("invalid instruction".into()),
        }
    }
}

#[test]
fn variable_parsing() {
    assert_eq!(Variable::W, "w".parse().unwrap())
}

#[test]
fn instruction_parsing() {
    assert_eq!(
        Instruction::Multiply(Item::Variable(Variable::Z), Item::Value(3)),
        "mul z 3".parse().unwrap()
    )
}

#[test]
fn variables_test() {
    let instruction = Instruction::Add(Item::Variable(Variable::X), Item::Variable(Variable::W));

    let vars: Vec<_> = instruction.variables().collect();
    let expected = vec![&Variable::X, &Variable::W];

    assert_eq!(vars, expected);

    let instruction = Instruction::Multiply(Item::Variable(Variable::X), Item::Value(1));

    let vars: Vec<_> = instruction.variables().collect();
    let expected = vec![&Variable::X];

    assert_eq!(vars, expected);

    let instruction = Instruction::Modulo(Item::Value(1), Item::Value(1));

    let vars: Vec<_> = instruction.variables().collect();
    let expected: Vec<&Variable> = Vec::new();

    assert_eq!(vars, expected)
}
