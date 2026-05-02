# Lacunas de captura M1 — `Vec<Tag>` vs `CounterStateLegacy`

Documento gerado durante P163 sub-passo .E para registar divergências detectadas entre o `Vec<Tag>` produzido pelo walk e o `CounterStateLegacy` mutado em paralelo.

P163 verificou consistência entre as duas representações via tests E2E. Esta nota documenta o que **não** é estritamente equivalente e a decisão tomada para cada caso.

---

## Lacuna #1 — `figure.kind` literal em tags vs colapsado em state

**Detectada em**: `figures_capturadas_em_paralelo` (P163 .D.2).

**Comportamento**:

- `Tag::Start(_, ElementInfo { payload: ElementPayload::Figure { kind: Option<String>, .. } })`: `kind` é o valor **literal** de `Content::Figure.kind`. Se input tem `kind: None`, o payload tem `kind: None`. Se input tem `kind: Some("image")`, payload tem `kind: Some("image")`.
- `state.figure_numbers: HashMap<String, Vec<usize>>`: kind é resolvido para "image" como default quando `kind: None`, conforme arm Figure existente de walk (`let kind_key = kind.as_deref().unwrap_or("image").to_string();`). Logo, 2 figures com `kind: Some("image")` e `kind: None` ambas contam para a mesma chave `"image"` em state.

**Implicação**: tags preservam mais informação que state — kind=None é distinguível de kind=Some("image") em tags, mas indistinguível em state.

**Decisão**: registar como divergência conhecida, **adiar correcção**. Tags com kind literal são úteis para futuro consumidor (`Introspector` em M3) que pode querer query por kind exacto. State agrega para o uso actual do Layouter (numeração visual). Os dois usos são compatíveis com as suas finalidades.

**Critério para reabrir**: se M2/M3 começar a consumir tags e descobrir que o consumer quer "todas as figures de kind 'image' incluindo as None resolvidas", então `extract_payload` deveria aplicar o mesmo default que walk. Sem esse caso de uso real, manter divergência.

---

## Lacuna #2 — `auto_label` para headings em state vs ausência em tags

**Detectada por inspecção** (não disparou test).

**Comportamento**:

- Walk arm `Content::Heading` cria uma `auto_label` automática (`Label("auto-toc-N")`) em `state.headings_for_toc` e `state.resolved_labels` para que a TOC possa referenciar o heading mesmo sem label explícita.
- `Tag::Start(_, ElementInfo { payload: Heading{..}, label })`: `label` é apenas a label vinda de `Content::Labelled` wrapper. Se o heading não estiver em wrapper, `label = None`.

**Implicação**: state guarda labels automáticas para todos os headings (mesmo sem `<label>` explícita); tags só guardam labels explícitas.

**Decisão**: registar como divergência conhecida, **adiar**. Auto-labels são um detalhe de implementação da TOC cristalina single-pass. Em vanilla, a TOC consome o `Introspector` directamente sem precisar de auto-labels. Quando M3 introduzir Introspector, auto-labels podem ser eliminados (TOC consume tags directamente). Em M1 manter ambos.

**Critério para reabrir**: M3 — quando Introspector for materializado, decidir se auto-labels devem ser removidas de state ou se devem aparecer em tags também (`label: Some(Label("auto-toc-N"))`).

---

## Lacuna #3 — `headings_for_toc` carrega frozen body em state vs hash em tags

**Detectada por inspecção** (não disparou test).

**Comportamento**:

- Walk arm `Content::Heading` chama `materialize_time(body, state)` e empurra `(auto_label, frozen_body, level)` para `state.headings_for_toc`. O body inteiro é guardado para a TOC poder renderizar formatação rica.
- `Tag::End(loc, content_hash)`: guarda apenas o hash u128 do body. O conteúdo real do body não está em tags.

**Implicação**: state guarda Content completo (potencialmente pesado); tags só guardam hash.

