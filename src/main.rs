use std::env;
use std::io::{self, Write};
use regex::Regex;

macro_rules! println_with_prompt {
    ($condition: expr, $($arg:tt)*) => {
        {
            if $condition {
                print!("calculator >>");
            }
            println!($($arg)*);
        }
    }
}

macro_rules! print_with_prompt {
    ($condition: expr, $($arg:tt)*) => {
        {
            if $condition {
                print!("calculator >>");
            }
            print!($($arg)*);
            io::stdout().flush().unwrap();
        }
    }
}

struct Config {
    pub enable_prompt: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            enable_prompt: true,
        }
    }

    fn set_config<T>(&mut self, args: T)
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        for arg in args {
            match arg.as_ref() {
                "-p" => self.enable_prompt = false,
                _ => (),
                //unknown => panic!("unknown argument: {unknown}"),
            }
        }
    }
}

struct Application {
    config: Config,
}

impl Application {
    fn new(config: Config) -> Self {
        Application {
            config,
        }
    }

    fn run(&self) {
        loop {
            let mut equation = String::new();
            
            print_with_prompt!(self.config.enable_prompt, "");
            io::stdin()
                .read_line(&mut equation)
                .expect("fail to read line.");
            if equation.is_empty() {
                continue;
            }
    
            let res = calculate(&equation);
            match res {
                Ok(num) => println_with_prompt!(self.config.enable_prompt, "{num}"),
                Err(_) => println_with_prompt!(self.config.enable_prompt, "invalid input: {equation}") 
            }
        }
    }
}

fn main() {
    let mut config = Config::new();
    config.set_config(env::args());

    let app = Application::new(config);
    app.run();
}

#[derive(Debug, PartialEq)]
enum CalUnit {
    Num(i32),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Sub,
    Mul,
    Div,
    LeftBracket,
    //RightBracket,
}

fn calculate(input: &String) -> Result<i32, ()> {
    let rpn;
    let mut stack: Vec<i32> = Vec::new();

    match convert(&input) {
        Ok(res) => rpn = res,
        Err(_) => return Err(())
    }

    //**************
    //dbg!(&rpn.stack);
    //**************

    for unit in rpn.stack {
        match unit {
            CalUnit::Operator(op) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match op {
                    Operator::Plus => stack.push(lhs+rhs),
                    Operator::Div => stack.push(lhs/rhs),
                    Operator::Mul => stack.push(lhs*rhs),
                    Operator::Sub => stack.push(lhs-rhs),
                    _ => (),
                }
            },
            CalUnit::Num(num) => {
                stack.push(num);
            }

        }
    }
        
    Ok(stack.pop().unwrap())
}

struct Stack {
    stack: Vec<CalUnit>,
}

impl Stack {//I implemented this badly... And this was part of my failure
    fn new() -> Self{
        Stack { stack: Vec::new() }
    }

    fn push_str<T: AsRef<str>>(&mut self, t: T) {//my purpose
        match t.as_ref() {
            "+" => self.stack.push(CalUnit::Operator(Operator::Plus)),
            "-" => self.stack.push(CalUnit::Operator(Operator::Sub)),
            "*" => self.stack.push(CalUnit::Operator(Operator::Mul)),
            "/" => self.stack.push(CalUnit::Operator(Operator::Div)),
            "(" => self.stack.push(CalUnit::Operator(Operator::LeftBracket)),
            int => self.stack.push(CalUnit::Num(int.parse().unwrap())),
        }
    }

    fn push(&mut self, t: CalUnit) {
        self.stack.push(t);
    }

    fn last(&self) -> Option<&CalUnit> {
        self.stack.last()
    }

    fn pop(&mut self) -> Option<CalUnit> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn convert(input: &String) -> Result<Stack, &'static str> {
    let pattern = Regex::new(r"(\d+|\+|\-|\*|\/)").unwrap();
    let mut tokens: Vec<String> = Vec::new();
    let mut ret = Stack::new();
    let mut stack = Stack::new();

    for cap in pattern.captures_iter(&input) {
        tokens.push(cap[0].to_string());
    }

    //stack.push_str(tokens.pop().ok_or("no numbers or operators matched")?);

    //************** */
    //dbg!(&tokens);
    //************** */

    for token in tokens {//history like: [None, Some("(")].contains(&stack.last().map(|s| s.as_str()))
        match token.as_str() {
            "+"|"-" => {
                while ![None, Some(&CalUnit::Operator(Operator::LeftBracket))].contains(&stack.last()) {
                    ret.push(stack.pop().unwrap());
                }
                stack.push_str(token);
            },

            "*"|"/" => {
                while [Some(&CalUnit::Operator(Operator::Mul)), 
                Some(&CalUnit::Operator(Operator::Div))]
                .contains(&stack.last()) {
                    ret.push(stack.pop().unwrap());
                }
                stack.push_str(token);
            },

            "(" => stack.push_str(token),

            ")" => {
                while stack.last() != Some(&CalUnit::Operator(Operator::LeftBracket)) {
                    ret.push(stack.pop().unwrap());
                }
                stack.pop();
            },

            _ => ret.push_str(token),
        }
    }
    while !stack.is_empty() {
        ret.push(stack.pop().unwrap());
    }

    Ok(ret)
}