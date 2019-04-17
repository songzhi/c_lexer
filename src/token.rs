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
    Splash,       // /
    Colon,        // :
    QuestionMark, // ?
    Comma,        // ,
    Dot,          // .
    SingleAnd,    // &
    InclusiveOr,  // |
    ExclusiveOr,  // ^
    Mod,          // %
    IDENTIFIER(String),
    IConstant(i64),
    FConstant(f64),
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