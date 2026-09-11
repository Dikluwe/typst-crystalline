# Prompt L0 — `entities/counter_update`
Hash do Código: a77f50a9

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter_update.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .6 — adaptação de nome do `CounterAction` legacy)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`CounterUpdate` é a operação a aplicar a um contador durante introspecção: avançar o contador em 1, ou fixá-lo num valor.

P161 sub-passo .1 confirmou que **já existe** um tipo similar no cristalino: `CounterAction` em `entities/counter_state.rs` (a renomear para `counter_state_legacy.rs`). Forma actual: `Step` (sem payload) | `Update(usize)`.

Per regra do P161 sub-passo .6 ("se já existir com forma diferente, **não duplicar**; adaptar nome ou estender o existente"), **adapta-se o nome**:

- Move-se a definição de `CounterAction` para um ficheiro próprio `counter_update.rs`.
- Renomeia-se o tipo para `CounterUpdate`.
- **Variantes ficam exactamente como estão**: `Step` (sem payload) e `Update(usize)`. Não se reshape para o `Set(usize)/Step(usize)` esquemático do P161 (forma vanilla); essa reshape é deferida para M9 ou para o passo que ligue Counter completo, com inventário de consumers.

`CounterStateLegacy` continua a usar `CounterUpdate` (renomeado) via `Content::CounterUpdate { key, action: CounterUpdate }` — só se actualizam imports e nome do field se necessário.

---

## Restrições Estruturais

- Camada **L1**: enum puro, sem I/O.
- Sem `Func` variant (vanilla tem `Func(Func)` para counter-via-função; cristalino adia até função-com-tracking estar pronta).
- `Clone` derivado para passar por valor em `Content::CounterUpdate` e `ElementPayload::*::counter_update`.

---

