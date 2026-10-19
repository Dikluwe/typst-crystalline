# Diagnóstico — Fase A do Passo 288 (`P-style-lang-variant`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-288.md`
**Origem**: P287 §4 / §5.2 (testes lang-aware adiados); Tabela B.3
(5 variants `Style` sem `Lang`) + Tabela B.4 (`StyleDelta.lang`
`implementado⁺` desde P144).

---

## A.1 — Inventário do caminho actual `lang` na cascade

### A.1.1 — `Style` enum (`entities/style.rs:25-36`)

5 variants (`Copy`, `PartialEq`):

| Variant | Tipo |
|---|---|
| `Bold(bool)` | bool |
| `Italic(bool)` | bool |
| `Size(Pt)` | Pt |
| `Fill(Color)` | Color (Copy) |
| `HeadingLevel(u8)` | u8 |

**Lang ausente** — confirmado literalmente.

### A.1.2 — `StyleDelta` (`entities/style_chain.rs:30-64`)

10 fields:
- `bold`, `italic`, `size`, `fill`, `heading_level` (paralelos aos 5
  `Style` variants).
- `weight`, `tracking`, `leading`, **`lang: Option<Lang>`**, `font`
  (sem variants `Style` correspondentes).

**Assimetria detectada**: 5 fields têm variant `Style` paralelo; **5
fields têm-no apenas via parse de `#set text(...)` directo** (P102
ADR-0040 + P130/P131B/P132B/P136-139/P144).

### A.1.3 — `StyleChain::push_styles` (`style_chain.rs:134-146`)

`match` exaustivo sobre 5 variants:

```rust
pub fn push_styles(&self, styles: &Styles) -> Self {
    let mut delta = StyleDelta::empty();
    for style in styles.iter() {
        match style {
            Style::Bold(b)         => delta.bold = Some(*b),
            Style::Italic(i)       => delta.italic = Some(*i),
            Style::Size(pt)        => delta.size = Some(pt.val()),
            Style::Fill(c)         => delta.fill = Some(*c),
            Style::HeadingLevel(l) => delta.heading_level = Some(*l),
        }
    }
    self.push(delta)
}
```

**`Style::Lang(...)` adicionado em P288 deve produzir
`delta.lang = Some(*l)`** — paralelo exacto aos 5 existentes.

### A.1.4 — `delta.lang` write site único (`eval/rules.rs:377-394`)

```rust
"lang" => {
    if let Value::Str(s) = val {
        match Lang::from_str(&s) {
            Ok(lang) => delta.lang = Some(lang),
            Err(msg) => { return Err(/* validação ISO 639 */); }
        }
    }
}
```

**Caminho exclusivo até P288**: `#set text(lang: "de")` é o único
produtor de `delta.lang`. Não há outro acesso via `Styles` collection
ou `Content::Styled` API.

### A.1.5 — `delta.lang` read site único (`style_chain.rs:226-229`)

```rust
pub fn lang(&self) -> Option<Lang> {
    let mut node = self.0.as_deref();
    while let Some(n) = node {
        if let Some(v) = n.delta.lang { return Some(v); }
        node = n.parent.as_deref();
    }
    None
}
```

Walk normal up-the-chain — paralelo aos restantes accessors.

### A.1.6 — Consumers de `style.lang`

`grep -rn "style.lang\b\|chain.lang()\|engine.styles.lang"`:

| Consumer | Localização | Função |
|---|---|---|
| `eval_markup` (P155) | `eval/mod.rs:293` | Resolve aspas smart-quote via `localize_quotes(&lang)` |
| Hyphenation (P144) | `layout/cursor.rs:56` | Activa `hypher::hyphenate(word, &lang)` |
| Figure supplement (P158B) | `lang/figure_supplement.rs` | Resolve "Figura"/"Figure" lang-aware |
| Smartquote consumer (P287) | `layout/mod.rs:1992` | Resolve aspas via `localize_quotes` |
| `TextStyle::from(&StyleChain)` | `style_chain.rs:286` | Captura lang para `TextStyle` (FrameItem::Text.style.lang) |
| Reflectors (P285) | vários | Pass-through pos-write |

**Todos consultam `chain.lang()` (read-only) — nunca persistem `Lang`
em `FrameItem`** (excepto `TextStyle.lang` que serve apenas para
diagnóstico futuro de shaping; export.rs NÃO consulta).

### A.1.7 — `FrameItem::Text` e emit `export.rs`

`grep -rn "style.lang\|text\.lang" 03_infra/src/export.rs`:
**zero hits**. O exportador L3 NÃO consulta `lang` — `TextStyle.lang`
é dead-code em emit (preserva forward-compat mas não influencia
PDF). Confirma **paradigma vanilla**: `lang` é input para layout-time
decisions (hyphenation, smartquote, supplement) mas não para emit
PDF estructural.

