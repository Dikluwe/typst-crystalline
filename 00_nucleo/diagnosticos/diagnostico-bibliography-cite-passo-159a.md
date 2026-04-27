# Diagnóstico Bibliography + Cite par acoplado — Passo P159A

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
primeiro sub-passo Bibliography + Cite per scope decidido em
diagnóstico P159 §3.5 (Estrutura A adaptada). **Décima quinta
aplicação consecutiva** do padrão diagnóstico-primeiro.

**Primeira aplicação isolada concreta de ADR-0065 critério #2**
(escolha de tipo) — decisão de `BibEntry` campos minimais 4
(key/author/title/year) é decisão arquitectural-chave registada
neste diagnóstico.

---

## 1. Assinatura vanilla minimal — confirmação

### 1.1 `BibliographyElem` vanilla campos

Per diagnóstico P159 §2.4 + relido:
- `sources: Derived<OneOrMultiple<DataSource>, Bibliography>` —
  fonte parseável (path/bytes/inline). **DIFERIDO** per P159A
  (input cristalino literal Vec<BibEntry>).
- `title: Smart<Option<Content>>` — título da seção. **CASO A
  ADR-0064 aplicado**.
- `full: bool` — full vs cited only. **DIFERIDO** per ADR-0054
  graded.
- `style: Derived<CslSource, CslStyle>` — CSL style. **DIFERIDO**.
- `lang`, `region` — i18n. **DIFERIDO**.

### 1.2 `CiteElem` vanilla campos

- `key: Label` (required). Label vanilla é `EcoString`-like;
  cristalino usa `String` directo per simplicidade.
- `supplement: Option<Content>` — page/chapter. **CASO A
  ADR-0064 aplicado**.
- `form: Option<CitationForm>` — Normal/Prose/etc. **DIFERIDO**.
- `style` — CSL override. **DIFERIDO**.

### 1.3 `BibEntry` cristalino — decisão de campos minimais

ADR-0065 critério #2 (escolha de tipo) — decisão isolada concreta:

```rust
pub struct BibEntry {
    pub key:    String,
    pub author: String,
    pub title:  String,
    pub year:   u32,
}
```

Justificação dos 4 fields:
- **`key`**: identificador único (paridade vanilla `Label`).
  Sem este, Cite não tem como referenciar.
- **`author`**: campo universal em todas as styles
  bibliográficas (autor-ano, IEEE, MLA, etc.). Sem este, citation
  é incompleta.
- **`title`**: campo universal idem.
- **`year`**: campo universal idem (formato `u32` para anos
  positivos; aceita 0 para "no year").

Fields vanilla **diferidos** per ADR-0054 graded:
- `volume`, `pages`, `journal`, `publisher`, `url`, `doi`, etc.
- Refino futuro candidato a expansão da struct sem breaking
  change (adição de `Option<String>` fields).

**Localização decidida**: `01_core/src/entities/bib_entry.rs`
ficheiro novo per padrão P156C `sides.rs`.

---

## 2. Comportamento observável (subset minimal)

**Vanilla**:
- `bibliography("works.bib", style: "apa")` parseia ficheiro
  externo via hayagriva + CSL.
- `cite(<key>)` resolve via Introspector cross-document; CSL
  formata.

**Cristalino P159A** (per ADR-0054 graded):
- ✓ `bibliography(entries, title)` aceita Vec<BibEntry> literal.
- ✓ `cite(key, supplement)` aceita key String.
- ✓ Layout placeholder: Bibliography como lista
  `"[{key}] {author}. {title} ({year})."`; Cite como `"[{key}]"`.
- ✗ Sem parsing de `.bib`/`.yaml` (hayagriva diferida).
- ✗ Sem CSL styles (placeholder fixo).
- ✗ Sem validação cross-reference Cite.key ∈ Bibliography.keys
  (ADR-0017 Introspection runtime adiada).
- ✗ Sem form variants Normal/Prose (default placeholder).

