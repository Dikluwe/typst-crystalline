# Relatório — Passo 298 (`P296.2 — MathOp` fecho cluster math 4/4)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-298.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-math-op-passo-298.md`
**Tipo declarado spec**: 3ª aplicação "cluster math handler
dedicado"; magnitude XS-S esperada; A.0.0 podia revelar HV''.
**Hipótese adoptada**: **HV'' confirmado** (heurística limits-style
já existe em `attach.rs`) + **A.2 → (b) `Box<Content>` + bool** +
**A.3 → (γ) handler trivial** + **A.4 cross-variant interaction** em
`attach.rs`.
**Magnitude da refutação A.0.0 N=6**: **alta (factual-significativa)**
— descoberta de mecanismo limits-style hardcoded existente.
**Baseline P297**: 2 841 testes  →  **P298**: 2 852 testes (Δ = +11)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**15º passo
consecutivo**: P282→P298)
**Hash `content.rs`**: `985ddc8c` → `82d3c47d` (+1 variant)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P298 materializa `Content::MathOp { text: Box<Content>, limits: bool }`
com:

- **Stdlib**: `native_op(text, limits: bool = false)` em
  `structural.rs`.
- **Layouter math**: handler **trivial** `layout_op` (delegate para
  `layout_node(text)`) em `rules/math/layout/mod.rs`.
- **Cross-variant interaction**: modificação em
  `rules/math/layout/attach.rs:55-61` para detectar
  `Content::MathOp { limits: true, .. }` como base de `MathAttach`
  e renderizar scripts em limits-style.
- **Emit**: agnóstico — produz `FrameItem::Text/Glyph` standard.

**Resultado funcional**: `#op("lim", limits: true)` com attach
`_(x→0)` em block mode → scripts em limits-style (empilhamento
vertical em vez de lateral). Operadores cristalino pré-existentes
(`MathIdent("lim")` via heurística `is_limit_function` hardcoded)
**continuam a funcionar** sem necessidade de `op()`.

**Resultado metodológico — descoberta significativa A.0.0 N=6**:
A spec P298 antecipou cristalino **sem heurística limits-style**.
Inspecção literal em `01_core/src/rules/math/layout/attach.rs:55-61`
revelou **heurística hardcoded já existente** via
`symbols::is_limit_function` (`"lim"`/`"max"`/`"min"`/`"sup"`/`"inf"`/
`"limsup"`/`"liminf"`) + `is_large_operator` (`∑`/`∫`/etc.).

**Magnitude**: **alta** — 2.ª consecutiva (P297 alta → P298 alta).
Trend não-decrescente fortemente confirmado; **§6.6 P295
degenerescência definitivamente refutada**.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=6 reaplica §8.7') | Heurística limits-style já existe (HV'' confirmado); magnitude **alta** |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact — emit agnóstico |
| A.1 inventário | 13 `Math*` variants pré-P298 + 1 novo; vanilla `OpElem { text: Content, limits: bool }` |
| A.2 decisão | **(b) Box<Content> + bool** — `bool` é discriminador estrutural mas NÃO qualifica "variant rico" N=5 (P297 estabeleceu critério Option) |
| A.3 integração | **(γ) handler trivial** — `layout_op` delegate; verdadeira inovação em A.4 |
| A.4 cross-variant | **Modificação `attach.rs:55-61`** — +1 arm `MathOp { limits, .. } => *limits`. Heurística pré-P298 preservada. |
| A.5 bugs latentes | 6 cenários verificados; nenhum bug |
| A.5' anti-reflexão | **N=7 cumulativo** (P291-P298); 5 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-math-op-passo-298.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/entities/content.rs` (+1 variant)

```rust
// ── Passo 298 — `MathOp` (P296.2 fecho cluster math 4/4) ─────────────
MathOp {
    text:   Box<Content>,
    limits: bool,
},
```

Paridade vanilla `OpElem { text: Content, limits: bool }` —
`Box<Content>` em cristalino para evitar recursive size.

