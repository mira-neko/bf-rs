use std::io::{stdin};
use read_char::read_next_char;
use value_enum::value_enum;

value_enum!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Bfi: char {
        Plus     = '+',
        Minus    = '-',
        Left     = '<',
        Right    = '>',
        LBracket = '[',
        RBracket = ']',
        Dot      = '.',
        Comma    = ','
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bci {
    Plus(usize),
    Minus(usize),
    Left(usize),
    Right(usize),
    LBracket,
    RBracket,
    Dot,
    Comma
}

fn parse(code: &str) -> Vec<Bfi> {
    code.chars().filter_map(|c| Bfi::try_from(c).ok()).collect()
}

fn compile(parsed: &[Bfi]) -> Vec<Bci> {
    let mut result = Vec::new();
    let mut last = None;
    let mut n = 0;
    for i in parsed.iter().map(Option::Some) {
        if i == last {
            n += 1;
        } else {
            if let Some(ulast) = last {
                match ulast {
                    Bfi::Plus => result.push(Bci::Plus(n)),
                    Bfi::Minus => result.push(Bci::Minus(n)),
                    Bfi::Left => result.push(Bci::Left(n)),
                    Bfi::Right => result.push(Bci::Right(n)),
                    Bfi::LBracket => for _ in 0..n {result.push(Bci::LBracket)},
                    Bfi::RBracket => for _ in 0..n {result.push(Bci::RBracket)},
                    Bfi::Dot => for _ in 0..n {result.push(Bci::Dot)},
                    Bfi::Comma => for _ in 0..n {result.push(Bci::Comma)},
                };
            }
            last = i;
            n = 1;
        }
    }
    if let Some(ulast) = last {
        match ulast {
            Bfi::Plus => result.push(Bci::Plus(n)),
            Bfi::Minus => result.push(Bci::Minus(n)),
            Bfi::Left => result.push(Bci::Left(n)),
            Bfi::Right => result.push(Bci::Right(n)),
            Bfi::LBracket => for _ in 0..n {result.push(Bci::LBracket)},
            Bfi::RBracket => for _ in 0..n {result.push(Bci::RBracket)},
            Bfi::Dot => for _ in 0..n {result.push(Bci::Dot)},
            Bfi::Comma => for _ in 0..n {result.push(Bci::Comma)},
        };
    }
    result
}

fn eval(bc: &[Bci]) {
    let mut mem = [0u32; 65536];
    let mut mp = 0u16;
    let mut ip = 0;
    let mut stack = Vec::new();
    // let mut i = 0;
    while ip < bc.len() {
        match bc[ip] {
            Bci::Plus(n) => {
                mem[mp as usize] = mem[mp as usize].overflowing_add(n as u32).0;
                ip += 1;
            },
            Bci::Minus(n) => {
                mem[mp as usize] = mem[mp as usize].overflowing_sub(n as u32).0;
                ip += 1;
            },
            Bci::Left(n) => {
                mp = mp.overflowing_sub(n as u16).0;
                ip += 1;
            },
            Bci::Right(n) => {
                mp = mp.overflowing_add(n as u16).0;
                ip += 1;
            },
            Bci::LBracket => {
                if mem[mp as usize] == 0 {
                    let mut level = 1;
                    while level != 0 {
                        ip += 1;
                        match bc[ip] {
                            Bci::LBracket => level += 1,
                            Bci::RBracket => level -= 1,
                            _ => ()
                        };
                    }
                    ip += 1;
                } else {
                    stack.push(ip);
                    ip += 1;
                }
            },
            Bci::RBracket => ip = stack.pop().unwrap(),
            Bci::Dot => {
                print!("{}", char::from_u32(mem[mp as usize]).unwrap_or('ᅠ'));
                ip += 1;
            },
            Bci::Comma => {
                mem[mp as usize] = read_next_char(&mut stdin()).unwrap() as u32;
                ip += 1;
            }
        }
        // i += 1;
    }
    // println!("\niterations: {}", i);
}

fn main() {
    let mut code = String::new();
    println!("enter your code:");
    stdin().read_line(&mut code).unwrap();
    let code = code;
    let parsed = parse(&code);
    let bc = compile(&parsed);
    // println!("{:?}", bc);
    eval(&bc);
}
