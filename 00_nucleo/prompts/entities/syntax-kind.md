# Prompt L0 — `entities/syntax_kind`
Hash do Código: e7bddc00

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/syntax_kind.rs`
**Criado em**: 2026-03-22 (Passo 1)
**Atualizado em**: 2026-04-12 (restauro — expandido com categorização completa, todos os métodos e critérios)
**ADRs relevantes**: nenhum ADR dedicado; vocabulário base de todo o pipeline parse/eval/layout

---

## Contexto e Objetivo

O `SyntaxKind` é o **vocabulário base** do parser e do lexer. Representa
exaustivamente todas as palavras-chave, símbolos, tokens terminais e
estruturas não-terminais (nós da AST) que a linguagem Typst suporta.

É o tipo sobre o qual `SyntaxNode`, `SyntaxSet`, `operators.rs` e `parse.rs`
tomam decisões. Qualquer caminho de decisão no pipeline de parsing é
invariavelmente um `match self_kind { ... }` sobre `SyntaxKind`.

`#[repr(u8)]` garante que as variantes são discriminadas por um único byte,
o que permite que `SyntaxSet` as use como índice de bitset sem custo.

Origem: `lab/typst-original/crates/typst-syntax/src/kind.rs`
Zero dependências externas — zero alterações necessárias ao original.

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global.
- `SyntaxKind` é `#[repr(u8)]` — discriminante ≤ 127 para caber no `SyntaxSet`.
- `#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]`.
- Sem dados associados em nenhuma variante (unit enum puro).
- Todos os métodos são `pub` e retornam `bool` ou `&'static str`.

---

## Instrução

### Enum (93 variantes totais)

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    // ── Controlo de stream ───────────────────────────────────────────────
    End, Error,

    // ── Trivia (ignorados pelo parser em code/math mode) ────────────────
    Shebang, LineComment, BlockComment,

    // ── Markup (texto, estrutura, elementos visuais) ─────────────────────
    Markup, Text, Space, Linebreak, Parbreak, Escape, Shorthand, SmartQuote,
    Strong, Emph, Raw, RawLang, RawDelim, RawTrimmed,
    Link, Label, Ref, RefMarker,
    Heading, HeadingMarker,
    ListItem, ListMarker, EnumItem, EnumMarker, TermItem, TermMarker,
    Equation,

    // ── Math (expressões matemáticas) ────────────────────────────────────
    Math, MathText, MathIdent, MathShorthand, MathAlignPoint,
    MathDelimited, MathAttach, MathPrimes, MathFrac, MathRoot,

    // ── Delimitadores e símbolos ─────────────────────────────────────────
    Hash, LeftBrace, RightBrace, LeftBracket, RightBracket,
    LeftParen, RightParen,
    Comma, Semicolon, Colon,
    Star, Underscore, Dollar,
    Plus, Minus, Slash, Hat, Dot,
    Eq, EqEq, ExclEq, Lt, LtEq, Gt, GtEq,
    PlusEq, HyphEq, StarEq, SlashEq,
    Dots, Arrow, Root, Bang,

    // ── Palavras-chave (operadores lógicos) ──────────────────────────────
    Not, And, Or, None, Auto,

    // ── Palavras-chave (controlo de fluxo e estrutura) ───────────────────
    Let, Set, Show, Context, If, Else, For, In, While,
    Break, Continue, Return, Import, Include, As,

    // ── Nós não-terminais (output do parser) ─────────────────────────────
    Code, Ident, Bool, Int, Float, Numeric, Str,
    CodeBlock, ContentBlock, Parenthesized,
    Array, Dict, Named, Keyed,
    Unary, Binary, FieldAccess, FuncCall, Args, Spread,
    Closure, Params,
    LetBinding, SetRule, ShowRule, Contextual,
    Conditional, WhileLoop, ForLoop,
    ModuleImport, ImportItems, ImportItemPath, RenamedImportItem,
    ModuleInclude, LoopBreak, LoopContinue, FuncReturn,
    Destructuring, DestructAssignment,
}
```

### Métodos auxiliares

```rust
impl SyntaxKind {
    /// Classificadores booleanos (decisão O(1) no parser/recovery):
    pub fn is_trivia(self) -> bool
    // → Shebang | LineComment | BlockComment | Space | Parbreak

