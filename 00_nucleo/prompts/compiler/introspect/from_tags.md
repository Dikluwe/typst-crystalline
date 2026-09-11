# Prompt L0 — `rules/introspect/from_tags`
Hash do Código: af056fe1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`compiler/introspect/from_tags.rs:422–472` reconstrói Equation com campos
realizados; `:190` já converte retorno de callback a Content para display.
Essa fase existente não é o primeiro snapshot de Heading (pipeline:644–666).

### Decisão proprietária

Adaptar leituras/escritas de elements ao carrier do owner Value. Na realização
de Equation, obter o Content por content(), produzir a mesma visão Styled
contratada em P1140.5-A e substituir a entrada por nova IntrospectedContent
com fields None. None aqui usa a projeção realizada de Equation preexistente,
não significa apagar essa realização. Não tocar entradas Heading Some.

Onde um resultado LocatedContent já é convertido para Content de display,
usar into_content() explicitamente. Isso não altera Values guardados em
state, arrays ou closures. Não acrescentar pós-processador de Heading,
antecipar callback ou realizar campos em loading. Aceitação preserva as
sondas de Equation (numbering/supplement/alt) e a ausência de mutação de
snapshots Heading durante esses pós-processadores.

---


> **P1140.4-A2:** `apply_equation_numberings` recebe tags, introspector já
> populado, `Engine` e `EvalContext`; formata pattern ou aplica callback unário
> ao inteiro, guarda `Content` por `Location` e propaga erros. É chamada pelo
> fixpoint e por `introspect_with_runtime`.

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/introspect/from_tags.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .E — construtor da `TagIntrospector`)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`from_tags(&[Tag]) -> TagIntrospector` é o construtor que transforma a sequência de tags emitidas pelo walk (P162) numa estrutura indexada e queriável. Single pass; match exaustivo sobre `ElementPayload` para forçar revisão quando variant novo for adicionado.

Vanilla equivalente: `ElementIntrospectorBuilder` em `lab/typst-original/.../introspection/introspector.rs::468`. Cristalino simplifica: função pura sem builder mutável intermediário; agrega directamente em `TagIntrospector`.

---

## Restrições Estruturais

- Camada **L1**: função sem I/O. **P173**: aceita `Engine + EvalContext` opcionais para eval de `StateUpdate::Func`. Sem Engine, mantém-se pura.
- `pub fn from_tags(tags: &[Tag], engine: Option<&mut Engine<'_>>, ctx: Option<&mut EvalContext>) -> TagIntrospector`.
- Match **exaustivo** sobre `ElementPayload` (compilador força revisão quando variant novo for adicionado a `ElementPayload`).
- Bracketing válido: assume tags já bracketed (`Tag::Start` sempre seguido eventualmente de `Tag::End` correspondente). Comportamento se mal-formado é debug-assert.
- Determinístico: mesma input produz mesmo output (assumindo Funcs deterministas — vanilla proíbe Funcs com side-effects em state).

## Lógica

