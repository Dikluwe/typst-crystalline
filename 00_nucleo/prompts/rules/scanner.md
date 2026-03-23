# Prompt L0 — `scanner` (motor de travessia de string)

**Camada**: L1
**Ficheiro**: `01_core/src/rules/lexer/scanner.rs`
**ADR**: `00_nucleo/adr/typst-adr-0014-unscanny.md`

---

## Contexto

`Scanner` é o motor central de travessia de string do lexer do Typst.
Gere um `&str` com um cursor de byte e expõe operações de peek, consume
e slice — abstracção fundamental sobre a qual o lexer inteiro opera.

Inlinado de `unscanny` (Apache-2.0, Typst GmbH) — ADR-0014.
A interface pública é mantida idêntica à do original para que a
migração de `lexer.rs` seja search-replace de imports sem alteração
de lógica:

```
// Antes:  use unscanny::Scanner;
// Depois: use crate::rules::lexer::scanner::Scanner;
```

---

## Interface pública

```rust
/// String scanner com cursor de byte.
///
/// Inlinado de `unscanny` (Apache-2.0) — ADR-0014.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scanner<'a> { ... }

impl<'a> Scanner<'a> {
    pub fn new(string: &'a str) -> Self;
    pub fn string(&self) -> &'a str;
    pub fn cursor(&self) -> usize;
    pub fn done(&self) -> bool;
    pub fn before(&self) -> &'a str;
    pub fn after(&self) -> &'a str;
    pub fn parts(&self) -> (&'a str, &'a str);
    pub fn from(&self, start: usize) -> &'a str;
    pub fn to(&self, end: usize) -> &'a str;
    pub fn get(&self, range: Range<usize>) -> &'a str;
    pub fn peek(&self) -> Option<char>;
    pub fn at<T>(&self, pat: impl Pattern<T>) -> bool;
    pub fn scout(&self, n: isize) -> Option<char>;
    pub fn locate(&self, n: isize) -> usize;
    pub fn eat(&mut self) -> Option<char>;
    pub fn uneat(&mut self) -> Option<char>;
    pub fn eat_if<T>(&mut self, pat: impl Pattern<T>) -> bool;
    pub fn eat_while<T>(&mut self, pat: impl Pattern<T>) -> &'a str;
    pub fn eat_until<T>(&mut self, pat: impl Pattern<T>) -> &'a str;
    pub fn eat_whitespace(&mut self) -> &'a str;
    pub fn expect<T>(&mut self, pat: impl Pattern<T>);
    pub fn jump(&mut self, target: usize);
}

/// Abstracção de padrão de matching — análogo a `std::str::pattern::Pattern`.
/// Implementado para: `char`, `&str`, `[char; N]`, `&[char]`,
/// `FnMut(char) -> bool`, `FnMut(&char) -> bool`.
pub trait Pattern<T>: Sealed<T> {}
```

---

## Critérios de verificação

**Cursor começa em zero**
- `Scanner::new("abc").cursor() == 0`

**`eat` avança e retorna char**
- `eat()` em "äbc" retorna `Some('ä')` e avança 2 bytes (UTF-8)

**`uneat` recua**
- após `eat()`, `uneat()` retorna o mesmo char e recua o cursor

**`eat_while` consome enquanto padrão**
- `eat_while(char::is_alphabetic)` em "abc123" retorna "abc"

**`eat_until` consome até padrão**
- `eat_until(char::is_numeric)` em "abc123" retorna "abc"

**`from`/`to` retornam slices correctos**
- `from(start)` retorna o slice desde `start` até o cursor
- `to(end)` retorna o slice desde o cursor até `end`

**`jump` salta para posição (snapped)**
- `jump(usize::MAX)` posiciona no fim da string

**`scout`/`locate` navegam relativamente**
- `scout(-1)` é o char antes do cursor
- `scout(0)` é igual a `peek()`

**`done` quando consumida**
- `done()` é `true` iff cursor == string.len()

**`at` com vários tipos de padrão**
- `at('a')`, `at("ab")`, `at(['a','b'])`, `at(char::is_alphabetic)`

**`expect` panic em mismatch**
- `expect('🐢')` em "no turtle" deve panic com mensagem descritiva

**Parity com unscanny**
- A implementação segue o comportamento do `unscanny` original byte a byte

---

## Notas de implementação

- A invariante de segurança de cursor: `0 <= cursor <= string.len()`
  e cursor está sempre numa fronteira de codepoint UTF-8
- Operações de slice usam `unsafe get_unchecked` com invariante verificada
  por `debug_assert!` — comportamento idêntico ao unscanny original
- `snap(index)` normaliza índices out-of-bounds para fronteira válida
- O módulo `lexer` pode adicionar métodos específicos do léxico Typst
  directamente a `Scanner` (ex: `eat_raw_delim`) — vantagem do inline
- Não adicionar `#[deny(missing_docs)]` — o scanner é interno a L1