**Decisão**: **correcto, manter**. Tags são para introspecção e queries; o body real serve consumidores diferentes (TOC) e fica em state. Esta divergência é arquitecturalmente desejada: tags são leves (hash + ID + payload pequeno); state guarda dados pesados quando necessário para a feature actual (TOC).

---

## Lacunas adicionais — detectadas em P167

P167 (M5 sub-passo 1, 2026-04-30) inventariou todos os consumers de `CounterStateLegacy` e mapeou cada field/método contra `TagIntrospector`. Confirmou 4 lacunas adicionais não cobertas pelas 3 originais.

### Lacuna #4 — `is_numbering_active` / `numbering_active`

`CounterStateLegacy.numbering_active: HashMap<String, bool>` controla por chave se a numeração está activada (populado pelo walk arm `Content::SetHeadingNumbering`). `TagIntrospector` não captura este estado. Consumer típico: `Layouter` consulta `is_numbering_active("heading")` antes de formatar prefixo de heading.

**Decisão**: adiar para passo dedicado (M9 ou similar) que adicione mecanismo de numbering-active ao Introspector. Possíveis caminhos:
- Adicionar variant locatable `SetHeadingNumbering` a `ElementPayload`.
- Adicionar campo `numbering_state: HashMap<String, bool>` a `TagIntrospector` populado por extracção paralela em `from_tags`.

### Lacuna #5 — `format_hierarchical` / hierarquia em `CounterRegistry` — ✅ **RESOLVIDA em P170**

`CounterStateLegacy.format_hierarchical("heading")` retorna string "1.2.3" (hierárquica). `CounterRegistry` (M3) era flat — `value(key)` retornava `&[usize]` mas só com 1 elemento porque `apply` não preservava hierarquia.

**Resolução em P170 (M9 sub-passo 2)**:
- `CounterRegistry::apply_hierarchical(key, level)` adicionado — paridade exacta com `CounterStateLegacy::step_hierarchical`.
- `CounterRegistry::format(key) -> Option<String>` adicionado — joins Vec<usize> com ".".
- `Introspector::formatted_counter(key) -> Option<String>` adicionado — método trait que delega para `counters.format`.
- `from_tags` arm Heading usa `apply_hierarchical(_, depth)` em vez de `apply(_, Step)` flat.
- Test E2E em `introspector_consistencia_heading` confirma paridade com `state.format_hierarchical` para sequência [1,2,2,3] → "1.2.1".

Counter-rico via `CounterKey` enum (Page/Selector/Str variants) permanece adiado — cristalino mantém String key (forma `Str` apenas) por enquanto. Outras variants ficam para passos futuros se algum consumer exigir.

### Lacuna #6 — `bib_entries` / `bib_numbers` — ✅ **RESOLVIDA em P181**

`CounterStateLegacy` armazenava `Vec<BibEntry>` e `HashMap<String, u32>` populados pelo walk arm `Content::Bibliography`. `TagIntrospector` não tinha mecanismo equivalente — `extract_payload` em M1 não cobria Bibliography.

**Resolução em P181 (sub-passos `.A`–`.I`, `diagnostico-bib-store-passo-181a.md` + relatórios P181B–P181I)**:

Mecanismo:
- **P181B**: sub-store `BibStore` em `01_core/src/entities/bib_store.rs` (`Vec<BibEntry>` + `HashMap<String, u32>`); field `pub bib_store: BibStore` em `TagIntrospector`.
- **P181C**: `ElementKind::Bibliography` + `ElementPayload::Bibliography { entries: Vec<BibEntry> }` adicionados aos enums discriminadores.
- **P181D**: `Content::Bibliography` promovida a locatable; `extract_payload` arm produz payload com entries.
- **P181E**: `from_tags` arm Bibliography popula `BibStore` via `add_bibliography(entries)` + loop de `assign_number(key, numbers_len()+1)`.
- **P181F**: trait `Introspector` ganha `bib_entry_for_key` + `bib_number_for_key`; impl em `TagIntrospector` delega para `BibStore`.
- **P181G**: Layouter cite-arm consulta `self.introspector.bib_*_for_key(...)` com fallback a state legacy (padrão substitution-with-fallback P168).
- **P181H**: walk arm `Content::Bibliography` puro (P163 invariante restaurada para bib); `layout()` legacy re-corre `introspect_with_introspector` internamente para obter Introspector populado.
- **P181I**: tests E2E codificam invariantes (pipeline completo + walk puro + multi-Bib concat + or_insert + 4 cite forms).

