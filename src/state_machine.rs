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
    LBraceAcc(StateMachine<LBraceAcc>),
    RBraceAcc(StateMachine<RBraceAcc>),
    LParenAcc(StateMachine<LParenAcc>),
    RParenAcc(StateMachine<RParenAcc>),
    LBracketAcc(StateMachine<LBracketAcc>),
    RBracketAcc(StateMachine<RBracketAcc>),

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

Edge!(InputElementDiv, LineTerminator);
Edge!(InputElementDiv, WhiteSpace);
Edge!(InputElementDiv, Slash);
Edge!(InputElementDiv, DotPart);
Edge!(Slash, SlashAcc);
Edge!(Slash, SingleLineComment);
Edge!(Slash, MultiLineComment);
Edge!(SingleLineComment, SingleLineCommentAcc);
Edge!(MultiLineComment, MultiLineCommentStar);
Edge!(MultiLineCommentStar, MultiLineCommentAcc);
Edge!(InputElementDiv, Identifier);
Edge!(InputElementDiv, LBrace);
Edge!(LBrace, LBraceAcc);
Edge!(InputElementDiv, RBrace);
Edge!(RBrace, RBraceAcc);
Edge!(InputElementDiv, LParen);
Edge!(LParen, LParenAcc);
Edge!(InputElementDiv, RParen);
Edge!(RParen, RParenAcc);
Edge!(InputElementDiv, LBracket);
Edge!(LBracket, LBracketAcc);
Edge!(InputElementDiv, RBracket);
Edge!(RBracket, RBracketAcc);
Edge!(InputElementDiv, Semicolon);
Edge!(Semicolon, SemicolonAcc);
Edge!(InputElementDiv, Comma);
Edge!(Comma, CommaAcc);
Edge!(InputElementDiv, Colon);
Edge!(Colon, ColonAcc);
Edge!(InputElementDiv, QuestionMark);
Edge!(QuestionMark, QuestionMarkAcc);
Edge!(InputElementDiv, Tilde);
Edge!(Tilde, TildeAcc);
Edge!(InputElementDiv, Lt);
Edge!(Lt, LtAcc);
Edge!(InputElementDiv, Gt);
Edge!(Gt, GtAcc);
Edge!(InputElementDiv, Assign);
Edge!(Assign, AssignAcc);
Edge!(InputElementDiv, Exclamation);
Edge!(Exclamation, ExclamationAcc);
Edge!(InputElementDiv, Plus);
Edge!(Plus, PlusAcc);
Edge!(InputElementDiv, Multi);
Edge!(Multi, MultiAcc);
Edge!(InputElementDiv, Mod);
Edge!(Mod, ModAcc);
Edge!(InputElementDiv, Minus);
Edge!(Minus, MinusAcc);
Edge!(InputElementDiv, Or);
Edge!(Or, OrAcc);
Edge!(InputElementDiv, ExclusiveOr);
Edge!(ExclusiveOr, ExclusiveOrAcc);
Edge!(InputElementDiv, String);
Edge!(And, AndAcc);
Edge!(InputElementDiv, And);
Edge!(InputElementDiv, SawZero);
Edge!(SawZero, Decimal);
Edge!(SawZero, DecimalAcc);
Edge!(InputElementDiv, Decimal);
Edge!(Decimal, DecimalAcc);
Edge!(Decimal, DecimalExponent);
Edge!(DecimalDigits, DecimalExponent);
Edge!(DecimalDigits, DecimalDigitsAcc);
Edge!(DecimalExponent, DecimalExponentSigned);
Edge!(DecimalExponentSigned, DecimalExponentSignedAcc);
Edge!(DecimalExponent, DecimalExponentAcc);
Edge!(Decimal, DecimalDigits);
Edge!(InputElementDiv, Hex);
Edge!(Hex, HexAcc);
Edge!(SawZero, Hex);
Edge!(InputElementDiv, Binary);
Edge!(Binary, BinaryAcc);
Edge!(SawZero, Binary);
Edge!(InputElementDiv, Octal);
Edge!(Octal, OctalAcc);
Edge!(SawZero, Octal);
Edge!(MultiLineCommentStar, MultiLineComment);
Edge!(LineTerminator, HELL);