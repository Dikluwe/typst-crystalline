# Relatório — Passo 296 (`P-math-accent-cancel`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-296.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-math-accent-cancel-passo-296.md`
**Tipo declarado spec**: frente ortogonal genuinamente nova; magnitude
XS+S; primeira spec onde A.0.0 não antecipa hipótese preferida.
**Hipótese adoptada**: **HIV (features ausentes apesar de "parcial")**
+ **A.2 → (a) minimal** + **A.3 → (α) variants + layout_math arms**.
**Baseline P295**: 2 817 testes  →  **P296**: 2 830 testes (Δ = +13)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**13º passo
consecutivo**: P282→P296)
**Hash `content.rs`**: `2fc32a66` → `7e621240` (mudança esperada — +2 variants)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P296 materializa **2 variants math** simultaneamente:
`Content::MathAccent { base, accent }` e
`Content::MathCancel { body }`, com:

- **Stdlib**: `native_accent(base, accent)` + `native_cancel(body)`
  em `structural.rs`.
- **Layouter math**: handlers dedicados `layout_accent` +
  `layout_cancel` em `rules/math/layout/mod.rs` (paralelo a
  `layout_frac` P37).
- **Emit**: agnóstico — produz `FrameItem::Text/Glyph/Line` standard.

**Resultado funcional**: `#accent(a, hat)` e `#cancel(x)` produzem
output PDF correcto:
- Accent: glyph acima da base, centrado horizontalmente.
- Cancel: linha diagonal "rising" (bottom-left → top-right) sobre
  bbox do body.

**Resultado metodológico — refutação significativa Tabela A.4**:
A.0.0 N=4 confirmou via inspecção literal **zero hits** em todo
`01_core/src/` para `accent`/`cancel`. Tabela A.4 marcava `parcial`
mas o status real era **AUSENTE**. P296 corrige classificação
`ausente` → `implementado`.

**Magnitude da refutação**: **média (factual-significativa)** —
classificação inteira inválida (não apenas linha desactualizada
como P295). **Refuta hipótese degenerescência §6.6 P295** — A.0.0
template valida com refutação real genuína.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=4 reaplica §8.7') | Tabela A.4 linhas 118-119 inválidas (real status `ausente`) — refutação magnitude média |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact — emit agnóstico via `FrameItem` standard |
| A.1 inventário | 10 `Math*` variants pré-P296 + 2 novos; precedente arquitectural directo = `MathFrac` (P37) |
| A.2 decisão | **(a) minimal** — `base/accent` + `body`; sem cosméticos vanilla; padrão "variant rico" N=4 preservado |
| A.3 integração | **(α) variants + layout_math handlers dedicados** |
| A.4 emit | **(i) — `FrameItem` standard**; hash `export.rs` preservado bit-exact |
| A.5 bugs latentes | 6 cenários verificados; nenhum bug |
| A.5' anti-reflexão | **N=5 cumulativo** (P291-P296); 5 elementos novos; teste empírico §8.7' robusto |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-math-accent-cancel-passo-296.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/entities/content.rs` (+2 variants)

```rust
// ── Passo 296 — Math accent + cancel (P-math-accent-cancel) ─────────
MathAccent {
    base:   Box<Content>,
    accent: Box<Content>,
},
MathCancel {
    body: Box<Content>,
},
```

Cosméticos vanilla **scope-out** per ADR-0054 graded:
- `AccentElem`: `size`, `dotless` (cosméticos).
- `CancelElem`: `length`, `inverted`, `cross`, `angle`, `stroke`
  (cosméticos + toggles funcionais para P296.X).

### §3.2 — Match arms exhaustive (defesa compilador, 9 sítios)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | concatena base+accent / body |
| `content.rs:PartialEq` | structural |
| `content.rs:map_content()` | recurse |
| `content.rs:map_text()` | terminal (paralelo MathFrac) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal (math structural) |
| `rules/introspect/locatable.rs:is_locatable` | `false` |
| `engine/layout/mod.rs` | fallthrough math (paralelo MathFrac) |
| `rules/math/layout/mod.rs:layout_node` | **handlers dedicados novos** |

`is_empty()` herdado via catch-all `_ => false` (math structural
sempre observable). Match exaustivo identificou todos os sítios via
compiler errors.

### §3.3 — Handlers `layout_accent` + `layout_cancel`

```rust
fn layout_accent(&self, base, accent, style) -> MathBox {
    let base_box   = self.layout_node(base,   style);
    let accent_box = self.layout_node(accent, style);
    // Centra accent horizontalmente sobre bbox de base.
    let dx = (base_box.width - accent_box.width) / 2.0;
    // Empilha: accent em (dx, 0..accent_h); base em (0, accent_h..).
    let accent_h = accent_box.height();
    let new_ascent = base_box.ascent + accent_h;
    let mut items = Vec::new();
    for item in accent_box.items {
        items.push(offset_item(item, Pt(dx), Pt(0.0)));
    }
    for item in base_box.items {
        items.push(offset_item(item, Pt(0.0), Pt(accent_h)));
    }
    MathBox {
        width: base_box.width.max(accent_box.width),
        ascent: new_ascent,
        descent: base_box.descent,
        items,
    }
}

