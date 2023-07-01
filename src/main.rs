// #![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use Token::*;
use Operator::*;
use Value::*;
use Instruction::*;

const KEYWORDS: [&str; 6] = ["BEG", "END", "BG", "BZ", "GOTO", "PRINT"];

#[derive(Clone, Debug)]
enum Instruction<> {
    PRINT(Vec<Token>),
    GOTO(Token),
    BZ(Vec<Token>, Box<Instruction>),
    BG(Vec<Token>, Box<Instruction>),
    MUTATE(String, Vec<Token>) // stub
}

#[derive(Clone, Debug)]
enum Value {
    Int(i32),
    Word(String)
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

#[derive(Debug, Clone)]
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

fn evalrpn(rpnstack: &Vec<Token>, vars: &HashMap<String, i32>, lc: usize, program: &mut Vec<Instruction>) -> i32 {
    let mut numstack: Vec<i32> = Vec::new();

    for token in rpnstack {
        match token {
            Val(val) => {
                match val {
                    Int(int) => numstack.push(*int),
                    Word(w) => match vars.contains_key(w) {
                        true => numstack.push(vars[w]),
                        false => {
                            if KEYWORDS.contains(&(w as &str)) {
                                match w as &str {
                                    // The 1 index should be valid when the rpnstack is passed to the instruction
                                    "PRINT" => program.push(PRINT(rpnstack[1..rpnstack.len()].to_vec())),
                                    "GOTO" => program.push(GOTO((&rpnstack[1]).clone())),
                                    _ => todo!()
                                }
                            }
                            else { panic!("Unknown word {w} at line {lc}"); }
                        }
                    }
                }
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
    
    return *numstack.last().unwrap();
}

fn evalprogram(program: Vec<Instruction>) {
    
}

fn main() {
    let mut lc = 1usize; // line counter
    let mut line: String;
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut program: Vec<Instruction> = Vec::new();

    loop {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();

        let mut opstack: Vec<Operator> = Vec::new();
        let mut tokstack: Vec<Token> = Vec::new();
        let mut rpnstack: Vec<Token> = Vec::new();
        let mut var = String::new();

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }
        line.remove(line.len() - 1); // Drop the ';'
        if line == "END" {
            break;
        }

        if line.contains(":=") {
            let mut split = line.split(":=");

            var = split.next().unwrap().trim().to_string();
            if &var[0..=0] != "$" {
                panic!("Variables must begin with $! (line {lc})");
            }

            line = split.next().unwrap().trim().to_string();
        }

        let mut it = line.split_whitespace();
        let mut next = it.next();

        while let Some(partition) = next {
            let mut num = 0i32;
            let mut is_num = false;

            let mut word = String::new();
            let mut is_word = false;

            for (i, ch) in partition.chars().enumerate() {
                if ch.is_alphabetic() || ch == '$' {
                    if is_num { panic!("Syntax error"); }
                    is_word = true;
                    word.push(ch);
                }
                else if ch.is_numeric() {
                    if is_word {
                        word.push(ch);
                    }
                    else {
                        is_num = true;
                        num = num * 10 + (ch as i32 - 48);
                    }
                }
                else {
                    if is_num { tokstack.push(Val(Int(num))); }
                    if is_word {
                        tokstack.push(Val(Word(word.clone())));
                    }
                    is_num = false;
                    is_word = false;
                    num = 0;
                    word.clear();

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

                if i == partition.len() - 1 {
                    if is_word {
                        tokstack.push(Val(Word(word.clone())));
                    }
                    if is_num { tokstack.push(Val(Int(num))); }
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

        if rpnstack.len() == 1 && var.is_empty() {
            match rpnstack.clone().last().unwrap() {
                Op(o) => panic!("Expected 2 operands for operator {:?} at line {lc}", o),
                Val(v) => match v {
                    Int(_) => {},
                    Word(w) => {
                        if &w[0..=0] == "$" {
                            vars.insert(w.clone(), 0);
                        }
                        else {
                            labels.insert(w.clone(), lc);
                        }
                        lc += 1;
                        continue;
                    }
                }
            }
        }
        
        for op in opstack.iter().rev() {
            rpnstack.push(Op(*op));
        }

        if !var.is_empty() {
            vars.insert(var, evalrpn(&rpnstack, &vars, lc, &mut program));
        }

        lc += 1;
    }

    evalprogram(program);
}