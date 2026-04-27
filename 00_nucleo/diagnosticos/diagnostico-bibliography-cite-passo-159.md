# Diagnóstico Bibliography + Cite — Passo P159

Inventário diagnóstico precedendo materialização per **ADR-0034 +
ADR-0065** — análogo estrutural a P157 (table foundations) e
P158 (figure-kinds). **Décima quarta aplicação consecutiva** do
padrão diagnóstico-primeiro; **terceira aplicação concreta** do
critério #5 ADR-0065 (scope determinado por inventário) após
P157 e P158.

P159 é o **maior dos três passos Model Fase 2 reservados** (XL
declarado em ADR-0060). Diagnóstico é particularmente importante
para decidir como dividir ou se subset minimal é viável.

---

## §1 — ADRs relevantes a Bibliography + Cite

### §1.1 ADR-0060 sobre Bibliography

ADR-0060 §"Decisão 2" — Fase 2:

> **ADR-0062 + Passo 159** (renumerados de ADR-0061+P158 em P156B)
> — `Content::Bibliography` + `Content::Cite` com autorização
> `hayagriva`. ADR-0062 documenta autorização (precedente
> ADR-0024 ecow, ADR-0023 indexmap, ADR-0057 hypher). Crate
> `hayagriva 0.9.1` já em cache local (per P152).

ADR-0060 §"Plano de materialização":

> ADR-0062 + 159 (renumerados em P156B) | **XL** | bibliography
> + cite | ADR-0062

**Subset declarado**: 2 variants Content novas (`Bibliography`,
`Cite`) com integração `hayagriva`. Tamanho XL — maior do roadmap.

### §1.2 ADR-0062 sobre hayagriva — NÃO existe como ficheiro

**Verificação**: `00_nucleo/adr/typst-adr-0062-*.md` **não existe**.
ADR-0062 é apenas **reserva documentada** em README ADRs e ADR-0060,
sem ficheiro materializado. Status implícito: reserva (não PROPOSTO/
IDEIA/etc.).

**Implicação crítica**: integração `hayagriva` exige criação
de ADR-0062 antes ou durante o passo de materialização. P159A
não pode usar `hayagriva` sem ADR-0062 promovida a `IMPLEMENTADO`
(ou pelo menos `PROPOSTO`).

### §1.3 ADR-0017 sobre Introspection runtime

ADR-0017 status `IMPLEMENTADO` (Passo 1; 2026-03-26). Decisão:
**`eval()` não migra neste passo** porque `typst-library` tem
dependências de I/O e rendering. Lista 7 imports incluindo
`typst_library::introspection::{EmptyIntrospector, Introspector}`.

**Impacto em cite()**: vanilla `cite` resolve cross-document via
`Introspector` runtime (counters cruzados pageinas). Sem
Introspection runtime em cristalino, cite() pode:
- (a) Resolver em walk single-pass como counters figure (P75/
  introspect.rs:279) — **viável se entries existem antes de cite**.
- (b) Bloquear se entries forem definidas after cite — **scope-out
  per ADR-0054 graded**.

ADR-0017 NÃO bloqueia cite() em walk single-pass; bloqueia apenas
cite() forward references.

### §1.4 ADRs auxiliares

- ADR-0024 (ecow), ADR-0023 (indexmap), ADR-0057 (hypher) —
  precedentes de autorização de crate externa em L1; aplicáveis
  a hayagriva se necessário.
- ADR-0033 (paridade observável) — divergência estrutural aceite;
  cite renderiza placeholder `[key]` se CSL scope-out.
- ADR-0054 (perfil graded) — autoriza scope-out de CSL parsing,
  numbering schemes, form variants.

---

## §2 — Estado de Bibliography + Cite em código

### §2.1 Pesquisa exaustiva

```bash
grep -rn "Content::Bibliography\|Content::Cite\|BibliographyEntry\|native_bibliography\|native_cite" 01_core/src --include="*.rs"
```

**Zero matches**. Bibliography + Cite **completamente ausentes**
em código cristalino.

### §2.2 Tabela A.6 Model