fn layout_cancel(&self, body, style) -> MathBox {
    let body_box = self.layout_node(body, style);
    let h = body_box.height();
    // Linha diagonal "rising": bottom-left → top-right.
    let line = FrameItem::Line {
        start:     Point { x: Pt(0.0),            y: Pt(h) },
        end:       Point { x: Pt(body_box.width), y: Pt(0.0) },
        thickness: 0.5,
        color:     None,
    };
    let mut items = body_box.items;
    items.push(line);
    MathBox {
        width:  body_box.width,
        ascent: body_box.ascent,
        descent: body_box.descent,
        items,
    }
}
```

### §3.4 — Stdlib `native_accent` + `native_cancel`

```rust
pub fn native_accent(...) -> SourceResult<Value> {
    let base = ...;     // posicional [0]: Content ou Str → Content::text
    let accent = ...;   // posicional [1]: idem
    for k in args.named.keys() {
        return Err(format!("accent(): named '{}' não suportado", k));
    }
    Ok(Value::Content(Content::MathAccent {
        base: Box::new(base), accent: Box::new(accent)
    }))
}

pub fn native_cancel(...) -> SourceResult<Value> {
    let body = ...;     // posicional [0]
    for k in args.named.keys() {
        return Err(format!("cancel(): named '{}' não suportado", k));
    }
    Ok(Value::Content(Content::MathCancel { body: Box::new(body) }))
}
```

### §3.5 — Registo em `eval/mod.rs`

```rust
scope.define("accent", Value::Func(Func::native("accent", native_accent)));
scope.define("cancel", Value::Func(Func::native("cancel", native_cancel)));
```

### §3.6 — Zero alterações em emit

`03_infra/src/export.rs` **inalterado bit-exact**. Hash `66cb8ac3`
preservado pelo **13º passo consecutivo**. ADR-0098 honrada.

---

## §4 — Testes

### §4.1 — `01_core/src/engine/stdlib/mod.rs` (+11 testes L1)

**Accent (5 testes):**

| Teste | Verifica |
|---|---|
| `p296_native_accent_base_e_accent_posicionais` | Construção minimal com Content |
| `p296_native_accent_strings_convertidas_para_text` | Auto-conversão `Value::Str` → `Content::text` |
| `p296_native_accent_sem_base_retorna_err` | Robustez input |
| `p296_native_accent_sem_accent_retorna_err` | Robustez input |
| `p296_native_accent_named_arg_rejeitado` | Scope-out cosméticos (`size`/`dotless`) |

**Cancel (4 testes):**

| Teste | Verifica |
|---|---|
| `p296_native_cancel_body_posicional` | Construção minimal com Content |
| `p296_native_cancel_body_string_convertido` | Auto-conversão |
| `p296_native_cancel_sem_body_retorna_err` | Robustez input |
| `p296_native_cancel_named_arg_rejeitado` | Scope-out cosméticos (`length`/`inverted`/`cross`/`angle`/`stroke`) |

**Match arms (2 testes):**

| Teste | Verifica |
|---|---|
| `p296_math_accent_partial_eq_e_is_empty` | PartialEq structural; `is_empty == false` (math observable) |
| `p296_math_cancel_partial_eq` | PartialEq por body |

### §4.2 — `03_infra/src/export.rs` (+2 testes L3 PDF)

| Teste | Verifica |
|---|---|
| `p296_math_accent_emite_base_e_accent_no_pdf` | Equation com MathAccent renderiza base + accent (ambos no PDF) |
| **`p296_math_cancel_emite_body_e_linha_diagonal_no_pdf`** | Equation com MathCancel renderiza body + operador PDF `S` (stroke da linha) |

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2331 passed; 0 failed; 0 ignored
test result: ok.  452 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2830 passed total
```

