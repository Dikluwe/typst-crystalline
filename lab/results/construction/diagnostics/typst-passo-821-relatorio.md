# Relatório — typst-passo-821: `foundations::target_` — `#target()` fora de `#context` não erra (achado #8 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-821.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree não commitado. Estado nas medições "antes" (`git status`): alterações de P823/P824/P827/P814/P815 vs HEAD + relatórios untracked. Estado nas medições "depois": os anteriores + os ficheiros de P821 (ver §Passo 2; `git diff HEAD --stat` final: 31 ficheiros, +1329/-166). Binário cristalino rebuildado após a implementação e novamente após o `--fix-hashes` (só header de comentário mudou nesse intervalo).
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Texto extraído com `pdftotext`.

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p821/` (`k1`–`k9`). Saída literal, ANTES da implementação:

| # | Documento | Vanilla | Cristalino (ANTES) |
|---|---|---|---|
| k1 | `#target()` | `error: can only be used when context is known` @ 1:1 + hint `try wrapping this in a `context` expression` + hint `the `context` expression should wrap everything that depends on this function` (exit 1) | exit 0, `paged` — **o achado** |
| k2 | `#context target()` | exit 0, `paged` | exit 0, `paged` ✓ (controlo) |
| k3 | `Texto #context target() fim` | exit 0, `Texto paged fim` | idem ✓ |
| k4 | `#target(1)` | `error: unexpected argument` @ 1:8 | `<detached>: error: target() não aceita argumentos, recebeu 1` |
| k5 | `#context type(1)` | exit 0, `int` | exit 0, **vazio** — colateral confirmado |
| k6 | `#context type("abc")` | exit 0, `str` | exit 0, **vazio** |
| k7 | `#context if target() == "paged" [sim] else [não]` | exit 0, `sim` | idem ✓ |
| k8 | `#target(x: 1)` | `error: unexpected argument: x` @ 1:8 | `<detached>: error: argumento nomeado inesperado: 'x'` |
| k9 | `#context target(1)` | `error: unexpected argument` @ 1:16 | `<detached>: error: target() não aceita argumentos, recebeu 1` |

Notas da sonda: (i) k4 fora de contexto erra `unexpected argument`, não o gate — a verificação de args corre **antes** do gate no vanilla; (ii) o colateral é **causa separada** do gate: `#context type()` puro (sem `target()`) também rende vazio — defeito de `value_to_content` (sem braço para `Value::Type`), não do mecanismo de contexto.

**Código identificado:**

- Vanilla: `lab/typst-original/crates/typst-library/src/foundations/target.rs:134-137` — `#[func(contextual)] pub fn target(context: Tracked<Context>)` → `context.styles()?`; a falha fora de contexto vem de `require(None)` em `lab/typst-original/crates/typst-library/src/foundations/context.rs:55-61` (mensagem + os 2 hints).
- Cristalino (ANTES): `01_core/src/engine/stdlib/foundations.rs:1393-1407` — `native_target` ignorava `ctx` (`_ctx`) e devolvia `"paged"` incondicionalmente; args com mensagens PT e `Span::detached()` (via `expect_no_named`/`err`).
- Colateral: `01_core/src/engine/stdlib/state.rs:137-157` — `value_to_content` com `_ => Content::Empty`; é a função usada por `03_infra/src/pipeline.rs:156` (`expand_context_blocks`, com `ctx.in_context = true` em `:126`) para converter o resultado da closure do `#context` em content.

## Passo 2 — Implementação

1. **`01_core/src/engine/stdlib/foundations.rs`** (`native_target`):
   - Named arg → `unexpected argument: {nome}`; posicional extra → `unexpected argument` — ambos com `args.span` (antes: PT + `<detached>`). Ordem: args **antes** do gate (medido em k4).
   - Gate de contexto: `if !ctx.in_context` → `can only be used when context is known` + os 2 hints verbatim do vanilla (mesma convenção `ctx.in_context` de `counter.get`/`measure`; o `#context` expande em L3 com `in_context = true` — `03_infra/src/pipeline.rs:126`).
2. **`01_core/src/engine/stdlib/state.rs`** (`value_to_content`): novo braço `Value::Type(t) => Content::text(t.name().to_string())` — display de um tipo é o seu nome curto (medido: `int`/`str`).
3. **Teste P772w actualizado** (`engine/stdlib/mod.rs:841`): `p772w_target_devolve_paged` chamava `native_target` com `in_context = false` e esperava `Ok` — contrato mudado em P821; o teste passa a definir `ctx.in_context = true` (o caso feliz contextual, como na expansão L3).
4. **L0 `00_nucleo/prompts/engine/stdlib/foundations.md`** (secção `native_target`): registado o gate P821, as mensagens verbatim, a ordem args→gate, e o colateral como causa separada. `crystalline-lint --fix-hashes .` → novo hash `1e792122`.

## Passo 3 — Validação (medição DEPOIS)

`./target/release/typst <doc>.typ -o <out>.pdf`, mesmas fixtures:

```text
k1: k1.typ:1:7: error: can only be used when context is known
      hint: try wrapping this in a `context` expression
      hint: the `context` expression should wrap everything that depends on this function   (= vanilla; span 1:7 vs 1:1 — nuance abaixo)
k2: exit 0, "paged"              (controlo ✓)
k3: exit 0, "Texto paged fim"    (controlo ✓)
k4: k4.typ:1:7: error: unexpected argument          (= vanilla; span 1:7 vs 1:8 — nuance)
k5: exit 0, "int"                (= vanilla — colateral corrigido)
k6: exit 0, "str"                (= vanilla)
k7: exit 0, "sim"                (controlo ✓)
k8: k8.typ:1:7: error: unexpected argument: x       (= vanilla)
k9: k9.typ:1:15: error: unexpected argument         (= vanilla)
```

**Nuance de span (débito conhecido P772s, como em P814):** o vanilla ancora o erro ao span da chamada completa `target()` (k1 @ 1:1) ou ao argumento (k4 @ 1:8); o cristalino ancora a `args.span` — a lista de argumentos (k1 @ 1:7, o `(`). O span deixa de ser `<detached>` e aponta para a chamada; a coluna difere por não haver spans por-argumento.

**Testes novos:**
- L1 `01_core/src/engine/eval/tests.rs` (4, prefixo `p821_`): erro fora de contexto com os 2 hints, posicional extra, named, e controlo (`#context target()` produz ContextBlock no eval L1 sem avaliar). Confirmados a falhar ANTES (`1 passed; 4 failed` no filtro — o controlo já passava).
- L1 `01_core/src/engine/stdlib/state.rs` (1): `value_to_content(Value::Type)` → `"int"`/`"str"`. Falhava ANTES.
- L3 `03_infra/src/integration_tests.rs` (3): `#context target()` expande para `paged` (controlo), `#context type(1)`/`type("abc")` expandem para `int`/`str` (colateral), `#target()` fora de contexto erra no pipeline completo com hint. 2 falhavam ANTES (o controlo passava).

**Suítes:**
- `cargo test -p typst-core`: ANTES `4395 passed; 0 failed; 2 ignored` → DEPOIS **`4400 passed; 0 failed; 2 ignored`** (+5).
- `cargo test -p typst-infra`: ANTES `661 passed; 0 failed; 5 ignored` → DEPOIS **`664 passed; 0 failed; 5 ignored`** (+3).

**Lint:** `crystalline-lint .` → **zero violations** (só os 3 warnings V7 de prompts órfãos pré-existentes, não relacionados).
