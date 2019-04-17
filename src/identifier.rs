use crate::token::Token;
use internship::IStr;
use std::str;

const KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "auto" => Token::AUTO,
    "break" => Token::BREAK,
    "case" => Token::CASE,
    "char" => Token::CHAR,
    "const" => Token::CONST,
    "continue"=> Token::CONTINUE,
    "default" => Token::DEFAULT,
    "do" => Token::DO,
    "double" => Token::DOUBLE,
    "else" => Token::ELSE,
    "enum" => Token::ENUM,
    "extern" => Token::EXTERN,
    "float" => Token::FLOAT,
    "for" => Token::FOR,
    "goto" => Token::GOTO,
    "if" => Token::IF,
    "inline" => Token::INLINE,
    "int" => Token::INT,
    "long" => Token::LONG,
    "register" => Token::REGISTER,
    "restrict" => Token::RESTRICT,
    "return" => Token::RETURN,
    "short" => Token::SHORT,
    "signed" => Token::SIGNED,
    "sizeof" => Token::SIZEOF,
    "static" => Token::STATIC,
    "struct" => Token::STRUCT,
    "switch" => Token::SWITCH,
    "typedef" => Token::TYPEDEF,
    "union" => Token::UNION,
    "unsigned" => Token::UNSIGNED,
    "void" => Token::VOID,
    "volatile" => Token::VOLATILE,
    "while" => Token::WHILE,
    "_Alignas" => Token::ALIGNAS,
    "_Alignof" => Token::ALIGNOF,
    "_Atomic" => Token::ATOMIC,
    "_Bool" => Token::BOOL,
    "_Complex" => Token::COMPLEX,
    "_Generic" => Token::GENERIC,
    "_Imaginary" => Token::IMAGINARY,
    "_Noreturn" => Token::NORETURN,
    "_Static_assert" => Token::StaticAssert,
    "_Thread_local" => Token::ThreadLocal,
    "__func__" => Token::FuncName,
};

#[inline]
fn is_identifier_part(cp: u8) -> bool {
    cp == 0x24
        || cp == 0x5F
        || (cp >= 0x41 && cp <= 0x5A)
        || (cp >= 0x61 && cp <= 0x7A)
        || (cp >= 0x30 && cp <= 0x39)
        || cp == 0x5C
        || cp >= 0x80
}

#[inline]
pub fn parse_identifier(input: &[u8], c_src: &mut usize) -> Token {
    let mut it = 0;
    for i in 0..input.len() - *c_src {
        if !unsafe { is_identifier_part(*input.get_unchecked(*c_src + i)) } {
            it = i;
            break;
        }
    }
    let ident = &input[*c_src - 1..*c_src + it];
    *c_src += it;
    let ident = unsafe { str::from_utf8_unchecked(ident) };
    KEYWORDS
        .get(ident)
        .cloned()
        .unwrap_or_else(|| Token::Identifier(IStr::new(ident)))
}