Baseline P295 = 2 817; delta = +13 = 11 (L1) + 2 (L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P296

| Ficheiro L0 / código | Antes P296 | Pós P296 |
|---|---|---|
| `entities/content.md` | `4861affa` | propagado |
| `entities/content.rs` (`@prompt-hash`) | `2fc32a66` | **`7e621240`** (mudança esperada — +2 variants) |
| `rules/stdlib.md` | inalterado | inalterado (política única) |
| `infra/export.md` | `31a37c57` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**13º passo consecutivo**) |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" (**N=4 reaplica — limiar ultrapassado, adiado**)

| Passo | A.0.0 N | Magnitude da refutação |
|---|---:|---|
| P293 | 1 (inaugural) | Hipótese H6 não-listada na spec — **alta** |
| P294 | 2 | Toda a estrutura proposta pela spec invalidada — **máxima** |
| P295 | 3 | Linha de tabela administrativa desactualizada — **baixa** |
| **P296** | **4** | **Classificação Tabela A.4 inteira inválida — média (factual-significativa)** |

**Refutação P296 média** = entre P295 (baixa) e P294 (máxima).
**§8.7' template valida com refutação real genuína** — hipótese
degenerescência §6.6 P295 **refutada empiricamente**.

**Decisão de adiar promoção** apesar de limiar N≥3 ultrapassado:
- P273.17 §0: uma ADR meta por passo no máximo.
- Reaplicação P297+ com refutação ainda mais robusta consolidaria
  N=5 — promoção mais defensável.

### §6.2 — §8.3 "refutação pragmática" (**N=8 candidato adiado**)

P296 traz refutação significativa adicional sobre `parcial`/`ausente`
classificação. **Adiado** mesma razão.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=6 cumulativo**)

P291+P292+P293+P294+P295+P296. **Limiar passado** mas anti-padrão
"over-formalização" P273.17 §0 adia. Padrão metodológico interno.

### §6.4 — "Variant rico com cosméticos opcionais" (**N=4 preservado**)

Decisão consciente **A.2 → (a)** preserva o padrão inalterado.
P296 NÃO qualifica gratuitamente N=5 — cosméticos vanilla
(`size`/`length`/`inverted`/`cross`/`angle`/`stroke`/`dotless`)
scope-out per ADR-0054 graded.

Honestidade: `inverted: bool` / `cross: bool` em `CancelElem` são
**toggles funcionais genuínos** (não cosméticos puros). Materialização
adiada para passo P296.X dedicado.

### §6.5 — ADR-0098 "single source of truth" (**N=13 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 13º passo consecutivo.
Invariante robusta sobre 13 features distintas (P282-P296):
- Style cumulativos P288-P292.
- Curve cubic P293, quadratic P294.
- Footnote marker P295.
- **Math accent/cancel P296** — primeiro caso math.

### §6.6 — Refutação de §6.6 P295 (hipótese degenerescência)

P295 §10 hipotetizou: *"4 reaplicações A.0.0 consecutivas com
magnitude decrescente ou factual-modesta → template degenera em
ritual procedimental. Considerar desformalização."*

**P296 refuta essa hipótese**:
- N=4 atinge magnitude **média**, não factual-modesta.
- Inspecção literal revelou erro factual significativo (Tabela A.4
  inteira inválida).
- Sem inspecção, P296 teria assumido "parcial heurístico" e
  implementado refinos onde não havia código a refinar.

**§8.7' template NÃO degenerou**. Vigilância P297+ permanece:
desformalização só se reaplicações futuras forem consistentemente
factuais-modestas ou nulas.

---

