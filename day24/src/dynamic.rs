use std::collections::BTreeMap;
use std::io::BufRead;
use day24_shared::{Instruction, Item, Variable};

#[derive(Debug, Default)]
pub struct Context {
    vars: BTreeMap<Variable, i64>,
    inputs: Vec<i8>,
}

impl Context {
    pub fn reset(&mut self) {
        *self = Context::default();
    }

    pub fn set_input(&mut self, input: usize) {
        self.inputs = input
            .to_string()
            .chars()
            .rev()
            .map(|ch| ch.to_digit(10).expect("digits are always valid") as i8)
            .collect();
    }

    pub fn set_input_string(&mut self, input: &str) {
        self.inputs = input
            .chars()
            .rev()
            .map(|ch| ch.to_digit(10).expect("digits are always valid") as i8)
            .collect();
    }

    pub fn apply(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Input(Item::Variable(var)) => {
                let value = self.vars.entry(var).or_insert(0);
                *value = self.inputs.pop().expect("input stack is empty") as i64;
            }
            Add(Item::Variable(var), y) => {
                let y = y.resolve(&self.vars);
                let value = self.vars.entry(var).or_insert(0);
                *value += y as i64;
            }
            Multiply(Item::Variable(var), y) => {
                let y = y.resolve(&self.vars);
                let value = self.vars.entry(var).or_insert(0);
                *value *= y as i64;
            }
            Divide(Item::Variable(var), y) => {
                let y = y.resolve(&self.vars);
                let value = self.vars.entry(var).or_insert(0);
                *value /= y as i64;
            }
            Modulo(Item::Variable(var), y) => {
                let y = y.resolve(&self.vars);
                let value = self.vars.entry(var).or_insert(0);
                *value %= y as i64;
            }
            Equal(Item::Variable(var), y) => {
                let y = y.resolve(&self.vars);
                let value = self.vars.entry(var).or_insert(0);
                if *value == y {
                    *value = 1
                } else {
                    *value = 0
                }
            }
            _ => panic!("invalid instruction"),
        }
    }

    pub fn apply_script<R: BufRead>(
        &mut self,
        reader: R,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for line in reader.lines() {
            let instruction: Instruction = line?.parse()?;
            self.apply(instruction);
        }

        Ok(())
    }

    pub fn apply_instructions(
        &mut self,
        instruction: &[Instruction],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for instruction in instruction {
            self.apply(instruction.clone());
        }

        Ok(())
    }
}


#[test]
fn binary_example_9() {
    let reader = std::io::Cursor::new(
        "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
    );
    let mut ctx = Context::default();
    ctx.set_input(9);

    ctx.apply_script(reader).unwrap();

    assert_eq!(&0, ctx.vars.get(&Variable::X).unwrap());
    assert_eq!(&0, ctx.vars.get(&Variable::Y).unwrap());
    assert_eq!(&1, ctx.vars.get(&Variable::Z).unwrap());
    assert_eq!(&1, ctx.vars.get(&Variable::W).unwrap());
}

#[test]
fn binary_example_4() {
    let reader = std::io::Cursor::new(
        "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
    );
    let mut ctx = Context::default();
    ctx.set_input(4);

    ctx.apply_script(reader).unwrap();

    assert_eq!(&1, ctx.vars.get(&Variable::X).unwrap());
    assert_eq!(&0, ctx.vars.get(&Variable::Y).unwrap());
    assert_eq!(&0, ctx.vars.get(&Variable::Z).unwrap());
    assert_eq!(&0, ctx.vars.get(&Variable::W).unwrap());
}

#[test]
fn calculate_test() {
    assert_eq!(
        4475999984,
        crate::CompiledScript::calculate(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5])
    )
}
