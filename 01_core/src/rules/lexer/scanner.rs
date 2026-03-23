//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/scanner.md
//! @prompt-hash a40b976d
//! @layer L1
//! @updated 2026-03-23
//!
//! String scanner para o lexer do Typst.
//! Inlinado de `unscanny` (Apache-2.0) — ADR-0014.
//! Origem: https://github.com/typst/unscanny

use std::fmt::{self, Debug, Formatter, Write};
use std::ops::Range;

/// String scanner com cursor de byte.
///
/// Gere um `&str` com um índice de byte e expõe operações de peek,
/// consume e slice para construção de lexers.
///
/// Invariante de segurança: `0 <= cursor <= string.len()` e o cursor
/// está sempre numa fronteira de codepoint UTF-8.
///
/// Inlinado de `unscanny` (Apache-2.0, Typst GmbH) — ADR-0014.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scanner<'a> {
    string: &'a str,
    cursor: usize,
}

impl<'a> Scanner<'a> {
    /// Cria um novo scanner com cursor em `0`.
    #[inline]
    pub fn new(string: &'a str) -> Self {
        Self { string, cursor: 0 }
    }

    /// A string fonte completa.
    #[inline]
    pub fn string(&self) -> &'a str {
        self.string
    }

    /// A posição actual do cursor (índice de byte).
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Se o scanner consumiu a string inteira.
    #[inline]
    pub fn done(&self) -> bool {
        self.cursor == self.string.len()
    }

    /// O slice antes do cursor.
    #[inline]
    pub fn before(&self) -> &'a str {
        debug_assert!(self.string.is_char_boundary(self.cursor));
        // Safety: cursor é sempre in-bounds e numa fronteira de codepoint.
        unsafe { self.string.get_unchecked(..self.cursor) }
    }

    /// O slice depois do cursor.
    #[inline]
    pub fn after(&self) -> &'a str {
        debug_assert!(self.string.is_char_boundary(self.cursor));
        // Safety: cursor é sempre in-bounds e numa fronteira de codepoint.
        unsafe { self.string.get_unchecked(self.cursor..) }
    }

    /// Os slices antes e depois do cursor.
    #[inline]
    pub fn parts(&self) -> (&'a str, &'a str) {
        (self.before(), self.after())
    }

    /// O slice desde `start` até ao cursor.
    ///
    /// Ajusta `start` para dentro dos bounds e para a próxima fronteira de char.
    #[inline]
    pub fn from(&self, start: usize) -> &'a str {
        let start = self.snap(start).min(self.cursor);
        debug_assert!(self.string.is_char_boundary(start));
        debug_assert!(self.string.is_char_boundary(self.cursor));
        // Safety: start e cursor são in-bounds e em fronteiras de codepoint.
        unsafe { self.string.get_unchecked(start..self.cursor) }
    }

    /// O slice desde o cursor até `end`.
    ///
    /// Ajusta `end` para dentro dos bounds e para a próxima fronteira de char.
    #[inline]
    pub fn to(&self, end: usize) -> &'a str {
        let end = self.snap(end).max(self.cursor);
        debug_assert!(self.string.is_char_boundary(self.cursor));
        debug_assert!(self.string.is_char_boundary(end));
        // Safety: cursor e end são in-bounds e em fronteiras de codepoint.
        unsafe { self.string.get_unchecked(self.cursor..end) }
    }

    /// O slice desde `start` até `end`.
    ///
    /// Ajusta ambos para dentro dos bounds e para fronteiras de char.
    #[inline]
    pub fn get(&self, range: Range<usize>) -> &'a str {
        let start = self.snap(range.start);
        let end = self.snap(range.end).max(start);
        debug_assert!(self.string.is_char_boundary(start));
        debug_assert!(self.string.is_char_boundary(end));
        // Safety: start e end são in-bounds e em fronteiras de codepoint.
        unsafe { self.string.get_unchecked(start..end) }
    }

    /// O char imediatamente à frente do cursor.
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.after().chars().next()
    }

    /// Se o que está à frente do cursor corresponde ao padrão.
    #[inline]
    pub fn at<T>(&self, mut pat: impl Pattern<T>) -> bool {
        pat.matches(self.after()).is_some()
    }

    /// O n-ésimo char relativo ao cursor (sem mover o cursor).
    ///
    /// - `scout(-1)` é o char antes do cursor.
    /// - `scout(0)` equivale a `peek()`.
    ///
    /// O(|n|).
    #[inline]
    pub fn scout(&self, n: isize) -> Option<char> {
        if n >= 0 {
            self.after().chars().nth(n as usize)
        } else {
            self.before().chars().nth_back((-n - 1) as usize)
        }
    }

    /// O índice de byte do n-ésimo char relativo ao cursor.
    ///
    /// - `locate(-1)` é a posição do char antes do cursor.
    /// - `locate(0)` equivale a `cursor()`.
    ///
    /// O(|n|).
    #[inline]
    pub fn locate(&self, n: isize) -> usize {
        if n >= 0 {
            let mut chars = self.after().chars();
            for _ in 0..n {
                if chars.next().is_none() {
                    break;
                }
            }
            self.string.len() - chars.as_str().len()
        } else {
            let mut chars = self.before().chars();
            for _ in 0..-n {
                if chars.next_back().is_none() {
                    break;
                }
            }
            chars.as_str().len()
        }
    }

    /// Consome e retorna o char imediatamente à frente do cursor.
    #[inline]
    pub fn eat(&mut self) -> Option<char> {
        let peeked = self.peek();
        if let Some(c) = peeked {
            // Safety: quando `c` está logo à frente do cursor, existe uma
            // fronteira de codepoint em `self.cursor + c.len_utf8()`.
            self.cursor += c.len_utf8();
        }
        peeked
    }

    /// Consome e retorna o char imediatamente atrás do cursor, recuando.
    #[inline]
    pub fn uneat(&mut self) -> Option<char> {
        let unpeeked = self.before().chars().next_back();
        if let Some(c) = unpeeked {
            // Safety: quando `c` está logo antes do cursor, existe uma
            // fronteira de codepoint em `self.cursor - c.len_utf8()`.
            self.cursor -= c.len_utf8();
        }
        unpeeked
    }

    /// Consome o padrão se corresponder ao que está à frente do cursor.
    ///
    /// Retorna `true` se o padrão foi consumido.
    #[inline]
    pub fn eat_if<T>(&mut self, mut pat: impl Pattern<T>) -> bool {
        if let Some(len) = pat.matches(self.after()) {
            // Safety: o contrato de `matches` garante uma fronteira de
            // codepoint em `len` bytes dentro de `self.after()`.
            self.cursor += len;
            true
        } else {
            false
        }
    }

    /// Consome enquanto o padrão corresponder ao que está à frente do cursor.
    ///
    /// Retorna o substring consumido.
    #[inline]
    pub fn eat_while<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str {
        let start = self.cursor;
        while let Some(len @ 1..) = pat.matches(self.after()) {
            // Safety: contrato de `matches` garante fronteira de codepoint.
            self.cursor += len;
        }
        self.from(start)
    }

    /// Consome até o padrão corresponder ao que está à frente do cursor.
    ///
    /// Retorna o substring consumido.
    #[inline]
    pub fn eat_until<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str {
        let start = self.cursor;
        while !self.done() && pat.matches(self.after()).is_none() {
            self.eat();
        }
        self.from(start)
    }

    /// Consome todo o whitespace até ao próximo char não-whitespace.
    ///
    /// Retorna o whitespace consumido.
    #[inline]
    pub fn eat_whitespace(&mut self) -> &'a str {
        self.eat_while(char::is_whitespace)
    }

    /// Consome o padrão ou panic se não corresponder.
    #[inline]
    #[track_caller]
    pub fn expect<T>(&mut self, mut pat: impl Pattern<T>) {
        if let Some(len) = pat.matches(self.after()) {
            // Safety: contrato de `matches` garante fronteira de codepoint.
            self.cursor += len;
        } else {
            pat.expected();
        }
    }

    /// Salta para uma posição de byte na string.
    ///
    /// Ajusta para dentro dos bounds e para a próxima fronteira de char.
    #[inline]
    pub fn jump(&mut self, target: usize) {
        // Safety: snap retorna um índice in-bounds numa fronteira de codepoint.
        self.cursor = self.snap(target);
    }
}

