## Convenções operacionais para `#[comemo::track]` em L1

Três descobertas acumuladas durante a migração. Futuros passos que
toquem tracked types devem conhecê-las.

### 1. `#[derive(Clone)]` obrigatório na struct tracked

`#[comemo::track] impl T { ... }` exige que `T` implemente `Clone`.
O comemo clona internamente para rollback de mutações tracked.
Sem o derive, erro de compilação na impl. Custo O(n) nos campos —
aceitável se struct é pequena ou tem dedup.
Descoberto: Passo 106 (materialização do `Sink`).

### 2. `Option<&str>` falha em métodos tracked

Métodos `#[comemo::track]` que recebem `Option<&str>` falham por
elisão de lifetimes não preservada pela proc-macro. Solução:
`&str` com convenção `"" = ausente`. Limitação aceite:
não distingue "ausente" de "string vazia intencional".
Descoberto: Passo 107 (extensão de `Sink::warn_note` com hint).

### 3. `TrackedMut::reborrow_mut` em descida de lifetime

`TrackedMut<'outer, T>` não encolhe automaticamente para lifetime
menor. Construir sub-estrutura com `'inner < 'outer` requer
`TrackedMut::reborrow_mut(&mut *outer)` em cada ponto de descida.
Sem isso, `E0597`.
Descoberto: Passo 109 (Engine<'a> scope-local).
