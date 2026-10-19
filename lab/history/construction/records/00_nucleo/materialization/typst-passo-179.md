# Passo P179 — Feature seguinte M9 (decisão em `.A`)

Continuação de M9. **Decisão diferida**: 4 candidatas
restantes têm perfis distintos. `.A` avalia
factualmente e escolhe.

**Pré-condição**: P178 concluído. M9 8/11 features.
Lacuna #7 fechada. Padrão fixpoint replicado 3 vezes
(P175/P176/P177).

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado**.
- API pública existente preservada.
- Output observable não muda; snapshot tests passam
  inalterados.
- Decisão `.A` deve identificar gate substancial se feature
  escolhida tiver pré-requisitos arquitecturais que
  inflam magnitude.

---

## Sub-passos

### .A Inventário e escolha da feature

Avaliação factual antes de decidir.

#### Candidata 1: `here()`

Stdlib que retorna `Location` actual durante eval.

**Pré-requisitos a verificar**:
- `Locator::current()` — actualmente apenas `next()`
  (confirmado P169). Adicionar é trivial — método de
  leitura sem mutação.
- `EvalContext.current_location: Option<Location>` —
  field novo. Inicializar `None`; mutar durante walk de
  cada Content que tem location.

**Trabalho estimado**:
- L0+L1 minimal de `Locator::current()`.
- Field em EvalContext + tests.
- Stdlib `here()` — consulta `ctx.current_location`.
- **Mas há problema fundamental**: `current_location`
  precisa estar disponível **durante eval**, não durante
  walk. Eval acontece *antes* de walk no pipeline. Não
  há "current location" durante eval — locations são
  atribuídas durante walk.
- **Resolução vanilla**: durante fixpoint, Locator é
  atribuído durante eval (Funcs invocadas têm Location
  associada). Cristalino sem este mecanismo —
  precisa criar.

**Magnitude real**: M-L, possivelmente L. Cresceu face
à estimativa inicial M.

**Gate substancial possível** se mecanismo "Locator
durante eval" não existe.

#### Candidata 2: `locate(callback)`

Stdlib que regista callback executado durante walk com
Location actual + Introspector.

**Pré-requisitos**:
- `Content::Locate { fn_value: Func }` — variant novo.
- Walk arm executa callback com Location actual + Introspector.
- Mas walk é puro (P163 invariante). Adicionar eval em
  walk quebra invariante.
- **Resolução tipo `state_update_with`**: walk emite
  tag `Locate(fn)` literal; `from_tags` resolve via
  `apply_func` (P173 padrão). Mas `locate` precisa
  Introspector — circular se Introspector está em
  construção.
- **Vanilla resolve via fixpoint + memoization**.
  Cristalino tem fixpoint (P174) mas sem memoization
  comemo.

**Magnitude real**: M-L.

#### Candidata 3: Bib state (lacuna #6)

Sub-store dedicado para bibliografia. Paralelo a
`MetadataStore`.

**Pré-requisitos**:
- `CounterStateLegacy` tem `bib_entries` e `bib_numbers`
  fields (P163 mencionou). Inventário em
  `m1-lacunas-captura.md` lista como "Adiar — feature
  dedicada".
- Inventário próprio necessário: o que `bib_entries`
  guarda? Como é populado? Que stdlib funcs lêem?
- **Magnitude desconhecida** sem inventário.

#### Candidata 4: `query` upgrade

Refino de P175. Stdlib `query(kind_str)` actualmente
retorna `Value::Int(count)`. Upgrade para retornar
`Vec<Location>`.

**Pré-requisitos**:
- `Value::Location(Location)` variant — confirmado
  ausente em P175 .A.
- Cascade: adicionar variant a `Value`; arms exhaustive
  em funções que matcham `Value`; serialização?
- `Value::Array(Vec<Value>)` — verificar se existe em
  cristalino.

**Magnitude estimada**: S-M. Depende de cascade `Value`
real.

