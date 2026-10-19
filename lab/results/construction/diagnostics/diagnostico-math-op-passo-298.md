# Diagnóstico — Fase A do Passo 298 (`P296.2 — MathOp`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-298.md`
**Origem**: P296 §8 + P297 §8 frente pendente.
**Tipo declarado spec**: 3ª aplicação "cluster math handler
dedicado"; magnitude XS-S esperada; A.0.0 podia revelar HV''.
**Hipótese adoptada**: **HV'' (heurística limits-style já existe
em `attach.rs`)** + **A.2 → (b) `Box<Content>` + bool**.
**Magnitude da refutação A.0.0 N=6**: **alta (factual-significativa)** —
descoberta de mecanismo limits existente em `symbols::is_limit_function`.

---

## A.0.0 — Verificação literal estado actual (N=6 §8.7')

### A.0.0.1 — Inspecção cristalino L1

```text
grep -rin "MathOp\|OpElem\|native_op" 01_core/src/ → 0 hits funcionais
```

`MathOp` confirmado ausente. Stdlib `native_op` ausente.

### A.0.0.2 — Inspecção crítica: heurística limits-style já existe

**Descoberta significativa** em `01_core/src/engine/math/layout/attach.rs:55-61`:

```rust
let is_limits = self.block && match base {
    Content::MathIdent(s) | Content::MathText(s) => {
        let ch = s.chars().next().unwrap_or('\0');
        symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
    }
    _ => false,
};
```

E em `symbols.rs:170`:

```rust
pub fn is_limit_function(s: &str) -> bool {
    matches!(s, "lim" | "max" | "min" | "sup" | "inf" | "limsup" | "liminf")
}
```

**Status real pré-P298**:
- Cristalino **já tem** detecção limits-style heuristica para
  operadores `MathIdent`/`MathText` (Σ, ∫, lim, max, etc.).
- Falta **mecanismo user-facing `op()`** para criar operators
  custom com `limits: bool` explícito (paridade vanilla).

**Refutação spec P298**: §1.2 antecipou cristalino **sem
heurística limits-style** ("se cristalino trata `lim` via
MathIdent simples"). **Refutação parcial**: cristalino TEM
heurística limits, mas hardcoded — falta mecanismo
user-customizable.

### A.0.0.3 — Inspecção vanilla `OpElem`

`lab/.../math/op.rs:24-32`:

```rust
#[elem(title = "Text Operator", Mathy)]
pub struct OpElem {
    #[required] pub text: Content,
    #[default(false)] pub limits: bool,
}
```

**Diferença factual vs spec**: spec P298 §1.2 antecipou
`text: EcoString`; vanilla usa `text: Content`. Refutação modesta
sobre tipo (não estrutural).

Vanilla também define 36+ operadores pré-definidos no scope math
(arccos, sin, cos, lim, sup, max, etc.) — fora do scope P298 per
§5 não-objectivo (P298.X candidato).

### A.0.0.4 — Magnitude da refutação A.0.0 N=6

| Aspecto refutado | Magnitude |
|---|---|
| Spec §1.2 "cristalino sem heurística limits-style" | Refutado — heurística existe parcial |
| Spec §1.2 `text: EcoString` | Refutado — vanilla é `Content` |
| Toda a estrutura proposta da spec | **Não** refutada — plano P298 procede adaptado |

**Magnitude: alta (factual-significativa)** — paralelo P296/P297
mas não máxima (paralelo P294). Descoberta significativa
mecanismo existente que precisa extensão.

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta |
| P294 | 2 | máxima |
| P295 | 3 | baixa |
| P296 | 4 | média |
| P297 | 5 | alta |
| **P298** | **6** | **alta** |

**Janela P294-P298** magnitudes: máxima, baixa, média, alta, **alta**.
Trend **alta sustentada** nos últimos 2 — tendência não-decrescente
robusta. §6.6 P295 degenerescência **definitivamente refutada**.

### A.0.0.5 — Decisão HV'' adaptada

**Plano materialização adaptado**:
1. `MathOp { text: Box<Content>, limits: bool }` (paridade vanilla
   literal `Content` em vez de `EcoString`).
2. `native_op(text, limits: bool = false)` stdlib.
3. `layout_op` handler em `math/layout/mod.rs` — delega para
   `layout_node(text, style)` (text emite como child padrão).
4. **Modificação crítica em `attach.rs:55-61`**: adicionar arm
   `Content::MathOp { limits, .. }` → `*limits`. Mínimo refactor.
5. `is_limit_function` hardcoded **preservado** — fallback para
   `MathIdent` literal (`lim`/`max`/etc. continuam a funcionar
   sem necessidade de `op()`).

---

## A.0 — Reuso ADR-0098

| Verificação | Esperado |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**15º passo consecutivo**) |
| `FrameItem` standard para op text | ✅ paralelo MathIdent/MathText |
| ADR-0098 N=15 cumulativo | ✅ |