impl<'a> Scanner<'a> {
    /// Normaliza um índice para dentro dos bounds e para a próxima fronteira de codepoint.
    #[inline]
    fn snap(&self, mut index: usize) -> usize {
        index = index.min(self.string.len());
        while !self.string.is_char_boundary(index) {
            index -= 1;
        }
        index
    }
}

impl Debug for Scanner<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Scanner(")?;
        let (before, after) = self.parts();
        if !before.is_empty() {
            before.fmt(f)?;
            f.write_char(' ')?;
        }
        f.write_char('|')?;
        if !after.is_empty() {
            f.write_char(' ')?;
            after.fmt(f)?;
        }
        f.write_char(')')
    }
}

// ---------------------------------------------------------------------------
// Pattern trait — análogo a `std::str::pattern::Pattern` (unstable).
// Inlinado de `unscanny` (Apache-2.0) — ADR-0014.
// ---------------------------------------------------------------------------

/// Algo com que uma string pode começar.
///
/// Implementado para `char`, `&str`, `[char; N]`, `&[char]`,
/// `FnMut(char) -> bool`, `FnMut(&char) -> bool`.
pub trait Pattern<T>: sealed::Sealed<T> {}

mod sealed {
    pub unsafe trait Sealed<T> {
        /// Se a string começa com o padrão, retorna `Some(len)` com o
        /// comprimento em bytes do match. Para segurança, `len` deve estar
        /// in-bounds e apontar para uma fronteira UTF-8.
        fn matches(&mut self, string: &str) -> Option<usize>;

