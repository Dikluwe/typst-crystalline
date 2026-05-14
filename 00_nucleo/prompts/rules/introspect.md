# L0 — Motor de Introspecção (`rules/introspect.rs`)
Hash do Código: 174b4808

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
| **E1** | `Content::Equation` | `state.step_flat("equation")` | **Fechou estruturalmente em P199B** (cenário α por construção): `Content::SetEquationNumbering { active: bool }` materializada (Reserva 1 desde P189B); `extract_payload` emite `StateUpdate { key: "numbering_active:equation" }`; `from_tags` arm StateUpdate genérica (P171) popula StateRegistry; gate em `from_tags` P186E activa em produção real; consumer Layouter `equation.rs:32-33` (substitution-with-fallback antes adormecida) activa first branch. Mutação legacy preservada como write paralelo M5 porque walk arm Equation + `compute_labelled` Equation arm (P195D) lêem `state.is_numbering_active("equation")` durante walk. Cleanup orgânico em M6. | Funcional fecha em M6. |
| **E2-residuo** | `Content::Heading` | `state.headings_for_toc.push` (1 mutação residual após P196B) | **Fechou estruturalmente em P200B** (trabalho híbrido — sub-store novo `intr.headings_for_toc` aberto + variant `ElementPayload::HeadingForToc` + walk arm emite 3ª Tag pós-recursão + consumer `outline.rs:24` substitution-with-fallback). Lacuna #3 fecha. Mutação 4 legacy preservada como write paralelo M5 porque Layouter assignments (`mod.rs:1490, 1521`) dependem; cleanup orgânico em M6. | Funcional fecha em M6. |
| **E3** | `Content::Figure` | `state.local_figure_counters`, `state.figure_numbers` | **Fechou estruturalmente em P197B** (cenário α — caminho Introspector activo desde P184: variant `ElementPayload::Figure` + `from_tags` arm popula `CounterRegistry` chave `figure:{kind}` + `figure_label_numbers`; consumer C3 P184D usa `figure_number_at_index`). Mutação legacy preservada como write paralelo M5 porque `compute_labelled` P195D Figure arm lê `state.figure_numbers.last()`. Pattern ADR-0069 (Tag pós-recursão) **dispensado** — extracção de helper `compute_figure` é refactor estilístico. Cleanup orgânico em M6. | Funcional fecha em M6. |
| **E4** | `Content::Labelled` | `state.figure_label_numbers`, `state.resolved_labels` | **Fechou estruturalmente em P195D** (caminho Introspector activo via Tag::Labelled). Mutação legacy mantida como write paralelo M5; remoção orgânica em M6. | Funcional fecha em M6. |
| **E5** | `Content::SetHeadingNumbering` | `state.numbering_active.insert("heading", ...)` | **Fechou estruturalmente em P198B** (cenário α — caminho Introspector activo desde P182C: `extract_payload` retorna `Some(StateUpdate { key: "numbering_active:heading" })`; `from_tags` arm StateUpdate popula `StateRegistry`; consumer C5 via `is_numbering_active`). Mutação legacy preservada como write paralelo M5 porque `compute_heading_auto_toc` P196B + walk arm Equation lêem `state.numbering_active` durante walk. Pattern ADR-0069 (Tag pós-recursão) **dispensado** — sem helper extraído (mutação trivial 1 linha). Cleanup orgânico em M6. | Funcional fecha em M6. |
| **E6** | `Content::CounterUpdate` | `state.step_*`, `update_flat` | **Fechou estruturalmente em P198C** (cenário β-promote — primeira aplicação): variant `ElementPayload::CounterUpdate { key, action }` adicionada; `is_locatable(CounterUpdate) = true`; `extract_payload` arm emite payload pré-recursão; `from_tags` arm popula `CounterRegistry` via `apply_at` (flat) ou `apply_hierarchical_at` (key="heading"); `kind_index[ElementKind::CounterUpdate]` populated. Mutação legacy preservada como write paralelo M5 porque `compute_*` helpers (P195D Equation, P196B Heading, P197B Figure) lêem `state.flat`/`hierarchical` durante walk. Cleanup orgânico em M6. | Funcional fecha em M6. |