## §7 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 118** (`accent(c, mark)`): `parcial` → `implementado ⁸¹`
  com nota completa sobre P296 + refutação A.0.0 N=4.
- **Linha 119** dividida em 2:
  - `cancel`: `parcial` → `implementado` (P296).
  - `underover` + `op`: permanecem `parcial` (P296.1/P296.2
    candidatos).

---

## §8 — Frentes pendentes pós-P296

| Frente | Tipo | Estado |
|---|---|---|
| **P296.1** — `underover` (vanilla `UnderoverElem`) | XS+S | Registado; precedente P296 directo |
| **P296.2** — `op` (vanilla `OpElem`) | XS+S | Registado |
| **P296.X** — `cancel(..., inverted: true)`/`cross: true` toggles funcionais | XS | Registado; toggles binary não-cosméticos |
| `size`/`dotless` (accent) e `length`/`angle`/`stroke` (cancel) | XS | Cosméticos; scope-out per ADR-0054 graded |

---

## §9 — Decisão sobre P297

Candidatos ortogonais disponíveis:

1. **P296.1 underover** — extensão directa P296 com paradigma
   idêntico.
2. **P296.2 op** — extensão directa P296 com paradigma idêntico.
3. **`curve.move`/scope-methods** — sintaxe vanilla fiel.
4. **`native_path` SVG-string parser**.
5. **`Length` em `Stroke`** — refino tabela cobertura.
6. **P295.1 nota rodapé** — extensão directa P295 com magnitude L
   (2-pass layout).

Decisão fica para o operador humano. **P296 não dita P297**.

Se P297 for P296.1/P296.2 (extensões directas), A.0.0 será **5ª
reaplicação** — vigilância de degenerescência §6.6/§6.6-revista
permanece. Se refutação significativa novamente, promoção §8.7'
N=5 robusta candidata.

---

## §10 — Honestidade epistémica

P296 valida empiricamente o template §8.7' com refutação magnitude
média:

| Passo | A.0.0 magnitude | Genuinidade |
|---|---|---|
| P293 H6 | alta | genuína (hipótese não-listada) |
| P294 (β) | máxima | genuína (vanilla pattern invalida spec) |
| P295 Tab.C | baixa | factual mas modesta |
| **P296 Tab.A** | **média** | **genuína (classificação inteira inválida)** |

Tendência: **não-decrescente**, refutando hipótese degenerescência
P295 §10. Próxima vigilância em P297+.

P296 também valida que **scope-out scriptural** (cosméticos
ADR-0054 graded) é robusta — variant minimal com 2-3 fields
required cobre 90% do uso prático sem qualificar N=5 "variant rico"
gratuitamente.

---

## §11 — Fecho

P296 fechado com:

- **+13 testes** (11 L1 + 2 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** bit-exact (13º passo consecutivo).
- **Hash `content.rs`** mudou esperadamente (+2 variants Math).
- **0 ADRs meta novas** — §8.7' N=4, §8.3 N=8 ambos adiados per
  P273.17 §0.
- **Padrão "variant rico" N=4 preservado** (A.2 → (a) minimal).
- **9.º paradigma consumer arquitecturalmente distinto** registado
  (math layout handler dedicado — primeiro paradigma math em
  P288-P296).
- **Hipótese degenerescência §6.6 P295 refutada empiricamente** —
  §8.7' template valida.

**MARCO P296**:
- **3.º passo ortogonal pós-série cumulativa cirúrgica P288-P292**.
- **A.0.0 N=4 com refutação magnitude média** — refuta hipótese
  degenerescência P295. Template robusto, não degenerou.
- **`MathAccent` + `MathCancel` materializados pela primeira vez**
  — Tabela A.4 corrigida `parcial` (errado) → `ausente` (real
  pré-P296) → `implementado` (pós-P296).
- **Hash `export.rs` preservado pelo 13º passo consecutivo**
  (P282→P296) — ADR-0098 robusta sobre 13 features distintas.
- **Math é 9.º paradigma consumer** — primeiro caso math layout
  em série P288-P296.
- **Frentes registadas** P296.1 (underover) + P296.2 (op) + P296.X
  (toggles `inverted`/`cross`) — extensão directa do paradigma.