## Interface pública

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CounterUpdate {
    /// Avança o contador em 1 (flat) ou avança o nível (hierárquico).
    /// Equivalente vanilla: `CounterUpdate::Step(NonZeroUsize::new(1).unwrap())`.
    Step,

    /// Força o contador para o valor indicado.
    /// Equivalente vanilla: `CounterUpdate::Set(CounterState(smallvec![v]))`.
    Update(usize),
}
```

`Hash` derivado é necessário para ElementPayload (que vai conter `counter_update: CounterUpdate` e precisa de ser hashável para detecção de mudanças entre iterações do fixpoint em M2+).

---

## Semântica

- `Step`: avança o contador em 1 unidade (interpretação flat) ou avança o nível actual (interpretação hierárquica). A interpretação concreta é decidida pelo consumer (Layouter / introspect walk).
- `Update(value)`: fixa o contador no valor exacto.

---

## Invariantes

- Apenas duas variantes em P161 — não adicionar `Set(usize)` separado, não adicionar `Func`. Ambas as adições requerem passo dedicado com inventário.
- Variantes preservam exactamente as semânticas do `CounterAction` original — comportamento observable não muda.

---

## Consumers actuais

- `entities/content.rs::Content::CounterUpdate` variant — campo `action` muda nome do tipo de `CounterAction` para `CounterUpdate`.
- `entities/counter_state_legacy.rs` — remoção da definição local; passa a `use crate::entities::counter_update::CounterUpdate as CounterAction` (alias temporário) ou substituição nominal nos call-sites internos.

## Consumers planeados

- `entities/element_payload.rs` (P161 sub-passo .7) — campo `counter_update: CounterUpdate` em `ElementPayload::Heading` e `ElementPayload::Figure`.
- `rules/introspect.rs` walk em P162 — derivação do `CounterUpdate` a embeber em cada `ElementPayload`.

---

## Sobre paridade

Vanilla `CounterUpdate` em `lab/typst-original/crates/typst-library/src/introspection/counter.rs` linha 566:

```rust
pub enum CounterUpdate {
    Set(CounterState),       // CounterState vanilla = SmallVec<[u64; 3]>
    Step(NonZeroUsize),      // step number = level
    Func(Func),
}
```

Diferenças em P161:

- Cristalino não usa `NonZeroUsize` (apenas semântica "Step = +1"). Equivalência: vanilla `Step(NonZeroUsize::new(1)?)` ↔ cristalino `Step`.
- Cristalino `Update(usize)` corresponde grosso modo a vanilla `Set(CounterState(smallvec![value]))` — caso flat single-level. Vanilla suporta hierárquico via `SmallVec`; cristalino não codifica isso aqui (o nível é inferido pelo `step_hierarchical(key, level)` no `CounterStateLegacy`).
- Sem `Func` — adiada per ADR-0066.

Refino futuro: P162+ pode reshape para `Set(usize)` + `Step(usize)` (paridade literal) quando os call-sites estiverem preparados. Sem reservar variantes futuras neste L0.

---

## Migração face ao actual

P161 sub-passo .6 implementa a renomeação:

1. Move `pub enum CounterAction { Step, Update(usize) }` de `entities/counter_state.rs` para novo `entities/counter_update.rs` como `pub enum CounterUpdate`.
2. Update do single import em `entities/content.rs`: `use crate::entities::counter_state::CounterAction;` → `use crate::entities::counter_update::CounterUpdate;`.
3. Update do single field name em `Content::CounterUpdate { action: CounterAction }` → `{ action: CounterUpdate }` (o nome `action` mantém-se).
4. Update de qualquer outro consumer (`introspect.rs`, `layout/counters.rs`) com find-and-replace `CounterAction` → `CounterUpdate`.

Sem alteração de comportamento observable.

---

## Resultado Esperado

- `01_core/src/entities/counter_update.rs` — enum movido + tests unitários (variantes, hash, clone).
- `entities/counter_state_legacy.rs` perde a definição local de `CounterAction` (pode reter `pub use` para compat se for útil; preferível: substituição nominal directa).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-12 | Criação como `CounterAction` em `counter_state.rs` (Passo 58) | `counter_state.rs` |
| 2026-04-30 | P161 sub-passo .6: extracção e renomeação para `CounterUpdate` em ficheiro próprio | `counter_update.rs`, `counter_state_legacy.rs`, `content.rs`, `counter_update.md` |

---

## Proposta P1148 — reshape de paridade (AGUARDA GATE ADR-0127)

### Medição antes da decisão

No vanilla ratificado
`a51e02804:crates/typst-library/src/introspection/counter.rs:459-517,567-576`,
`counter.step` aceita `level: NonZeroUsize = 1` e produz
`CounterUpdate::Step(level)`; `counter.update` aceita estado hierárquico ou
função e produz `CounterUpdate::Set(CounterState)` ou `CounterUpdate::Func`.
As sondas nos dois binários ratificados também confirmaram que `step` e
`update` são fields públicos tanto estáticos quanto de instância.

O contrato cristalino vigente acima, confirmado em
`01_core/src/entities/counter_update.rs`, reduz essas formas a `Step` sem
payload e `Update(usize)`. Logo a hipótese de P1148 ser somente glue foi
refutada: `level: 2`, arrays e callbacks não podem atravessar o domínio atual.

### Contrato proposto

Após confirmação explícita do gate, esta secção substitui a interface e as
invariantes históricas de P161:

```rust
pub enum CounterUpdate {
    Set(Vec<usize>),
    Step(NonZeroUsize),
    Func(Func),
}
```

- `Set` preserva todos os componentes do array da linguagem; inteiro vira
  vector de um componente.
- `Step(level)` preserva o nível positivo. Zero, negativo e tipos errados são
  rejeitados antes da construção.
- `Func` preserva o callback para aplicação ao estado corrente durante a
  construção do introspector, usando o mesmo `Engine + EvalContext` já
  transportado para `StateUpdate::Func`.
- `Clone`, igualdade e hashing exigidos pelos payloads permanecem disponíveis.
  Para `Func`, devem seguir a identidade/equivalência já definida por `Func`;
  não executar callbacks para comparar ou calcular hash.
- Todos os producers automáticos antes escritos como `Step` passam a
  `Step(NonZeroUsize::MIN)`; `Update(v)` passa a `Set(vec![v])`.

Esta é mudança de enum público e, portanto, fica parada no ponto 3 do
protocolo de nucleação até confirmação do dono. Nenhum teste ou código P1148
é autorizado por esta proposta antes do gate.

## P1344 — acesso owner-local ao carrier da ação `Func`

### Medição anterior à decisão

O enum vigente já materializado contém `CounterUpdate::Func(Func)`. O blocker
P1343 R3 demonstrou que o helper dessa unidade não pode ser declarado dentro
de `impl Func`: além de ser Rust inválido naquela posição, colocaria lógica de
`CounterUpdate` no owner errado. `entities/counter_update.rs` já é o consumer
1:1 desta especificação e contém o `impl CounterUpdate` real.

### Decisão vigente

Somente sob `cfg(p1339_observation)`, `CounterUpdate` expõe ao crate um helper
owner-local read-only que:

- casa exclusivamente a variante real `CounterUpdate::Func(function)`;
- consulta e devolve por clone/borrow o mesmo carrier privado já anexado ao
  `Func`, sem criar ocorrência ou executar callback;
- permite aos callsites H10/H12 provar continuidade da ação real;
- devolve ausência para `Set` e `Step`, sem inventar identidade;
- não altera variantes, construtores, Eq, Hash, Debug ou API normal;
- não usa side table, trait de extensão, macro, impl não local, nome, output,
  igualdade ou endereço para reconstrução.

Sem `p1339_observation`, o helper e qualquer referência aos tipos de
observação não existem. É transporte interno de teste, sem mudança pública,
default, compatibilidade ou fase conforme ADR-0127.
