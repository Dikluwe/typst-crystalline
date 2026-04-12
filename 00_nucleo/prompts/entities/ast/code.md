# Prompt L0 — `entities/ast/code`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ast/code.rs`
**Criado em**: 2026-03-26 (Passo 6)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado criado; antes coberto por `ast/mod.md`)
**ADRs relevantes**: ADR-0006 (zero-copy AST), ADR-0017 (adiamento eval)

---

## Contexto e Objetivo

Define os nós tipados das instruções de controlo de fluxo e regras de
formatação do modo Code. Tipa os nós que representam bindings (`LetBinding`),
regras de set/show (`SetRule`, `ShowRule`), condicionais (`Conditional`),
ciclos (`WhileLoop`, `ForLoop`) e o sistema de imports/includes.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Usa `Scanner`, `is_ident` e `PackageSpec` (todos L1).
- Sem dependências de `eval.rs` — os nós apenas expõem a estrutura, não a avaliam.
- `ModuleImport::bare_name` pode usar `std::path::Path` e `FromStr` para extrair
  o nome base de imports de string (ex: `"utils.typ"` → `"utils"`).

---

## Instrução — Nós públicos e interface

```rust
// ── Let binding ─────────────────────────────────────────────────────────────
node! { struct LetBinding }

pub enum LetBindingKind<'a> {
    Normal(Pattern<'a>),     // let x = expr
    Closure(Ident<'a>),      // let f(x) = body  (closure nomeada)
}
impl LetBindingKind<'a> { fn bindings(self) -> Vec<Ident<'a>> }

impl LetBinding<'a> {
    fn kind(self) -> LetBindingKind<'a>
    fn init(self) -> Option<Expr<'a>>   // None para let sem valor
}

// ── Destructuring assignment ─────────────────────────────────────────────────
node! { struct DestructAssignment }
impl DestructAssignment<'a> {
    fn pattern(self) -> Pattern<'a>
    fn value(self) -> Expr<'a>
}

// ── Set rule ─────────────────────────────────────────────────────────────────
// set text(size: 12pt) [if condition]
node! { struct SetRule }
impl SetRule<'a> {
    fn target(self) -> Expr<'a>           // "text"
    fn args(self) -> Args<'a>             // "(size: 12pt)"
    fn condition(self) -> Option<Expr<'a>> // após "if" — opcional
}

// ── Show rule ────────────────────────────────────────────────────────────────
// show heading: it => emph(it.body)
node! { struct ShowRule }
impl ShowRule<'a> {
    fn selector(self) -> Option<Expr<'a>> // antes de ":" (None = wildcard)
    fn transform(self) -> Expr<'a>        // após ":"
}

// ── Context ─────────────────────────────────────────────────────────────────
node! { struct Contextual }
impl Contextual<'a> { fn body(self) -> Expr<'a> }

// ── Condicional ─────────────────────────────────────────────────────────────
node! { struct Conditional }
impl Conditional<'a> {
    fn condition(self) -> Expr<'a>
    fn if_body(self) -> Expr<'a>           // panic se malformed
    fn else_body(self) -> Option<Expr<'a>> // None se sem else
}

// ── Ciclos ───────────────────────────────────────────────────────────────────
node! { struct WhileLoop }
impl WhileLoop<'a> {
    fn condition(self) -> Expr<'a>
    fn body(self) -> Expr<'a>
}

node! { struct ForLoop }
impl ForLoop<'a> {
    fn pattern(self) -> Pattern<'a>    // variável(is) de iteração
    fn iterable(self) -> Expr<'a>      // após "in" — panic se malformed
    fn body(self) -> Expr<'a>
}

// ── Imports ──────────────────────────────────────────────────────────────────
node! { struct ModuleImport }

pub enum BareImportError { Dynamic, PathInvalid, PackageInvalid }

impl ModuleImport<'a> {
    fn source(self) -> Expr<'a>              // o caminho ou ident como expr
    fn imports(self) -> Option<Imports<'a>>  // None = import sem ":"
    fn bare_name(self) -> Result<String, BareImportError>
    // extrai nome base: "utils.typ" → "utils", "@preview/algo:0.1.0" → "algo"
    fn new_name(self) -> Option<Ident<'a>>   // "as nome"
}

pub enum Imports<'a> { Wildcard, Items(ImportItems<'a>) }

node! { struct ImportItems }
impl ImportItems<'a> {
    fn iter(self) -> impl DoubleEndedIterator<Item = ImportItem<'a>>
}

pub enum ImportItem<'a> { Simple(ImportItemPath<'a>), Renamed(RenamedImportItem<'a>) }
impl ImportItem<'a> {
    fn path(self) -> ImportItemPath<'a>
    fn original_name(self) -> Ident<'a>
    fn bound_name(self) -> Ident<'a>    // nome local após import
}

node! { struct ImportItemPath }      // "a.b.c"
node! { struct RenamedImportItem }   // "original as novo"
impl RenamedImportItem<'a> {
    fn path(self) -> ImportItemPath<'a>
    fn original_name(self) -> Ident<'a>
    fn new_name(self) -> Ident<'a>
}

node! { struct ModuleInclude }
impl ModuleInclude<'a> { fn source(self) -> Expr<'a> }

// ── Controlo de fluxo ────────────────────────────────────────────────────────
node! { struct LoopBreak }
node! { struct LoopContinue }
node! { struct FuncReturn }
impl FuncReturn<'a> { fn body(self) -> Option<Expr<'a>> }  // None = "return" sem valor
```

---

## Critérios de Verificação

```
// Tipos existem
BareImportError::Dynamic
BareImportError::PathInvalid
BareImportError::PackageInvalid

LetBinding::from_untyped; // tipo existe
SyntaxKind::LetBinding;   // variante existe
ModuleImport::from_untyped; // tipo existe

// SetRule: selector e args tipados
// ShowRule: selector (Option) e transform

// Destructuring
DestructAssignment.pattern() → Pattern
DestructAssignment.value() → Expr

// ForLoop iterable: após "in"
// WhileLoop: condition + body
```

---

## Resultado Esperado

- `01_core/src/entities/ast/code.rs` com todos os nós documentados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/ast/code.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-26 | Criação — Passo 6: LetBinding, SetRule, ShowRule, ciclos, imports | `ast/code.rs` |
| 2026-04-12 | Restauro — prompt dedicado; interface completa, BareImportError, Imports enum, critérios | `ast/code.md` |
