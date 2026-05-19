# ADR-0098 — Single source of truth como invariante anti-bug

**Status**: IMPLEMENTADO (desde P281; formalizado em P288)
**Data**: 2026-05-19
**Passo promotor**: P288 (`P-style-lang-variant`)
**Categoria**: Meta-arquitectural

---

## Contexto

Pós-P281 (unificação β-completa de stream-builders em
`build_page_stream` + `PageContext` + `FontScenario`), o exporter L3
ficou estruturado em torno de **helpers únicos** (`emit_text_pdf`,
`emit_glyph_pdf`, `line_rg_prefix` introduzido em P285) invocados
tanto pelo caminho top-level (`build_page_stream`) como pelo caminho
local em Group (`draw_item_local`).

Esta estrutura foi originalmente motivada por **simplicidade** (evitar
divergência local vs top-level) — mas P282 §1.1 descobriu
empiricamente que ela **automaticamente elimina classes inteiras de
bugs latents** ao garantir que qualquer alteração futura ao emit
afecta os dois caminhos simetricamente.

Passos subsequentes (P285, P286, P287, P288) confirmaram cumulativamente
que o padrão **estende-se naturalmente** para alterações em L1 que
adicionam features: o hash L0 `export.rs` é preservado bit-exact
sempre que a feature pode ser implementada **reutilizando o emit
existente** em vez de criar emit ad-hoc.

Atingido **limiar histórico N=5 aplicações cumulativas** (critério
empírico ADR-0065 para formalização de padrão meta).

---

## Decisão

Formaliza-se o padrão **"Single source of truth como invariante
anti-bug"** como meta-arquitectural perene em cristalino:

> **Regra**: qualquer feature nova que possa ser implementada
> reutilizando primitivas L1/L3 pré-existentes (`FrameItem` variants,
> emit helpers) **deve fazê-lo** em vez de criar emit ad-hoc.
> Quando o emit é estruturalmente igual ao pré-existente, o hash L0
> `export.rs` permanece **preservado bit-exact** por construção.

**Consequência operacional**: a Fase A de cada passo deve **explicitamente
inspeccionar** se a feature pode reutilizar primitivas existentes
**antes** de propor emit novo. Diagnóstico empírico antes de
arquitectura nova.

**Consequência testável**: hash L0 `export.rs` é métrica de aderência
ao padrão. Mudanças autorizadas são apenas quando a feature
**genuinamente** requer novo emit (e.g. operadores PDF não cobertos
pelos helpers existentes).

---

## 5 aplicações cumulativas (justificação do limiar N=5)

### N=1: P282 §1.1 — Auditoria empírica refutou 6/6 suspeitas

P282 fase A1 audita paridade emit local vs top-level pós-P281.
Inventaria 6 suspeitas de divergência (faux-bold local simplificado,
tracking local em falta, /F2/F3 hardcoded, Glyph multifont local
divergente, Line local sem `RG`, multifont Glyph dispatch).
**6/6 refutadas** — paridade total bit-exact confirmada por construção.

Win arquitectural P281 documentado §C.1 do diagnóstico P282.

### N=2: P285 §8.2 — Alteração simétrica via helper único

P285 adiciona `color: Option<Color>` a `FrameItem::Line` + emit
condicional `RG`. **Helper privado `line_rg_prefix(color) -> String`**
chamado em ambos `build_page_stream` e `draw_item_local`:
- `None` → `""` (bit-exact pré-P285).
- `Some(c)` → `r g b RG ` (paint stroke).

Divergência local vs top-level **estructuralmente impossível** —
ambos chamam mesma assinatura. Hash `export.rs` muda intencionalmente
(`bc7b8b95 → 66cb8ac3`) mas **simétricamente**.

### N=3: P286 §5.2 — Alteração em L1 sem tocar L3

P286 estende consumer Layouter `Content::Underline/Strike/Overline`
para wrap-aware (N `FrameItem::Line` em vez de 1). **Reutiliza
`FrameItem::Line` pré-existente sem modificação** — hash `export.rs`
preservado bit-exact pelo **1º passo consecutivo** pós-P285.

### N=4: P287 §5.1 — Consumer reusa `Content::Text` → `FrameItem::Text`

P287 materializa `Content::SmartQuote { double }` + função stdlib
`#smartquote(...)`. Consumer Layouter resolve glyph lang-aware via
`localize_quotes` + **recurse `Content::Text`** — emit PDF reusa
caminho `FrameItem::Text` existente. Hash `export.rs` preservado pelo
**2º passo consecutivo**.

### N=5: P288 §A.4 — `Style::Lang(Lang)` extende caminho parse sem tocar emit