    pub fn is_keyword(self) -> bool
    // → Not | And | Or | None | Auto | Let | Set | Show | Context |
    //   If | Else | For | In | While | Break | Continue | Return |
    //   Import | Include | As

    pub fn is_error(self) -> bool    // → self == Error
    pub fn is_grouping(self) -> bool // → {( [ ) ] }
    pub fn is_terminator(self) -> bool // → End | Semicolon | } ) ]
    pub fn is_block(self) -> bool    // → CodeBlock | ContentBlock
    pub fn is_stmt(self) -> bool
    // → LetBinding | SetRule | ShowRule | ModuleImport | ModuleInclude

    /// Nome legível para diagnósticos de erro.
    /// Ex: SyntaxKind::Let.name() = "keyword `let`"
    pub fn name(self) -> &'static str
}
```

### Categorização das variantes por grupo

| Grupo | Variantes |
|-------|-----------|
| Trivia | `Shebang`, `LineComment`, `BlockComment`, `Space`, `Parbreak` |
| Keywords | `Not`, `And`, `Or`, `None`, `Auto`, `Let`, `Set`, `Show`, `Context`, `If`, `Else`, `For`, `In`, `While`, `Break`, `Continue`, `Return`, `Import`, `Include`, `As` |
| Tokens literais | `Ident`, `Bool`, `Int`, `Float`, `Numeric`, `Str` |
| Delimitadores | `{`, `}`, `[`, `]`, `(`, `)`, `,`, `;`, `:` |
| Operadores | `+`, `-`, `*`, `/`, `^`, `.`, `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `+=`, `-=`, `*=`, `/=`, `..`, `=>` |
| Nós de markup | `Markup`, `Text`, `Strong`, `Emph`, `Heading`, etc. |
| Nós de math | `Math`, `MathText`, `MathAttach`, `MathFrac`, etc. |
| Nós de code | `Code`, `CodeBlock`, `FuncCall`, `Closure`, `LetBinding`, etc. |
| Controlo | `End`, `Error` |

---

## Critérios de Verificação

```
// Keywords
SyntaxKind::Let.is_keyword()    = true
SyntaxKind::For.is_keyword()    = true
SyntaxKind::Return.is_keyword() = true
SyntaxKind::Import.is_keyword() = true
SyntaxKind::Text.is_keyword()   = false
SyntaxKind::Ident.is_keyword()  = false
SyntaxKind::Int.is_keyword()    = false

// Trivia
SyntaxKind::Space.is_trivia()       = true
SyntaxKind::LineComment.is_trivia() = true
SyntaxKind::Parbreak.is_trivia()    = true
SyntaxKind::Text.is_trivia()        = false

// Error
SyntaxKind::Error.is_error()        = true
SyntaxKind::Text.is_error()         = false

// Grouping
SyntaxKind::LeftBracket.is_grouping() = true
SyntaxKind::RightParen.is_grouping()  = true
SyntaxKind::Comma.is_grouping()       = false

// Name
SyntaxKind::Text.name()  = "text"
SyntaxKind::Error.name() = "syntax error"
SyntaxKind::Let.name()   = "keyword `let`"

// Discriminante < 128 para compatibilidade com SyntaxSet
// (assert em tempo de compilação em syntax_set.rs)
```

---

## Resultado Esperado

- `01_core/src/entities/syntax_kind.rs` com todas as 93 variantes e métodos documentados
- Testes co-localizados em `#[cfg(test)]` cobrindo os critérios acima
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/syntax-kind.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 1: migração directa do original; 93 variantes | `syntax_kind.rs` |
| 2026-04-12 | Restauro — expandido: categorização por grupo, tabela de variantes, todos os métodos, critérios ricos | `syntax-kind.md` |
