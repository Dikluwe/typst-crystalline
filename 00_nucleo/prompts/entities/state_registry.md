# Prompt L0 — `entities/state_registry`
Hash do Código: d32fc8b7

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_registry.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .E — sub-store de M9 state feature)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateRegistry` é o sub-store de runtime mutable state para `Introspector`. P171 (M9 sub-passo 3) — feature `state(key, init)` + `state_update(key, value)`.

Vanilla equivalente: `lab/typst-original/.../introspection/state.rs`. Cristalino isola num sub-store dedicado por simetria com `LabelRegistry`/`CounterRegistry`/`MetadataStore`.

---

## Estrutura

`HashMap<String, Vec<(Location, Value)>>` — para cada key, lista ordenada de pares (Location, value). Init é a primeira entrada (cronologicamente). Updates posteriores adicionam ao Vec.

Decisão (P171.A.3): escolhida `HashMap` por:
- Lookup por key é O(1).
- Vec interno permite append em O(1).
- `value_at(key, location)` é O(n) onde n = updates da key (aceitável; n é tipicamente pequeno).

Alternativas consideradas: `BTreeMap<Location, ...>` (ordenação automática mas lookup por key complicado), `Vec<(Location, key, Value)>` (sort needed antes de lookup).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn init` / `update` / `apply_update` durante construção em `from_tags`).
- `Clone` derivado.

---

## Interface pública

```rust
use crate::entities::location::Location;
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct StateRegistry { /* HashMap<String, Vec<(Location, Value)>> */ }

impl StateRegistry {
    pub fn empty() -> Self;
    pub fn value_at(&self, key: &str, location: Location) -> Option<&Value>;
    pub fn final_value(&self, key: &str) -> Option<&Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn init(&mut self, key: String, init: Value, location: Location);
    pub(crate) fn update(&mut self, key: String, value: Value, location: Location);
}
```

**P173**: o método de conveniência `apply_update(key, StateUpdate, location)`
foi removido. Razão: `StateUpdate::Func` requer `Engine + EvalContext`
para eval, que não estão neste sub-store. O match sobre `StateUpdate`
vive agora em `from_tags::from_tags` onde Engine está disponível;
`StateRegistry` expõe apenas as primitivas `init` + `update`.

---

## Semântica

- `empty()`: registry vazio.
- `init(key, init, location)`: regista valor inicial. Apenas a **primeira chamada para cada key** é considerada — segundo init é ignorado (decisão do cristalino; ver bloco P1031).
- `update(key, value, location)`: regista update. **Se key não foi inicializada, update é ignorado** (defensive; ver bloco P1031).

> **Correcção P1031 — duas alegações sobre o vanilla, ambas sem base na fonte.**
>
> A redacção anterior atribuía ao vanilla dois comportamentos: *"segundo init é ignorado
> (paridade vanilla)"* e *"vanilla geraria erro"* para update sem init. Nenhum dos dois se
> encontra no vanilla ratificado (`e0e8ca4d`) — e a razão é estrutural: **no Typst, `state`
> não tem "registo de init" nem "update sem init".** `state(key, init)` é um construtor que
> carrega o valor inicial consigo (`crates/typst-library/src/introspection/state.rs:192-194`:
> *"The key that identifies the state."* / *"The initial value of the state."*), pelo que
> todo `update` parte de um estado já definido por construção. As duas situações que estas
> linhas descrevem não existem na linguagem.
>
> **O que a documentação diz, e que é o que importa** (doc comments `#[func]`,
> `state.rs:300-315`, publicado em `typst.app/docs/reference/introspection/state/`):
>
> *"State is a part of your document and runs like a thread embedded in the document
> content. The value of a state is the result of all state updates that happened in the
> document up until that point. That's why `state.update` returns an invisible sliver of
> content that you need to return and include in the document — a state update that is not
> "placed" in the document does not happen, and "when" it happens is determined by where you
> place it."*
>
> Ou seja: o observável é o valor num ponto do documento, determinado pela posição dos
> updates. É contra isso que a paridade se mede, não contra a mecânica do registry.
>
> **Reclassificação**: as duas regras acima passam de "paridade vanilla" a **decisões
> defensivas do cristalino** para estados que a linguagem não pode produzir. Não são achados
> de código; são achados de enquadramento, corrigidos aqui.
- `value_at(key, location)`: encontra último (key-value) pair com `loc <= location` na ordem do Vec; retorna value ou None.
- `final_value(key)`: retorna o último value registado (init se nenhum update, ou último update aplicado).