---

## A.1 — Inventário literal

### A.1.1 — `Content::Math*` variants pós-P297 (13)

`MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`,
`MathRoot`, `MathDelimited`, `MathAlignPoint`, `MathMatrix`,
`MathCases`, `MathAccent`, `MathCancel`, `MathUnderover`.

Pós-P298: **14 variants** (+`MathOp`).

### A.1.2 — Tipos relacionados

`Box<Content>` (paralelo `MathAccent.base`); `bool` discriminador
estrutural.

### A.1.3 — Match arms exhaustive (9 sítios paralelo P296/P297)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | `text.plain_text()` |
| `content.rs:PartialEq` | structural |
| `content.rs:map_content()` | recurse em text |
| `content.rs:map_text()` | terminal |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `engine/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | handler dedicado |

### A.1.4 — Vanilla `OpElem`

Vide A.0.0.3.

### A.1.5 — Operadores pré-definidos vanilla

36+ operadores em scope math via macro `ops!`. **Scope-out P298**
— passo P298.X candidato se necessário.

### A.1.6 — `layout_attach` actual com heurística limits

Vide A.0.0.2. Modificação P298: +1 arm para `MathOp`.

### A.1.7 — Emit standard

`FrameItem::Text/Glyph` via `layout_node` recursivo. Sem operador
PDF novo.

### A.1.8 — Diagrama de fluxo P298

```
#op("lim", limits: true) + attach _("n→∞")
       │
       ▼
parse → eval_call (native_op)
       │
       ▼
Content::MathOp { text: "lim", limits: true }
       │
       ▼
Content::MathAttach { base: MathOp{..}, sub: "n→∞", .. }
       │
       ▼
Math Layouter::layout_node (arm MathAttach)
       └── layout_attach:
           ├── is_limits: agora detecta MathOp{limits:true}  [P298 NEW]
           ├── if is_limits → empilha sub abaixo (limits-style)
           └── else        → scripts-style padrão
       │
       ▼
MathBox { items: Vec<FrameItem> }
       │
       ▼