Para cada tag:
- `Tag::Start(loc, info)`:
  1. Se `info.label.is_some()`: `labels.add(label, loc)`.
  2. Match sobre `info.payload`:
     - `Heading { depth, counter_update: _, .. }`: kind_index[Heading].push(loc); **P170**: `counters.apply_hierarchical("heading", *depth as usize)` em vez de apply flat — paridade com walk arm `Content::Heading` em introspect.rs:279. counter_update é ignorado para Heading (depth é fonte autoritativa).
     - `Figure { kind, counter_update, is_counted, .. }`: kind_index[Figure].push(loc). **P184B**: `counters.apply_at(format!("figure:{}", kind.as_deref().unwrap_or("image")), counter_update, loc)` — chave per-kind (`figure:image`, `figure:table`, …) com default `"image"` replicando `introspect.rs:391` e `mod.rs:431` (P184A cláusula 1). Em paralelo, `counters.apply_at("figure", counter_update, loc)` mantém a chave global durante janela compat M6 (P184A cláusula 5 — dead code factual em produção, simétrico com walk legacy `state.figure_numbers` que também não é copiado ao Layouter; cleanup orgânico em M6 junto com `CounterStateLegacy`). Convenção `figure:{kind}` originalmente documentada em `element_payload.rs:52` mas não implementada até P184B. **P168**: se `is_counted == true` E `info.label.is_some()`, indexar em `figure_label_numbers` com número 1-based sequencial.
     - `Citation { .. }`: kind_index[Citation].push(loc); (sem counter_update — Citation não tem campo counter_update).
     - `Metadata { value }`: kind_index[Metadata].push(loc); `metadata.add(*value.clone())` em ordem de aparecimento. **P169 M9**.
     - `State { key, init }`: kind_index[State].push(loc); `state.init(key.clone(), (**init).clone(), loc)`. **P171 M9**.
     - `Outline`: kind_index[Outline].push(loc) — feature minimal P178.
     - `Bibliography { entries }` **P181E**: kind_index[Bibliography].push(loc); para cada entry em entries, `bib_store.assign_number(entry.key.clone(), bib_store.len() as u32 + 1)` (numeração 1-based contínua, replica `state.bib_numbers.len() + 1` em walk arm); finalmente `bib_store.add_bibliography(entries.clone())` (extend, cláusula 2 P181A). Multi-Bibliography concatena entries e preserva primeiro número via `or_insert` (cláusula 3 P181A — comportamento herdado de `assign_number`).
     - `Equation { block, counter_update }` **P186E**: `kind_index[Equation].push(loc)` (P186D estendeu o stub introduzido em P186B com este populate); gate location-aware `if *block && matches!(state.value_at("numbering_active:equation", loc), Some(Value::Bool(true)))` → quando dispara, `counters.apply_at("equation".to_string(), counter_update.clone(), loc)`. Padrão Figure (P184B) com gate adicional simétrico ao walk legacy (`introspect.rs:377-382`). **Gate dormente em produção**: `Content::SetEquationNumbering` ainda não existe em cristalino (descoberta P186A §11.2), logo `state.value_at("numbering_active:equation", _)` é sempre `None` em runtime real → counter `equation` permanece vazio. Gate activa quando equation set rule materializar (passo dedicado fora da série P186). Suporta C2 desbloqueio per ADR-0068 (eixo 2 P183C); consumer migra em P188 com substitution-with-fallback.
     - `StateUpdate { key, update }`: kind_index[StateUpdate].push(loc).
       - `update == StateUpdate::Set(value)`:
         - **P182C**: se `state.value_at(key, loc) == None` (key nunca inicializada), `state.init(key, *value, loc)` (auto-init na primeira ocorrência). Suporta state interno emitido por `Content::SetHeadingNumbering` (chave `numbering_active:heading`), que não tem `Content::State` antecedente.
         - Senão: `state.update(key, *value, loc)` (caminho normal P171; userspace `Content::State` inicializa via arm dedicado acima).
       - `update == StateUpdate::Func(fn)` **P173 M9**:
         - Se `engine` e `ctx` ambos `Some`: consultar `state.value_at(key, loc)`; se `Some(curr)`, chamar `apply_func(fn, Args::positional(vec![curr]), ctx, engine)`. Em `Ok(new)`, registar `state.update(key, new, loc)`. Em `Err(_)`, defensive ignore (refino futuro: diagnostics).
         - Se `engine` ou `ctx` ausente: defensive ignore (Func ignorada, registry inalterado).
         - Sem init prévio (`value_at == None`): defensive ignore (P171 padrão).
- `Tag::End(_, _)`: ignorar em M3. Hash do conteúdo é input para detecção de mudança em M7+ fixpoint.

---

## Interface pública

```rust
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::tag::Tag;
use crate::compiler::eval::EvalContext;

pub fn from_tags(
    tags:   &[Tag],
    engine: Option<&mut Engine<'_>>,
    ctx:    Option<&mut EvalContext>,
) -> TagIntrospector;
```

**P173**: assinatura estendida com `Engine` + `EvalContext` opcionais. Quando ambos `Some`, eval real de `StateUpdate::Func` via `apply_func`. Quando algum `None`, comportamento defensivo: `Func` é ignorada (cf. P171 "update sem init é ignorado").

---

## Semântica

- `from_tags(&[])` produz `TagIntrospector::empty()`.
- Tags que aparecem várias vezes (e.g. mesma label em dois Headings — improvável mas possível): primeira inserção em `LabelRegistry` ganha (per `LabelRegistry::add`); todas as locations entram no `kind_index` (preserva ordem de aparecimento).
- Counters acumulam: 3 Headings com `CounterUpdate::Step` → `counters.value("heading") == Some([3])`.

---

## Invariantes

- Função pura, sem state global.
- Match exaustivo: `_ => unreachable!()` é proibido — todos os 3 variants de `ElementPayload` devem ser cobertos explicitamente.
- Determinismo: para o mesmo input, output é estruturalmente igual (modulo ordering interna de HashMap, que é não-ordenado mas determinístico em conteúdo).

---

## Tests obrigatórios (sub-passo .E P165)

