# Relatório — typst-passo-814: `typst_eval` — `#eval` sem `mode:`/`scope:`, erros genéricos e spans detached (achado #1 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-814.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree não commitado. Estado nas medições "antes" (`git diff HEAD --stat`): 6 ficheiros, +329/-54 — `D 00_nucleo/README-indice-813-827.md` (pré-existente, externo) + alterações de P823/P824/P827 em `loading.rs`, `loading.md` (L0), `03_infra/src/{integration_tests.rs, world.rs}`, `crystalline.toml`; mais 3 relatórios untracked (P823/P824/P827). Nenhum toca `eval`. Estado nas medições "depois": os anteriores + os ficheiros de P814 (ver §Passo 2; `git diff HEAD --stat` final: 20 ficheiros, +795/-140). Binário cristalino rebuildado após a implementação e novamente após o `--fix-hashes` (só comentários de header mudaram nesse intervalo).
**Binários:** `./target/release/typst` (cristalino — sem subcomando `compile`; uso: `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Texto extraído com `pdftotext`.

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p814/` (`t1`–`t16`). Saída literal dos dois binários, ANTES da implementação:

| # | Documento | Vanilla | Cristalino (ANTES) |
|---|---|---|---|
| t1 | `#eval("1 + 2")` | exit 0, `3` | exit 0, `3` (paridade — controlo) |
| t2 | `#eval("= Heading", mode: "markup")` | exit 0, `Heading` | `<detached>: error: argumento nomeado inesperado: 'mode'` (exit 1) |
| t3 | `#eval("x + 1", scope: (x: 2))` | exit 0, `3` | `<detached>: error: argumento nomeado inesperado: 'scope'` (exit 1) |
| t4 | `#eval("1 +")` | `error: expected expression` @ `t4.typ:1:6` (span sublinha `"1 +"` dentro da chamada) | `<detached>: error: erro de sintaxe em eval()` (genérico) |
| t5 | `#eval(42)` | `error: expected string, found integer` @ `1:6` | `<detached>: error: eval() espera string, recebeu int` |
| t6 | `#eval("1_2^3", mode: "math")` | exit 0, equação inline | `error: argumento nomeado inesperado: 'mode'` |
| t7 | `#eval("1", mode: "wrong")` | `error: expected "markup", "math", or "code"` @ `1:17` | `error: argumento nomeado inesperado: 'mode'` |
| t8 | `#eval("x", scope: 5)` | `error: expected dictionary, found integer` @ `1:18` | `error: argumento nomeado inesperado: 'scope'` |
| t9 | `#eval("zzz + 1")` (linha 2) | `error: unknown variable: zzz` @ `2:6` (span na string) | `<detached>: error: unknown variable: zzz` (mensagem já em paridade; span detached) |
| t10 | `#eval("1", foo: 2)` | `error: unexpected argument: foo` @ `1:11` | `<detached>: error: argumento nomeado inesperado: 'foo'` |
| t11 | `#eval("1", mode: 1)` | `error: expected "markup", "math", or "code", found integer` @ `1:17` | `error: argumento nomeado inesperado: 'mode'` |
| t12 | `#let y = 10` + `#eval("y + 1")` | **`error: unknown variable: y`** (exit 1) | exit 0, `11` — **divergência de semântica medida** (ver §Achado novo) |
| t13 | `#eval()` | `error: missing argument: source` @ `1:1` | `<detached>: error: eval() requer 1 argumento (source)` |
| t14 | `#eval("1", "2")` | `error: unexpected argument` @ `1:11` | `<detached>: error: eval() requer 1 argumento (source)` |
| t15 | `#let y = 10` + `#eval("y + 1", scope: (y: 2))` | exit 0, `3` | `error: argumento nomeado inesperado: 'scope'` |
| t16 | `#eval("#let q = 7")` + `#q` | `error: unknown variable: q` (o `let` fica confinado ao eval) | ANTES: `let` vazava para o scope do chamador (medido indirectamente; DEPOIS em paridade — ver §Passo 3) |

Nota: `t16` na forma `#eval("let q = 7")` (sem `#` dentro da string — com `#` ambos erram verbatim `the character '#' is not valid in code` + 2 hints, o que já demonstrou paridade de hints do parser).

**Código identificado:**

- Vanilla: `lab/typst-original/crates/typst-library/src/foundations/mod.rs:267-322` — `eval(source: Spanned<String>, mode: SyntaxMode = Code, scope: Dict)`; os bindings do dict entram num `Scope` empurrado na pilha do VM. `lab/typst-original/crates/typst-eval/src/lib.rs:103-165` (`eval_string`): despacha `parse_code`/`parse`/`parse_math`, aplica `SpanMode::Uniform(span)` → `root.synthesize(span)` (todos os spans da árvore = span do literal string), propaga `root.errors_and_warnings()` com mensagens reais, e avalia num **`Scopes` fresco** (só stdlib + `scope:` — não vê o scope do chamador). Modo math → `EquationElem::new(..).with_block(false)`. Cast de `SyntaxMode`: `lab/typst-original/crates/typst-library/src/foundations/cast.rs:467-480` (ordem `"markup", "math", "code"` na mensagem).
- Cristalino (ANTES): `01_core/src/engine/stdlib/eval.rs:45` (`expect_no_named` — rejeitava `mode:`/`scope:`), `:49` (`Source::detached_with_parser` com `FileId` sentinel 1 — spans numbered sem source resolvel → `<detached>` no report), `:53-58` (erro genérico `"erro de sintaxe em eval()"`), `:83-90` (mensagens de tipo/idade em português). Comentário doc em `:34-36` **falso** (afirmava default vanilla `"markup"`; medido: `#[default(SyntaxMode::Code)]`, vanilla `foundations/mod.rs:279`).

## Passo 2 — Implementação

1. **`01_core/src/engine/parse/mod.rs`** — nova função **`parse_anchored(text, mode, anchor) -> SyntaxNode`**: despacha `parse_code`/`parse`/`parse_math` e ancora todos os spans ao `anchor` via `SyntaxNode::synthesize` (já existente, `entities/syntax_node.rs:185-191`); `anchor` detached → árvore intacta. É o mecanismo de span sintético reutilizável (P815, P819).
2. **`01_core/src/engine/stdlib/eval.rs`** — `native_eval` reescrito:
   - named `mode:` (`"code"`/`"markup"`/`"math"`, default `"code"` = paridade vanilla) e `scope:` (`Dict`); mensagens de cast **verbatim do vanilla** (medidas): `expected "markup", "math", or "code"` (+`, found {tipo}` para não-string), `expected dictionary, found {tipo}`, `unexpected argument: {nome}`, `expected string, found {tipo}`, `missing argument: source`, `unexpected argument`. Nomes longos via `long_type_name` (`integer`/`string`/`boolean`).
   - Parse via `parse_anchored(source, mode, args.span)`; erros de sintaxe propagam mensagem + hints reais do parser com o span âncora.
   - `scopes.enter()`/`exit()` à volta da avaliação: bindings do dict `scope:` confinados (sombreiam o chamador, não vazam) — efeito colateral medido: `#let` dentro do eval deixou de vazar (paridade vanilla, t16).
   - Modo markup → `eval_markup`; modo math → `Content::equation(body, false)` (paridade `with_block(false)`); confinamento `#set`/`#show` em engine local mantido.
   - Comentário falso corrigido (default vanilla = `SyntaxMode::Code`, com referência à medição).
3. **`01_core/src/engine/eval/bindings.rs`** — `long_type_name` promovido a `pub(crate)` (reexport em `engine/eval/mod.rs` — o módulo `bindings` permanece privado).
4. **Teste P394 actualizado** (`engine/stdlib/mod.rs:12598`): `p394_eval_named_arg_inesperado_erro` usava `mode:` como exemplo de named rejeitado — contrato mudou em P814; passa a usar `foo:` (intenção do teste preservada).
5. **L0 actualizados**: `00_nucleo/prompts/engine/stdlib/eval.md` (§2/§3/§4/§5/§6 — `mode:`/`scope:`, mensagens, span âncora, divergência do scope do chamador registada como decisão medida, scope-out de `file:` corrigido) e `00_nucleo/prompts/engine/parse.md` (`parse_anchored` na interface pública + critérios). `crystalline-lint --fix-hashes .` → novos hashes (`eval.rs` → `e297890d`, `parse/*` → `d3230214`).

**Nuance registada (débito conhecido P772s):** o vanilla ancora ao span do **literal string** (t4 @ `1:6`); o cristalino ancora a `args.span` — a **lista de argumentos** (t4 @ `1:5`, o `(`). Spans por-argumento não existem no cristalino (`entities/args.md`, decisão medida de custo). O span deixa de ser `<detached>` e aponta para dentro da chamada; a coluna difere de 1.

## Passo 3 — Validação (medição DEPOIS)

`./target/release/typst <doc>.typ -o <out>.pdf`, mesmas fixtures:

```text
t1:  exit 0, "3"                                                          (= vanilla)
t2:  exit 0, "Heading"                                                    (= vanilla)
t3:  exit 0, "3"                                                          (= vanilla)
t4:  t4.typ:1:5: error: expected expression                               (mensagem = vanilla; col 5 vs 6 — nuance P772s)
t5:  t5.typ:1:5: error: expected string, found integer                    (= vanilla)
t6:  exit 0, equação inline ("132" no pdftotext)                          (= vanilla)
t7:  t7.typ:1:5: error: expected "markup", "math", or "code"              (= vanilla)
t8:  t8.typ:1:5: error: expected dictionary, found integer                (= vanilla)
t9:  t9.typ:2:5: error: unknown variable: zzz                             (= vanilla; span na linha certa)
t10: t10.typ:1:5: error: unexpected argument: foo                         (= vanilla)
t11: t11.typ:1:5: error: expected "markup", "math", or "code", found integer  (= vanilla)
t12: exit 0, "11"   ← divergência MANTIDA (decisão do L0 vigente — ver §Achado novo)
t13: t13.typ:1:5: error: missing argument: source                         (= vanilla)
t14: t14.typ:1:5: error: unexpected argument                              (= vanilla)
t15: exit 0, "3"                                                          (= vanilla)
t16: t16.typ:2:1: error: unknown variable: q                              (= vanilla — let confinado; ANTES vazava)
```

Também medido (bónus): `#eval("#let q = 7")` → `error: the character '#' is not valid in code` + os 2 hints do parser (`you are already in code mode`, `try removing the '#'`) — verbatim nos dois binários.

**Testes novos** (14 em `01_core/src/engine/eval/tests.rs` secção P814 + 4 em `01_core/src/engine/parse/mod.rs`): confirmados a falhar ANTES (`test result: FAILED. 0 passed; 14 failed` no filtro `p814_`) e a passar DEPOIS (`18 passed; 0 failed`).

**Suíte `cargo test -p typst-core`:**
- ANTES: `4362 passed; 0 failed; 2 ignored` (estado do handoff).
- DEPOIS: **`4380 passed; 0 failed; 2 ignored`** (4362 + 18 novos).

**Lint:** `crystalline-lint .` → **zero violations** (restam apenas os 3 warnings V7 de prompts órfãos pré-existentes — `stdlib/layout.md`, `stdlib/structural.md`, `infra/package_version_resolution.md` — não relacionados com P814).

## Achado novo (medido neste passo — fora do âmbito do achado #1, registado para decisão)

**`#eval` vê o scope do chamador no cristalino; no vanilla não vê.** t12: `#let y = 10` + `#eval("y + 1")` → vanilla `error: unknown variable: y` (`eval_string` cria `Scopes` fresco — `typst-eval/src/lib.rs:151`); cristalino `11`. O L0 P394 declarou "a re-avaliação vê as variáveis do scope onde eval é chamado" e há um teste (`eval_ve_escopo_actual`) que fixa esse comportamento. **Mantido o comportamento do L0 vigente** (não alterado neste passo — mudar exige revisão do L0 `stdlib/eval.md` §4 + desse teste, decisão do dono). O L0 actualizado regista a divergência como medida e declarada.

## Mecanismo de span sintético (nota para P815/P819)

`crate::engine::parse::parse_anchored(text, mode, anchor) -> SyntaxNode` (`01_core/src/engine/parse/mod.rs`): parseia no `SyntaxMode` indicado e chama `root.synthesize(anchor)` (`entities/syntax_node.rs:185`) — todos os nós e nós de erro ficam com o span âncora. Erros de sintaxe extraem-se com `root.errors()` (`SyntaxError { span, message, hints }`). O `anchor` disponível nas nativas é `args.span` (span da lista de argumentos da chamada, P772s). Equivalente ao `SpanMode::Uniform` do vanilla. Uso em `stdlib/eval.rs::native_eval` como referência.
