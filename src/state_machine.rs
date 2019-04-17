use crate::{
    equivalence::{Equivalence, EQUIVALENCE_CLASS},
    error::Error,
    identifier,
    number::{parse_exponent, parse_number, parse_number_decimal, parse_number_radix},
    state::*,
    string,
    token::Token,
};
use std::str;

#[derive(Debug)]
struct StateMachine<S: State> {
    state: S,
}

impl<S: State> StateMachine<S> {
    fn new() -> StateMachine<InputElementDiv> {
        StateMachine {
            state: InputElementDiv,
        }
    }

    fn is_final(&self) -> bool {
        self.state.is_final()
    }
}

#[derive(Debug)]
enum StateMachineWrapper {
    LineTerminator(StateMachine<LineTerminator>),
    WhiteSpace(StateMachine<WhiteSpace>),
    SingleLineCommentAcc(StateMachine<SingleLineCommentAcc>),
    MultiLineCommentAcc(StateMachine<MultiLineCommentAcc>),
    DotPart(StateMachine<DotPart>),
    Comma(StateMachine<Comma>),
    CommaAcc(StateMachine<CommaAcc>),
    Semicolon(StateMachine<Semicolon>),
    SemicolonAcc(StateMachine<SemicolonAcc>),
    LtAcc(StateMachine<LtAcc>),
    GtAcc(StateMachine<GtAcc>),
    AssignAcc(StateMachine<AssignAcc>),
    ExclamationAcc(StateMachine<ExclamationAcc>),
    PlusAcc(StateMachine<PlusAcc>),
    MinusAcc(StateMachine<MinusAcc>),
    MultiAcc(StateMachine<MultiAcc>),
    ModAcc(StateMachine<ModAcc>),
    AndAcc(StateMachine<AndAcc>),
    OrAcc(StateMachine<OrAcc>),
    Tilde(StateMachine<Tilde>),
    TildeAcc(StateMachine<TildeAcc>),
    QuestionMark(StateMachine<QuestionMark>),
    QuestionMarkAcc(StateMachine<QuestionMarkAcc>),
    ColonAcc(StateMachine<ColonAcc>),
    ExclusiveOrAcc(StateMachine<ExclusiveOrAcc>),
    String(StateMachine<String>),
    Char(StateMachine<Char>),
    BinaryAcc(StateMachine<BinaryAcc>),
    OctalAcc(StateMachine<OctalAcc>),
    HexAcc(StateMachine<HexAcc>),
    DecimalAcc(StateMachine<DecimalAcc>),
    DecimalDigitsAcc(StateMachine<DecimalDigitsAcc>),
    DecimalExponentAcc(StateMachine<DecimalExponentAcc>),
    DecimalExponentSignedAcc(StateMachine<DecimalExponentSignedAcc>),
    Identifier(StateMachine<Identifier>),
    SlashAcc(StateMachine<SlashAcc>),
    LCurlyAcc(StateMachine<LBraceAcc>),
    RCurlyAcc(StateMachine<RBraceAcc>),
    LRoundAcc(StateMachine<LParenAcc>),
    RRoundAcc(StateMachine<RParenAcc>),
    LSquareAcc(StateMachine<LBracketAcc>),
    RSquareAcc(StateMachine<RBracketAcc>),

    InputElementDiv(StateMachine<InputElementDiv>),
    Slash(StateMachine<Slash>),
    SingleLineComment(StateMachine<SingleLineComment>),
    MultiLineComment(StateMachine<MultiLineComment>),
    MultiLineCommentStar(StateMachine<MultiLineCommentStar>),
    Lt(StateMachine<Lt>),
    Gt(StateMachine<Gt>),
    Colon(StateMachine<Colon>),
    LBrace(StateMachine<LBrace>),
    LParen(StateMachine<LParen>),
    RParen(StateMachine<RParen>),
    LBracket(StateMachine<LBracket>),
    RBracket(StateMachine<RBracket>),
    RBrace(StateMachine<RBrace>),
    Assign(StateMachine<Assign>),
    Exclamation(StateMachine<Exclamation>),
    Plus(StateMachine<Plus>),
    Minus(StateMachine<Minus>),
    Multi(StateMachine<Multi>),
    Mod(StateMachine<Mod>),
    And(StateMachine<And>),
    Or(StateMachine<Or>),
    ExclusiveOr(StateMachine<ExclusiveOr>),
    SawZero(StateMachine<SawZero>),
    Binary(StateMachine<Binary>),
    Octal(StateMachine<Octal>),
    Hex(StateMachine<Hex>),
    Decimal(StateMachine<Decimal>),
    DecimalDigits(StateMachine<DecimalDigits>),
    DecimalExponent(StateMachine<DecimalExponent>),
    DecimalExponentSigned(StateMachine<DecimalExponentSigned>),
}