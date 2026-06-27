# Prompt L0 — `Symbol` — símbolo Unicode nomeado
Hash do Código: b21b38a5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/symbol.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 471 — `Value::Symbol` subset minimal (S); terceiro tipo simples do portão ADR-0017 em Trilha 8.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (modificadores scope-out).

---

## 1. Contexto

O Typst vanilla expõe `Symbol` como tipo de runtime para caracteres simbólicos acessíveis via notação de ponto (`sym.arrow.r`, `sym.eq.not`). O cristalino não tinha `Value::Symbol` nem módulo `sym`.

Divergência declarada: o vanilla suporta modificadores encadeados (`sym.arrow.r.double`) via `Modifier` struct. O cristalino implementa apenas nomes simples e compostos pré-definidos como chaves planas. Modificadores encadeados são scope-out deste passo.

## 2. Tipo L1

```rust
// 01_core/src/entities/symbol.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub ch:   char,
    pub name: EcoString,
}

impl Symbol {
    pub fn new(ch: char, name: impl Into<EcoString>) -> Self;
}
```

- `char` Unicode: o glyph concreto.
- `EcoString` nome canónico: clone O(1).
- Sem `Modifier` neste passo (scope-out).

## 3. Variant `Value::Symbol`

```rust
// 01_core/src/entities/value.rs
Symbol(crate::entities::symbol::Symbol),
```

Actualizações necessárias:
- `type_name()` → `"symbol"`.
- `repr()` (em `eval/repr.rs`) → o char como string (`"→"`).
- `From<Symbol> for Value`.
- `Hash` via `format!("{:?}", self)` (já implementado).

## 4. Conversão em markup

Quando `Value::Symbol(s)` aparece em contexto de markup, o eval converte em `Content::Text(EcoString::from(s.ch))`. Arm adicionado em `eval_markup` no match de `eval_expr`.

## 5. Scope-out

- **Modificadores encadeados** (`sym.arrow.r.double`) — requer `Modifier` struct. Futuro. Divergência declarada.
- **Acesso `sym.eq.not` via eval** — `sym.eq` retorna `Symbol`, `.not` falha (dois FieldAccess). O L1 `sym_lookup("eq.not")` funciona directamente.
- **`Value::Symbol` em `match` de `StyleChain`** — sem impacto em set rules neste passo.
