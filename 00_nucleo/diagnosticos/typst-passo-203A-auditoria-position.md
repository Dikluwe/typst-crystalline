# Auditoria empírica P203A — Estado actual de Position

**Data**: 2026-05-05.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-203A.md`.
**Natureza**: factos empíricos — sem decisões. As decisões
ficam no diagnóstico (`typst-passo-203A-diagnostico.md`).

---

## §1 Estado de partida verificado

Per spec §2 — confirmações antes de auditar:

- ✅ `Introspector::position_of` retorna `Option<()>`
  (verificado: `01_core/src/entities/introspector.rs:55`).
- ✅ Comentário em `TagIntrospector` declara
  `// positions: HashMap<Location, Position> — adiado para M5/M9`
  (linha final do struct).
- ✅ Lacuna #1 último passo (canónico): P165
  (TagIntrospector criada com stub).
- ⚠️ **DIVERGÊNCIA CRÍTICA**: a definição operacional de
  lacuna #1 / #1b / #2 declarada pelo spec **não coincide
  com a definição canónica**. Ver A8 abaixo. Resultado: a
  premissa de P203A está errada. Registado em
  `P203A.div-1`.

---

## §2 Cláusulas de auditoria (A1-A10)

### A1 — Existência do tipo `Position` em L1 — **CONFIRMADO ausente**

**Comando**:
```bash
grep -rn "^pub struct Position\b\|^pub enum Position\b" \
  01_core/src/entities/ 01_core/src/contracts/
```

**Resultado**: nenhum match. Tipo `Position` **não existe**
em L1.

**Comando**:
```bash
grep -rn "Position " 01_core/src/entities/introspector.rs
```

**Resultado** (1 match):
```
01_core/src/entities/introspector.rs:53:    /// M3 stub: retorna sempre `None`. Mapa Location→Position fica
```

**Conclusão A1**: `Position` é apenas referido em
comentário de doc (`Mapa Location→Position fica`) e na
assinatura do stub `position_of() -> Option<()>`. **Não há
tipo nominal `Position` definido em L1**.

---

### A2 — Forma vanilla de `Position` — **CONFIRMADO**

**Comando**:
```bash
grep -rn "^pub struct Position\b\|^pub enum Position\b" \
  lab/typst-original/crates/
```

**Resultado** — 1 match não-relevante:
- `lab/typst-original/crates/typst-library/src/math/ir/item.rs:1166`:
  ```rust
  pub enum Position { Above, Below }
  ```
  Marker para super/subscript em matemática.
  **NÃO é o Position relevante** para introspection.

**Pesquisa adicional** — Position tipos relevantes:

```bash
grep -B3 -A8 "^pub struct PagedPosition\|^pub enum DocumentPosition" \
  lab/typst-original/crates/typst-library/src/introspection/position.rs
```

**Resultado**:

```rust
// lab/typst-original/crates/typst-library/src/introspection/position.rs

/// Physical position in a document, be it paged or HTML.
#[derive(Clone, Debug, Hash)]
pub enum DocumentPosition {
    /// If the document is paged, the position is expressed
    /// as coordinates inside of a page.
    Paged(PagedPosition),
    /// If the document is an HTML document, the position
    /// points to a specific node in the DOM tree.
    Html(HtmlPosition),
}

/// A physical position in a paged document.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PagedPosition {
    /// The page, starting at 1.
    pub page: NonZeroUsize,
    /// The exact coordinates on the page (from the top
    /// left, as usual).
    pub point: Point,
}

/// A position in an HTML tree.
pub struct HtmlPosition {
    element: EcoVec<usize>,
    inner: Option<InnerHtmlPosition>,
}
```

**Conclusão A2**: vanilla tem **família de tipos**:
- `DocumentPosition` (enum target-agnóstico).
- `PagedPosition` (paged: page + point).
- `HtmlPosition` (HTML: element path + inner).

Trait `Introspector::position` (linha 67):
```rust
fn position(&self, location: Location) -> Option<DocumentPosition>;
```

---

### A3 — Locais que consomem `position_of` — **CONFIRMADO**

**Comando**:
```bash
grep -rn "\.position_of(" 01_core/ 02_shell/ 03_infra/ 04_wiring/
```

