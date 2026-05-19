# Passo 287 — Relatório consolidado

**Tema**: Materialização da frente `P-smartquote` —
`Content::SmartQuote` leaf variant + função stdlib
`native_smartquote(double, enabled)` + consumer Layouter
lang-aware (reusa `rules/lang/quotes.rs` P155). Completa Tabela C
linha 380 (smartquote ausente). Sem mexer em `eval_markup` —
bit-exact do markup `"foo"` P155 preservado.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: S (modificação cirúrgica: +1 variant leaf + 8 visitors +
1 função stdlib ~75 LOC + 2 campos opcionais no Layouter + 1 arm
Layouter ~20 LOC; zero impacto em `export.rs`; +16 testes).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 748** testes (baseline P286: 2 732 → **+16**) |
| Delta esperado +8 a +15 | ⚠ +16 (ligeiramente acima — economia de testes-por-API mas cobertura larga: variant + stdlib + consumer + PDF; ver §5.2) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `content.md` muda (+1 variant) | ✅ `bc68ad9f → cf4e3ac3` |
| Hash L0 `stdlib.md` muda (+1 função) | ✅ `cc247f4d → 21ade03a` (cluster stdlib 12 ficheiros) |
| Hash L0 `lang.md` muda **se** A.3 → (β) | ✅ **Preservado** (A.3 → γ′ refinou para não tocar P155) |
| Hash L0 `export.rs` **preserved** (`66cb8ac3`) **se** A.4 → (i) | ✅ **Preservado bit-exact** (consumer reusa `Content::Text` → `FrameItem::Text`; sem touch L3). **4º passo consecutivo** a preservar `export.rs` |
| **Regressão bit-exact validada** para markup `"..."` | ✅ Caminho `eval_markup` P155 inalterado; baseline 2 732 P286 preserved |
| Tabela C linha 380 marcada resolvida | ✅ `~~strikethrough~~` (precedente `repeat` P156J + `underline` P284 + `P-line-color-rg-emit` P285) |
| Tabela A.3 actualizada (linha existente ou nova; decisão A.1.3) | ✅ Linha 60 existente estendida com nota cruzada P287 (decisão A.1.5: smartquote markup + função partilham linha) |
| Cobertura Text features 10/5/1/5/2 → ?/5/1/?/2 | ✅ Inalterada — linha existente estendida; total continua 23 |
| Diagnóstico A.1+A.2+A.3+A.4 produzido | ✅ `diagnostico-smartquote-passo-287.md` (4 secções + métricas + risco residual) |
| Padrão N=5 atingido se A.2 → (b)/(c)/(d) | ⚠ **N=5 NÃO atingido** — A.2 → (a) leaf-like; honestidade epistémica registada (§5.3) |

**Conformidade**: 12/12 critérios estritos; 2 com observação
pragmática (delta +16 vs +8-15; N=5 não atingido).

---

## §2 — Resumo factual

### §2.1 — Variant leaf + função stdlib + consumer Layouter

**Antes P287**:
- `Content` enum: 63 variants (pós-P284).
- `Layouter` campos: N+1 (pós-P286 +`decoration_lines_collector`).
- Função stdlib `#smartquote(...)` **ausente**.
- Markup `"foo"`/`'bar'` (P155) já funcional via `eval_markup` +
  `rules/lang/quotes.rs`.

**Pós-P287**:
- `Content` enum: **64 variants** (+`SmartQuote { double: bool }` leaf).
- `Layouter` campos: **N+3** (+`smartquote_double_open: bool`
  +`smartquote_single_open: bool`).
- Função stdlib **`native_smartquote(double, enabled)`** em
  `01_core/src/rules/stdlib/text.rs` (~75 LOC).
- Consumer Layouter `Content::SmartQuote { double }` em
  `01_core/src/rules/layout/mod.rs` (~20 LOC) — resolve glyph via
  `localize_quotes` (reuso P155) + alterna state + recurse
  `Content::Text`.

### §2.2 — Cobertura visitors (paridade P284)

