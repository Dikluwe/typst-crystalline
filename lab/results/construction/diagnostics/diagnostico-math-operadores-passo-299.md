# Diagnóstico — Fase A do Passo 299 (`P298.X — Operadores math pré-definidos`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-299.md`
**Origem**: P298 §8 frente pendente; 1.º ortogonal pós-cluster math
P296-P298 (paralelo P293 pós-Style P288-P292).
**Tipo declarado spec**: magnitude S-M esperada; scope concreto
condicional a A.0.0.
**Hipótese adoptada**: **HM (vanilla macro vs cristalino Dict)** +
**A.0.0' decisão P299.D agregado (41 operadores)** + **A.2 → (b)
scope module `math`** + **A.3 → (α) SSoT via MathOp**.
**Magnitude da refutação A.0.0 N=7**: **baixa-modesta**
(clarificação factual paralela P295) — cristalino tem `calc` module
precedente claro; `math` module ausente; vanilla macro `ops!` é
abordagem Rust-específica não replicável.

---

## A.0.0 — Verificação literal estado actual operadores (N=7 §8.7')

### A.0.0.1 — Heurística limits-style actual cristalino

`01_core/src/engine/math/symbols.rs:170` (P50/P298 herdado):

```rust
pub fn is_limit_function(s: &str) -> bool {
    matches!(s, "lim" | "max" | "min" | "sup" | "inf" | "limsup" | "liminf")
}
```

**7 strings** hardcoded. `is_large_operator` cobre 18 caracteres
Unicode (`∑`, `∏`, `∫`, etc.).

### A.0.0.2 — Lista completa vanilla `op.rs` macro `ops!`

`lab/.../math/op.rs:62-105` define **41 operadores**:

**Scripts-style (29)**: `arccos`, `arcsin`, `arctan`, `arg`, `cos`,
`cosh`, `cot`, `coth`, `csc`, `csch`, `ctg`, `deg`, `dim`, `exp`,
`hom`, `id`, `im`, `ker`, `lg`, `ln`, `log`, `mod`, `sec`, `sech`,
`sin`, `sinc`, `sinh`, `tan`, `tanh`, `tg`, `tr`.

**Limits-style (12)**: `det`, `gcd`, `lcm`, `inf`, `lim`,
`liminf` (text `"lim inf"`), `limsup` (text `"lim sup"`), `max`,
`min`, `Pr`, `sup`.

**Cobertura cristalino pré-P299**: heurística `is_limit_function`
cobre **7 dos 12 limits-style** (`lim`, `max`, `min`, `sup`, `inf`,
`limsup`, `liminf`). **5 ausentes**: `det`, `gcd`, `lcm`, `Pr`.

Scripts-style: **nenhum cobertura explícita** — `MathIdent("sin")`
renderiza como text simples sem semântica scripts especial
(default attach é scripts-style, mas é fallback genérico, não
registo).

### A.0.0.3 — Scope `math` module cristalino: AUSENTE

```text
grep "make_math_module\|scope.define(\"math\"" 01_core/src/  → 0 hits
```

Cristalino **não tem** scope `math` module. Tem `calc` module
(`make_calc_module()` paralelo) registado como
`scope.define("calc", ...)`.

### A.0.0.4 — `calc` module pattern (precedente directo)

`01_core/src/engine/stdlib/calc.rs:35`:

```rust
pub fn make_calc_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("abs".into(), Value::Func(Func::native("calc.abs", calc_abs)));
    // ... ~30 funcs trig/log/exp/constantes
}
```

P299 adopta padrão paralelo: `make_math_module()` + registo
`scope.define("math", ...)`.

### A.0.0.5 — Magnitude da refutação

| Aspecto | Veredicto |
|---|---|
| Spec antecipou cristalino sem operadores pré-definidos | ✅ Confirmado (HJ parcial) |
| Spec antecipou heurística cobre tudo | ❌ Refutado parcialmente — 7/12 limits, 0/29 scripts |
| Spec antecipou vanilla macro vs cristalino Dict | ✅ Confirmado (HM) |