- `from_tags(&[])` produz struct vazia (todos os queries retornam `None`/`Vec::new()`).
- `from_tags` com 1 par Start/End de Heading produz `kind_index[Heading] = [loc]`, `counters["heading"] = [1]`.
- Heading com label produz `LabelRegistry` com par.
- 3 Headings com counter_update Step produzem `counters["heading"] = [3]`.
- Sequência mista (Heading, Figure, Citation) produz índices isolados por kind.

---

## Consumers actuais

Nenhum no momento da criação. Consumido em P165 .F por `pub fn introspect()` que chama `from_tags(&tags)` e descarta o resultado.

## Consumers planeados

- M4: `introspect_with_introspector()` ou similar entry point que expõe `TagIntrospector` ao caller.
- M5+: layout migra de `CounterStateLegacy` para `TagIntrospector` para queries de label/counter.

---

## Sobre paridade

Vanilla `ElementIntrospectorBuilder<P>` (linhas 468+) com pilha (`stack: Vec<Vec<BuilderItem<P>>>`), sink, `seen` set, etc. Cristalino simplifica para single-pass linear sem builder explícito — TagIntrospector é construída em loop directo.

Razão da simplificação: vanilla precisa de pilha porque elements podem aninhar arbitrariamente em `Content` (vtable + Packed); cristalino tem aninhamento via Tag::Start/End sequence (já bracketed pelo walk em P162 .E). O bracket-tracking é implícito na ordem das tags — sem necessidade de stack explícito.

Refino futuro possível: se M5+ precisar de informação contextual (e.g. heading actual quando processar figure dentro), adicionar tracking. Em M3 não é preciso.

---

## Resultado Esperado

- `01_core/src/compiler/introspect/from_tags.rs` — função + tests.
- `01_core/src/compiler/introspect.rs` — adicionar `pub mod from_tags;` em paralelo a `pub mod extract_payload;` e `pub mod locatable;`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .E: construtor de TagIntrospector a partir de Vec<Tag> | `from_tags.rs`, `from_tags.md`, `rules/introspect.rs` |
| 2026-04-29 | P173 sub-passo .B: cascade Engine + EvalContext opcionais; eval real de `StateUpdate::Func` via `apply_func` | `from_tags.rs`, `from_tags.md` |
| 2026-05-02 | P182C: arm `StateUpdate::Set` auto-inicia a key se ainda não foi vista (suporte a state interno `numbering_active:*` sem `Content::State` antecedente). Caminho normal preservado para keys já inicializadas. | `from_tags.rs`, `from_tags.md` |
| 2026-05-01 | P181E sub-passo .E: arm `Bibliography { entries }` substitui no-op (P181C) — popula `kind_index[Bibliography]` + `bib_store` via loop de `assign_number` + `add_bibliography` | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P184B: arm `Figure` refinado para popular `CounterRegistry` com chave per-kind `figure:{kind}` (default `"image"`); chave global `"figure"` mantida em paralelo durante janela compat M6 (dead code factual). Promove convenção documentada em `element_payload.rs:52` para implementação. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186B: stub no-op `ElementPayload::Equation { .. } => {}` adicionado para preservar exhaustividade do match após variant ser introduzido em P186B `entities/element_payload`. Cláusula gate trivial — funcionalidade real virá em P186E. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186D: stub estendido com `kind_index[Equation].push(loc)` para preservar sincronização-por-construção da ADR-0068 (test P185D `gating_locator_apenas_em_locatables` agrega `kind_index.values()`; sem populate, walk_locs ≠ layout_locs). Counter logic continua para P186E. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186E: arm `Equation` completo — counter logic `apply_at("equation", counter_update, loc)` gated por `block && matches!(state.value_at("numbering_active:equation", loc), Some(Value::Bool(true)))`. Gate location-aware (Opção B) escolhido por futureproofing alinhado com P185 direcção arquitectural. **Gate dormente em produção** porque `Content::SetEquationNumbering` ausente em cristalino (P186A §11.2). Eixo 2 do bloqueio P183C resolvido estruturalmente. Suporta C2 desbloqueio per ADR-0068; consumer migra em P188 com substitution-with-fallback. | `from_tags.rs`, `from_tags.md` |
| 2026-05-04 | P195B: stub no-op `ElementPayload::Labelled { .. } => {}` adicionado para preservar exhaustividade do match após variant ser introduzido em P195B `entities/element_payload`. Cláusula gate trivial. Variant emergiu de pattern arquitectural novo "post-recursion tag emission" (ADR-0069 PROPOSTO) porque `extract_payload` puro não suporta state-dependent payload. Funcionalidade real (populate `intr.resolved_labels` + `intr.figure_label_numbers`) virá em P195C. | `from_tags.rs`, `from_tags.md` |
| 2026-05-04 | P195C: stub no-op P195B substituído por arm funcional. Match destructure `{ label, resolved_text, figure_number }`; `if let Some(text) = resolved_text` popula `intr.resolved_labels.insert(label.clone(), text.clone())`; `if let Some(n) = figure_number` popula `intr.figure_label_numbers.insert(label.clone(), *n)`. **Walk arm não emite Tag até P195D** — Tags Labelled chegam apenas via tests unit; sub-stores permanecem vazios em produção até P195D. Pattern post-recursion tag emission per ADR-0069. | `from_tags.rs`, `from_tags.md` |

