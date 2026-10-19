# Diagnóstico figure supplement por lang — Passo P158B

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
segundo sub-passo Model figure-kinds. **Décima sétima
aplicação consecutiva** do padrão diagnóstico-primeiro.

Refino comportamental análogo a P158A (auto-detect kind), agora
sobre prefix de label localizado por lang. **Reuso do padrão
consolidado `localize_quotes(lang)` em `rules/lang/quotes.rs`**
(P155) — primeiro reuso explícito cross-feature do pattern P155.

---

## 1. Assinatura vanilla `FigureElem.supplement`

Per relatório P158/P158A diagnóstico — vanilla `FigureElem`
campo `supplement: Smart<Option<Supplement>>` com default
`Smart::Auto` resolve via:
1. `kind` resolvido (P158A já implementado em cristalino).
2. `kind.local_name(lang, region)` produz string localizada
   ("Figure"/"Figura"/"Abbildung"/etc.).
3. Fallback se nenhum nome disponível: erro "please specify
   the figure's supplement".

Cristalino actual NÃO suporta `supplement` field em variant
ou stdlib — refino vai aplicar-se em `introspect.rs` no
momento de gerar label.

**Subset minimal P158B**: hard-code 6 langs × 3 kinds (18
entradas) com fallback. Sem `supplement` field user-facing
(refino M futuro NÃO reservado).

---

## 2. Comportamento observável

**Vanilla**:
- `figure(image(...), caption: [...])` em lang=en produz
  label "Figure 1".
- Idem em lang=pt produz "Figura 1"; lang=de "Abbildung 1";
  lang=fr "Figure 1"; etc.
- Numbering format integra supplement com colon ou period
  per CSL/style.

**Cristalino actual** (`introspect.rs:334`):
```rust
Some(format!("Figura {}", n))
```
Hardcoded **"Figura"** (PT), sem distinção por lang ou kind.

**Cristalino P158B** (proposto):
```rust
let supplement = figure_supplement_for_lang(kind, lang);
Some(format!("{} {}", supplement, n))
```
Lookup por `(kind, lang)` com fallback.

**Decisão arquitetural-chave §1 deste diagnóstico — fallback**:
- **Fallback PT (não EN)** para preservar backwards compat com
  tests existentes que esperam "Figura".
- Cristalino é projecto português; PT como default é razoável.
- Refino futuro pode refactorar fallback se prioritário (NÃO
  reservado per política P158).

---

## 3. ADR-0064 caso aplicável

**NÃO directamente aplicável** em P158B. `kind` continua
String directo; `lang` é parâmetro contextual; `supplement`
não é variant field neste passo.

Aplicação futura potencial em refactor `supplement: Option<Content>`
field (Caso A) — NÃO reservado.

---

## 4. Variants Content existentes a estender

**Nenhuma**. Refino comportamental apenas. `Content::Figure`
permanece inalterado.

---

## 5. Helpers stdlib reusáveis

### 5.1 Padrão `localize_quotes` (P155) reusado

`rules/lang/quotes.rs:41`:
```rust
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str)
```

Estrutura:
- Tabela estática `LANG_QUOTES: &[(&str, (&str, &str))]`.
- Lookup linear por exact match no `Lang::as_str()`.
- Fallback constante `DEFAULT_QUOTES`.

**Replicar para `figure_supplement_for_lang(kind, lang)`**:
- Tabela estática `LANG_SUPPLEMENTS: &[((&str, &str), &str)]`
  ou similar (par chave (kind, lang) → supplement).
- Lookup linear.
- Fallback per (kind, lang): primeiro tenta (kind, "pt");
  se kind desconhecido, devolve "Figura" capitalizado.

### 5.2 Helper novo `figure_supplement_for_lang`

```rust
pub fn figure_supplement_for_lang(kind: &str, lang: Option<&Lang>) -> &'static str
```

Localização: `rules/lang/figure_supplement.rs` ficheiro novo
per padrão `quotes.rs`.

**N=1** sem candidato a reuso ainda; promoção diferida per
política consistente N=3-4.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P158B | Refino futuro |
|---------|--------------|---------------|
| 6 langs × 3 kinds = 18 entradas | ✓ implementado | langs/kinds adicionais NÃO reservados |
| Fallback PT (paridade backwards compat) | ✓ implementado | refactor para EN ou outro NÃO reservado |
| Numbering format integrado simples (`"{} {}"`) | ✓ implementado | CSL-aware format diferido (depende hayagriva) |
| `supplement: Option<Content>` field user-facing | ✗ scope-out | refino M futuro per ADR-0064 Caso A NÃO reservado |
| `lang` resolution via Style chain | ✗ scope-out | refino M futuro; P158B usa state.lang directo |
| Region-specific supplements (pt-BR vs pt-PT) | ✗ scope-out | Lang cristalino é 2-3 letras sem region (ADR-0052) |
| Helper público promoção | ✗ scope-out | política N=3-4 |