P288 adiciona `Style::Lang(Lang)` variant em `entities/style.rs` +
arm em `StyleChain::push_styles`. **2ª fonte de entrada** para
`StyleDelta.lang` (paralela à parse-driven em
`eval/rules.rs:385`). `lang` continua a ser consultado em runtime
por consumers L1 (smartquote P287, hyphenation P144, supplement
P158B) sem persistir em emit — paridade absoluta com P144 paradigm.

Hash `export.rs` preservado pelo **3º passo consecutivo** — cumula
**N=5 ao longo dos 5 passos P282-P285-P286-P287-P288**.

---

## Alternativas consideradas

### (a) Manter helpers fragmentados (status quo pré-P281)

Cada passo introduz emit ad-hoc duplicado entre top-level e local.
**Rejeitada empiricamente por P281** — divergência local vs top-level
foi a causa-raiz de classes de bugs P273.10/P279/P280.

### (b) Não formalizar (continuar implícito)

Padrão emerge naturalmente em cada passo mas sem ADR. Risco: passo
futuro pode quebrar a invariante por desconhecimento (e.g. propor
helper novo quando existente cobriria). Limiar N=5 atingido —
formalização legítima per ADR-0065.

### (c) Formalizar como ADR mas sem nome (sucinta)

Rejeitada — nome "Single source of truth como invariante anti-bug"
captura ambos os ângulos (princípio + consequência). ADRs vivem
com nomes para serem indexáveis.

---

## Consequências

### Imediatas (já materializadas)

1. **Hash `export.rs` é métrica de aderência** ao padrão. Mudanças
   estritas no hash devem ter justificação empírica explícita no
   diagnóstico Fase A do passo.
2. **Reuso de primitivas L1/L3 é preferência** sobre criar primitivas
   novas. Variants novos em `FrameItem` requerem fase A
   explícita demonstrando que reuso não é viável.
3. **Helpers únicos** (`emit_text_pdf`, `emit_glyph_pdf`,
   `line_rg_prefix`) são pontos de extensão privilegiados — alterações
   ao emit passam por aqui.

### Futuras (operacionais)

1. **Fase A de passos futuros deve incluir secção "potencial de
   reuso"** — inspecção literal de `FrameItem` variants + emit
   helpers antes de propor estrutura nova.
2. **Métrica de saúde do padrão**: contar passos consecutivos com
   hash `export.rs` preservado. Quebra deve ser investigada — pode
   indicar feature genuinamente nova ou regressão arquitectural.
3. **Critério de promoção** em diagnósticos: "este passo cita o
   padrão N=X cumulativo" registado explicitamente.

### Não-objectivos

1. **Não** proibir alterações em `export.rs`. Features genuinamente
   novas (e.g. operadores PDF não cobertos) podem e devem motivar
   mudanças no hash. A invariante é **bit-exactness quando
   estructuralmente possível**, não **imutabilidade absoluta**.
2. **Não** impor reuso a custos arquiteturais altos. Reuso forçado
   que distorce semântica viola spirit da ADR — bem como ADR-0054
   (graded scope).

---

## Status

`IMPLEMENTADO` desde **P281** (helpers unificados); **formalizado
em P288** (limiar histórico N=5 atingido empiricamente via passos
P282/P285/P286/P287/P288).

## Cross-references

- **ADR-0065** — inventariar-primeiro (precedente metodológico
  + criterio empírico N≥5 para promoção meta-ADR).
- **P281** — Unificação β-completa stream-builders.
- **P282 §1.1** — Auditoria empírica (N=1 cumulativo).
- **P285 §8.2** — Alteração simétrica via helper único (N=2).
- **P286 §5.2** — Reuso `FrameItem::Line` sem modificação (N=3).
- **P287 §5.1** — Consumer reusa `Content::Text` (N=4).
- **P288 §A.4** — `Style::Lang(...)` extende parse sem tocar emit (N=5).
- **ADR-0093** — Meta-metodologia evolução ADRs (justifica
  formalização incremental).
- **ADR-0094** — Meta-operacional specs (justifica diagnósticos
  imutáveis pré-materialização).
- **P273.17 §0** — Anti-padrão over-formalização (justifica que
  promoção é **condicional** ao gatilho disparar genuinamente; P288
  §A.4 confirmou empiricamente).

---

*ADR-0098 formaliza o padrão "Single source of truth como invariante
anti-bug" como meta-arquitectural perene em cristalino, com base em
5 aplicações cumulativas empiricamente confirmadas (P282-P285-P286-
P287-P288). Hash L0 `export.rs` torna-se métrica testável de aderência
ao padrão. Promoção legítima per ADR-0065 critério N≥5 + ADR-0093
política de formalização incremental.*
