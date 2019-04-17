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
    //    Char(StateMachine<Char>),
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

impl StateMachineWrapper {
    #[inline]
    fn step(self, e: Equivalence) -> Self {
        match (self, e) {
            (StateMachineWrapper::InputElementDiv(s), Equivalence::LineTerminator) =>
                StateMachineWrapper::LineTerminator(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::White) =>
                StateMachineWrapper::WhiteSpace(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Slash) =>
                StateMachineWrapper::Slash(s.into()), // /
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Dot) =>
                StateMachineWrapper::DotPart(s.into()),
            (StateMachineWrapper::Slash(s), Equivalence::Assign) =>
                StateMachineWrapper::SlashAcc(s.into()), // /
            (StateMachineWrapper::Slash(s), Equivalence::Slash) =>
                StateMachineWrapper::SingleLineComment(s.into()), // //
            (StateMachineWrapper::Slash(s), Equivalence::Multi) =>
                StateMachineWrapper::MultiLineComment(s.into()),
            (StateMachineWrapper::Slash(s), _) =>
                StateMachineWrapper::SlashAcc(s.into()),

            // single line comment
            (StateMachineWrapper::SingleLineComment(s), Equivalence::LineTerminator) =>
                StateMachineWrapper::SingleLineCommentAcc(s.into()),
            (StateMachineWrapper::SingleLineComment(s), _) =>
                StateMachineWrapper::SingleLineComment(s),

            // Multiline comment
            (StateMachineWrapper::MultiLineComment(s), Equivalence::Multi) =>
                StateMachineWrapper::MultiLineCommentMulti(s.into()),
            (StateMachineWrapper::MultiLineCommentMulti(s), Equivalence::Multi) =>
                StateMachineWrapper::MultiLineCommentMulti(s),
            (StateMachineWrapper::MultiLineCommentMulti(s), Equivalence::Slash) =>
                StateMachineWrapper::MultiLineCommentAcc(s.into()),

            (StateMachineWrapper::MultiLineComment(s), _) =>
                StateMachineWrapper::MultiLineComment(s),
            (StateMachineWrapper::MultiLineCommentMulti(s), _) =>
                StateMachineWrapper::MultiLineComment(s.into()),
            // Identifier
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Letter) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::A) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::B) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::C) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::D) =>
                StateMachineWrapper::Identifier(s.into()),

            (StateMachineWrapper::InputElementDiv(s), Equivalence::E) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::F) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::O) =>
                StateMachineWrapper::Identifier(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::X) =>
                StateMachineWrapper::Identifier(s.into()),

            // Punctuator
            (StateMachineWrapper::LBrace(s), _) =>
                StateMachineWrapper::LBraceAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::LBrace) =>
                StateMachineWrapper::LBrace(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::RBrace) =>
                StateMachineWrapper::RBrace(s.into()),
            (StateMachineWrapper::RBrace(s), _) =>
                StateMachineWrapper::RBraceAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::LParen) =>
                StateMachineWrapper::LParen(s.into()),
            (StateMachineWrapper::LParen(s), _) =>
                StateMachineWrapper::LParenAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::RParen) =>
                StateMachineWrapper::RParen(s.into()),
            (StateMachineWrapper::RParen(s), _) =>
                StateMachineWrapper::RParenAcc(s.into()),

            (StateMachineWrapper::InputElementDiv(s), Equivalence::LBracket) =>
                StateMachineWrapper::LBracket(s.into()),
            (StateMachineWrapper::LBracket(s), _) =>
                StateMachineWrapper::LBracketAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::RBracket) =>
                StateMachineWrapper::RBracket(s.into()),

            (StateMachineWrapper::RBracket(s), _) =>
                StateMachineWrapper::RBracketAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Semicolon) =>
                StateMachineWrapper::Semicolon(s.into()),
            (StateMachineWrapper::Semicolon(s), _) =>
                StateMachineWrapper::SemicolonAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Comma) =>
                StateMachineWrapper::Comma(s.into()),
            (StateMachineWrapper::Comma(s), _) =>
                StateMachineWrapper::CommaAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Colon) =>
                StateMachineWrapper::Colon(s.into()),
            (StateMachineWrapper::Colon(s), _) =>
                StateMachineWrapper::ColonAcc(s.into()),

            (StateMachineWrapper::InputElementDiv(s), Equivalence::Questionmark) =>
                StateMachineWrapper::QuestionMark(s.into()),
            (StateMachineWrapper::QuestionMark(s), _) =>
                StateMachineWrapper::QuestionMarkAcc(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Tilde) =>
                StateMachineWrapper::Tilde(s.into()),

            (StateMachineWrapper::Tilde(s), _) =>
                StateMachineWrapper::TildeAcc(s.into()),

            // Less than
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Lt) =>
                StateMachineWrapper::Lt(s.into()),
            (StateMachineWrapper::Lt(s), Equivalence::Assign) =>
                StateMachineWrapper::Lt(s),
            (StateMachineWrapper::Lt(s), Equivalence::Lt) =>
                StateMachineWrapper::Lt(s),
            (StateMachineWrapper::Lt(s), _) =>
                StateMachineWrapper::LtAcc(s.into()),

            // Greater than
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Gt) =>
                StateMachineWrapper::Gt(s.into()),
            (StateMachineWrapper::Gt(s), Equivalence::Assign) =>
                StateMachineWrapper::Gt(s),
            (StateMachineWrapper::Gt(s), Equivalence::Gt) =>
                StateMachineWrapper::Gt(s),
            (StateMachineWrapper::Gt(s), _) =>
                StateMachineWrapper::GtAcc(s.into()),

            // Assign
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Assign) =>
                StateMachineWrapper::Assign(s.into()),
            (StateMachineWrapper::Assign(s), Equivalence::Gt) =>
                StateMachineWrapper::Assign(s),
            (StateMachineWrapper::Assign(s), Equivalence::Assign) =>
                StateMachineWrapper::Assign(s),
            (StateMachineWrapper::Assign(s), _) =>
                StateMachineWrapper::AssignAcc(s.into()),

            // Exclamation
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Exclamation) =>
                StateMachineWrapper::Exclamation(s.into()),
            (StateMachineWrapper::Exclamation(s), Equivalence::Assign) =>
                StateMachineWrapper::Exclamation(s),
            (StateMachineWrapper::Exclamation(s), _) =>
                StateMachineWrapper::ExclamationAcc(s.into()),

            // Plus
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Plus) =>
                StateMachineWrapper::Plus(s.into()),

            (StateMachineWrapper::Plus(s), Equivalence::Plus) =>
                StateMachineWrapper::Plus(s),

            (StateMachineWrapper::Plus(s), Equivalence::Assign) =>
                StateMachineWrapper::Plus(s),

            (StateMachineWrapper::Plus(s), _) =>
                StateMachineWrapper::PlusAcc(s.into()),

            // Minus
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Minus) =>
                StateMachineWrapper::Minus(s.into()),
            (StateMachineWrapper::Minus(s), Equivalence::Minus) =>
                StateMachineWrapper::Minus(s),
            (StateMachineWrapper::Minus(s), Equivalence::Assign) =>
                StateMachineWrapper::Minus(s),
            (StateMachineWrapper::Minus(s), _) =>
                StateMachineWrapper::MinusAcc(s.into()),

            // Multi
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Multi) =>
                StateMachineWrapper::Multi(s.into()),
            (StateMachineWrapper::Multi(s), Equivalence::Multi) =>
                StateMachineWrapper::Multi(s),
            (StateMachineWrapper::Multi(s), Equivalence::Assign) =>
                StateMachineWrapper::Multi(s),
            (StateMachineWrapper::Multi(s), _) =>
                StateMachineWrapper::MultiAcc(s.into()),

            // Mod
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Mod) =>
                StateMachineWrapper::Mod(s.into()),
            (StateMachineWrapper::Mod(s), Equivalence::Mod) =>
                StateMachineWrapper::Mod(s),
            (StateMachineWrapper::Mod(s), Equivalence::Assign) =>
                StateMachineWrapper::Mod(s),
            (StateMachineWrapper::Mod(s), _) =>
                StateMachineWrapper::ModAcc(s.into()),

            // and
            (StateMachineWrapper::InputElementDiv(s), Equivalence::And) =>
                StateMachineWrapper::And(s.into()),
            (StateMachineWrapper::And(s), Equivalence::And) => StateMachineWrapper::And(s),
            (StateMachineWrapper::And(s), Equivalence::Assign) =>
                StateMachineWrapper::And(s),
            (StateMachineWrapper::And(s), _) => StateMachineWrapper::AndAcc(s.into()),
            // or
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Or) =>
                StateMachineWrapper::Or(s.into()),

            (StateMachineWrapper::Or(s), Equivalence::Or) => StateMachineWrapper::Or(s),
            (StateMachineWrapper::Or(s), Equivalence::Assign) => StateMachineWrapper::Or(s),
            (StateMachineWrapper::Or(s), _) => StateMachineWrapper::OrAcc(s.into()),
            // ExclusiveOr
            (StateMachineWrapper::InputElementDiv(s), Equivalence::ExclusiveOr) =>
                StateMachineWrapper::ExclusiveOr(s.into()),
            (StateMachineWrapper::ExclusiveOr(s), Equivalence::ExclusiveOr) =>
                StateMachineWrapper::ExclusiveOr(s),
            (StateMachineWrapper::ExclusiveOr(s), Equivalence::Assign) =>
                StateMachineWrapper::ExclusiveOr(s),
            (StateMachineWrapper::ExclusiveOr(s), _) =>
                StateMachineWrapper::ExclusiveOrAcc(s.into()),

            // string
            (StateMachineWrapper::InputElementDiv(s), Equivalence::String) =>
                StateMachineWrapper::String(s.into()),
            // numbers
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Zero) =>
                StateMachineWrapper::SawZero(s.into()),

            (StateMachineWrapper::InputElementDiv(s), Equivalence::One) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Two) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Three) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Four) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Five) =>
                StateMachineWrapper::Decimal(s.into()),

            (StateMachineWrapper::InputElementDiv(s), Equivalence::Six) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::Seven) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::InputElementDiv(s), Equivalence::EightNine) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::B) =>
                StateMachineWrapper::Binary(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::O) =>
                StateMachineWrapper::Octal(s.into()),

            (StateMachineWrapper::SawZero(s), Equivalence::X) => StateMachineWrapper::Hex(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::One) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::Two) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::Three) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::Four) =>
                StateMachineWrapper::Decimal(s.into()),

            (StateMachineWrapper::SawZero(s), Equivalence::Five) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::Six) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::Seven) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), Equivalence::EightNine) =>
                StateMachineWrapper::Decimal(s.into()),
            (StateMachineWrapper::SawZero(s), _) =>
                StateMachineWrapper::DecimalAcc(s.into()),
            (StateMachineWrapper::Decimal(s), Equivalence::Zero) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::One) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Two) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Three) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Four) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Five) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Six) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Seven) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::EightNine) =>
                StateMachineWrapper::Decimal(s),
            (StateMachineWrapper::Decimal(s), Equivalence::Dot) =>
                StateMachineWrapper::DecimalDigits(s.into()),
            (StateMachineWrapper::Decimal(s), Equivalence::E) =>
                StateMachineWrapper::DecimalExponent(s.into()),
            (StateMachineWrapper::Decimal(s), _) =>
                StateMachineWrapper::DecimalAcc(s.into()),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Zero) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::One) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Two) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Three) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Four) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Five) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Six) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::Seven) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::EightNine) =>
                StateMachineWrapper::DecimalDigits(s),
            (StateMachineWrapper::DecimalDigits(s), Equivalence::E) =>
                StateMachineWrapper::DecimalExponent(s.into()),
            (StateMachineWrapper::DecimalDigits(s), _) =>
                StateMachineWrapper::DecimalDigitsAcc(s.into()),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Zero) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::One) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Two) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Three) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Four) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Five) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Six) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Seven) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::EightNine) =>
                StateMachineWrapper::DecimalExponent(s),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Minus) =>
                StateMachineWrapper::DecimalExponentSigned(s.into()),
            (StateMachineWrapper::DecimalExponent(s), Equivalence::Plus) =>
                StateMachineWrapper::DecimalExponentSigned(s.into()),
            (StateMachineWrapper::DecimalExponent(s), _) =>
                StateMachineWrapper::DecimalExponentAcc(s.into()),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Zero) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::One) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Two) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Three) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Four) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Five) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Six) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::Seven) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), Equivalence::EightNine) =>
                StateMachineWrapper::DecimalExponentSigned(s),
            (StateMachineWrapper::DecimalExponentSigned(s), _) =>
                StateMachineWrapper::DecimalExponentSignedAcc(s.into()),
            (StateMachineWrapper::Octal(s), Equivalence::Zero) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::One) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Two) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Three) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Four) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Five) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Six) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), Equivalence::Seven) =>
                StateMachineWrapper::Octal(s),
            (StateMachineWrapper::Octal(s), _) =>
                StateMachineWrapper::OctalAcc(s.into()),
            (StateMachineWrapper::Hex(s), Equivalence::Zero) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::One) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Two) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Three) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Four) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Five) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Six) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::Seven) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::EightNine) =>
                StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::A) =>
                StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::B) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::C) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::D) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::E) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), Equivalence::F) => StateMachineWrapper::Hex(s),
            (StateMachineWrapper::Hex(s), _) => StateMachineWrapper::HexAcc(s.into()),
            (StateMachineWrapper::Binary(s), Equivalence::One) => StateMachineWrapper::Binary(s),
            (StateMachineWrapper::Binary(s), Equivalence::Zero) => StateMachineWrapper::Binary(s),
            (StateMachineWrapper::Binary(s), _) => StateMachineWrapper::BinaryAcc(s.into()),
            (_, Equivalence::HELL) => StateMachineWrapper::InputElementDiv(StateMachine::<InputElementDiv>::new()),
            a => unreachable!("Invalid state:  :? ", a)
        }
    }

    #[inline]
    fn is_final(&self) -> bool {
        match self {
            StateMachineWrapper::LineTerminator(n) => n.is_final(),
            StateMachineWrapper::WhiteSpace(n) => n.is_final(),
            StateMachineWrapper::SingleLineCommentAcc(n) => n.is_final(),
            StateMachineWrapper::MultiLineCommentAcc(n) => n.is_final(),
            StateMachineWrapper::LBrace(n) => n.is_final(),
            StateMachineWrapper::LBraceAcc(n) => n.is_final(),
            StateMachineWrapper::LParen(n) => n.is_final(),
            StateMachineWrapper::RParen(n) => n.is_final(),
            StateMachineWrapper::LBracket(n) => n.is_final(),
            StateMachineWrapper::RBracket(n) => n.is_final(),
            StateMachineWrapper::LParenAcc(n) => n.is_final(),
            StateMachineWrapper::RParenAcc(n) => n.is_final(),
            StateMachineWrapper::LBracketAcc(n) => n.is_final(),
            StateMachineWrapper::RBracketAcc(n) => n.is_final(),
            StateMachineWrapper::DotPart(n) => n.is_final(),
            StateMachineWrapper::Comma(n) => n.is_final(),
            StateMachineWrapper::CommaAcc(n) => n.is_final(),
            StateMachineWrapper::Semicolon(n) => n.is_final(),
            StateMachineWrapper::SemicolonAcc(n) => n.is_final(),
            StateMachineWrapper::LtAcc(n) => n.is_final(),
            StateMachineWrapper::GtAcc(n) => n.is_final(),
            StateMachineWrapper::AssignAcc(n) => n.is_final(),
            StateMachineWrapper::ExclamationAcc(n) => n.is_final(),
            StateMachineWrapper::PlusAcc(n) => n.is_final(),
            StateMachineWrapper::MinusAcc(n) => n.is_final(),
            StateMachineWrapper::MultiAcc(n) => n.is_final(),
            StateMachineWrapper::ModAcc(n) => n.is_final(),
            StateMachineWrapper::AndAcc(n) => n.is_final(),
            StateMachineWrapper::OrAcc(n) => n.is_final(),
            StateMachineWrapper::Tilde(n) => n.is_final(),
            StateMachineWrapper::TildeAcc(n) => n.is_final(),
            StateMachineWrapper::QuestionMark(n) => n.is_final(),
            StateMachineWrapper::QuestionMarkAcc(n) => n.is_final(),
            StateMachineWrapper::ColonAcc(n) => n.is_final(),
            StateMachineWrapper::ExclusiveOrAcc(n) => n.is_final(),
            StateMachineWrapper::String(n) => n.is_final(),
//            StateMachineWrapper::Char(n) => n.is_final(),
            StateMachineWrapper::BinaryAcc(n) => n.is_final(),
            StateMachineWrapper::OctalAcc(n) => n.is_final(),
            StateMachineWrapper::HexAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalDigitsAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentSignedAcc(n) => n.is_final(),
            StateMachineWrapper::Identifier(n) => n.is_final(),
            StateMachineWrapper::SlashAcc(n) => n.is_final(),
            StateMachineWrapper::RBrace(n) => n.is_final(),
            StateMachineWrapper::RBraceAcc(n) => n.is_final(),

            StateMachineWrapper::InputElementDiv(n) => n.is_final(),
            StateMachineWrapper::Slash(n) => n.is_final(),
            StateMachineWrapper::SingleLineComment(n) => n.is_final(),
            StateMachineWrapper::MultiLineComment(n) => n.is_final(),
            StateMachineWrapper::Colon(n) => n.is_final(),
            StateMachineWrapper::MultiLineCommentStar(n) => n.is_final(),
            StateMachineWrapper::Lt(n) => n.is_final(),
            StateMachineWrapper::Gt(n) => n.is_final(),
            StateMachineWrapper::Assign(n) => n.is_final(),
            StateMachineWrapper::Exclamation(n) => n.is_final(),
            StateMachineWrapper::Plus(n) => n.is_final(),
            StateMachineWrapper::Minus(n) => n.is_final(),
            StateMachineWrapper::Multi(n) => n.is_final(),
            StateMachineWrapper::Mod(n) => n.is_final(),
            StateMachineWrapper::And(n) => n.is_final(),
            StateMachineWrapper::Or(n) => n.is_final(),
            StateMachineWrapper::ExclusiveOr(n) => n.is_final(),
            StateMachineWrapper::SawZero(n) => n.is_final(),
            StateMachineWrapper::Binary(n) => n.is_final(),
            StateMachineWrapper::Octal(n) => n.is_final(),
            StateMachineWrapper::Hex(n) => n.is_final(),
            StateMachineWrapper::Decimal(n) => n.is_final(),
            StateMachineWrapper::DecimalDigits(n) => n.is_final(),
            StateMachineWrapper::DecimalExponent(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentSigned(n) => n.is_final(),
        }
    }
}