        /// Panic com mensagem indicando o padrão esperado.
        fn expected(&self);
    }
}

impl Pattern<()> for char {}
unsafe impl sealed::Sealed<()> for char {
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        let mut buf = [0; 4];
        let needle = &*self.encode_utf8(&mut buf);
        string.starts_with(needle).then(|| needle.len())
    }

    #[cold]
    fn expected(&self) {
        panic!("expected {self:?}");
    }
}

impl Pattern<()> for &str {}
unsafe impl sealed::Sealed<()> for &str {
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        string.starts_with(&*self).then(|| self.len())
    }

    #[cold]
    fn expected(&self) {
        panic!("expected {self:?}");
    }
}

impl Pattern<()> for &[char] {}
unsafe impl sealed::Sealed<()> for &[char] {
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        let next = string.chars().next()?;
        self.iter().any(|&c| c == next).then(|| next.len_utf8())
    }

    #[cold]
    fn expected(&self) {
        struct Or<'a>(&'a [char]);
        impl Debug for Or<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let mut iter = self.0.iter();
                if let Some(c) = iter.next() {
                    c.fmt(f)?;
                    for c in iter {
                        f.write_str(" or ")?;
                        c.fmt(f)?;
                    }
                }
                Ok(())
            }
        }
        if self.is_empty() {
            panic!("empty slice cannot match");
        } else {
            panic!("expected {:?}", Or(self));
        }
    }
}

impl<const N: usize> Pattern<()> for [char; N] {}
unsafe impl<const N: usize> sealed::Sealed<()> for [char; N] {
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        self.as_slice().matches(string)
    }

    #[cold]
    fn expected(&self) {
        self.as_slice().expected();
    }
}

impl<const N: usize> Pattern<()> for &[char; N] {}
unsafe impl<const N: usize> sealed::Sealed<()> for &[char; N] {
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        self.as_slice().matches(string)
    }

    #[cold]
    fn expected(&self) {
        self.as_slice().expected();
    }
}

impl<F> Pattern<char> for F where F: FnMut(char) -> bool {}
unsafe impl<F> sealed::Sealed<char> for F
where
    F: FnMut(char) -> bool,
{
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        string.chars().next().filter(|&c| self(c)).map(char::len_utf8)
    }

    #[cold]
    fn expected(&self) {
        panic!("expected closure to return `true`");
    }
}

