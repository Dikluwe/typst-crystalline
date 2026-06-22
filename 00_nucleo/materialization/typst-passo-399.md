# Passo 399 — Modelagem de Tipos: `Value::Decimal` (S)

**Tipo**: Modelagem de tipos primitivos (L1 — pureza; zero I/O; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out — operações aritméticas ricas).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Decimal` ausente, bloqueia literais decimais e precisão aritmética.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Este passo é o **primeiro da série de tipos S puros** (Decimal → Duration → Version). Ritmo rápido, zero consumer complexo.

---

## 1. Contexto

P395 abriu o portão ADR-0017 com `Value::Tiling`. P398 modelou `Value::Bytes` e ativou `read` binário. Agora o portão está aberto para os tipos primitivos restantes: `Decimal`, `Duration`, `Version`.

No vanilla:
```typ
#let x = 1.5  // Float
#let d = decimal("1.234567890123456789")  // Decimal — precisão arbitrária
#let sum = d + decimal("0.000000000000000001")
```

`Decimal` é **precisão arbitrária** (não f64). No cristalino, usamos `rust_decimal::Decimal` ou similar. Este passo é **S puro** — apenas modela o tipo e o integra no pipeline `Value`. Operações aritméticas (`+`, `-`, `*`, `/`) são passos futuros (S cada, ou agregado em um M).

---

## 2. Decisão de engenharia

### 2.1 — `Decimal` tipo L1

```rust
// entities/decimal.rs — tipo L1 puro
pub use rust_decimal::Decimal as InnerDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);
```

**Decisão crate**: `rust_decimal` é o crate standard Rust para decimal de precisão fixa (28 dígitos). É puro-Rust, no-std compatível, `Copy` (128-bit interno). Alternativa: `bigdecimal` (precisão arbitrária, `Vec` interno, não `Copy`).

**Decisão ADR-0107 (língua vs mecânica)**: a paridade linguagem é com o **valor decimal** (precisão fixa 28 dígitos), não com a mecânica interna. `rust_decimal::Decimal` é suficiente.

**Decisão ADR-0029 (pureza L1)**: `Decimal` é puro — nenhum I/O, nenhum alloc dinâmico (128-bit stack value).

### 2.2 — Enum `Value::Decimal`

```rust
// entities/value.rs — novo variant
Decimal(Decimal),
```

**Por que não Arc**: `Decimal` é `Copy` (128-bit). Sem overhead de heap.

### 2.3 — Parsing de literal decimal

No vanilla, literais decimais são criados via `decimal("...")` (função stdlib) ou via suffixo (não existe — é sempre função). No cristalino:

- `native_decimal(string: Str)` → `Value::Decimal` (parse de string).
- Cast de `Value::Float` → `Value::Decimal` (f64 → Decimal, com perda de precisão documentada).
- Cast de `Value::Int` → `Value::Decimal` (preciso).

**Não implementar literal suffixo** (ex.: `1.5dec`) — scope-out ADR-0054 graded. O vanilla não tem literal suffixo para decimal.

### 2.4 — Impacto cross-module (match exhaustivo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/value.rs` | +1 variant | `Decimal(Decimal)` |
| `eval/repr.rs` | +1 arm | `"1.234567890123456789"` (string sem notação científica) |
| `eval/cast.rs` | +1 arm | `Decimal → Decimal` (identity); `Float → Decimal`; `Int → Decimal`; `Str → Decimal` (parse) |
| `eval/ops.rs` | +1 arm | `==` por `InnerDecimal` equality; `+`/`-`/`*`/`/` — scope-out ADR-0054 graded (futuro) |
| `layout/types.rs` | Nenhum | Decimal não é Paint/Fill/Style/Length |
| `export.rs` | Nenhum | Decimal não emite direto |
| `stdlib/` | +1 func futura | `native_decimal` (P400 ou agregado) — não este passo |

**Decisão**: este passo é **modelagem pura** — não implementa `native_decimal` nem operações aritméticas. Apenas modela o tipo e o integra no enum `Value`. O constructor e as operações são passos futuros (S cada).

**Razão**: separar modelagem (S) de funcionalidade (S) mantém o ritmo. Unir tudo vira M ou M+.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `decimal.md`

