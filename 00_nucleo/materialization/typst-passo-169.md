# Passo P169 — Primeira feature Introspection vanilla (M9 sub-passo 1)

Início de M9 — features Introspection vanilla. Decisão pós-P167:
todas as 11 features antes de M5 retomar para os consumers
bloqueados. Estratégia: **features simples primeiro**;
features grandes (state, query, CounterKey hierárquico) depois.

**Decisão pendente em `.A`**: qual feature simples começar.
Candidatas avaliadas:

- `metadata(value)` — Content variant + sub-store + stdlib.
  Auto-contida.
- `here()` — função stdlib retornando Location. Sem Content
  variant. Consumer de mecanismo existente (`Locator`).
- `position(label)` — stdlib. Consumer de `LabelRegistry`
  + position lookup (`Position` ainda não populado em M3 —
  pode ser limitação).
- `locate(fn)` — Content variant + walk arm + stdlib. Tem
  dependência implícita em fixpoint (M7) — provavelmente
  rejeitada por agora.

**Pré-condição**: P168 concluído. M5 em pausa. `Introspector`
trait + `TagIntrospector` impl + sub-stores existentes.

**Restrições**:
- Não modificar consumers de M5 (figure-ref já migrado;
  outros bloqueados).
- Não eliminar `CounterStateLegacy` (M6).
- Output observable não muda; snapshot tests passam
  inalterados.
- Estratégia "simples primeiro": rejeitar features grandes
  (`state(key, init)`, `query()`, `CounterKey` enum
  hierárquico) neste passo.

---

## Sub-passos

### .A Inventário e escolha da primeira feature

Avaliação factual antes de decidir.

1. **Confirmar estado de pré-requisitos** das 4 candidatas:

   #### `metadata(value)`
   - **Content variant**: actualmente ausente. Adicionar
     `Content::Metadata { value: Value }` (variants 56→57).
   - **ElementPayload variant**: adicionar
     `ElementPayload::Metadata { value }`.
   - **`extract_payload` arm**: adicionar.
   - **Sub-store**: criar `MetadataStore` em
     `entities/metadata_store.rs` (registado como pendência
     adiada em P165).
   - **Stdlib**: `metadata(value)` — função pública que
     constrói `Content::Metadata`.
   - **Method no `Introspector` trait**: adicionar
     `query_metadata(&self) -> Vec<Value>` ou similar.
   - **Dependências**: nenhuma além de tipos existentes.

   #### `here()`
   - **Content variant**: nenhum. `here()` retorna `Location`
     directamente.
   - **Stdlib**: função pública `here()` que retorna
     `Location` actual durante walk/eval.
   - **Mecanismo**: precisa de "Location actual" — vanilla
     usa `Locator::current()`. Cristalino actual: confirmar
     se `Locator` expõe método `current()` ou apenas
     `next()`. Se apenas `next()`, adicionar `current()`.
   - **Dependências**: pode precisar de mecanismo de
     contexto eval (qual a Location actual durante eval de
     stdlib call). Verificar se cristalino tem isto.

   #### `position(label)`
   - **Content variant**: nenhum. `position()` retorna
     `Position` por label.
   - **Stdlib**: função pública `position(label) -> Position`.
   - **Sub-store**: `LabelRegistry::lookup` (existe em P165).
   - **Limitação conhecida**: `Position` ainda não tem
     mecanismo de população em M3. `Introspector::position_of`
     retorna sempre `None` (P165 decisão registada).
     `position()` retornaria `None`/erro até layout integrar.
     **Provavelmente ainda não viável**.

   #### `locate(fn)`
   - **Content variant**: adicionar `Content::Locate { fn_value: Func }`.
   - **Walk arm**: complicado — `fn` pode consultar
     `Introspector` que ainda não convergiu.
   - **Dependência implícita**: fixpoint (M7).
   - **Provavelmente rejeitada** para "primeira feature
     simples".

2. **Inspeccionar cristalino** para cada candidata:
   - `Locator::current()` existe?
     `grep -rn "fn current" 01_core/src/entities/locator.rs`.
   - Mecanismo de "Location actual" durante eval de stdlib
     existe? Procurar `EvalContext` ou similar para field
     `current_location`.
   - `Position` map em `TagIntrospector` está vazio (P165
     confirmou).
   - `Func` value type em cristalino: existe e é evaluable?

