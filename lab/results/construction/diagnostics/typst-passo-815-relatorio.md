# Relatório — typst-passo-815: `typst_eval::methods` — método inexistente diverge, dict-key-call sem hints (achado #2 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-815.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree não commitado. Estado nas medições "antes" (`git status`): alterações de P823/P824/P827/P814 (20 ficheiros modificados vs HEAD, incl. `bindings.rs`, `closures.rs` via `eval/mod.rs`, `eval/tests.rs`, L0s `eval.md`/`parse.md`/`loading.md`) + 4 relatórios untracked. Estado nas medições "depois": os anteriores + os ficheiros de P815 (ver §Passo 2). Binário cristalino rebuildado após a implementação e novamente após o `--fix-hashes` (só headers de comentário mudaram nesse intervalo).
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`).

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p815/` (`m1`–`m18`). Saída literal, ANTES da implementação:

| # | Documento | Vanilla | Cristalino (ANTES) |
|---|---|---|---|
| m1 | `#(1).foo()` | `error: type integer has no method `foo`` @ 1:1 | `error: cannot access fields on type int` |
| m2 | `#"texto".metodo_inexistente()` | `type string has no method `metodo_inexistente`` | `cannot access fields on type str` |
| m3 | `#let d = (x: 1)` + `#d.x()` | `cannot directly call dictionary keys as functions` + hint `to access the `x` key, remove the function arguments: `d.x`` + hint `dictionary keys cannot be used with method syntax as keys could conflict with built-in method names` | `não é possível chamar int` (sem hints) |
| m4 | `#d.zzz()` (chave ausente) | `type dictionary has no method `zzz`` | `dictionary does not contain key "zzz"` |
| m5 | `#let d = (f: x => x * 2)` + `#d.f()` | mesmo erro + hint `to call the stored function, wrap the field access in parentheses: `(d.f)(..)`` + hint dict | **chamava a função guardada** — `error: cannot apply Mul to none and int` (bug) |
| m6 | `#"ab".push("c")` (controlo) | `cannot mutate a temporary value` | `cannot mutate a temporary value` ✓ (paridade) |
| m7 | `#(1.5).foo()` | `type float has no method `foo`` | `cannot access fields on type float` |
| m8 | `#(1, 2).zzz()` | `type array has no method `zzz`` | `array does not contain field "zzz"` |
| m9 | `#(1).foo` (**sem** parênteses) | `cannot access fields on type integer` @ 1:5 (span do field) | `cannot access fields on type int` — divergência do caminho **não-chamada** (nome curto; fora do âmbito, registada) |
| m10 | `#d.x` (controlo) | `1` | `1` ✓ |
| m11 | `#(10pt).abs()` | `` `abs` is not a valid method for type `length` `` + hint `to access the `abs` field, remove the function arguments: `(10pt).abs`` | `não é possível chamar length` |
| m12 | `#let f(..args) = args.positional()` + `#f(1, 2)` | `type arguments has no method `positional`` (o vanilla **não** tem o campo `.positional` em `arguments`) | `não é possível chamar array` (o cristalino tem o campo — P504 — e avaliava-o) |
| m13 | `#strong[x].body()` | `` `body` is not a valid method for element `strong` `` + hint | `não é possível chamar content` |
| m14 | `#strong[x].func()` | exit 0, `strong` (método real) | `strong does not have field "func"` — **métodos de content em falta** (achado adjacente, registado) |
| m15 | `#strong[x].zzz()` | `element strong has no method `zzz`` | `strong does not have field "zzz"` |
| m16 | `$d.x()$` | exit 0 (parênteses = agrupamento math) | exit 0 ✓ |
| m17 | `$#d.x()$` | erro dict-key + hints **não-math** | `chamada em modo math espera função, recebeu int` (despacho math separado — registado, fora do âmbito) |
| m18 | `#rgb("#ff0000").foo()` | `type color has no method `foo`` | `cannot access fields on type color` |

**Hipótese do prompt refutada (ADR-0108):** o prompt sugeria distância de edição nos hints ("como P772r"). A fonte do vanilla **não** usa distância de edição neste caminho — `call.rs:339-340`: *"The field does not exist. We don't try as hard on the error here to avoid assuming the user's intent."* Os hints do dict-key-call são textos fixos. Não existe mecanismo de distância de edição em `01_core` (P772r é a heurística de hífen/subtracção). Nada a reaproveitar.

**Código identificado:**

- Vanilla: `lab/typst-original/crates/typst-eval/src/call.rs:239-355` (`eval_field_callee` — só `Symbol`/`Func`/`Type`/`Module` chamam campos directamente; ramo de erro em `:258-345`), `:359-365` (`element_or_type_with_name` → `("element", elem)` / `("type", long_name)`).
- Cristalino (ANTES): o caminho de chamada avaliava o callee como field access genérico — `01_core/src/engine/eval/bindings.rs:1733` (braço `other` → `cannot access fields on type {curto}`), `:1666` (`array does not contain field`), e `closures.rs:837-840` (`não é possível chamar {tipo}`, sem hints).

## Passo 2 — Implementação

1. **`01_core/src/engine/eval/bindings.rs`** (fim do ficheiro) — novas funções:
   - `element_or_type_with_name(&Value) -> (&'static str, String)` — mirror de `call.rs:359-365`.
   - **`field_callee_error(target, access) -> Option<Vec<SourceDiagnostic>>`** — mirror do ramo de erro de `eval_field_callee`: `None` para `Symbol`/`Func`/`Type`/`Module` (fall-through); campo existente → erro dict/args/"not a valid method" com os hints verbatim (hint "stored function" quando o valor é `Func`); campo inexistente → `{kind} {name} has no method `{field}``. `full_text` via `access.to_untyped().clone().into_text()`.