#### Tabela de decisão

| Critério | here() | locate() | bib | query upgrade |
|----------|--------|----------|-----|---------------|
| Pré-reqs satisfeitos | ✗ (mecanismo eval-time location) | ✗ (eval em walk ou Introspector circular) | ? (inventário ausente) | ✗ (Value::Location ausente) |
| Magnitude estimada | M-L | M-L | ? | S-M |
| Replica padrão estabelecido | ⚠️ não — feature nova de natureza diferente | ⚠️ não — semântica única | ⚠️ não — domínio próprio | ✓ (P175 padrão) |
| Valor (lacuna fechada) | nenhuma | nenhuma | #6 | parcial #7 (já fechada) |
| Inventário em .A é factível | ✓ | ✓ | precisa P179 dedicado | ✓ |
| Decisão informada | sim | sim | NÃO sem inventário próprio | sim |

#### Regras de escolha

- **Bib excluída** — magnitude desconhecida; inventário
  próprio precisa de passo dedicado P_inventário antes
  de P_implementação. Sugerir como P180 ou P181.
- **Entre as 3 com pré-requisitos ausentes**:
  - `query` upgrade: pré-req mais localizado
    (`Value::Location` variant). Cascade pequeno.
  - `here()`: pré-req requer infrastructure nova
    (eval-time location).
  - `locate()`: pré-req requer arquitectura nova
    (eval em walk ou Introspector circular).
- **Critério: menor pré-requisito arquitectural** →
  `query` upgrade.
- **Critério alternativo: feature canónica esperada** →
  `here()`.

#### Sugestão prévia

**`query` upgrade**: pré-requisito é variant de Value;
trabalho replica padrão P175 com upgrade incremental.
Magnitude controlada. Não é feature nova mas é refino
útil.

`here()` é alternativa razoável se `query` upgrade
revelar cascade maior do que esperado em `.A`.

`locate()` rejeitada como P179 — magnitude L-XL provável
com infraestrutura nova.

`bib` rejeitada como P179 — precisa P_inventário antes.

#### Output `.A`

Notas internas + decisão registada com:
- Feature escolhida.
- Magnitude confirmada.
- Pré-requisitos verificados.
- Alternativas rejeitadas com justificação.

**Critério de saída e gate de decisão**:
- Se `query` upgrade tem pré-req localizado: prosseguir.
- Se `Value::Location` cascade é maior do que esperado:
  cláusula gate trivial — fallback para count + Locations
  via JSON ou similar.
- Se `here()` foi escolhida: confirmar mecanismo
  eval-time location é viável; senão gate substancial.
- Se descobrir feature mais adequada não listada:
  reabrir.

### .B Sub-passos da feature escolhida

Padrão genérico replicável. Detalhes em sub-passos
específicos conforme escolha em `.A`.

#### .B.query_upgrade (sugerido)

1. Verificar `Value::Array` em cristalino:
   - `grep -rn "Value::Array\|Array(Vec" 01_core/src/entities/value*.rs`.
   - Se ausente: adicionar variant. Cascade em arms
     exhaustive de Value.

2. Adicionar `Value::Location(Location)` variant:
   - L0 modify `entities/value.md` (se existe).
   - L1 modify `entities/value.rs`.
   - Hash manual via `format!("{:?}", ...)` se Location
     não deriva Hash directamente. (Location já deriva
     Hash desde P161 — confirmar.)
   - Tests co-localizados.

3. Modificar stdlib `query`:
   - Antes (P175):
     ```rust
     query(kind_str) -> Value::Int(count)
     ```
   - Depois:
     ```rust
     query(kind_str) -> Value::Array(Vec<Value::Location>)
     ```
   - Update L0 stdlib.
   - Tests: `query("heading")` retorna array com Locations.

4. Tests E2E via `introspect_to_fixpoint`:
   - Doc com 3 headings → query retorna array de 3
     Locations.
   - Doc vazio → query retorna array vazio.

