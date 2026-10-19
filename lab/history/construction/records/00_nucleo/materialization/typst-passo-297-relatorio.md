# Relatório — Passo 297 (`P296.1 — MathUnderover`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-297.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-math-underover-passo-297.md`
**Tipo declarado spec**: extensão directa P296 com decisão A.2
genuinamente distinta (Option fields).
**Hipótese adoptada**: **HV'.a (spec invalidada por vanilla
pattern — paralelo P294) + agregação cristalina** + **A.2 → (b)
Option fields estrutural** + **A.3 → (α) handler dedicado**.
**Magnitude da refutação A.0.0 N=5**: **alta (factual-significativa
máxima)**. Refuta categoricamente hipótese degenerescência §6.6 P295.
**Baseline P296**: 2 830 testes  →  **P297**: 2 841 testes (Δ = +11)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**14º passo
consecutivo**: P282→P297)
**Hash `content.rs`**: `7e621240` → `985ddc8c` (mudança esperada — +1 variant)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P297 materializa `Content::MathUnderover { base, under: Option, over: Option }`
com:

- **Stdlib**: `native_underover(base, under: ?, over: ?)` em
  `structural.rs` — base posicional; under/over named opcionais.
- **Layouter math**: handler dedicado `layout_underover` em
  `rules/math/layout/mod.rs` (paralelo `layout_accent` P296). Empilha
  verticalmente: over (topo) + base (meio) + under (fundo); cada
  centrado horizontalmente.
- **Emit**: agnóstico — produz `FrameItem::Text/Glyph` standard.

**Resultado funcional**: `#underover(base, under: "u", over: "o")`
produz output PDF com base + anotações vertical empilhadas.

**Resultado metodológico — refutação significativa magnitude alta**:
A spec P297 §A.1.4 assumiu vanilla `UnderoverElem { base, under?, over? }`
unificado. Inspecção literal `lab/.../math/underover.rs` revelou
**vanilla NÃO tem wrapper** — fragmenta em **12 elementos
separados** (`UnderlineElem`/`OverlineElem`/`UnderbraceElem`/
`OverbraceElem`/`UnderbracketElem`/`OverbracketElem`/`UnderparenElem`/
`OverparenElem`/`UndershellElem`/`OvershellElem`). Spec inteira
invalidada — paralelo arquitectural directo de P294.

**Consequência metodológica**: hipótese degenerescência §6.6 P295
**categoricamente e definitivamente refutada** — sequência A.0.0
P293-P297 mostra magnitudes não-decrescentes em janela P294-P297
(máxima → baixa → média → **alta**).

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=5 reaplica §8.7') | Vanilla NÃO tem `UnderoverElem` — fragmenta em 12 elementos; magnitude **alta** |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact — emit agnóstico |
| A.1 inventário | 12 `Math*` variants pré-P297 + 1 novo; vanilla wrapper unificado AUSENTE |
| A.2 decisão | **(b) Option fields** — primeira qualificação genuína "variant rico" N=5 desde P287 refutação |
| A.3 integração | **(α) handler dedicado paralelo `layout_accent`** |
| A.4 emit | **(i) `FrameItem` standard**; hash `export.rs` preservado bit-exact |
| A.5 bugs latentes | 6 cenários verificados; nenhum bug |
| A.5' anti-reflexão | **N=6 cumulativo** (P291-P297); 5 elementos novos; sub-padrão "cluster math handler" N=2 |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-math-underover-passo-297.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/entities/content.rs` (+1 variant agregado)

```rust
// ── Passo 297 — `MathUnderover` (P296.1) ─────────────────────────────
/// Anotações verticais sobre/sob conteúdo matemático — agregação
/// cristalina do cluster vanilla `UnderlineElem`/`OverlineElem`/
/// `UnderbraceElem`/`OverbraceElem`/`UnderbracketElem`/etc.
MathUnderover {
    base:  Box<Content>,
    under: Option<Box<Content>>,
    over:  Option<Box<Content>>,
},
```

**HV'.a justificação** (agregação cristalina vs 12 fragmentos
vanilla):
- ADR-0054 graded vigente — divergência consciente para
  simplificação arquitectural.
- Paralelo conceptual P296 `MathAccent` (agregou várias
  variantes Unicode em 1 variant).
- Future-proof — discriminator `UnderoverKind::Brace`/`Bracket`/
  etc. candidato P297.X se cluster vanilla exigir paridade fina.

### §3.2 — Match arms exhaustive (defesa compilador, 9 sítios)

