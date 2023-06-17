#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use Token::*;
use Operator::*;
use Value::*;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BG", "BZ", "GOTO", "PRINT"];

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

fn precedence(op: Operator) -> i32 {
    match op {
        Open => 0,
        Closed => 0,
        Plus => 1,
        Minus => 1,
        Mul => 2,
        Div => 2
    }    
}

fn unwindopstack(opstack: Vec<Operator>, op: Operator) -> bool {
    if opstack.is_empty() {
        return false;
    }
    return precedence(*opstack.last().unwrap()) >= precedence(op);
}

fn read(input: &mut String) {
    stdout().flush().expect("Flush");
    stdin().read_line(input).expect("Read");
}

// This is only temporary, and should only be used in the math evaluator stage
fn evalrpn(rpnstack: Vec<Token>) {
    let mut numstack: Vec<i32> = Vec::new();

    for token in rpnstack {
        match token {
            Val(val) => {
                numstack.push(match val {
                    Var(_) => panic!("Variables are not yet supported"),
                    Int(int) => int
                });
            }
            Op(op) => {
                if numstack.len() < 2 { panic!("Not enough arguments for operator {:?}", op); }
                
                let b = numstack.pop().unwrap();
                let a = numstack.pop().unwrap();
                numstack.push(match op {
                    Plus => a + b,
                    Minus => a - b,
                    Mul => a * b,
                    Div => a / b,
                    _ => panic!("Something went wrong at parsing to RPN")
                });
            }
        }
    }

    if numstack.len() > 1 { panic!("Too many numbers for the given operators"); }
    println!("{}", numstack.last().unwrap());
}

fn main() {
    let mut lc = 1usize; // line counter
    let mut line = String::from("");
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    loop {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();

        let mut opstack: Vec<Operator> = Vec::new();
        let mut tokstack: Vec<Token> = Vec::new();
        let mut rpnstack: Vec<Token> = Vec::new();

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }
        line.remove(line.len() - 1); // Drop the ';'
        if line == "END" {
            break;
        }

        let mut it = line.split_whitespace();
        let mut next = it.next();

        while let Some(word) = next {
            let mut num = 0i32;
            let mut is_num = false;

            let mut var = String::new();
            let mut is_var = false;

            for (i, ch) in word.chars().enumerate() {
                if ch.is_alphabetic() {
                    if is_num { panic!("Syntax error"); }
                    is_var = true;
                    var.push(ch);
                }
                else if ch.is_numeric() {
                    if is_var {
                        var.push(ch);
                    }
                    else {
                        is_num = true;
                        num = num * 10 + (ch as i32 - 48);
                    }
                }
                else {
                    if is_num { tokstack.push(Val(Int(num))); }
                    if is_var { tokstack.push(Val(Var(var.clone()))); }
                    is_num = false;
                    is_var = false;
                    num = 0;
                    var.clear();

                    if ch == '(' {
                        tokstack.push(Op(Open));
                    }
                    else if ch == ')' {
                        tokstack.push(Op(Closed));
                    }
                    else {
                        tokstack.push(match ch {
                            '+' => Op(Plus),
                            '-' => Op(Minus),
                            '*' => Op(Mul),
                            '/' => Op(Div),
                            _ => panic!("Something went wrong")
                        });
                    }
                }

                if i == word.len() - 1 {
                    if is_var {
                        tokstack.push(Val(Var(var.clone())));
                    }
                    if is_num {
                        tokstack.push(Val(Int(num)));
                    }
                }
            }

            next = it.next();
        }

        for token in &tokstack {
            match token {
                Val(v) => rpnstack.push(Val(v.clone())),
                Op(op) => {
                    match *op {
                        Open => opstack.push(*op),
                        Closed => {
                            while opstack.last().unwrap() != &Open {
                                rpnstack.push(Op(opstack.pop().unwrap()));
                            }
                            opstack.pop().unwrap();
                        },
                        any => {
                            while unwindopstack(opstack.clone(), any) {
                                rpnstack.push(Op(opstack.pop().unwrap()));
                            }
                            opstack.push(any);
                        }
                    }
                }
            }
        }
        
        for op in opstack.iter().rev() {
            rpnstack.push(Op(*op));
        }
        evalrpn(rpnstack);
    }
}