- `cite(key)`: **`ausente`** | "requer bibliography".
- `bibliography(path)`: **`ausente`** | "escopo XL: CSL parsing".

### §2.3 DEBT-55 — conteúdo completo

**DEBT-55** (P154A; renumerada P156B): Bibliography + Cite XL,
pré-condição ADR-0062 hayagriva.

**Plano declarado**:
- [ ] ADR-0062 criada (autorização hayagriva).
- [ ] `Cargo.toml` + `crystalline.toml` configurados.
- [ ] `Content::Bibliography {...}` + `Content::Cite {key,
  supplement, form}` variants.
- [ ] `native_bibliography` + `native_cite` em stdlib.
- [ ] Pipeline introspect com resolução cruzada.
- [ ] Render layout para ambos.
- [ ] 5-10 testes; corpus paridade ganha 2-3 ficheiros.

**Critério de fecho**:
- [ ] ADR-0062 IMPLEMENTADO.
- [ ] bibliography + cite materializados.
- [ ] Tests verdes; lint zero.
- [ ] Inventário 148 actualizado.

**Notas DEBT-55**: explicitamente menciona **"pode ser materializado
em paralelo com Fase 1 ou após"** — não bloqueia outras fases.

### §2.4 Estado vanilla — referência

`lab/typst-original/.../model/bibliography.rs`: **1226 linhas**.
Importa `hayagriva::{archive, io, Entry, ...}` directamente.

`BibliographyElem` campos:
- `sources: Derived<OneOrMultiple<DataSource>, Bibliography>` —
  fonte (path, bytes ou inline).
- `title: Smart<Option<Content>>` — título da seção.
- `full: bool` — full vs cited only.
- `style: Derived<CslSource, CslStyle>` — estilo CSL.
- `lang`, `region` — localização.

`lab/typst-original/.../model/cite.rs`: **182 linhas**.

`CiteElem` campos:
- `key: Label` — chave da entrada (required).
- `supplement: Option<Content>` — page/chapter.
- `form: Option<CitationForm>` — Normal/Prose/etc. (default Normal).
- `style: Smart<...>` — CSL style override.

**Acoplamento profundo com hayagriva**: `Bibliography` interno usa
`Arc<ManuallyHash<IndexMap<Label, hayagriva::Entry, FxBuildHasher>>>`.

### §2.5 Hashes actuais relevantes

- `entities/content.rs`: `ec58d849` (preservado 7 passos).
- `rules/stdlib/structural.rs`: hash via lineage.
- `rules/introspect.rs`: hash via lineage.

---

## §3 — Scope de "bibliography + cite" — avaliação 3 estruturas

### §3.1 Decomposição conceptual

3 conceitos vanilla **acoplados mas separáveis em cristalino**:

1. **Bibliography** (lista de entries) — pode ser:
   - Vanilla: parse `.bib`/`.yaml` via hayagriva + CSL style.
   - Cristalino mínimo: `Vec<BibEntry>` literal com fields
     hardcoded (key, author, title, year).
2. **Cite** (referência inline) — pode ser:
   - Vanilla: CSL formata consoante estilo (autor-ano, numérico,
     etc.).
   - Cristalino mínimo: render placeholder `[key]` ou
     `[author, year]` simples.
3. **Numbering scheme** (numérico/autor-ano/etc.) — pode ser:
   - Vanilla: CSL escolhe formato.
   - Cristalino mínimo: scope-out per ADR-0054 graded; só
     `[key]` placeholder.

### §3.2 Estrutura A — multi-passo análogo a P157

- **P159A**: `Content::Bibliography` minimal (Vec<BibEntry>
  literal; sem CSL; sem hayagriva). M.
- **P159B**: `Content::Cite` minimal (key lookup; placeholder
  render). M.
- **P159C**: integração hayagriva + CSL + numbering schemes
  (XL; ADR-0062 promovida). XL/L+.

Vantagens:
- Granularidade preservada N=14/15/16 nos primeiros 2 sub-passos.
- ADR-0062 só necessária em P159C — adia decisão de crate externa.

