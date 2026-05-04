# L0 — Motor de Introspecção (`rules/introspect.rs`)
Hash do Código: d25dfc47

## Módulo
`01_core/src/rules/introspect.rs`

**Histórico relevante**:
- 2026-04-30 (P161): renomeação `CounterState` → `CounterStateLegacy`.
- 2026-04-30 (P162): walk passa a aceitar `&mut Locator` + `&mut Vec<Tag>` + `Option<&Label>`; emite `Tag::Start`/`Tag::End` em paralelo a mutação de estado.
- 2026-04-30 (P163): este L0 refinado para reflectir P162.
- 2026-04-30 (P165): `from_tags` constrói `TagIntrospector` em paralelo; resultado descartado em M3.
- 2026-04-30 (P166 / M4): adicionado entry point `introspect_with_introspector`; `introspect()` passa a ser wrapper que descarta `TagIntrospector`.
- 2026-04-29 (P173 / M9 sub-passo 5): `introspect_with_introspector` aceita `Engine + EvalContext` opcionais; cascade habilita eval real de `StateUpdate::Func` em `from_tags`. Walk preservado puro.
- 2026-05-01 (P181H): walk arm `Content::Bibliography` restaurado a puro — não muta `state.bib_*` directamente (P163 invariante restaurada para bib). Tag emitida via `extract_payload` (P181D); `BibStore` populado por `from_tags` arm Bibliography (P181E). Walk arm legacy (P159C/F) reduzido a apenas descida em `title`.

## Propósito
Pré-passagem analítica sobre `Content`. Constrói o `CounterStateLegacy`
completo (incluindo `resolved_labels`) antes do layout físico arrancar.
Permite resolver referências para a frente (forward refs).

A partir de P162, em paralelo, produz uma sequência `Vec<Tag>` que captura
Heading/Figure/Cite com `Location` única (gerada por `Locator`) e
`ElementInfo` com payload + label opcional. As tags são **descartadas em
M1** — consumo real começa em M2/M3 quando `Introspector` for materializado.

## Regras de negócio

### O que a introspecção faz
- Percorre `Content` recursivamente via `walk()`.
- Avança contadores (`step_hierarchical`, `step_flat`) nos mesmos
  nós onde o Layouter o faria.
- Regista `resolved_labels` para cada `Labelled` encontrado.
- Intercede em `SetHeadingNumbering` e `CounterUpdate` para replicar
  os side-effects de estado.
- **P162**: emite `Tag::Start(Location, ElementInfo)` quando o nó é
  payload-yielder (Heading/Figure/Cite) e `Tag::End(Location, u128)`
  ao subir. Emparelhamento garantido por construção.

### O que a introspecção NÃO faz
- Não acede a `FontMetrics`.
- Não aloca `Frame`, `FrameItem`, ou `PagedDocument`.
- Não produz output visual de nenhum tipo.
- **P162**: não consome o `Vec<Tag>` produzido — descarta no fim de
  `introspect()`. M2/M3 começarão a consumir.

### Isolamento
A função pública `introspect(content: &Content) -> CounterStateLegacy`
é pura: dado o mesmo `Content`, retorna sempre o mesmo
`CounterStateLegacy`. Não tem estado global.

`Locator` e `Vec<Tag>` são internos a cada chamada — instanciados em
`introspect()`, propagados por `walk` recursivo, e descartados no fim.
Sem partilha entre chamadas.

### Integração com o layout físico
A função `layout(content)` executa automaticamente:
1. `introspect(content)` → obtém `resolved_labels` populado.
2. Inicia o Layouter com `resolved_labels` injectados.
3. Reconstrói `hierarchical`, `flat` e `numbering_active` nó a nó
   durante o layout — NÃO copia estes campos da introspecção, para que
   os prefixos visuais sejam gerados na ordem correcta.

## Assinaturas internas (P162)

```rust
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
);
```

`label_from_parent` carrega a label de um wrapper `Content::Labelled`
para o `target` recursivo (P162 .A decisão sobre mecanismo de label).
No arm `Content::Labelled`, walk recursa com `Some(label)`; nos outros
arms, recursa com `None`.

### Lógica de emissão de tags

