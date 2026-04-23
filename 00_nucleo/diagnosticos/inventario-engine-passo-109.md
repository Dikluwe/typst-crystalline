# Passo 109.A — Inventário Engine<'a>

**Data**: 2026-04-23
**Input**: assinaturas actuais de `eval_*` em `01_core/src/rules/eval/`
(estado pós-Passo 107).

---

## Parte 1 — Lifetimes exactos de cada campo

Assinaturas actuais (ex: `eval_set_rule`):

```rust
pub(super) fn eval_set_rule<'r>(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value>
```

`EvalContext<'w>` contém `world: &'w dyn World` (trait **não-tracked**
no cristalino — não é `Tracked<dyn World>` do comemo como no vanilla).

| Parâmetro | Tipo exacto | Lifetime |
|-----------|-------------|----------|
| `world` (via ctx) | `&'w dyn World` | `'w` (campo de EvalContext) |
| `route` | `Tracked<'r, Route<'r>>` | `'r` (argumento explícito) |
| `styles` | `&mut StyleChain` | elidido |
| `show_rules` | `&mut Arc<[ShowRule]>` | elidido |
| `active_guards` | `&mut Vec<RuleId>` | elidido |
| `current_file` | `FileId` | nenhum (Copy) |
| `figure_numbering` | `&mut Option<String>` | elidido |
| `sink` | `&mut TrackedMut<'_, Sink>` | elidido (externo) + elidido (interno) |

Todos os `&mut` elididos reborrow no site de chamada; lifetime
concreto é definido pelo Rust (NLL) como o mais curto que satisfaz
os usos.

---

## Parte 2 — Compatibilidade entre lifetimes

### Observação chave

Os lifetimes `'w` (world), `'r` (route), e os `&mut` elididos
podem ser **unificados** num único lifetime `'a` — a vida útil
da frame `eval()`.

Porquê: em `eval()` público, todos os borrows derivam do stack
frame; todos vivem enquanto `eval()` corre.

Detalhe sobre `route`: `Tracked<'r, Route<'r>>` — o `'r` corresponde
ao stack slot onde `Route` vive. Em `eval()` inicial é a frame toda;
em `apply_closure` / `eval_module_include`, um **novo** `Route` é
criado com `Route::extend(...)` e o `.track()` dele tem lifetime
**mais curto**. Consequência: dentro desses dois sítios, o `engine`
precisa de ser reconstruído com o novo `route` (ver forma da
migração).

### Grupo 1 (compatíveis, entram em Engine)

Todos os 8 campos cabem em `Engine<'a>` com lifetime uniforme `'a`
**se aceitarmos reconstruir Engine nos sítios de scope**:

- `world`
- `route`
- `styles`
- `show_rules`
- `active_guards`
- `current_file`
- `figure_numbering`
- `sink`

### Grupo 2 (incompatíveis)

**Nenhum.** Todos os campos cabem em um único `'a`.

### Conflitos esperados

**Sítios que mudam um ou mais campos** e requerem reconstrução
local de `Engine`:

1. `Expr::CodeBlock` em `eval_expr` — `local_styles`,
   `local_show_rules`.
2. `Expr::ContentBlock` em `eval_expr` — `local_styles`.
3. `eval_strong` / `eval_emph` / `eval_heading` em `markup.rs` —
   `local_styles` via `styles.push(delta)`.
4. `apply_closure` em `closures.rs` — `child_route`, `local_styles`.
5. `eval_module_include` em `modules.rs` — `child_route`,
   `child_current_file`.

**Total**: 5-7 sítios. Cada um faz `let mut local_engine = Engine
{ ..., styles: &mut local_styles, ..reborrow restantes }` antes de
passar `&mut local_engine` ao filho.

Não é um conflito de lifetime propriamente — é **reconstrução
ordinária** de struct agregadora.

### Detalhe técnico: `TrackedMut<'_, Sink>`

`sink: &'a mut TrackedMut<'b, Sink>` teria **dois** lifetimes. Para
manter `Engine<'a>` com um só, declaramos:

```rust
pub sink: &'a mut TrackedMut<'a, Sink>,
```