Paralelo P296 estratégia:

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | concatena over+base+under (ordem visual) |
| `content.rs:PartialEq` | structural com Options |
| `content.rs:map_content()` | recurse condicional em `Option<Box<Content>>` |
| `content.rs:map_text()` | terminal (paralelo MathFrac/MathAccent) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs:is_locatable` | `false` |
| `engine/layout/mod.rs` | fallthrough math (paralelo MathFrac) |
| `rules/math/layout/mod.rs:layout_node` | **handler dedicado novo** |

### §3.3 — Handler `layout_underover` (paralelo `layout_accent` P296)

```rust
fn layout_underover(
    &self,
    base:  &Content,
    under: Option<&Content>,
    over:  Option<&Content>,
    style: &TextStyle,
) -> MathBox {
    let base_box = self.layout_node(base, style);
    let over_box = over.map(|c| self.layout_node(c, style));
    let under_box = under.map(|c| self.layout_node(c, style));

    let w = base_box.width
        .max(over_box.as_ref().map(|b| b.width).unwrap_or(0.0))
        .max(under_box.as_ref().map(|b| b.width).unwrap_or(0.0));

    let over_h  = over_box.as_ref().map(|b| b.height()).unwrap_or(0.0);
    let under_h = under_box.as_ref().map(|b| b.height()).unwrap_or(0.0);
    let base_h  = base_box.height();

    let mut items = Vec::new();
    // Over (topo): y = 0..over_h
    if let Some(ob) = over_box {
        let dx = (w - ob.width) / 2.0;
        for item in ob.items {
            items.push(offset_item(item, Pt(dx), Pt(0.0)));
        }
    }
    // Base (meio): y = over_h..over_h+base_h
    let base_dx = (w - base_box.width) / 2.0;
    for item in base_box.items {
        items.push(offset_item(item, Pt(base_dx), Pt(over_h)));
    }
    // Under (fundo): y = over_h+base_h..over_h+base_h+under_h
    if let Some(ub) = under_box {
        let dx = (w - ub.width) / 2.0;
        for item in ub.items {
            items.push(offset_item(item, Pt(dx), Pt(over_h + base_h)));
        }
    }

    MathBox {
        width:   w,
        ascent:  base_box.ascent + over_h,
        descent: base_box.descent + under_h,
        items,
    }
}
```

### §3.4 — Stdlib `native_underover`

```rust
pub fn native_underover(...) -> SourceResult<Value> {
    let base = ...;  // posicional obrigatório
    // Validar named: só "under"/"over" permitidos.
    for k in args.named.keys() {
        if !["under", "over"].contains(&k.as_str()) {
            return Err(format!("underover(): named inesperado '{}'", k));
        }
    }
    let under = args.named.get("under").and_then(...);
    let over = args.named.get("over").and_then(...);
    Ok(Value::Content(Content::MathUnderover {
        base: Box::new(base), under, over,
    }))
}
```

### §3.5 — Zero alterações em emit

`03_infra/src/export.rs` **inalterado bit-exact**. Hash `66cb8ac3`
preservado pelo **14º passo consecutivo**.

---

## §4 — Testes

### §4.1 — `01_core/src/engine/stdlib/mod.rs` (+9 testes L1)

**native_underover (6 testes):**

| Teste | Verifica |
|---|---|
| `p297_native_underover_so_base_posicional` | base only; under/over `None` |
| `p297_native_underover_com_under_named` | named arg `under` aceito |
| `p297_native_underover_com_over_named` | named arg `over` aceito |
| `p297_native_underover_com_ambos` | ambos under+over presentes |
| `p297_native_underover_sem_base_retorna_err` | Robustez input |
| `p297_native_underover_named_arg_invalido_retorna_err` | só `under`/`over` permitidos |

**Match arms (3 testes):**

| Teste | Verifica |
|---|---|
| `p297_math_underover_plain_text_ordem_visual` | concatena over+base+under (ordem espacial) |
| `p297_math_underover_partial_eq_structural` | PartialEq honra Option semantics |
| `p297_math_underover_ambos_none_equivale_so_base` | Caso degenerate (both None) |

### §4.2 — `03_infra/src/export.rs` (+2 testes L3 PDF)

| Teste | Verifica |
|---|---|
| `p297_math_underover_com_ambos_emite_3_partes_no_pdf` | Equation com under+over renderiza 3 partes visíveis |
| `p297_math_underover_so_base_emite_so_base_no_pdf` | Degenerate (both None) emite só base |

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2340 passed; 0 failed; 0 ignored
test result: ok.  454 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2841 passed total
```