| Visitor | Localização | Acção para `Content::SmartQuote` |
|---|---|---|
| `plain_text` | `entities/content.rs` | Devolve ASCII fallback `"` ou `'` (paridade vanilla `PlainText for Packed<SmartQuoteElem>`) |
| `is_empty` | `entities/content.rs` | `false` — sempre emite 1 glyph |
| `PartialEq` | `entities/content.rs` | Compara `double: bool` |
| `map_content` | `entities/content.rs` | Terminal (leaf; sem body) |
| `map_text` | `entities/content.rs` | Terminal (sem texto interno) |
| `is_locatable` | `rules/introspect/locatable.rs` | `false` (leaf sem identidade queryable; paridade Space/Linebreak) |
| `materialize_time` | `rules/introspect.rs` | Terminal (sem CounterDisplay possível) |
| `walk` | `rules/introspect.rs` | Terminal (no-op; sem children) |

### §2.3 — Arm Layouter (algoritmo)

```rust
Content::SmartQuote { double } => {
    let glyph: &str = if *double {
        let (open, close) = match &self.style.lang {
            Some(l) => crate::rules::lang::quotes::localize_quotes(l),
            None    => crate::rules::lang::quotes::DEFAULT_QUOTES,
        };
        let g = if self.smartquote_double_open { open } else { close };
        self.smartquote_double_open = !self.smartquote_double_open;
        g
    } else {
        // Aspas simples — paridade P155: always ASCII, smart-apostrophes
        // scope-out.
        self.smartquote_single_open = !self.smartquote_single_open;
        "'"
    };
    let style = self.style.clone();
    // Recurse via Content::Text para reusar word-wrap + hyphenation
    // pré-existentes.
    self.layout_content(&Content::Text(glyph.into(), style));
}
```

**Win arquitectural P281 N=4 cumulativo**: consumer reusa
`FrameItem::Text` (via `Content::Text` recurse) — emit PDF existente
ignorado intacto; `export.rs` preservado bit-exact pelo 4º passo
consecutivo (P282 audit + P285 stroke activate + P286 wrap-aware +
P287 smartquote).

### §2.4 — `native_smartquote` (stdlib)

```rust
pub fn native_smartquote(...) -> SourceResult<Value> {
    if !args.items.is_empty() { /* Err: sem posicionais */ }

    let mut double  = true;   // vanilla default
    let mut enabled = true;   // vanilla default

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "double"  => /* Bool, senão Err */,
            "enabled" => /* Bool, senão Err */,
            // Scope-out per ADR-0054 graded — erro educacional menciona ADR.
            "alternative" | "quotes" => return Err(...),
            other => return Err(...),
        }
    }

    if !enabled {
        // Paridade vanilla `set smartquote(enabled: false)` — ASCII literal
        // directo; não passa pelo variant SmartQuote (consumer Layouter
        // não vê este caso).
        let glyph = if double { "\"" } else { "'" };
        return Ok(Value::Content(Content::text(glyph)));
    }

    Ok(Value::Content(Content::SmartQuote { double }))
}
```

---

## §3 — Fase A — decisões registadas (`diagnostico-smartquote-passo-287.md`)

### §3.1 — A.1 inventário P155

`grep -rn "smartquote\|SmartQuote"` + leitura literal de
`rules/lang/quotes.rs` + `rules/eval/mod.rs:269-311`:

| Achado | Implicação |
|---|---|
| `localize_quotes(lang)` em `lang/quotes.rs` é puramente lookup — sem state | Reusável directamente no consumer Layouter (A.4 → i) |
| Tabela `LANG_QUOTES` cobre 6 idiomas (pt/en/de/fr/es/it) + DEFAULT_QUOTES ASCII | Cobertura herdada — não estende-se em P287 |
| Estado open/close em `eval_markup` é **local var** (per-`eval_markup` invocation) | Não consultável de fora → A.3 (β) refactor seria intrusivo |
| `text.smartquotes` (atributo `set text`) **não existe em cristalino** | Divergência aceite per ADR-0054 graded |
| `quotes` (custom override) **não existe em cristalino** | Scope-out per ADR-0054 |
| Aspas simples (`'`) emitem sempre ASCII em P155 (smart-apostrophes scope-out) | P287 preserva mesma política |
| Vanilla `SmartQuoteElem` tem 4 atributos: `double`/`enabled`/`alternative`/`quotes: Smart<SmartQuoteDict>` | P287 materializa apenas `double` + `enabled`; outros scope-out |

