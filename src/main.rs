// #![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]
use std::collections::HashMap;
use std::slice::Iter;
use Token::*;
use Operator::*;
use Value::*;
use Instruction::*;

const KEYWORDS: [&str; 5] = ["BG", "BZ", "GOTO", "PRINT", "NEWL"];

#[derive(Clone, Debug)]
enum Instruction {
    PRINT(Vec<Token>),
    NEWL,
    GOTO(Token),
    BZ(Vec<Token>, Box<Instruction>),
    BG(Vec<Token>, Box<Instruction>),
    LABEL,
    MUTATE(String, Vec<Token>)
}

#[derive(Clone, Debug)]
enum Value {
    Int(i32),
    Word(String),
    Str(String)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
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
        Div => 2,
        Mod => 2,
        And => 2,
        Or => 2,
        Xor => 2
    }    
}

fn unwindopstack(opstack: Vec<Operator>, op: Operator) -> bool {
    if opstack.is_empty() {
        return false;
    }
    return precedence(*opstack.last().unwrap()) >= precedence(op);
}

fn token_to_string(tok: &Token) -> Option<&String> {
    match tok {
        Val(v) => {
            match v {
                Word(w) => Some(w),
                _ => None
            }
        }
        _ => None
    }
}

fn return_instr(w: &str, rpnstack: &Vec<Token>, it: &mut Iter<'_, Token>) -> Option<Instruction> {
    match w {
        "PRINT" => Some(PRINT(rpnstack[1..rpnstack.len()].to_vec())),
        "NEWL" => Some(NEWL),
        "GOTO" => Some(GOTO((&rpnstack[1]).clone())),
        "BG" => {
            let mut tmpit = it.clone();
            let mut tmp = tmpit.next();
            let mut idx = 1usize;
            while let Some(t) = tmp {
                if token_to_string(t) != None {
                    if KEYWORDS.contains(&(&token_to_string(t).unwrap() as &str)) {
                        return Some(BG(rpnstack[1..idx].to_vec(), Box::new(return_instr(&token_to_string(t).unwrap(), &rpnstack[idx..rpnstack.len()].to_vec(), &mut tmpit).unwrap())));
                    }
                }
                tmp = tmpit.next();
                idx += 1;
            }
            return None;
        }
        "BZ" => {
            let mut tmpit = it.clone();
            let mut tmp = tmpit.next();
            let mut idx = 1usize;
            while let Some(t) = tmp {
                if token_to_string(t) != None {
                    if KEYWORDS.contains(&(&token_to_string(t).unwrap() as &str)) {
                        return Some(BZ(rpnstack[1..idx].to_vec(), Box::new(return_instr(&token_to_string(t).unwrap(), &rpnstack[idx..rpnstack.len()].to_vec(), &mut tmpit).unwrap())));
                    }
                }
                tmp = tmpit.next();
                idx += 1;
            }
            return None;
        }
        _ => panic!("Something went wrong")
    }
}

fn evalrpn(rpnstack: &Vec<Token>, vars: &HashMap<String, i32>, lc: usize, program: &mut Vec<Instruction>, labels: &HashMap<String, usize>) -> (i32, String) {
    let mut numstack: Vec<i32> = Vec::new();
    let mut string = String::new();

    let mut it = rpnstack.iter();
    let mut next = it.next();

    while let Some(token) = next {
        match token {
            Val(val) => {
                match val {
                    Int(int) => numstack.push(*int),
                    Word(w) => match vars.contains_key(w) {
                        true => numstack.push(vars[w]),
                        false => {
                            if KEYWORDS.contains(&(w as &str)) {
                                program.push(return_instr(w as &str, rpnstack, &mut it).unwrap());
                                break;
                            }
                            else if labels.contains_key(w) {
                                // This should never happen
                                numstack.push(-1);
                            }
                            else { panic!("Unknown word {w} at line {lc}"); }
                        }
                    }
                    Str(s) => string = s.clone()
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
                    Mod => a % b,
                    And => a & b,
                    Or => a | b,
                    Xor => a ^ b,
                    _ => panic!("Something went wrong at parsing to RPN")
                });
            }
        }
        next = it.next();
    }

    if numstack.len() > 1 { panic!("Unappropriate numstack len: {}", numstack.len()); }
    if numstack.len() > 0 && !string.is_empty() { panic!("Cannot return an integer and a string at the same time"); }
    
    if let Some(n) = numstack.last() {
        return (*n, string);
    }
    return (-1, string); // Still not the optimal solution
}