Desvantagens:
- 3 sub-passos (paridade P157A/B/C); algum overhead.
- P159C é XL inerente.

### §3.3 Estrutura B — minimal análogo a P158

- **P159A**: `Content::Bibliography` + `Content::Cite` num
  único passo M+, sem hayagriva. Granularidade quebra (2
  features simultâneas). Render placeholder.

Vantagens:
- Único sub-passo; menos overhead administrativo.

Desvantagens:
- Quebra granularidade N=13 → M+.
- Mistura concerns (Bibliography é container de entries; Cite é
  referência) — análogo a P157A/B sendo um único passo.

### §3.4 Estrutura C — diferimento total

- **P159A**: passo administrativo XS de scope-out documentado.
- Bibliography + Cite ficam ausentes formalmente per ADR-0054
  graded até ADR-0062 promovida e/ou Introspection runtime
  resolvida.

Vantagens:
- Sem código novo; risco zero.
- Documenta honestamente que XL é grande demais para subset
  minimal viável neste momento.

Desvantagens:
- Cobertura Model não cresce.
- DEBT-55 mantém-se aberto sem progresso.

### §3.5 Recomendação adoptada — Estrutura A com adaptação realista

**P159A**: `Content::Bibliography` + `Content::Cite` **par
acoplado** (paridade decisão P157C par simétrico Header+Footer)
materializados num único passo **M+** preservando precedente
"par lógico contado como 1 unidade" mas com 2 variants distintos.

Justificação:
- Bibliography e Cite são **inseparáveis funcionalmente** —
  `cite(key)` só faz sentido com `bibliography([entries])`
  presente. Materializar separadamente seria artificial.
- Paridade vanilla: cite e bibliography vivem em ficheiros
  distintos mas semanticamente acoplados.
- Granularidade quebrada **honestamente registada** (M+ vs M
  target) — análogo a P156C (pad+hide M+ par lógico).
- Tamanho M+ aceitável para par funcional acoplado per
  precedente P156C/D.

Escopo final P159A:
- `Content::Bibliography { entries: Vec<BibEntry>, title: Option<Box<Content>> }`
  com `BibEntry { key, author, title, year }` em
  `entities/bib_entry.rs` novo.
- `Content::Cite { key: String, supplement: Option<Box<Content>> }`.
- Stdlib `native_bibliography(entries: array, title: ?)` e
  `native_cite(key, supplement: ?)`.
- Layout: Bibliography renderiza entries como lista; Cite
  renderiza placeholder `[key]`.
- Introspect: Bibliography não conta em counters; Cite walk
  single-pass valida key existe (warning se não — ou aceita
  graded).
- Tests ~15-20.

**Sem hayagriva, sem CSL, sem form variants, sem numbering
schemes** per ADR-0054 graded (refinos futuros NÃO reservados
per política P158).

**Sub-passos seguintes possíveis** (NÃO reservados):
- Integração hayagriva (P159B futuro com ADR-0062 promovida).
- CSL parsing (P159C futuro XL).
- Form variants Normal/Prose (P159D futuro).

---

## §4 — Dependências bloqueantes

### §4.1 Dependências internas

| Dependência | Estado | Bloqueia P159A subset minimal? |
|-------------|--------|:-------------------------------:|
| `Content::Figure` infraestrutura | implementado⁺ | Não |
| Counter system (introspect.rs) | implementado | Não — Bibliography não usa counters |
| Walk single-pass | implementado | Não — Cite resolve em walk com warning se key ausente |
| FieldAccess `Value::Func.subname` | NÃO implementado | Não — naming flat `bibliography` e `cite` per padrão P157B |
| Introspection runtime ADR-0017 | adiada | **Não** para subset minimal — Cite resolve forward references como warning per ADR-0054 graded; **Sim** para refino futuro com cross-document forward |

### §4.2 Dependências externas

| Dependência | Estado | Bloqueia P159A subset minimal? |
|-------------|--------|:-------------------------------:|
| **hayagriva crate** | pré-cache local; ADR-0062 NÃO criada | **Não** para subset minimal — input cristalino é Vec<BibEntry> literal, sem parsing externo |
| CSL parser | depende hayagriva | Não — render placeholder per ADR-0054 graded |

