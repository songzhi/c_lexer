use internship::IStr;

/// Number representation of parsed number
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Number {
    /// Whole part of number
    pub integer: u32,
    /// Decimal part of number
    pub decimal: u32,
    /// Number behind E / e (exponent)
    pub exponent: i64,
    /// base of number
    pub radix: u8,
}

impl Number {
    /// Create instance of js representation of number
    #[inline]
    pub fn new(integer: u32, decimal: u32, exponent: i64, radix: u8) -> Self {
        Self {
            integer,
            decimal,
            exponent,
            radix,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    LBrace,       // {
    RBrace,       // }
    LParen,       // (
    RParen,       // )
    LBracket,     // [
    RBracket,     // ]
    Semicolon,    // ;
    Assign,       // =
    Lt,           // <
    Gt,           // >
    Minus,        // -
    Tilde,        // ~
    Exclamation,  // !
    Plus,         // +
    Multi,        // *
    Slash,       // /
    Colon,        // :
    QuestionMark, // ?
    Comma,        // ,
    Dot,          // .
    SingleAnd,    // &
    InclusiveOr,  // |
    ExclusiveOr,  // ^
    Mod,          // %
    Identifier(IStr),
    NumericLiteral(Number),
    StringLiteral(String),
    FuncName,    // __func__
    SIZEOF,      // sizeof
    PtrOp,       // ->
    IncOp,       // ++
    DecOp,       // --
    LeftOp,      // <<
    RightOp,     // >>
    LeOp,        // <=
    GeOp,        // >=
    EqOp,        // ==
    NeOp,        // !=
    AndOp,       // &&
    OrOp,        // ||
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=
    AddAssign,   // +=
    SubAssign,   // -=
    LeftAssign,  // <<=
    RightAssign, // >>=
    AndAssign,   // &=
    XorAssign,   // ^=
    OrAssign,    // |=
    // TODO: this should be done when we found this is a typedef name,
    //       typedef LL int, then LL is typedef_name
    TypedefName,
    ELLIPSIS,                    // ...
    EnumerationConstant(String), // TODO: add check
    LineTerminator,
    EOF,

    TYPEDEF,
    EXTERN,
    STATIC,
    AUTO,
    REGISTER,
    INLINE,
    CONST,
    RESTRICT,
    VOLATILE,
    BOOL,
    CHAR,
    SHORT,
    INT,
    LONG,
    SIGNED,
    UNSIGNED,
    FLOAT,
    DOUBLE,
    VOID,
    COMPLEX,
    IMAGINARY,
    STRUCT,
    UNION,
    ENUM,
    CASE,
    DEFAULT,
    IF,
    ELSE,
    SWITCH,
    WHILE,
    DO,
    FOR,
    GOTO,
    CONTINUE,
    BREAK,
    RETURN,
    ALIGNAS,
    ALIGNOF,
    ATOMIC,
    GENERIC,
    NORETURN,
    StaticAssert,
    ThreadLocal,
}

pub const TOKENS: phf::Map<&'static str, Token> = phf_map! {
    "{" => Token::LBrace,
    "}" => Token::RBrace,
    "(" => Token::LParen,
    ")" => Token::RParen,
    "[" => Token::LBracket,
    "]" => Token::RBracket,
    ";" => Token::Semicolon,
    "=" => Token::Assign,
    "<" => Token::Lt,
    ">" => Token::Gt,
    "-" => Token::Minus,
    "~" => Token::Tilde,
    "!" => Token::Exclamation,
    "+" => Token::Plus,
    "*" => Token::Multi,
    ":" => Token::Colon,
    "?" => Token::QuestionMark,
    "," => Token::Comma,
    "." =>  Token::Dot,
    "&" => Token::SingleAnd,
    "|" => Token::InclusiveOr,
    "^" => Token::ExclusiveOr,
    "%" => Token::Mod,
    "->" => Token::PtrOp,
    "++" => Token::IncOp,
    "--" => Token::DecOp,
    "<<" => Token::LeftOp,
    ">>" => Token::RightOp,
    "<=" => Token::LeOp,
    ">=" => Token::GeOp,
    "==" => Token::EqOp,
    "!=" => Token::NeOp,
    "&&" => Token::AndOp,
    "||" => Token::OrOp,
    "*=" => Token::MulAssign,
    "%=" => Token::ModAssign,
    "+=" => Token::AddAssign,
    "-=" => Token::SubAssign,
    "<<=" => Token::LeftAssign,
    ">>=" => Token::RightAssign,
    "&=" => Token::AndAssign,
    "^=" => Token::XorAssign,
    "|=" => Token::OrAssign,
};
