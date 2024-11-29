pub mod clang {
    pub mod tok {

        //
        //          Language keywords.
        //
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum TokenKind {
            UNKNOWN,            // Not a token.
            EOF,                // End of file.
            EOD,                // End of preprocessing directive (end of line inside a directive,

            CodeCompletion,     // Code completion marker

            Comment,            // Comment (only in -E -C[C] mode,

            Identifier,         // abcde123
            RawIdentifier,      // Used only in raw lexing mode.

            NumericConstant,    // 0x123

            BinaryData,         // Directly holds numerical value. Used to process C23 #embed

            CharConstant,       // 'a'
            WideCharConstant,   // L'b'

            Utf8CharConstant,   // u8'a'

            Utf16CharConstant,  // u'a'
            Utf32CharConstant,  // U'a'

            StringLiteral,      // "foo"
            WideStringLiteral,  // L"foo"

            HeaderName,         // <foo>, or "foo" lexed as a header-name

            Utf8StringLiteral,  // u8"foo"
            Utf16StringLiteral, // u"foo"
            Utf32StringLiteral  // U"foo"
        }

        //
        //          Preprocessor keywords.
        //
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum PPKeywordKind {
            NotKeyword,

            If,
            Ifdef,
            Ifndef,
            Elif,
            Elifdef,
            Elifndef,
            Else,
            Endif,
            Defined,

            Include,
            IncludeMacros,

            Define,
            Undef,

            Line,
            Error,

            Pragma,
            Embed,

            Import,
            IncludeNext,
            Warning,
            Ident,
            Sccs,
            Assert,
            Unassert,

            PublicMacro,
            PrivateMacro
        }

        //
        //          Preprocessor keywords.
        //
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum ObjCKeywordKind {
            NotKeyword,
            Class,
            CompatibilityAlias,
            Defs,
            Encode,
            End,
            Implementation,
            Interface,
            Private,
            Protected,
            Protocol,
            Public,
            Selector,
            Throw,
            Try,
            Catch,
            Finally,
            Synchronized,
            Autoreleasepool,

            Property,
            Package,
            Required,
            Optional,
            Synthesize,
            Dynamic,
            Import,
            Available,
        }

        //
        //          Notable identifiers.
        //
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum NotableIdentifierKind {
            NotNotable,
            FILE,
            JmpBuf,
            SigjmpBuf,
            UcontextT,
            FloatT,
            DoubleT,
        }

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum OnOffSwitch {
            OosON,
            OosOFF,
            OosDEFAULT
        }

        const TOK_NAMES: [&str; 198] = [
            "unknown", "eof", "eod", "code_completion", "comment", "identifier", "raw_identifier", "numeric_constant", "binary_data", "char_constant", "wide_char_constant", "utf8_char_constant", "utf16_char_constant", "utf32_char_constant", "string_literal", "wide_string_literal", "header_name", "utf8_string_literal", "utf16_string_literal", "utf32_string_literal", "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else", "enum", "extern", "float", "for", "goto", "if", "int", "_ExtInt", "_BitInt", "long", "register", "return", "short", "signed", "static", "struct", "switch", "typedef", "union", "unsigned", "void", "volatile", "while", "_Alignas", "_Alignof", "_Atomic", "_Bool", "_Complex", "_Generic", "_Imaginary", "_Noreturn", "_Static_assert", "_Thread_local", "__func__", "__objc_yes", "__objc_no", "asm", "bool", "catch", "class", "const_cast", "delete", "dynamic_cast", "explicit", "export", "false", "friend", "mutable", "namespace", "new", "operator", "private", "protected", "public", "reinterpret_cast", "static_cast", "template", "this", "throw", "true", "try", "typename", "typeid", "using", "virtual", "wchar_t", "_Accum", "_Fract", "_Sat", "_Decimal32", "_Decimal64", "_Decimal128", "__null", "__attribute", "__builtin_choose_expr", "__builtin_offsetof", "__builtin_FILE", "__builtin_FILE_NAME", "__builtin_FUNCTION", "__builtin_FUNCSIG", "__builtin_LINE", "__builtin_COLUMN", "__builtin_source_location", "__builtin_va_arg", "__extension__", "__float128", "__ibm128", "__imag", "__int128", "__label__", "__real", "__thread", "__FUNCTION__", "__PRETTY_FUNCTION__", "__auto_type", "__FUNCDNAME__", "__FUNCSIG__", "L__FUNCTION__", "L__FUNCSIG__", "__private_extern__", "__module_private__", "__declspec", "__cdecl", "__stdcall", "__fastcall", "__thiscall", "__regcall", "__vectorcall", "__forceinline", "__unaligned", "__super", "__global", "__local", "__constant", "__private", "__generic", "__kernel", "__read_only", "__write_only", "__read_write", "__builtin_astype", "pipe", "addrspace_cast", "__noinline__", "cbuffer", "tbuffer", "groupshared", "in", "inout", "out", "__pascal", "__vector", "__pixel", "__bool", "__bf16", "half", "__bridge", "__bridge_transfer", "__bridge_retained", "__bridge_retain", "__covariant", "__contravariant", "__kindof", "_Nonnull", "_Nullable", "_Nullable_result", "_Null_unspecified", "__funcref", "__ptr64", "__ptr32", "__sptr", "__uptr", "__w64", "__uuidof", "__try", "__finally", "__leave", "__int64", "__if_exists", "__if_not_exists", "__single_inheritance", "__multiple_inheritance", "__virtual_inheritance", "__interface", "__builtin_convertvector", "__builtin_bit_cast", "__builtin_available", "__builtin_sycl_unique_stable_name",
        ];

        /// Determines the name of a token as used within the front end.
        ///
        /// The name of a token will be an internal name (such as "l_square")
        /// and should not be used as part of diagnostic messages.
        pub fn get_token_name(k: TokenKind) -> &'static str {
            let kind_idx = k as usize;

            if kind_idx < TOK_NAMES.len() {
                TOK_NAMES[kind_idx]
            } else {
                unreachable!("unknown TokenKind: {:?}", k)
            }
        }

        /// Determines the spelling of simple punctuation tokens like
        /// '!' or '%', and returns NULL for literal and annotation tokens.
        ///
        /// This routine only retrieves the "simple" spelling of the token,
        /// and will not produce any alternative spellings (e.g., a
        /// digraph). For the actual spelling of a given Token, use
        /// Preprocessor::getSpelling().
        pub fn get_punctuator_spelling(Kind: TokenKind) -> String {

        }

        /// Determines the spelling of simple keyword and contextual keyword
        /// tokens like 'int' and 'dynamic_cast'. Returns NULL for other token kinds.
        pub fn get_keyword_spelling(Kind: TokenKind) -> String {

        }

        /// Returns the spelling of preprocessor keywords, such as "else".
        pub fn get_pp_keyword_spelling(Kind: PPKeywordKind) -> String {

        }

        pub fn is_any_identifier(t: TokenKind) -> bool {
            t == TokenKind::Identifier || t == TokenKind::RawIdentifier
        }
    }
}