### §4.3 DEBTs abertos relevantes

| DEBT | Descrição | Impacto P159A |
|------|-----------|---------------|
| **DEBT-55** | Bibliography + Cite XL com pré-condição ADR-0062 | **Pré-condição contornada** em subset minimal — input literal sem hayagriva; DEBT-55 permanece aberto para refinos futuros |

### §4.4 ADRs em vigor relevantes

| ADR | Aplicação a P159A subset minimal |
|-----|----------------------------------|
| ADR-0017 | Estratégia gradual — autoriza scope-out de Introspection runtime |
| ADR-0026/-R1 | Content enum fechado — variants Bibliography + Cite compatíveis |
| ADR-0033 | Paridade observable estrutural — Cite render placeholder aceite |
| ADR-0034 | Diagnóstico cumprido (este doc) |
| ADR-0054 | Perfil graded — autoriza scope-out de hayagriva, CSL, form, numbering |
| ADR-0060 | Roadmap Model — autoriza P159 como sub-passo Fase 2 |
| ADR-0064 | Smart→Option/default — Caso A potencialmente aplicável a Cite.form (`Smart<CitationForm>` → `Option<CitationForm>`); subset minimal sem form |
| ADR-0065 critério #5 | Inventariar primeiro — terceira aplicação concreta |

### §4.5 ADRs pendentes / candidatas

| ADR | Estado | Bloqueia P159A subset minimal? |
|-----|--------|:-------------------------------:|
| ADR-0061 (Layout roadmap) | PROPOSTO | Não |
| **ADR-0062 (hayagriva)** | reserva sem ficheiro | **Não** para subset minimal; **Sim** para refinos futuros com hayagriva |

### §4.6 Conclusão de dependências

**Zero bloqueios hard** para P159A subset minimal estrutura A
adaptada. Toda a infraestrutura interna existe; hayagriva
contornada com input cristalino literal.

**Bloqueios para refinos futuros** (NÃO scope P159A):
- Integração hayagriva → exige ADR-0062 promovida (passo
  administrativo prévio).
- CSL parsing → exige hayagriva.
- Cross-document forward refs → exige Introspection runtime
  ADR-0017 promovida.

---

## §5 — Esboço de P159A (passo substantivo seguinte)

### §5.1 Identificador

**P159A** — segue precedente P157A/B/C + P158A.

### §5.2 Tamanho

**M+** (par funcional acoplado Bibliography + Cite; quebra
granularidade N=13 → M+; análogo a P156C pad+hide M+ par
lógico).

### §5.3 Subset concreto

