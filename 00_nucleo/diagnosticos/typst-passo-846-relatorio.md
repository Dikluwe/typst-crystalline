# Relatório — typst-passo-846: span de `#eval` (#56) e call trace (#57)

**Data**: 2026-07-23
**Executor**: agente IA (subagente Kimi Code)
**Origem**: achados #56 (S1) e #57 (S2) de P831 (lote 5)

## Proveniência

- **Commit HEAD no arranque**: `da9ef329c5874aa83312adc7a6ccb29eff075047` (`chore: P845 — #55 ...`).
- **Árvore no arranque**: limpa (`git status --porcelain` vazio).
- **Baseline medido no arranque**: `cargo test -p typst-core` → **4625 passed, 0 failed**; `cargo test -p typst-infra` → **714 passed, 0 failed**.
- Binários: vanilla `lab/typst-original/target/release/typst` (build 2026-06-29); cristalino `./target/release/typst` — **rebuild release feito antes da medição "depois"** (`cargo build --release`).
- Fixtures: `temp/p846/` (`span7.typ` copiado de `temp/p831/`; `e1`–`e6`, `trace1`–`trace5` criados neste passo).

---

## #56 (S1) — span de erros dentro de `#eval`

### Sonda / medição ANTES

Comando: `<vanilla> compile span7.typ out.pdf` vs `./target/release/typst span7.typ -o out.pdf`
(`span7.typ` = `Texto.\n\n#eval(\n  "abc +"\n)\n`).

| Caso | Vanilla | Cristalino ANTES |
|---|---|---|
| `span7.typ` (parse error) | `4:2` (literal string) | `3:5` (lista de argumentos) |
| `e1` = `#eval(5)` (cast error) | `1:6` (o `5`) | `1:5` |
| `e2` = `#eval()` (missing arg) | `1:1` (chamada inteira) | `1:5` |
| `e3` = `#let f = eval` + `#f("abc +")` | `2:3` (literal) | `2:2` |
| `e5` = `#eval("zzz + 1")` (semântico) | `1:6` (literal) | `1:5` |

Divergência de **linha** confirmada em `span7.typ` (não só coluna) — a nota do L0 §3 ("nuance de uma coluna", P814) subestimava. Causa: vanilla ancora ao literal string (`SpanMode::Uniform`, `lab/typst-original/crates/typst-library/src/foundations/mod.rs:267,318`); cristalino ancorava a `args.span` (`01_core/src/engine/stdlib/eval.rs:135`).

### Decisão (do executor): correção pontual, SEM o débito estrutural

O call site genérico (`eval_func_call`, `01_core/src/engine/eval/closures.rs`) tem acesso ao AST dos argumentos. A correção ficou contida aí: quando o callee avalia para o nativo `eval` (`FuncRepr::NativeWithEngine` com `name == "eval"`) e existe argumento posicional, `args.span` é substituído pelo span da expressão do primeiro posicional antes de `apply_func`. `native_eval` não mudou (continua a consumir `args.span`); `Args` não mudou (sem span-por-item, débito P772s intacto). Não é uma "correção parcial que deixa o L0 mais incorreto" — cobre exatamente a divergência medida (erros dentro do string + cast error do posicional).

### Código

- `01_core/src/engine/eval/closures.rs` — em `eval_func_call`: captura de `eval_anchor` (span do primeiro `Arg::Pos` quando o callee é o nativo `eval`) e override de `args.span` antes de `apply_func`.

### Testes (RED confirmado → GREEN)

- Novos (`01_core/src/engine/eval/tests.rs`): `p846_eval_span_ancora_no_literal_string_multilinha` (caso span7 → `(4,2)`), `p846_eval_span_cast_error_ancora_no_argumento` (`#eval(5)` → `(1,6)`).
- Atualizados (fixavam o comportamento divergente): `p814_eval_erro_sintaxe_mensagem_real_e_span_util` e `p814_eval_erro_semantico_span_util` — `(1,5)` → `(1,6)`, com comentário de proveniência P846.
- RED: 5 falhas em 6 testes novos/atualizados antes da implementação (o 6.º, de trace, passava por vacuidade).

### Medição DEPOIS (release rebuildado)

| Caso | Vanilla | Cristalino DEPOIS | Estado |
|---|---|---|---|
| `span7.typ` | `4:2` | `4:2` | ✅ paridade |
| `e1` `#eval(5)` | `1:6` | `1:6` | ✅ paridade |
| `e3` alias `#f("abc +")` | `2:3` | `2:3` | ✅ paridade |
| `e5` `#eval("zzz + 1")` | `1:6` | `1:6` | ✅ paridade |
| `e2` `#eval()` | `1:1` | `1:5` | ⚠️ divergência residual (documentada) |

