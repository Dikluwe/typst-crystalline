# Relatório — Passo 299 (`P298.X — Operadores math pré-definidos`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-299.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-math-operadores-passo-299.md`
**Tipo declarado spec**: 1.º passo ortogonal pós-cluster math
P296-P298 (paralelo P293 pós-Style P288-P292); magnitude S-M
esperada; scope concreto via A.0.0'.
**Hipótese adoptada**: **HM** (vanilla macro vs cristalino Dict) +
**A.0.0' decisão P299.D agregado** (todos 42 operadores num único
`make_math_module()`) + **A.2 → (b)** scope module `math` +
**A.3 → (α) SSoT via MathOp**.
**Baseline P298**: 2 852 testes  →  **P299**: 2 862 testes (Δ = +10)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**16º passo
consecutivo**: P282→P299)
**Hash `content.rs`**: `82d3c47d` **inalterado** (P299 não cria variants)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P299 materializa **`make_math_module()`** em `structural.rs`
paralelo `make_calc_module()` (P283), registando **42 operadores
vanilla pré-definidos** como `Value::Content(Content::MathOp { ... })`
acessíveis via `math.sin`/`math.lim`/etc.

**Estrutura**:
- **31 scripts-style** (`limits: false`): `arccos`, `arcsin`,
  `arctan`, `arg`, `cos`, `cosh`, `cot`, `coth`, `csc`, `csch`,
  `ctg`, `deg`, `dim`, `exp`, `hom`, `id`, `im`, `ker`, `lg`, `ln`,
  `log`, `mod`, `sec`, `sech`, `sin`, `sinc`, `sinh`, `tan`,
  `tanh`, `tg`, `tr`.
- **11 limits-style** (`limits: true`): `det`, `gcd`, `lcm`, `inf`,
  `lim`, `liminf` (text `"lim inf"`), `limsup` (text `"lim sup"`),
  `max`, `min`, `Pr`, `sup`.

**Resultado funcional**: `#math.sin x` produz `Content::MathOp`
agregado pelo parser e renderizado via P298 handler standard.
`math.lim_(x→0) f` em block mode aplica limits-style via P298
cross-variant interaction (`attach.rs is_limits`).

**Resultado metodológico — magnitude baixa-modesta refuta sequência
de altas**: A.0.0 N=7 (7.ª reaplicação consecutiva) revelou que
cristalino tem `calc` module precedente claro (P283) — `math`
module ausente é gap previsível. **Magnitude baixa-modesta** (paralelo
P295) interrompe sequência 2 consecutivas altas (P297+P298).
Tendência **não-monotonicamente decrescente** — flutuação saudável
preserva validade do template §8.7'.

**1.ª aplicação prática `Content::MathOp`** pós-materialização P298 —
confirma variant como **single source of truth** para operadores math.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=7 reaplica §8.7') | Cristalino tem `calc` precedente; `math` module ausente; vanilla `ops!` macro Rust-específica não replicável; **magnitude baixa-modesta** |
| A.0.0' (subdivision decision, inaugural) | **P299.D agregado** (42 operadores num só passo) — paralelo P293 A.0.0 |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact (16º passo) |
| A.1 inventário | 7 limits cobertos hardcoded; 0 scripts explícitos; vanilla `op.rs` lista 42 canónicos |
| A.2 decisão | **(b) scope module `math`** paralelo `make_calc_module()` |
| A.3 integração | **(α) SSoT via MathOp** — cada operador é `Value::Content(MathOp{...})` |
| A.4 emit | **(i) `FrameItem` standard**; hash preservado |
| A.5 bugs latentes | 5 cenários verificados; nenhum bug; regressão `MathIdent` preservada |
| A.5' anti-reflexão | **N=8 cumulativo** (P291-P299); 4 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-math-operadores-passo-299.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/stdlib/structural.rs` (+`make_math_module()`)