3. **Aplicar regras de prioridade**:

   | Critério | metadata | here() | position() | locate() |
   |----------|----------|--------|------------|----------|
   | Auto-contida | ✓ | ✗ (precisa eval ctx) | ✗ (precisa Position) | ✗ (precisa fixpoint) |
   | Adiciona Content variant | Sim | Não | Não | Sim |
   | Sub-store novo | Sim (MetadataStore) | Não | Não | Não |
   | Pré-requisitos satisfeitos | ✓ | Verificar em .A.2 | ✗ (Position ausente) | ✗ (M7 ausente) |
   | Tamanho | M | S-M | S | M-L |

4. **Decisão**:
   - Default sugerido: `metadata(value)` — auto-contida,
     pré-requisitos satisfeitos, validar ciclo completo
     de feature nova.
   - Alternativa: `here()` se P169 .A.2 confirmar que
     `Locator::current()` é trivial e `EvalContext` tem
     mecanismo de location actual.
   - Rejeitar: `position(label)` (Position ausente em M3),
     `locate(fn)` (fixpoint ausente em M7).

Output: notas internas + decisão registada com justificação.

**Critério de saída e gate de decisão**:
- Se `metadata` é viável e há justificação clara:
  prosseguir com `metadata`.
- Se `here()` tem mecanismo simples (e.g. `Locator::current()`
  trivial): pode ser escolhido como alternativa.
- Se ambos têm complicações inesperadas: gate substancial,
  reabrir.

### .B Sub-passos da feature escolhida

A implementação concreta depende da feature escolhida em
`.A`. Cada feature tem trabalho próprio.

#### .B.metadata Implementar `metadata(value)`

Apenas se `.A` escolheu `metadata`.

1. **Adicionar variant a `Content`**:
   - L0 `00_nucleo/prompts/entities/content.md` (existente):
     adicionar entrada para `Metadata { value: Value }`.
   - L1 `01_core/src/entities/content.rs`: adicionar variant.
   - Tests co-localizados: construir `Content::Metadata`,
     igualdade, hash determinístico.

2. **Adicionar variant a `ElementPayload`**:
   - L0 `00_nucleo/prompts/entities/element_payload.md`:
     adicionar entrada para `Metadata { value }`.
   - L1: adicionar variant.

3. **Adicionar arm a `extract_payload`**:
   - L0 `00_nucleo/prompts/rules/introspect/extract_payload.md`:
     documentar arm novo.
   - L1: adicionar arm `Content::Metadata { value } => Some(ElementPayload::Metadata { value: value.clone() })`.
   - Tests: extract_payload de Content::Metadata retorna
     payload correcto.

4. **Adicionar variant a `ElementKind`**:
   - L0 `00_nucleo/prompts/entities/element_kind.md`:
     adicionar `Metadata`.
   - L1: adicionar variant.

5. **Adicionar arm a `is_locatable`**:
   - L0 `00_nucleo/prompts/rules/introspect/locatable.md`:
     documentar arm novo (`Metadata => true`).
   - L1: adicionar arm. Match continua exaustivo.
   - Tests: `is_locatable(&Content::Metadata {..})` retorna
     `true`; invariante com `extract_payload` continua.

6. **Criar `MetadataStore`**:
   - L0 `00_nucleo/prompts/entities/metadata_store.md`.
   - L1 `01_core/src/entities/metadata_store.rs`:
     `pub struct MetadataStore { values: Vec<Value> }` com
     método `add` (pub(crate)) e `query() -> &[Value]`.
   - Tests co-localizados.

7. **Estender `from_tags`**:
   - Adicionar arm para `ElementPayload::Metadata` que
     popula `MetadataStore`.
   - Tests: tag stream com metadata produz `MetadataStore`
     populado.

8. **Estender `Introspector` trait**:
   - L0 `00_nucleo/prompts/entities/introspector.md`:
     adicionar método `query_metadata(&self) -> Vec<Value>`.
   - L1: adicionar método ao trait + impl em
     `TagIntrospector` (delega para `MetadataStore`).

9. **Stdlib `metadata(value)`**:
   - Identificar onde stdlib functions são registadas
     no cristalino (provavelmente `01_core/src/rules/stdlib/`).
   - Adicionar função `metadata(value)` que retorna
     `Content::Metadata { value }`.
   - Tests: avaliar `metadata("hello")` produz Content
     correcto.

10. **Update `Layouter`** (mínimo):
    - `Content::Metadata` deve ser **invisível em layout**
      (zero-size, sem caixa). Verificar walk arm em layout
      retorna `None`/`Frame::empty()` para este variant.