No topo de `walk`:
1. Chamar `extract_payload(content)` (em `rules/introspect/extract_payload.rs`).
2. Se `Some(payload)`:
   - `let location = locator.next();`
   - `let info = ElementInfo { payload, label: label_from_parent.cloned() };`
   - `tags.push(Tag::Start(location, info));`
   - Guardar `location` para emissão do `End`.
3. Mutação de estado existente prossegue (não alterada).
4. Recursão para filhos (passando `state`, `locator`, `tags` por mutação;
   `label_from_parent` é `None` excepto no arm `Content::Labelled`).
5. Se emitiu `Start`, no fim de walk:
   - `tags.push(Tag::End(location, hash_content(content)));`

Pareamento Start↔End garantido: cada `Start(loc, _)` é seguido pelo seu
`End(loc, _)` correspondente após a recursão dos filhos. Bracketing
válido por construção (verificado por tests E2E em P163.C.2).

## Interface pública

Duas funções pub a partir de M4 (P166); estendidas em P173 com cascade
opcional de Engine + EvalContext:

```rust
/// Entry point legacy — wrapper que descarta TagIntrospector.
/// **P173**: Funcs em `state.update(key, fn)` são silenciosamente
/// ignoradas neste path (sem Engine).
pub fn introspect(content: &Content) -> CounterStateLegacy;

/// Entry point M4 / P166. **P173**: aceita `Engine + EvalContext`
/// opcionais; ambos `Some` habilitam eval real de `StateUpdate::Func`.
pub fn introspect_with_introspector(
    content: &Content,
    engine:  Option<&mut Engine<'_>>,
    ctx:     Option<&mut EvalContext>,
) -> (CounterStateLegacy, TagIntrospector);
```

Forma interna (P173):
```rust
pub fn introspect(content: &Content) -> CounterStateLegacy {
    let (state, _introspector) = introspect_with_introspector(content, None, None);
    state
}

pub fn introspect_with_introspector(
    content: &Content,
    engine:  Option<&mut Engine<'_>>,
    ctx:     Option<&mut EvalContext>,
) -> (CounterStateLegacy, TagIntrospector) {
    let mut state = CounterStateLegacy::new();
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    walk(content, &mut state, &mut locator, &mut tags, None);
    let introspector = self::from_tags::from_tags(&tags, engine, ctx);
    (state, introspector)
}
```

**Walk único**: state + introspector vêm da mesma passagem — não há
duplicação. **Walk continua puro** — Engine só intervém em
`from_tags::from_tags` para eval de `StateUpdate::Func`. P163 invariante
preservado.

`introspect()` é wrapper legacy preservado: passa `None, None`.
Comportamento defensivo documentado: Funcs em modo legacy ficam
ignoradas silenciosamente.

Padrão de migração M5+: caller que actualmente faz `let state =
introspect(&c)` e quer queries via Introspector adopta
`let (state, intr) = introspect_with_introspector(&c)` sem custo
adicional. M6 eliminará o wrapper + `CounterStateLegacy` quando todos
os consumers tiverem migrado.

Helper de teste em `#[cfg(test)]`:
```rust
fn introspect_with_tags(content: &Content) -> (CounterStateLegacy, Vec<Tag>);
```
Disponível só em testes para verificar a captura de tags em paralelo
(P162.G + P163.C/.D). API pública não muda.

## Sobre paridade

Vanilla não tem walk explícito sobre `Content`. Usa `comemo` + `convergence::analyze` para fixpoint multi-iteração com type-erased `Introspect` ops. Cristalino diverge: walk single-pass directo + tags em paralelo (P162). Quando M2/M3 introduzirem consumo real, a divergência pode estreitar mas não inverter — cristalino não vai usar `comemo` para introspecção runtime (decisão herdada de ADR-0066 PROPOSTO).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) para o mapa completo de tipos vanilla e quais cristalino materializa.

## Critérios de verificação
- `Labelled` após `Heading` → `resolved_labels` contém a chave.
- `Labelled` antes de `Heading` (forward ref) → `resolved_labels` contém
  a chave (porque `walk` percorre o `target` antes de registar).
