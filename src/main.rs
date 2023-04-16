#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BN", "BZ", "GOTO", "PRINT"];

fn read(input: &mut String) {
    stdout().flush().expect("Flush");
    stdin().read_line(input).expect("Read");
}

struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn new() -> Self {
        Self {
            instructions: Vec::new()
        }
    }
}

enum Instruction {
    BEG,
    END,
    ZVAR(String),
    VAR(String, Expression),
    LABEL(String),
    GOTO(String),
    PRINT(Expression),
    BZ(Expression, Box<Instruction>),
    BG(Expression, Box<Instruction>)
}

type Expression = i32;

enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    Xor,
    And,
    Or
}

fn main() {

    let mut lc = 1u32; // line counter
    let mut line = String::from("");
    let mut vars: HashMap<String, Expression> = HashMap::new();
    let mut program = Program::new();

    while line != "END;" {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();

        let mut stack: Vec<Operator> = Vec::new();
        let mut substack: Vec<Expression> = Vec::new();

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }

        let mut it = line.split_whitespace();
        let first = it.next().unwrap();
        
        // TODO: skip the first word to process the numerical stack (if there is any),
        // then decide what to do with the value from the top of the stack (preferably the stack should only have 1 element)

        if KEYWORDS.contains(&first) {
            match first {
                _ => {}
            }
        }
        else {
            if &first[0..=0] == "$" {

            }
            else {

            }
        }

        lc += 1;
    }

}