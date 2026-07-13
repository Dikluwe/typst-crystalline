# Relatório P728 — short-circuit de `and`/`or` + `join` em code block

**Data:** 2026-07-13
**Passo:** `00_nucleo/diagnosticos/typst-passo-728.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir), ADR-0114 (sonda antes da spec — mudança em mecanismo central de avaliação).
**Commit:** A PREENCHER
**Proveniência das medições (regra de proveniência):** commit base `c07e22af83827974a628ab69f9fad8eececcef00` ("P727: preenche hash do commit no relatório"), working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/eval.md`, `00_nucleo/prompts/rules/eval/ops.md`, `01_core/src/rules/eval/mod.rs`, `01_core/src/rules/eval/operators.rs`, `01_core/src/rules/eval/tests.rs` + headers `@prompt-hash` retocados pelo `--fix-hashes` em `bibliography.rs`, `closures.rs`, `control_flow.rs`, `flow.rs`, `markup.rs`, `math.rs`, `modules.rs`, `rules.rs`). Medições vanilla: binário `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T20:39Z).

---

## Sonda — medições antes de decidir (ADR-0108/ADR-0114)

### Bug 1 — `and`/`or` sem short-circuit (confirmado)

Vanilla `apply_binary` em `lab/typst-original/crates/typst-eval/src/ops.rs:52-66`: avalia `lhs`, e se `(And && lhs == false) || (Or && lhs == true)` retorna `lhs` **sem avaliar `rhs`**. O cristalino (`01_core/src/rules/eval/mod.rs`, braço genérico `Expr::Binary`) avaliava sempre os dois operandos.

Medição vanilla (`/tmp/p728-shortcircuit.typ`, pdftotext) — os 4 casos:

```
#(type(a) == str and a.contains("."))   → false
#(type(a) == array or a.contains("."))  → true
#(false and (1/0 == 0))                 → false
#(true or (1/0 == 0))                   → true
```

O cristalino errava no 1º caso ("campo desconhecido em array: 'contains'") e nos dois com divisão por zero. É o idioma "verificar antes de aceder" — semântica fundamental, usada pelo cetz em `draw/shapes.typ:608,611`.

`ops::and`/`ops::or` vanilla (`lab/typst-original/crates/typst-library/src/foundations/ops.rs:381-394`) exigem `Bool`/`Bool` — qualquer outro tipo é erro. O cristalino (`operators.rs`, braços `BinOp::And`/`BinOp::Or`) já é equivalente; só faltava o short-circuit.

### Bug 2 — a "anomalia de ordem" era ausência de `join` em code block (confirmado, não é memoização)

A anomalia notada em P727 ("`line` antes de `circle` compila mas a linha não aparece; suspeita de memoização") foi investigada primeiro pela hipótese de memoização e **refutada**: não há `#[comemo::memoize]` no eval de closures, e casos puros de closure erravam consistentemente (não dependiam de cache).

A causa real: o braço `Expr::CodeBlock` (`01_core/src/rules/eval/mod.rs`) devolvia só o valor da **última** expressão. O vanilla (`lab/typst-original/crates/typst-eval/src/code.rs:57`) faz `output = ops::join(output, value)` **por expressão**. Medições vanilla vs cristalino (pré-correcção):

| Caso | Vanilla | Cristalino (antes) |
|---|---|---|
| `{ (1,); (2,) }` | `(1, 2)` | `(2)` |
| `{ "a"; "b" }` | `"ab"` | `"b"` |
| `{ none; (1,) }` | `(1,)` | `(1,)` (coincidia) |
| `{ (:); (a: 1) }` | `(a: 1)` | `(a: 1)` (coincidia) |
| `{ 1; none }` | `1` | `1` (coincidia) |
| `{ 1; 2 }` | **erro** "cannot join integer with integer" | `2` |

Isto explica a anomalia do cetz: o body `{ line(...); circle(...) }` vale `(closure_line, closure_circle)` no vanilla, mas só `(closure_circle)` no cristalino — a primeira expressão perdia-se. Confirmado com instrumentação temporária (removida; working tree verificado limpo): `line` + `circle` compilava sem a closure do `line` correr.

`ops::join` vanilla (`lab/typst-original/crates/typst-library/src/foundations/ops.rs:24-45`): `None` é identidade nos dois lados; `Str`/`Symbol` concatenam (resultado `Str`); `Bytes` concatenam; `Content` com `Content`/`Str`/`Symbol` faz sequência; `Array` concatena; `Dict` faz merge; `Args` faz merge; qualquer outra combinação é **erro** "cannot join X with Y".

O passo mandava corrigir ambos ("se a anomalia for confirmada como bug real e relacionado: corrigir também") — confirmado como bug real e directamente relacionado: os dois bugs juntos explicam os dois sintomas do cetz (erro de compilação e linha ausente).

## L0 (Prompt)