- `CounterUpdate { action: Update(5) }` → `flat["equation"] == 5`.
- `SetHeadingNumbering { active: true }` → `is_numbering_active("heading") == true`.
- Dois documentos independentes → estados independentes (sem partilha).
- `layout(content)` com forward ref → texto resolvido (não `@nome`).
- **P162/P163**:
  - Walk sobre `Content::Heading` produz `Tag::Start` + `Tag::End`
    com `Location` igual.
  - Walk sobre `Content::Text` não produz tags.
  - Walk sobre `Content::Labelled { target: Heading, label }` emite
    `Tag::Start` para Heading com `info.label = Some(label)`.
  - Walk duas vezes sobre o mesmo Content produz `Vec<Tag>` idêntico
    (determinismo).
  - Bracketing válido em qualquer aninhamento (todo Start tem o seu End,
    sem overlapping).
  - Hash em `Tag::End` distingue Contents diferentes.
  - Número de `ElementPayload::Heading` em tags == número de headings no
    input.

## Secção: Walk puro M5 incremental (P189B)

P189B materializa **a primeira peça** de M5 walk puro:

- **Outline arm migrado** (`introspect.rs:Content::Outline`):
  mutação `state.has_outline = true` removida. Flag obtida via
  `intr.kind_index.contains_key(&ElementKind::Outline)` no
  consumer (`mod.rs:layout_with_introspector`). Arm continua a
  emitir `Tag::Start` no topo de `walk` fn — apenas a mutação
  directa em state foi removida.

**M5 universal não fecha em P189**. Análise empírica em P189A
revelou cadeia de dependências
(`Heading→Labelled→resolved_labels`) que bloqueia migração
universal sem pré-requisitos. P189 fecha 1 arm migrável e
declara 6 excepções honestamente.

### Excepções M5

Walk arms que **continuam a mutar state directamente**, com
justificação literal e plano de fechamento:

| # | Arm | Mutação | Razão | Pré-requisito |
|---|-----|---------|-------|---------------|
| **E1** | `Content::Equation` | `state.step_flat("equation")` | `Content::SetEquationNumbering` ausente (Reserva 1; P186A §11.2). Sem ele, gate em `from_tags` P186E nunca dispara → counter introspector vazio → P188B fallback legacy é caminho funcional permanente. | Materializar `Content::SetEquationNumbering` (passo dedicado). |
| **E2-residuo** | `Content::Heading` | `state.headings_for_toc.push` (1 mutação residual após P196B) | P196B fechou 3 das 4 mutações estruturalmente via Tag::Labelled auto-toc (pattern ADR-0069). Resta `headings_for_toc.push` porque sub-store `intr.headings_for_toc` **não existe** (lacuna #3). Outras 3 mutações (`step_hierarchical`, `auto_label_counter++`, `resolved_labels.insert`) preservadas como **write paralelo** durante janela compat M5 — fecham orgânicamente em M6. | Sub-store `intr.headings_for_toc` (passo dedicado fora série P196). |
| **E3** | `Content::Figure` | `state.local_figure_counters`, `state.figure_numbers` | **Fechou estruturalmente em P197B** (cenário α — caminho Introspector activo desde P184: variant `ElementPayload::Figure` + `from_tags` arm popula `CounterRegistry` chave `figure:{kind}` + `figure_label_numbers`; consumer C3 P184D usa `figure_number_at_index`). Mutação legacy preservada como write paralelo M5 porque `compute_labelled` P195D Figure arm lê `state.figure_numbers.last()`. Pattern ADR-0069 (Tag pós-recursão) **dispensado** — extracção de helper `compute_figure` é refactor estilístico. Cleanup orgânico em M6. | Funcional fecha em M6. |
| **E4** | `Content::Labelled` | `state.figure_label_numbers`, `state.resolved_labels` | **Fechou estruturalmente em P195D** (caminho Introspector activo via Tag::Labelled). Mutação legacy mantida como write paralelo M5; remoção orgânica em M6. | Funcional fecha em M6. |
| **E5** | `Content::SetHeadingNumbering` | `state.numbering_active.insert("heading", ...)` | **Fechou estruturalmente em P198B** (cenário α — caminho Introspector activo desde P182C: `extract_payload` retorna `Some(StateUpdate { key: "numbering_active:heading" })`; `from_tags` arm StateUpdate popula `StateRegistry`; consumer C5 via `is_numbering_active`). Mutação legacy preservada como write paralelo M5 porque `compute_heading_auto_toc` P196B + walk arm Equation lêem `state.numbering_active` durante walk. Pattern ADR-0069 (Tag pós-recursão) **dispensado** — sem helper extraído (mutação trivial 1 linha). Cleanup orgânico em M6. | Funcional fecha em M6. |
| **E6** | `Content::CounterUpdate` | `state.step_*`, `update_flat` | **Fechou estruturalmente em P198C** (cenário β-promote — primeira aplicação): variant `ElementPayload::CounterUpdate { key, action }` adicionada; `is_locatable(CounterUpdate) = true`; `extract_payload` arm emite payload pré-recursão; `from_tags` arm popula `CounterRegistry` via `apply_at` (flat) ou `apply_hierarchical_at` (key="heading"); `kind_index[ElementKind::CounterUpdate]` populated. Mutação legacy preservada como write paralelo M5 porque `compute_*` helpers (P195D Equation, P196B Heading, P197B Figure) lêem `state.flat`/`hierarchical` durante walk. Cleanup orgânico em M6. | Funcional fecha em M6. |

**Padrão de cadeia**: 5 das 6 excepções (E2/E3/E4/E5/E6) fecham
em sequência após desbloquear sub-store `resolved_labels` + C4
migration. E1 é independente (Reserva 1 distinta).

**Estado P198C (2026-05-04)**: E4 fechou estruturalmente em
P195D; E2 fechou estruturalmente em P196B (3 das 4 mutações
migradas via Tag pattern ADR-0069); E3 fechou estruturalmente
em P197B (cenário α — caminho Introspector activo desde P184);
E5 fechou estruturalmente em P198B (cenário α — caminho
Introspector activo desde P182C); **E6 fechou estruturalmente
em P198C (cenário β-promote — primeira aplicação: variant
nova + promote locatable + 2 arms novos)**. Restam apenas:
- **E2-residuo** (`headings_for_toc.push`) bloqueado por lacuna #3.
- **E1** (Equation) bloqueada por `Content::SetEquationNumbering` ausente.

**Ordem inversa à mutação**: para fechar M5 universalmente,
migração tem que acontecer da camada mais baixa (sub-stores)
para a mais alta (Layouter consumers). Concretamente:

1. ✅ Abrir sub-store `resolved_labels` (P193B).
2. ✅ Migrar consumer Ref-arm em Layouter para ler do sub-store
   (C4 migration P194B com fallback substitution).
3. ✅ Migrar walk arm `Labelled` para emitir Tag em vez de mutar
   directamente (P195D — E4 fecha estruturalmente).
4. ✅ Migrar walk arm `Heading` (P196B — E2 fecha estruturalmente,
   resta E2-residuo).
5. Abrir sub-store `intr.headings_for_toc` (passo dedicado;
   fecha E2-residuo).
6. ✅ Migrar walk arm `Figure` (P197B — E3 fecha estruturalmente
   via cenário α; caminho Introspector já activo desde P184).
7. ✅ Migrar walk arm `SetHeadingNumbering` (P198B — E5 fecha
   estruturalmente via cenário α; caminho Introspector já
   activo desde P182C).
8. ✅ Migrar walk arm `CounterUpdate` (P198C — E6 fecha
   estruturalmente via cenário β-promote: variant nova +
   promote locatable + extract_payload arm + from_tags arm).
9. Quando `Content::SetEquationNumbering` materializar, E1 fecha.

Após esses passos sequenciais, walk torna-se universalmente
puro. Segue M6 (eliminação `CounterStateLegacy`).

**DEBT M5-residual**: cobre E1–E6 (Cenário B per P189A §8 —
sem DEBT formal aberto; apenas notas preventivas).

## Secção: Walk arm Labelled migrado (P195D, ADR-0069)

**Pattern arquitectural novo: post-recursion tag emission for
state-dependent payload** (ADR-0069 PROPOSTO).

Walk arm `Content::Labelled` em `introspect.rs:Content::Labelled`
foi modificado em P195D para:

1. **Recursão primeiro**: `walk(target, state, locator,
   tags, Some(label))` — propaga label via
   `label_from_parent`; muta counters do target via walks
   recursivos.

2. **Computar payload via helper privado**: `compute_labelled(target,
   state)` retorna `(Option<String>, Option<usize>)` —
   função pura que replica lógica legacy:
   - Heading → `"Secção {n}"`.
   - Equation block → `"Equação ({n})"`.
   - Figure numbering+captioned → `"{supplement} {n}"` +
     `figure_number = Some(n)`.
   - Figure sem numbering ou caption → `Some("")`.
   - Outros → `(None, None)`.

3. **Mutação legacy preservada** (write paralelo durante
   janela compat M5):
   - `state.figure_label_numbers.insert(label, n)` se
     `figure_number.is_some()`.
   - `state.resolved_labels.insert(label, text)` se
     `resolved_text.is_some()`.

4. **Tag pós-recursão emitida** (pattern ADR-0069):
   - Snapshot `tags.len()` antes da recursão; após
     recursão, find_map para a primeira `Tag::Start` no
     range novo → reuso da Location do target.
   - `tags.push(Tag::Start(loc, ElementInfo::new(
     ElementPayload::Labelled { label, resolved_text,
     figure_number })))` + `tags.push(Tag::End(loc, 0))`.
   - **Reuso de Location** preserva sincronização-por-construção
     ADR-0068 (walk Locator e Layouter Locator não avançam
     para Labelled em nenhum dos lados).
   - Caso target não-locatable (sem Tag::Start no range):
     Tag não emitida; sub-store via Tag não populated;
     mutação legacy preservada.

### Estado após P195D

- **E4 fecha estruturalmente** — caminho Introspector
  activa para explicit labels.
- **E4 funcionalmente fecha em M6** quando mutação legacy
  for removida.
- **E2 (Heading auto-toc)** continua activa — só fecha em
  P196.
- Output observable em produção **inalterado** —
  mutação legacy fornece valores idênticos via write
  paralelo; consumer C4 P194B recebe `Some(text)` do
  Introspector path mas fallback legacy continua funcional.

### Helper `compute_labelled`

Função privada em `introspect.rs` (sem `pub`) — uso
interno apenas. Isola lógica de computação para reuso
entre mutação legacy e populate Tag. Reduz duplicação.

Replica literal da lógica match legacy do walk arm; sem
mutação (state ref imutável).

## Secção: Walk arm Heading migrado (P196B, ADR-0069)

**Segunda aplicação do pattern post-recursion tag emission**
(ADR-0069 ACEITE em P195E). Análoga a P195D mas mais simples:
Heading é locatable, então `emitted_loc` (do walk top) já
está disponível na arm — sem necessidade de snapshot+find_map.

Walk arm `Content::Heading` em `introspect.rs:Content::Heading`
foi modificado em P196B para:

1. **Mutação legacy preservada** (write paralelo M5 → M6):
   - `state.step_hierarchical("heading", level)`.
   - `state.auto_label_counter += 1`.
   - `state.resolved_labels.insert(auto_label, resolved_text)`.
   - `state.headings_for_toc.push((auto_label, frozen_body, level))` —
     **E2-residuo** porque sub-store `intr.headings_for_toc`
     não existe (lacuna #3).

2. **Computar payload via helper privado**:
   `compute_heading_auto_toc(state, auto_label_counter)` retorna
   `(Label, String)` — função pura sobre referência imutável de
   state. Replica lógica legacy:
   - `Label("auto-toc-{n}")` sempre.
   - `resolved_text = "Secção {prefix}"` se
     `is_numbering_active("heading") && format_hierarchical("heading").is_some()`.
   - `resolved_text = ""` (string vazia) caso contrário —
     paridade legacy preserva insert mesmo quando
     numbering inactivo.

3. **Recursão no body**: `walk(body, state, locator, tags, None)` —
   propaga state mutado para filhos do heading.

4. **Tag pós-recursão emitida** (pattern ADR-0069):
   - `if let Some(loc) = emitted_loc { … }` — Heading é
     locatable, walk top emitiu Tag::Start no topo, então
     `loc` está disponível directamente (sem snapshot).
   - `tags.push(Tag::Start(loc, ElementInfo::new(
     ElementPayload::Labelled { label: auto_label,
     resolved_text: Some(resolved_text), figure_number: None })))`
     + `tags.push(Tag::End(loc, 0))`.
   - **Reuso de Location** preserva sincronização-por-construção
     ADR-0068 (4 tags do Heading partilham mesma Location).

### Estado após P196B

- **E2 fecha estruturalmente** (3 de 4 mutações migradas via
  Tag::Labelled auto-toc). Caminho Introspector activo para
  auto-toc labels Heading.
- **E2-residuo** persiste com 1 mutação (`headings_for_toc.push`)
  porque sub-store ausente (lacuna #3). Fecha em passo
  dedicado fora série P196.
- **E2 funcionalmente fecha em M6** quando mutação legacy
  for removida.
- Output observable em produção **inalterado** —
  mutação legacy fornece valores idênticos via write
  paralelo; consumer C4 P194B recebe `Some(text)` do
  Introspector path com auto_label sintetizada.

### Helper `compute_heading_auto_toc`

Função privada em `introspect.rs` (sem `pub`) — uso
interno apenas. Análoga a `compute_labelled` (P195D).
Sempre retorna concrete `(Label, String)` em vez de
`Option`s — paridade legacy preserva insert de
`auto_label → ""` quando numbering inactivo (insert
informativo de presença, não de conteúdo).

### Sequência de tags emitida

Para `Content::Heading { level: 1, body: text("título") }`:

```
Tag::Start(loc, Heading)               // walk top
Tag::Start(loc, Labelled auto-toc-1)   // arm pós-recursão
Tag::End(loc, 0)                       // arm pós-recursão (hash=0)
Tag::End(loc, hash_content(heading))   // walk bottom
```

4 tags com mesma Location, 2 pares Start/End válidos.
Bracketing preservado.

Para `Content::Heading { level: 1, body: figure }`:

```
Tag::Start(loc_h, Heading)
Tag::Start(loc_f, Figure)
Tag::End(loc_f, hash_figure)
Tag::Start(loc_h, Labelled auto-toc-1)
Tag::End(loc_h, 0)
Tag::End(loc_h, hash_heading)
```

6 tags. Bracketing válido (Heading bracket envolve Figure
bracket; auto-toc é par próprio inserido entre fim de
recursão e End externo).

## Secção: Walk arm Figure migrado (P197B, cenário α)

**Refactor estilístico — pattern ADR-0069 (Tag pós-recursão)
dispensado**. Diferente de P195D/P196B: walk arm Figure não
emite Tag pós-recursão porque o caminho Introspector para
figure numbering já está activo em produção desde P184.

Walk arm `Content::Figure` em `introspect.rs:Content::Figure`
foi modificado em P197B para:

1. **Helper privado `compute_figure` extraído**:
   `fn compute_figure(state: &CounterStateLegacy, kind: &Option<String>, is_counted: bool) -> Option<usize>`.
   Análogo a `compute_labelled` (P195D) e
   `compute_heading_auto_toc` (P196B). Função pura sobre
   `(state, kind, is_counted)` — sem mutação. Projecta o
   próximo número de Figure (1-based) para o kind indicado.
   `None` quando `is_counted = false`.

2. **Walk arm chama helper**:
   ```rust
   let is_counted = numbering.is_some() && caption.is_some();
   if let Some(figure_number) = compute_figure(state, kind, is_counted) {
       // mutação legacy preservada (write paralelo M5)
       …
   }
   ```

3. **Mutação legacy preservada** (write paralelo M5 →
   cleanup orgânico em M6):
   - `state.local_figure_counters.entry(kind_key).or_insert(0); *counter += 1;`
   - `state.figure_numbers.entry(kind_key).or_default().push(figure_number);`
   Necessárias porque `compute_labelled` P195D Figure arm
   lê `state.figure_numbers.last()` durante walk para
   popular `figure_label_numbers` quando target Figure
   é wrapped em `Labelled`. Cadeia E2-E3 preservada.

4. **Tag pós-recursão dispensada**: walk top já emite
   `Tag::Start(loc, ElementInfo { payload: Figure { kind,
   counter_update, is_counted }, label: label_from_parent })`
   antes de entrar no match arm via locatable +
   `extract_payload`. `from_tags` arm Figure (introduzido em
   P184B) popula 4 sub-stores em produção:
   - `kind_index[Figure]` ← locations.
   - `counters` ← `apply_at("figure:{kind}", Step, loc)`.
   - `counters` ← `apply_at("figure", Step, loc)` global (write paralelo M6).
   - `figure_label_numbers` ← quando `is_counted && label_from_parent.is_some()`.

### Estado após P197B

- **E3 fecha estruturalmente** — caminho Introspector activo
  desde P184; consumer C3 (`mod.rs:484`, P184D) já usa
  `figure_number_at_index` via Introspector path.
- **E3 funcionalmente fecha em M6** quando mutação legacy
  for removida (após `compute_labelled` Figure arm migrar
  para CounterRegistry).
- Output observable em produção **inalterado** —
  refactor estilístico puro; helper extraído replica lógica
  legacy bit-for-bit.

### Helper `compute_figure`

Função privada em `introspect.rs` (sem `pub`) — uso
interno apenas. Análoga a `compute_labelled` (P195D) e
`compute_heading_auto_toc` (P196B) — terceiro helper na
família. Não há reuso entre helpers (cada um cobre lógica
distinta) — apenas paralelismo de shape.

### Cross-references

- **P184B** — variant `ElementPayload::Figure` materializada;
  `from_tags` arm Figure popula CounterRegistry chave
  `figure:{kind}`.
- **P184C** — método trait `figure_number_at_index`.
- **P184D** — consumer C3 (Layouter) substitution-with-fallback.
- **P168** — `figure_label_numbers` populated em from_tags
  arm Figure quando is_counted+label.
- **P195D** — `compute_labelled` Figure arm lê
  `state.figure_numbers.last()` (cadeia E2-E3; mutação
  legacy preservada por isso).
- **ADR-0069** — pattern stylesheet de helper privado
  reaplicado; Tag pós-recursão dispensada per cenário α.

## Secção: Walk arm SetHeadingNumbering migrado (P198B, cenário α)

**Declaração formal — sem refactor de código produção**.
Diferente de P195D/P196B/P197B: walk arm SetHeadingNumbering
não tem helper extraído porque mutação é trivial (1 linha).
Cenário α aceita ambas formas (com ou sem helper).

Walk arm `Content::SetHeadingNumbering` em
`introspect.rs:Content::SetHeadingNumbering` foi marcado em
P198B como:

1. **Caminho Introspector activo desde P182C**:
   - `is_locatable(SetHeadingNumbering) = true` (P182C
     promoção, locatable.rs:49).
   - `extract_payload(SetHeadingNumbering)` retorna
     `Some(ElementPayload::StateUpdate { key:
     "numbering_active:heading", update: Set(Bool(active)) })`
     (extract_payload.rs:63-66).
   - Walk top emite Tag::Start pré-recursão (sem body para
     recursão — leaf content).
   - `from_tags` arm StateUpdate (P171/P173) popula
     `intr.state` (StateRegistry) com chave canónica
     `numbering_active:heading`.

2. **Mutação legacy preservada** (write paralelo M5 →
   cleanup orgânico em M6):
   - `state.numbering_active.insert("heading", *active);`
   Necessária porque:
   - `compute_heading_auto_toc` (P196B helper, introspect.rs:384)
     lê `state.is_numbering_active("heading")` durante walk
     para resolver auto-toc text.
   - Walk arm `Content::Equation` (introspect.rs:517) lê
     `state.is_numbering_active("equation")` para gate
     do counter step.
   Cadeia E5 preservada.

3. **Tag pós-recursão dispensada**: walk top já emite
   `Tag::Start(loc, ElementInfo { payload: StateUpdate {...},
   label: None })`. Sem helper extraído porque mutação é
   trivial — extracção não acrescenta valor ao stylesheet.

### Estado após P198B

- **E5 fecha estruturalmente** — caminho Introspector
  activo desde P182C; consumer C5 (`is_numbering_active` via
  StateRegistry) já recebe `Some(true/false)` via
  Introspector path.
- **E5 funcionalmente fecha em M6** quando mutação legacy
  for removida (após `compute_heading_auto_toc` + Equation
  walk arm migrarem para StateRegistry location-aware via
  `is_numbering_active_at`).
- Output observable em produção **inalterado** —
  declaração formal sem código modificado.

### Cross-references

- **P171** — `ElementPayload::StateUpdate` variant
  materializada para Content::State / StateUpdate user-space.
- **P173** — Engine + EvalContext cascade para Funcs em
  `from_tags` arm StateUpdate.
- **P182C** — promoção SetHeadingNumbering a locatable;
  arm em `extract_payload` reusa `StateUpdate` sob chave
  canónica `numbering_active:heading`.
- **P185B** — método trait `is_numbering_active_at` para
  consumer C5 location-aware.
- **P196B** — `compute_heading_auto_toc` consumer da
  mutação legacy (cadeia E5).
- **P197B** — primeira aplicação cenário α (Figure);
  P198B é segunda aplicação.
- **ADR-0069** — pattern stylesheet; Tag pós-recursão
  dispensada per cenário α.

## Secção: Walk arm CounterUpdate migrado (P198C, cenário β-promote)

**Primeira aplicação do cenário β-promote** — promote
`Content::CounterUpdate` a locatable + variant nova
`ElementPayload::CounterUpdate` + 2 arms novos. Distinção
de cenário α (P197B/P198B): caminho Introspector NÃO estava
activo pré-P198C; precisou de promotion concreta.

Trabalho concreto em P198C:

1. **Variant nova `ElementPayload::CounterUpdate { key, action }`**
   adicionada a `entities/element_payload.rs` (12ª variant).
   Field `action: CounterUpdate` reusa enum existente
   (counter_update.rs P161 rename de CounterAction).

2. **`ElementKind::CounterUpdate`** adicionada a
   `entities/element_kind.rs` (10ª variant). Convenção
   cristalino — todo locatable tem ElementKind correspondente.

3. **`is_locatable(Content::CounterUpdate) = true`** activada
   em `rules/introspect/locatable.rs`. Movida da lista
   non-locatable.

4. **`extract_payload` arm** adicionada a
   `rules/introspect/extract_payload.rs`:
   ```rust
   Content::CounterUpdate { key, action } => Some(ElementPayload::CounterUpdate {
       key:    key.clone(),
       action: action.clone(),
   }),
   ```

5. **`from_tags` arm** adicionada a `rules/introspect/from_tags.rs`:
   - 3 caminhos sob `match action`:
     - `Step + key="heading"` → `intr.counters.apply_hierarchical_at(key, 1, loc)`.
     - `Step + key!="heading"` → `intr.counters.apply_at(key, Step, loc)`.
     - `Update(val)` → `intr.counters.apply_at(key, Update(val), loc)`.
   - Indexa em `kind_index[ElementKind::CounterUpdate]`.

6. **Walk arm preservado** — 3 caminhos legacy mantidos
   (`state.step_hierarchical`, `state.step_flat`,
   `state.update_flat`) como write paralelo M5. Comentário
   inline P198C declara cenário β-promote.

### Estado após P198C

- **E6 fecha estruturalmente** — caminho Introspector activo
  para CounterUpdate; CounterRegistry populated
  paralelamente a state legacy.
- **E6 funcionalmente fecha em M6** quando `compute_*` helpers
  migrarem para CounterRegistry location-aware (`flat_counter_at`
  ou similar) e mutação legacy puder ser removida.
- Output observable em produção **inalterado** — write paralelo
  fornece valores idênticos via legacy + Introspector.

### Distinção cenário α vs β-promote

| Aspecto | Cenário α (P197B, P198B) | Cenário β-promote (P198C) |
|---------|--------------------------|---------------------------|
| Caminho Introspector pré-passo | Activo | Inactivo |
| Variant ElementPayload | Reuso existente | Nova (12ª) |
| ElementKind | Reuso existente | Nova (10ª) |
| `is_locatable` | Já true | False → true |
| `extract_payload` arm | Já existe | Adicionar |
| `from_tags` arm | Já existe | Adicionar |
| Magnitude | S | M |

### Cross-references

- **P184B** — `CounterRegistry` sub-store (consumido por P198C from_tags arm).
- **P186C** — precedente promotion de Equation (mas com pré-requisito Reserva 1).
- **P195D / P196B / P197B** — `compute_*` helpers consumidores da mutação legacy.
- **ADR-0069** — pattern stylesheet; cenário β-promote primeira aplicação concreta.