**Padrão de cadeia**: 5 das 6 excepções (E2/E3/E4/E5/E6) fecham
em sequência após desbloquear sub-store `resolved_labels` + C4
migration. E1 é independente (Reserva 1 distinta).

**Estado P200B (2026-05-04)**: **M5 universal completo pela
primeira vez desde P189B**. Todas excepções fechadas
estruturalmente:
- E4 fechou estruturalmente em P195D.
- E2 fechou parcialmente em P196B (3 das 4 mutações; resíduo
  declarado).
- E3 fechou estruturalmente em P197B (cenário α — caminho
  Introspector activo desde P184).
- E5 fechou estruturalmente em P198B (cenário α — caminho
  Introspector activo desde P182C).
- E6 fechou estruturalmente em P198C (cenário β-promote — 1ª
  aplicação).
- E1 fechou estruturalmente em P199B (cenário α por
  construção — materialização de `Content::SetEquationNumbering`).
- **E2-residuo fechou estruturalmente em P200B** (trabalho
  híbrido — sub-store novo + variant + walk arm + consumer
  migration). Lacuna #3 fecha.

**0 excepções activas + 0 residuos + 0 pré-requisitos
restantes**. Desbloqueia M6 (P190A reescrita do zero —
eliminação `CounterStateLegacy`; magnitude L cross-modular).

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
5. ✅ Abrir sub-store `intr.headings_for_toc` (P200B — fecha
   E2-residuo + lacuna #3 via trabalho híbrido).
6. ✅ Migrar walk arm `Figure` (P197B — E3 fecha estruturalmente
   via cenário α; caminho Introspector já activo desde P184).
7. ✅ Migrar walk arm `SetHeadingNumbering` (P198B — E5 fecha
   estruturalmente via cenário α; caminho Introspector já
   activo desde P182C).
8. ✅ Migrar walk arm `CounterUpdate` (P198C — E6 fecha
   estruturalmente via cenário β-promote: variant nova +
   promote locatable + extract_payload arm + from_tags arm).
9. ✅ Materializar `Content::SetEquationNumbering` (P199B —
   E1 fecha estruturalmente via cenário α por construção:
   variant nova + 3 arms; reusa `from_tags::StateUpdate`
   genérica P171; activa Layouter equation.rs:32-33
   substitution-with-fallback antes adormecida).

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

## Secção: Variant `Content::SetEquationNumbering` materializada (P199B, cenário α por construção)

**Cenário α por construção — sub-variante de cenário α**.
Distinção das 4 variantes operacionais ADR-0069 conhecidas:

| Variante | Pré-passo | Trabalho |
|----------|-----------|----------|
| P195D (não-locatable) | Caminho inactivo | Tag pós-recursão + snapshot+find_map |
| P196B (locatable + body) | Caminho inactivo | Tag pós-recursão + emitted_loc directo |
| Cenário α (P197B, P198B) | Caminho activo | Refactor estilístico ou declaração formal |
| **Cenário α por construção (P199B)** | **Caminho activável** | **Materializar variant — caminho activa imediatamente** |
| Cenário β-promote (P198C) | Caminho inactivo | Promote completo (variant + locatable + 2 arms) |

**Cenário α por construção** distingue-se de cenário α padrão:
- P197B/P198B: variant **já existia**; cenário α era declaração formal/refactor.
- **P199B**: variant **não existia**; cenário α por construção é declaração formal **no momento da materialização** da variant — toda a infraestrutura downstream pronta a activar (em P199B esta infra inclui Layouter `equation.rs:32-33` substitution-with-fallback antes adormecida).

Trabalho concreto em P199B:

1. **Variant nova `Content::SetEquationNumbering { active: bool }`**
   adicionada a `entities/content.rs` análoga a `SetHeadingNumbering`.
   Comentário documenta DEBT-10 (StyleChain futuro) e referência P57 + P199B.

2. **Match arms exaustivos induzidos cobertos**:
   - `Content::plain_text` (content.rs:1040+).
   - `Content::eq` / comparação (content.rs:1200+).
   - 2 listas de "terminais sem effect em counters"
     (content.rs:1483, 1694).
   - `materialize_time` em introspect.rs (lista de terminais
     linha 165+).

3. **`is_locatable(Content::SetEquationNumbering) = true`**
   activado em `rules/introspect/locatable.rs`.

4. **`extract_payload` arm** adicionada a
   `rules/introspect/extract_payload.rs`:
   ```rust
   Content::SetEquationNumbering { active } => Some(ElementPayload::StateUpdate {
       key:    "numbering_active:equation".to_string(),
       update: StateUpdate::Set(Box::new(Value::Bool(*active))),
   }),
   ```
   Reusa `ElementPayload::StateUpdate` (P171) sob chave canónica
   `numbering_active:equation`.

5. **Walk arm** adicionada a `rules/introspect.rs`:
   ```rust
   Content::SetEquationNumbering { active } => {
       state.numbering_active.insert("equation".to_string(), *active);
   }
   ```
   Comentário inline P199B declara cenário α por construção.

6. **Layouter consumer** adicionado a `rules/layout/mod.rs`:
   ```rust
   Content::SetEquationNumbering { active } => {
       counters::layout_set_equation_numbering(&mut self.counter, *active);
   }
   ```
   Helper `layout_set_equation_numbering` em
   `rules/layout/counters.rs` (paralelo a
   `layout_set_heading_numbering`).

7. **`from_tags` arm StateUpdate NÃO modificado** — genérica
   (P171) processa `numbering_active:equation` transparentemente.

### Estado após P199B

- **E1 fecha estruturalmente** via cenário α por construção.
- **Reserva 1 (P189B) materializada** — última pendência da
  série §9 P189 que não era pré-requisito paralelo.
- Caminho Introspector activado em produção real:
  StateRegistry populated com `numbering_active:equation`;
  CounterRegistry para `equation` activa via gate em
  `from_tags::Equation` (P186E); Layouter `equation.rs:32`
  first branch retorna Some.
- Mutação legacy preservada como write paralelo M5 — walk arm
  Equation (gate) + `compute_labelled` Equation arm (P195D)
  continuam a ler `state.numbering_active("equation")` /
  `state.get_flat("equation")` durante walk.

### Cross-references

- **P57** — template original SetHeadingNumbering.
- **P171** — `ElementPayload::StateUpdate` variant + `from_tags` arm StateUpdate genérica.
- **P173** — Engine + EvalContext cascade.
- **P182C** — promoção SetHeadingNumbering a locatable (template directo replicado em P199B).
- **P186C/D/E** — Equation locatable + gate em from_tags arm.
- **P195D** — `compute_labelled` Equation arm consumer da mutação legacy.
- **P198B** — primeira aplicação cenário α (SetHeadingNumbering); P199B é segunda aplicação distinguida como **cenário α por construção** (variant nova).
- **DEBT-10** — substituir StateUpdate por StyleChain quando motor de introspecção completo for implementado (futuro M6+).
- **ADR-0069** — pattern stylesheet; cenário α por construção é sub-variante operacional do cenário α padrão.

## Secção: Walk arm Heading mutação 4 fechada (P200B, trabalho híbrido)

**Marco arquitectural — M5 universal completo pela primeira
vez desde declaração em P189B**. P200B fecha **E2-residuo**
e **lacuna #3** simultaneamente via trabalho **híbrido**
combinando 3 padrões testados — sem nova variante operacional
ADR-0069.

### 3 categorias de trabalho combinadas

| Categoria | Padrão precedente | Trabalho P200B |
|-----------|-------------------|-----------------|
| **A — Sub-store novo** | P193B (`ResolvedLabelStore`) | `headings_for_toc: Vec<(Label, Content, usize)>` em `TagIntrospector` (8º → 9º) |
| **B — Variant Tag pós-recursão locatable** | Variante P196B (Heading auto-toc) | `ElementPayload::HeadingForToc { label, body, level }` (12ª → 13ª variant) emitida pelo walk arm Heading com `emitted_loc` directo |
| **C — Consumer migration substitution-with-fallback** | P184D (figure ref-arm) / P194B (text ref-arm) | `outline.rs:24` migrado para tentar Introspector primeiro, fallback legacy |

### Trabalho concreto em P200B

1. **Sub-store novo** em `entities/introspector.rs`:
   `pub headings_for_toc: Vec<(Label, Content, usize)>` em
   `TagIntrospector` + trait method `headings_for_toc(&self)`
   (trait passa de 19 → 20 métodos).

2. **Variant nova** em `entities/element_payload.rs`:
   `ElementPayload::HeadingForToc { label: Label, body: Content,
   level: usize }` (12ª → 13ª variant). Não há
   `ElementKind::HeadingForToc` correspondente — HeadingForToc
   é Tag derivada de Heading (não Content standalone).

3. **Helper `compute_heading_for_toc`** privado em
   `rules/introspect.rs` (4º helper na família ADR-0069
   stylesheet após `compute_labelled` P195D,
   `compute_heading_auto_toc` P196B, `compute_figure` P197B).
   Função pura `(state, frozen_body, level) → Option<(Label,
   Content, usize)>`. Sempre `Some` — paridade com mutação 4
   legacy (push incondicional).

4. **Walk arm Heading** modificado: emitir 3ª Tag pós-recursão
   `Tag::HeadingForToc` após `Tag::Labelled` auto-toc P196B.
   Mesma `emitted_loc`. Mutação 4 legacy
   (`state.headings_for_toc.push`) **preservada** como write
   paralelo M5 (Layouter assignments `mod.rs:1490, 1521`
   dependem).

5. **`from_tags` arm `HeadingForToc`** em
   `rules/introspect/from_tags.rs`: push directo em
   `intr.headings_for_toc.push((label, body, level))`.

6. **Consumer outline.rs:24** migrado para
   substitution-with-fallback (Introspector first; legacy
   fallback).

### Sequência de tags por Heading (após P200B)

Para `Content::Heading { level: 1, body: text("Capítulo") }`:

```
Tag::Start(loc, Heading)               // walk top
[recursive body tags]
Tag::Start(loc, Labelled auto-toc-N)   // P196B
Tag::End(loc, 0)
Tag::Start(loc, HeadingForToc)         // P200B (NOVO)
Tag::End(loc, 0)
Tag::End(loc, hash_content(heading))   // walk bottom
```

**6 tags por Heading folha** (era 4 pós-P196B). 3 pares
Start/End válidos — bracketing preservado.

### Estado após P200B

- **E2-residuo fecha estruturalmente** — `intr.headings_for_toc`
  populated via Tag::HeadingForToc. Sub-store agora cobre
  semântica completa (auto-label + body + level).
- **Lacuna #3 fecha**.
- **E2 inteira fecha**: 3 mutações estruturalmente migradas em
  P196B + 1 mutação estruturalmente migrada em P200B.
- **M5 universal completo** — 0 excepções activas + 0 residuos
  + 0 pré-requisitos restantes.
- Output observable em produção **inalterado** —
  substitution-with-fallback no consumer outline garante
  paridade; sub-store fornece dados; legacy fica funcional
  como backup.

### Mutação legacy preservada (M6)

Mutação 4 (`state.headings_for_toc.push`) continua activa
porque Layouter assignments (`mod.rs:1490, 1521`) fazem
`l.counter.headings_for_toc = initial_state.headings_for_toc`.
Mover para Introspector path completo exige refactor
cross-modular — domínio de **M6 (P190A reescrita do zero —
eliminação `CounterStateLegacy`)**.

### Cross-references

- **P189B** — Outline migrado; declaração das 6 excepções +
  Reserva 1 + lacuna #3.
- **P193B** — sub-store `ResolvedLabelStore` (template
  categoria A).
- **P194B / P184D** — consumer migrations
  substitution-with-fallback (template categoria C).
- **P196B** — walk arm Heading auto-toc (Tag::Labelled
  pós-recursão + 3 mutações migradas; declaração formal de
  E2-residuo).
- **P196A §11.5** — Tag auto-toc partilha Location com Tag
  Heading (template para Tag HeadingForToc partilhar mesma
  Location).
- **ADR-0069** — pattern stylesheet (5 variantes operacionais
  consolidadas). P200B é trabalho **híbrido**, não nova
  variante.

## Marco: M5 universal completo (P200B)

**Primeira vez desde declaração em P189B**.

Após **P200B**:
- Todos walk arms cristalinos fechados estruturalmente:
  - Outline migrado (P189B).
  - Bibliography migrado (P181H).
  - Labelled migrado estruturalmente (P195D).
  - **Heading migrado estruturalmente** (P196B 3 mutações
    + **P200B 4ª mutação**).
  - Figure fechada estruturalmente (P197B — cenário α).
  - SetHeadingNumbering fechada estruturalmente (P198B —
    cenário α).
  - CounterUpdate fechada estruturalmente (P198C — cenário
    β-promote).
  - SetEquationNumbering fechada estruturalmente (P199B —
    cenário α por construção).
- 0 excepções M5 activas.
- 0 residuos.
- 0 pré-requisitos restantes.

**Desbloqueia M6 (P190A reescrita do zero — eliminação
`CounterStateLegacy`; magnitude L cross-modular)**.

Mutações legacy ainda activas como write paralelo M5
(consumers `compute_*` helpers + Layouter assignments
dependem). Cleanup orgânico em M6.

## `apply_state_displays` — Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ)

Slim post-pass para `Content::StateDisplay` paralelo absoluto
a `apply_state_funcs` P191B. Localização:
`01_core/src/rules/introspect/from_tags.rs`.

```rust
pub fn apply_state_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
)
```

**Algoritmo**:
1. Iterar `tags` procurando `Tag::Start(loc, info)` com
   `info.payload = ElementPayload::StateDisplay { key, callback }`.
2. Para cada match: lookup `intr.state.value_at(key, loc)` →
   se `None`, usar `Value::None`.
3. Se `callback.is_some()`: chamar
   `apply_func(callback.clone(), Args::positional(vec![value]),
   ctx, engine)`:
   - `Ok(Value::Content(c))` → `c`.
   - `Ok(Value::Str(s))` → `Content::text(s.as_str())`.
   - `Ok(_)` → `Content::Empty` (fallback outros tipos).
   - `Err(_)` → `Content::Empty` (defensive ignore paridade P191B).
4. Se `callback.is_none()`: converter value directo
   (`Value::Content(c)` → `c`; `Value::Str(s)` →
   `Content::text(s)`; outros → `Content::Empty`).
5. Armazenar `intr.state_displays.insert((key.clone(), loc),
   pre_rendered)`.

**Caller**: `fixpoint::run_fixpoint` (após `apply_state_funcs`,
para que state values cumulativos estejam materializados).

```rust
// Em run_fixpoint:
apply_state_funcs(&tags, &mut introspector, engine, ctx);
apply_state_displays(&tags, &mut introspector, engine, ctx);
```

**Ordem location-monotónica**: walk emite tags por ordem de
Locator (counter incrementado). `apply_state_displays` processa
na mesma ordem; `state.value_at(key, loc)` devolve o valor
cumulativo correcto pós-`apply_state_funcs`.

## `Introspector::state_display_value` — Passo 240

```rust
fn state_display_value(
    &self,
    key: String,
    location: Location,
) -> Option<Content>;
```

Lookup do Content pre-rendered armazenado por
`apply_state_displays`. Retorna Owned `Content` (clone) —
necessário porque `comemo::Tracked` não permite retornar
`&Content` directo. Caller layout arm `Content::StateDisplay`
consome valor:

```rust
Content::StateDisplay { key, callback: _ } => {
    use crate::entities::introspector::Introspector;
    if let Some(loc) = self.current_location {
        if let Some(pre) = self.introspector
            .state_display_value(key.clone(), loc)
        {
            self.layout_content(&pre);
        }
    }
}
```

**Layouter permanece puro** — sem Engine+ctx em signature;
paridade arquitectural estrita preservada (Opção γ vs α/β/δ
P239 audit).

**Primeira excepção justificada à aplicação automática ADR-0080
EM VIGOR pós-P229** — feature runtime nova + walk integration
merece L0 tocado partial (bloco StateDisplay em
`entities/content.md` + bloco state_display em `rules/stdlib.md`
+ este bloco).

## `apply_counter_displays` — Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto P240)

Slim post-pass para `Content::CounterDisplayCallback` **paralelo
absoluto** `apply_state_displays` P240. Localização:
`01_core/src/rules/introspect/from_tags.rs`.

```rust
pub fn apply_counter_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
)
```

**Algoritmo**:
1. Iterar `tags` procurando `Tag::Start(loc, info)` com
   `info.payload = ElementPayload::CounterDisplay { key, callback }`.
2. Para cada match: lookup `intr.counters.value_at(key, loc)`
   `Option<&[usize]>` → converter para `Value::Array(Vec<Value::Int>)`
   (counter inexistente → `Value::Array(vec![])`).
3. Se `callback.is_some()`: chamar
   `apply_func(callback.clone(), Args::positional(vec![array]),
   ctx, engine)`:
   - `Ok(Value::Content(c))` → `c`.
   - `Ok(Value::Str(s))` → `Content::text(s.as_str())`.
   - `Ok(_)` → `Content::Empty` (fallback outros tipos).
   - `Err(_)` → `Content::Empty` (defensive ignore paridade
     `apply_state_displays`).
4. Se `callback.is_none()`:
   - Counter populated: formato default "1.2.3" via join "." dos
     items do slice (paridade `formatted_counter_at` P177).
   - Counter inexistente: `Content::Empty`.
5. Armazenar `intr.counter_displays.insert((key.clone(), loc),
   pre_rendered)`.

**Caller**: `fixpoint::run_fixpoint` (após `apply_state_displays`).

```rust
// Em run_fixpoint:
apply_state_funcs(&tags, &mut introspector, engine, ctx);
apply_state_displays(&tags, &mut introspector, engine, ctx);
apply_counter_displays(&tags, &mut introspector, engine, ctx);
```

**Ordem location-monotónica**: walk emite tags por ordem de
Locator. `apply_counter_displays` processa na mesma ordem;
`counters.value_at(key, loc)` devolve o snapshot cumulativo
correcto pós-`apply_state_funcs`.

## `Introspector::counter_display_value` — Passo 241

```rust
fn counter_display_value(
    &self,
    key: String,
    location: Location,
) -> Option<Content>;
```

Lookup do Content pre-rendered armazenado por
`apply_counter_displays`. Paralelo absoluto a
`state_display_value` P240. Caller layout arm
`Content::CounterDisplayCallback`:

```rust
Content::CounterDisplayCallback { key, callback: _ } => {
    use crate::entities::introspector::Introspector;
    if let Some(loc) = self.current_location {
        if let Some(pre) = self.introspector
            .counter_display_value(key.clone(), loc)
        {
            self.layout_content(&pre);
        }
    }
}
```

**Layouter permanece puro** — sem Engine+ctx em signature;
paridade arquitectural estrita preservada (Opção γ vs α/β/δ
P239 audit).

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229** —
N=1 (P240) → 2 (P241) cumulativo (este bloco + bloco
`Content::CounterDisplayCallback` em `entities/content.md` +
bloco `counter_display(key, [callback])` em `rules/stdlib.md`).
