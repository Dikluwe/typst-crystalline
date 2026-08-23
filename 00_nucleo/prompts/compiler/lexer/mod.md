# Prompt L0 — `rules/lexer/mod` — Motor de Tokenização (Lexer)
Hash do Código: 91afcc6a

**Camada**: L1
**Ficheiros alvo**: `01_core/src/compiler/lexer/mod.rs`,
`01_core/src/compiler/lexer/markup.rs`
**Passo de origem**: Passo 2 (lexer base), expandido em Passos 10, 23, 32, 45
**ADRs relevantes**: ADR-0003 (modos de tokenização), ADR-0010 (SyntaxMode)

---

## Contexto e Objetivo

O `Lexer` é o **analisador léxico** do compilador Cristalino — a primeira fase
do pipeline de compilação. Recebe uma `&str` de código-fonte e produz um fluxo
de `(SyntaxKind, SyntaxNode)` que o `Parser` (`parse.rs`) consome.

Este módulo (`lexer/mod.rs`) contém o `Lexer` completo. O `Scanner`
(`lexer/scanner.rs`) fornece a primitiva de cursor sobre a string.

---

## Struct `Lexer<'s>`

```rust
pub(super) struct Lexer<'s> {
    s:       Scanner<'s>,    // cursor na string-fonte
    mode:    SyntaxMode,     // modo actual: Markup, Code ou Math
    newline: bool,           // último token continha newline?
    error:   Option<SyntaxError>, // erro pendente (emitido no próximo SyntaxNode)
}
```

### Construção e Controlo

```rust
pub fn new(text: &'s str, mode: SyntaxMode) -> Self
pub fn mode(&self) -> SyntaxMode
pub fn set_mode(&mut self, mode: SyntaxMode)
pub fn cursor(&self) -> usize   // próximo byte a ser consumido
pub fn jump(&mut self, index: usize)
pub fn newline(&self) -> bool
pub fn column(&self, index: usize) -> usize  // número de chars desde o último \n
```

### Token Principal

```rust
pub fn next(&mut self) -> (SyntaxKind, SyntaxNode)
```

Produz o próximo token. A estratégia por modo:

| Modo | Caractere | Handler |
|------|-----------|---------|
| Universal | espaço não absorvido por texto markup | `whitespace()` → `Space` ou `ParBreak` (≥2 newlines em Markup) |
| Universal | `//` | `line_comment()` → `LineComment` |
| Universal | `/*` | `block_comment()` → `BlockComment` (nested suportado) |
| Universal | `` ` `` (não Math) | `raw()` → `Raw` (inline ou block, com dedent) |
| Markup | `\` | `backslash()` → `Escape` ou `Linebreak` |
| Markup | `http://` `https://` | `link()` → `Link` |
| Markup | `<id>` | `label()` → `Label` |
| Markup | `@id` | `ref_marker()` → `RefMarker` |
| Markup | `==` (fim de linha) | `HeadingMarker` |
| Markup | `- ` (fim de linha) | `ListMarker` |
| Markup | `+ ` | `EnumMarker`; `N. ` → `EnumMarker` |
| Markup | `/ ` | `TermMarker` |
| Markup | `*` `_` (fora de palavra) | `Star`, `Underscore` |
| Math | `->` `=>` `!=` etc. | `MathShorthand` (23+ shorthands) |
| Math | identificadores multi-char | `math_ident_or_field()` → `MathIdent` ou `FieldAccess` |
| Math | `'` | `MathPrimes` (múltiplos `'` consecutivos) |
| Math | delimitadores `(` `)` | `LeftParen`, `RightParen` |
| Math | delimitadores classe Opening/Closing | `LeftBrace`, `RightBrace` |
| Code | `==` `!=` `<=` `>=` | `EqEq`, `ExclEq`, `LtEq`, `GtEq` |
| Code | `+=` `-=` `*=` `/=` | `PlusEq`, `HyphEq`, `StarEq`, `SlashEq` |
| Code | `..` `=>` | `Dots`, `Arrow` |
| Code | literais numéricos | `number()` → `Int`, `Float` |

### Helpers Especiais

```rust
// Detecção de contexto de palavra (para * e _ em Markup)
fn in_word(&self) -> bool
// Espaço, fim, ou início de comentário a seguir
fn space_or_end(&self) -> bool

// Argumentos em chamadas matemáticas
pub fn maybe_math_named_arg(&mut self, start: usize) -> Option<SyntaxNode>
pub fn maybe_math_spread_arg(&mut self, start: usize) -> Option<SyntaxNode>
```

### Raw Blocks

O lexer consome blocos `raw` completos em vez de delegar ao parser —
optimização para evitar round-trips. Lógica de dedent implementada via
`blocky_raw()`:
- Calcula o mínimo de whitespace inicial em todas as linhas internas
- Emite `RawTrimmed` para whitespace removido e `Text` para conteúdo
- Suporte a lang tag na primeira linha