```rust
fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::MathOp {
        text:   Box::new(Content::text(text)),
        limits,
    })
}

/// Constrói o módulo `math` como `Value::Dict` com 42 operadores
/// vanilla pré-definidos (paralelo `make_calc_module()` P283).
pub fn make_math_module() -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    use ecow::EcoString;
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    // Scripts-style operators (31) — limits: false.
    for name in [
        "arccos", "arcsin", "arctan", "arg",
        "cos", "cosh", "cot", "coth",
        "csc", "csch", "ctg", "deg",
        "dim", "exp", "hom", "id",
        "im", "ker", "lg", "ln",
        "log", "mod", "sec", "sech",
        "sin", "sinc", "sinh", "tan",
        "tanh", "tg", "tr",
    ] {
        dict.insert(name.into(), op_value(name, false));
    }

    // Limits-style operators (11) — limits: true. Multi-word usam
    // text literal vanilla (e.g. `liminf` → "lim inf").
    for (name, text) in [
        ("det", "det"), ("gcd", "gcd"), ("lcm", "lcm"),
        ("inf", "inf"), ("lim", "lim"),
        ("liminf", "lim inf"), ("limsup", "lim sup"),
        ("max", "max"), ("min", "min"),
        ("Pr", "Pr"), ("sup", "sup"),
    ] {
        dict.insert(name.into(), op_value(text, true));
    }

    Value::Dict(dict)
}
```

### §3.2 — Re-export + registo

`01_core/src/rules/stdlib/mod.rs:45`:

```rust
pub use crate::rules::stdlib::structural::{
    make_math_module, ...
};
```

`01_core/src/rules/eval/mod.rs:768`:

```rust
// P299 — `math.sin`/`math.lim`/etc. (P298.X; 42 operadores
// pré-definidos paridade vanilla via SSoT MathOp).
scope.define("math",     make_math_module());
```

### §3.3 — Zero alterações em outros sítios

| Componente | Pós-P299 |
|---|---|
| `01_core/src/entities/content.rs` | **Inalterado** — P299 não cria variants; hash `82d3c47d` preservado |
| `01_core/src/rules/layout/` | **Inalterado** — handlers P296-P298 consumem MathOp sem alteração |
| `01_core/src/rules/math/layout/attach.rs` | **Inalterado** — heurística pré-P299 preservada |
| `01_core/src/rules/math/symbols.rs` | **Inalterado** — `is_limit_function` hardcoded preservado (fallback) |
| `03_infra/src/export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (16º passo) |

---

## §4 — Testes

### §4.1 — `01_core/src/rules/stdlib/mod.rs` (+10 testes L1)

| Teste | Verifica |
|---|---|
| `p299_math_module_contem_sin_scripts_style` | `math.sin` existe; limits=false |
| `p299_math_module_contem_lim_limits_style` | `math.lim` existe; limits=true |
| `p299_math_module_contem_det_limits_style_novo` | `det` (não coberto pré-P299) agora explícito limits-style |
| **`p299_math_module_contem_liminf_multi_word_text`** | `liminf` mapeia para text `"lim inf"` |
| **`p299_math_module_contem_limsup_multi_word_text`** | `limsup` mapeia para text `"lim sup"` |
| **`p299_math_module_total_42_operadores`** | confirma 31 scripts + 11 limits = 42 entries |
| `p299_math_module_todos_scripts_style_limits_false` | amostra: sin/cos/tan/ln/log/exp/arccos todos scripts-style |
| `p299_math_module_pr_case_sensitive` | `Pr` (capitalizado) existe; `pr` (minúsculo) NÃO |
| `p299_math_module_nome_inexistente_retorna_none` | Robustez lookup |
| **`p299_regressao_math_ident_lim_preservado_pre_p299`** | **INVARIANTE CRÍTICA**: fallback `MathIdent("lim")` preservado |

### §4.2 — Sem alterações em L3

P299 não modifica nada em L3 (emit agnóstico via P298). Sem testes
L3 dedicados — testes pré-existentes math (incluindo P298 L3)
preservados bit-exact.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2358 passed; 0 failed; 0 ignored
test result: ok.  457 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2862 passed total
```

