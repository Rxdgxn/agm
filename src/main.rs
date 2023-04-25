#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BG", "BZ", "GOTO", "PRINT"];

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
    DVAR(String),
    VAR(String, Expression),
    LABEL(String),
    GOTO(String),
    PRINT(Expression),
    BZ(Expression, *const Instruction),
    BG(Expression, *const Instruction)
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

// bz 0 print 1

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

        let mut non_default = false;

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }

        line.remove(line.len() - 1); // Drop the ';'

        let mut it = line.split_whitespace().rev();
        // let first = it.next().unwrap();

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
                            if nx == ":=" {
                                non_default = true;
                            }
                            else {
                                if KEYWORDS.contains(&nx) {
                                    // TODO: labels
                                    match nx {
                                        "BEG" => program.instructions.push(Instruction::BEG),
                                        "END" => program.instructions.push(Instruction::END),
                                        "BG" => {
                                            if let Some(instr) = program.instructions.last() {
                                                program.instructions.push(Instruction::BG(*expstack.last().unwrap(), instr));
                                            }
                                            else {
                                                panic!("BG block must contain an instruction (line {lc})");
                                            }
                                        },
                                        "BZ" => {
                                            if let Some(instr) = program.instructions.last() {
                                                program.instructions.push(Instruction::BZ(*expstack.last().unwrap(), instr));
                                            }
                                            else {
                                                panic!("BZ block must contain an instruction (line {lc})");
                                            }
                                        },
                                        "PRINT" => program.instructions.push(Instruction::PRINT(*expstack.last().unwrap())),
                                        _ => {}
                                    }
                                }
                                else {
                                    // TODO: variabless
                                    panic!("Unexpected word '{}' (line {})", nx, lc);
                                }
                            }
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

        println!("{:?}", expstack);

        lc += 1;
    }
}