**Magnitude**: **baixa-modesta** — paralelo P295. Clarificação
factual mas sem refutação de toda a estrutura. **Não é alta**
como P297/P298.

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
Tendência flutuante mas **não monotonicamente decrescente** —
template §8.7' continua viável.

**Decisão sobre promoção §8.7' N=7**: spec sugeriu "promover se 3
magnitudes altas consecutivas" (P297+P298+P299). **Não atingido** —
P299 modesta interrompe sequência. **Adiar** promoção; preservar
honestidade.

---

## A.0.0' — Decisão de subdivisão (paralelo P293 A.0.0)

### A.0.0'.1 — Avaliação dos subsets

| Subset | Conteúdo | Magnitude | Decisão |
|---|---|---|---|
| **P299.A** scripts apenas | 29 funcs | S | Não escolhido — fragmenta artificialmente |
| **P299.B** limits-style complementar | 5 funcs (det/gcd/lcm/Pr + multi-word) | XS | Não escolhido — minimal demais |
| **P299.A+B** agregado | 41 - duplicates | S-M | **NÃO escolhido** — `lim`/`max`/etc. já via heurística |
| **P299.D** total agregado | 41 todos | M | **ESCOLHIDO** — coerência arquitectural |
| **P299.C** scope module separado | n/a (incluído em P299.D via make_math_module) | — | Integrado em P299.D |

### A.0.0'.2 — Justificação P299.D agregado

- **Padrão repetitivo trivial**: 41 `dict.insert(name, MathOp{text, limits})`
  é magnitude controlada via IndexMap.
- **Lista canónica vanilla bem definida**: 41 entries sem ambiguidade.
- **Single registo num só sítio**: `make_math_module()` →
  `scope.define("math", ...)`.
- **Sem regressão**: heurística `is_limit_function` permanece —
  `MathIdent("lim")` continua a funcionar; `math.lim` é alternativa.

### A.0.0'.3 — Acesso user-facing

Em vanilla typst dentro de `$...$`, identifiers resolvem contra
`math` scope automaticamente (`$sin x$` funciona). Cristalino
**não tem** esse lookup automático — user usa **acesso explícito**:

```typst
$math.sin x$       // Funciona via lookup namespaced
$math.lim_(x→0) x$ // Funciona; lim tem limits=true em block mode
$sin x$            // Continua a funcionar via MathIdent literal + fallback heurístico
```

**Divergência consciente per ADR-0054 graded**: cristalino requer
prefix `math.`. Refino futuro (auto-lookup em math mode) é frente
independente P299.X candidato.

---

## A.0 — Reuso ADR-0098

| Verificação | Esperado |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**16º passo consecutivo**) |
| `FrameItem::Text` suficiente | ✅ paralelo P298 — MathOp text emite standard |
| ADR-0098 N=16 cumulativo | ✅ |

---

## A.1 — Inventário literal

### A.1.1 — Operadores cobertos cristalino pré-P299

- 7 limits-style via `is_limit_function`.
- 18 caracteres Unicode via `is_large_operator`.
- 0 scripts-style explícitos.

### A.1.2 — Operadores vanilla (lista canónica)

41 operadores (vide A.0.0.2). 5 limits-style ausentes em cristalino;
29 scripts-style sem registo.

### A.1.3 — Scope `math` ausente

Vide A.0.0.3. P299 cria.

### A.1.4 — `MathIdent("sin")` comportamento actual

Renderiza como text simples via `layout_node` fallback. Sem
semântica "scripts-style" explícita (mas default attach é
scripts-style, então `MathIdent("sin")` + attach `^2` funciona
correctamente).

### A.1.5 — `native_op` pós-P298

Assinatura: `native_op(text, limits: bool = false) → Content::MathOp`.
P299 reusa internamente para construir operadores pré-definidos.

### A.1.6 — Vanilla `math::op::sin`

Vanilla typst scope math: `math.sin x` ou `$sin x$` (auto-lookup).
Cristalino: `math.sin x` (lookup explícito).

### A.1.7 — Emit standard

`FrameItem::Text/Glyph` paralelo P298. Sem alteração.

### A.1.8 — Diagrama de fluxo P299

