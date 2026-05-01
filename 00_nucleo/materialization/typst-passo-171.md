# Passo P171 — `state(key, init)` runtime mutable state (M9 sub-passo 3)

Terceira feature M9. Materializa **runtime mutable state** —
feature substancial que destrava lacuna #4 (`numbering_active`)
e estabelece infraestrutura genérica para outras flags
runtime futuras.

**Pré-condição**: P170 concluído. M9 2/11 features. Hierarquia
em `CounterRegistry` materializada.

**Restrições**:
- Não migrar consumer `Layouter` para usar state (M5 retorno).
- P171 apenas materializa a feature; consumer real virá depois.
- Não eliminar `CounterStateLegacy` (M6).
- Output observable não muda; snapshot tests passam
  inalterados.
- Decisão sobre callbacks (`Func`): provavelmente adiada
  para P171+1 — confirmar em `.A`.

---

## Sub-passos

### .A Inventário e decisões

Reverificar (não confiar em P170):

1. **Padrão `metadata` validado em P169**:
   - `Content::Metadata { value }` + `ElementPayload::Metadata`
     + `MetadataStore` + stdlib + arms exhaustivos.
   - Forma replicável para `state`: 2 Content variants (`State`,
     `StateUpdate`) + 2 ElementPayload variants + `StateRegistry`
     + 2 stdlib funcs + arms exhaustivos.

2. **Decidir suporte a callbacks (`Func`) em `state.update`**:
   - Vanilla aceita `state.update(key, fn)` onde `fn` é
     callable que recebe valor actual e retorna novo.
   - Cristalino: `Func` value type existe? Mecanismo de eval
     de Func no walk context existe?
     - `grep -rn "fn eval_func\|Func::call\|Func::apply" 01_core/src/`.
   - Se ausente: **adiar callbacks**. P171 implementa apenas
     `state.update(key, value)` (Set). Closures (`Func`)
     adiadas para passo M9+1 com mecanismo de eval em walk.
   - Se presente: avaliar viabilidade de incluir em P171
     ou adiar.

3. **Estrutura de `StateRegistry`**:
   - `MetadataStore` (P169): guarda valores numa lista. Sem
     histórico por Location.
   - `StateRegistry`: precisa guardar **histórico ordenado
     por Location**. Decisão entre:
     - `BTreeMap<Location, (key, Value)>` (ordenação directa).
     - `Vec<(Location, key, Value)>` ordenado por inserção
       (assume tags ordenadas por construção).
     - `HashMap<key, Vec<(Location, Value)>>` (indexação
       primária por key, ordenada por Location dentro).
   - Decidir em `.A` qual estrutura. Ver "Notas operacionais".

4. **Forma de `StateUpdate` no `ElementPayload`**:
   - Opção 1: enum `StateUpdate { Set(Value) }` (espaço
     para `Func` futuro como variant adicional).
   - Opção 2: struct directa `StateUpdate { value: Value }`
     (sem enum, mais simples).
   - Cláusula gate trivial: escolher conforme padrão do
     desenho. Se Func vai ser adicionada depois, enum é
     melhor (permite variant novo sem mudar API). Se nunca,
     struct é mais simples.

5. **Método `state_value` no `Introspector` trait**:
   - Forma proposta: `state_value(&self, key: &str, location: Location) -> Option<&Value>`.
   - Algoritmo: iterar updates ordenados por Location até
     atingir `location`; retornar último valor (ou init se
     nenhum update anterior).
   - Confirmar que `LabelRegistry`-style lookup é viável
     com a estrutura escolhida em (3).

6. **Pré-requisitos verificados no cristalino**:
   - `Value` type actual: confirmar que pode ser usado
     directamente em payload (P169 já fez para Metadata).
   - Stdlib registry existe (P169 usou `make_stdlib`).
   - Walk arms exaustivos: identificar todas as funções com
     match em `Content` (lista P169 § "O que pode sair errado"
     — 7 arms tocadas).

Output: notas internas + decisões registadas:
- Suporte a callbacks (P171 sim, P171+1 sim, ou nunca em
  cristalino).
- Estrutura de `StateRegistry`.
- Forma de `StateUpdate` (enum vs struct).
- Lista exacta de match arms a actualizar.