### §3.2 — Match arms exhaustive (9 sítios paralelo P296/P297)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | `text.plain_text()` (limits é layout) |
| `content.rs:PartialEq` | structural (text + limits) |
| `content.rs:map_content()` | recurse em text; preserva limits |
| `content.rs:map_text()` | terminal (paralelo cluster math) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `rules/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | `layout_op` trivial delegate |

### §3.3 — Handler `layout_op` (trivial delegate)

```rust
fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox {
    self.layout_node(text, style)
}
```

**Trivial wrapper** — `op` é marcação semântica para o attach pai
detectar; layout real do text é responsabilidade de
`MathIdent`/`MathText` standard.

### §3.4 — Modificação cross-variant em `attach.rs` (paradigma inaugural)

```rust
// P298 (cross-variant interaction)
let is_limits = self.block && match base {
    Content::MathIdent(s) | Content::MathText(s) => {
        let ch = s.chars().next().unwrap_or('\0');
        symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
    }
    Content::MathOp { limits, .. } => *limits,    // P298 NEW
    _ => false,
};
```

**Pattern arquitectural novo** — primeira vez no cluster math que
um variant afecta o layout de outro variant. Paradigma **cross-variant
interaction** inaugurado N=1 em P298.

**Heurística pré-P298 preservada**:
- `MathIdent("lim")` continua a funcionar via `is_limit_function`.
- `MathIdent("Σ")` continua a funcionar via `is_large_operator`.
- Regressão validada por teste dedicado.

### §3.5 — Stdlib `native_op`

```rust
pub fn native_op(...) -> SourceResult<Value> {
    let text = ...;     // posicional: Content ou Str → Content::text
    // Validar named: só "limits" permitido.
    for k in args.named.keys() {
        if k.as_str() != "limits" { return Err(...); }
    }
    let limits = match args.named.get("limits") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None)    => false,
        Some(other) => return Err("op(limits:) espera bool"),
        None => false,    // default
    };
    Ok(Value::Content(Content::MathOp { text: Box::new(text), limits }))
}
```

### §3.6 — Zero alterações em emit

`03_infra/src/export.rs` **inalterado bit-exact**. Hash `66cb8ac3`
preservado pelo **15º passo consecutivo**.

---

## §4 — Testes

### §4.1 — `01_core/src/rules/stdlib/mod.rs` (+8 testes L1)

**native_op (6 testes):**

| Teste | Verifica |
|---|---|
| `p298_native_op_text_posicional_default_limits_false` | Construção minimal; limits default false |
| `p298_native_op_com_limits_true_named` | named arg `limits: true` aceito |
| `p298_native_op_text_content_preservado` | Content posicional preservado estructuralmente |
| `p298_native_op_sem_text_retorna_err` | Robustez input |
| `p298_native_op_named_invalido_retorna_err` | só `limits` permitido |
| `p298_native_op_limits_nao_bool_retorna_err` | `limits` espera bool |

**Match arms (2 testes):**

| Teste | Verifica |
|---|---|
| `p298_math_op_partial_eq_structural_com_limits` | PartialEq honra `text + limits` |
| `p298_math_op_plain_text_so_text_sem_limits` | plain_text retorna só text (limits é layout) |

### §4.2 — `03_infra/src/export.rs` (+3 testes L3 PDF)

| Teste | Verifica |
|---|---|
| `p298_math_op_text_emite_no_pdf` | op text renderiza standard |
| `p298_math_attach_com_op_limits_renderiza_pdf_valido` | **Cross-variant**: `MathOp{limits:true}` + attach produz PDF válido em limits-style |
| **`p298_regressao_math_ident_lim_continua_a_funcionar`** | **INVARIANTE CRÍTICA**: heurística pré-P298 hardcoded preservada — `MathIdent("lim")` + attach em block mode continua limits-style sem necessidade de `op()` |

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2348 passed; 0 failed; 0 ignored
test result: ok.  457 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2852 passed total
```

Baseline P297 = 2 841; delta = +11 = 8 (L1) + 3 (L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P298

| Ficheiro L0 / código | Antes P298 | Pós P298 |
|---|---|---|
| `entities/content.md` | propagado P297 | propagado |
| `entities/content.rs` (`@prompt-hash`) | `985ddc8c` | **`82d3c47d`** (+1 variant) |
| `rules/stdlib.md` | inalterado | inalterado (política única) |
| `infra/export.md` | `31a37c57` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**15º passo consecutivo**) |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — magnitude alta consecutiva

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta |
| P294 | 2 | máxima |
| P295 | 3 | baixa |
| P296 | 4 | média |
| P297 | 5 | alta |
| **P298** | **6** | **alta** |

**Janela P294-P298**: máxima, baixa, média, alta, **alta**. **Duas
magnitudes altas consecutivas** (P297+P298) — tendência
não-decrescente robustamente confirmada. **§6.6 P295
degenerescência definitivamente refutada**.

