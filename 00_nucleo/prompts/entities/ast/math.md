# Prompt L0 — `entities/ast/math`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ast/math.rs`
**Criado em**: 2026-03-26 (Passo 6)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado criado; antes coberto por `ast/mod.md`)
**ADRs relevantes**: ADR-0006 (zero-copy AST)

---

## Contexto e Objetivo

Mapeia as estruturas sintáticas exclusivas do modo matemático (`$ ... $`).
Fornece acesso tipado aos operandos matemáticos — base, superscript, subscript
(`MathAttach`), frações, raízes e shorthands matemáticos.

Opera **puramente como AST** — sem dependência de lógica de renderização
(`Layout`), `ttf-parser` ou qualquer código de tipografia.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Depende de `AstNode`, `SyntaxNode`, `SyntaxKind`, `Expr` (todos L1).
- Nenhum nó aloca memória — wrappers de `&'a SyntaxNode`.
- `MathShorthand::LIST` é uma constante estática — sem alocação.
- `Equation::block()` determina "bloco vs inline" pela presença de `Space` após
  `$` e antes de `$` — sem regexp, apenas inspeção da árvore.

---

## Instrução — Nós públicos e interface

```rust
// ── Equação ─────────────────────────────────────────────────────────────────
node! { struct Equation }      // "$x$" ou "$ x^2 $"
impl Equation<'a> {
    fn body(self) -> Math<'a>    // conteúdo matemático
    fn block(self) -> bool       // true se "$ ... $" (Space em children[1] e children[n-2])
}

// ── Conteúdo matemático ─────────────────────────────────────────────────────
node! { struct Math }          // conteúdo do corpo da equação
impl Math<'a> {
    fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>>
    fn was_deparenthesized(self) -> bool  // true se o corpo era "(math)"
}

// ── Texto e identificadores math ────────────────────────────────────────────
node! { struct MathText }      // fragmento de texto em math: "x", "25", "="
pub enum MathTextKind<'a> {
    Grapheme(&'a str),  // símbolo/grafema único
    Number(&'a str),    // literal numérico
}
impl MathText<'a> { fn get(self) -> MathTextKind<'a> }

node! { struct MathIdent }     // identificador: "pi", "alpha"
impl MathIdent<'a> {
    fn get(self) -> &'a str
    fn as_str(self) -> &'a str
}
impl Deref for MathIdent<'_> { type Target = str }

// ── Shorthands matemáticos ──────────────────────────────────────────────────
node! { struct MathShorthand }  // "!=" → '≠', "<=" → '≤', etc.
impl MathShorthand<'_> {
    const LIST: &'static [(&'static str, char)]  // 40+ entradas
    fn get(self) -> char  // lookup em LIST
}

// ── Ponto de alinhamento ────────────────────────────────────────────────────
node! { struct MathAlignPoint }  // "&" em equações alinhadas

// ── Delimitadores pareados ───────────────────────────────────────────────────
node! { struct MathDelimited }   // "[x + y]", "(a + b)"
impl MathDelimited<'a> {
    fn open(self) -> Expr<'a>    // delimitador de abertura
    fn body(self) -> Math<'a>    // conteúdo interior
    fn close(self) -> Expr<'a>   // delimitador de fecho
}

// ── Anexos (superscript / subscript) ────────────────────────────────────────
node! { struct MathAttach }      // "a_1^2"
impl MathAttach<'a> {
    fn base(self) -> Expr<'a>            // "a" — a base
    fn bottom(self) -> Option<Expr<'a>> // "_1" — subscript após Underscore
    fn top(self) -> Option<Expr<'a>>    // "^2" — superscript após Hat
    fn primes(self) -> Option<MathPrimes<'a>>  // primes em a''' (passo após base)
}

node! { struct MathPrimes }      // "'''"
impl MathPrimes<'_> { fn count(self) -> usize }  // len do texto (nº de ')

// ── Fracção ─────────────────────────────────────────────────────────────────
node! { struct MathFrac }        // "x/2"
impl MathFrac<'a> {
    fn num(self) -> Expr<'a>     // numerador (cast_first)
    fn denom(self) -> Expr<'a>   // denominador (cast_last)
}

// ── Raiz ────────────────────────────────────────────────────────────────────
node! { struct MathRoot }        // "√x", "∛x", "∜x"
impl MathRoot<'a> {
    fn index(self) -> Option<u8>  // Some(3) para ∛, Some(4) para ∜, None para √
    fn radicand(self) -> Expr<'a> // cast_first do conteúdo
}
```

---

## Critérios de Verificação

```
// MathShorthand::LIST não vazio
MathShorthand::LIST.is_empty() = false

// Equation com espaços é bloco
Source::detached("$ x $").root().children()
    .any(|n| n.kind() == SyntaxKind::Equation)    = true

// MathAttach: base / top / bottom
// (requer parsing real de "a_1^2")

// MathFrac: num e denom
// (requer parsing real de "x/2")

// MathRoot: index
// (símbolo "∛" → Some(3), "∜" → Some(4), "√" → None)

// from_untyped com kind errado
Equation::from_untyped(text_node) = None
```

---

## Resultado Esperado

- `01_core/src/entities/ast/math.rs` com todos os nós documentados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/ast/math.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-26 | Criação — Passo 6: nós de math `Equation`, `MathAttach`, `MathFrac`, `MathRoot` | `ast/math.rs` |
| 2026-04-12 | Restauro — prompt dedicado; interface completa, `MathTextKind`, `MathShorthand::LIST`, critérios | `ast/math.md` |
