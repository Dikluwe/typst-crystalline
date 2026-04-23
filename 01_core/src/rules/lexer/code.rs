//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lexer/mod.md
//! @prompt-hash 91498f0f
//! @layer L1
//! @updated 2026-04-23
//!
//! Lexer do Typst — modo `Code`. Extraído de `lexer/mod.rs` no
//! Passo 96.9 conforme ADR-0037.

use std::num::IntErrorKind;


use crate::entities::syntax_kind::SyntaxKind;

use super::{is_id_continue, is_id_start, keyword, Lexer};

/// Code.
impl Lexer<'_> {
    pub(super) fn code(&mut self, start: usize, c: char) -> SyntaxKind {
        match c {
            '<' if self.s.at(is_id_continue) => self.label(),
            '0'..='9' => self.number(start, c),
            '.' if self.s.at(char::is_ascii_digit) => self.number(start, c),
            '"' => self.string(),

            '=' if self.s.eat_if('=') => SyntaxKind::EqEq,
            '!' if self.s.eat_if('=') => SyntaxKind::ExclEq,
            '<' if self.s.eat_if('=') => SyntaxKind::LtEq,
            '>' if self.s.eat_if('=') => SyntaxKind::GtEq,
            '+' if self.s.eat_if('=') => SyntaxKind::PlusEq,
            '-' | '\u{2212}' if self.s.eat_if('=') => SyntaxKind::HyphEq,
            '*' if self.s.eat_if('=') => SyntaxKind::StarEq,
            '/' if self.s.eat_if('=') => SyntaxKind::SlashEq,
            '.' if self.s.eat_if('.') => SyntaxKind::Dots,
            '=' if self.s.eat_if('>') => SyntaxKind::Arrow,

            '{' => SyntaxKind::LeftBrace,
            '}' => SyntaxKind::RightBrace,
            '[' => SyntaxKind::LeftBracket,
            ']' => SyntaxKind::RightBracket,
            '(' => SyntaxKind::LeftParen,
            ')' => SyntaxKind::RightParen,
            '$' => SyntaxKind::Dollar,
            ',' => SyntaxKind::Comma,
            ';' => SyntaxKind::Semicolon,
            ':' => SyntaxKind::Colon,
            '.' => SyntaxKind::Dot,
            '+' => SyntaxKind::Plus,
            '-' | '\u{2212}' => SyntaxKind::Minus,
            '*' => SyntaxKind::Star,
            '/' => SyntaxKind::Slash,
            '=' => SyntaxKind::Eq,
            '<' => SyntaxKind::Lt,
            '>' => SyntaxKind::Gt,

            c if is_id_start(c) => self.ident(start),

            c => self.invalid_char_in_code(c),
        }
    }

    /// Error for an invalid character in code, but try to give good hints for
    /// commonly confusing operators.
    fn invalid_char_in_code(&mut self, c: char) -> SyntaxKind {
        let invalid_char = || format!("the character `{c}` is not valid in code");
        let invalid_str = |s: &str| format!("`{s}` is not valid in code");
        match c {
            // Give a custom hint if we immediately follow a hash.
            _ if self.s.scout(-2) == Some('#') => {
                self.error(invalid_char());
                // This is only an accurate hint if we just came from markup or
                // math, but `#!` or `##` in code should be rare enough that
                // it's fine (and the first hash will produce its own error).
                self.hint("the preceding hash is causing this to parse in code mode");
                self.hint("try escaping the preceding hash: `\\#`");
                // The span for these hints isn't great, but it's hard to fix.
            }
            '#' => {
                self.error(invalid_char());
                self.hint("you are already in code mode");
                self.hint("try removing the `#`");
            }
            '&' if self.s.eat_if('&') => {
                self.error(invalid_str("&&"));
                self.hint("in Typst, `and` is used for logical AND");
            }
            '|' if self.s.eat_if('|') => {
                self.error(invalid_str("||"));
                self.hint("in Typst, `or` is used for logical OR");
            }
            '!' => {
                self.error(invalid_char());
                self.hint("in Typst, `not` is used for negation");
                self.hint("or did you mean to write `!=` for not-equal?");
            }
            '~' if self.s.eat_if('=') => {
                self.error(invalid_str("~="));
                self.hint("in Typst, `!=` is used for not-equal");
            }
            _ => {
                self.error(invalid_char());
            }
        }
        SyntaxKind::Error
    }

    fn ident(&mut self, start: usize) -> SyntaxKind {
        self.s.eat_while(is_id_continue);
        let ident = self.s.from(start);

        let prev = self.s.get(0..start);
        if !prev.ends_with(['.', '@']) || prev.ends_with("..") {
            if let Some(keyword) = keyword(ident) {
                return keyword;
            }
        }

        if ident == "_" { SyntaxKind::Underscore } else { SyntaxKind::Ident }
    }

    fn number(&mut self, start: usize, first_c: char) -> SyntaxKind {
        // Handle alternative integer bases.
        let base = match first_c {
            '0' if self.s.eat_if('b') => 2,
            '0' if self.s.eat_if('o') => 8,
            '0' if self.s.eat_if('x') => 16,
            _ => 10,
        };

        // Read the initial digits.
        if base == 16 {
            self.s.eat_while(char::is_ascii_alphanumeric);
        } else {
            self.s.eat_while(char::is_ascii_digit);
        }

        // Read floating point digits and exponents.
        let mut is_float = false;
        if base == 10 {
            // Read digits following a dot. Make sure not to confuse a spread
            // operator or a method call for the decimal separator.
            if first_c == '.' {
                is_float = true; // We already ate the trailing digits above.
            } else if !self.s.at("..")
                && !self.s.scout(1).is_some_and(is_id_start)
                && self.s.eat_if('.')
            {
                is_float = true;
                self.s.eat_while(char::is_ascii_digit);
            }

            // Read the exponent.
            if !self.s.at("em") && self.s.eat_if(['e', 'E']) {
                is_float = true;
                self.s.eat_if(['+', '-']);
                self.s.eat_while(char::is_ascii_digit);
            }
        }

        let number = self.s.from(start);
        let suffix = self.s.eat_while(|c: char| c.is_ascii_alphanumeric() || c == '%');

        // Parse large integer literals as floats
        if base == 10 && !is_float {
            if let Err(e) = i64::from_str_radix(number, base) {
                if matches!(e.kind(), IntErrorKind::PosOverflow | IntErrorKind::NegOverflow)
                    && number.parse::<f64>().is_ok()
                {
                    is_float = true;
                }
            }
        }

        let mut suffix_result = match suffix {
            "" => Ok(None),
            "pt" | "mm" | "cm" | "in" | "deg" | "rad" | "em" | "fr" | "%" => Ok(Some(())),
            _ => Err(format!("invalid number suffix: {suffix}")),
        };

        let number_result = if is_float && number.parse::<f64>().is_err() {
            // The only invalid case should be when a float lacks digits after
            // the exponent: e.g. `1.2e`, `2.3E-`, or `1EM`.
            Err(format!("invalid floating point number: {number}"))
        } else if base == 10 {
            Ok(())
        } else {
            let name = match base {
                2 => "binary",
                8 => "octal",
                16 => "hexadecimal",
                _ => unreachable!(),
            };
            // The index `[2..]` skips the leading `0b`/`0o`/`0x`.
            match i64::from_str_radix(&number[2..], base) {
                Ok(_) if suffix.is_empty() => Ok(()),
                Ok(value) => {
                    if suffix_result.is_ok() {
                        suffix_result = Err(format!(
                            "try using a decimal number: {value}{suffix}"
                        ));
                    }
                    Err(format!("{name} numbers cannot have a suffix"))
                }
                Err(_) => Err(format!("invalid {name} number: {number}")),
            }
        };

        // Return our number or write an error with helpful hints.
        match (number_result, suffix_result) {
            // Valid numbers :D
            (Ok(()), Ok(None)) if is_float => SyntaxKind::Float,
            (Ok(()), Ok(None)) => SyntaxKind::Int,
            (Ok(()), Ok(Some(()))) => SyntaxKind::Numeric,
            // Invalid numbers :(
            (Err(number_err), Err(suffix_err)) => {
                let error = self.error(number_err);
                self.hint(suffix_err);
                error
            }
            (Ok(()), Err(msg)) | (Err(msg), Ok(_)) => self.error(msg),
        }
    }

    pub(super) fn string(&mut self) -> SyntaxKind {
        let mut escaped = false;
        self.s.eat_until(|c| {
            let stop = c == '"' && !escaped;
            escaped = c == '\\' && !escaped;
            stop
        });

        if !self.s.eat_if('"') {
            return self.error("unclosed string");
        }

        SyntaxKind::Str
    }
}



#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