### Atualização do L0 (obrigatória)

`00_nucleo/prompts/engine/stdlib/eval.md` §3 ("Span âncora"): reescrito — regista a divergência real medida (linha, não coluna), a implementação P846, e as ressalvas residuais: `missing argument: source` (vanilla: span da chamada inteira; cris: lista), erros de named args / posicionais extra (vanilla: argumento em causa; cris: primeiro posicional quando existe), e `#eval(..spread)`.

---

## #57 (S2) — call trace ausente

### Sonda / formato exato do vanilla (medido no binário)

`trace1.typ` = `#let c() = { 1 + "a" }\n#let b() = { c() }\n#let a() = { b() }\n#a()\n`:

```text
error: cannot add integer and string
  ┌─ trace1.typ:1:13
  │
1 │ #let c() = { 1 + "a" }
  │              ^^^^^^^

  while calling `c` at trace1.typ:2:13
    c()
  while calling `b` at trace1.typ:3:13
    b()
  while calling `a` at trace1.typ:4:1
    a()
```

Formato por nível (fonte: `lab/typst-original/crates/typst-kit/src/diagnostics.rs:105-146`): 2 espaços + `<tracepoint> at <ficheiro>:<linha>:<col>` (linha 1-based, col 0-based) + linha seguinte com 4 espaços e o **texto fonte do span do tracepoint** (multi-linha: primeira linha + `…` + último char não-whitespace). Ordem: **innermost primeiro**. Níveis: um por chamada, sem limite observado (recursão `trace4` mostra os 3 níveis repetidos). `Call(None)` → `while calling function` (sem backticks).

Regra de omissão (fonte: `lab/typst-original/crates/typst-library/src/diag.rs:464-479` + `typst-eval/src/call.rs:166-180`): todo o `call_func` envolve o resultado com `.trace(...)`, mas o tracepoint é **omitido** quando o span da chamada **contém** o span do erro (mesma fonte). Medido: `trace2` (`eval("xyz +")` no corpo de `boom`) mostra `boom` mas **não** `eval`; `trace5` (`#(() => { 1 + "a" })()`) não mostra tracepoint nenhum (o corpo está dentro do span da chamada).

Cristalino ANTES: só a linha `path:1:13: error: cannot add integer and string` — trace completamente ausente (`Tracepoint` existia em `entities/source_result.rs:22` mas `trace` nunca era populado).

### Decisões do executor

1. **Ponto de população**: call site genérico (`eval_func_call`), com `call.span()` (span da chamada inteira, como o vanilla) — não `args.span`. Callbacks invocados por nativas (`map`/`fold`/etc.) não ganham nível (no vanilla também não — chamam `Func::call` diretamente, fora de `call_func`). Chamadas em modo math (`eval/math.rs`) ficaram fora — não medidas no achado (scope-out registado no L0 `engine/eval.md` §P846).
2. **Formato de rendering**: as linhas de trace do vanilla não têm box-drawing — reproduzidas verbatim no formato gcc-style do cristalino, após os hints. Em modo `colored`, localização e snippet levam `dim` (paleta ADR-0048; o vanilla usa underline/cinza — fora da paleta cristalina; decisão minha, registada no L0 `shell/diagnostic.md`).
3. **`Source::span_byte_range`** (novo método L1, `entities/source.rs`) — equivalente ao `world.range(span)` do vanilla; partilhado por `trace_call` (regra de contenção) e pelo renderer L2 (snippet).
4. **Divergência conhecida (não introduzida por este passo)**: `#let f = () => ...; #f()` — o cristalino nomeia a closure pelo binding (`set_name` em `eval_let`, `bindings.rs:69-72`, extensão deliberada pré-existente), logo mostra `while calling \`f\`` onde o vanilla mostra `while calling function` (medido em `trace3`).

### Código

- `01_core/src/engine/eval/closures.rs` — `trace_call(result, func, call_span, engine)` (mirror de `call_func` + `Trace::trace`: skip quando o span da chamada não resolve; skip por erro contido; senão push de `Spanned::new(Tracepoint::Call(func.name()), call_span)`), aplicada no ramo `Value::Func` de `eval_func_call`.
- `01_core/src/entities/source.rs` — `Source::span_byte_range(span)`.
- `02_shell/src/diagnostic.rs` — rendering do trace em `format_diagnostic` (formato verbatim medido; tracepoints não resolvidos omitidos).