#### Tipo novo `BibEntry` em `entities/bib_entry.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BibEntry {
    pub key:    String,
    pub author: Option<String>,
    pub title:  Option<String>,
    pub year:   Option<i32>,
}
```

#### Variants `Content::Bibliography` + `Content::Cite`:

```rust
Bibliography {
    entries: Vec<BibEntry>,
    title:   Option<Box<Content>>,
},
Cite {
    key:        String,
    supplement: Option<Box<Content>>,
},
```

#### Stdlib em `stdlib/structural.rs` (continuação Model):

- `native_bibliography(entries: array, title: ?)`.
- `native_cite(key, supplement: ?)`.
- Naming flat per padrão P157B (sem `bibliography.style` etc.).

#### Layout:
- `Content::Bibliography`: render entries como lista (paridade
  Terms/TermItem em layout).
- `Content::Cite`: render placeholder `[key]`.

#### Introspect:
- Bibliography não consume counter.
- Cite walk single-pass (sem validação rigorosa neste subset
  per ADR-0054 graded).

### §5.4 Sub-passos previstos (alto nível)

1. **Inventário** em `diagnostico-bibliography-cite-passo-159a.md`.
2. **Tipo `BibEntry`** em `entities/bib_entry.rs` novo.
3. **Variants `Bibliography` + `Cite`** em `entities/content.rs`
   (cobertura 9 sítios pattern-match).
4. **Stdlib `native_bibliography` + `native_cite`** em
   `structural.rs`.
5. **Layout arms** em `layout/mod.rs`.
6. **Introspect arms** em `introspect.rs`.
7. **Tests** ~15-20 (variants + stdlib + layout E2E).
8. **Hashes**: `crystalline-lint --fix-hashes` (esperado
   actualizar `entities/content.rs` — primeira alteração ao
   variant Content após **7 passos consecutivos** preservando
   hash; **quebra padrão de estabilidade**).

### §5.5 Granularidade

**Quebrada N=13 → M+** honestamente registada — par funcional
acoplado Bibliography+Cite análogo a P156C par lógico pad+hide
M+. **Não viola padrão metodológico** (precedente existe).

### §5.6 Padrões aplicáveis

- **ADR-0064 Caso A potencialmente aplicável** se Cite.form for
  introduzido em refino futuro (`Smart<CitationForm>` →
  `Option<CitationForm>`); **NÃO em P159A**.
- **ADR-0065 critério #1 (naming flat)** + critério #5 (scope)
  + critério #6 (divergência da spec — Vec<BibEntry> literal vs
  hayagriva parsing).
- **Reuso de tipo novo `BibEntry`**: N=1 aplicação concreta.
  Candidato a futuro `BibEntryRef`/`BibStyle` se hayagriva
  integrada.

### §5.7 Risco estimado

**Médio**:
- Dois variants novos em paralelo (par lógico) — risco maior
  que single variant aditivo.
- Tipo novo `BibEntry` em entities — primeira alteração a
  entities após sequência longa de refinos em stdlib.
- Quebra padrão "preservação hash content.rs" 7 → 8 passos
  consecutivos — primeira alteração ao variant Content após
  P156L. **Documentar honestamente como quebra de subpadrão**.
- Subset minimal honesto (placeholder render) — paridade
  vanilla mínima per ADR-0054 graded.

---

## Resumo executivo

P159 confirma factualmente:

1. **ADR-0060 declara P159 XL** com `hayagriva` autorizada via
   ADR-0062 (NÃO criada como ficheiro).
2. **ADR-0062 é apenas reserva** sem ficheiro materializado —
   integração hayagriva exige criação prévia.
3. **ADR-0017 não bloqueia** subset minimal — Cite walk
   single-pass viável.
4. **Bibliography + Cite completamente ausentes** em código.
   DEBT-55 documenta plano completo XL com hayagriva.
5. **Vanilla integra hayagriva profundamente** (1226 linhas
   bibliography.rs; CSL style engine; Bibliography interno usa
   `hayagriva::Entry` directamente).
6. **3 estruturas avaliadas** em §3:
   - A multi-passo: P159A bibliography + P159B cite + P159C
     hayagriva (3 sub-passos M cada).
   - B minimal: par num único passo M+ sem hayagriva.
   - C diferimento: scope-out total per ADR-0054 graded.
7. **Recomendação**: **Estrutura A adaptada** — par acoplado
   Bibliography+Cite num único passo **M+** sem hayagriva
   (input cristalino literal Vec<BibEntry>); refinos futuros
   (hayagriva, CSL, form variants, numbering) **NÃO reservados**.
8. **Zero bloqueios hard** para subset minimal.
9. **P159A esboço M+**: tipo `BibEntry` novo + 2 variants Content
   + 2 stdlib funcs + layout + introspect + tests ~15-20.
   **Quebra padrão "preservação hash content.rs"** (sétimo passo
   consecutivo — P159A será o oitavo onde hash muda).

**Auto-validação ADR-0065 critério #5**: este diagnóstico é
**terceira aplicação concreta** após P157 (table foundations
multi-passo) e P158 (figure-kinds subset selection). Padrão
consolidado: scope determinado por inventário factual, não
inferido. Critério #5 demonstra flexibilidade cross-feature
(table multi-passo; figure-kinds subset; bibliography par
acoplado).

**Política "sem novas reservas" preservada** (P158 estabeleceu;
P158A respeitou; P159 respeita). Refinos futuros pós-P159A
permanecem candidatos NÃO-reservados.
