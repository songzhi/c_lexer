use crate::{
    error::Error,
    token::{Number, Token},
};
use std::str;

#[inline]
pub fn parse_number_radix(
    input: &[u8],
    c_src: &mut usize,
    token_len: u64,
    radix: u8,
) -> Result<Token, Error> {
    let i =
        unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize + 2..*c_src - 1]) };
    let i = u32::from_str_radix(i, u32::from(radix))?;
    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(i, 0, 1, radix)))
}

#[inline]
pub fn parse_number(input: &[u8], c_src: &mut usize, token_len: u64) -> Result<Token, Error> {
    let i = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..*c_src - 1]) };
    let i = u32::from_str_radix(i, 10)?;
    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(i, 0, 1, 10)))
}

#[inline]
pub fn parse_number_decimal(
    input: &[u8],
    c_src: &mut usize,
    token_len: u64,
) -> Result<Token, Error> {
    let mut i_point = 0;
    for (i, item) in input
        .iter()
        .enumerate()
        .take(*c_src - 1)
        .skip(*c_src - token_len as usize)
    {
        if *item == b'.' {
            i_point = i;
            break;
        }
    }
    let integer = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_point]) };
    let integer = u32::from_str_radix(integer, 10)?;

    let decimal = unsafe { str::from_utf8_unchecked(&input[i_point + 1..*c_src - 1]) };
    let decimal = u32::from_str_radix(decimal, 10)?;

    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(integer, decimal, 1, 10)))
}

#[inline]
pub fn parse_exponent(input: &[u8], c_src: &mut usize, token_len: u64) -> Result<Token, Error> {
    let mut i_e = 0;
    let mut i_point = None;
    for (i, item) in input
        .iter()
        .enumerate()
        .take(*c_src - 1)
        .skip(*c_src - token_len as usize)
    {
        if *item == b'.' {
            i_point = Some(i);
        }
        if *item == b'e' || *item == b'E' {
            i_e = i;
            break;
        }
    }

    let (integer, decimal) = if i_point.is_some() {
        let integer = unsafe {
            str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_point.unwrap()])
        };
        let integer = u32::from_str_radix(integer, 10)?;
        let decimal = unsafe { str::from_utf8_unchecked(&input[i_point.unwrap() + 1..i_e]) };
        (integer, u32::from_str_radix(decimal, 10)?)
    } else {
        let integer = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_e]) };
        let integer = u32::from_str_radix(integer, 10)?;
        (integer, 0)
    };

    let exponent = unsafe { str::from_utf8_unchecked(&input[i_e + 1..*c_src - 1]) };
    let exponent = i64::from_str_radix(exponent, 10).unwrap();
    *c_src += 1;
    Ok(Token::NumericLiteral(Number::new(
        integer, decimal, exponent, 10,
    )))
}