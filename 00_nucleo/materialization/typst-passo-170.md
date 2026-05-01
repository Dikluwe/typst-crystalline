# Passo P170 — Segunda feature Introspection (M9 sub-passo 2)

Continuação de M9. Decisão pós-P169: estratégia **caso a caso**
— sem regra fixa "simples primeiro". Cada passo M9 escolhe
feature com base em pré-requisitos satisfeitos, magnitude real,
e impacto em M5 (destrava lacuna ou não).

**Decisão pendente em `.A`**: qual feature segunda. Candidatas
com classificação preliminar:

**Group A** (destravam consumers M5 bloqueados):
- `state(key, init)` — destrava lacuna #4. Substancial.
- `CounterKey` enum + hierarquia — destrava lacuna #5. Refino.
- `query()` user-facing — destrava lacuna #7. M.
- `counter.at(label)` / `counter.final()` — depende de
  `CounterKey`. M.

**Group B** (não destravam M5 imediato, ou bloqueadas por
pré-requisitos):
- `here()` — precisa de `Locator::current()` + EvalContext
  field. M-L.
- `position(label)` — precisa de `Position` map populado.
  M.
- `locate(fn)` — precisa de fixpoint M7. Não viável.

**Pré-condição**: P169 concluído. `metadata(value)` existe,
`MetadataStore` populado, ciclo completo de feature nova
validado.

**Restrições**:
- Não modificar consumers de M5 (em pausa).
- Não eliminar `CounterStateLegacy` (M6).
- Output observable não muda; snapshot tests passam
  inalterados.
- Estratégia caso a caso: sem regra fixa de prioridade.

---

## Sub-passos

### .A Inventário e escolha da feature

Análise factual antes de decidir.

1. **Confirmar estado dos pré-requisitos** das candidatas
   Group A:

   #### `state(key, init)`
   - **Content variant**: ausente. Adicionar
     `Content::State { key, init }` + `Content::StateUpdate { key, update }`.
   - **ElementPayload variants**: adicionar `State` +
     `StateUpdate`.
   - **`extract_payload` arms**: 2 arms novos.
   - **Sub-store**: criar `StateRegistry` em
     `entities/state_registry.rs` (similar a
     `MetadataStore` mas mais complexo — guarda histórico
     ordenado por Location).
   - **Stdlib**: `state(key, init)` retorna `Content::State`;
     `state.update(key, fn)` retorna `Content::StateUpdate`.
   - **Method no `Introspector`**: `state_value(key, location)`.
   - **Dependências**: pode reutilizar padrão de
     `MetadataStore` (P169). Sem dependência em fixpoint
     directa — first-iteration-only resolve init+ordered
     updates.
   - **Lacuna que destrava**: #4 `numbering_active`.
   - **Magnitude estimada**: M-L (substancial mas
     decomponível).

   #### `CounterKey` enum + hierarquia
   - **Tipo**: substituir `String` actual em
     `CounterRegistry` por enum `CounterKey { Page, Selector(SelectorRef), Str(String) }`.
     Vanilla forma.
   - **Hierarquia**: counters actuais são flat
     (`Vec<usize>` representa apenas valor atómico).
     Hierarquia significa indexação por nível
     (`heading.level(2)` distinto de `heading.level(1)`).
     Refactor de `Counter` em `entities/counter.rs` (existe
     desde P165? confirmar).
   - **Format hierárquico**: `format_hierarchical(key)`
     em `CounterStateLegacy` retorna string `"1.2.3"`.
     Equivalente em `Introspector`: método novo
     `formatted_counter(key) -> Option<String>` que
     reconstrói via hierarquia.
   - **Dependências**: nenhuma feature; apenas refactor de
     tipo existente.
   - **Lacuna que destrava**: #5 `format_hierarchical`
     hierárquico.
   - **Magnitude estimada**: M (refactor controlado).

   #### `query()` user-facing
   - **Content variant**: nenhum. `query()` é stdlib.
   - **Stdlib**: função pública `query(selector) -> Vec<Content>`.
   - **Sub-tipo**: `QueryEngine` (registado no desenho mas
     não criado em M3 — é responsabilidade do
     `Introspector` actual via métodos de query).
   - **Dependências**: depende de `Introspector` ter API
     suficiente; pode revelar lacunas no trait actual.
   - **Lacuna que destrava**: #7 `has_outline` (consumer
     de outline pode usar `query(Outline)`).
   - **Magnitude estimada**: M (stdlib + métodos novos no
     trait).

   #### `counter.at(label)` / `counter.final()`
   - **Stdlib**: `counter.at(label)` consulta
     `LabelRegistry::lookup(label) → Location`, depois
     `CounterRegistry::value_at(key, location)`.
     `counter.final()` retorna valor final.
   - **Dependências**: precisa de `CounterRegistry` com
     hierarquia (depende de `CounterKey` enum). Bloqueado
     se não houver hierarquia.
   - **Magnitude estimada**: S-M (depende de pré-requisito).