fn evalprogram(program: &mut Vec<Instruction>, vars: &mut HashMap<String, i32>, labels: &mut HashMap<String, usize>) -> usize {
    let mut idx = 0usize;

    while idx < program.len() {
        match &program[idx].clone() {
            PRINT(rpnstack) => {
                let res = evalrpn(rpnstack, vars, idx, program, &labels);
                if res.1.is_empty() {
                    print!("{}", res.0);
                }
                else {
                    print!("{}", res.1);
                }
            }
            NEWL => println!(),
            GOTO(at) => {
                if let Some(w) = token_to_string(at) {
                    idx = labels[w];
                    continue;
                }
                panic!("Invalid GOTO parameter at line {idx}: {:?}", at);
            }
            BG(rpnstack, instr) => {
                if evalrpn(rpnstack, vars, idx, program, &labels).0 > 0 {
                    let mut p = Vec::new();
                    p.push(*instr.clone());
                    idx = evalprogram(&mut p, vars, labels);
                }
            }
            BZ(rpnstack, instr) => {
                if evalrpn(rpnstack, vars, idx, program, &labels).0 == 0 {
                    let mut p = Vec::new();
                    p.push(*instr.clone());
                    idx = evalprogram(&mut p, vars, labels);
                }
            }
            LABEL => {}
            MUTATE(var, rpnstack) => _ = vars.insert(var.clone(), evalrpn(rpnstack, vars, idx, program, labels).0)
        }
        idx += 1;
    }

    idx
}

fn main() {
    let mut lc = 1usize; // line counter
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut program: Vec<Instruction> = Vec::new();

    let fc = include_str!("../fib.agm");
    let split = fc.split(';');

    for line in split {
        let mut line = line.trim().to_string();

        let mut opstack: Vec<Operator> = Vec::new();
        let mut tokstack: Vec<Token> = Vec::new();
        let mut rpnstack: Vec<Token> = Vec::new();
        let mut var = String::new();

        let mut in_string = false;
        let mut string = String::new();

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

        let mut num = 0i32;
        let mut is_num = false;

        let mut word = String::new();
        let mut is_word = false;

        for (i, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() || ch == '$' {
                if !in_string {
                    if is_num { panic!("Syntax error"); }
                    is_word = true;
                    word.push(ch);
                }
                else {
                    string.push(ch);
                }
            }
            else if ch.is_numeric() {
                if !in_string {
                    if is_word {
                        word.push(ch);
                    }
                    else {
                        is_num = true;
                        num = num * 10 + (ch as i32 - 48);
                    }
                }
                else {
                    string.push(ch);
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

                if ch == '\"' {
                    in_string = !in_string;
                    if !in_string {
                        tokstack.push(Val(Str(string.clone())));
                        string.clear();
                    }
                    continue;
                }
                
                if in_string {
                    string.push(ch);
                    continue;
                }

                if ch == ' ' {
                    continue;
                }

                tokstack.push(match ch {
                    '+' => Op(Plus),
                    '-' => Op(Minus),
                    '*' => Op(Mul),
                    '/' => Op(Div),
                    '(' => Op(Open),
                    ')' => Op(Closed),
                    '%' => Op(Mod),
                    '&' => Op(And),
                    '|' => Op(Or),
                    '^' => Op(Xor),
                    _ => panic!("Something went wrong")
                });
            }

            if i == line.len() - 1 {
                if is_word {
                    tokstack.push(Val(Word(word.clone())));
                }
                if is_num { tokstack.push(Val(Int(num))); }
            }
        }

        for token in &tokstack {

            if let Some(t) = token_to_string(token) {
                if KEYWORDS.contains(&(t as &str)) {
                    for op in opstack.iter().rev() {
                        rpnstack.push(Op(*op));
                    }
                    opstack.clear();
                }
            }

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

        if rpnstack.len() == 1 && var.is_empty() && !KEYWORDS.contains(&(token_to_string(rpnstack.clone().last().unwrap()).unwrap() as &str)) {
            match rpnstack.clone().last().unwrap() {
                Op(o) => panic!("Expected 2 operands for operator {:?} at line {lc}", o),
                Val(v) => match v {
                    Int(_) => {},
                    Word(w) => {
                        if &w[0..=0] == "$" {
                            program.push(MUTATE(w.clone(), Vec::from([Val(Int(0))])));
                        }
                        else {
                            labels.insert(w.clone(), lc);
                            program.push(LABEL); // LABEL is just a marker
                        }
                        lc += 1;
                        continue;
                    }
                    Str(_) => {}
                }
            }
        }

        if !var.is_empty() {
            program.push(MUTATE(var, rpnstack));
        }
        else {
            evalrpn(&rpnstack, &vars, lc, &mut program, &labels);
        }

        lc += 1;
    }

    evalprogram(&mut program, &mut vars, &mut labels);
}