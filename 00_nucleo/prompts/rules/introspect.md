# L0 — Motor de Introspecção (`rules/introspect.rs`)
Hash do Código: 3bc33823

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
| **E3** | `Content::Figure` | `state.local_figure_counters`, `state.figure_numbers` | `Labelled` arm lê `figure_numbers` durante walk para popular `figure_label_numbers`. Sub-stores existem (P184B + P168) mas chained com E2-residuo. | E2-residuo fecha primeiro. |
| **E4** | `Content::Labelled` | `state.figure_label_numbers`, `state.resolved_labels` | **Fechou estruturalmente em P195D** (caminho Introspector activo via Tag::Labelled). Mutação legacy mantida como write paralelo M5; remoção orgânica em M6. | Funcional fecha em M6. |
| **E5** | `Content::SetHeadingNumbering` | `state.numbering_active.insert("heading", ...)` | `Heading` arm lê `is_numbering_active("heading")` durante walk para resolver auto-toc text. Tag StateUpdate emitida paralelamente via P182C; `StateRegistry` populado independentemente — legacy mutation é write paralelo. | E2 fecha; legacy mutation removida orgânicamente. |
| **E6** | `Content::CounterUpdate` | `state.step_*`, `update_flat` | `Labelled` arm pode ler counter mutado via CounterUpdate durante walk. Chained com E2 (Reserva 2 alargada). | E2 fecha primeiro. |

**Padrão de cadeia**: 5 das 6 excepções (E2/E3/E4/E5/E6) fecham
em sequência após desbloquear sub-store `resolved_labels` + C4
migration. E1 é independente (Reserva 1 distinta).

**Estado P196B (2026-05-03)**: E4 fechou estruturalmente em
P195D; E2 fechou estruturalmente em P196B (3 das 4 mutações
migradas via Tag pattern ADR-0069). Resta **E2-residuo**
(`headings_for_toc.push`) bloqueado por lacuna #3 (sub-store
ausente). E3/E5/E6 continuam activas mas a cadeia já não
bloqueia caminho Introspector funcional.

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
6. Migrar walk arm `Figure` (E3 fecha).
7. Migrar walk arms `SetHeadingNumbering` + `CounterUpdate` (E5/E6
   fecham residual).
8. Quando `Content::SetEquationNumbering` materializar, E1 fecha.

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