§8.7' N=6 candidato genuíno. **Adiado** per P273.17 §0 (uma ADR
meta por passo).

### §6.2 — §8.3 "refutação pragmática" (**N=10 candidato adiado**)

Refutação de spec sobre heurística pré-existente. Mesma razão de
adiamento.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=8 cumulativo**)

P291-P298. Limiar passado mas anti-padrão adia.

### §6.4 — "Variant rico" N=5 — P298 NÃO qualifica

P297 §6.4 estabeleceu N=5 como qualificação **Option `Box<Content>`
estrutural genuíno**. P298 tem **`bool limits`** — discriminador
estrutural mas **não Option**:

| Tentativa | Tipo | Qualifica N=5? |
|---|---|---|
| P156G/H/I | bool defaults (graded) | Refutado P287 |
| P284 | Option primitives (Length/Color) | Categoria diferente |
| P297 MathUnderover | Option `Box<Content>` estrutural | **Qualifica genuíno** |
| **P298 MathOp** | **`bool limits` discriminador** | **Caso intermédio — não qualifica** |

**Decisão honesta**: NÃO qualificar P298. Diluir o gatilho com
casos limítrofes viola anti-padrão over-formalização P273.17 §0.
Padrão "variant rico" N=5 candidato **adiado em P297 e preservado
em P298**.

### §6.5 — Sub-padrão "cluster math handler dedicado" — N=3 ambíguo

Quantitativo: 3 handlers dedicados existem
(`layout_accent`/`layout_cancel`/`layout_underover`/`layout_op`).

Qualitativo: `layout_op` é **trivial delegate** (1 linha
`self.layout_node(text, style)`). Não é estructuralmente igual a
P296 (geometria) ou P297 (empilhamento composicional).

**Decisão**: adiar promoção. Promover apenas para "atingir N=3"
quando o 3.º caso é trivial viola critério de robustez. Sub-padrão
preservado para passos futuros com handler mais substantivo.

### §6.6 — Sub-padrão "cross-variant interaction" — N=1 inaugural

P298 inaugura paradigma novo: variant `MathOp` afecta layout de
variant `MathAttach`. Não há precedente na sequência P296-P297.

**N=1 inaugural** — longe de limiar tentativo N≥3. Registado para
vigilância em passos futuros.

### §6.7 — ADR-0098 "single source of truth" (**N=15 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 15º passo consecutivo
(P282→P298). Invariante robusta sobre 15 features distintas:
Style P288-P292, Curve P293/P294, Footnote P295, Math accent/cancel
P296, Math underover P297, **Math op P298** (com cross-variant em
`attach.rs` mas sem efeito em emit).

### §6.8 — Anti-padrão over-formalização rigorosamente honrado

P298 avaliou **3 promoções candidatas**:
1. §8.7' A.0.0 template N=6
2. Sub-padrão "cluster math handler dedicado" N=3
3. Sub-padrão "cross-variant interaction" N=1

**Todas adiadas**. Razões:
- §8.7' N=6 — preferência por consolidação cumulativa.
- Sub-padrão "cluster math handler" — qualidade do 3.º caso ambígua.
- Sub-padrão "cross-variant" — longe de limiar (N=1).

**P273.17 §0 honrado** — uma ADR meta por passo no máximo;
preferível zero quando ambíguo.

---

## §7 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 119** (`op`): `parcial` → `implementado ⁸³` com nota
  completa sobre P298, cross-variant interaction, heurística
  pré-P298 preservada, e operadores vanilla pré-definidos
  scope-out P298.X.

**Cluster math 4/4 fechado**:
- ✅ `accent` (P296)
- ✅ `cancel` (P296)
- ✅ `underover` (P297)
- ✅ **`op` (P298)**

---

## §8 — Frentes pendentes pós-P298

| Frente | Tipo | Estado |
|---|---|---|
| **P298.X** — operadores vanilla pré-definidos (`lim`/`sin`/`cos`/`max`/`min`/etc., 36+) | S-M | Vanilla `op.rs` define via macro `ops!`; cristalino actual usa `MathIdent` literal + heurística — workaround viável |
| **P297.X** — discriminator `UnderoverKind` | XS+ | Cosmético per ADR-0054 graded |
| **P296.X** — toggles `inverted`/`cross` cancel | XS | Toggles binary não-cosméticos |
| Cosméticos accent/cancel/underover/op (`size`/`length`/`angle`/`stroke`/`dotless`) | XS | ADR-0054 graded |