Novo em `00_nucleo/prompts/entities/decimal.md`:

- **Paridade**: `Decimal` ≡ vanilla `Decimal` tipo (precisão fixa 28 dígitos).
- **Substrato**: tipo L1 puro `Decimal(InnerDecimal)`; `Copy` (128-bit); zero I/O; zero alloc.
- **Crate**: `rust_decimal` (puro-Rust, no-std, `Copy`).
- **Semântica**: precisão fixa — não f64. Operações aritméticas: scope-out ADR-0054 graded (futuro).
- **Variant `Value`**: `Decimal(Decimal)` — não Arc (Copy).
- **Cast**: `Decimal → Decimal` (identity); `Float → Decimal` (com perda); `Int → Decimal` (preciso); `Str → Decimal` (parse, fallible).
- **Repr**: string literal decimal, sem notação científica (`"1.234567890123456789"`).
- **Literal**: não existe literal suffixo — constructor via `decimal("...")` (stdlib futuro).
- **Teste**: construção `Decimal::new(12345, 2)` → `Value::Decimal` → repr → cast.

### A.2 — CHECKPOINT

Parar. Apresentar `decimal.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Decimal`

Em `01_core/src/entities/decimal.rs`:

```rust
pub use rust_decimal::Decimal as InnerDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);

impl Decimal {
    pub fn new(num: i64, scale: u32) -> Self {
        Self(InnerDecimal::new(num, scale))
    }

    pub fn from_str(s: &str) -> Option<Self> {
        InnerDecimal::from_str(s).ok().map(Self)
    }

    pub fn from_f64(f: f64) -> Option<Self> {
        InnerDecimal::from_f64(f).map(Self)
    }

    pub fn from_i64(i: i64) -> Self {
        Self(InnerDecimal::from(i))
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self(InnerDecimal::default())
    }
}
```

**Nota**: `rust_decimal` deve ser adicionado ao `Cargo.toml` de `01_core` (ou workspace). Verificar se já está presente (P387 usou crates de parsing; `rust_decimal` pode ser novo).

### B.2 — Variant `Value::Decimal`

Em `entities/value.rs`:

```rust
Decimal(Decimal),
```

Atualizar:
- `PartialEq` — match arm `Decimal(a) => matches!(other, Decimal(b) if a == b)`.
- `Repr` — `decimal.to_string()` (sem notação científica; `rust_decimal::Decimal::to_string()` já faz isso).
- `Cast` — `Decimal(d) => Ok(d)`; `Float(f) => Decimal::from_f64(*f).ok_or(...)`; `Int(i) => Ok(Decimal::from_i64(*i))`; `Str(s) => Decimal::from_str(s).ok_or(...)`.
- `type_name` — `"decimal"`.

### B.3 — Testes

1. **Unit `entities/decimal.rs`** (5-6 tests):
   - `decimal_new` — `Decimal::new(12345, 2)` → `123.45`.
   - `decimal_from_str` — `"1.2345678901234567890123456789"` → parse ok.
   - `decimal_from_str_invalid` — `"abc"` → `None`.
   - `decimal_from_f64` — `1.5` → `Decimal` (com perda documentada).
   - `decimal_from_i64` — `42` → `Decimal` preciso.
   - `decimal_equality` — `Decimal(1.0) == Decimal(1.00)` (precisão diferente, valor igual).
   - `decimal_copy` — Copy trait (sem move).

2. **Unit `entities/value.rs`** (4-5 tests):
   - `value_decimal_variant` — discriminação `Value::Decimal`.
   - `value_decimal_repr` — `"123.45"` para `Decimal::new(12345, 2)`.
   - `value_decimal_cast_identity` — `Decimal → Decimal`.
   - `value_decimal_cast_from_int` — `Int(42) → Decimal(42.0)`.
   - `value_decimal_cast_from_float` — `Float(1.5) → Decimal(1.5)`.
   - `value_decimal_cast_from_str` — `Str("3.14") → Decimal(3.14)`.
   - `value_decimal_cast_from_str_invalid` — `Str("abc") → Err`.
   - `value_decimal_partial_eq` — equality com outro Decimal.