### A.1.8 — Diagrama de fluxo actual

```
#set text(lang: "de")
       │
       ▼
parse → eval_set_rule (eval/rules.rs:377)
       │
       ▼
delta.lang = Some(Lang::DE)            ← write directo, bypass `Style` enum
       │
       ▼
chain.push(delta) → StyleChain
       │
       ▼
chain.lang() (style_chain.rs:226)      ← read via walk up-the-chain
       │
       ├──► eval_markup → localize_quotes (P155)
       ├──► Layouter (hyphenation, smartquote consumer P287, ...)
       └──► TextStyle.lang (FrameItem::Text — dead-code em emit)
              │
              ▼
       export.rs:2256 (emit_text_pdf) → IGNORADO (sem branch lang)
```

**Conclusão A.1**: caminho 100% laterado de `Style` enum até P288.
P288 adiciona uma **2ª fonte de entrada** (`Style::Lang(...)` →
`delta.lang` via `push_styles`) que reúne com a fonte parse-driven
existente. Caminho parse continua intacto.

---

## A.2 — Estrutura do variant `Style::Lang`

### A.2.1 — Análise das opções

| Opção | Veredicto |
|---|---|
| (a) `Lang(Lang)` paralelo | ✅ **Escolhida** — simétrico aos 5 variants existentes; `Lang` é `Copy` (linha 23 `lang.rs`) → `Style` mantém `Copy`/`PartialEq` derives intactos |
| (b) `Lang(Option<Lang>)` | ❌ Vanilla não suporta `lang: none`; A.1 confirma write `Some(...)` apenas |
| (c) `LangCode(EcoString)` | ❌ Replica parsing; quebra `Copy` |

### A.2.2 — Sintaxe final

```rust
pub enum Style {
    Bold(bool),
    Italic(bool),
    Size(Pt),
    Fill(Color),
    HeadingLevel(u8),
    Lang(Lang),  // P288 — paralelo arquitectural aos 5 existentes
}
```

**Style continua `Copy`** porque todos os tipos contidos são `Copy`.

### A.2.3 — Honestidade epistémica

P288 estende um **enum plano de variants atómicos** — paralelo
arquitectural aos 5 existentes. **Não é "variant rico com `body`
+ cosméticos opcionais"** (padrão N=4 cumulativo P156G/H/I+P284 sobre
`Content`). Logo, **N=4 desse padrão inalterado** pelo P288. Mesma
lógica P287 §A.2.2.

---

## A.3 — Integração com `StyleDelta`

### A.3.1 — Decisão

**Decidido**: opção **(α)** — `delta.lang = Some(*l)` em
`push_styles` (paridade absoluta com os 5 arms existentes).

A opção (β) "merge herdado" é o que `chain.lang()` (linha 226) já
faz via walk — separadamente do `apply`. P288 não muda esta dinâmica.

### A.3.2 — Implementação literal

```rust
// Em StyleChain::push_styles (style_chain.rs:137-143):
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),  // P288 — paralelo
}
```

**+1 LOC no match.** Match continua exaustivo (compilador detecta
ausência se Style ganha variant futuro).

### A.3.3 — Convivência com parse-driven path

A 2ª fonte (`Style::Lang` em `Styles` collection) coexiste com a
parse-driven (`eval_set_rule` em `eval/rules.rs:385`). Last-write
wins per LIFO da chain — comportamento determinístico.

---

## A.4 — Impacto em `FrameItem::Text` e emit (**decisivo para N=5**)

### A.4.1 — Análise empírica do gatilho

**Decidido**: opção **(i)** — `FrameItem::Text` já consulta `Lang`
indirectamente via `StyleDelta` herdada do Layouter (paradigma P144).
P288 **apenas estende caminho de entrada** ao `delta.lang`.

| Critério | Evidência empírica |
|---|---|
| `export.rs` consulta `lang` algures? | `grep "lang" 03_infra/src/export.rs` → 0 hits funcionais |
| `FrameItem::Text` precisa novo field `lang`? | **Não** — `TextStyle.lang: Option<Lang>` já existe (P136 Fase A DEBT-52) e é capturado em `eval_markup`/Layouter |
| Consumer P287 SmartQuote já lê `self.style.lang`? | **Sim** — `layout/mod.rs:1992` `match &self.style.lang { Some(l) => localize_quotes(l), None => DEFAULT_QUOTES }` |
| Existe algum reflector que tocaria `export.rs`? | **Não** — A.1.7 confirma zero acesso L3 a lang |