**Resultado** (4 matches, todos no mesmo ficheiro):
- `01_core/src/entities/introspector.rs:346`:
  `assert_eq!(i.position_of(loc(1)), None);` — test
- `01_core/src/entities/introspector.rs:369`:
  `assert_eq!(i.position_of(loc(7)), None);` — test
- `01_core/src/entities/introspector.rs:55`: declaração
  no trait.
- `01_core/src/entities/introspector.rs:248`: impl em
  `TagIntrospector` (retorna `None`).

**Consumers em produção**: **ZERO**. Não há call site em
`01_core/src/rules/`, `02_shell/`, `03_infra/`, ou
`04_wiring/`. Apenas tests do próprio módulo invocam o
stub para confirmar que retorna `None`.

**Conclusão A3**: `position_of` é stub estrutural sem
consumers reais. Materializar concrete não desbloqueia
nenhum consumer existente. Trabalho seria "build for
future use".

---

### A4 — Stores existentes em `TagIntrospector` — **CONFIRMADO 9 sub-stores**

**Comando**:
```bash
awk '/^pub struct TagIntrospector/,/^\}/' \
  01_core/src/entities/introspector.rs
```

**9 sub-stores activos** (per snapshot §5):
1. `labels: LabelRegistry`
2. `counters: CounterRegistry`
3. `kind_index: HashMap<ElementKind, Vec<Location>>`
4. `figure_label_numbers: HashMap<Label, usize>` (P168)
5. `metadata: MetadataStore` (P169)
6. `state: StateRegistry` (P171)
7. `bib_store: BibStore` (P181B)
8. `resolved_labels: ResolvedLabelStore` (P193B)
9. `headings_for_toc: Vec<(Label, Content, usize)>` (P200B)

**Comentário interno final**:
```
// positions: HashMap<Location, Position> — adiado para M5/M9.
```

**Posição lógica de `positions` (caso seja adicionado)**:
provavelmente após `kind_index` e antes de
`figure_label_numbers`, ou no fim como 10º field
(consistente com padrão de adição cronológica).

---

### A5 — Walk fn — onde Position seria emitido — **NÃO PODE**

**Comando**:
```bash
grep -B2 -A12 "^pub(crate) fn walk" \
  01_core/src/rules/introspect.rs
```

**Resultado** — assinatura actual (7 parâmetros):

```rust
pub(crate) fn walk(
    content:            &Content,
    locator:            &mut Locator,
    tags:               &mut Vec<Tag>,
    intr:               &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang:               Option<&Lang>,
    label_from_parent:  Option<&Label>,
)
```

**Análise crítica**:
- Walk fn **não tem acesso a layout state**. Não conhece
  `current_page`, `current_y`, `pages`, ou `page_config`.
- Walk corre como **pre-pass de introspection** antes do
  layout. Não há informação de paginação disponível
  walk-time.
- `Locator` (parâmetro) é determinístico-por-construção
  e gere `Location` IDs — não tem informação de página.

**Conclusão A5**: walk-time **não pode** calcular
`Position` (page + point). A informação simplesmente não
existe nesse ponto da pipeline. Position precisa de
feedback do Layouter. **Caminho walk-time puro está
impossibilitado**.

---

### A6 — Layouter — onde Position é determinada — **CONFIRMADO**

**Comando**:
```bash
grep -n "page\|current_page\|page_number\|pages\b" \
  01_core/src/rules/layout/mod.rs | head -30
```

**Achados relevantes**:
- `pub(super) pages: Vec<Page>` (linha 85): páginas
  acumuladas (índice = page number - 1).
- `pub page_config: PageConfig` (linha 84): config activa.
- `current_page_is_empty()` (linha 244): detecta página
  vazia.
- `new_page()` (várias linhas): empurra Page para `pages`.
- `cursor_y, cursor_x` (linhas 88-89): coordenadas
  correntes na página activa.
- `current_location: Option<Location>` (linha 145; P185C):
  Location do último content locatable processado.

**Comando**:
```bash
grep -n "Location" 01_core/src/rules/layout/mod.rs | head -15
```

**Achados**:
- `use ... location::Location` (linha 23).
- `current_location: Option<Location>` (linha 145).
- `locator: Locator` (linha 139): gerador determinístico
  sincronizado com walk.