2. **Verificar pré-requisitos no cristalino**:

   - `Counter` tipo existe (P165 .C criou). Confirmar
     forma actual (`Vec<usize>` flat ou já hierárquico?).
   - `Func` value type existe em cristalino (relevante
     para `state.update(key, fn)` que aceita callback).
   - Stdlib registry (mesmo mecanismo de P169) está
     disponível.
   - `Selector` ou tipo similar para `CounterKey::Selector`
     variant existe? Provavelmente não — é tipo de match
     vanilla. Pode ser limitação inicial.

3. **Aplicar regras de prioridade caso a caso**:

   | Critério | state | CounterKey | query | counter.at |
   |----------|-------|------------|-------|------------|
   | Destrava M5 | ✓ #4 | ✓ #5 | ✓ #7 | parcial |
   | Auto-contida | ~ (parecido metadata) | ~ (refactor isolado) | depende Introspector | ✗ depende CounterKey |
   | Pré-requisitos satisfeitos | ✓ | ✓ | a verificar | ✗ |
   | Magnitude | M-L | M | M | S-M |
   | Reutiliza padrão (metadata) | ✓ | ✗ | ✗ | ✗ |
   | Bloqueia outras features | não | sim (counter.at) | não | não |

4. **Decisão**:

   Critérios para escolha (sem ordem fixa — caso a caso):
   - **Maior magnitude tolerada**: até M, eventualmente
     M-L se ganho for substancial.
   - **Lacuna destravada**: preferir features que destravem
     consumers M5.
   - **Reutilização de padrão**: features que reutilizam
     trabalho anterior facilitam.
   - **Pré-requisitos satisfeitos**: rejeitar features
     bloqueadas.

   Sugestões prováveis (sem decidir):
   - **`state(key, init)`** — reutiliza padrão metadata,
     destrava lacuna #4, magnitude controlada.
   - **`CounterKey` enum + hierarquia** — refino sem feature
     nova, destrava lacuna #5, desbloqueia
     `counter.at`/`final` para passos seguintes.

   `query()` e `counter.at` são candidatas mas têm
   dependências que tornam P170 menos auto-contida.

Output: notas internas + decisão registada com justificação.

**Critério de saída e gate de decisão**:
- Se uma das 4 candidatas Group A tem pré-requisitos
  satisfeitos e magnitude ≤ M-L: prosseguir.
- Se múltiplas são viáveis: cláusula gate trivial,
  escolher com critérios documentados.
- Se nenhuma é viável (improvável): gate substancial,
  reabrir.

### .B Sub-passos da feature escolhida

A implementação concreta depende da escolha em `.A`. Para
cada candidata viável, um sub-passo `.B.<feature>` próprio.

Não desenvolvo aqui em detalhe para evitar inflar passo
prematuramente. Cada feature, quando escolhida, segue padrão
estabelecido em P169 (`.B.metadata`):

1. Adicionar variants a `Content` se aplicável (decisões
   tomadas em .A).
2. Adicionar variants a `ElementPayload`, `ElementKind` se
   aplicável.
3. Adicionar arms a `extract_payload`, `is_locatable`,
   `hash_content`, `layout/mod.rs::layout_content`,
   `content.rs::plain_text`/`map_content`/`map_text`,
   `introspect.rs::materialize_time`/`walk` (se variant
   novo em Content).
4. Criar sub-store em `entities/<store>.rs` se aplicável.
5. Estender `from_tags` para popular sub-store.
6. Estender `Introspector` trait com métodos novos.
7. Stdlib registar função(ões).
8. Tests co-localizados + E2E.

Variantes específicas:

#### `state(key, init)`
- 2 Content variants (`State`, `StateUpdate`).
- `StateRegistry` mais complexo que `MetadataStore`
  (histórico ordenado por Location).
- Stdlib `state(key, init)` + `state.update(key, fn)`.
- Method no Introspector: `state_value(key, location)`.

#### `CounterKey` enum + hierarquia
- Modificar tipo `Counter` em `entities/counter.rs`
  para hierarquia.
- Substituir `String` em `CounterRegistry` por
  `CounterKey` enum.
- Method no Introspector:
  `formatted_counter(key) -> Option<String>`.
- Refactor possivelmente afeta `extract_payload` arm
  para `Heading` (passa de `CounterUpdate::Step(usize)`
  para algo hierárquico).

#### `query()`
- Stdlib `query(selector) -> Vec<Content>`.
- Selector tipo: provavelmente forma minimal
  (`Selector::Kind(ElementKind)` apenas; outras vanilla
  variants adiadas).
- Method no Introspector: `query(&self, selector)`.

#### `counter.at(label)` / `counter.final()`
- Apenas viável se `CounterKey` hierárquico já existe.
- Stdlib `counter.at(label)` e `counter.final(key)`.
- Methods no Introspector: `counter_value_at(key, location)`,
  `counter_final_value(key)`.

