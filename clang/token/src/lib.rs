use strum_macros::EnumCount;

// Provides a simple uniform namespace for tokens from all C languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
pub enum TokenKind {
    Unknown, Eof, Eod, CodeCompletion, Comment, Identifier, RawIdentifier,
    NumericConstant, BinaryData, CharConstant, WideCharConstant,
    Utf8CharConstant, Utf16CharConstant, Utf32CharConstant, StringLiteral,
    WideStringLiteral, HeaderName, Utf8StringLiteral, Utf16StringLiteral,
    Utf32StringLiteral,
    // punctuators
    Lsquare, Rsquare, Lparen, Rparen, Lbrace, Rbrace, Period, Ellipsis,
    Amp, Ampamp, Ampequal, Star, Starequal, Plus, Plusplus, Plusequal,
    Minus, Arrow, Minusminus, Minusequal, Tilde, Exclaim, Exclaimequal,
    Slash, Slashequal, Percent, Percentequal, Less, Lessless, Lessequal,
    Lesslessequal, Spaceship, Greater, Greatergreater, Greaterequal,
    Greatergreaterequal, Caret, Caretequal, Pipe, Pipepipe, Pipeequal,
    Question, Colon, Semi, Equal, Equalequal, Comma, Hash, Hashhash,
    Hashat, Periodstar, Arrowstar, Coloncolon, At, Lesslessless,
    Greatergreatergreater,
}

const TOKEN_NAMES: [&str; 76] = [
    "unknown", "eof", "eod", "code_completion", "comment", "identifier", "raw_identifier",
    "numeric_constant", "binary_data", "char_constant", "wide_char_constant", "utf8_char_constant",
    "utf16_char_constant", "utf32_char_constant", "string_literal", "wide_string_literal",
    "header_name", "utf8_string_literal", "utf16_string_literal", "utf32_string_literal",
    // punctuators
    "[", "]", "(", ")", "{", "}", ".", "...", "&", "&&", "&=", "*", "*=", "+", "++",
    "+=", "-", "->", "--", "-=", "~", "!", "!=", "/", "/=", "%", "%=", "<", "<<", "<=",
    "<<=", "<=>", ">", ">>", ">=", ">>=", "^", "^=", "|", "||", "|=", "?", ":", ";",
    "=", "==", ",", "#", "##", "#@", ".*", "->*", "::", "@", "<<<", ">>>",
];

// Provides a namespace for preprocessor keywords which start with a
// '#' at the beginning of the line.
pub enum PPKeywordKind {
    NotKeyword, If, Ifdef, Ifndef, Elif, Elifdef, Elifndef, Else, Endif,
    Defined, Include, IncludeMacros, Define, Undef, Line, Error, Pragma,
    Embed, Import, IncludeNext, Warning, Ident, Sccs, Assert, Unassert,
    PublicMacro, PrivateMacro,
}

// Provides a namespace for Objective-C keywords which start with
// an '@'.
pub enum ObjCKeywordKind {
    NotKeyword, Class, CompatibilityAlias, Defs, Encode, End, Implementation,
    Interface, Private, Protected, Protocol, Public, Selector, Throw,
    Try, Catch, Finally, Synchronized, Autoreleasepool, Property,
    Package, Required, Optional, Synthesize, Dynamic, Import, Available,
}

// Provides a namespace for notable identifers such as float_t and
// double_t.
pub enum NotableIdentifierKind {
    NotNotable, FILE, JmpBuf, SigjmpBuf, Ucontextt, Floatt, Doublet,
}

pub enum OnOffSwitch {
    OosON,
    OosOFF,
    OosDEFAULT
}

/// Determines the name of a token as used within the front end.
///
/// The name of a token will be an internal name (such as "l_square")
/// and should not be used as part of diagnostic messages.
pub fn get_token_name(kind: TokenKind) -> &'static str {
    if (kind as usize) < TOKEN_NAMES.len() {
        TOKEN_NAMES[kind as usize]
    } else {
        TOKEN_NAMES[0]
    }
}