Baseline P298 = 2 852; delta = +10 (10 L1, 0 L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P299

| Ficheiro L0 / código | Antes P299 | Pós P299 |
|---|---|---|
| `entities/content.md` | propagado P298 | **inalterado** (P299 não cria variants) |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` | **`82d3c47d`** inalterado |
| `rules/stdlib.md` | inalterado | inalterado (política única) |
| `infra/export.md` | `31a37c57` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**16º passo consecutivo**) |

`crystalline-lint --fix-hashes`: "Nothing to fix" — confirma
ausência de drift.

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — magnitude baixa-modesta interrompe consecutivas altas

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta |
| P294 | 2 | máxima |
| P295 | 3 | baixa |
| P296 | 4 | média |
| P297 | 5 | alta |
| P298 | 6 | alta |
| **P299** | **7** | **baixa-modesta** |

**Janela P294-P299**: máxima, baixa, média, alta, alta, **baixa**.

**Sequência 2-consecutivas altas (P297+P298) interrompida por P299
baixa**. Spec P298 §9 antecipou: *"Se P299 reaplicar com refutação
alta de novo, atinge 3 magnitudes altas consecutivas — §8.7' N=7
promoção altamente robusta candidata (raríssimas circunstâncias
para adiamento)"*. **Não atingido** — magnitude baixa.

**Decisão**: §8.7' N=7 **adiado** honestamente. Promoção só se
gatilho **inequivocamente** disparar (anti-padrão P273.17 §0
honrado).

**Tendência**: não-monotonicamente decrescente; flutuação saudável.
Template §8.7' continua viável sem necessidade de descoberta a
cada passo — **"confirmação esperada" categoria honesta** (P295
§10 inaugural) aplicável a P299.

### §6.2 — §8.3 "refutação pragmática" (**N=11 candidato adiado**)

Refutação modesta (gap previsível). Mesma razão de adiamento.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=9 cumulativo**)

P291-P299. Limiar passado; anti-padrão adia.

### §6.4 — Sub-padrão "module namespaced" (**N=2 cumulativo**)

P283 inaugurou `make_calc_module()` (calc). P299 reaplica com
`make_math_module()` (math). **N=2 cumulativo** — emergente,
longe de limiar tentativo N≥3.

Cristalino agora tem **3 modules namespaced**: `calc` (P283),
`gradient` (P262), `math` (P299).

### §6.5 — Sub-padrão "operadores pré-definidos via SSoT" (**N=1 inaugural**)

P299 inaugura: 42 operadores registados via SSoT `Content::MathOp`.
Pattern novo — 1.ª aplicação prática de variant existente para
finalidade arquitectural diferente (registo em massa vs criação
on-demand).

Sub-padrão **N=1 inaugural** — longe de limiar.

### §6.6 — "Variant rico" N=5 — não aplicável

P299 **não cria variants novos** (paralelo arquitectural P294 —
materialização sem ampliar enum). Gatilho não aplicável.

### §6.7 — ADR-0098 "single source of truth" (**N=16 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 16º passo consecutivo
(P282→P299). Invariante robusta sobre 16 features distintas:
Style P288-P292, Curve P293/P294, Footnote P295, Math
accent/cancel P296, Math underover P297, Math op P298, **Math
operadores pré-definidos P299** (sem efeito em emit — registo
stdlib via SSoT).

### §6.8 — Anti-padrão over-formalização rigorosamente honrado

Avaliados 4 candidatos:
1. §8.7' A.0.0 template N=7 — magnitude baixa adiou.
2. §8.3 refutação pragmática N=11 — adiado.
3. Sub-padrão "module namespaced" N=2 — emergente, longe de limiar.
4. Sub-padrão "operadores SSoT" N=1 — inaugural, longe de limiar.

**0 ADRs meta promovidas** — P273.17 §0 honrado; promoção só se
gatilho inequívoco.

---

## §7 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 121** (`op`): nota actualizada para incluir P299:
  `make_math_module()` paralelo `make_calc_module()`; 42 operadores
  vanilla pré-definidos acessíveis via `math.sin`/`math.lim`/etc.;
  divergência consciente per ADR-0054 graded (vanilla auto-lookup em
  math mode fora de scope P299).

---

## §8 — Frentes pendentes pós-P299

| Frente | Tipo | Estado |
|---|---|---|
| **Auto-lookup math mode** (`$sin x$` sem prefix `math.`) | M | Frente independente; requer modificação parser/eval math mode |
| **P297.X** discriminator `UnderoverKind` | XS+ | Cosmético |
| **P296.X** toggles `inverted`/`cross` cancel | XS | Toggles binary |
| Cosméticos `OpElem` (`size`/etc.) | XS | ADR-0054 graded |

---

## §9 — Decisão sobre P300

**P299 fechou frente cluster math derivada** (P298.X). P300 deve
ser **ortogonal** — paralelo P293 (1.º ortogonal pós-Style).

Candidatos disponíveis:
1. **Auto-lookup math mode** — fecha integração natural P299.
2. **Frentes não-math** — P295.1 nota rodapé; `Length` em `Stroke`;
   `curve.move`/scope-methods; outras P282 §6.
3. **Frente nova** distinta de math.

Decisão fica para o operador humano. **P299 não dita P300**.

**P300 é um marco numérico** — pode ser oportunidade para
**consolidação** dos padrões cumulativos sem materialização nova
(retrospectiva metodológica).

---

## §10 — Honestidade epistémica

P299 valida a **flutuação saudável** do template §8.7':

| Janela | Magnitudes | Tendência |
|---|---|---|
| P293-P295 (inaugural+2) | alta, máxima, baixa | decrescente inicial |
| P295-P298 (4 passos) | baixa, média, alta, alta | **crescente** |
| **P297-P299 (3 últimos)** | **alta, alta, baixa** | **flutuação** |

A flutuação **não é degenerescência** — template continua a gerar
valor:
- **P299 baixa-modesta** evitou implementação cega assumindo
  cristalino sem precedente (`calc` module existia, paralelo
  directo).
- **A.0.0' subdivision decision** preveniu fragmentação artificial
  (P299.A/B separados) em favor de P299.D agregado coerente.

**Lição metodológica**: A.0.0 produz valor mesmo em magnitude
baixa-modesta quando a informação confirmada **guia a decisão
arquitectural** (adoptar pattern existente vs criar novo).

**Anti-padrão honrado**: 4 candidatos a promoção avaliados; **0
promovidos** porque nenhum dispara inequivocamente.

---

## §11 — Fecho

P299 fechado com:

- **+10 testes** (10 L1, 0 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** bit-exact (16º passo consecutivo).
- **Hash `content.rs` inalterado** — confirma natureza ortogonal pura
  (sem ampliar enum).
- **0 ADRs meta novas** — 4 candidatos avaliados e adiados (§8.7' N=7
  magnitude baixa não justifica; sub-padrões N=2/N=1 longe de
  limiar).

**MARCO P299**:
- **1.º passo ortogonal pós-cluster math** P296-P298 — paralelo
  arquitectural P293 (1.º ortogonal pós-Style P288-P292).
- **A.0.0 N=7 magnitude baixa-modesta** — interrompe sequência 2
  consecutivas altas (P297+P298); flutuação **saudável**.
- **A.0.0' subdivision decision** inaugural — pattern metodológico
  novo emergente.
- **1.ª aplicação prática `Content::MathOp`** pós-materialização
  P298 — confirma SSoT.
- **`make_math_module()` paralelo `make_calc_module()`** —
  sub-padrão "module namespaced" N=2 cumulativo (calc + math).
- **42 operadores vanilla registados** via P299.D agregado —
  paridade vanilla literal (31 scripts + 11 limits).
- **Hash `export.rs` preservado pelo 16º passo consecutivo**
  (P282→P299) — ADR-0098 robusta sobre 16 features distintas.
- **Hash `content.rs` inalterado** — P299 puramente stdlib;
  ortogonalidade arquitectural confirmada.
- **Anti-padrão over-formalização rigorosamente honrado** — 4
  candidatos avaliados; 0 promovidos.
- **Template §8.7' valida flutuação saudável** — magnitude baixa
  após 2 altas NÃO é degenerescência (lição metodológica).
- **Frente P298.X resolvida** — fecha a frente pendente principal
  do cluster math.
