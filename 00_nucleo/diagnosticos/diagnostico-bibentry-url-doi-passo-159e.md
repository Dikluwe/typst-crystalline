# Diagnóstico BibEntry url + doi — Passo P159E

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
**primeiro sub-passo família 159 fora do Bloco A** do diagnóstico
P159B (Bloco A esgotado pós-P159F). **Vigésima segunda aplicação
consecutiva** do padrão diagnóstico-primeiro.

Refino estrutural de tipo entity `BibEntry` adicionando 2 fields
opcionais (`url`, `doi`) — par natural identificado em P159D §9
como candidato a sub-passo M futuro. **Pattern P159D replicado
fielmente** — subpadrão #16 "refino tipo entity sem alteração
ao variant Content" cresce N=1 → 2.

---

## 1. Assinatura vanilla `hayagriva::Entry` para url e doi

Per `hayagriva::Entry` (vanilla integração), `url` e `doi` são
fields tipográficos comuns. Vanilla:
- `url: Option<QualifiedUrl>` — wrapper around `url::Url` com
  validation.
- `doi: Option<String>` ou tipo dedicado conforme versão
  hayagriva.

**Subset minimal P159E**: `Option<String>` directo para ambos
per ADR-0054 graded. URL/DOI parsing e validation diferidos.

---

## 2. Comportamento observable

**Vanilla** (style numeric/APA com hayagriva):
- URL: hyperlink em PDF; plaintext em outros formatos.
- DOI: prefixo `https://doi.org/...` ou `doi:...` conforme style.

**Cristalino actual** (P159D — sem url/doi): output APA-like
sem url/doi.

**Cristalino P159E** (proposto): output APA-like com url/doi
appendados condicionalmente:
```
[smith2024] Smith, J.. On Crystal Math. Nature Communications vol. 12, pp. 1-10. ACM (2024) https://example.com/paper, doi:10.1234/abc.
```

**Plaintext simples** — sem hyperlinks per ADR-0033 + ADR-0054
graded. Hyperlinks dependem de Layout/PDF infrastructure
cross-módulo (Bloco C).

---

## 3. ADR-0064 caso aplicável

**NÃO directamente aplicável** — fields são `Option<String>`
trivial sem mapping `Smart<T>`. Pattern Optional simples paralelo
a P159D fields adicionais.

---

## 4. Variants Content existentes a estender

**Nenhum**. `Content::Bibliography` e `Content::Cite` ambos
inalterados. P159E toca apenas `BibEntry` struct em
`entities/bib_entry.rs`.

**Hash content.rs preservado esperado** — 16º passo consecutivo
via L0-baseline interpretation. **Hash bib_entry.rs também
preservado esperado** (paridade P159D resultado — extensão via
doc-comment não modifica prompt L0 `bib_entry.md`).

**Subpadrão #16 N=1 → 2**: "refino tipo entity sem alteração
Content" — P159D BibEntry 4 fields + **P159E BibEntry 2 fields**.

---

## 5. Helpers stdlib reusáveis

### 5.1 Helper inline `optional_str` (P159D) reusado

Localização: `rules/stdlib/structural.rs:580` (inline em
`extract_bib_entries`). Reuso directo:
- P159D N=2 usos no mesmo passo (4 fields cumulativos: volume/
  pages/journal/publisher).
- **P159E N=2 usos no mesmo passo** (2 fields: url/doi).
- **N=4 usos cumulativos** — atinge limiar promoção N=3-4.

**Promoção a `pub(super)` ou helper público diferida** per
política consistente. Reavaliação em passo administrativo XS
futuro NÃO reservado.

### 5.2 Builder pattern fluente

Métodos novos `with_url` e `with_doi` paridade P159D
(`with_volume`/etc.). Pattern consistente N=6 cumulativo
(4 P159D + 2 P159E).

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159E | Refino futuro |
|---------|--------------|---------------|
| 2 fields adicionais (url/doi) | ✓ implementado | restantes vanilla (editor/series/note/isbn/location/organization) NÃO reservados |
| Fields como `Option<String>` directo | ✓ implementado | `QualifiedUrl`/`Doi` estruturados NÃO reservados |
| Plaintext simples (sem hyperlinks) | ✓ implementado | Hyperlinks dependem Layout/PDF infra (Bloco C) |
| URL validation | ✗ scope-out | depende refino futuro |
| DOI validation regex | ✗ scope-out | depende refino futuro |
| Layout formato APA-like | ✓ implementado | CSL real depende hayagriva (ADR-0062) |

---

## 7. Tests planeados

### 7.1 Unit tests `BibEntry` (`entities/bib_entry.rs`, ~3)

1. Constructor com url/doi via builder (`with_url` + `with_doi`).
2. PartialEq cobre 10 fields agora (regressão de cada divergente).
3. Backwards compat: `new(4 args)` continua a produzir entry com
   url/doi `None` (regression P159A+P159D).