**Critério de saída e gate de decisão**:
- Se callbacks são adiadas (esperado): prosseguir com Set
  apenas.
- Se `Value` ou stdlib registry têm forma diferente:
  cláusula gate trivial.
- Se algum pré-requisito está ausente: gate substancial.

### .B Adicionar variants a `Content`

1. L0 `00_nucleo/prompts/entities/content.md`: adicionar
   entradas para:
   - `State { key: Str, init: Box<Value> }`.
   - `StateUpdate { key: Str, update: <forma decidida em .A> }`.
2. L1 `01_core/src/entities/content.rs`: adicionar variants.
   Variants 57→59 (P169 levou de 56 a 57).
3. Tests co-localizados: construir cada variant; igualdade;
   hash determinístico (via `format!("{:?}", c)` per padrão
   `content_hash`).

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Adicionar variants a `ElementPayload` + `ElementKind`

1. L0 `00_nucleo/prompts/entities/element_payload.md`:
   - `State { key: Str, init: Box<Value> }`.
   - `StateUpdate { key: Str, update: <forma .A> }`.
2. L1 `01_core/src/entities/element_payload.rs`: adicionar
   2 variants. Adaptar `Hash` manual (P169 estabeleceu
   forma).
3. L0 `00_nucleo/prompts/entities/element_kind.md`:
   - `State`, `StateUpdate` adicionados.
4. L1 `01_core/src/entities/element_kind.rs`: adicionar
   2 variants. Variants 4→6.
5. Tests co-localizados.

**Critério de saída**: igual a .B.

### .D Adicionar arms exhaustivos

Match arms forçados pelo compilador. Lista (verificada em
`.A.6`):

1. `01_core/src/entities/content.rs::plain_text` — State e
   StateUpdate produzem `String::new()` (invisíveis).
2. `01_core/src/entities/content.rs::map_content` — terminal,
   `self.clone()`.
3. `01_core/src/entities/content.rs::map_text` — terminal,
   `self.clone()`.
4. `01_core/src/entities/content_hash.rs` — incluir variants
   no `format!("{:?}", c)` (automático se Debug derive).
5. `01_core/src/rules/introspect/locatable.rs::is_locatable`
   — `State => true`, `StateUpdate => true`.
6. `01_core/src/rules/introspect/extract_payload.rs` — arms
   novos:
   ```rust
   Content::State { key, init } => Some(
       ElementPayload::State { key: key.clone(), init: init.clone() }
   ),
   Content::StateUpdate { key, update } => Some(
       ElementPayload::StateUpdate { key: key.clone(), update: update.clone() }
   ),
   ```
7. `01_core/src/rules/introspect.rs::materialize_time` —
   terminal, `content.clone()`.
8. `01_core/src/rules/introspect.rs::walk` — terminal, no-op
   (Tag::Start/End emitido no topo via extract_payload).
9. `01_core/src/rules/layout/mod.rs::layout_content` —
   zero-size para ambos (sem rendering).

L0s correspondentes actualizados:
- `locatable.md`, `extract_payload.md`.
- L0 de `introspect.rs` (P165 docs walk).

**Critério de saída**:
- `cargo check` passa.
- Linter passa.

### .E Criar `StateRegistry`