Critérios P181A §2.6 (Opção 3) verificados literalmente:
1. ✅ `01_core/src/entities/bib_store.rs` existe com `BibStore { entries, numbers }` + 9 métodos.
2. ✅ `Introspector::bib_entry_for_key` + `bib_number_for_key` no trait + impl `TagIntrospector` delegante; `from_tags` arm Bibliography popula `bib_store`.
3. ✅ Layouter cite-arm em `mod.rs:591, 599` consulta via `self.introspector.bib_*_for_key(...)` (fallback a state legacy preservado para janela compat).

**Pendências M6**: campos legacy `bib_entries`/`bib_numbers` em `CounterStateLegacy` continuam a existir (vazios em produção pós-P181H); fallback cite-arm preservado como segurança extra; copy-sites em `pub fn layout`/`pub fn layout_with_introspector` preservados; re-walk em `layout()` legacy para construir Introspector. M6 elimina todos quando F1 retomar.

**M9 features**: 10/11 (Bibliography conta após fecho da lacuna #6). Restante: lacuna #4 (`numbering_active`).

### Lacuna #7 — `has_outline`

`CounterStateLegacy.has_outline: bool` indica se o documento contém `Content::Outline`. Layouter usa para decidir se fixpoint de páginas é necessário. `TagIntrospector` não rastreia.

**Decisão**: adiar. Caminho provável: `query_by_kind(Outline)` se Outline for promovido a payload kind, OU adicionar bool dedicado a `TagIntrospector` populado em `from_tags`.

**✅ Resolvida em P178**: `ElementKind::Outline` adicionado; `is_locatable(Content::Outline) == true`; `extract_payload` retorna `Some(ElementPayload::Outline)`; `from_tags` indexa em `kind_index[Outline]`. Stdlib `query("outline")` (P175) retorna agora count correcto. Equivalente a `has_outline := query("outline") > 0`. Caminho promovido foi o primeiro (`query_by_kind`), conforme caminho provável documentado.

---

## Resumo

7 divergências/lacunas documentadas (3 originais P163 + 4 novas P167). Nenhuma é bug — são consequências da topologia "Introspector M3 deliberadamente minimal".

| # | Divergência/Lacuna | Origem | Decisão |
|---|--------------------|--------|---------|
| 1 | `figure.kind` None vs "image" default | P163 | Adiar; relevante para P168 figure-ref filter |
| 2 | Auto-labels só em state | P163 | Adiar; M3+ |
| 3 | Body frozen em state vs hash em tags | P163 | Manter — intencional |
| 4 | `is_numbering_active` / `numbering_active` | P167 | Adiar — M9 ou passo dedicado |
| 5 | `format_hierarchical` / hierarquia em CounterRegistry | P167 | ✅ **Resolvida em P170** (M9 sub-passo 2) |
| 6 | `bib_entries` / `bib_numbers` | P167 | ✅ **Resolvida em P181** (`bib_store.rs` sub-store + `ElementKind::Bibliography` locatable + `Introspector::bib_*_for_key` + cite-arm migrado + walk puro restaurado; 3 critérios P181A §2.6 verificados; M9: 10/11) |
| 7 | `has_outline` | P167 | ✅ **Resolvida em P178** (cascade `ElementKind::Outline`) |

Sem alteração de código resultante deste documento. Sem ADR nova. Lista é instrumento de referência para passos M5+ que migrem consumers e M9+ que estendam Introspector.