**Conclusão A6**: Layouter **tem toda a informação**
necessária para emitir Position:
- `pages.len()` → page number 1-based.
- `cursor_x, cursor_y` → ponto na página.
- `current_location` → Location associada.

Para emitir Position, falta apenas **canal de feedback**
para `TagIntrospector`. Mecanismo análogo ao já existente
`runtime: LayouterRuntimeState` (P190C/D pattern).

---

### A7 — Vanilla typst — `position_of` pipeline — **CONFIRMADO POST-LAYOUT**

**Comando**:
```bash
grep -rn "fn position\b\|positions:" \
  lab/typst-original/crates/typst-library/src/introspection/
```

**Trait** (`introspector.rs:68`):
```rust
fn position(&self, location: Location) -> Option<DocumentPosition>;
```

**Default impl** (`introspector.rs:141-143`):
```rust
fn position(&self, _: Location) -> Option<DocumentPosition> {
    None
}
```

**Construtor `ElementIntrospector::position`** (linha 417):
```rust
pub fn position(&self, location: Location) -> Option<&P> {
    self.locations.get(&location).map(|r| self.get_pos_by_idx(r.start))
}
```

**Implementação em typst-layout** (`typst-layout/src/introspect.rs:35-58`):

```rust
impl PagedIntrospector {
    pub fn new(pages: &[Page]) -> PagedIntrospector {
        let mut builder = PagedIntrospectorBuilder::default();
        let mut page_numberings = Vec::with_capacity(pages.len());
        let mut page_supplements = Vec::with_capacity(pages.len());

        // Discover all elements.
        for (i, page) in pages.iter().enumerate() {
            let nr = NonZeroUsize::new(1 + i).unwrap();
            page_numberings.push(page.numbering.clone());
            page_supplements.push(page.supplement.clone());
            builder.discover_frame(&page.frame, Transform::identity(),
                &mut |point| { PagedPosition { page: nr, point } });
        }

        builder.finish(...)
    }

    pub fn position(&self, location: Location) -> Option<PagedPosition> {
        self.elements.position(location).copied()
    }
}
```

**Pipeline vanilla**:
1. Layouter completa layout (todas as páginas geradas).
2. **POST-layout**: `PagedIntrospector::new(&[Page])`
   itera páginas + frames, computando `PagedPosition`
   por cada elemento descoberto.
3. `Introspector::position(location)` consulta o
   pre-computed mapping.

**Conclusão A7**: vanilla **não calcula Position
walk-time** nem **durante layout**. Calcula
**post-layout** sobre `&[Page]` finalizadas. Pipeline
divide-se em três fases:
1. `walk` introspect (mapping Location → Element).
2. `layout` produz `Vec<Page>`.
3. `PagedIntrospector::new(pages)` produz mapping
   `Location → PagedPosition`.

**Implicação para cristalino**: pipeline análoga
viável — adicionar fase 3 sobre `Layouter.pages`
finalizado.

---

### A8 — Lacuna #1b — definição operacional — **DIVERGÊNCIA CRÍTICA**

**Comando**:
```bash
grep -rn "#1b\|lacuna 1b\|Position-related" 00_nucleo/
```

**Achados**:

#### A8.1 — Definição canónica em P200 consolidado §7

```
| # | Lacuna | Pré-P200 | Pós-P200 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa (ortogonal a M5) |
| #1b | from_tags arm Figure sem gate `is_counted` | activa | activa (ortogonal) |
| #2 | reservada | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa, bloqueia E2-residuo | **fechada (P200B)** |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |
```

#### A8.2 — Definição em `m1-lacunas-captura.md` (canónico original)

7 lacunas catalogadas. As primeiras 3 (P163):

| # | Lacuna |
|---|--------|
| 1 | `figure.kind` literal em tags vs colapsado em state |
| 2 | `auto_label` para headings em state vs ausência em tags |
| 3 | `headings_for_toc` carrega frozen body em state vs hash em tags |

E 4 adicionais (P167): #4 (numbering_active) ✅ resolvida
P182, #5 (format_hierarchical) ✅ resolvida P170,
#6 (bib_entries) ✅ resolvida P181, #7 (has_outline)
✅ resolvida P178.