### A.4.2 — Consequência arquitectural: gatilho N=5 dispara

**`export.rs` preservado bit-exact pelo 5º passo consecutivo**:

| Passo | Razão |
|---|---|
| N=1: P282 | Auditoria empírica refutou 6/6 suspeitas |
| N=2: P285 | Alteração em emit propaga simetricamente via helper único |
| N=3: P286 | Alteração em L1 sem tocar L3 (wrap-aware reusa Line) |
| N=4: P287 | Consumer SmartQuote reusa Content::Text → FrameItem::Text |
| **N=5: P288** | `Style::Lang(...)` extende caminho parse SEM tocar emit; lang continua dead-code em L3 (paradigma P144 preserved) |

**Padrão "Win arquitectural single source of truth como invariante
anti-bug" atinge limiar histórico N=5** definido em ADR-0065 (criterio
empírico de formalização ADR meta).

### A.4.3 — Promoção condicional ADR meta

Per spec §3 ponto 7 + §5 não-objectivo "não promover se A.4 →
(ii)/(iii)":

| Condição | Acção |
|---|---|
| A.4 → (i) confirmada empiricamente | ✅ **Promoção dispara** |
| Falha sutil (e.g. reflector trivial em export.rs) | n/a — refutado em A.1.7 |
| 5 citações cumulativas explícitas | ✅ P282/P285/P286/P287/P288 (citantes registados nos relatórios respectivos) |
| Padrão claro com definição operacional | ✅ "Hash `export.rs` preservado em alterações L1 que adicionam features" |

**Decisão**: **promover ADR-0098 "Single source of truth como
invariante anti-bug"** com status `IMPLEMENTADO` + referência aos 5
passos cumulativos.

Per spec §3 ponto 7: a ADR contém:
- Definição operacional do padrão.
- 5 aplicações cumulativas detalhadas (P282/P285/P286/P287/P288).
- Consequências arquiteturais (anti-bug por construção).
- Alternativas consideradas (manter helpers fragmentados; refazer
  emit ad-hoc por passo).
- Status `IMPLEMENTADO` desde P281 (helpers unificados).

---

## §Métricas do impacto

| Métrica | Antes | Pós-P288 |
|---|---:|---:|
| `Style` variants | 5 | **6** (+`Lang(Lang)`) |
| `StyleDelta` fields | 10 | 10 (inalterado) |
| `push_styles` arms | 5 | **6** (+1 LOC) |
| Caminhos de entrada para `delta.lang` | 1 (parse-driven) | **2** (parse + Style::Lang) |
| Caminhos de leitura `chain.lang()` | 4+ consumers | 4+ (idem) |
| Hash L0 `style.md` | actual | **muda** (+1 variant) |
| Hash L0 `content.md` | actual | **preservado** (sem novo Content variant) |
| Hash L0 `stdlib.md` | actual | **preservado** (sem nova função) |
| Hash L0 `export.rs` | `66cb8ac3` (P285) | **preservado bit-exact** (5º passo consecutivo) |
| ADR meta promovida | n/a | **ADR-0098** "Single source of truth" `IMPLEMENTADO` |
| Padrão N "single source of truth" | 4 (P287 limiar) | **5 cumulativo (limiar atingido, formalizado)** |
| Padrão N "variant rico" | 4 cumulativo | 4 (Style::Lang variant atómico, não rico) |
| Testes lang-aware adiados P287 | 3 | **0** (reactivados) |

---

## §Risco residual mitigado

- **Risco principal** (§7 spec — `delta.lang` dead-code?): refutado
  por A.1.6 (5+ consumers activos em layout-time).
- **Risco secundário** (A.4 → ii): refutado empiricamente por A.1.7
  (`export.rs` zero hits funcionais para lang).
- **Risco terciário** (assimetria oculta — mais 4 fields sem variant):
  A.1.2 confirma `weight`/`tracking`/`leading`/`font` também sem
  variants `Style`. **Não escopo de P288** (per §5 não-objectivo).
  Passos próprios candidatos: P288.1-P288.4 ou aglomerado em P288.5
  cluster — registado mas não materializado.
- **Risco quaternário** (promoção ADR meta indevida): **gatilho real
  empiricamente confirmado** por A.4 — não acidental. Promoção
  legítima.

---

## §Fecho da Fase A

Inventário literal (8 sub-secções A.1.1-A.1.8 + diagrama de fluxo)
+ decisão variant (a) paralela + integração cascade (α) +
**confirmação empírica do gatilho N=5 do padrão "single source of
truth"** registadas. **Promoção ADR-0098 dispara legitimamente
neste passo** — registo histórico do limiar atingido.

Procede-se a §3 do passo + materialização da ADR-0098.
