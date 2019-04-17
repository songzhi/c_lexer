use crate::token::{Token, Number};
use std::{char::from_u32, str};

// # Performance
// If string doesn't contain any escaping characters we can skip
// allocation entirely. This is achieved by allocating new string
// of size of input only after escaping character is encountered
#[inline]
fn to_unescaped(input: String) -> String {
    let mut result: Option<String> = None;
    let bytes = input.as_bytes();
    let mut escaping = false;
    let mut i = 0..bytes.len();
    loop {
        let next = i.next();
        if next == None {
            break;
        }
        let c = unsafe { bytes.get_unchecked(next.unwrap()) };
        if *c == b'\\' {
            escaping = true;
            if result.is_none() {
                result = Some(String::from(&input[..next.unwrap()]));
                result.as_mut().unwrap().reserve(input.len());
            }
            continue;
        }
        if escaping {
            escaping = false;
            let res = match *c as char {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'b' => '\x08',
                'v' => '\x0B',
                'f' => '\x0C',
                '0' => '\0',
                'u' => {
                    let index = i.next().unwrap();
                    let nums = &bytes[index..index + 4];
                    for _ in 0..3 {
                        i.next();
                    }
                    let as_num = u64::from_str_radix(unsafe { str::from_utf8_unchecked(nums) }, 16).unwrap_or(0);
                    from_u32(as_num as u32).expect(format!("{} is not a valid unicode scalar value", as_num).as_str())
                }
                'x' => {
                    let index = i.next().unwrap();
                    let nums = &bytes[index..index + 2];
                    for _ in 0..3 {
                        i.next();
                    }
                    let as_num = u64::from_str_radix(unsafe { str::from_utf8_unchecked(nums) }, 16).unwrap_or(0);
                    from_u32(as_num as u32).expect(format!("{} is not a valid unicode scalar value", as_num).as_str())
                }
                _ => *c as char,
            };
            result.as_mut().unwrap().push(res);
            continue;
        }
        if result.is_some() {
            result.as_mut().unwrap().push(*c as char);
        }
    }
    result.unwrap_or(input)
}

#[inline]
fn parse(input: &[u8], c_src: &mut usize, type_: u8) -> String {
    let mut token_len = 0;
    while input.len() - 1 > *c_src && (input[*c_src] != type_ || input[*c_src - 1] == b'\\') {
        *c_src += 1;
        token_len += 1;
    }
    let res = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len..*c_src]).to_string() };
    let res = to_unescaped(res);
    *c_src += 1;
    res
}

#[inline]
pub fn parse_string(input: &[u8], c_src: &mut usize) -> Token {
    Token::StringLiteral(parse(input, c_src, b'"'))
}

#[inline]
pub fn parse_char(input: &[u8], c_src: &mut usize) -> Token {
    let res = parse(input, c_src, b'\'');
    debug_assert_eq!(1, res.len());
    Token::NumericLiteral(Number::new(res.chars().next().unwrap() as u32, 0, 0, 10))
}