#### A8.3 — Conclusão A8

**As lacunas #1, #1b, #2 NÃO se referem a Position**:

- **#1 = Figure kind=None ↔ Introspector** (Figure
  introspection). Origem: P163; em ambos sistemas.
- **#1b = from_tags arm Figure sem gate `is_counted`**
  (sub-variante de #1; população from_tags).
- **#2 = reservada** (vazia). Em `m1-lacunas-captura.md`
  era "auto_label headings" mas P200 consolidado §7
  declara "reservada" sem definição activa.

**A premissa de P203A — "endereçar lacunas #1
(Position) e #1b (Position-related)"** — **é
empíricamente incorrecta**.

**Origem do erro**: a auditoria delta P201 §2 (escrita
nesta sessão por Claude Code executando P201) atribuiu
Position a #1/#1b/#2 baseando-se na frase do spec P201
"snapshot 2026-05-05 estado pré-M8". O spec P202 reificou
essa interpretação. O spec P203A herdou.

**Cadeia de propagação**:
1. P201 spec §4 C8 lista "lacunas residuais (#1
   Position, #1b Position-related, #2 Counter at
   locations)" como input para auditoria delta.
2. P201 auditoria delta §2 atribui "Position" /
   "Position-related" / "Counter at locations" a
   #1/#1b/#2 sem verificação cruzada com
   `m1-lacunas-captura.md` ou P200 consolidado §7.
3. P202 reconciliação cita auditoria delta §2 como
   fonte; reescreve snapshot mantendo "Position" como
   #1.
4. P203A spec herda e endereça "Position" como
   lacuna #1.

**Position concrete IS um concern real** (stub
`position_of() -> Option<()>` existe), **mas não é
formalmente catalogado como lacuna**. ADR-0066
(Introspection runtime adiada) cobre estrategicamente
o adiamento.

**Registo**: divergência marcada como `P203A.div-1`.

---

### A9 — Tests que tocam Position actualmente — **CONFIRMADO 2 stubs**

**Comando**:
```bash
grep -rn "position_of\|Position " 01_core/src/ 03_infra/src/ \
  | grep -E "test|assert"
```

**Resultado** (2 matches):
- `01_core/src/entities/introspector.rs:346`:
  `assert_eq!(i.position_of(loc(1)), None);`
- `01_core/src/entities/introspector.rs:369`:
  `assert_eq!(i.position_of(loc(7)), None);`

**Conclusão A9**: 2 tests no mesmo módulo confirmam que
o stub retorna `None`. Nenhum test E2E ou de paridade
exercita Position.

---

### A10 — Corpus de paridade — casos que precisam de Position — **CONFIRMADO ZERO**

**Comando**:
```bash
ls lab/parity/
grep -rn "position\|location" lab/parity/
```

**Achados**:
- `lab/parity/` contém Cargo.toml, corpus, reports, src,
  tests.
- Match `position` (5 matches): em `frame_dto.rs` —
  `item_positions: Vec<(f64, f64)>` (coordenadas de
  frame items para comparação layout). **NÃO é Position
  introspection**.

**Comando**:
```bash
grep -rn "\.position\b\|here()\|locate(" lab/parity/corpus/ 2>/dev/null
```

**Resultado**: nenhum match.

**Conclusão A10**: corpus de paridade **não tem** casos
que invoquem `here().position()`, `locate(...).position()`
ou `query(...).position()`. **Zero pressão empírica**
para materializar Position concrete agora.

---

## §3 Resumo dos achados empíricos

| Item | Etiqueta | Sumário |
|------|:--:|---------|
| A1 | CONFIRMADO ausente | Tipo `Position` não existe em L1 |
| A2 | CONFIRMADO | Vanilla: `DocumentPosition` enum + `PagedPosition` (page+point) + `HtmlPosition` |
| A3 | CONFIRMADO | 0 consumers de `position_of` em produção; só 2 tests stub |
| A4 | CONFIRMADO 9 sub-stores | 10º slot disponível para `positions` se decidido |
| A5 | NÃO PODE | Walk fn não tem informação de página; walk-time impossibilitado |
| A6 | CONFIRMADO | Layouter tem `pages`, `cursor_x/y`, `current_location` — informação suficiente |
| A7 | CONFIRMADO POST-LAYOUT | Vanilla calcula Position post-layout sobre `&[Page]` finalizadas |
| A8 | **DIVERGÊNCIA CRÍTICA** | Lacunas #1/#1b/#2 NÃO são Position; spec P203A baseado em premissa errada |
| A9 | CONFIRMADO | 2 tests stub apenas; nenhum test E2E exercita Position |
| A10 | CONFIRMADO ZERO | Corpus de paridade não tem casos que precisem de Position |

