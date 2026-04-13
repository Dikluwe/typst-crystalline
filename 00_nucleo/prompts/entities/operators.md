# Prompt L0 — `entities/operators`
Hash do Código: 6e11eb9b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/operators.rs`
**Criado em**: 2026-04-12 (restauro — Passo 6 original, sem prompt dedicado)
**ADRs relevantes**: ADR-0004 (estrutura de L1), integrado ao prompt `rules/parse.md` (parse.rs referencia operators.rs)

---

## Contexto

`operators.rs` define os tipos de domínio para os operadores unários e binários
do Typst. São tipos de dado puro que representam os nós semânticos da AST após
o parsing — são usados pelo `eval.rs` para despachar a lógica de avaliação.

O ficheiro partilha o cabeçalho `@prompt rules/parse.md` porque surgiu como
parte do pipeline de parsing (Passo 6). Este prompt dedicado regista a
especificação canónica do módulo para fins de auditoria L0.

Origem: `lab/typst-original/crates/typst-syntax/src/ast/code.rs` (operadores)
e `lab/typst-original/crates/typst-library/src/foundations/ops.rs` (semântica).

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global. Tipos `Copy + Clone + Eq + Hash`.
- Sem dependências externas. Apenas `SyntaxKind` da própria L1.
- Os tipos devem ser `#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]`.
- Os operadores são **tipos de dado** — não contêm lógica de avaliação.
  A semântica é implementada em `eval.rs`.
- Proibido: `use BinOp::*` e `use UnOp::*` em `eval.rs` — o linter (V14)
  pode confundir com imports externos. Usar sempre `BinOp::Add`, etc.

---

## Instrução

### `UnOp` — Operador Unário

```rust
pub enum UnOp {
    Pos,  // `+`  — precedência 7
    Neg,  // `-`  — precedência 7
    Not,  // `not` — precedência 4
}

impl UnOp {
    pub fn from_kind(token: SyntaxKind) -> Option<Self>
    pub fn precedence(self) -> u8
    pub fn as_str(self) -> &'static str
}
```

### `BinOp` — Operador Binário

```rust
pub enum BinOp {
    Add, Sub, Mul, Div,                  // aritmética — precedência 5/6
    And, Or,                             // lógica — precedência 2/3
    Eq, Neq, Lt, Leq, Gt, Geq, In, NotIn, // comparação/pertença — 4
    Assign,                              // atribuição — precedência 1 (right-assoc)
    AddAssign, SubAssign, MulAssign, DivAssign, // atrib. composta — 1
}

impl BinOp {
    pub fn from_kind(token: SyntaxKind) -> Option<Self>
    pub fn precedence(self) -> u8
    pub fn assoc(self) -> Assoc
    pub fn as_str(self) -> &'static str
}
```

### `Assoc` — Associatividade

```rust
pub enum Assoc {
    Left,   // todos os operadores excepto atribuição
    Right,  // Assign, AddAssign, SubAssign, MulAssign, DivAssign
}
```

### Tabela de precedências

| Precedência | Operadores |
|-------------|-----------|
| 7 | `Pos`, `Neg` (unários) |
| 6 | `Mul`, `Div` |
| 5 | `Add`, `Sub` |
| 4 | `Eq`, `Neq`, `Lt`, `Leq`, `Gt`, `Geq`, `In`, `NotIn`, `Not` (unário) |
| 3 | `And` |
| 2 | `Or` |
| 1 | `Assign`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` |

### Nota sobre `NotIn`

`NotIn` não tem uma correspondência directa num único `SyntaxKind` — é construído
pelo parser com lógica especial (`Not` + `In`). Por isso, `BinOp::from_kind` nunca
retorna `NotIn`. A precedência e associatividade de `NotIn` devem mesmo assim ser
confirmadas nos testes (cobre o contrato implícito do parser).

---

## Critérios de Verificação

```
UnOp::from_kind(SyntaxKind::Plus)  = Some(UnOp::Pos)
UnOp::from_kind(SyntaxKind::Minus) = Some(UnOp::Neg)
UnOp::from_kind(SyntaxKind::Not)   = Some(UnOp::Not)
UnOp::from_kind(SyntaxKind::Star)  = None

UnOp::Pos.precedence() = 7
UnOp::Not.precedence() = 4

BinOp::from_kind(SyntaxKind::Plus)    = Some(BinOp::Add)
BinOp::from_kind(SyntaxKind::EqEq)   = Some(BinOp::Eq)
BinOp::from_kind(SyntaxKind::Eq)     = Some(BinOp::Assign)
BinOp::from_kind(SyntaxKind::Not)    = None

BinOp::Mul.precedence() > BinOp::Add.precedence()
BinOp::Add.precedence() > BinOp::And.precedence()

BinOp::Add.assoc()    = Assoc::Left
BinOp::Assign.assoc() = Assoc::Right

// NotIn — sem from_kind mas com precedência e assoc definidos:
BinOp::NotIn.precedence() = 4
BinOp::NotIn.assoc()      = Assoc::Left
```

---

## Resultado Esperado

- `01_core/src/entities/operators.rs` com `UnOp`, `BinOp`, `Assoc` e respetivos `impl`
- Testes co-localizados em `#[cfg(test)]`
- Cabeçalho de linhagem apontando para este ficheiro (`@prompt 00_nucleo/prompts/entities/operators.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-12 | Restauro — prompt dedicado criado a partir da implementação existente | `operators.md` |
