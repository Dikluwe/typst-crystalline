//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lexer/mod.md
//! @prompt-hash 91498f0f
//! @layer L1
//! @updated 2026-04-23
//!
//! Lexer do Typst — modo `Markup`. Extraído de `lexer/mod.rs` no
//! Passo 96.9 conforme ADR-0037.


use unicode_script::{Script, UnicodeScript};

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::{SyntaxError, SyntaxNode};
use crate::rules::lexer::scanner::Scanner;

use super::{is_id_continue, is_newline,
    is_valid_in_label_literal, link_prefix, split_newlines, Lexer, ScannerExt};

/// Markup.
impl Lexer<'_> {
    pub(super) fn markup(&mut self, start: usize, c: char) -> SyntaxKind {
        match c {
            '\\' => self.backslash(),
            'h' if self.s.eat_if("ttp://") => self.link(),
            'h' if self.s.eat_if("ttps://") => self.link(),
            '<' if self.s.at(is_id_continue) => self.label(),
            '@' if self.s.at(is_id_continue) => self.ref_marker(),

            '.' if self.s.eat_if("..") => SyntaxKind::Shorthand,
            '-' if self.s.eat_if("--") => SyntaxKind::Shorthand,
            '-' if self.s.eat_if('-') => SyntaxKind::Shorthand,
            '-' if self.s.eat_if('?') => SyntaxKind::Shorthand,
            '-' if self.s.at(char::is_numeric) => SyntaxKind::Shorthand,
            '*' if !self.in_word() => SyntaxKind::Star,
            '_' if !self.in_word() => SyntaxKind::Underscore,

            '#' => SyntaxKind::Hash,
            '[' => SyntaxKind::LeftBracket,
            ']' => SyntaxKind::RightBracket,
            '\'' => SyntaxKind::SmartQuote,
            '"' => SyntaxKind::SmartQuote,
            '$' => SyntaxKind::Dollar,
            '~' => SyntaxKind::Shorthand,
            ':' => SyntaxKind::Colon,
            '=' => {
                self.s.eat_while('=');
                if self.space_or_end() { SyntaxKind::HeadingMarker } else { self.text() }
            }
            '-' if self.space_or_end() => SyntaxKind::ListMarker,
            '+' if self.space_or_end() => SyntaxKind::EnumMarker,
            '/' if self.space_or_end() => SyntaxKind::TermMarker,
            '0'..='9' => self.numbering(start),

            _ => self.text(),
        }
    }

    pub(super) fn backslash(&mut self) -> SyntaxKind {
        if self.s.eat_if("u{") {
            let hex = self.s.eat_while(char::is_ascii_alphanumeric);
            if !self.s.eat_if('}') {
                return self.error("unclosed Unicode escape sequence");
            }

            if u32::from_str_radix(hex, 16)
                .ok()
                .and_then(std::char::from_u32)
                .is_none()
            {
                return self.error(format!("invalid Unicode codepoint: {}", hex));
            }

            return SyntaxKind::Escape;
        }

        if self.s.done() || self.s.at(char::is_whitespace) {
            SyntaxKind::Linebreak
        } else {
            self.s.eat();
            SyntaxKind::Escape
        }
    }

    /// We parse entire raw segments in the lexer as a convenience to avoid
    /// going to and from the parser for each raw section. See comments in
    /// [`Self::blocky_raw`] and [`Self::inline_raw`] for specific details.
    pub(super) fn raw(&mut self) -> (SyntaxKind, SyntaxNode) {
        let start = self.s.cursor() - 1;

        // Determine number of opening backticks.
        let mut backticks = 1;
        while self.s.eat_if('`') {
            backticks += 1;
        }

        // Special case for ``.
        if backticks == 2 {
            let nodes = vec![
                SyntaxNode::leaf(SyntaxKind::RawDelim, "`"),
                SyntaxNode::leaf(SyntaxKind::RawDelim, "`"),
            ];
            return (SyntaxKind::Raw, SyntaxNode::inner(SyntaxKind::Raw, nodes));
        }

        // Find end of raw text.
        let mut found = 0;
        while found < backticks {
            match self.s.eat() {
                Some('`') => found += 1,
                Some(_) => found = 0,
                None => {
                    let msg = SyntaxError::new("unclosed raw text");
                    let error = SyntaxNode::error(msg, self.s.from(start));
                    return (SyntaxKind::Error, error);
                }
            }
        }
        let end = self.s.cursor();

        let mut nodes = Vec::with_capacity(3); // Will have at least 3.

        // A closure for pushing a node onto our raw vector. Assumes the caller
        // will move the scanner to the next location at each step.
        let mut prev_start = start;
        let mut push_raw = |kind, s: &Scanner| {
            nodes.push(SyntaxNode::leaf(kind, s.from(prev_start)));
            prev_start = s.cursor();
        };

        // Opening delimiter.
        self.s.jump(start + backticks);
        push_raw(SyntaxKind::RawDelim, &self.s);

        if backticks >= 3 {
            self.blocky_raw(end - backticks, &mut push_raw);
        } else {
            self.inline_raw(end - backticks, &mut push_raw);
        }

        // Closing delimiter.
        self.s.jump(end);
        push_raw(SyntaxKind::RawDelim, &self.s);

        (SyntaxKind::Raw, SyntaxNode::inner(SyntaxKind::Raw, nodes))
    }

    /// Raw blocks parse a language tag, have smart behavior for trimming
    /// whitespace in the start/end lines, and trim common leading whitespace
    /// from all other lines as the "dedent". The exact behavior is described
    /// below.
    ///
    /// ### The initial line:
    /// - Text until the first whitespace or backtick is parsed as the language tag.
    /// - We check the rest of the line and if all characters are whitespace,
    ///   trim it. Otherwise we trim a single leading space if present.
    ///   - If more trimmed characters follow on future lines, they will be
    ///     merged into the same trimmed element.
    /// - If we didn't trim the entire line, the rest is kept as text.
    ///
    /// ### Inner lines:
    /// - We determine the "dedent" by iterating over the lines. The dedent is
    ///   the minimum number of leading whitespace characters (not bytes) before
    ///   each line that has any non-whitespace characters.
    ///   - The opening delimiter's line does not contribute to the dedent, but
    ///     the closing delimiter's line does (even if that line is entirely
    ///     whitespace up to the delimiter).
    /// - We then trim the newline and dedent characters of each line, and add a
    ///   (potentially empty) text element of all remaining characters.
    ///
    /// ### The final line:
    /// - If the last line is entirely whitespace, it is trimmed.
    /// - Otherwise its text is kept like an inner line. However, if the last
    ///   non-whitespace character of the final line is a backtick, then one
    ///   ascii space (if present) is trimmed from the end.
    fn blocky_raw<F>(&mut self, inner_end: usize, mut push_raw: F)
    where
        F: FnMut(SyntaxKind, &Scanner),
    {
        // Language tag.
        let tag = self.s.eat_until(|c: char| c.is_whitespace() || c == '`');
        if !tag.is_empty() {
            push_raw(SyntaxKind::RawLang, &self.s);
        }

        // The rest of the function operates on the lines between the backticks.
        let mut lines = split_newlines(self.s.to(inner_end));

        // Determine dedent level.
        let dedent = lines
            .iter()
            .skip(1)
            .filter(|line| !line.chars().all(char::is_whitespace))
            // The line with the closing ``` is always taken into account
            .chain(lines.last())
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        // Trim whitespace from the last line. Will be added as a `RawTrimmed`
        // kind by the check for `self.s.cursor() != inner_end` below.
        if lines.last().is_some_and(|last| last.chars().all(char::is_whitespace)) {
            lines.pop();
        } else if let Some(last) = lines.last_mut() {
            // If last line ends in a backtick, try to trim a single space. This
            // check must happen before we add the first line since the last and
            // first lines might be the same.
            if last.trim_end().ends_with('`') {
                *last = last.strip_suffix(' ').unwrap_or(last);
            }
        }

        let mut lines = lines.into_iter();

        // Handle the first line: trim if all whitespace, or trim a single space
        // at the start. Note that the first line does not affect the dedent
        // value.
        if let Some(first_line) = lines.next() {
            if first_line.chars().all(char::is_whitespace) {
                self.s.advance(first_line.len());
                // This is the only spot we advance the scanner, but don't
                // immediately call `push_raw`. But the rest of the function
                // ensures we will always add this text to a `RawTrimmed` later.
                debug_assert!(self.s.cursor() != inner_end);
                // A proof by cases follows:
                // # First case: The loop runs
                // If the loop runs, there must be a newline following, so
                // `cursor != inner_end`. And if the loop runs, the first thing
                // it does is add a trimmed element.
                // # Second case: The final if-statement runs.
                // To _not_ reach the loop from here, we must have only one or
                // two lines:
                // 1. If one line, we cannot be here, because the first and last
                //    lines are the same, so this line will have been removed by
                //    the check for the last line being all whitespace.
                // 2. If two lines, the loop will run unless the last is fully
                //    whitespace, but if it is, it will have been popped, then
                //    the final if-statement will run because the text removed
                //    by the last line must include at least a newline, so
                //    `cursor != inner_end` here.
            } else {
                let line_end = self.s.cursor() + first_line.len();
                if self.s.eat_if(' ') {
                    // Trim a single space after the lang tag on the first line.
                    push_raw(SyntaxKind::RawTrimmed, &self.s);
                }
                // We know here that the rest of the line is non-empty.
                self.s.jump(line_end);
                push_raw(SyntaxKind::Text, &self.s);
            }
        }

        // Add lines.
        for line in lines {
            let offset: usize = line.chars().take(dedent).map(char::len_utf8).sum();
            self.s.eat_newline();
            self.s.advance(offset);
            push_raw(SyntaxKind::RawTrimmed, &self.s);
            self.s.advance(line.len() - offset);
            push_raw(SyntaxKind::Text, &self.s);
        }

        // Add final trimmed.
        if self.s.cursor() < inner_end {
            self.s.jump(inner_end);
            push_raw(SyntaxKind::RawTrimmed, &self.s);
        }
    }

    /// Inline raw text is split on lines with non-newlines as `Text` kinds and
    /// newlines as `RawTrimmed`. Inline raw text does not dedent the text, all
    /// non-newline whitespace is kept.
    fn inline_raw<F>(&mut self, inner_end: usize, mut push_raw: F)
    where
        F: FnMut(SyntaxKind, &Scanner),
    {
        while self.s.cursor() < inner_end {
            if self.s.at(is_newline) {
                push_raw(SyntaxKind::Text, &self.s);
                self.s.eat_newline();
                push_raw(SyntaxKind::RawTrimmed, &self.s);
                continue;
            }
            self.s.eat();
        }
        push_raw(SyntaxKind::Text, &self.s);
    }

    fn link(&mut self) -> SyntaxKind {
        let (link, balanced) = link_prefix(self.s.after());
        self.s.advance(link.len());

        if !balanced {
            return self.error(
                "automatic links cannot contain unbalanced brackets, \
                 use the `link` function instead",
            );
        }

        SyntaxKind::Link
    }

    fn numbering(&mut self, start: usize) -> SyntaxKind {
        self.s.eat_while(char::is_ascii_digit);

        let read = self.s.from(start);
        if self.s.eat_if('.') && self.space_or_end() && read.parse::<u64>().is_ok() {
            return SyntaxKind::EnumMarker;
        }

        self.text()
    }

    fn ref_marker(&mut self) -> SyntaxKind {
        self.s.eat_while(is_valid_in_label_literal);

        // Don't include the trailing characters likely to be part of text.
        while matches!(self.s.scout(-1), Some('.' | ':')) {
            self.s.uneat();
        }

        SyntaxKind::RefMarker
    }

    pub(super) fn label(&mut self) -> SyntaxKind {
        let label = self.s.eat_while(is_valid_in_label_literal);
        if label.is_empty() {
            return self.error("label cannot be empty");
        }

        if !self.s.eat_if('>') {
            return self.error("unclosed label");
        }

        SyntaxKind::Label
    }

    fn text(&mut self) -> SyntaxKind {
        macro_rules! table {
            ($(|$c:literal)*) => {
                static TABLE: [bool; 128] = {
                    let mut t = [false; 128];
                    $(t[$c as usize] = true;)*
                    t
                };
            };
        }

        table! {
            | ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\\' | '/'
            | '[' | ']' | '~' | '-' | '.' | '\'' | '"' | '*' | '_'
            | ':' | 'h' | '`' | '$' | '<' | '>' | '@' | '#'
        };

        loop {
            self.s.eat_until(|c: char| {
                TABLE.get(c as usize).copied().unwrap_or_else(|| c.is_whitespace())
            });

            // Continue with the same text node if the thing would become text
            // anyway.
            let mut s = self.s;
            match s.eat() {
                Some(' ') if s.at(char::is_alphanumeric) => {}
                Some('/') if !s.at(['/', '*']) => {}
                Some('-') if !s.at(['-', '?']) => {}
                Some('.') if !s.at("..") => {}
                Some('h') if !s.at("ttp://") && !s.at("ttps://") => {}
                Some('@') if !s.at(is_valid_in_label_literal) => {}
                _ => break,
            }

            self.s = s;
        }

        SyntaxKind::Text
    }

    fn in_word(&self) -> bool {
        let wordy = |c: Option<char>| {
            c.is_some_and(|c| {
                c.is_alphanumeric()
                    && !matches!(
                        c.script(),
                        Script::Han
                            | Script::Hiragana
                            | Script::Katakana
                            | Script::Hangul
                    )
            })
        };
        let prev = self.s.scout(-2);
        let next = self.s.peek();
        wordy(prev) && wordy(next)
    }

    pub(super) fn space_or_end(&self) -> bool {
        self.s.done()
            || self.s.at(char::is_whitespace)
            || self.s.at("//")
            || self.s.at("/*")
    }
}



#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