11. **Tests E2E**:
    - Documento com `#metadata("foo")` + query no
      Introspector retorna `["foo"]`.
    - Output observable do documento não muda (metadata
      não tem caixa visível).

**Critério de saída** (.B.metadata):
- 8 L0s actualizados/criados.
- 8 L1s actualizados/criados.
- Stdlib registada.
- Tests passam.
- Linter passa.

#### .B.here Implementar `here()`

Apenas se `.A` escolheu `here()`.

(estrutura análoga, mais leve — sem Content variant, sem
sub-store novo, mas precisa de mecanismo "Location actual"
no eval context).

Detalhes ficam para se for a escolha — não desenvolvo aqui
para não inflar o passo prematuramente.

### .C Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P168 baseline (1597). Para `metadata`: provavelmente
   +15 a +25 tests.
3. `crystalline-lint`: zero violations.
4. Feature escolhida em `.A` materializada com L0+L1
   completos.
5. Se `metadata`:
   - `Content::Metadata` existe.
   - `MetadataStore` existe.
   - `Introspector::query_metadata` existe.
   - Stdlib `metadata(value)` registada.
6. Snapshot tests de paridade ADR-0033 passam inalterados.
7. Linter passa em verificação final.

### .D Encerramento

Escrever
`00_nucleo/materialization/typst-passo-169-relatorio.md` com:

- Resumo: feature escolhida em `.A` com justificação;
  materialização completa.
- Confirmação de cada verificação .C.
- Hashes finais de L0s novos/modificados (preenchidos pelo
  linter).
- Decisões registadas em `.A`:
  - Feature escolhida.
  - Pré-requisitos verificados.
  - Alternativas rejeitadas com justificação.
- Δ tests vs baseline P168.
- **Estado de M9**: 1/11 features materializadas.
- Pendências cumulativas + actualização (lacuna específica
  resolvida por esta feature, se aplicável).
- Estado pós-passo: P169 concluído. P170 desbloqueado —
  segunda feature M9.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário e escolheu feature com
   justificação.
2. Feature materializada (L0+L1+stdlib+tests).
3. Verificações `.C` passam.
4. Relatório `.D` escrito.
5. Output observable não muda.
6. M9 1/11 features concluída.

---

## O que pode sair errado

- **`metadata` precisa stdlib registry que não existe ou é
  diferente**: cristalino pode ter forma idiomática própria.
  Cláusula gate trivial: adaptar ao formato existente.
- **`here()` precisa de eval context que não tem field de
  location actual**: gate substancial. Pode forçar criação
  de campo novo em `EvalContext` — passo cresce de S para
  M-L. Reabrir.
- **`MetadataStore` colide com algum tipo existente**:
  improvável (P165 confirmou ausência), mas verificar em
  `.A`.
- **Variant novo em `Content` (`Metadata`) força revisão
  em todos os matches exaustivos**:
  - `is_locatable` (cobre).
  - `extract_payload` (cobre).
  - `hash_content` (precisa novo arm — adicionar).
  - Layouter walk arm (precisa novo arm que retorna
    invisível).
  - Outros consumers de `Content`. Lista completa:
    `grep -rn "match content\|match c {" 01_core/src/`.
  - Compilador força revisão; cada arm é trabalho extra.
- **Snapshot tests detectam mudança observable**: improvável
  (`metadata` é zero-size por design), mas verificar.
  Se houver, investigar — pode ser que walk arm em layout
  está a renderizar acidentalmente.
- **Linter divergência L0↔L1 com tantos ficheiros tocados**:
  ajustar conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M-L se `metadata`. S-M se `here()`. Decidido
  em `.A`.
- **Precondição P170**: feature seguinte de M9. Estratégia
  "simples primeiro" continua. Lista provável (sem ordem
  fixada): outras features simples (`here()`, `position()`
  com workaround), depois `state(key, init)`, `query()`,
  `CounterKey` hierárquico.
- **`metadata` como base para outras features**: alguns
  designs vanilla constroem `state` sobre `metadata`. Se
  `metadata` for primeira, `state` em M9+1 ou +2 fica mais
  simples.
- **Cláusula gate trivial**: aplicável a decisões locais
  sobre forma idiomática (registry de stdlib, location
  current).
- **`Content::Metadata` zero-size**: requisito explícito
  para preservar output observable. Layouter arm deve
  retornar frame vazio.
- **`MetadataStore` em `entities/`**: confirma que
  `entities/` continua a ser o local certo para sub-stores
  (P165 estabeleceu este padrão).