## Proposta P1148 — `CounterUpdate::Func` (AGUARDA GATE ADR-0127)

Se o reshape público de `entities/counter_update.md` for aprovado, o arm de
`ElementPayload::CounterUpdate` deve aplicar também callbacks ao estado
corrente. Ele reutiliza `Engine + EvalContext` já presentes nesta fase para
`StateUpdate::Func`, passa o array corrente como argumentos posicionais da
linguagem, converte o retorno `int | array<int>` em estado completo e propaga
o erro pelo caminho de introspecção/fixpoint. `Set` e `Step(level)` continuam
determinísticos e não precisam de avaliação.

Não se move a execução do callback para eval do field nem para layout. A
proposta amplia uma operação dentro da fase de introspecção existente; se a
implementação exigir alterar a assinatura pública do construtor, deve voltar
ao gate antes dessa alteração.

### P1149 — aridade ratificada do callback

Sonda nos dois binários ratificados confirma que um estado `(2, 3)` pode ser
atualizado por `(a, b) => (a + 1, b + 2)`, resultando em `(3, 5)`. Portanto,
`apply_counter_funcs` passa os componentes como argumentos posicionais
separados, não como `Value::Array` único. O retorno continua a aceitar inteiro
ou array de inteiros não-negativos e é gravado na location do update.

## P1339 — resolução sob demanda de contador filtrado (fase aprovada; gates de integração pendentes)

### Medição anterior à decisão

Em HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
`compiler/introspect/from_tags.rs:85-136` calcula Func só no pós-walk e
acrescenta Set sem recalcular Steps posteriores. Produção expande contextos
antes desse runtime (`03_infra/src/pipeline.rs:644-666`). Os recibos
`diagnosticos/p1339-where-counter-phase-probe-runs.json` e
`diagnosticos/p1339-where-counter-phase-file-runs.json` registram:
vanilla aceita assert de valor 12 em Step/Func/Step no contexto; sem consulta
ao contador, panic do contexto precede panic no callback anterior. Portanto
executar antecipadamente todas as Func para preparar contextos é refutado.
O controle bare no cristalino falha assert; o filtro vazio falha antes, na
construção where. Não atribuir a esta última falha prova de runtime.

Medição focal posterior, anterior à decisão abaixo: o recibo
`diagnosticos/p1339-where-counter-runtime-probe-runs.json`, SHA-256
`178a8637df7eb75d21131a46ad68005b9f7e35670644c9cb9b7eeb93031d5ae3`,
registra no mesmo HEAD, em 2026-09-10 às 01:36:19.991799–01:36:23.537105 UTC,
get anterior a callback com panic produzindo esse erro posterior. O callback
que lê outro counter falha por ausência de contexto, mesmo criado dentro de
context. A fonte vanilla `introspection/counter.rs:797-798` resolve a sequência
antes do prefixo; `:612` chama Func com Context::none(). Updates gerados em
context e assert dependente passam nos controles com uma página. Os estados
da árvore, entradas e canais integrais estão no recibo; não há teste GREEN
cristalino nesta medição.

### Decisão de fase aprovada — somente chaves contendo Element

Permitir resolução de CounterUpdate::Func **sob demanda de leitura contextual**
da nova chave filtrada, com Engine/Scopes já disponíveis ao despacho. O
algoritmo é interno pub(crate), retorna estado owned via SourceResult e lê
o log imutável da iteração. Não mudar assinatura pública existente nem trait.
Não executar callback no field access que apenas descobre um método, em
entities, no walk puro, em query(selector) ou no layouter.

Fazer replay dos eventos relevantes em ordem: automáticos cujo elemento
capturado casa o seletor, mais manuais da chave com igualdade da linguagem,
não PartialEq/Hash derivados (ordem dos fields distingue; int/float equivalem;
NaN não é reflexivo). Aplicar Step/Set reais;
quando a sequência alcança Func, passar componentes do estado
anterior como posicionais separados e validar o retorno pelo contrato P1149.
Prosseguir com ações posteriores; não usar snapshots calculados antes da Func
como se já refletissem seu efeito. Resolver a sequência completa da chave
antes de selecionar o estado até a Location em at/get; final usa seu estado
final. Erro em callback posterior da mesma chave é observável em get anterior.
Update de chave não demandada não é executado
por uma varredura global de preparação.