```
$math.sin x$
       │
       ▼
parse → field access math.sin → Value::Content(MathOp{text:"sin", limits:false})
       │
       ▼
Content::MathOp { text: "sin", limits: false }
       │
       ▼
layout_node (P298 layout_op trivial delegate)
       │
       ▼
FrameItem::Text standard
       │
       ▼
PDF "sin" inline + attach se aplicável
```

### A.1.9 — Paradigma consumer P299

**Single source of truth via MathOp** — todos os operadores
pré-definidos retornam `Content::MathOp`. **1ª aplicação prática
de `MathOp`** pós-materialização P298 — confirma variant como ponto
único de produção.

---

## A.2 — Decisão arquitectural (registo)

**Decisão A.2 → (b) scope module `math`**:

```rust
pub fn make_math_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    // Scripts-style (29)
    dict.insert("sin".into(),  op_value("sin", false));
    dict.insert("cos".into(),  op_value("cos", false));
    // ... etc
    // Limits-style (12)
    dict.insert("lim".into(),  op_value("lim", true));
    dict.insert("max".into(),  op_value("max", true));
    // ... etc
    Value::Dict(dict)
}

fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::MathOp {
        text:   Box::new(Content::text(text)),
        limits,
    })
}
```

Paralelo directo `make_calc_module()` (P283).

**Decisão NÃO escolhe (a)**: scope global poluído com 41 nomes
genéricos colide com user namespace. **Decisão NÃO escolhe (c)
tabela estática**: requer lookup runtime e API extra.

---

## A.3 — Integração SSoT via MathOp (decisão α)

**Decisão A.3 → (α)**: cada operador pré-definido é
`Value::Content(Content::MathOp { ... })` — **single source of truth**.

Layouter consume via P298 paradigm (`layout_op` trivial delegate +
`attach.rs is_limits` cross-variant).

**Padrão "operadores pré-definidos via SSoT"** inaugurado N=1.
Longe de limiar tentativo para promoção ADR meta.

---

## A.4 — Impacto em emit (preservado bit-exact)

**Decisão A.4 → (i)** — `FrameItem::Text` standard. Hash `export.rs`
preservado bit-exact pelo **16º passo consecutivo**.

---

## A.5 — Detecção de bugs latentes

5 cenários fronteira:

| Cenário | Resultado esperado |
|---|---|
| `$math.sin x$` | renderiza "sin x" inline (scripts-style attach se houver) |
| `$math.lim_(x→0) f$` | renderiza limits-style em block via P298 cross-variant |
| `$lim_(x→0) f$` (sem `math.`) | continua a funcionar via fallback `MathIdent("lim")` + heurística |
| `$math.sin$` standalone | só "sin" |
| `$math.det X$` | renderiza limits-style em block (det tem limits=true vanilla) |

### A.5.1 — Sem bugs latentes esperados

Padrão §8.4 N=1 estável. Regressão `MathIdent` paths preservada
bit-exact (sem alteração de heurística).

---

## A.5' — Anti-reflexão N=8 cumulativo

### A.5'.1 — Comparação A.1.9 P288-P299

| Passo | Tipo | Paradigma consumer |
|---|---|---|
| P288-P292 | cumulativo | 5 paradigmas style/text |
| P293-P296 | ortogonais | 4 paradigmas distintos |
| P297-P298 | extensões cluster math | underover + op |
| **P299** | **ortogonal pós-cluster** | **1ª aplicação prática MathOp via stdlib registo** |

P299 inaugura sub-padrão **"operadores pré-definidos via SSoT"**.
Não é paradigma genuinamente novo (reusa MathOp P298) mas é **1ª
aplicação prática** de variant existente para finalidade
arquitectural nova (registo em massa).

### A.5'.2 — A.0.0 N=7 reaplicação — magnitude baixa-modesta

P299 interrompe sequência 2-consecutivas altas (P297+P298). Janela:
máxima, baixa, média, alta, alta, **baixa**. **Tendência variada
saudável** — não-monotónica preserva validade do template.

### A.5'.3 — Elementos estructuralmente novos identificados

4 elementos:

1. **1ª aplicação prática `Content::MathOp`** — primeiro uso
   concreto pós-materialização P298. Confirma SSoT.