---

## 7. Tests planeados

### 7.1 Helper tests (`figure_supplement.rs`, ~5)

1. Lookup `image/pt` → "Figura".
2. Lookup `table/de` → "Tabelle".
3. Lookup `raw/it` → "Listato".
4. Fallback `image/zh` → "Figura" (PT fallback per decisão §2).
5. Fallback `custom-kind/en` → "Figure" (kind desconhecido +
   lang válido — devolve kind capitalizado em lang).

### 7.2 Integração tests (`stdlib/mod.rs` ou `introspect_tests`, ~7)

- `figure_label_pt_image` — figure(image(...)) lang=pt label
  "Figura 1".
- `figure_label_en_table` — figure(table(...)) lang=en label
  "Table 1".
- `figure_label_de_raw` — figure(raw(...)) lang=de label
  "Listing 1".
- `figure_label_fallback_lang_unknown` — lang=zh produz label
  PT.
- `figure_label_default_no_lang_set` — lang None produz PT
  (backwards compat).
- `figure_multiple_figures_numbering_independente` — counter
  por kind continua a funcionar (regression P157A/P158A).
- `figure_label_explicit_kind_override` — kind="custom" + lang=en
  → "Custom 1" (kind capitalizado).

**Δ esperado**: +12-13 tests.

---

## 8. Lang resolution — decisão arquitetural

### 8.1 Estado actual

`introspect()` signature: `pub fn introspect(content: &Content) -> CounterState`.
NÃO recebe lang.

`walk()` (interno): `fn walk(content: &Content, state: &mut CounterState)`.
NÃO acede lang.

`CounterState`: NÃO tem field `lang`.

### 8.2 Decisão adoptada

**Adicionar field `pub lang: Option<Lang>` a `CounterState`**.
- Default `None` → walk usa fallback PT em
  `figure_supplement_for_lang(kind, None)`.
- Caller pode setar `state.lang = Some(lang)` antes de passar
  a `layout()` se quiser comportamento lang-aware.
- **Backwards compat preservada**: tests existentes que não
  setam lang continuam a receber "Figura" (PT default).

### 8.3 Alternativas rejeitadas

- **Modificar signature `introspect()` para receber lang**:
  quebra todos os call sites (10+ tests).
- **Walk acompanhar Styled e extrair lang activo**: complexidade
  arquitectural maior; refactor não-trivial.
- **Lang resolution em layout em vez de walk**: muda momento de
  resolução de label — divergência semântica vs vanilla.

---

## 9. Padrão P155 reusado cross-feature

Primeiro reuso explícito do pattern `localize_quotes` em
diferente feature (quotes → figure supplement).

**Subpadrão emergente N=1** "padrão P155 i18n reusado
cross-feature" — candidato a registo se outro feature reusar
(e.g. table caption supplement futuro; bibliography lang per
P159B §4).

---

## Resumo executivo

P158B materializa **supplement automático por lang em figure**:
- Helper novo `figure_supplement_for_lang(kind, lang) ->
  &'static str` em `rules/lang/figure_supplement.rs` com
  18 entradas (3 kinds × 6 langs) + fallback PT.
- Field novo `lang: Option<Lang>` em `CounterState`.
- Modificação trivial em `introspect.rs` linha 334 (label
  format).
- Sem alteração a variant `Content::Figure` ou stdlib.

**Decisões arquitecturais P158B**:
- **Fallback PT** (não EN) para backwards compat com tests
  pré-existentes.
- **Lang resolution via `state.lang`** (novo field opcional).
- **Reuso pattern P155** `localize_quotes` em estrutura paralela.

**Decisões diferidas (NÃO reservadas)**:
- Mais langs além das 6 minimais.
- `supplement` field user-facing.
- Numbering format CSL-aware (depende hayagriva).
- Region-specific supplements (Lang cristalino sem region).

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (PT fallback).
- ADR-0054: graded scope-out de langs/supplement field/CSL.
- ADR-0060: refino qualitativo de feature implementada.
- ADR-0064 NÃO directamente aplicável.
- ADR-0065 critério #1 (naming `figure_supplement_for_lang`)
  + critério #5 (scope) implícitos.

**Tests planeados**: Δ +12-13.

**Risco**: muito baixo. Refino comportamental aditivo; sem
alteração de variant; reuso pattern consolidado P155.
