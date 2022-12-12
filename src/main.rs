use std::io::{stdin, Read, Error};
use value_enum::value_enum;
use std::fmt;

value_enum!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Token: char {
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

#[derive(Debug, Clone)]
struct Program(Vec<Token>);

impl From<&str> for Program {
    fn from(code: &str) -> Program {
        Program(code.chars().filter_map(|c| Token::try_from(c).ok()).collect())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn read_u8() -> Result<u8, Error> {
            let mut input = [0u8; 1];
            stdin().read_exact(&mut input)?;
            Ok(input[0])
        }
    
        let mut mem = Box::new([0isize; 65536]);
        let mut ptr = 0u16;
        let mut skip = 0;
        let mut stack = Vec::new();
        let mut ip = 0usize;
        while ip < self.0.len() {
            if skip == 0 {
                match self.0[ip] {
                    Token::Plus     => { mem[ptr as usize] += 1;                                              },
                    Token::Minus    => { mem[ptr as usize] -= 1;                                              },
                    Token::Left     => { ptr -= 1;                                                            },
                    Token::Right    => { ptr += 1;                                                            },
                    Token::LBracket => { if mem[ptr as usize] == 0 { skip = 1; } else { stack.push(ip); }     },
                    Token::RBracket => { ip = stack.pop().unwrap_or(ip + 1) - 1;                              },
                    Token::Dot      => { write!(f, "{}", char::from_u32(mem[ptr as usize] as u32).unwrap())?; },
                    Token::Comma    => { mem[ptr as usize] = read_u8().unwrap() as isize;                     }
                }
            } else {
                match self.0[ip] {
                    Token::LBracket => { skip += 1 },
                    Token::RBracket => { skip -= 1 },
                    _               => {           }
                }
            }
            ip += 1;
        }
        Ok(())
    }
}

fn main() {
    let read_line = || {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        Ok::<String, Error>(line)
    };

    println!("enter your code:");
    read_line()
    .map(|code| Program::from(code.as_str()))
    .map(|program| print!("{}", program))
    .unwrap();
}