2. **`make_math_module()` paralelo `make_calc_module()`** —
   2.º módulo namespaced cristalino; sub-padrão "module
   namespaced" N=2 cumulativo.
3. **A.0.0' subdivision decision section** — nova; primeira
   spec com decisão explícita pós-A.0.0.
4. **Magnitude baixa-modesta pós-2-altas** — flutuação confirma
   template viável sem necessidade de descoberta a cada passo.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos:
- **§8.7' N=7** — magnitude baixa NÃO justifica promoção; **adiado**.
- **§8.3 N=11** candidato adiado.
- **Sub-padrão "operadores pré-definidos via SSoT"** N=1 inaugural.
- **Sub-padrão "module namespaced"** N=2 (calc P283 + math P299).
- **"Variant rico" N=5** — não aplicável (P299 não cria variants).

**Decisão**: **0 ADRs meta novas**. Razões:
- §8.7' N=7 critério "magnitude alta" não atingido (P299 baixa).
- Anti-padrão over-formalização P273.17 §0 rigoroso.
- Promoção é **inequívoca**, não automática.

§A.5'.4 spec P299 sugeriu "promover se gatilho dispara
inequivocamente". P299 A.0.0 modesta — gatilho **NÃO dispara
inequivocamente**. Adiar é honesto.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P299 |
|---|---:|---:|
| `Content` variants | 69 | 69 (inalterado — P299 não cria variants) |
| Stdlib módulos namespaced | 2 (calc, gradient) | **3** (+math) |
| Operadores math pré-definidos registados | 0 | **41** |
| Hash L0 `content.md` | propagado | inalterado (P299 não toca) |
| Hash L0 `stdlib.md` | inalterado | inalterado (política única) |
| Hash `export.rs` | `66cb8ac3` | **preservado bit-exact** (**16º passo consecutivo**) |
| Padrão §8.6 A.5' N | 8 | **9** (P291-P299) |
| Padrão §8.7' A.0.0 N | 6 | **7** reaplica magnitude baixa |
| Padrão §8.3 N candidato | 10 | 11 candidato adiado |
| Sub-padrão "module namespaced" N | 1 (calc P283) | **2** (calc + math) |
| Sub-padrão "operadores SSoT" N | n/a | **1 inaugural** |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (regressão MathIdent): ✅ heurística pré-P299
  preservada; testes regression específicos.
- **Risco secundário** (subdivisão A.0.0' provisória): ✅ resolvido
  via P299.D agregado.
- **Risco terciário** (§8.7' N=7 promoção forçada): ✅ magnitude
  baixa adiou conservador.
- **Risco quaternário** (scope global vs `math` conflito): ✅
  resolvido via (b) scope module namespaced.
- **Risco quinário** (sequência reflexa subdivisões): ✅ P299.D
  agregado não fragmenta.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=7 reaplica §8.7' com
magnitude baixa-modesta** + **A.0.0' decisão P299.D agregado** +
decisão **(b) scope module `math` + (α) SSoT via MathOp + (i) emit
preservado** + A.5 sem bugs + **A.5' N=8 cumulativo com 4 elementos
novos**. **0 ADRs meta novas** — §8.7' N=7 magnitude baixa não
justifica.

**MARCO P299**:
- **1.º passo ortogonal pós-cluster math** P296-P298 (paralelo P293
  pós-Style P288-P292).
- **A.0.0 N=7 magnitude baixa-modesta** — interrompe sequência 2
  consecutivas altas (P297+P298); flutuação **saudável** confirma
  template viável.
- **1.ª aplicação prática `Content::MathOp`** pós-materialização
  P298 — confirma SSoT.
- **`make_math_module()` paralelo `make_calc_module()`** —
  sub-padrão "module namespaced" N=2 cumulativo (calc + math).
- **41 operadores vanilla registados** via P299.D agregado.
- **Hash `export.rs` preservado pelo 16º passo consecutivo** —
  ADR-0098 robusta sobre 16 features distintas.
- **Anti-padrão over-formalização rigorosamente honrado** — §8.7'
  N=7 candidato mas magnitude baixa adia.

Procede-se a §3 da spec (com plano P299.D + (b) + (α)).