**Divergência aceite per ADR-0033**: paridade observable
estrutural mínima; refinos futuros podem reduzir divergência.

---

## 3. ADR-0064 caso aplicável

| Field | Caso ADR-0064 |
|-------|---------------|
| `Bibliography.title: Smart<Option<Content>>` vanilla | **Caso A** → `Option<Box<Content>>` cristalino |
| `Cite.supplement: Option<Content>` vanilla | (Option vanilla; passa directo) → `Option<Box<Content>>` |
| `Bibliography.entries`, `Cite.key` | tipos directos sem Smart |

**ADR-0064 Caso A patamar cresce**: P156G/H/I (Layout) +
P157B (Model TableCell.x/y) + **P159A (Model
Bibliography.title + Cite.supplement)** = N=4 → **N=5**.
Diversidade cross-domínio: 60% Layout + 40% Model.

---

## 4. Variants Content existentes a estender

**Nenhuma**. Bibliography + Cite são variants novos par acoplado.

---

## 5. Helpers stdlib reusáveis

Nenhum directo. **Helper privado novo** `extract_bib_entries`
em `stdlib/structural.rs`:

```rust
fn extract_bib_entries(val: Option<&Value>) -> SourceResult<Vec<BibEntry>>
```

Parse de `Value::Array(Vec<Value::Dict>)` onde cada Dict tem
keys `key`/`author`/`title`/`year`. Validação hard de fields
obrigatórios.

**Promoção a `pub(super)` ou helper público**: diferida per
política consistente N=2-3.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159A | Refino futuro |
|---------|--------------|---------------|
| Hayagriva integration (parsing externo) | ✗ scope-out | ADR-0062 promovida + passo dedicado (NÃO reservado) |
| CSL styles (numérico, autor-ano, IEEE, etc.) | ✗ scope-out | NÃO reservado |
| Form variants (Normal/Prose/Author/Year) | ✗ scope-out | NÃO reservado |
| Cross-reference validation `key ∈ entries` | ✗ scope-out | ADR-0017 promovida (NÃO reservado) |
| Numbering schemes dinâmicos | ✗ scope-out | NÃO reservado |
| Fields adicionais BibEntry (volume/journal/etc.) | ✗ scope-out | extensível sem breaking change |
| `lang`/`region` para i18n bibliography | ✗ scope-out | NÃO reservado |
| `full: bool` (full vs cited only) | ✗ scope-out | NÃO reservado |
| Variants Bibliography + Cite + tipo BibEntry | ✓ implementado | — |
| Stdlib funcs + parse Vec<BibEntry> | ✓ implementado | — |
| Layout placeholder render | ✓ implementado | — |

---

## 7. Tests planeados

### 7.1 Tipo entity tests (`bib_entry.rs`, ~3)

1. Constructor — fields acessíveis.
2. PartialEq — equivalência por todos os fields.
3. Debug formatting trivial.

### 7.2 Unit tests `Content::Bibliography` (`entities/content.rs`, ~5)

1. Constructor default (entries vazias, title None).
2. Constructor com entries e title.
3. `is_empty` proxy via entries+title.
4. `plain_text` concatena title + entries formatadas.
5. `PartialEq` cobertura.

### 7.3 Unit tests `Content::Cite` (`entities/content.rs`, ~4)

1. Constructor com key.
2. Constructor com key + supplement.
3. `is_empty` sempre false.
4. `plain_text` emite `"[{key}]"`.

### 7.4 Stdlib tests (~6)

3 pares Bibliography↔Cite:
- Defaults (entries vazias / supplement None).
- Argumentos completos.
- Validation hard (field obrigatório ausente / key vazia).

### 7.5 Layout E2E tests (~3)

1. Bibliography renderiza entries como lista.
2. Cite renderiza placeholder com key.
3. Bibliography e Cite no mesmo documento.

**Δ esperado**: +18-21 (range alinhado com esboço P159 §5).

---

## 8. Par acoplado — confirmação simetria