A reborrow no call site pode exigir `TrackedMut::reborrow_mut` se o
compilador não conseguir encurtar automaticamente a lifetime
interna. Na prática, em Rust 2021, `&mut *engine.sink` funciona
como reborrow com lifetime encurtado — o tipo resultante é
`&'short mut TrackedMut<'a, Sink>`, o que colide com o `'short`
esperado pelo child `Engine<'short>`. Mitigação: usar
`TrackedMut::reborrow_mut(&mut *engine.sink)` explicitamente no
site de reconstrução. Verificar na implementação.

---

## Parte 3 — Sítios de construção

`eval()` público em `01_core/src/rules/eval/mod.rs:150`:

```rust
pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    // ...
    let route = Route::root().with_id(source.id());
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();
    let current_file = source.id();
    let mut figure_numbering: Option<String> = None;
    // ...
    // Engine a construir aqui:
    let mut engine = Engine {
        world,
        route: route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file,
        figure_numbering: &mut figure_numbering,
        sink: &mut sink,
    };
    eval_markup(root, &mut scopes, &mut ctx, &mut engine)?;
}
```

Também: `eval_for_test_with_limits` em `rules/eval/tests.rs:38`
— helper de teste que replica o mesmo pattern. Actualizar para
construir Engine localmente.

---

## Parte 4 — Forma da migração

### Avaliação das 3 alternativas

**Big-bang**:
- Prós: forma coerente — todas as assinaturas migram de uma vez.
  Testes só validam no fim. Diff grande mas conceptualmente único.
- Contras: se algo falha no meio, não há checkpoint intermédio.
- Viabilidade: 24 funções × ~3 call sites = ~70 call sites + 5-7
  sites de reconstrução local. Gerível.

**Incremental-por-campo** (1 campo de cada vez):
- Prós: cada sub-passo compila + testa isolado.
- Contras: 8 sub-passos. Cada sub-passo mexe em quase todos os
  ficheiros para adicionar UM campo ao Engine incompleto — alto
  custo de boilerplate temporário (campos no Engine + continuar
  a passar o mesmo campo como parâmetro até absorver).
- Viabilidade: alto custo / baixo ganho para este caso.

**Incremental-por-função**:
- Prós: Engine completo desde o início; migra função a função.
- Contras: funções convertidas não chamam funções não-convertidas
  sem adaptador — cross-calls quebram imediatamente. Precisava de
  adaptadores temporários (wrappers que destroem o Engine antes de
  chamar a função antiga).
- Viabilidade: cross-calls inviabilizam (e.g., `eval_expr` chama
  `eval_set_rule` que chama `eval_expr` de volta — os três têm de
  migrar juntos).

### Decisão: **big-bang**

Razões:
1. As funções cross-chamam-se; conversão parcial cria adaptadores
   temporários mais frágeis do que a conversão total.
2. O refactor é mecânico — substituir 10 params por 1 + scope
   reconstructions. Alto volume, baixa complexidade unitária.
3. Contagem do Passo 107: 24 funções, 7 ficheiros. Ordem de
   grandeza aceitável para big-bang (comparável ao Passo 100).
4. Gate não dispara: 1 lifetime `'a`, nenhum conflito absoluto,
   Engine cobre 8/8 campos pretendidos.

### Compromisso explícito

- **Engine inclui 8 campos** (todos os parâmetros actuais). Uniformidade.
- **Engine não inclui** `introspector`, `routines`, `traced` —
  vanilla tem, cristalino ainda não materializou. Comentários
  `// stub futuro` documentam a divergência.
- **`world` move de `EvalContext` para `Engine`**. `EvalContext`
  passa de 4 campos (Regra 4) para 3.
- **`EvalContext` perde o lifetime `'w`**.

---

## Conclusão

- Lifetimes unificáveis em `'a`. Engine tem 1 lifetime.
- 8/8 campos absorvidos. Subconjunto = total.
- Forma: **big-bang**.
- 5-7 sítios requerem reconstrução local de Engine (scope changes).
- Ceremony aceitável (~60 linhas de construções struct literais).
- Gate 109.A.2 não dispara.

**Pronto para 109.B (ADR) e 109.C (implementação)**.