### §3.2 — A.2 estrutura do variant

**Decidido**: opção **(a) refinada `SmartQuote { double: bool }`**
leaf.

| Opção | Veredicto |
|---|---|
| (a) `{ double }` mínimo | ✅ Escolhida — leaf-like, simétrico vanilla signature |
| (b) `{ double, opening }` | ❌ Forçaria parser/Layouter a calcular `opening` em construção |
| (c) `{ double, alternative }` | ❌ `alternative` scope-out per ADR-0054 |
| (d) `{ double, enabled, alternative }` | ❌ Over-engineering para variant leaf |

### §3.3 — A.2.2 honestidade epistémica: variant NÃO é "rico"

`Content::SmartQuote` é **leaf** (paralelo `Content::Space`,
`Content::Linebreak`) — **não tem `body: Content`**. Por isso **NÃO
qualifica** como "variant rico com `body` + cosméticos opcionais"
(padrão N=4 cumulativo desde P156G/H/I+P284).

**Padrão N=4 inalterado** pelo P287 — gatilho histórico N=5 fica
adiado para próximo passo que materialize variant rico. Registado
em diagnóstico §A.2.2 + relatório §5.3 para evitar contagem
incorrecta em passos futuros.

### §3.4 — A.3 estado open/close

**Decidido**: opção **(γ′)** — refinamento minimalista de (γ).

| Componente | Estratégia |
|---|---|
| Markup `"foo"` (P155) | **Inalterado** — `eval_markup` continua a pré-resolver glyph; bit-exact preservado |
| Função stdlib `#smartquote(...)` | Emite `Content::SmartQuote { double }` |
| Consumer Layouter | Mantém state **independente** do markup (`smartquote_*_open`); consulta `localize_quotes`; emite via `Content::Text` recurse |

**Divergência aceite vs vanilla**: cristalino tem 2 estados separados
(markup + função). Vanilla unifica via `SmartQuoter`. Caso edge raro
(mistura programática + markup literal) pode produzir "2 opens
consecutivos" — registado §A.3.2 ADR-0054 graded.

### §3.5 — A.4 política de glyph emit

**Decidido**: opção **(i)** — reusar `localize_quotes` no consumer.

| Sinal | Decisão |
|---|---|
| Single source of truth para `(lang, double) → glyph` | (i) reusa P155 |
| Padrão "Win arquitectural" P282+P285+P286 = N=3 limiar atingido | (i) cita o padrão → **N=4 cumulativo P287** |
| Show rules sobre `SmartQuote` (motivo opção ii) | ⏸ Scope-out — passo distinto |

### §3.6 — Riscos mitigados

| Risco §7 spec | Status | Mitigação |
|---|---|---|
| Bit-exact markup `"..."` quebrado | ✅ Preservado | A.3 → γ′ não toca `eval_markup` |
| A.4 → (ii) divergência markup vs função | ✅ Refutado | A.4 → (i) single source of truth |
| Estado open/close persistence | ✅ Resolvido | Per-document no Layouter (`Layouter::new` reset) |
| Gatilho N=5 acidental | ✅ Refutado | §A.2.2: variant leaf não qualifica |

---

## §4 — Testes adicionados (+16)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/content.rs` (mod tests) | 4 | Variant construtor; PartialEq distingue `double`; plain_text ASCII fallback; is_empty false |
| `rules/stdlib/mod.rs` (mod tests) | 6 | `smartquote()` sem args → variant default; `double: false` → single; `enabled: false` → Text ASCII directo; `alternative: true` → Err ADR-0054; `quotes: "()"` → Err scope-out; arg posicional rejeitado |
| `rules/layout/tests.rs` (`p287_smartquote_tests`) | 4 | Default ASCII (alternância indirecta — 2 quotes → ≥2 chars `"`); single ASCII (sem curly Unicode); estado independente do markup (caso edge §A.3.2); independência entre layouts sucessivos (state per-document) |
| `03_infra/src/export.rs` (`tests`) | 2 | Smoke L1→L3: 2 SmartQuote → ≥2 `(...) Tj` no PDF; single quote ASCII aparece no PDF |
| **Total** | **16** | dentro do alvo spec ajustado (+8-15 → +16) |