---

## §9 — Decisão sobre P299

**P298 fecha cluster math 4/4** — P299 deve ser **ortogonal por
construção** (paralelo P293 pós-série Style cumulativa P288-P292).

Candidatos:

1. **Frentes math secundárias** (P297.X / P296.X / P298.X — refinos).
2. **Frentes não-math** (P295.1 nota rodapé; `Length` em `Stroke`;
   `curve.move`/scope-methods; outras P282 §6).
3. **Frente nova** completamente distinta.

Decisão fica para o operador humano. **P298 não dita P299**.

Se P299 reaplicar A.0.0 com refutação **modesta** após P298 alta,
mantém-se confiança no template §8.7' — magnitude variada é
saudável (P296 média → P297 alta → P298 alta → P299 modesta seria
flutuação normal).

Se P299 reaplicar com refutação **alta de novo**, atinge **3
magnitudes altas consecutivas** — §8.7' N=7 promoção altamente
robusta candidata (raríssimas circunstâncias para adiamento).

---

## §10 — Honestidade epistémica

P298 inverte parcialmente a antecipação da spec:

| Cenário antecipado spec | Realidade |
|---|---|
| "Cristalino sem heurística limits-style" | **Refutado** — heurística hardcoded existe (`is_limit_function`/`is_large_operator`) |
| `text: EcoString` (vanilla) | **Refutado** — vanilla usa `text: Content` |
| Sub-padrão "cluster math handler" N=3 promoção candidata clara | **Refutado** — handler trivial; qualidade ambígua |
| "Variant rico" N=5 qualificação clara | **Refutado** — `bool` ambíguo; adiar conservador |

**Lições metodológicas**:

1. **A.0.0 deve preceder pressuposto da spec** — sem inspecção
   literal, P298 teria assumido cristalino sem heurística e duplicado
   funcionalidade.
2. **Magnitude alta cumulativa robusta** — 2.ª consecutiva confirma
   tendência não-decrescente.
3. **Disparo de gatilhos quantitativos não justifica promoção** —
   sub-padrão N=3 quantitativo mas qualitativo ambíguo → preserved
   para consolidação futura.
4. **Cross-variant interaction inaugurado** — pattern arquitectural
   novo emerge organicamente do trabalho concreto.

---

## §11 — Fecho

P298 fechado com:

- **+11 testes** (8 L1 + 3 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** bit-exact (15º passo consecutivo).
- **Hash `content.rs`** mudou esperadamente (+1 variant).
- **0 ADRs meta novas** — 3 promoções candidatas avaliadas e
  adiadas (§8.7' N=6, sub-padrão "cluster math handler" N=3
  ambíguo, sub-padrão "cross-variant" N=1 inaugural).
- **Padrão "variant rico" N=5 P298 NÃO qualifica** — `bool`
  discriminador caso intermédio honestamente registado.
- **Sub-padrão "cluster math handler dedicado" N=3 ambíguo** —
  handler trivial não justifica promoção.
- **Sub-padrão "cross-variant interaction" N=1 inaugural** —
  pattern emergente registado.
- **Hipótese degenerescência §6.6 P295 definitivamente refutada**
  — 2 magnitudes altas consecutivas (P297+P298).
- **Heurística pré-P298 preservada bit-exact** — regressão
  validada por teste dedicado.

**MARCO P298**:
- **Cluster math 4/4 fechado** — accent + cancel + underover + op
  todos implementados.
- **Paradigma cross-variant interaction inaugurado** — `MathOp.limits`
  afecta `MathAttach` layout via modificação `is_limits` em
  `attach.rs`. Pattern arquitectural novo no cluster math.
- **A.0.0 N=6 com magnitude alta** — 2.ª consecutiva; refuta
  definitivamente §6.6 P295 degenerescência.
- **Heurística limits-style hardcoded preservada** — `MathIdent("lim")`
  continua a funcionar via fallback. P298 estende sem substituir.
- **Hash `export.rs` preservado pelo 15º passo consecutivo**
  (P282→P298) — ADR-0098 robusta sobre 15 features distintas.
- **Anti-padrão over-formalização rigorosamente honrado** — 3
  promoções candidatas, todas adiadas; "uma ADR meta por passo
  máximo" preferencialmente zero.
- **P299 ortogonal por construção** — cluster math fechado libera
  espaço para frente nova.
