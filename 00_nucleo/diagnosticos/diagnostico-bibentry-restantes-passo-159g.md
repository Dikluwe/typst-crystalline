# Diagnóstico BibEntry 6 fields restantes — Passo P159G

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
**segundo sub-passo família 159 fora do Bloco A** do diagnóstico
P159B (Bloco A esgotado pós-P159F). **Vigésima terceira aplicação
consecutiva** do padrão diagnóstico-primeiro.

Refino estrutural de tipo entity `BibEntry` adicionando os 6
fields restantes mais comuns de `hayagriva::Entry` (`editor`,
`series`, `note`, `isbn`, `location`, `organization`) — listados
em P159D §9.3 como diferidos por menor universalidade. **Pattern
P159D replicado pela terceira vez** — subpadrão #16 cresce N=2
→ 3 (atinge limiar formalização N=3-4).

---

## 1. Assinatura vanilla `hayagriva::Entry` para os 6 fields

Per `hayagriva::Entry` (vanilla), os 6 fields têm tipos diversos:
- `editor: Vec<Person>` — lista de pessoas estruturadas.
- `series: Option<String>` — string directa.
- `note: Option<String>` — string directa.
- `isbn: Option<String>` — string com checksum (sem validation
  default em hayagriva).
- `location: Option<String>` — string directa (city/country).
- `organization: Option<String>` — string directa (institution
  publishing).

**Subset minimal P159G**: `Option<String>` para todos per
ADR-0054 graded. `editor` simplificado para `Option<String>`
(lista renderizada como string concatenada) — refino futuro
para `Vec<String>` ou `Vec<Person>` NÃO reservado.

---

## 2. Comportamento observable

**Vanilla** (style APA com hayagriva):
- editor: `Smith, J. (Ed.)` ou `(Ed. by Smith)`.
- series: `... (Crystal Studies, vol. 12)`.
- note: `[See also Smith 2023]` em algumas styles.
- isbn: `ISBN 978-...` ao final.
- location: `New York: Springer` (location:publisher).
- organization: substitutivo a publisher em tech reports.

**Cristalino actual** (P159E — sem estes 6 fields): output
APA-like com 10 fields apenas.

**Cristalino P159G** (proposto): output APA-like com 16 fields
appendados condicionalmente per ordem decidida em §8.

**Plaintext simples** — sem hyperlinks (paridade P159E).

---

## 3. ADR-0064 caso aplicável

**NÃO directamente aplicável** — fields são `Option<String>`
trivial sem mapping `Smart<T>`. Pattern Optional simples paralelo
a P159D/E.

---

## 4. Variants Content existentes a estender

**Nenhum**. `Content::Bibliography` e `Content::Cite` ambos
inalterados. P159G toca apenas `BibEntry` struct em
`entities/bib_entry.rs`.

**Hash content.rs preservado esperado** — 17º passo consecutivo
via L0-baseline interpretation. **Hash bib_entry.rs também
preservado esperado** (paridade P159D+P159E resultado).

**Subpadrão #16 N=2 → 3**: "refino tipo entity sem alteração
Content" — P159D BibEntry 4 fields + P159E BibEntry 2 fields +
**P159G BibEntry 6 fields**. Atinge limiar formalização N=3-4.

---

## 5. Helpers stdlib reusáveis

### 5.1 Helper inline `optional_str` (P159D/E) reusado

Localização: `rules/stdlib/structural.rs` (inline em
`extract_bib_entries`). Reuso directo:
- P159D N=4 usos cumulativos (volume/pages/journal/publisher).
- P159E N=2 usos cumulativos (url/doi).
- **P159G N=6 usos no mesmo passo** (editor/series/note/isbn/
  location/organization).
- **N=12 usos cumulativos** — largamente acima do limiar
  promoção N=3-4.

**Promoção a `pub(super)` ou helper público diferida** em passo
administrativo XS futuro NÃO reservado. P159G atinge "largamente
promovível" empiricamente.

### 5.2 Builder pattern fluente

6 métodos novos `with_*` paridade P159D/E. Pattern consistente
N=12 cumulativo (4 P159D + 2 P159E + 6 P159G).

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159G | Refino futuro |
|---------|--------------|---------------|
| 6 fields adicionais (editor/series/note/isbn/location/organization) | ✓ implementado | restantes vanilla (booktitle/address/chapter/type/institution/etc.) NÃO reservados |
| Fields como `Option<String>` directo | ✓ implementado | tipos estruturados (`Vec<Person>` editor, location codes) NÃO reservados |
| ISBN sem validation checksum | ✓ implementado | validation diferida |
| Plaintext simples | ✓ implementado | hyperlinks (Bloco C) |
| Editor como string concatenada | ✓ implementado | `Vec<String>` ou `Vec<Person>` NÃO reservado |
| Layout formato APA-like | ✓ implementado | CSL real depende hayagriva (ADR-0062) |

---

## 7. Tests planeados

### 7.1 Unit tests `BibEntry` (`entities/bib_entry.rs`, ~3-4)

1. Constructor com os 6 fields novos via builder.
2. PartialEq cobre 16 fields agora (regressão de cada divergente).
3. Backwards compat: `new(4 args)` continua a produzir entry
   com fields novos `None` (regression P159A+P159D+P159E).
4. Builder pattern combinando subset (e.g. só editor + isbn)
   funciona correctamente.

### 7.2 Stdlib tests (`stdlib/mod.rs`, ~3-4)

1. Parse com todos os 6 fields presentes.
2. Parse com subset (3 fields apenas).
3. Parse sem fields novos (regression P159E — 4 obrigatórios +
   4 P159D + 2 P159E apenas).
