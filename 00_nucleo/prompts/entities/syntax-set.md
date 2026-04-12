# Prompt L0 — `entities/syntax_set`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/syntax_set.rs`
**Criado em**: 2026-03-22 (Passo 4)
**Atualizado em**: 2026-04-12 (restauro — expandido com constantes pré-definidas e macro)
**ADRs relevantes**: nenhum ADR dedicado; parte integrante do parser (Passo 4)

---

## Contexto e Objetivo

Durante as fases de *parsing* e recuperação de erros (*error recovery*), o
parser precisa de verificar em O(1) se o token atual pertence a um conjunto
de tokens esperados (ex: "pode este token iniciar uma expressão de código?").

`SyntaxSet` implementa um conjunto ultra-rápido de variantes `SyntaxKind`
como um **bitset de `u128`**. Cada bit corresponde ao discriminante `u8` de
uma variante de `SyntaxKind`. Apenas variantes com discriminante < 128 podem
ser armazenadas (verificado por `assert!` em tempo de compilação).

Inspirado no `TokenSet` do `rust-analyzer`.

Origem: `lab/typst-original/crates/typst-syntax/src/set.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero alocação no heap.
- `SyntaxSet` é `Copy + Clone + Default` — nunca aloca.
- Todos os métodos são `const fn` — podem ser chamados em contextos `const`.
- Dependências: apenas `SyntaxKind` (L1 interno). Zero crates externas.
- A macro `syntax_set!` gera constantes em tempo de compilação (sem custo em runtime).
- Discriminante máximo suportado: 127 (limite do `u128`).

---

## Instrução

### Estrutura e interface pública

```rust
/// Bitset de SyntaxKind baseado em u128.
/// Cada bit corresponde ao discriminante u8 de uma variante SyntaxKind.
#[derive(Default, Copy, Clone)]
pub struct SyntaxSet(u128);

impl SyntaxSet {
    pub const fn new() -> Self                          // conjunto vazio (0u128)
    pub const fn add(self, kind: SyntaxKind) -> Self   // OR bit
    pub const fn remove(self, kind: SyntaxKind) -> Self // AND NOT bit
    pub const fn union(self, other: Self) -> Self      // OR dois sets
    pub const fn contains(&self, kind: SyntaxKind) -> bool // (mask & bit) != 0
}
```

### Macro de conveniência

```rust
/// Gera um SyntaxSet constante dos kinds indicados.
macro_rules! syntax_set {
    ($($kind:ident),* $(,)?) => {{ ... }}
}
```

### Constantes pré-definidas

| Constante | Conteúdo |
|-----------|---------|
| `STMT` | `Let, Set, Show, Import, Include, Return` |
| `MATH_EXPR` | Tokens que iniciam expressões matemáticas |
| `CODE_EXPR` | `CODE_PRIMARY ∪ UNARY_OP` |
| `ATOMIC_CODE_EXPR` | `ATOMIC_CODE_PRIMARY` |
| `CODE_PRIMARY` | `ATOMIC_CODE_PRIMARY ∪ {Underscore}` |
| `ATOMIC_CODE_PRIMARY` | `Ident, LeftBrace, LeftBracket, LeftParen, Dollar, Let, Set, Show, Context, If, While, For, Import, Include, Break, Continue, Return, None, Auto, Int, Float, Bool, Numeric, Str, Label, Raw` |
| `UNARY_OP` | `Plus, Minus, Not` |
| `BINARY_OP` | `Plus, Minus, Star, Slash, And, Or, EqEq, ExclEq, Lt, LtEq, Gt, GtEq, Eq, In, PlusEq, HyphEq, StarEq, SlashEq` |
| `ARRAY_OR_DICT_ITEM` | `CODE_EXPR ∪ {Dots}` |
| `ARG` | `CODE_EXPR ∪ {Dots}` |
| `PARAM` | `PATTERN ∪ {Dots}` |
| `DESTRUCTURING_ITEM` | `PATTERN ∪ {Dots}` |
| `PATTERN` | `PATTERN_LEAF ∪ {LeftParen, Underscore}` |
| `PATTERN_LEAF` | `ATOMIC_CODE_EXPR` |

---

## Critérios de Verificação

```
SyntaxSet::new().contains(k)           = false para qualquer k

set.add(k).contains(k)                 = true
set.add(k).remove(k).contains(k)       = false

// union sem alocação
let a = SyntaxSet::new().add(SyntaxKind::Plus);
let b = SyntaxSet::new().add(SyntaxKind::Minus);
a.union(b).contains(Plus)              = true
a.union(b).contains(Minus)             = true
a.union(b).contains(Star)              = false

// SyntaxSet é Copy
let x = SyntaxSet::new().add(SyntaxKind::Text);
let y = x; // cópia, não move
x.contains(SyntaxKind::Text)           = true
y.contains(SyntaxKind::Text)           = true

// Constantes
STMT.contains(SyntaxKind::Let)         = true
STMT.contains(SyntaxKind::Set)         = true
STMT.contains(SyntaxKind::Import)      = true
STMT.contains(SyntaxKind::Text)        = false
STMT.contains(SyntaxKind::Ident)       = false

UNARY_OP.contains(SyntaxKind::Not)     = true
UNARY_OP.contains(SyntaxKind::Plus)    = true
UNARY_OP.contains(SyntaxKind::Star)    = false

CODE_EXPR.contains(SyntaxKind::Ident)  = true
CODE_EXPR.contains(SyntaxKind::Not)    = true
CODE_EXPR.contains(SyntaxKind::Int)    = true
```

---

## Resultado Esperado

- `01_core/src/entities/syntax_set.rs` com `SyntaxSet`, `syntax_set!`, e todas as constantes
- Testes co-localizados em `#[cfg(test)]` cobrindo os critérios acima
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/syntax-set.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 4: bitset de SyntaxKind para o parser | `syntax_set.rs` |
| 2026-04-12 | Restauro — expandido com constantes pré-definidas, macro e critérios completos | `syntax-set.md` |