- `00_nucleo/prompts/rules/eval.md` — secção de code block actualizada: o valor do bloco é o `join` sequencial, não a última expressão; `Binary` ganha braço dedicado para `And`/`Or`.
- `00_nucleo/prompts/rules/eval/ops.md` — nova secção "P728 — Short-circuit de `and`/`or` + `join` em code block" com mecanismos vanilla file:line, tabela completa de `join` e decisão sobre a mensagem de erro: texto cristalino "cannot join {a} with {b}" com `type_name()` cristalino (divergência de texto face aos nomes vanilla aceite — mesmo padrão da fronteira genérica "cannot apply").
- `crystalline-lint --fix-hashes .` actualizou os headers afectados; `crystalline-lint .` → **0 violations**.

## Implementação

1. `01_core/src/rules/eval/operators.rs` — nova função `pub(crate) fn join(lhs: Value, rhs: Value) -> Result<Value, String>` com a tabela completa de paridade (None identidade; str/symbol/bytes/content concatenam; array concatena; dict/args fazem merge; resto erro "cannot join X with Y"). Import novo: `crate::entities::bytes::Bytes`.
2. `01_core/src/rules/eval/mod.rs`:
   - Braço `Expr::CodeBlock`: `let mut output = Value::None` e `output = operators::join(output, value)?` por expressão (span capturado antes de consumir `expr`), em vez de `last = ...`.
   - Novo braço `Expr::Binary(binary) if matches!(binary.op(), BinOp::And | BinOp::Or)` antes do genérico: avalia `lhs`; se decidido (`And`+`false` ou `Or`+`true`) retorna `lhs` sem avaliar `rhs`; caso contrário avalia `rhs` e despacha para `eval_binary_op` (que mantém a exigência Bool/Bool).
3. `01_core/src/rules/eval/tests.rs` — 16 testes P728: 8 de short-circuit (não-avaliação do rhs com `1/0`, idioma "verificar antes de aceder", caso comum com rhs avaliado, tipos inválidos erram) e 8 de join (arrays, strs, dicts, None identidade esquerda/direita, valor antes de None, join inválido erra, expressão única sem regressão, `join` unitário).

## Validação

- `cargo test -p typst-core p728` — **16 passed, 0 failed**.
- `cargo test --workspace` — **4692 passed, 0 failed** (3997 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Zero regressões — incluindo o risco conhecido de testes escritos contra o comportamento antigo ("só última expressão").
- `crystalline-lint .` — **0 violations**.

### cetz — o bloqueio mudou (número exacto do novo bloqueio)

Com a reprodução exacta do passo (`/tmp/p728-cetz.typ`, `line` + `circle`):

- **Antes de P728** (P727): compilava, mas renderizava só o círculo (1100 px não-brancos vs 1451 do vanilla) — a closure do `line` nunca corria.
- **Depois de P728**: o body do canvas passa a valer `(closure_line, closure_circle)` e a closure do `line` **corre** — a compilação avança e pára num bloqueio **novo**, registado com mensagem exacta:

```
/tmp/p728-cetz.typ:<detached>: error: campo desconhecido em array: 'slice'
```

Medição do caso mínimo (`/tmp/p728-slice.typ`, commit base + alterações deste passo):

| Caso | Vanilla | Cristalino |
|---|---|---|
| `(1,2,3,4).slice(1, 3)` | `(2, 3)` | erro "campo desconhecido em array: 'slice'" |
| `(1,2,3,4).slice(-2)` | `(3, 4)` | erro idem |

Causa: a stdlib cristalina só tem `slice` para `Str` (`01_core/src/rules/stdlib/collections.rs:79`); falta `Array.slice`. O cetz usa `array.slice` em ≥10 locais (`draw/shapes.typ:620,624,630,971,973,1949,2160`, `anchor.typ:218`, `coordinate.typ:203`, `drawable.typ:145`). Registado como aberto em `achados-adiados-cetz.md` — candidato natural a P729.

**O diff de pixels final não é medível neste passo** (a compilação pára no novo bloqueio), e a cadeia P678-728 **não fecha ainda**: P728 removeu os dois bloqueios que lhe eram atribuídos e expôs o seguinte. Número registado conforme o critério do passo: erro exacto + caso mínimo medido, em substituição do diff de pixels.

## Campos fixos cetz (estado após P728)

- `line` sozinha: compilação pára em `array.slice` (antes: erro em `and`/`or` ou linha ausente conforme a ordem).
- `line` + `circle`: ambas as closures correm; a do `line` pára em `array.slice` (`shapes.typ:620`, via `pts.slice(0, 2)`).
- `circle` sozinho: continua a renderizar (fallback P727 intacto).

## Critério de fecho do passo

- [x] Sonda completa, mecanismo confirmado, anomalia investigada e explicada (não memoização — ausência de `join`).
- [x] Short-circuit implementado e testado.
- [x] Anomalia corrigida (confirmada como bug real: ausência de `join` em code block).
- [x] Sem regressão em `cargo test --workspace` (4692 passed, 0 failed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — resultado exacto registado (novo bloqueio `array.slice`, com mensagem e caso mínimo medidos; diff de pixels ainda não medível).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p728.md`, com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (e novo achado `array.slice` registado).
- [ ] Resumo completo da cadeia P678-728 — **não aplicável**: a cadeia não fechou (bloqueio `array.slice` aberto).
