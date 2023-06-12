#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BG", "BZ", "GOTO", "PRINT"];

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

#[derive(Clone, Debug)]
enum Instruction {
    END,
    GOTO(String),
    PRINT(Value),
    BZ(Value, Box<Instruction>),
    BG(Value, Box<Instruction>),
    MUTATE(Value, Value) // stub
}

#[derive(Clone, Debug)]
enum Value {
    Int(i32),
    Var(String)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Open,
    Closed
}

#[derive(Debug)]
enum Token {
    Op(Operator),
    Val(Value)
}

fn read(input: &mut String) {
    stdout().flush().expect("Flush");
    stdin().read_line(input).expect("Read");
}

fn main() {
    let mut lc = 1usize; // line counter
    let mut line = String::from("");
    let mut vars: HashMap<String, Value> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut program = Program::new();

    while line != "END" {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();

        let mut opstack: Vec<Operator> = Vec::new();
        let mut tokstack: Vec<Token> = Vec::new();

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }
        line.remove(line.len() - 1); // Drop the ';'

        let mut it = line.split_whitespace();
        let mut next = it.next();

        while let Some(word) = next {
            use Token::*;
            use Operator::*;
            use Value::*;

            let mut num = 0i32;
            let mut is_num = false;

            let mut var = String::new();

            for (i, ch) in word.chars().enumerate() {
                
                if ch.is_alphabetic() {
                    var.push(ch);
                }
                else {
                    if !var.is_empty() {
                        tokstack.push(Val(Var(var)));
                        var = String::new();
                    }

                    if !ch.is_digit(10) || i == word.len() - 1 {
                        if is_num {
                            tokstack.push(Val(Int(num)));
                        }
                        is_num = false;
                        num = 0;
                        
                        tokstack.push(match ch {
                            '(' => Op(Open),
                            ')' => Op(Closed),
                            '+' => Op(Plus),
                            '-' => Op(Minus),
                            '*' => Op(Mul),
                            '/' => Op(Div),
                            _ => {
                                if ch.is_digit(10) {
                                    Val(Int(ch as i32 - 48))
                                }
                                else {
                                    panic!("Oh oh")
                                }
                            }
                        });
                    }
                    else {
                        num = num * 10 + (ch as i32 - 48);
                        is_num = true;
                    }
                }
            }

            next = it.next();
        }

        println!("{:?}", tokstack);
    }
}