2. **`01_core/src/engine/eval/closures.rs`** (`eval_func_call`, imediatamente antes da avaliação genérica do callee) — intercepção P815: para callee `Expr::FieldAccess`, avalia o target e chama `field_callee_error`; `Some(err)` → retorna o erro. Corre **depois** de todos os despachos legítimos (P417/P423/P504/P717/P466/P702/P710/P712/P742/P792/P796/P506/P707), logo métodos reais ficam intactos.
3. **Teste P742 actualizado** (`eval/tests.rs:11480`): `p742_metodo_desconhecido_cai_no_caminho_generico` fixava a mensagem antiga; medido no vanilla (`m18`) que a nova é `type color has no method `foo`` — teste e comentário actualizados.
4. **L0 `00_nucleo/prompts/engine/eval.md`** — nova secção §P815 (decisão, mirror, scope-out das variantes de hint `in_math`, hipótese de distância de edição refutada, critérios verbatim). `crystalline-lint --fix-hashes .` → novo hash `c11a05c6` nos ficheiros com `@prompt engine/eval.md`.

## Passo 3 — Validação (medição DEPOIS)

`./target/release/typst <doc>.typ -o <out>.pdf`, mesmas fixtures:

```text
m1:  m1.typ:1:1:  error: type integer has no method `foo`                          (= vanilla)
m2:  m2.typ:1:1:  error: type string has no method `metodo_inexistente`            (= vanilla)
m3:  m3.typ:2:1:  error: cannot directly call dictionary keys as functions
       hint: to access the `x` key, remove the function arguments: `d.x`
       hint: dictionary keys cannot be used with method syntax as keys could conflict with built-in method names   (= vanilla)
m4:  m4.typ:2:1:  error: type dictionary has no method `zzz`                       (= vanilla)
m5:  m5.typ:2:1:  error: cannot directly call dictionary keys as functions
       hint: to call the stored function, wrap the field access in parentheses: `(d.f)(..)`
       hint: dictionary keys cannot be used with method syntax ...                 (= vanilla; bug da chamada eliminado)
m6:  cannot mutate a temporary value                                             (controlo ✓)
m7:  error: type float has no method `foo`                                       (= vanilla)
m8:  error: type array has no method `zzz`                                       (= vanilla)
m9:  error: cannot access fields on type int                                     (INALTERADO — caminho não-chamada; vanilla: "... type integer". Registado)
m10: exit 0, "1"                                                                 (controlo ✓)
m11: error: `abs` is not a valid method for type `length` + hint                 (= vanilla)
m12: error: cannot directly call named argument fields as functions + 2 hints
       (vanilla: `type arguments has no method `positional`` — o vanilla não tem
        o campo `.positional` em arguments; o cristalino tem (P504) e agora produz
        a forma vanilla para campo-que-existe. Nuance registada)
m13: error: `body` is not a valid method for element `strong` + hint             (= vanilla)
m14: error: element strong has no method `func`                                  (métodos de content em falta — achado adjacente; vanilla: exit 0 `strong`)
m15: error: element strong has no method `zzz`                                   (= vanilla)
m17: error: chamada em modo math espera função, recebeu int                      (INALTERADO — despacho math separado; registado)
m18: error: type color has no method `foo`                                       (= vanilla)
```

**Testes novos** (15 em `01_core/src/engine/eval/tests.rs`, prefixo `p815_`): 11 confirmados a falhar ANTES (`0 passed; 11 failed` no filtro — os 4 controlos já passavam, como esperado) e 15/15 a passar DEPOIS.

**Suítes:**
- `cargo test -p typst-core`: ANTES `4380 passed; 0 failed; 2 ignored` → DEPOIS **`4395 passed; 0 failed; 2 ignored`** (+15 novos).
- `cargo test -p typst-infra`: **`661 passed; 0 failed; 5 ignored`** — inalterado (a alteração não toca mensagens que a infra testa).

**Lint:** `crystalline-lint .` → **zero violations** (só os 3 warnings V7 de prompts órfãos pré-existentes, não relacionados).

## Achados adjacentes registados (medidos neste passo; fora do âmbito do achado #2)

1. **Caminho não-chamada** (m9): `#(1).foo` sem parênteses → cristalino `cannot access fields on type int` (nome curto, span 1:1) vs vanilla `cannot access fields on type integer` (nome longo, span do field 1:5). `eval_value_field_access` usa `type_name()` curto no braço `other` (`bindings.rs:1733-1736`).
2. **Métodos de content em falta** (m14): `.func()`/`.has()`/`.at()`/`.fields()`/`.location()` do vanilla não existem no cristalino — `#strong[x].func()` devolve `strong` no vanilla. Após P815, métodos desconhecidos de content erram com a forma vanilla (`element strong has no method `x``), mas os métodos reais continuam por implementar.
3. **Despacho de chamada em math** (m17): `$#d.x()$` segue um caminho separado (`engine/eval/math.rs`) com mensagem própria (`chamada em modo math espera função, recebeu int`) — não passa por `eval_field_callee` nem no vanilla (que produz o erro dict-key + hints não-math, medido).
4. **Campos extra do cristalino**: `arguments.positional`/`.named` (P504) e `array.len`/`.first`/`.last` como campos (P493a) não existem como campos no vanilla (são métodos) — m12 mede a consequência. Registado para auditoria de substância dos L0s respectivos.
