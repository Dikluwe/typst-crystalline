# Passo 109 — Relatório de encerramento (Engine<'a> materializado)

**Data**: 2026-04-23
**Precondição**: Passo 108 encerrado; análise recomendou Candidato 5
(Engine<'a>); 803 L1 + 184 L3 + 6 ignorados; zero violations.
**ADR criada**: ADR-0044 "Engine<'a> como agregador de estado de
eval em L1" — **PROMOVIDA A EM VIGOR** em 109.E.

---

## Sumário

`Engine<'a>` materializado em `01_core/src/entities/engine.rs` com
**8 campos**. Todas as 24 funções `eval_*` tocadas nas 5 aplicações
da ADR-0036 passam de `10 params + ctx` para **`1 param (engine) +
ctx`**. `world` movido de `EvalContext` para `Engine`; `EvalContext`
fica com **3 campos Regra 4** e perde o lifetime `'w`.

Zero regressão funcional: **803 L1 + 184 L3 + 6 ignorados**
(exactamente igual à linha de base). `crystalline-lint .` → zero
violations.

---

## 109.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-engine-passo-109.md`.

### Lifetimes

Unificáveis em `'a` único. Detalhe: `Tracked<'a, Route<'a>>` +
`&'a mut StyleChain` + ... + `&'a mut TrackedMut<'a, Sink>`.
Gate 109.A.2 não disparou (1 lifetime, 8/8 campos absorvidos).

### Forma

**Big-bang**. Cross-calls entre eval_* obrigavam conversão conjunta;
incremental-por-função exigia adaptadores frágeis.

---

## 109.B — ADR-0044

Criada em `00_nucleo/adr/typst-adr-0044-engine-agregador.md`.
**Promovida a EM VIGOR em 109.E**.

Pontos-chave:

- **Inversão controlada** da ADR-0036 — não a revoga; reconhece
  que "extrair primeiro, agregar depois" é padrão válido quando o
  número de params cruza limite pragmático (10 em 107).
- **Campos omitidos** (introspector, routines, traced) documentam
  a divergência face ao vanilla em comentários; entram em passos
  dedicados.
- **`world` move de `EvalContext` para `Engine`**. `EvalContext`
  reduzido a 3 campos Regra 4.
- **Engine não é `#[comemo::track]`**. Tracking via campos
  individuais (`route: Tracked`, `sink: TrackedMut`).
- **Assinatura pública de `eval()` mantida** — `TrackedMut<Sink>`
  continua a entrar por valor (ADR-0043).

---

## 109.C — Implementação

### Ficheiros criados

1. `01_core/src/entities/engine.rs` (78 linhas) — struct Engine<'a>
   + módulo test-level (vazio por spec).
2. `00_nucleo/prompts/entities/engine.md` — prompt L0.

### Ficheiros tocados

| Ficheiro | Mudanças |
|----------|---------|
| `rules/eval/mod.rs` | `EvalContext` sem `'w`; `eval()` constrói Engine; `eval_markup`/`eval_expr`/`eval_markup_body` recebem Engine; scopes locais (CodeBlock, ContentBlock). |
| `rules/eval/rules.rs` | `apply_show_rules`, `intercept_content`, `eval_set_rule`, `eval_show_rule` via Engine. |
| `rules/eval/bindings.rs` | 3 funções migradas. |
| `rules/eval/closures.rs` | 5 funções migradas; `apply_closure` com scope local. |
| `rules/eval/control_flow.rs` | 3 funções migradas. |
| `rules/eval/markup.rs` | 5 funções migradas; helper `eval_body_with_delta` para scope de estilos. |
| `rules/eval/modules.rs` | `eval_module_include` com scope local (route + current_file). |
| `rules/eval/tests.rs` | `eval_for_test_with_limits` actualizado. |
| `entities/func.rs` | ABI `NativeFunc` ganha `&dyn World` como 3º parâmetro. |
| `entities/mod.rs` | Adiciona `pub mod engine`. |
| stdlib/* (10 ficheiros) | Assinaturas de 30+ `native_*` ganham `_world: &dyn World` (via sed). |
| `stdlib/figure_image.rs` | `ctx.world` → `world` (via parâmetro). |
| `stdlib/mod.rs` | `null_ctx!` macro + `null_world()` helper. |

### Ceremony: scope-local Engine

5 sítios precisam de reconstruir Engine local (campos que mudam no
descent). Todos usam o mesmo padrão:

```rust
let mut local_styles = ...;  // ou local_show_rules, child_route
let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
let mut local_engine = Engine {
    world: engine.world,
    route: engine.route,  // ou child_route.track()
    styles: &mut local_styles,  // ou engine.styles reborrow
    show_rules: &mut *engine.show_rules,
    active_guards: &mut *engine.active_guards,
    current_file: engine.current_file,  // ou src_id
    figure_numbering: &mut *engine.figure_numbering,
    sink: &mut local_sink,
};
child_function(..., &mut local_engine)
```

**Descoberta (gate 109.A.3 implícito)**: `TrackedMut::reborrow_mut`
é **obrigatório** para encurtar o lifetime interno em descida,
caso contrário `Engine<'a>` do filho colide com `'caller`. Sem este
reborrow, erros `E0597` ("TrackedMut<'caller, Sink> does not live
long enough"). Aplicado em todos os 6 sítios (caller `eval()` +
5 scope locais).

### Forma da migração: big-bang executada

Compilação intermédia impossível — cross-calls obrigam migração
conjunta. Compilação final após todas as mudanças.

### Trade-off ABI de NativeFunc

`world` saiu de `EvalContext`. Native functions usavam `ctx.world`
em 1 sítio (`figure_image.rs:95`). Opções consideradas:

- **Manter world dentro do ctx para natives** — viola
  consolidação.
- **Passar engine às natives** — expõe muito Engine às natives,
  quebra encapsulamento ABI (ADR-0036 Regra 1).
- **Adicionar `&dyn World` ao ABI** — decisão tomada. Todas as
  natives ganham `_world: &dyn World` como 3º parâmetro; usa-se
  nas que fazem I/O (`native_image`); ignora-se nas puras.

Custo: +1 parâmetro em ~30 natives. Mantém natives desacopladas
do Engine.

---

## 109.D — Testes

**Nenhum teste novo** (spec 109.D exige). Critério: testes
existentes passam.

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...     (L1 inalterado)
test result: ok. 184 passed; 0 failed; 6 ignored ...     (L3 inalterado)

$ grep -rn "ctx\.world" 01_core/src/
(zero matches — world só no Engine agora)

$ crystalline-lint .
✓ No violations found
```

### Borrow checker

O padrão de scope-local Engine com reborrow não trigueou conflicts
de borrow-checker inesperados. Apenas a necessidade de
`TrackedMut::reborrow_mut` (documentado acima).

---

## 109.E — Encerramento

### Subconjunto real dos campos no Engine

**8/8 campos** (spec 109.A tabela de candidatos):

| Campo | Tipo |
|-------|------|
| `world` | `&'a dyn World` |
| `route` | `Tracked<'a, Route<'a>>` |
| `styles` | `&'a mut StyleChain` |
| `show_rules` | `&'a mut Arc<[ShowRule]>` |
| `active_guards` | `&'a mut Vec<RuleId>` |
| `current_file` | `FileId` |
| `figure_numbering` | `&'a mut Option<String>` |
| `sink` | `&'a mut TrackedMut<'a, Sink>` |

### Contagem de parâmetros

| Função | Antes (Passo 107) | Depois (Passo 109) |
|--------|-------------------|---------------------|
| `eval_set_rule` | 10 + ctx | **1 + ctx** (engine) |
| `eval_markup` / `eval_expr` | 10 + ctx | **1 + ctx** |
| `apply_show_rules` / `intercept_content` | 10 + ctx | **1 + ctx** |
| `eval_*` genérica | ~10 + ctx | **1 + ctx** |

Redução: **~90%** dos parâmetros de eval absorvidos. Visualmente
sustentável outra vez.

### DEBT-45 status

Grep: `check_call_depth` e `check_layout_depth` permanecem funções
livres, não integradas via trait. Nenhum passo a integrar trivialmente
agora via Engine — permanece pendente para passo dedicado.

### ADR

**ADR-0044** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | 803 (inalterado) |
| L3 tests | 184 | 184 (inalterado) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 43 | **44** (+0044) |
| DEBTs abertos | 12 | 12 (inalterado) |
| Params `eval_*` | 10 + ctx | **1 + ctx** |
| Campos `EvalContext` | 4 (com `'w`) | **3** (sem lifetime) |

---

## Lições

1. **`TrackedMut::reborrow_mut` é obrigatório em descida**: o
   `TrackedMut<'outer, T>` mantém lifetime do caller; construir
   Engine<'short> sem `reborrow_mut` falha com E0597. Adicionado
   em 6 sítios. Documentar para futuros que construam Engine
   scope-local.

2. **ABI de NativeFunc ganha `&dyn World`**: remover `world` do
   `EvalContext` forçou a decisão. Alternativas (passar Engine às
   natives, ou manter world no ctx) violam encapsulamento mais do
   que a adição dum parâmetro. Custo pequeno (prefixar `_world` em
   ~30 natives puras).

3. **Big-bang foi certo**: migração incremental-por-função criaria
   adaptadores frágeis porque `eval_expr` e `eval_set_rule` etc.
   cross-call. Conversão conjunta em 1 commit conceptual, mesmo
   que o diff seja grande.

4. **8/8 campos cabem em 1 lifetime**: inventário 109.A já previa;
   confirmado na implementação. O padrão "Engine<'a>" com lifetime
   único funciona em Rust 2021 com NLL.

5. **Scope-local Engine é explícito, não transparente**: cada sítio
   que muda um campo do Engine (CodeBlock, ContentBlock, strong,
   emph, heading, apply_closure, eval_module_include) faz um
   `let mut local_engine = Engine { ... }` literal. ~60 linhas de
   boilerplate. Preço justo pela redução de 10→1 em 24 funções.

6. **`EvalContext` volta a ser só Regra 4**: 3 campos
   (`loop_iterations`, `max_loop_iterations`, `next_rule_id`),
   nenhum com lifetime. Validação *a posteriori* da categorização
   da ADR-0036 — "Regra 4" era o subconjunto real do contexto que
   não é fluxo de eval.

---

## Estado pós-Passo 109

### Arquitectura eval

```
eval()                              [pub, 5 params]
    ↓ constrói Engine<'_>
Engine<'a> {
    world, route, styles, show_rules,
    active_guards, current_file,
    figure_numbering, sink
}
    ↓ &mut engine
eval_markup / eval_expr / ...       [1 param engine + ctx]
    ↓ (scope change?)
    ↓     ├── CodeBlock: local_styles + local_show_rules
    ↓     ├── ContentBlock: local_styles
    ↓     ├── eval_strong/emph/heading: local_styles (+ delta)
    ↓     ├── apply_closure: child_route + local_styles
    ↓     └── eval_module_include: child_route + child_current_file
    ↓ construção local_engine
eval_* filho recebe &mut local_engine
```

### Trabalho futuro identificado

1. **Candidato 2 do Passo 108**: `Introspector` wrapping
   `CounterState` → adicionar como campo de Engine quando
   materializar.
2. **Candidato 4 do Passo 108**: `Location + here + counter.at`
   → requer decisão arquitectural (ver 108-relatorio).
3. **Candidato 3 do Passo 108**: `query(...)` → requer multi-pass.
4. **Routines** e **Traced** materialização — entram em Engine
   quando concretizados.
5. **DEBT-45 integração** — `check_call_depth` pode vir a ser
   método de `Route` via `engine.route.check_call_depth(...)` —
   mas é refactor separado.
