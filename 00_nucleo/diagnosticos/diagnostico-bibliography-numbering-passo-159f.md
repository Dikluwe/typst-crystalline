# Diagnóstico Bibliography numbering — Passo P159F

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
quarto sub-passo Bibliography + Cite (Bloco A do diagnóstico
P159B §3.5). **Vigésima primeira aplicação consecutiva** do
padrão diagnóstico-primeiro.

Materializa **counter local de bib entries + render numerado
em Cite Normal/None**. Reusa infraestrutura state lookup
P158B/C (subpadrão #15 cresce N=2 → 3). **Último candidato do
Bloco A** após P158B/C/P159C/D — pós-P159F, Bloco A esgotado.

---

## 1. Assinatura vanilla `BibliographyElem.style`

Per `lab/typst-original/crates/typst-library/src/model/bibliography.rs`,
vanilla `BibliographyElem.style: Smart<BibliographyStyle>` com
default `Smart::Auto` resolvido para style configurada via
`#set bibliography(style: "ieee")` ou similar.

Styles vanilla:
- `numeric` (default) — `[1]`, `[2]`, `[3]`.
- `alphanumeric` — `[Smi24]`.
- `author-date` — `(Smith, 2024)`.
- `chicago-author-date`, `mla`, `apa`, etc. — CSL styles
  (depende hayagriva).

**Subset minimal P159F**: apenas `numeric` (default vanilla)
per ADR-0054 graded. Outros styles dependem hayagriva (Bloco B).

---

## 2. Comportamento observable

**Vanilla** (style numeric):
- `cite("smith")` (Normal/None) → `[1]` (índice da entry).
- `cite("smith", form: "prose")` → `Smith (2024)` (P159C
  comportamento — style numeric não afecta forms diferenciadas).

**Cristalino actual** (P159C):
- `cite("smith")` (Normal/None) → `[smith]` (placeholder
  literal).
- `cite("smith", form: "prose")` → `Smith, J. (2024)` (P159C).

**Cristalino P159F** (proposto, Opção C decidida):
- `cite("smith")` (Normal/None) com Bibliography contendo
  `smith` → `[1]` (numerado via lookup).
- `cite("smith")` (Normal/None) sem Bibliography ou key não
  encontrada → `[smith]` (fallback P159A backwards compat).
- `cite("smith", form: "prose")` → `Smith, J. (2024)` (P159C
  inalterado — forms diferenciadas não ganham numeração).

**Decisão arquitectural-chave §1**: render é placeholder
melhorado (estilo numeric simplificado), não CSL real. CSL
depende hayagriva (Bloco B).

---

## 3. ADR-0064 caso aplicável

**NÃO directamente aplicável** se Opção C (decidida abaixo).
Sem field novo `Bibliography.style`; comportamento implícito
via `state.bib_numbers` populado.

Aplicável se Opção B fosse escolhida: `Bibliography.style:
Smart<BibliographyStyle>` → `Option<BibliographyStyle>`
(patamar Caso A N=7 → 8). **Rejeitada** per §8 abaixo.

---

## 4. Variants Content existentes a estender

**Nenhum**. `Content::Bibliography` e `Content::Cite` ambos
inalterados estruturalmente. P159F toca apenas
`CounterState` (field aditivo `bib_numbers`) + walk em
introspect + render em layout.

**Hash content.rs preservado esperado** — 15º passo consecutivo
via L0-baseline interpretation. Sem alteração ao variant.

---

## 5. Helpers stdlib reusáveis

- `extract_bib_entries` (P159A, P159D extendido) — inalterado.
- `extract_citation_form` (P159C) — inalterado.

**Sem helper novo significativo** — numbering é trivial inline:
- Walk: `state.bib_numbers.insert(key.clone(), len + 1)`.
- Layout: `state.bib_numbers.get(key).map(|n| format!("[{}]", n))`.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159F | Refino futuro |
|---------|--------------|---------------|
| Style numeric apenas | ✓ implementado | alphanumeric/author-date/CSL diferidos (Bloco B) |
| Counter single-pass | ✓ implementado | cross-document refs bloqueado ADR-0017 |
| Multi-Bibliography contínua | ✓ implementado | independente NÃO reservado |
| `Bibliography.style` field user-facing | ✗ scope-out | refino futuro com Opção B (NÃO reservado) |
| Helper público `numbering_for_key` | ✗ scope-out | promoção diferida N=3-4 |

---

## 7. Tests planeados

### 7.1 Unit tests `CounterState` (`entities/counter_state.rs`, ~2)

1. Default `bib_numbers` empty.
2. Insertion preserva ordem (HashMap não garante ordem; teste
   verifica presença + valor — não ordem).

### 7.2 Stdlib tests (`stdlib/mod.rs`, ~0-2)

Stdlib não muda assinatura — testes existentes (P159A/D)
preservam-se. **Possível teste opcional**: introspect com
Bibliography popula `bib_numbers` correctamente.

### 7.3 Layout E2E tests (`layout/tests.rs`, ~5-7)

1. `cite_normal_renderiza_numero_quando_bib_populada` —
   `bibliography(...)` + `cite("smith")` → `[1]`.
2. `cite_normal_fallback_placeholder_quando_bib_vazia` —
   sem Bibliography → `[smith]` (regression P159A).
3. `cite_normal_multiple_entries_numeradas_em_ordem` —
   3 entries; cite primeira → `[1]`, cite segunda → `[2]`,
   cite terceira → `[3]`.
4. `cite_form_prose_inalterada_com_bib_numerada` —
   `cite("smith", form: "prose")` continua a renderizar
   "Author (Year)" mesmo com Bibliography numerada (regression
   P159C; comportamento P159F restrito a Normal/None).
5. `cite_unknown_key_fallback_placeholder` — Cite com key
   não em Bibliography → `[unknown_key]` (regression P159A).
6. `cite_normal_multi_bibliography_continua` —
   2 Bibliographies; numeração contínua (Bib1: [1, 2];
   Bib2: [3]; Cite "third" → `[3]`).

**Δ esperado**: +7-10 tests novos + 0 adaptados (alinhado com
esboço P159B §3.5 range 10-15; ligeiramente abaixo por
helper inline trivial).

---

## 8. Decisão arquitectural-chave: Opção A/B/C (§8)

### 8.1 Avaliação multi-critério

| Critério | Opção A (substituir sempre) | Opção B (style field novo) | Opção C (Cite.form interaction) |
|----------|----------------------------|---------------------------|--------------------------------|
| Backwards compat tests P159A/C | ✗ quebra | ✓ preserva | ✓ preserva |
| Alteração estrutural Bibliography | ✗ não | ✓ field novo | ✗ não |
| Hash content.rs L0-baseline | preservado | preservado se prompt não alterado | preservado |
| Configurabilidade user | ✗ não | ✓ via `style:` arg | ✗ implícito |
| ADR-0064 patamar | inalterado | N=7→8 (Caso A) | inalterado |
| Complexidade implementação | baixa | média | baixa |
| Comportamento implícito | sim | não | sim (depende ordem walk) |

### 8.2 Opção C adoptada

**Justificação**:
- Backwards compat trivial — tests pré-existentes (P159A/C)
  passam inalterados quando Bibliography vazia ou Cite key
  não encontrada.
- Sem alteração estrutural — `Bibliography` e `Cite` variants
  preservados (paridade política P158C).
- Reusa pattern P159C (`state.bib_entries` populado em walk);
  subpadrão #15 cresce N=2 → 3 — patamar moderado consistente.
- Comportamento intuitivo "Bibliography popula numeração;
  Cite Normal/None usa-a se possível".
- Sem necessidade de field user-facing — `Bibliography.style`
  pode ser adicionado em refino futuro (Opção B) se prioritário
  (NÃO reservado).

**Trade-off aceite**: comportamento implícito (depende ordem
walk; Bibliography deve aparecer antes ou depois de Cite no
documento — walk single-pass garante state populado antes do
layout).

### 8.3 Alternativas rejeitadas

- **Opção A**: quebra tests P159A/C; força adaptação cascading
  desnecessária.
- **Opção B**: alteração estrutural sem ganho proporcional;
  `Bibliography.style` field user-facing deve esperar até
  styles adicionais (Bloco B) serem suportadas para justificar
  o field.

---

## 9. Multi-Bibliography numbering: contínua vs independente (§9)

### 9.1 Decisão: **contínua** (paridade vanilla)

**Justificação**:
- Paridade vanilla numeric style (numeração global ao
  documento).
- Mais simples — `state.bib_numbers.insert(key, len + 1)`
  funciona para multi-Bibliography sem lógica especial.
- Multi-Bibliography é caso edge — não vale complexidade
  adicional para independent numbering.

### 9.2 Algoritmo

```rust
Content::Bibliography { entries, ... } => {
    state.bib_entries.extend(entries.iter().cloned()); // P159C
    // P159F: numbering contínua
    for entry in entries {
        let next_num = state.bib_numbers.len() as u32 + 1;
        state.bib_numbers.entry(entry.key.clone()).or_insert(next_num);
    }
    if let Some(t) = title { walk(t, state); }
}
```

**`or_insert`**: duplicate keys (mesmo key em múltiplas
Bibliographies) preservam o primeiro número (paridade
HashMap; documentar como comportamento determinístico).

---

## 10. Interação Cite.form (§10)

### 10.1 Decisão: numeração só em Normal/None

**Justificação**:
- Forms diferenciadas (Prose/Author/Year) têm semântica
  específica — numeração não substitui semântica.
- P159C estabeleceu padrão "form determina render";
  numbering é refino do `Normal` apenas.
- Paridade vanilla — style numeric afecta apenas form Normal
  (forms diferenciadas têm próprio render).

### 10.2 Algoritmo

```rust
Content::Cite { key, supplement, form } => {
    let resolved_form = form.unwrap_or_default();
    let entry = self.counter.bib_entries.iter().find(|e| e.key == *key);
    use crate::entities::citation_form::CitationForm;
    let text = match (resolved_form, entry) {
        (CitationForm::Normal, _) => {
            // P159F: lookup numbering; fallback `[key]`.
            self.counter.bib_numbers.get(key)
                .map(|n| format!("[{}]", n))
                .unwrap_or_else(|| format!("[{}]", key))
        }
        (CitationForm::Prose,  Some(e)) => format!("{} ({})", e.author, e.year),
        (CitationForm::Author, Some(e)) => e.author.clone(),
        (CitationForm::Year,   Some(e)) => e.year.to_string(),
        (_, None)                       => format!("[{}]", key),
    };
    self.layout_content(&Content::text(text));
    if let Some(s) = supplement { self.layout_content(s); }
}
```

---

## Resumo executivo

P159F materializa **counter local de bib entries +
render numerado em Cite Normal/None** per ADR-0033 + ADR-0054
graded:
- Field `pub bib_numbers: HashMap<String, u32>` em `CounterState`
  (paridade aditiva P158B `state.lang` + P159C `state.bib_entries`).
- Walk arm `Content::Bibliography` popula `bib_numbers`
  contínuamente.
- Layout arm `Content::Cite` Normal/None faz lookup
  `state.bib_numbers.get(key)` → `[N]` ou fallback `[key]`.
- Forms diferenciadas (Prose/Author/Year) inalteradas (P159C).

**Decisões arquitecturais P159F**:
- **Opção C** adoptada — Cite.form interaction (sem field
  user-facing; reusa subpadrão #15).
- **Multi-Bibliography contínua** (paridade vanilla numeric).
- **Numeração só em Normal/None** (preserva forms diferenciadas).
- **Subpadrão #15 N=2 → 3** (state.lang + state.bib_entries +
  **state.bib_numbers**).

**Decisões diferidas (NÃO reservadas)**:
- Outras styles (alphanumeric, author-date, CSL) — Bloco B.
- `Bibliography.style` field user-facing — refino futuro.
- Numeração independente multi-Bibliography.
- Helper público `numbering_for_key`.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (numeric simplified).
- ADR-0054: graded scope-out de outras styles.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: NÃO directamente aplicável (Opção C; sem field novo).
- ADR-0065 critério #5: scope determinado por inventário.

**Tests planeados**: Δ +7-10 (range spec 10-15; ligeiramente
abaixo por helper inline trivial).

**Risco**: baixo-médio. Refino comportamental + extensão
infraestrutura state lookup + decisão arquitectural-chave Opção
A/B/C resolvida com pré-recomendação Opção C confirmada.

**Marca conceptual**: P159F **esgota Bloco A** do diagnóstico
P159B. Pós-P159F, Model puro está saturado per recomendação
do diagnóstico (~55-60% estimado com 24 entradas parciais).