Para estas chaves, o pós-processador global apply_counter_funcs não executa
Func avidamente; consumidores efetivos, incluindo display, delegam à mesma
resolução. Preservar as chaves antigas sem Element, callbacks de state,
numbering e equação nos seus pontos vigentes. Não alterar a ordem global da
pipeline nem antecipar todo introspect_with_runtime.

O estado intermediário de replay é local à demanda, não uma mutação do
snapshot público de eval. O log original não vira Set artificial e não é
reescrito durante consulta. Otimização/cache não pode mudar ordem de erros,
capturas, warnings ou reexecutar efeito observado; sua necessidade e política
devem ser medidas antes de selo. Executar o corpo de Func sem contexto
introspectivo disponível, preservando capturas lexicais e Engine: capturar
Counter é válido, consultá-lo dentro desse callback deve produzir o erro
contextual, não iniciar resolução recursiva. Não herdar o contexto da leitura
nem da criação da closure. Eventos nascidos em context exigem controle com
assert dependente, não só contagem de páginas; ausência no snapshot não prova
estado zero nem convergência. A estabilização desses contextos foi incluída
no escopo pelo dono em `diagnosticos/p1339-stabilization-approval.json`; sua
orquestração e observações pertencem a `infra/pipeline.md` e
`compiler/eval.md`. O gate público foi aprovado em
`diagnosticos/p1339-observation-interface-approval.json`; a integração e os
gates independentes permanecem pendentes.

Mudança de fase autorizada pelo dono em
`diagnosticos/p1339-where-counter-phase-approval.json` (ADR-0127 ponto 3).
Esta autorização não equivale a solução demonstrada nem dispensa contrato,
selo, RED e validação independente do P1339.

## P1140.4-C — materialização de suplementos de equação

### Medição antes da decisão

`math/equation.rs:179-188` materializa uma vez: `auto` vira nome localizado,
`none` vira vazio, conteúdo permanece conteúdo e função recebe a equação.
`model/reference.rs:341-355` apenas escolhe precedência e junta NBSP. A
pipeline cristalina já chama pós-processadores com Engine por P1140.4-A.

### Decisão

Adicionar `apply_equation_supplements(tags, intr, engine, ctx)`, chamado nos
mesmos orquestradores de `apply_equation_numberings`. Para cada Equation, gera
um `Content`: `auto` usa o nome localizado capturado; `none` usa Empty;
conteúdo/string usa cast de display; função é aplicada ao conteúdo público da
equação e o retorno passa pelo mesmo cast. Erros propagam. O resultado é
guardado por Location e o layout permanece read-only.

Tabela medida: `en Equation`, `pt Equação`, `de Gleichung`, `fr Équation`,
`es Ecuación`, `it Equazione`; língua ausente/desconhecida cai em inglês.

## P1140.5-A — realização final de equações

Após numbering e supplement, `realize_equation_elements` combina o elemento
base e os valores capturados/materializados em uma visão pública por Location.
Preserva função/pattern de numbering quando esse é o field da linguagem,
default de number-align, supplement sintetizado, `alt` efetivo e body/block.
É chamada nos mesmos orquestradores runtime/fixpoint antes de expor query.

## P1342 — replay do callback ligado ao carrier real

### Medição anterior à decisão

A auditoria P1342 mede que `resolve_filtered_counter` seleciona a ação real por
Location, cria um `EvalContext` filho e aplica o `Func`; sem propagação explícita,
o ledger do produtor não alcança o dispatch/corpo/Dict.

### Decisão vigente

Sob `cfg(p1339_observation)`, ao alcançar `CounterUpdate::Func`, obter somente
do próprio Func o carrier P1342 preservado. Antes de `apply_func`, instalar no
`EvalContext` filho o mesmo ledger/célula/id de ocorrência e anexar o snapshot
prévio e a Location do evento real. Depois da aplicação, anexar resultado/erro e
snapshot posterior efetivo. O dispatch e o body escrevem no mesmo handle.

Carrier ausente em fixture obrigatória é Violated no harness, nunca fabricado
ou Unknown. A resolução normal, `Context::none`, Args posicionais, ordem das
ações, erro e estado continuam iguais; nenhum callback adicional é executado.
Sem o cfg não há branch ou estado novo. O recorte é só o replay focal P1342.