3. **Integration** (1-2 tests, opcional):
   - `decimal_no_arithmetic_yet` — confirmar que `Value::Decimal` não participa de ops `+`/`-`/`*`/`/` (se ops.rs tiver match, adicionar braço que retorna `Err` claro: `"operações aritméticas em decimal — scope-out futuro"`).

### B.4 — Linhagem

- `@prompt` aponta para `decimal.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P395 (portão ADR-0017 aberto), P398 (Bytes — modelo de tipo S), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `native_decimal` (stdlib constructor) — é P400 (S) ou agregado com operações (M).
- **Não** implementar operações aritméticas (`+`, `-`, `*`, `/`, `%`, `pow`, etc.) — scope-out ADR-0054 graded (futuro S ou M).
- **Não** implementar comparações (`<`, `>`, `<=`, `>=`) — scope-out ADR-0054 graded (futuro S; `PartialOrd` já existe no tipo).
- **Não** implementar `decimal` como literal suffixo — não existe no vanilla.
- **Não** tocar em `Duration`/`Version` — são P400/P401.
- **Não** adicionar `Decimal` como `Length`/`Angle`/`Ratio` — não é dimensional.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `Value::Decimal(Decimal)` compila e participa de `match` exhaustivo em eval.
2. `Decimal` é `Copy`, puro-Rust, zero alloc.
3. Cast: `Int → Decimal` (preciso), `Float → Decimal` (com perda), `Str → Decimal` (parse, fallible).
4. Repr: string literal decimal, sem notação científica.
5. Zero consumer complexo; zero I/O; zero func stdlib nova.
6. Testes verdes (≥10 unit + 1-2 integration); lint zero; hashes propagados.
7. Inventário 148: `Value::Decimal` transita `ausente` → `implementado` (tipo); `decimal()` stdlib permanece `ausente` (P400).
8. L0 salvo e hashado antes do código (protocolo de nucleação).
9. Ritmo S: tempo de ciclo < 50% do tempo de P398 (baseline para tipos S puros).

---

## 7. O que pode sair errado

- **`rust_decimal` não está no Cargo.toml.** Mitigação: adicionar ao `Cargo.toml` de `01_core` (ou workspace root). Verificar versão compatível com no-std se necessário.
- **`rust_decimal::Decimal` não é `Copy` em alguma versão.** Mitigação: verificar documentação; se não for Copy, usar `Clone` + Arc (como `Value::Tiling`). Mas a documentação afirma que é `Copy` (128-bit struct).
- **`Decimal::from_f64` retorna `None` para valores especiais (NaN, Inf).** Mitigação: documentar no L0; cast de Float retorna `Err` para NaN/Inf.
- **`Decimal::to_string` usa notação científica para valores muito grandes/pequenos.** Mitigação: verificar; se sim, usar `normalize()` antes ou formatar manualmente. Mas `rust_decimal` geralmente usa notação decimal pura.
- **Tentação de já implementar `native_decimal` ou operações.** Mitigação: um passo de cada vez; este é S puro de modelagem.

---

## 8. Referências

- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- P398 — `Value::Bytes` (modelo de tipo S + consumer ativado).
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica (precisão fixa 28 dígitos).
- ADR-0029 — pureza L1.
- ADR-0054 — graded scope-out (operações aritméticas, constructor stdlib).
- `rust_decimal` crate docs — https://docs.rs/rust_decimal

---

## 9. Nota sobre o Tekt

Este passo é **S puro de modelagem** — o mais simples possível após abertura do portão ADR-0017. Não ativa consumer, não toca L3, não adiciona func stdlib. Apenas expande o enum `Value` com um tipo primitivo puro.

O ritmo de P399 → P400 → P401 deve ser **rápido e uniforme**: cada um é S puro, ~10-15 tests, zero surprises. Se algum deles virar M ou M+, o portão ADR-0017 não está funcionando como deveria — sinal de que o tipo tem dependências ocultas não mapeadas.

Registar o tempo de ciclo de P399 como **baseline de tipo S puro** — comparar com P400 (Duration) e P401 (Version) para validar ritmo.
