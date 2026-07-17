# Prompt L0 — `sym` — módulo de símbolos Unicode
Hash do Código: 374d3514

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/sym.rs`, `01_core/src/engine/eval/mod.rs`
**Origem**: Passo 471 — módulo `sym` com tabela de ~70 símbolos prioritários; registado no scope como `Value::Dict`. **P731**: passa a `Value::Module` (paridade vanilla — medido: `type(sym)` → `module`). **P766**: expansão por uso real do corpus (tilde, integral, chevron, suit, tack, space, emptyset, bracket, amp, e variantes de plus/gt/diamond).
**ADRs**: ADR-0017, ADR-0107 (paridade linguagem vs mecânica), ADR-0029 (pureza L1).

---

## 1. Contexto

O vanilla expõe `sym` como módulo acessível via `sym.arrow`, `sym.alpha`, etc. O cristalino implementa um subset prioritário como `Value::Module` no scope global.

Divergência declarada: o vanilla suporta modificadores encadeados (`sym.arrow.r.double`). O cristalino suporta `sym.arrow.r.filled` via `Symbol::with_variants` e `Symbol::modified`; símbolos sem variantes rejeitam modifiers.

## 2. Tabela

Duas categorias:

1. **Símbolos simples** (`SYM_SIMPLE`): nome → caractere. Inclui letras gregas, operadores básicos, e variantes pré-definidas como `eq.not`.
2. **Grupos com variantes** (`SYM_GROUPS`): nome de grupo → `Symbol::with_variants`. Cada variante é `(modifiers, char)`. Exemplos: `arrow`, `tilde`, `integral`, `chevron`, `suit`, `tack`, `space`, `emptyset`, `bracket`, `amp`.

## 3. Funções

```rust
/// Procura um símbolo pelo nome (incluindo nomes compostos pré-definidos e
/// variants encadeados como "arrow.r.filled" ou "tilde.equiv").
pub fn sym_lookup(name: &str) -> Option<Symbol>;

/// Constrói o Value::Module com entradas sem ponto no scope (acessíveis via FieldAccess, P731).
pub fn build_sym_module() -> Value;
```

## 4. Registo no scope

```rust
// 01_core/src/engine/eval/mod.rs
scope.define("sym", build_sym_module());
```

Apenas entradas sem `.` no nome ficam no scope do módulo. As compostas (`"eq.not"`, `"arrow.r.filled"`) são resolvidas via `sym_lookup`.

## 5. Eval markup

`Value::Symbol(s)` em contexto de markup → `Content::Text(EcoString::from(s.ch))`.

## 6. Scope-out

- Tabela completa do vanilla (~centenas de símbolos emoji) — apenas grupos com uso identificado no corpus são implementados em P766; restantes ficam scope-out consciente.
