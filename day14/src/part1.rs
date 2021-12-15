use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{parse_template_and_rules, Counter, Rules, Template};

fn find_next_template(template: Template, rules: &Rules) -> Template {
    let mut next_template = Vec::new();
    for pair in template.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if next_template.is_empty() {
            next_template.push(a)
        }
        if let Some(insertion_rule) = rules.get(&(a, b)) {
            next_template.extend_from_slice(&[*insertion_rule, b])
        } else {
            next_template.push(b)
        }
    }
    next_template
}

pub fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);

    let lines = Box::new(buffer.lines());
    let (template, rules) = parse_template_and_rules(lines)?;

    let mut next_template = Vec::from_iter(template.chars());
    for _ in 0..10 {
        next_template = find_next_template(next_template, &rules)
    }

    let mut counter = Counter::default();
    for ch in next_template {
        counter.put(ch)
    }

    Ok(counter.max().unwrap().1 - counter.min().unwrap().1)
}
