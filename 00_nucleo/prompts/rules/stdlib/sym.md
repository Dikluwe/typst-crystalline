# Prompt L0 — `sym` — módulo de símbolos Unicode
Hash do Código: a71656b1

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/sym.rs`, `01_core/src/rules/eval/mod.rs`
**Origem**: Passo 471 — módulo `sym` com tabela de ~70 símbolos prioritários; registered no scope como `Value::Dict`.
**ADRs**: ADR-0017, ADR-0107 (paridade linguagem vs mecânica), ADR-0029 (pureza L1).

---

## 1. Contexto

O vanilla expõe `sym` como módulo acessível via `sym.arrow`, `sym.alpha`, etc. O cristalino implementa um subset de ~70 símbolos prioritários como `Value::Dict` no scope global.

Divergência declarada: o vanilla suporta modificadores encadeados (`sym.arrow.r.double`). O cristalino usa lookup flat: `sym.arrow` → `Symbol('→')`, `sym.eq.not` disponível via `sym_lookup("eq.not")` mas não via eval FieldAccess encadeado.

## 2. Tabela

```rust
// 01_core/src/rules/stdlib/sym.rs
pub static SYM_TABLE: &[(&str, char)] = &[
    ("arrow",   '→'), ("arrow.l", '←'), ("arrow.r", '→'),
    ("arrow.t", '↑'), ("arrow.b", '↓'), ("arrow.lr", '↔'),
    ("eq",      '='), ("eq.not", '≠'),
    ("lt",      '<'), ("gt",     '>'), ("lt.eq", '≤'), ("gt.eq", '≥'),
    ("plus",    '+'), ("minus",  '−'), ("times",  '×'), ("div",   '÷'),
    // ... letras gregas α–ω ...
    ("alpha", 'α'), ("beta", 'β'), ("gamma", 'γ'), ("delta", 'δ'),
    // ... operadores matemáticos ...
    ("infinity", '∞'), ("sum", '∑'), ("product", '∏'), ("integral", '∫'),
    ("sqrt", '√'), ("in", '∈'), ("not.in", '∉'),
    // ... etc. (total ~70 entradas) ...
];
```

## 3. Funções

```rust
/// Procura um símbolo pelo nome (incluindo nomes compostos pré-definidos).
pub fn sym_lookup(name: &str) -> Option<Symbol>;

/// Constrói o Value::Dict com entradas sem ponto (acessíveis via FieldAccess).
pub fn build_sym_dict() -> Value;
```

## 4. Registo no scope

```rust
// 01_core/src/rules/eval/mod.rs
scope.define("sym", build_sym_dict());
```

Apenas entradas sem `.` no nome ficam no Dict. As compostas (`"eq.not"`) estão na `SYM_TABLE` para `sym_lookup` mas não no Dict.

## 5. Eval markup

`Value::Symbol(s)` em contexto de markup → `Content::Text(EcoString::from(s.ch))`. Arm adicionado no match de `eval_markup`.

## 6. Scope-out

- Entradas compostas acessíveis via eval (`sym.eq.not` como dois FieldAccess) — scope-out futuro.
- `sym.emoji.*` — fora deste subset.
- Tabela completa do vanilla (~centenas de símbolos) — apenas ~70 neste passo.
