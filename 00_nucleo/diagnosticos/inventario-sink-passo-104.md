# Inventário `Sink` — Passo 104.A

Data: 2026-04-23.

---

## Parte 1 — Estado actual de `Sink`

### Definição (pré-Passo 104)

Em `01_core/src/entities/world_types.rs:385-401`:

```rust
/// Colector de diagnósticos durante eval().
///
/// Stub — o original usa EcoVec (ecow), Introspection, Value, Styles.
/// ADR-0017: implementação real quando esses tipos migrarem.
#[derive(Hash)]
pub struct Sink(());

impl Sink {
    pub fn new() -> Self { Self(()) }
}

impl Default for Sink {
    fn default() -> Self { Self::new() }
}

#[comemo::track]
impl Sink {}
```

- **Forma**: stub puro `Sink(())`, sem campos.
- **API**: apenas `new` / `default`. Nenhum método tracked.
- **`#[comemo::track]`**: presente mas vazio — preparado para
  integração futura com memoização comemo.

### `TrackedMut<Sink>` no `eval`

`01_core/src/rules/eval/mod.rs:150`:

```rust
pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,          // ← recebido mas não usado
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module>
```

`_sink` é recebido mas prefixado `_` (não usado). Nenhuma
propagação ao longo da cadeia eval. Nenhum `into_diagnostics`
chamado.

---

## Parte 2 — Candidatos a consumidor piloto

### DEBT-49 sites

Grep por sinais de silenciamento em `01_core/src/`:

| Ficheiro | Linha | Contexto |
|----------|------:|----------|
| `rules/eval/rules.rs` | 294 | `// DEBT: propriedades de #set text não suportadas ... silenciosamente ignoradas.` |

**Apenas 1 site** com comentário DEBT explícito. Outros silenciamentos
são implícitos via `_ => {}` mas não têm mensagem de warning óbvia
— estão no escopo do DEBT-49 completo, não deste piloto.

### Ranking de candidatos

1. **`rules.rs:294` (DEBT-49 pilot)** — `#set text(font: ...)` e
   outras propriedades não suportadas.
   - Ficheiro: `rules/eval/rules.rs`.
   - Função: `eval_set_rule`.
   - Profundidade de propagação desde `eval`:
     `eval → eval_markup → eval_expr → eval_set_rule` = **3 níveis**.
   - Veredicto: **excede o gate "1 função"**.

### Nenhum candidato com propagação ≤ 1 nível

`_sink: TrackedMut<Sink>` está em `eval()`; os silenciamentos estão
3+ níveis abaixo. Propagar sink como parâmetro `&mut Sink` por
`eval_markup`, `eval_expr`, e `eval_set_rule` é exactamente a
"propagação larga" que o spec identifica como trabalho do DEBT-49
completo.

### Decisão

**Não migrar consumidor neste passo**. Gate triggered —
reescolhemos para: materialização + tests, sem migrar consumer.

A cadeia de propagação `eval → eval_markup → eval_expr →
eval_set_rule` é a mesma arquitectura aplicada nos Passos 92–98
para `route`, `styles`, `show_rules`, `active_guards`,
`current_file`, `figure_numbering`. Adicionar `sink` lá é a
5ª aplicação da ADR-0036, trabalho próprio do DEBT-49.

---

## Parte 3 — Saída dos warnings

Grep `into_diagnostics` em `01_core/` e `03_infra/`:

```
(zero matches)
```

**Hoje**, os warnings não têm caminho para o caller L3/CLI:

- `eval()` retorna `SourceResult<Module>` — `Vec<SourceDiagnostic>`
  só para erros via `Err`.
- `_sink` entra como `TrackedMut<Sink>` e sai da stack sem ser
  lido.
- L3 (`03_infra/src/world.rs`) não chama `into_diagnostics`.

Lacuna documentada. **Não cria caminho L1→L3 neste passo** — é
trabalho independente (novo DEBT-51 se for necessário destacar).

---

## Parte 4 — Compatibilidade com `#[comemo::track]`

`#[comemo::track] impl Sink {}` é actualmente vazio. Adicionar
métodos ao bloco tracked tem restrições: argumentos têm de ser
`Send + Sync + Hash` (ou similar — depende da versão comemo).

`SourceDiagnostic` não deriva `Hash`. `warn(diag: SourceDiagnostic)`
como método tracked seria problemático.

**Decisão**: manter `#[comemo::track] impl Sink {}` vazio. Os
métodos `warn`/`into_diagnostics`/`is_empty` vivem num bloco
`impl Sink` **não-tracked** — acessíveis via `&mut Sink` directo,
não via `TrackedMut<Sink>`. Quando a integração comemo for
accionada, os métodos serão movidos caso-a-caso.

Esta decisão preserva a compatibilidade com o `eval` actual sem
forçar integração prematura.

---

## Recomendação

1. **Materializar `Sink`** em `01_core/src/entities/sink.rs` (ficheiro
   novo) com campos `diagnostics: Vec<SourceDiagnostic>` + `seen:
   FxHashSet<(Span, String)>` para dedup. ADR-0018 autoriza `FxHashSet`
   em L1.
2. **API em `impl Sink`** (não-tracked):
   - `new() / default()`.
   - `warn(&mut self, diag: SourceDiagnostic)`.
   - `is_empty(&self) -> bool`.
   - `into_diagnostics(self) -> Vec<SourceDiagnostic>`.
3. **Manter `#[comemo::track] impl Sink {}` vazio** — integração
   comemo adiada.
4. **Remover stub** de `world_types.rs`. Re-export via
   `entities/mod.rs`.
5. **Sem migração de consumidor** — gate triggered. DEBT-49 fica
   pendente mas agora tem Sink real para migrar quando for
   accionado.
6. **Tests unitários** (7): construção, warn, dedup por
   (span, message), dedup não aplica quando diferem, ordem
   preservada, is_empty, into_diagnostics consome.
7. **ADR-0042** documenta API, dedup, decisão comemo, decisão
   sobre consumer (adiado).
8. **DEBT-49 actualizado** — Sink existe; migração em massa
   continua pendente mas agora tem destino definido.
9. **DEBT-51 aberto (opcional)** — "Warnings do Sink não chegam
   a L3/CLI". Documenta a lacuna.