---

## §4 Divergências relevantes registadas

### `P203A.div-1` — Lacunas #1/#1b/#2 não são sobre Position

**Detectada em**: A8.

**Estado anterior** (snapshot 2026-05-05 §7; spec P203A §1):
```
| #1  | Position           | residual |
| #1b | Position-related   | residual |
| #2  | Counter at locations | parcial |
```

**Estado real** (P200 consolidado §7; m1-lacunas-captura.md):
```
| #1  | Figure kind=None ↔ Introspector       | activa (ortogonal) |
| #1b | from_tags arm Figure sem gate is_counted | activa (ortogonal) |
| #2  | reservada                              | — |
```

**Origem**: erro propagado de P201 auditoria delta §2 →
P202 reconciliação → P203A spec.

**Decisão**: registar e ramificar (per spec P203A §6
opção "ramificar"). Ver diagnóstico C-cláusulas para
plano de pivot.

---

## §5 Adicional — outras observações

### §5.1 ADR-0066 já endereça Position estratégicamente

ADR-0066 (Introspection runtime adiada; ACEITE em P192B
com nota "intermediário até M8") cobre a decisão
estratégica de adiar Position runtime para M8.

Materializar Position concrete antes de M8 é
**redundante** com o trabalho M8 (paridade vanilla via
comemo, que naturalmente cobre Position).

### §5.2 Pipeline cristalino vs vanilla — análise estrutural

**Vanilla** divide em 3 fases:
1. Walk introspect → mapping Location → Element.
2. Layout produz `Vec<Page>`.
3. `PagedIntrospector::new(pages)` produz mapping
   Location → PagedPosition (post-layout).

**Cristalino actual** tem fases 1-2; falta fase 3.

**Implementação cristalina viável** (caso a decisão seja
materializar):
- Adicionar `runtime.label_pages` já existe (P190C).
- Adicionar `runtime.positions: HashMap<Location, Position>`
  como extensão.
- Layouter popula durante layout (não post-layout
  separada — não há ciclo de pages finalizadas separado
  do layout em cristalino).

Vantagem cristalina vs vanilla: pipeline single-pass
(integrado no layout), não separada.

### §5.3 Cobertura empírica

- 0 consumers em produção.
- 2 tests stub.
- 0 corpus failures.
- ADR-0066 estratégica adiamento M8.

**Conclusão estrutural**: P203 (Position concrete) tem
**baixa pressão empírica**. M8 (comemo) cobre o concern
naturalmente. P203 isolado é trabalho redundante.

---

## §6 Referências

- `01_core/src/entities/introspector.rs` (trait + impl;
  linhas 53, 55, 248).
- `01_core/src/rules/introspect.rs:714` (walk fn
  signature).
- `01_core/src/rules/layout/mod.rs:69, 84-85, 145`
  (Layouter struct).
- `lab/typst-original/crates/typst-library/src/introspection/position.rs`
  (vanilla DocumentPosition / PagedPosition).
- `lab/typst-original/crates/typst-library/src/introspection/introspector.rs:67-68, 141-143, 415-419`
  (vanilla Introspector trait + ElementIntrospector
  position).
- `lab/typst-original/crates/typst-layout/src/introspect.rs:35-63`
  (vanilla PagedIntrospector pipeline).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` (lacunas
  catalogadas canónicas).
- `00_nucleo/materialization/typst-passo-200-relatorio-consolidado.md` §7
  (lacunas operacionais P200).
- `00_nucleo/snapshot-2026-05-05.md` §7 (lacunas declaradas
  no snapshot reconciliado P202).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md` §2
  (origem do erro Position).
- `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (estratégia adiamento M8).