---

## Modos de Tokenização

O lexer é um **autómato por modo** — o modo muda dinamicamente durante o parse
(ex: ao encontrar `$` → entra em Math; ao encontrar `{` em Markup → entra em Code).
O modo é controlado externamente pelo Parser via `set_mode()`.

```
SyntaxMode::Markup  → texto, headings, listas, links, equações inline
SyntaxMode::Code    → expressões, let, if, for, chamadas de função
SyntaxMode::Math    → expressões matemáticas (entre $...$)
```

---

## Critérios de Verificação

### Texto markup e espaço ASCII interno — P1137

Medição no vanilla ratificado `upstream/main a51e02804`:
`lab/typst-original/crates/typst-syntax/src/lexer.rs:600-636` mantém um espaço
ASCII no token `Text` quando o próximo caractere é alfanumérico, pelo braço
`Some(' ') if s.at(char::is_alphanumeric) => {}`. O cristalino não tinha esse
braço em `compiler/lexer/markup.rs:346-385`, produzindo uma morfologia diferente.

Decisão: em `SyntaxMode::Markup`, `text()` absorve exatamente `' '` quando o
scanner temporário aponta em seguida para `char::is_alphanumeric`. Tabs,
newlines, whitespace Unicode, espaço antes de pontuação/delimitador/fim e os
modos Code/Math continuam a produzir trivia separada. Esta é a origem léxica
da regra morfológica especificada em `compiler/parse.md` §P1137.

```
// Tokenização básica Markup
Lexer::new("hello", Markup).next() = (Text, node("hello"))
Lexer::new("hello world", Markup) → Text("hello world")
Lexer::new("hello !", Markup) → Text("hello"), Space(" "), Text("!")
Lexer::new("hello\tworld", Markup) → Text("hello"), Space("\t"), Text("world")
Lexer::new("// comentário\n", Markup).next() = (LineComment, ...)
Lexer::new("  \n\n  ", Markup) → Space, Parbreak (2 newlines)

// Comentário aninhado
Lexer::new("/* /* */ */", Markup) → BlockComment (não fecha precocemente)

// Modo Code
Lexer::new("!=", Code).next() = (ExclEq, ...)
Lexer::new("=>", Code).next() = (Arrow, ...)

// Modo Math
Lexer::new("->", Math).next() = (MathShorthand, "→")
Lexer::new("alpha", Math) → (MathIdent, "alpha")
Lexer::new("x.y", Math) → (FieldAccess, node com x.y)
Lexer::new("'''", Math) → (MathPrimes, "'''")  // triplo prime

// Escapes em Markup
Lexer::new("\\n ", Markup).next() = (Linebreak, "\\") seguido de (Text, "n ")
Lexer::new("\\u{03B1}", Markup).next() = (Escape, "\\u{03B1}")
Lexer::new("\\u{ZZZZZ}", Markup).next() = (Error, ...)  // codepoint inválido

// column()
Lexer::new("abc\nde", Markup).column(5) = 1  // 'e' é o 2º char da linha 2

// in_word — * e _ apenas fora de palavra
Lexer::new("a*b", Markup) → Text, não Star  // em palavra
Lexer::new("* item", Markup) → Star         // fora de palavra

// Smart quotes (Passo 445)
Lexer::new("\"hello\"", Markup) → SmartQuote, Text, SmartQuote
Lexer::new("'hello'", Markup) → SmartQuote, Text, SmartQuote
Lexer::new("\"hello\"", Code)  → Str (não SmartQuote)  // code mode inalterado
Lexer::new("$x'$", Math)       → MathPrimes (não SmartQuote)  // math mode inalterado
```

## Smart Quotes (Passo 445)

Em modo `Markup`, os caracteres `"` e `'` são emitidos como `SyntaxKind::SmartQuote`.
A conversão para aspas tipográficas (curly) é feita no `eval_markup` com base no contexto:

- `"` precedido de whitespace ou início de texto → abertura (`U+201C` em `en`).
- `"` noutro contexto → fecho (`U+201D` em `en`).
- `'` precedido de whitespace/início e não seguido por letra imediatamente? → abertura (`U+2018`).
- `'` entre caracteres alfanuméricos (`don't`, `Alice's`) → apóstrofo/fecho (`U+2019`).
- `'` noutro contexto → fecho (`U+2019`).

A localização das aspas depende do `text.lang` activo; línguas sem tabela específica usam
aspas curly inglesas por defeito para simples e ASCII para duplas.

Em `Code`, `"` continua a delimitar literais de string. Em `Math`, `'` continua a representar
primes (`MathPrimes`).