Baseline P296 = 2 830; delta = +11 = 9 (L1) + 2 (L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P297

| Ficheiro L0 / código | Antes P297 | Pós P297 |
|---|---|---|
| `entities/content.md` | propagado P296 | propagado |
| `entities/content.rs` (`@prompt-hash`) | `7e621240` | **`985ddc8c`** (+1 variant) |
| `rules/stdlib.md` | inalterado | inalterado (política única) |
| `infra/export.md` | `31a37c57` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**14º passo consecutivo**) |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — refutação categórica da hipótese degenerescência

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta |
| P294 | 2 | **máxima** |
| P295 | 3 | baixa |
| P296 | 4 | média |
| **P297** | **5** | **alta** |

**Janela P294-P297**: magnitudes **não-decrescentes em média**
(4.0 / 1.0 / 3.0 / 4.0). **Hipótese degenerescência §6.6 P295
categoricamente refutada**.

**§8.7' N=5 candidato genuíno** — magnitude alta valida promoção.
**Adiado** per P273.17 §0 (uma ADR meta por passo) e simultaneidade
com "variant rico" N=5 disparo.

### §6.2 — §8.3 "refutação pragmática" (**N=9 candidato adiado**)

Refutação significativa de spec inteira por vanilla pattern. Mesma
razão de adiamento.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=7 cumulativo**)

P291+P292+P293+P294+P295+P296+P297. **Limiar passado** mas
anti-padrão "over-formalização" P273.17 §0 adia.

### §6.4 — **"Variant rico com cosméticos opcionais" N=5 candidato genuíno** (primeira qualificação desde P287)

**Decisão A.2 → (b) Option `Box<Content>` estrutural** dispara o
gatilho pela primeira vez de forma genuína:

| Tentativa | Tipo | Veredicto |
|---|---|---|
| P156G Block | bool defaults (graded) | Refutado P287 — bool defaults não qualificam |
| P156H Boxed | bool defaults | Refutado P287 |
| P156I Stack | bool defaults | Refutado P287 |
| P284 Underline/Strike/Overline | Option primitives (Length/Color) | Categoria diferente — atributos cosméticos primitivos |
| **P297 MathUnderover** | **Option `Box<Content>` estrutural** | **Primeira qualificação genuína** |

`under` / `over` são **estruturais** (não cosméticos):
- `None` = "elemento ausente" semanticamente.
- `Some(c)` = "elemento presente com conteúdo `c`".
- Layouter trata cada caso distintamente (skip vs render).

**Promoção candidata robusta** — primeira ocorrência válida do
padrão. **Adiada** per P273.17 §0:
- §8.7' N=5 também qualifica simultaneamente.
- Promover um forçaria escolha arbitrária.
- Consolidação cumulativa P298+ permitirá promoção mais robusta.

### §6.5 — Sub-padrão "cluster math handler dedicado" (**N=2 cumulativo P296+P297**)

P296 inaugurou `layout_accent`/`layout_cancel` como handlers
dedicados em `rules/math/layout/mod.rs`. P297 consolida com
`layout_underover` — sub-padrão N=2.

**Não promovido** — sub-padrão emergente; consolidação em N=3+
(extensão `op`/refinos futuros).

### §6.6 — ADR-0098 "single source of truth" (**N=14 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 14º passo consecutivo.
Invariante robusta sobre 14 features distintas (P282-P297):
- Style cumulativos P288-P292.
- Curve P293/P294.
- Footnote P295.
- Math accent/cancel P296.
- **Math underover P297** — agregação cluster vanilla 12→1.

### §6.7 — Refutação definitiva §6.6 P295 (degenerescência)

P295 §10 hipotetizou: *"4 reaplicações A.0.0 consecutivas com
magnitude decrescente ou factual-modesta → template degenera em
ritual procedimental."*

**P296 refutou em N=4 com magnitude média.**
**P297 refuta categoricamente em N=5 com magnitude alta.**

A magnitude **não é monotonicamente decrescente** — flutua mas
contém picos significativos:
- P295 baixo → P296 médio → P297 alto.
- Tendência **não-decrescente** em janela 3-pasos.

**Conclusão**: template §8.7' **NÃO degenera**. Continua a gerar
valor empírico genuíno. Vigilância em P298+ permanece mas com
menor probabilidade de degenerescência.

---

## §7 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 119** dividida em 2 (separação de `underover` vs `op`):
  - `underover`: `parcial` → `implementado ⁸²` (P297; HV'.a
    agregação; nota explícita sobre refutação magnitude alta).
  - `op`: permanece `parcial` (P296.2 candidato dedicado).

