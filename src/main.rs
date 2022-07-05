use std::io::{stdin, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Plus,
    Minus,
    Left,
    Right,
    LBracket,
    RBracket,
    Dot,
    Comma
}

fn tokenize(code: &str) -> impl Iterator<Item = Token> {
    let mut tokens = Vec::with_capacity(code.len());
    for c in code.chars() {
        match c {
            '+' => { tokens.push(Token::Plus);     },
            '-' => { tokens.push(Token::Minus);    },
            '<' => { tokens.push(Token::Left);     },
            '>' => { tokens.push(Token::Right);    },
            '[' => { tokens.push(Token::LBracket); },
            ']' => { tokens.push(Token::RBracket); },
            '.' => { tokens.push(Token::Dot);      },
            ',' => { tokens.push(Token::Comma);    },
            _   => {                               }
        }
    }
    tokens.into_iter()
}

fn execute(tokens: impl Iterator<Item = Token>) {
    let tokens = tokens.collect::<Vec::<_>>();
    let mut mem = Box::new([0isize; 65536]);
    let mut ptr = 0u16;
    let mut skip = 0;
    let mut stack = Vec::new();
    let mut ip = 0usize;
    while ip < tokens.len() {
        if skip == 0 {
            match tokens[ip] {
                Token::Plus     => { mem[ptr as usize] += 1;                                                                                   },
                Token::Minus    => { mem[ptr as usize] -= 1;                                                                                   },
                Token::Left     => { ptr -= 1;                                                                                                 },
                Token::Right    => { ptr += 1;                                                                                                 },
                Token::LBracket => { if mem[ptr as usize] == 0 { skip = 1; } else { stack.push(ip); }                                          },
                Token::RBracket => { ip = stack.pop().unwrap_or(ip + 1) - 1;                                                                   },
                Token::Dot      => { print!("{}", char::from_u32(mem[ptr as usize] as u32).unwrap_or('@'));                                    },
                Token::Comma    => { let mut input = [0u8; 1]; stdin().read_exact(&mut input).unwrap(); mem[ptr as usize] = input[0] as isize; }
            }
        } else {
            match tokens[ip] {
                Token::LBracket => { skip += 1 },
                Token::RBracket => { skip -= 1 },
                _               => {           }
            }
        }
        // println!("ip: {}; token: {:?}; ptr: {}; mem: {:?}; stack: {:?}; skip: {}", ip, tokens[ip], ptr, &mem[0..3], stack, skip);
        ip += 1;
    }
}

fn main() {
    println!("enter your code:");
    let mut codes = String::new();
    stdin().read_line(&mut codes).unwrap();
    let code = codes.as_str();
    execute(tokenize(code));
}