impl<F> Pattern<&char> for F where F: FnMut(&char) -> bool {}
unsafe impl<F> sealed::Sealed<&char> for F
where
    F: FnMut(&char) -> bool,
{
    #[inline]
    fn matches(&mut self, string: &str) -> Option<usize> {
        string.chars().next().filter(self).map(char::len_utf8)
    }

    #[cold]
    fn expected(&self) {
        panic!("expected closure to return `true`");
    }
}

// ---------------------------------------------------------------------------
// Testes de paridade com unscanny (Apache-2.0) — ADR-0014
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Scanner;

    #[test]
    fn fmt_debug() {
        let mut s = Scanner::new("hello world");
        assert_eq!(format!("{s:?}"), r#"Scanner(| "hello world")"#);
        s.eat_while(char::is_alphabetic);
        assert_eq!(format!("{s:?}"), r#"Scanner("hello" | " world")"#);
        s.eat_while(|_| true);
        assert_eq!(format!("{s:?}"), r#"Scanner("hello world" |)"#);
    }

    #[test]
    fn empty_string() {
        let mut s = Scanner::new("");
        s.jump(10);
        assert_eq!(s.cursor(), 0);
        assert_eq!(s.done(), true);
        assert_eq!(s.before(), "");
        assert_eq!(s.after(), "");
        assert_eq!(s.from(0), "");
        assert_eq!(s.from(10), "");
        assert_eq!(s.to(10), "");
        assert_eq!(s.get(10..20), "");
        assert_eq!(s.at(""), true);
        assert_eq!(s.at('a'), false);
        assert_eq!(s.at(|_| true), false);
        assert_eq!(s.scout(-1), None);
        assert_eq!(s.scout(1), None);
        assert_eq!(s.locate(-1), 0);
        assert_eq!(s.locate(0), 0);
        assert_eq!(s.locate(1), 0);
        assert_eq!(s.eat(), None);
        assert_eq!(s.uneat(), None);
        assert_eq!(s.eat_if(""), true);
        assert_eq!(s.eat_if('a'), false);
        assert_eq!(s.eat_while(""), "");
        assert_eq!(s.eat_while('a'), "");
        assert_eq!(s.eat_until(""), "");
        assert_eq!(s.eat_whitespace(), "");
    }

    #[test]
    fn unicode_slices() {
        let mut s = Scanner::new("zoo 🦍🌴🎍 party");
        assert_eq!(s.parts(), ("", "zoo 🦍🌴🎍 party"));
        assert_eq!(s.get(2..9), "o 🦍");
        assert_eq!(s.get(2..22), "o 🦍🌴🎍 party");
        s.eat_while(char::is_ascii);
        assert_eq!(s.parts(), ("zoo ", "🦍🌴🎍 party"));
        assert_eq!(s.from(1), "oo ");
        assert_eq!(s.to(15), "🦍🌴");
        assert_eq!(s.to(16), "🦍🌴🎍");
        assert_eq!(s.to(17), "🦍🌴🎍 ");
        assert_eq!(s.to(usize::MAX), "🦍🌴🎍 party");
        s.eat_until(char::is_whitespace);
        assert_eq!(s.parts(), ("zoo 🦍🌴🎍", " party"));
        assert_eq!(s.from(3), " 🦍🌴🎍");
    }

    #[test]
    fn done_and_peek() {
        let mut s = Scanner::new("äbc");
        assert_eq!(s.done(), false);
        assert_eq!(s.peek(), Some('ä'));
        s.eat();
        assert_eq!(s.done(), false);
        assert_eq!(s.peek(), Some('b'));
        s.eat();
        assert_eq!(s.done(), false);
        assert_eq!(s.peek(), Some('c'));
        s.eat();
        assert_eq!(s.done(), true);
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn at_with_patterns() {
        let mut s = Scanner::new("Ђ12");
        assert!(s.at('Ђ'));
        assert!(s.at(['b', 'Ђ', 'Њ']));
        assert!(s.at("Ђ"));
        assert!(s.at("Ђ1"));
        assert!(s.at(char::is_alphabetic));
        assert!(!s.at(&['b', 'c']));
        assert!(!s.at("a13"));
        assert!(!s.at(char::is_numeric));
        s.eat();
        assert!(s.at(char::is_numeric));
        assert!(s.at(char::is_ascii_digit));
    }

    #[test]
    fn scout_and_locate() {
        let mut s = Scanner::new("a🐆c1Ф");
        s.eat_until(char::is_numeric);
        assert_eq!(s.scout(-4), None);
        assert_eq!(s.scout(-3), Some('a'));
        assert_eq!(s.scout(-2), Some('🐆'));
        assert_eq!(s.scout(-1), Some('c'));
        assert_eq!(s.scout(0), Some('1'));
        assert_eq!(s.scout(1), Some('Ф'));
        assert_eq!(s.scout(2), None);
        assert_eq!(s.locate(-4), 0);
        assert_eq!(s.locate(-3), 0);
        assert_eq!(s.locate(-2), 1);
        assert_eq!(s.locate(-1), 5);
        assert_eq!(s.locate(0), 6);
        assert_eq!(s.locate(1), 7);
        assert_eq!(s.locate(2), 9);
        assert_eq!(s.locate(3), 9);
    }

    #[test]
    fn eat_and_uneat() {
        let mut s = Scanner::new("🐶🐱🐭");
        assert_eq!(s.eat(), Some('🐶'));
        s.jump(usize::MAX);
        assert_eq!(s.uneat(), Some('🐭'));
        assert_eq!(s.uneat(), Some('🐱'));
        assert_eq!(s.uneat(), Some('🐶'));
        assert_eq!(s.uneat(), None);
        assert_eq!(s.eat(), Some('🐶'));
    }

    #[test]
    fn conditional_and_looping() {
        let mut s = Scanner::new("abc123def33");
        assert_eq!(s.eat_if('b'), false);
        assert_eq!(s.eat_if('a'), true);
        assert_eq!(s.eat_while(['a', 'b', 'c']), "bc");
        assert_eq!(s.eat_while(char::is_numeric), "123");
        assert_eq!(s.eat_until(char::is_numeric), "def");
        assert_eq!(s.eat_while('3'), "33");
    }

    #[test]
    fn eat_whitespace() {
        let mut s = Scanner::new("ሙም  \n  b\tቂ");
        assert_eq!(s.eat_whitespace(), "");
        assert_eq!(s.eat_while(char::is_alphabetic), "ሙም");
        assert_eq!(s.eat_whitespace(), "  \n  ");
        assert_eq!(s.eat_if('b'), true);
        assert_eq!(s.eat_whitespace(), "\t");
        assert_eq!(s.eat_while(char::is_alphabetic), "ቂ");
    }

    #[test]
    fn expect_ok() {
        let mut s = Scanner::new("🦚12");
        s.expect('🦚');
        s.jump(1);
        s.expect("🦚");
        assert_eq!(s.after(), "12");
    }

    #[test]
    #[should_panic(expected = "expected '🐢'")]
    fn expect_char_panic() {
        let mut s = Scanner::new("no turtle in sight");
        s.expect('🐢');
    }

    #[test]
    #[should_panic(expected = "expected \"🐢\"")]
    fn expect_str_panic() {
        let mut s = Scanner::new("no turtle in sight");
        s.expect("🐢");
    }

    #[test]
    #[should_panic(expected = "empty slice cannot match")]
    fn expect_empty_array_panic() {
        let mut s = Scanner::new("");
        s.expect([]);
    }

    #[test]
    #[should_panic(expected = "expected '🐢' or '🐬'")]
    fn expect_array_panic() {
        let mut s = Scanner::new("no turtle or dolphin in sight");
        s.expect(['🐢', '🐬']);
    }

    #[test]
    #[should_panic(expected = "expected closure to return `true`")]
    fn expect_closure_panic() {
        let mut s = Scanner::new("no numbers in sight");
        s.expect(char::is_numeric);
    }
}