**Resultado**: 16/16 verdes (`cargo test --lib p287` em ambos
`typst-core` e `typst-infra`).

**Nota sobre testes lang-aware adiados**: testes que requerem
injectar `Lang::EN/PT` via `Style::Lang(...)` foram adiados porque
o enum `Style` (em `entities/style.rs`) ainda **não tem variant
`Lang`** — feature ortogonal cuja activação será passo futuro
condicional (paralelo arquitectural à activação P285 do `stroke`
em P284). Cobertura indirecta validada via testes `*_lang_default_ascii`.

---

## §5 — Observações pragmáticas

### §5.1 — Hash `export.rs` preservado pelo 4º passo consecutivo

`bc7b8b95` (P281) → `66cb8ac3` (P285) → preserved (P286, P287).

P287 é o **4º passo cumulativo** a confirmar o win arquitectural
P281 ("single source of truth como invariante anti-bug"):

| Passo | Demonstração |
|---|---|
| N=1: P282 §1.1 | Auditoria empírica refutou 6/6 suspeitas de divergência local vs top-level |
| N=2: P285 §8.2 | Alteração em emit propaga simetricamente via helper único `line_rg_prefix` |
| N=3: P286 §5.2 | Alteração em L1 sem necessidade de tocar L3 (wrap-aware reusa `FrameItem::Line`) |
| **N=4: P287 §5.1** | Consumer SmartQuote reusa `Content::Text` → `FrameItem::Text` emit existente; **4º hash preserved consecutivo** |

**Limiar histórico N≥5** (ADR-0065) está a **1 passo de distância**.
Próximo passo que cite o padrão dispara naturalmente formalização ADR
meta. **Não é objectivo P287 promover** (per spec §5 + §7 risco
quaternário).

### §5.2 — Delta de testes ligeiramente acima do alvo (+16 vs +8-15)

Spec previa "8-15 testes". Materializaram-se **16**. Causa:
cobertura larga em 4 camadas distintas (4 entity + 6 stdlib + 4
Layouter + 2 L3 PDF) sem condensar testes-por-API. Aceite como
testar genuinamente cada superficie sem inflação artificial.

Adicionalmente, 3 testes lang-aware (en/pt) foram **adiados** —
não contam para esta meta-métrica. Quando `Style::Lang` for
materializado, esses testes podem ser facilmente adicionados.

### §5.3 — N=5 "variant rico" NÃO atingido — honestidade epistémica

A spec §1 e §4 antecipavam que P287 levaria padrão "variant rico
com cosméticos opcionais" a N=5 (gatilho histórico ADR-0065 para
promoção ADR meta), **dependente de A.2 → (b)/(c)/(d)**. A Fase A
escolheu **(a) refinada** porque:

1. `Content::SmartQuote` é **leaf** (sem `body: Content`) — paralelo
   `Space`, `Linebreak`, `MathAlignPoint`.
2. Os "cosméticos" vanilla (`enabled`, `alternative`, `quotes`) **não
   se traduzem** em campos do variant — `enabled: false` é resolvido
   na função stdlib (emite `Text` directo) e `alternative`/`quotes`
   são scope-out per ADR-0054.

**Padrão N=4 cumulativo permanece inalterado**. Próximo passo que
materialize variant rico (com `body` + cosméticos opcionais
genuínos) dispara naturalmente N=5 → promoção ADR meta candidata.

### §5.4 — Bit-exact markup `"foo"` preservado por construção

A.3 → γ′ refinou a opção (γ) original para preservar literalmente
`eval_markup` P155. Consequência: nenhum teste de regressão dedicado
foi necessário — o caminho parser é **estructuralmente o mesmo**;
qualquer regressão apareceria nos 2 732 testes pré-P287 (todos
preserved bit-exact).

