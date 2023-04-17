#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BN", "BZ", "GOTO", "PRINT"];

macro_rules! update_stack {
    ($st: expr, $op: tt) => {
        $st.push($st[0] $op $st[1]);
        chop!($st);
    };
}
macro_rules! chop {
    ($st: expr) => {
        $st.remove(0);
        $st.remove(0);
    };
}

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

#[derive(Debug)]
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

    while line != "END" {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();

        let mut opstack: Vec<Operator> = Vec::new();
        let mut expstack: Vec<Expression> = Vec::new();

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }

        line.remove(line.len() - 1); // Drop the ';'

        let mut it = line.split_whitespace();
        let first = it.next().unwrap();

        let mut next = it.next();

        while let Some(nx) = next {
            let try_expr = nx.parse::<Expression>();
            let mut isexp = true;

            match try_expr {
                Ok(n) => expstack.push(n),
                _ => isexp = false
            }

            // For now, parantheses are ignored
            if !isexp {
                match nx {
                    "+" => opstack.push(Operator::Plus),
                    "-" => opstack.push(Operator::Minus),
                    "*" => opstack.push(Operator::Mul),
                    "/" => opstack.push(Operator::Div),
                    "%" => opstack.push(Operator::Mod),
                    "**" => opstack.push(Operator::Pow),
                    "^" => opstack.push(Operator::Xor),
                    "&" => opstack.push(Operator::And),
                    "|" => opstack.push(Operator::Or),
                    _ => {
                        if vars.contains_key(nx) {
                            expstack.push(*vars.get(nx).unwrap());
                        }
                        else {
                            panic!("Unexpected word '{nx}' (line {})", lc);
                        }
                    }
                }
            }

            next = it.next();
        }

        // Note: (1 + 1) and (1 1 +) are both considered correct by the interpreter and (should) act the same
        
        for op in opstack {
            if expstack.len() < 2 {
                panic!("Cannot apply operator {:?} to less than 2 numbers from the stack (line {})", op, lc);
            }
            use Operator::*;
            match op {
                Plus => {
                    update_stack!(expstack, +);
                },
                Minus => {
                    update_stack!(expstack, -);
                },
                Mul => {
                    update_stack!(expstack, *);
                },
                Div => {
                    update_stack!(expstack, /);
                },
                Mod => {
                    update_stack!(expstack, %);
                },
                Pow => {
                    let a = expstack[0];
                    let b = expstack[1];
                    chop!(expstack);
                    if b > 0 {
                        expstack.push(a.pow(b as u32));
                    }
                    else {
                        expstack.push(1 / a.pow(-b as u32));
                    }
                },
                Xor => {
                    update_stack!(expstack, ^);
                },
                And => {
                    update_stack!(expstack, &);
                },
                Or => {
                    update_stack!(expstack, |);
                }
            }
        }
        
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