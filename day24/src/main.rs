use day24_macro::AluProgram;
use std::collections::BTreeMap;

pub mod dynamic;

#[derive(AluProgram)]
#[alu_program("day24/src/input.txt")]
pub struct CompiledScript;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("part one: {:?}", parts(true)?);
    println!("part two: {:?}", parts(false)?);

    Ok(())
}

// keep the lowest N results (which speeds up the calculation a lot)
const N: usize = 10000;

fn parts(forward: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let mut set = BTreeMap::new();
    for j in 1..10 {
        set.insert(CompiledScript::calculate_n(0, j, 0), vec![j]);
    }

    let till = 14;
    let mut found = false;
    let mut code: Option<usize> = None;
    for i in 1..till {
        let mut new_set = BTreeMap::new();
        for j in range(forward) {
            for (k, v) in set.iter() {
                let next = CompiledScript::calculate_n(i, j, *k);
                let mut path = v.clone();
                path.push(j);
                new_set.insert(next, path);
            }
        }
        set = new_set.into_iter().take(N).collect();
        if i == till - 1 {
            if set.keys().next() == Some(&0) {
                found = true
            }
        }

        if found {
            let monad: Vec<_> = set
                .values()
                .next()
                .expect("this exists")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            code = Some(monad.join("").parse()?)
        }
    }

    if let Some(code) = code {
        Ok(code)
    } else {
        Ok(0)
    }
}

fn range(forward: bool) -> Box<dyn Iterator<Item = i64>> {
    if forward {
        Box::new(1..10)
    } else {
        Box::new((1..10).rev())
    }
}

// 11711691612189
// 11934998949189
// 12934998949199

#[test]
fn day24_part_one() {
    assert_eq!(12934998949199, parts(true).unwrap());
}

#[test]
fn day24_part_two() {
    assert_eq!(11711691612189, parts(false).unwrap());
}