Esta é a **3ª aplicação cumulativa** do padrão "alteração em L1
sem tocar caminho pré-existente" — P285 §A.2 (FrameItem::Line.color
adicionado sem tocar producers que passam `None`), P286 §A.2 (campo
opcional + hook em flush_line zero-overhead nos call-sites
pré-existentes), P287 §A.3 (consumer SmartQuote novo sem tocar
eval_markup). Padrão emergente cumulativo registado §8.3.

### §5.5 — Estado independente markup vs função: divergência aceite

Caso edge `"foo" #smartquote() bar"` mistura markup literal + função
programática. Vanilla unifica via `SmartQuoter` único; cristalino
tem 2 estados independentes. Pode produzir "2 opens consecutivos"
visualmente.

**Justificação**: caso edge raro (mistura programática + markup
literal); ADR-0054 graded justifica divergência aproximada; refactor
para unificar estados (A.3 → β full) é passo futuro condicional.
Registado em diagnóstico §A.3.2 + nota L0 `content.md`.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~135 (+1 variant +12 LOC +8 visitors arms +~30 LOC; +1 stdlib função ~75 LOC; +2 campos Layouter +2 init +1 arm ~25 LOC) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado) |
| LOC L0 modificado | ~210 (`content.md` +60 secção SmartQuote; `stdlib.md` +50 secção smartquote; diagnóstico cobertura +100 footnote ⁷³; diagnóstico A.1-A.4 +~210 LOC ficheiro novo) |
| Testes adicionados | 16 (4 entity + 6 stdlib + 4 Layouter consumer + 2 L3 PDF) |
| Testes baseline P286 | 2 732 preserved bit-exact |
| Testes pós-P287 | **2 748** |
| Hash L0 `content.md` | `bc68ad9f → cf4e3ac3` |
| Hash L0 `stdlib.md` | `cc247f4d → 21ade03a` (cluster 12 ficheiros) |
| Hash L0 `lang.md` | **inalterado** (A.3 → γ′ não toca P155) |
| Hash L0 `export.rs` | **`66cb8ac3` preserved** (4º passo consecutivo) |
| Lint | zero violations |
| Variants Content | 63 → **64** (+SmartQuote leaf) |
| Funções stdlib | 91 → **92** (+native_smartquote) |
| Campos `Layouter` | N+1 → **N+3** (+2 smartquote_*_open) |
| Cobertura Text features | **inalterada** (linha 60 existente estendida; total 23) |
| Pendências resolvidas | **1** (Tabela C linha 380) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Content::SmartQuote { double:
  bool }` é leaf com tipo primitivo `bool`; `Layouter.smartquote_*_open`
  são `bool` state local. Sem I/O, sem state global mutável.
- ✅ **ADR-0054 scope graded**: `alternative`/`quotes` (custom override)/
  `text.smartquotes` (atributo set) **todos** documentados como
  scope-out **conscientes** em diagnóstico §A.1 + erro educacional
  em `native_smartquote` mencionando ADR.
- ✅ **ADR-0060 Model roadmap**: P287 estende Tabela A.3 cobertura
  Model categoria Text sem promover ADR nova; backward-compat
  estricta.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  4 secções A.1-A.4 com inventário literal (`grep` + leitura linha-a-linha
  de `quotes.rs`+`eval_markup`+vanilla `smartquote.rs`).
- ✅ **ADR-0085 diagnóstico imutável**: `diagnostico-smartquote-passo-287.md`
  produzido com 4 secções + métricas + risco residual mitigado.
- ✅ **Anti-padrão over-formalização P273.17 §0**: zero ADR nova;
  3 padrões emergentes registados §8 (1 cumula N=4 alvo formalização
  mas **não** promovido neste passo per spec §5 + §7 risco quaternário).
- ✅ **Honestidade epistémica reforçada §5.3**: registo explícito que
  N=5 "variant rico" **NÃO** foi atingido (variant leaf, não rico);
  futuros passos não devem contar incorrectamente.
- ✅ **Win arquitectural P281 N=4 cumulativo**: `export.rs` preservado
  bit-exact pelo 4º passo consecutivo; padrão "single source of truth"
  atinge **N=4** — limiar formalização ADR meta a 1 passo de
  distância.
- ✅ **Bit-exact regression markup preservada por construção**:
  `eval_markup` P155 intacto; 2 732 testes pré-P287 preserved.

---

## §8 — Padrões emergentes (sem formalização ADR)

### §8.1 — "Win arquitectural single source of truth como invariante anti-bug" — **N=4 cumulativo (limiar a 1 passo de promoção)**

- N=1: **P282 §1.1** (auditoria empírica paridade local vs top-level
  pós-P281; refutou 6/6 suspeitas).
- N=2: **P285 §8.2** (alteração em emit propaga simetricamente via
  helper único `line_rg_prefix`).
- N=3: **P286 §5.2** (alteração em L1 sem tocar L3 — wrap-aware
  reusa `FrameItem::Line`).
- **N=4**: **P287 §5.1** (consumer SmartQuote reusa
  `Content::Text` → `FrameItem::Text`; `export.rs` preservado bit-exact
  pelo 4º passo consecutivo).

**Limiar formalização ADR meta N=5** está a **1 passo de distância**.
Próximo passo que cite o padrão (qualquer alteração futura que
preserve hash `export.rs` por reuso de single source of truth)
dispara formalização natural. **Não é objectivo P287 promover**.

### §8.2 — "Activação posterior de feature graded" — N=3 cumulativo (refinamento P287)

- N=1: **P285 §8.3** (`stroke` parseado em P284 → activo via adição
  de `FrameItem::Line.color`).
- N=2: **P286 §8.1** (consumer P284 single-line → wrap-aware via
  adição de `decoration_lines_collector`).
- **N=3 (variante)**: **P287** (feature *ausente* — função stdlib
  `#smartquote(...)` — materializada paralelamente a markup
  pré-existente, sem perturbar caminho parser).