### .C Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P169 baseline (1612).
3. `crystalline-lint`: zero violations.
4. Feature escolhida em `.A` materializada com L0+L1
   completos.
5. Specifics da feature confirmados:
   - **`state`**: Content::State + StateRegistry + stdlib
     state.
   - **`CounterKey`**: enum existe; CounterRegistry usa
     enum; format hierárquico funciona.
   - **`query`**: stdlib query + Selector tipo + method
     no trait.
   - **`counter.at`/`final`**: stdlib funcs + methods
     no trait.
6. Snapshot tests de paridade ADR-0033 passam inalterados.
7. Linter passa em verificação final.

### .D Encerramento

Escrever
`00_nucleo/materialization/typst-passo-170-relatorio.md` com:

- Resumo: feature escolhida em `.A` com justificação;
  materialização completa.
- Confirmação de cada verificação .C.
- Hashes finais de L0s novos/modificados.
- Decisões registadas em `.A`:
  - Feature escolhida + justificação caso a caso.
  - Alternativas avaliadas.
  - Pré-requisitos verificados ou criados.
- Δ tests vs baseline P169.
- **Estado de M9**: 2/11 features materializadas.
- **Lacuna resolvida** (se aplicável): #4 (state), #5
  (CounterKey), ou #7 (query).
- Pendências cumulativas + actualização.
- Estado pós-passo: P170 concluído. P171 desbloqueado —
  terceira feature M9.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário e escolheu feature com
   justificação caso a caso.
2. Feature materializada (L0+L1+stdlib+tests).
3. Verificações `.C` passam.
4. Relatório `.D` escrito.
5. Output observable não muda.
6. M9 2/11 features concluída.
7. Pelo menos 1 lacuna em `m1-lacunas-captura.md`
   actualizada como resolvida (se feature destrava lacuna).

---

## O que pode sair errado

- **`Counter` tipo existente em cristalino é diferente
  do esperado**: P165 .C criou `Counter` mas forma
  exacta não foi documentada para esta sessão. Se for
  já hierárquico, parte do trabalho de `CounterKey` é
  trivial. Se for flat, refactor maior.
- **`Selector` tipo não existe em cristalino**: para
  `CounterKey::Selector` ou `query(selector)`, precisa
  de tipo Selector. Em vanilla é complexo (Heading
  com nível X, Figure com kind Y, etc.). Cristalino
  pode adoptar forma minimal (`Selector::Kind(ElementKind)`)
  e adiar restante.
- **`Func` value não tem mecanismo de eval em walk
  context**: relevante para `state.update(key, fn)`. Pode
  forçar adiar callbacks (apenas suportar
  `state.update(key, value)` sem closures).
- **Variant novo em `Content` força revisão em mais arms
  que P169**: P169 já adicionou 7 arms para `Metadata`.
  P170 pode adicionar 1-2 variants (state/StateUpdate),
  forçando 7-14 arms novos. Revisar todas as funções
  exhaustive matching.
- **`StateRegistry` mais complexo que `MetadataStore`**:
  histórico ordenado por Location significa estrutura
  diferente. Pode usar `BTreeMap<Location, Value>` ou
  `Vec<(Location, Value)>` ordenado. Decisão local.
- **`format_hierarchical` exigir mais que apenas
  hierarquia**: pode incluir formatting (números arábicos,
  romanos, alfabético, etc.). `Numbering` tipo do
  cristalino (existe? verificar). Se ausente, criar
  forma minimal.
- **Linter detecta divergência com tantos ficheiros
  tocados**: ajustar conforme erro.
- **`metadata` como base para `state`**: alguns designs
  vanilla constroem state sobre metadata. Pode ou não
  ser viável em cristalino. Verificar e adoptar se
  viável.

---

## Notas operacionais

- **Tamanho**: M (CounterKey, query) a M-L (state). Decidido
  em `.A`.
- **Pré-condição P171**: feature seguinte de M9. Estratégia
  caso a caso continua.
- **Cláusula gate trivial**: aplicável em decisões locais
  (forma de Selector, mecanismo de Func, formato de
  hierarquia).
- **`metadata` validou ciclo completo**: P170 reutiliza
  padrão. Adicionar Content variant + ElementPayload variant
  + sub-store + arms exhaustivos + stdlib é trabalho
  conhecido após P169.
- **Lacunas #4, #5, #7 destravam M5**: cada uma destas
  features é candidata óptima por desbloquear consumer
  específico. Lacunas #2, #3, #6 não são endereçadas por
  features Introspection (são divergências/limitações
  arquitecturais ou domínio bibliografia). Quando M9
  inteiro estiver concluído, M5 retomar com base em
  Introspector mais completo.
- **Estado de pendências**: M9 começa a fechar pendências
  registadas. Cada feature implementada deve actualizar
  `m1-lacunas-captura.md` se aplicável.