1. L0 `00_nucleo/prompts/entities/state_registry.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/state_registry.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla:
     `lab/typst-original/.../introspection/state.rs`
     (paralelo a `MetadataStore` cristalino).
   - Restrições:
     - Read-only após construção via `from_tags`.
     - `Clone`.
     - Estrutura de dados decidida em `.A.3`.
   - API:
     - `pub fn empty() -> Self`.
     - `pub(crate) fn init(&mut self, key: Str, init: Value, location: Location)`.
     - `pub(crate) fn update(&mut self, key: Str, value: Value, location: Location)`.
     - `pub fn value_at(&self, key: &str, location: Location) -> Option<&Value>`.
     - `pub fn final_value(&self, key: &str) -> Option<&Value>`.
   - Critérios de verificação:
     - `StateRegistry::empty().value_at("k", any)` retorna
       `None`.
     - Após `init("counter", 0, loc1)`,
       `value_at("counter", loc1)` retorna `Some(0)`.
     - Após `update("counter", 5, loc2)` com `loc2 > loc1`,
       `value_at("counter", loc2)` retorna `Some(5)`,
       `value_at("counter", loc1)` retorna `Some(0)`.
     - `final_value("counter")` retorna `Some(5)`.
     - Múltiplas keys independentes.

2. L1 `01_core/src/entities/state_registry.rs`:
   - Cabeçalho `@prompt`.
   - Implementação conforme estrutura escolhida em `.A.3`.
   - Método `value_at` aplica updates ordenados por Location
     até atingir o valor pedido. Algoritmo:
     ```
     - encontrar todas (loc, value) para a key onde loc <= location.
     - retornar último (por ordem de Location) ou None se vazio.
     ```
   - Tests co-localizados (5 mínimos por critérios acima).

3. Update `01_core/src/entities/mod.rs`: re-export
   `StateRegistry`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .F Estender `from_tags` + `Introspector` trait

1. `01_core/src/rules/introspect/from_tags.rs`:
   - Adicionar arms para `ElementPayload::State` e
     `ElementPayload::StateUpdate`.
   - State arm: `state_registry.init(key, init, location)`.
   - StateUpdate arm: `state_registry.update(key, value, location)`.
   - Update L0 correspondente.

2. `01_core/src/entities/introspector.rs`:
   - Adicionar ao trait `Introspector`:
     - `state_value(&self, key: &str, location: Location) -> Option<&Value>`.
     - `state_final_value(&self, key: &str) -> Option<&Value>`.
   - Implementar em `TagIntrospector` (delega para
     `state_registry`).
   - Update L0 correspondente.
   - Tests co-localizados.

3. `TagIntrospector` ganha campo
   `state_registry: StateRegistry`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .G Stdlib `state(key, init)` + `state.update(key, value)`

1. Identificar localização de stdlib registry (P169 usou
   `make_stdlib`).
2. Adicionar:
   - `state(key, init)` — retorna `Content::State { key, init }`.
   - `state.update(key, value)` — retorna
     `Content::StateUpdate { key, update: <forma> }`.
   - **Não** adicionar `state.update(key, fn)` callback
     se decidido em `.A.2`.
3. Tests:
   - Avaliar `state("counter", 0)` produz `Content::State`
     correcto.
   - Avaliar `state.update("counter", 5)` produz
     `Content::StateUpdate` correcto.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .H Tests E2E

1. Documento com `#state("counter", 0)` + heading +
   `#state.update("counter", 5)` + heading:
   - `introspector.state_value("counter", loc_heading_1)`
     retorna `Some(0)`.
   - `introspector.state_value("counter", loc_heading_2)`
     retorna `Some(5)`.
   - `introspector.state_final_value("counter")` retorna
     `Some(5)`.
2. Documento sem state — `state_value` retorna sempre `None`.
3. State e StateUpdate são invisíveis em layout
   (zero-size).
4. Multiple keys independentes: `state("a", ..)` e
   `state("b", ..)` não interferem.

**Critério de saída**:
- 4 tests novos passam.
- Linter passa.

### .I Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P170 baseline (1619). Estimativa: +20 a +30 tests
   (StateRegistry 5+ + extract_payload 2+ + from_tags 2+ +
   layout 4+ + stdlib 2+ + content variants 2+ ...).
3. `crystalline-lint`: zero violations.
4. `Content::State` e `Content::StateUpdate` existem.
5. `ElementPayload::State` e `ElementPayload::StateUpdate`
   existem.
6. `StateRegistry` existe em `entities/`.
7. `Introspector::state_value` e `state_final_value`
   existem no trait + impl.
8. Stdlib `state(key, init)` e `state.update(key, value)`
   registadas.
9. **Callbacks (`Func`) NÃO implementados** se decidido em
   `.A`. Documentar pendência.
10. Snapshot tests de paridade ADR-0033 passam inalterados.
11. Linter passa em verificação final.

### .J Encerramento

Escrever
`00_nucleo/materialization/typst-passo-171-relatorio.md` com:

- Resumo: `state(key, init)` materializado; callbacks
  adiados se aplicável.
- Confirmação de cada verificação .I.
- Hashes finais de L0s novos/modificados.
- Decisões registadas em `.A`:
  - Callbacks: incluído ou adiado.
  - Estrutura de `StateRegistry` escolhida.
  - Forma de `StateUpdate` (enum vs struct).