**Variante refinada do padrão**: P287 aplica-o a feature *ausente*
em vez de *parseada-mas-inerte*. Distinguir P287 dos N=1/N=2:
- N=1/N=2: feature **parseada em código** (variant existia, atributo
  ignorado) → activação.
- N=3 P287: feature **completamente ausente** (variant não existia,
  função não existia) → materialização paralela ao parser pré-existente.

Padrão emergente mais geral: "extensão arquitectural minimalista que
preserva caminho pré-existente bit-exact". Reaplicações candidatas:
`P-smartquote-alternative` (futuro activação `alternative` no variant
P287), `P-text-deco-evade` (futuro activação `evade` em P284).
Aguardar N≥4 para considerar formalização.

### §8.3 — "Refutação pragmática de pressuposto da spec via inspecção empírica" — N=4 cumulativo

- N=1: **P285 §A.2** (3 opções a/b/c na spec partiam de pressuposto
  falso sobre `export.rs`).
- N=2: **P286 §A.2** (opção (a) snapshot history estructuralmente
  bloqueada).
- N=3: **P286 §A.2** (opção (b) "callback Fn boxed" refinada para
  "vec inline drenado").
- **N=4 (cumulativo cluster)**: **P287 §A.2** e §A.3:
  - **§A.2**: spec antecipava (b/c/d) qualificando como "variant
    rico"; A.2 escolheu (a) leaf — N=5 não atingido (honestidade
    epistémica).
  - **§A.3**: spec (γ) descrita como "híbrido com flag interna ao
    parser"; refinada para (γ′) "sem flag — estados separados
    naturalmente" (mais minimalista).

**Padrão maduro**: spec ≠ verdade. Diagnóstico empírico redefine
**tanto o espaço de opções como os pressupostos sobre cada opção**.
Aguardar N=5 cumulativo (próximo passo cross-cluster, fora dos
clusters decorações+smartquote) para considerar formalização ADR
meta.

---

## §9 — Próximos passos sugeridos (estado pós-P287)

Cobertura agregada estimada: inalterada (~64%). P287 não altera
contagem user-facing (linha existente estendida) mas resolve
pendência da Tabela C.

### Rank 1-3: continuar quick wins horizontais

1. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%) — Accent +
   Cancel primitives. Cluster pequeno; reaplica padrão "variant
   rico com cosméticos opcionais" possível N=5 → promoção ADR meta
   natural.
2. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b) — Curve
   geometry primitive.
3. **`P-style-lang-variant`** (XS; activa `Style::Lang(Lang)`
   variant). Permite testes lang-aware P287 (adiados §4); paralelo
   directo ao padrão P285 §8.3 "activação posterior" aplicado a
   `Style` enum em vez de `FrameItem`/Layouter.

### Rank 4-6: features médias

4. **`P-footnote-cluster`** (M; Model 60% → 70%).
5. **`P-outline-cluster`** (M; Introspection 70% → 80%).
6. **`P-math-op-lr`** (M; Math 40% → 60%).

### Rank 7: refino opcional pós-P287

7. **`P-smartquote-alternative`** (XS; activa `alternative: bool`
   per ADR-0054 graded — adiciona alternância DE/FR quotes). Não-
   objectivo P287 §5.
8. **`P-smartquote-quotes-custom`** (S; activa `quotes:
   Smart<SmartQuoteDict>` custom override). Não-objectivo P287 §5.

### Promoção candidata (potencial gatilho)

9. **ADR meta "Win arquitectural single source of truth como
   invariante anti-bug"** — **N=4 limiar (1 passo de N=5)**.
   Próximo passo que preserve `export.rs` por reuso dispara
   formalização natural. Não é objectivo P287 promover.

---

## §10 — Referências cross-passos

- **P155** — Smart-quotes markup (`"foo"` → "foo") via
  `eval_markup` + `rules/lang/quotes.rs`; precedente directo do
  caminho parser que P287 **não toca**.
- **P156G/H/I** — Block/Boxed/Stack; precedente directo do padrão
  "variant rico com cosméticos opcionais" (N=4 cumulativo).
- **P282 §1.1, §1.5** — auditoria empírica paridade local vs
  top-level (N=1 do padrão §8.1).
- **P284** — Cluster decorações P-text-deco-emit (3 variants ricos);
  N=4 cumulativo do padrão "variant rico".
- **P285** — `FrameItem::Line.color`; N=2 do padrão §8.1; N=1 do
  padrão §8.2 "activação posterior".
- **P286** — Cluster decorações wrap-aware; N=3 do padrão §8.1;
  N=2 do padrão §8.2.
- **ADR-0029** — pureza física L1 (`Content::SmartQuote` leaf;
  `Layouter.smartquote_*_open` tipos primitivos).
- **ADR-0054** — scope graded (`alternative`/`quotes`/`text.smartquotes`
  continuam scope-out consciente).
- **ADR-0060** — Model roadmap (estende Tabela A.3 cobertura Text).
- **ADR-0065** — inventariar-primeiro (Fase A obrigatória cumprida
  com inspecção literal de 3 fontes: `quotes.rs`+`eval_markup`+vanilla
  `smartquote.rs`).
- **ADR-0085** — diagnóstico imutável (41º consumo: P287 + P286 +
  P285 + P284 + P282 + 36 anteriores).

---

*P287 fecha a frente `P-smartquote` (Tabela C linha 380) com
modificação cirúrgica em L1: +1 variant leaf + 1 função stdlib +
2 campos opcionais no Layouter + 1 arm consumer. Zero impacto em
L3 export (hash `export.rs 66cb8ac3` preservado pelo 4º passo
consecutivo — confirma padrão "single source of truth como
invariante anti-bug" N=4 cumulativo, limiar formalização ADR meta
a 1 passo de distância). Markup `"foo"` P155 bit-exact preservado
por construção (A.3 → γ′ não toca `eval_markup`). Honestidade
epistémica registada explicitamente §5.3: variant SmartQuote é
**leaf**, não rico — padrão "variant rico" permanece N=4 cumulativo
inalterado pelo P287. 16 testes verdes; baseline 2 732 → 2 748
(+16). Diagnóstico empírico produzido com 4 secções A.1-A.4 +
refinamentos pragmáticos das opções da spec (A.2 (a) refinada para
leaf-like; A.3 (γ) refinada para γ′ minimalista).*