| Aspecto | Bibliography | Cite |
|---------|:------------:|:----:|
| `body: Box<Content>` | ✗ (entries + title) | ✗ (key + supplement) |
| Container ou leaf? | container | leaf |
| `is_empty` proxy via | entries.is_empty && title.is_none | sempre false (key non-empty) |
| `plain_text` | concatena title + entries | `"[{key}]"` |
| `PartialEq` | 2 fields | 2 fields |
| `map_text` recurse | em title; preserva entries | em supplement; preserva key |
| Layout | lista renderizada | placeholder |
| Walk single-pass | walk title; iterate entries (sem walk — dados puros) | walk supplement |

**Diferença intencional**: Bibliography é container semântico
de entries (struct dados, não Content); Cite é leaf placeholder.
Paridade em `PartialEq`/`map_text`/`walk` é parcial — nem todos
arms são linha-a-linha simétricos como P157C.

---

## 9. Localização decidida — `entities/bib_entry.rs` ficheiro novo

Per padrão P156C `sides.rs`:
- Ficheiro novo `01_core/src/entities/bib_entry.rs`.
- `pub mod bib_entry;` adicionado a `entities/mod.rs`.
- Sem L0 prompt dedicado (módulo entity novo; lineage gerado
  automaticamente).

**Alternativa rejeitada**: adicionar `BibEntry` a ficheiro
existente. Rejeitada porque viola ADR-0037 coesão por domínio
(BibEntry é tipo entity standalone, não pertence a nenhum
módulo existente).

---

## 10. Quebra padrão "estabilidade hash content.rs"

P159A é **primeiro passo a modificar variant Content** após
**8 passos consecutivos** preservando hash `ec58d849` (P156L
→ P159).

**Reconhecimento explícito**: quebra é **inevitável** porque
adicionar variants novos ao enum exige alteração ao ficheiro.
Padrão "passos aditivos preservam hash" preserva-se
**conceptualmente** (P159A é classificado como aditivo) mas
**quebra contagem** porque hash muda.

**Documentação**: relatório §"Análise de risco" e §"Estado
pós-passo" registam quebra honestamente. Padrão "estabilidade
hash" pode reformular-se como "8-passo run record" quebrado
em P159A.

---

## Resumo executivo

P159A materializa **par acoplado Bibliography + Cite minimal**:
- Tipo entity novo `BibEntry { key, author, title, year }` em
  `entities/bib_entry.rs`.
- Variants `Content::Bibliography { entries, title }` +
  `Content::Cite { key, supplement }`.
- Stdlib `native_bibliography` + `native_cite` em
  `structural.rs` (continuação Model).
- Layout placeholder render.
- Walk single-pass (sem cross-reference validation).
- Tests +18-21.

**Decisões arquitecturais P159A**:
- **Tipo entity novo** com 4 fields minimais — ADR-0065 critério
  #2 (escolha de tipo) primeira aplicação isolada concreta.
- **Par acoplado** num único passo M+ — quebra granularidade
  N=13 → M+ honestamente registada (precedente P156C).
- **Hayagriva contornada** com input literal — ADR-0062
  permanece reserva.
- **Layout placeholder** per ADR-0033 + ADR-0054 graded.
- **Quebra padrão "estabilidade hash content.rs"** após 8 passos
  consecutivos — reconhecida e documentada.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural mínima.
- ADR-0054: graded scope-out de hayagriva, CSL, form, numbering.
- ADR-0060: variants per Decisão 2 Fase 2.
- ADR-0064 Caso A: title (Bibliography) + supplement (Cite) —
  patamar Caso A cresce N=4 → 5.
- ADR-0065 critério #2 (escolha de tipo) primeira aplicação
  isolada + critério #5 (scope) reforçado.

**Tests planeados**: Δ +18-21.

**Risco**: médio. Mitigação: par acoplado é precedente conhecido
(P156C); tipo entity novo segue padrão P156C `sides.rs`; layout
placeholder evita acoplamento com hayagriva.