---

## §8 — Frentes pendentes pós-P297

| Frente | Tipo | Estado |
|---|---|---|
| **P297.X** — discriminator `UnderoverKind::Brace`/`Bracket`/`Paren`/`Shell` para paridade visual fina vanilla | XS+ | Cosmético per ADR-0054 graded |
| **P296.2** — `op` (vanilla `OpElem`) | XS+S | Extensão directa do cluster math |
| **P296.X** — `cancel(..., inverted: true)`/`cross: true` toggles funcionais | XS | Toggles binary não-cosméticos |
| Cosméticos `size`/`length`/`angle`/`stroke`/`dotless` | XS | ADR-0054 graded |

---

## §9 — Decisão sobre P298

Candidatos disponíveis:

1. **P296.2 op** — extensão directa cluster math (paridade P296+P297).
2. **P297.X discriminator UnderoverKind** — refino visual.
3. **P296.X toggles cancel** — extensão cosmética.
4. **P295.1 nota rodapé** — extensão directa P295 magnitude L.
5. **`curve.move`/scope-methods** — sintaxe vanilla fiel.
6. **Outras frentes** P282 §6.

Decisão fica para o operador humano. **P297 não dita P298**.

Se P298 fechar cluster math (op + P297.X), **sub-padrão "cluster
math handler" atingiria N=3** — promoção candidata robusta.

Se P298 reaplicar A.0.0 com refutação **modesta** após P297 alta,
mantém-se confiança no template §8.7' (magnitude variada é
saudável).

---

## §10 — Honestidade epistémica

P297 inverte conclusivamente a tendência presumida em P295 §10:

| Cenário antecipado spec P297 | Realidade |
|---|---|
| "Confirmação esperada" (magnitude baixa) | **Refutado** — magnitude alta |
| Decisão trivial paralela P296 | **Refutado** — decisão A.2 → (b) genuína |
| Materialização rotineira | **Refutado** — agregação cristalina vs vanilla 12 fragmentos |

**Lições metodológicas**:

1. **A.0.0 deve preceder pressuposto** — sem inspecção literal,
   P297 teria implementado wrapper vanilla inexistente.
2. **Magnitude alta cumulativa valida template** — refuta
   degenerescência mesmo com magnitude flutuante.
3. **Disparo simultâneo de gatilhos requer adiamento** — §8.7' N=5
   e "variant rico" N=5 simultâneos forçam preservação para
   consolidação.

Diagnose honesta: P297 produziu **2 candidatos genuínos a
promoção ADR meta** mas escolheu **adiar ambos** — anti-padrão
over-formalização P273.17 §0 vigente.

---

## §11 — Fecho

P297 fechado com:

- **+11 testes** (9 L1 + 2 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** bit-exact (14º passo consecutivo).
- **Hash `content.rs`** mudou esperadamente (+1 variant agregado).
- **0 ADRs meta novas** — §8.7' N=5 + "variant rico" N=5 ambos
  qualificam genuinamente; ambos **adiados** per P273.17 §0.
- **Padrão "variant rico" N=5 candidato genuíno** — primeira
  qualificação Option estrutural desde P287 refutação.
- **Sub-padrão "cluster math handler dedicado" N=2 cumulativo**
  (P296+P297) — não-promovido (sub-padrão emergente).
- **Hipótese degenerescência §6.6 P295 categoricamente refutada**
  — A.0.0 N=5 com magnitude alta.

**MARCO P297**:
- **Extensão directa P296** com **decisão estrutural A.2 → (b)
  genuinamente nova** (Option `Box<Content>` estrutural).
- **A.0.0 N=5 magnitude alta** — refuta categoricamente
  degenerescência §6.6 P295.
- **Vanilla underover.rs revela 12 elementos** — spec P297
  invalidada (paralelo P294 arquitectural); cristalino agrega
  per ADR-0054 graded.
- **`MathUnderover` materializado como variant agregado** —
  Tabela A.4 linha 119 corrigida `parcial` → `implementado`.
- **Hash `export.rs` preservado pelo 14º passo consecutivo**
  (P282→P297) — ADR-0098 robusta sobre 14 features distintas.
- **Cluster math 3/4 features fechadas** (accent+cancel+underover);
  `op` permanece para P296.2 dedicado.
- **Tendência A.0.0 magnitude não-decrescente confirmada** —
  janela P294-P297 prova template robusto.