- Δ tests vs baseline P170.
- **Estado de M9**: 3/11 features materializadas.
- **Lacuna #4**: estado actualizado. P171 adiciona
  infraestrutura `state` mas não migra consumer
  (`Layouter`); lacuna #4 fica em "infraestrutura pronta,
  consumer aguarda M5 retomar".
- Pendências cumulativas + actualização.
- Estado pós-passo: P171 concluído. P172 desbloqueado —
  quarta feature M9.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário e decisões registadas.
2. 2 Content variants + 2 ElementPayload variants +
   2 ElementKind variants adicionados.
3. Match arms exaustivos actualizados em 9 sítios.
4. `StateRegistry` materializado com L0+L1.
5. `Introspector` trait estendido com 2 métodos.
6. Stdlib functions registadas.
7. Tests E2E passam.
8. Verificações `.I` 1-11 passam.
9. Relatório `.J` escrito.
10. Output observable não muda.
11. M9 3/11 features concluída.

---

## O que pode sair errado

- **Callbacks (`Func`) impossíveis de adiar**: se algum
  test ou consumer existente exigir callback, não há
  fallback simples. Improvável em P171 (apenas materializa
  feature, sem migração de consumer).
- **`StateRegistry` estrutura escolhida sub-óptima**: e.g.
  `BTreeMap<Location, ...>` pode complicar lookup por key.
  Opção `HashMap<key, Vec<(Location, Value)>>` é mais
  natural mas exige sort dentro de cada Vec. Performance
  vs simplicidade. Cláusula gate trivial.
- **Variants novos em `Content` forçam revisão em mais
  arms que P169 esperava**: P169 documentou 7 arms; P171
  adiciona 2 variants forçando 2x9 = 18 toques. Compilador
  guia. Linter força sincronização L0↔L1.
- **Stdlib `state.update` namespace**: vanilla expõe como
  método (`state.update(key, fn)`). Cristalino: confirmar
  se stdlib suporta methods em values, ou adoptar forma
  funcional (`state_update(key, value)`). Cláusula gate
  trivial.
- **Snapshot tests detectam mudança observable**: improvável
  (State e StateUpdate são zero-size por design), mas
  verificar.
- **`Box<Value>` em variants força adapt em derive `Eq, Hash`**
  como em P169. Padrão `format!("{:?}", self).hash()`
  estabelecido — replicar.
- **Tests de paridade vs `CounterStateLegacy.numbering_active`**:
  se P171 quiser validar paridade observable da lacuna #4,
  precisa de comparar contra consumer migrado. Mas P171
  não migra consumer — comparação só faz sentido em M5
  retorno. Adiar.
- **Linter divergência com 9+ ficheiros tocados**: ajustar
  conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M-L. Maior que P170 (refactor isolado), mais
  pequeno que P165 (4 L0+L1 novos + grandes mudanças no
  walk). Comparável a P169 com escala maior.
- **Pré-condição P172**: feature seguinte de M9. Estratégia
  caso a caso continua.
- **Cláusula gate trivial**: aplicável a decisões locais
  sobre estrutura de `StateRegistry`, forma de `StateUpdate`,
  forma de stdlib (method vs func).
- **Callbacks adiadas**: razão é magnitude. Implementar
  eval de `Func` em walk context é trabalho substancial
  separado. P171+1 ou similar pode adicionar `Func`
  callback como variant adicional em `StateUpdate` enum
  (se a forma escolhida for enum, é trivial; se struct, é
  refactor).
- **Lacuna #4 não fechada totalmente em P171**: feature
  está pronta mas consumer (`Layouter::layout`) não migra
  aqui. Lacuna fica em "infraestrutura pronta, consumer
  aguarda". Quando M5 retomar, consumer pode usar
  `introspector.state_value("heading_numbering", location)`
  em vez de `state.numbering_active` legacy.
- **State na infraestrutura genérica**: estabelece padrão
  para flags runtime futuras (e.g. quando alguém precisar
  de `set_text_lang`, `set_heading_style`, etc.).
- **Padrão `metadata` reutilizado**: trabalho cascade em
  P171 segue exactamente o que P169 estabeleceu, com
  mais ficheiros tocados por causa de 2 variants em vez
  de 1.