**P173**: `apply_update` removido. Match sobre `StateUpdate` (Set vs Func) vive em `from_tags` onde `Engine` está disponível para eval de Func.

---

## Algoritmo `value_at`

```rust
fn value_at(&self, key: &str, location: Location) -> Option<&Value> {
    let history = self.inner.get(key)?;
    history
        .iter()
        .filter(|(loc, _)| loc.as_u128() <= location.as_u128())
        .last()
        .map(|(_, v)| v)
}
```

Comparação por `as_u128()` directa — Locations são monotonicamente crescentes pelo `Locator` (P161), portanto a ordem de inserção é a ordem cronológica.

---

## Tests obrigatórios (sub-passo .E P171)

- `empty()` retorna `None` em qualquer query.
- `init` only → `value_at` em qualquer location ≥ init_loc retorna init.
- `update` após `init` aplica no ponto correcto (antes do update: init; depois: novo).
- Múltiplos updates em sequência: cada location resolve para o valor correcto.
- Keys distintas isoladas.
- `update` sem `init` é ignorado.
- Segundo `init` para mesma key é ignorado.

---

## Consumers actuais

Nenhum no momento da criação. Consumido em P171 .F por `rules/introspect/from_tags.rs` arms `ElementPayload::State` e `ElementPayload::StateUpdate`.

## Consumers planeados

- `entities/introspector.rs::TagIntrospector.state` field — exposição via `state_value` / `state_final_value`.
- M5 retorno: `Layouter` pode consumir `state` para resolver flags como `numbering_active("heading")`.
- M9+ features que dependem de state runtime.

---

## Sobre paridade

Vanilla resolve o estado por **iterações de layout** repetidas até convergir. Cristalino simplifica para single-pass linear (sem fixpoint M7). Suficiente para casos comuns onde `state.update` aparece no mesmo documento que `state.value_at`. Refino futuro (M9+) pode adicionar fixpoint quando consumers reais exigirem.

> **Fonte de paridade (P1031)** — a afirmação anterior (*"Vanilla `state.rs` armazena state
> via fixpoint comemo"*) nomeava o mecanismo errado: `comemo` é a camada de memoização, não
> o resolvedor de estado. O mecanismo real é a **iteração de layout**, e está **documentado
> na linguagem** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/state.rs:339-349`, doc comment do parâmetro de
> `state.update`:
>
> *"When updating the state based on its previous value, you should prefer the function form
> instead of retrieving the previous value from the context. This allows the compiler to
> resolve the final state efficiently, **minimizing the number of layout iterations
> required**. In the following example, `{fill.update(f => not f)}` will paint odd items in
> the bullet list as expected. However, if it's replaced with
> `{context fill.update(not fill.get())}`, then **layout will not converge within 5
> attempts**, as each update will take one additional iteration to propagate."*
>
> E `state.rs:184-188`: *"In general, you should try not to generate state updates from
> within context expressions. […] Sometimes, it cannot be helped, but in those cases it is
> up to you to ensure that the result converges."*
>
> **Consequência para o scope-out**: a convergência iterativa **é superfície de linguagem
> observável** (o utilizador vê o erro de não-convergência, e a página de referência de
> iterações do compilador é pública). O single-pass do cristalino não é, portanto, apenas
> uma simplificação mecânica invisível — é uma restrição de comportamento. Continua
> legítimo como scope-out declarado, mas fica registado com essa natureza e não como
> "mecânica interna". Frase corrigida acima.

---

## Resultado Esperado

- `01_core/src/entities/state_registry.rs` — struct + 8 métodos + 7 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: sub-store de runtime mutable state para Introspector M9 | `state_registry.rs`, `state_registry.md` |
| 2026-04-29 | P173: removido `apply_update` — match Set/Func vive em `from_tags` onde `Engine` está disponível | `state_registry.rs`, `state_registry.md` |