5. Backward compat: tests P175 que esperavam
   `Value::Int` precisam adaptar.

#### .B.here (alternativa)

(Estrutura análoga; trabalho concentra-se em
`Locator::current()` + `EvalContext.current_location` +
mecanismo "current location" durante walk e
disponibilizado para eval da iteração seguinte do
fixpoint.)

Detalhes ficam para se for a escolha — não desenvolvo
aqui para evitar inflar passo prematuramente.

### .C Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P178 baseline (1698). Estimativa: +5 a +15
   conforme escolha.
3. `crystalline-lint`: zero violations.
4. Feature escolhida em `.A` materializada com L0+L1
   correctos.
5. Specifics da feature confirmados.
6. Walk **NÃO modificado**.
7. Snapshot tests ADR-0033 verdes.
8. Linter passa final.

### .D Encerramento

Escrever
`00_nucleo/materialization/typst-passo-179-relatorio.md` com:

- Resumo: feature escolhida em `.A` com justificação;
  materialização completa.
- Confirmação de cada verificação `.C`.
- Hashes finais de L0s novos/modificados.
- Decisões registadas em `.A`:
  - Feature escolhida.
  - Pré-requisitos verificados.
  - Alternativas rejeitadas.
- Δ tests vs baseline P178.
- **Estado de M9**: 9/11 features.
- Pendências cumulativas + actualização.
- Estado pós-passo: P179 concluído. P180 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário e escolheu feature com
   justificação.
2. Feature materializada (L0+L1+stdlib+tests).
3. Verificações `.C` passam.
4. Relatório `.D` escrito.
5. Output observable não muda.
6. M9 9/11 features.

---

## O que pode sair errado

- **`Value::Location` cascade maior do que esperado**:
  pode forçar adopção de fallback (count + JSON-like, ou
  retornar índices em vez de Locations directas).
  Cláusula gate trivial.
- **`Value::Array` ausente**: criar variant. Cascade
  adicional em arms exhaustive de Value.
- **`Location` não deriva Hash em algum sítio**: P161
  estabeleceu que sim. Confirmar; ajustar se necessário.
- **`here()` escolhida e pré-req `eval-time location`
  não viável**: gate substancial, reabrir.
- **`locate()` escolhida acidentalmente**: rejeitada na
  spec; cláusula gate trivial — recuar para `query`
  upgrade ou `here()`.
- **Bib escolhida em vez de outras**: requer inventário
  próprio. Se Claude Code escolher, sub-passo `.A` deve
  produzir inventário detalhado antes de qualquer
  implementação.
- **Backward compat tests P175 falham com `query`
  upgrade**: tests precisam adaptar de `Value::Int` para
  `Value::Array`. Ajustar.
- **Linter divergência**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: depende da escolha. S-M para `query`
  upgrade; M-L para `here()`. Decidido em `.A`.
- **Pré-condição P180**: feature seguinte M9 ou início
  de outra fase (M5 retomar, bib state inventário, M7
  refinos).
- **Cláusula gate trivial**: aplicável a decisões locais
  sobre forma de stdlib, cascade Value, mecanismos de
  conversão.
- **Estratégia caso a caso continua**: P179 é decisão
  com perfis variados nas restantes 4 candidatas. Sem
  ordem fixa.
- **`here()` e `locate()` provavelmente passos
  dedicados**: cada uma com magnitude M-L exigirá
  inventário cuidadoso. Não tentar em P179 se pré-req
  não for trivial.
- **`bib state` precisa P_inventário próprio**: lacuna
  #6 é única lacuna restante onde infraestrutura está
  ausente. Trabalho de inventário antes de
  implementação.
- **`query` upgrade fecha refino de P175**: pendência
  cumulativa "stdlib query retorna count em vez de
  array" referenciada em P175 e P176.
- **M9 status pós-P179**: 9/11 features. Restantes:
  `here()`, `locate()` ou bib state — todas com
  pré-requisitos ou magnitude desconhecida.