### 7.2 Stdlib tests (`stdlib/mod.rs`, ~3)

1. Parse com url/doi presentes.
2. Parse sem url/doi (regression P159D — 4 obrigatórios + 4
   opcionais P159D apenas).
3. Tipo errado em url/doi rejeitado com mensagem clara
   mencionando field específico.

### 7.3 Layout E2E tests (`layout/tests.rs`, ~2)

1. Bibliography com entry completa (incluindo url/doi)
   renderiza formato extendido APA-like (verifica presença de
   url + `doi:`).
2. Bibliography com entry mínima (sem url/doi) renderiza
   formato P159D original (regression — sem `doi:` ou URL no
   output).

**Δ esperado**: +8 tests (paridade P159D Δ=+8; range spec 5-8).

---

## 8. Decisão de ordem layout (§8)

### 8.1 Avaliação multi-critério

| Critério | Opção A: depois publisher (`... ACM. URL, doi:...`) | Opção B: antes (year) (`... ACM URL, doi:... (2024).`) | Opção C: depois (year) (`... ACM (2024). URL, doi:...`) |
|----------|-----------------------------------------------------|--------------------------------------------------------|---------------------------------------------------------|
| Paridade APA | ✓ alta | ✗ atípica | ✓ alta |
| Backwards compat P159D | ✓ ano final preservado | ✗ ano após URL | ✓ ano antes URL |
| Posição prominent URL | menor | maior | maior |

### 8.2 Opção C adoptada

**Justificação**:
- Paridade APA — `(year)` no final do bloco principal; URL/DOI
  como anexo após.
- Backwards compat P159D preservada — quando url/doi ausentes,
  output termina em `(year).` exactamente como P159D.
- URL/DOI como "metadata" prominent após o bloco principal.

**Formato decidido**:
```
[key] author. title journal vol. volume, pp. pages. publisher (year). url, doi:DDDD.
```

**Separadores**:
- `" "` entre `(year).` e url (espaço simples; URL inicia
  segmento auxiliar).
- `", doi:"` entre url e DOI (vírgula + prefixo `doi:` per APA).
- Sem separador final adicional.

---

## 9. Decisão de formato (§9)

### 9.1 URL formato

**Decisão**: plaintext literal `https://example.com/paper`.

**Justificação**:
- Subset minimal — sem URL parsing/validation.
- Plaintext compatível com qualquer output (PDF/HTML/text).
- Hyperlinks dependem Layout/PDF infra (Bloco C — NÃO reservado).

### 9.2 DOI formato

**Decisão**: prefixo `doi:10.1234/abc` (paridade APA estilo
prose).

**Alternativas rejeitadas**:
- `https://doi.org/10.1234/abc` — URL completa; menos comum em
  styles APA prose. Reservar para refino futuro se prioritário
  (NÃO reservado).
- `10.1234/abc` literal — sem prefixo identificador; ambíguo.

**Justificação**:
- Paridade APA estilo prose — `doi:` prefix identifica como DOI
  vs URL genérica.
- Compacto — economiza espaço vs URL completa.
- Reconhecível para humans (pattern bem estabelecido).

---

## Resumo executivo

P159E materializa **expansão de struct entity `BibEntry`** com
2 fields adicionais opcionais:
- `url: Option<String>` + `doi: Option<String>`.
- Builder pattern fluente extendido (`with_url`, `with_doi`).
- Helper inline `optional_str` reusado N=2→4 cumulativos
  (atinge limiar promoção N=3-4).
- Layout `format_bib_entry` extendido com concatenação
  condicional APA-like (Opção C ordem; URL plaintext; DOI
  prefixo `doi:`).

**Decisões arquitecturais P159E**:
- **2 fields universais** (url/doi) — par natural identificado
  em P159D §9.
- **Builder pattern fluente** paridade P159D.
- **Backwards compat trivial** — fields novos default `None`
  preservam output P159D.
- **Layout Opção C** — url/doi após `(year).` per paridade APA
  + backwards compat.
- **Formato APA** — URL plaintext; DOI prefixo `doi:`.

**Decisões diferidas (NÃO reservadas)**:
- Restantes fields vanilla (`editor`/`series`/`note`/`isbn`/
  `location`/`organization`).
- Tipos estruturados (`QualifiedUrl`/`Doi`).
- URL/DOI validation.
- Hyperlinks no output (Bloco C).
- Promoção `optional_str` a helper público.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (plaintext).
- ADR-0054: graded scope-out de validation/hyperlinks.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: NÃO directamente aplicável.
- ADR-0065 critério #5: scope determinado por inventário.

**Tests planeados**: Δ +8 (paridade P159D; range spec 5-8).

**Risco**: baixo. Pattern P159D replicado fielmente; helper
`optional_str` reusado; backwards compat trivial via fields
opcionais default `None`.