4. Tipo errado em isbn (Int em vez de Str) rejeitado com
   mensagem clara.

### 7.3 Layout E2E tests (`layout/tests.rs`, ~3-4)

1. Bibliography com entry completa (16 fields) renderiza
   formato extendido (verifica presença dos 6 prefixos: `Ed.`,
   `(`series`)`, `[`note`]`, `isbn:`, `location:`, organization
   substitutivo).
2. Bibliography com entry intermédia (P159E + editor apenas)
   renderiza formato parcialmente extendido.
3. Bibliography com entry mínima (sem fields novos) renderiza
   formato P159E original (regression).
4. Bibliography com organization sem publisher renderiza
   organization no slot publisher.

**Δ esperado**: +10 tests (range spec 8-12).

---

## 8. Ordem layout dos 6 fields (§8)

### 8.1 Avaliação multi-critério

Per paridade APA + manter formato existente P159E:

| Field | Posição | Justificação |
|-------|---------|--------------|
| editor | logo após author (`author. (Ed. editor)`) ou substitutivo author | APA: editor toma posição autor se autor ausente; cristalino simplifica para "depois de author/title" — usa formato `(Ed. {editor})` após title. |
| series | depois de title | APA: parêntese após title. |
| location | antes de publisher | APA: `{location}: {publisher}`. |
| organization | substitutivo a publisher | APA tech reports: `{organization}` no slot publisher se publisher ausente. |
| isbn | ao final, antes de url/doi | APA: `isbn:{isbn}` antes URL/DOI. |
| note | ao final, depois de url/doi | APA: `[{note}]` ao final. |

### 8.2 Ordem decidida

```
[key] author. title (Ed. editor) (series) journal vol. volume,
      pp. pages. location: publisher (year). isbn:XXX url, doi:YYY [note].
```

Quando `organization` presente e `publisher` ausente:
```
[key] author. title (Ed. editor) (series) journal vol. volume,
      pp. pages. location: organization (year). isbn:XXX url, doi:YYY [note].
```

**Backwards compat**: quando todos os 6 fields novos `None`,
output P159E preservado exactamente.

---

## 9. Formatos individuais (§9)

### 9.1 editor

**Decisão**: `(Ed. {editor})` após title.

**Justificação**: APA editor convention (`Ed.` abbreviation +
parentheses). Compactness.

**Alternativas rejeitadas**:
- `Edited by {editor}` — verboso.
- `{editor} (Ed.)` — coloca editor antes; menos comum APA.

### 9.2 series

**Decisão**: `({series})` após title.

**Justificação**: APA series convention (parentheses).
Simplicidade.

### 9.3 location

**Decisão**: `{location}:` antes de publisher.

**Justificação**: APA `City: Publisher` convention.

### 9.4 organization

**Decisão**: substitutivo a publisher quando publisher ausente.
Quando ambos presentes, usa publisher e organization
ignorada (decisão arbitrária per ADR-0054 graded; refino futuro
pode renderizar ambos).

**Justificação**: APA tech reports usam organization como
publisher equivalent.

### 9.5 isbn

**Decisão**: `isbn:{isbn}` em lowercase prefix.

**Justificação**: paridade P159E doi prefix lowercase.
Consistência cristalino.

**Alternativa rejeitada**: `ISBN {isbn}` uppercase — APA
convention mas inconsistente com doi/url.

### 9.6 note

**Decisão**: `[{note}]` brackets ao final.

**Justificação**: distinção visual; brackets sinalizam metadata
auxiliary.

---

## Resumo executivo

P159G materializa **expansão de struct entity `BibEntry`** com
os 6 fields restantes mais comuns de hayagriva:
- `editor`/`series`/`note`/`isbn`/`location`/`organization`
  todos `Option<String>`.
- Builder pattern fluente extendido (6 novos `with_*` métodos).
- Helper inline `optional_str` reusado N=12 cumulativos
  (largamente acima limiar promoção N=3-4).
- Layout `format_bib_entry` extendido com concatenação
  condicional APA-like extendida (decisões §8.2 + §9).

**Decisões arquitecturais P159G**:
- **6 fields universais comuns** — segunda metade (após P159D
  4 fields + P159E 2 fields).
- **Builder pattern fluente** paridade P159D/E.
- **Backwards compat trivial** — fields novos default `None`
  preservam output P159E.
- **Layout APA-like extendido** — ordem decidida §8.2; formatos
  individuais §9.
- **organization substitutivo a publisher** quando publisher
  ausente (decisão arbitrária per ADR-0054 graded).
- **Subpadrão #16 N=2 → 3** atinge limiar formalização.

**Decisões diferidas (NÃO reservadas)**:
- Restantes fields vanilla (`booktitle`/`address`/`chapter`/
  `type`/`institution`/etc.).
- Tipos estruturados (`Vec<Person>` editor, location codes,
  ISBN validation).
- CSL real (depende hayagriva ADR-0062).
- Promoção `optional_str` a helper público (N=12 cumulativos
  largamente promovível).
- ADR meta subpadrão #16 (N=3 atinge limiar formalização).

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (plaintext APA).
- ADR-0054: graded scope-out de validation/tipos estruturados.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: NÃO directamente aplicável.
- ADR-0065 critério #5: scope determinado por inventário.

**Tests planeados**: Δ +10 (range spec 8-12).

**Risco**: baixo. Pattern P159D replicado pela terceira vez;
helper `optional_str` reusado; backwards compat trivial via
fields opcionais default `None`; decisões cosméticas em §8/§9
sem impacto estrutural.