export.rs emit standard — INALTERADO
```

### A.1.9 — Paradigma consumer P298

**Cross-variant interaction**: `MathOp.limits` afecta layout de
`MathAttach`. Paradigma **genuinamente novo** dentro do cluster
math (P296/P297 não tinham cross-variant).

**Sub-padrão "cluster math handler dedicado" N=3 candidato** mas
`layout_op` em si é trivial (delegate). Verdadeira inovação é
**modificação `is_limits` em `attach.rs`** — extensão de heurística
existente.

---

## A.2 — Estrutura variant (decisão (b))

**Decisão A.2 → (b)**: `MathOp { text: Box<Content>, limits: bool }`.

Paridade vanilla literal:
- `text: Box<Content>` (vanilla `Content` evita recursive size).
- `limits: bool` discriminador estrutural.

**Implicação "variant rico" N=5**:
- `bool limits` é **discriminador estrutural** (afecta layout
  fundamental do attach).
- **Mas não é `Option<Box<Content>>` estrutural** (sempre presente).
- **Caso ambíguo** P297 §6.4 — `bool` discriminador.

**Decisão sobre N=5**: **NÃO qualificar P298** porque:
- P297 estabeleceu N=5 como qualificação Option estrutural genuíno.
- `bool` é discriminador, não Option — diluir o gatilho com casos
  limítrofes viola anti-padrão over-formalização.
- Padrão "variant rico" N=5 **adiado em P297 + P298 não dispara**.

Promoção candidata em P298: **sub-padrão "cluster math handler
dedicado" N=3 cumulativo**.

---

## A.3 — Integração com layout (decisão (γ) handler trivial)

**Decisão A.3 → (γ) handler dedicado mas trivial**:

```rust
fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox {
    self.layout_node(text, style)
}
```

Não é (α) full handler (paralelo MathAccent/MathCancel/MathUnderover).
**Trivial wrapper** porque `op` em si é apenas marcação semântica
— layout real do text é responsabilidade de `MathIdent`/`MathText`
padrão.

**Implicação sub-padrão "cluster math handler"**: N=3 **ambíguo**:
- 3 handlers existem (`layout_accent`, `layout_cancel`,
  `layout_underover`, `layout_op`).
- Mas `layout_op` é **trivial** (delegate single-line).
- Verdadeira inovação P298 está em **modificação `is_limits`**.

**Decisão sobre N=3 promoção**: **adiar** — N=3 dispara
quantitativamente mas qualidade do P298 handler é menor que P296/P297.
Não promover apenas para "atingir N=3" — anti-padrão
over-formalização.

---

## A.4 — Cross-variant interaction (paradigma novo)

**Modificação `attach.rs:55-61`**:

```rust
let is_limits = self.block && match base {
    Content::MathIdent(s) | Content::MathText(s) => {
        let ch = s.chars().next().unwrap_or('\0');
        symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
    }
    Content::MathOp { limits, .. } => *limits,    // P298 NEW
    _ => false,
};
```

**Paradigma cross-variant interaction** — `MathOp.limits` afecta
comportamento de `MathAttach`. Paradigma **genuinamente novo** no
cluster math.

---

## A.4-bis — Impacto em emit (preservado bit-exact)

Hash `export.rs 66cb8ac3` preservado pelo **15º passo consecutivo**.
ADR-0098 N=15 cumulativo.

---

## A.5 — Detecção de bugs latentes

6 cenários fronteira:

| Cenário | Comportamento esperado |
|---|---|
| `op("lim", limits: true)` + attach `_(x→0)` em block | sub posicionado abaixo (limits-style) |
| `op("sin", limits: false)` + attach `^2` | scripts-style (lateral superior) |
| `op("max")` (limits default false) sem attach | só text renderizado |
| `op("")` text vazio | width zero; degenerate |
| `op("Σ", limits: true)` | text é "Σ"; limits-style aplica |
| `lim` como `MathIdent` (sem `op()`) | continua a funcionar via `is_limit_function` heurístico — paridade preservada |

### A.5.1 — Sem bugs latentes esperados

Regressão `MathIdent("lim")` preservada via fallback hardcoded
heurístico. Padrão §8.4 N=1 estável.

---

## A.5' — Anti-reflexão N=7 cumulativo

### A.5'.1 — Comparação A.1.9 P288-P298

| Passo | Tipo | Paradigma consumer |
|---|---|---|
| P288-P292 | cumulativo | 5 paradigmas style/text |
| P293-P297 | ortogonais | 5 paradigmas distintos |
| **P298** | **extensão directa P296/P297 com cross-variant** | **Cross-variant interaction** (`MathOp` afecta `MathAttach`) |

P298 **inaugura paradigma novo**: cross-variant interaction. Apesar
de handler `layout_op` trivial, a **modificação `attach.rs`**
estabelece pattern arquitectural novo.

### A.5'.2 — A.0.0 N=6 reaplicação — magnitude alta

P298 magnitude **alta** confirma tendência:
- 2 magnitudes altas consecutivas (P297+P298).
- Janela P294-P298: máxima, baixa, média, alta, alta.
- §6.6 P295 degenerescência **definitivamente refutada**.

### A.5'.3 — Elementos estructuralmente novos identificados

5 elementos:

1. **Cross-variant interaction** — paradigma novo (P296/P297 sem
   interaction inter-variants).
2. **Extensão de heurística existente** — modificação `is_limits`
   estende, não substitui. Refactor controlado.
3. **`bool` discriminador estrutural ambíguo** — caso intermédio
   para "variant rico" N=5; **adiado**.
4. **Handler trivial vs estructural** — `layout_op` é delegate;
   sub-padrão "cluster math handler" N=3 qualifica quantitativo
   mas qualitativo ambíguo.
5. **Fecho cluster math 4/4** — accent + cancel + underover + op.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos:
- **§8.7' N=6** — magnitude alta consecutiva valida; promoção
  candidata.
- **§8.3 N=10** — refutação pragmática (heurística existente).
- **Sub-padrão "cluster math handler dedicado" N=3** — quantitativo
  qualifica mas qualitativo ambíguo (P298 handler trivial).
- **"Variant rico" N=5** — `bool` discriminador caso ambíguo;
  **NÃO qualifica P298**.
- **Sub-padrão "cross-variant interaction"** — N=1 inaugural P298;
  longe de limiar.

**Decisão**: **0 ADRs meta novas**. Razões:
- §8.7' N=6 candidato genuíno mas P273.17 §0 + tendência
  cumulativa P298+ pode atingir N=7 mais robusto.
- Sub-padrão "cluster math handler" N=3 ambíguo qualitativamente —
  não promover apenas para atingir N=3.
- "Variant rico" `bool` ambíguo — adiado conservador.

Anti-padrão over-formalização rigorosamente honrado.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P298 |
|---|---:|---:|
| `Content` variants | 68 | **69** (+1 MathOp) |
| `Content::Math*` variants | 13 | **14** |
| Stdlib funções math | 3 (P296+P297) | 4 (+`native_op`) |
| Hash L0 `content.md` | propagado | propagado |
| Hash `export.rs` | `66cb8ac3` | **preservado bit-exact** (**15º passo consecutivo**) |
| Padrão §8.6 A.5' N | 7 | **8** (P291-P298) |
| Padrão §8.7' A.0.0 N | 5 | **6** (reaplica com magnitude alta) |
| Padrão §8.3 N candidato | 9 | **10** candidato adiado |
| Padrão "variant rico" N | 5 candidato | 5 candidato (P298 não qualifica — `bool` ambíguo) |
| Sub-padrão "cluster math handler" N | 2 (P296+P297) | 3 ambíguo (handler trivial; adiado) |
| Sub-padrão "cross-variant interaction" | n/a | **1 inaugural** P298 |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (A.4 modifica layout_attach regressão): ⚖
  testes regression bit-exact obrigatórios — verificar `MathIdent("lim")`
  preservado.
- **Risco secundário** (A.0.0 magnitude alta inesperada HV''): ✅
  confirmado; refactor controlado (1 arm match adicional).
- **Risco terciário** (`bool` ambíguo): ✅ adiado conservador.
- **Risco quaternário** (sub-padrão N=3 promoção forçada): ✅
  adiado (handler trivial não justifica promoção).
- **Risco quinário** (6ª A.0.0 sem paradigma novo): ✅ refutado —
  cross-variant interaction é paradigma novo.
- **Risco senário** (fecho cluster 4/4 pressão sequência): ⚖
  registado; P299 deve ser ortogonal por construção.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=6 reaplica §8.7' com
refutação magnitude alta (HV'' confirmado)** + decisão **HV'' + (b)
text+bool + (γ) handler trivial + cross-variant interaction A.4** +
A.4-bis hash preservado + A.5 sem bugs + **A.5' N=7 cumulativo com
5 elementos novos**. **0 ADRs meta novas** — §8.7' N=6, sub-padrão
"cluster math handler" N=3 ambíguo, "variant rico" `bool` ambíguo
— todos adiados per P273.17 §0.

**MARCO P298**:
- **Fecho cluster math 4/4** — accent+cancel+underover+op
  implementados.
- **A.0.0 N=6 com magnitude alta** — 2ª consecutiva; refuta
  definitivamente §6.6 P295 degenerescência.
- **Cross-variant interaction paradigma inaugural** — `MathOp.limits`
  afecta `MathAttach` layout via modificação `is_limits` em
  `attach.rs`.
- **Heurística limits-style existente preservada** — fallback
  `MathIdent("lim")` continua a funcionar via `is_limit_function`.
- **Hash `export.rs` preservado pelo 15º passo consecutivo** —
  ADR-0098 robusta sobre 15 features distintas.
- **Anti-padrão over-formalização rigorosamente honrado** — 3
  promoções candidatas avaliadas, todas adiadas.

Procede-se a §3 da spec (com plano HV'' adaptado).