### Testes (RED confirmado → GREEN)

- L1 (`engine/eval/tests.rs`): `p846_call_trace_cadeia_de_chamadas` (3 níveis, nomes + spans), `p846_call_trace_omite_chamada_que_contem_o_erro` (só `f`, sem `eval`), `p846_call_trace_omite_closure_inline_no_callee` (sem tracepoint), `p846_call_trace_arrow_closure_com_let_usa_nome_do_binding` (divergência documentada).
- L2 (`02_shell/src/diagnostic.rs`): `formato_com_call_trace_sem_cores` (verbatim, inclui `while calling function`), `formato_trace_snippet_multilinha_com_ellipsis`, `formato_trace_span_nao_resolvido_e_omitido`.

### Medição DEPOIS (release rebuildado)

| Caso | Vanilla | Cristalino DEPOIS | Estado |
|---|---|---|---|
| `trace1` (cadeia 3 níveis) | erro `1:13` + `c`@`2:13`, `b`@`3:13`, `a`@`4:1` + snippets | idêntico | ✅ |
| `trace2` (eval dentro de closure) | `1:21` + só `boom`@`2:1` | idêntico | ✅ |
| `trace3` (arrow closure com let) | `while calling function` | `while calling \`f\`` | ⚠️ divergência documentada (raiz pré-existente) |
| `trace4` (recursão) | 3× `rec`@`1:27` + `rec`@`2:1` | idêntico | ✅ |
| `trace5` (closure inline) | sem trace | sem trace | ✅ |

Nota: o cristalino imprime o path absoluto (comportamento pré-existente da linha principal, que o trace segue); o vanilla imprime o path como passado na CLI. Diferença de apresentação do path, não do trace.

---

## Contagens (antes → depois)

| Suite | Antes | Depois | Delta |
|---|---|---|---|
| `typst-core` | 4625 passed, 0 failed | **4631 passed, 0 failed** | +6 testes P846 (2 testes P814 atualizados in-place) |
| `typst-shell` | 33 passed, 0 failed | **36 passed, 0 failed** | +3 testes P846 |
| `typst-infra` | 714 passed, 0 failed | **714 passed, 0 failed** | sem mudança |

## Lint

`crystalline-lint --fix-hashes .` → re-análise `✅ 0 drift warnings remaining`; `crystalline-lint .` → **exit 0** (3 warnings V7 pré-existentes de prompts órfãos, sem relação com este passo: `engine/stdlib/layout.md`, `engine/stdlib/structural.md`, `infra/package_version_resolution.md`).

## Ficheiros tocados

Código:
- `01_core/src/engine/eval/closures.rs` (#56 âncora + #57 `trace_call`)
- `01_core/src/entities/source.rs` (`span_byte_range`)
- `02_shell/src/diagnostic.rs` (rendering do trace)
- `01_core/src/engine/eval/tests.rs` (+6 testes, 2 atualizados)

L0 (prompts):
- `00_nucleo/prompts/engine/stdlib/eval.md` §3 (nota corrigida — obrigatória)
- `00_nucleo/prompts/engine/eval.md` (nova §P846)
- `00_nucleo/prompts/entities/source.md` (`span_byte_range`)
- `00_nucleo/prompts/shell/diagnostic.md` (formato do trace)

Só hash (`--fix-hashes`, headers `@prompt-hash` de `engine/eval.md`): `bibliography.rs`, `control_flow.rs`, `flow.rs`, `markup.rs`, `math.rs`, `modules.rs` (e os ficheiros de código acima).

## Desvios / limitações a rever antes do commit

1. `#eval()` sem argumentos: `missing argument: source` continua ancorado à lista (`1:5` vs vanilla `1:1`) — divergência residual registada no L0 §3.
2. Erros de named args de `eval` e posicionais extra ancoram ao primeiro posicional (vanilla: ao argumento em causa) — registado no L0 §3.
3. Chamadas em modo math não ganham tracepoint (scope-out, L0 `engine/eval.md` §P846).
4. `#let f = () => ...; #f()` mostra `while calling \`f\`` (vanilla: `while calling function`) — raiz pré-existente (`set_name` em `eval_let`), não tocada.
5. Em modo `colored`, o trace usa `dim` em vez de underline/cinza do vanilla — decisão do executor (paleta ADR-0048), registada no L0 `shell/diagnostic.md`.
6. Sem commit (instrução do passo).
