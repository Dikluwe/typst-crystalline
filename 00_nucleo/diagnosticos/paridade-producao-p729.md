# Relatório P729 — auditoria: `join` sequencial em todos os corpos de bloco

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-729.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir), ADR-0114 (sonda antes da spec — mecanismo central de avaliação).
**Commit:** `9cf87bf044c6823e0cbcd1df1e35e6fe3a5cb50b`
**Proveniência das medições (regra de proveniência):** commit base `809983e5c1a80f4929e96f5b594a50cb70f39784` ("P728: preenche hash do commit no relatório"), working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/eval.md`, `00_nucleo/prompts/rules/eval/ops.md`, `01_core/src/rules/eval/control_flow.rs`, `01_core/src/rules/eval/tests.rs` + headers `@prompt-hash` retocados pelo `--fix-hashes` em `bibliography.rs`, `closures.rs`, `flow.rs`, `markup.rs`, `math.rs`, `mod.rs`, `modules.rs`, `operators.rs`, `rules.rs`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T20:55Z).

---

## Sonda — medições antes de decidir (ADR-0108/ADR-0114)

P728 corrigiu o `join` sequencial só em `Expr::CodeBlock`. Este passo auditou, construto a construto, todos os sítios com o padrão "sequência de expressões → um valor" — mesmo padrão de duplicação já visto em P715 (`let`/atribuição), P723 (`for`), P724 (parâmetros de closure).

### Medições vanilla (caso combinado, `/tmp/p729-join-todos.typ`, pdftotext)

```
#if true { (1,); (2,) }                     → (1, 2)
#if false { (9,) } else { (1,); (2,) }      → (1, 2)
#let f() = { (1,); (2,) }  #f()             → (1, 2)
#for i in (1,) { (1,); (2,) }               → (1, 2)
#let i = 0  #while i < 1 { i += 1; (1,); (2,) } → (1, 2)
```

Os cinco outputs são `(1, 2)` — o join aplica-se a **todos** os corpos.

### Estado do cristalino, construto a construto (casos isolados, `/tmp/p729-c1..c5.typ`)

| Construto | Cristalino pré-P729 | Veredito |
|---|---|---|
| `if`/`else` | `(1, 2)` | **correcto** — delega em `Expr::CodeBlock` (P728) |
| corpo de closure | `(1, 2)` | **correcto** — `closures.rs:289` avalia o corpo como `Expr` (CodeBlock) |
| `for` | **erro** "corpo do for deve ser content, encontrado array" | **bug** — implementação própria em `control_flow.rs:143-154` |
| `while` | **output vazio** (sem erro) | **bug** — `control_flow.rs:62` descartava o valor do corpo; `Ok(Value::None)` no fim |

### Localização exacta (file:line) e mecanismo vanilla

- `while`: vanilla `typst-eval/src/flow.rs:69,86` — `let mut output = Value::None;` + `output = ops::join(output, value).at(body.span())?` por iteração, **antes** do match de flow. Cristalino: `control_flow.rs:62` avaliava o corpo e descartava o valor (`eval_expr(...)?;`).
- `for`: vanilla `typst-eval/src/flow.rs:120,132` — mesmo padrão. Cristalino: `run_for_loop` (`control_flow.rs:122-175`) aceitava só `Content`/`Str`/`None` no corpo (acumulava em `parts: Vec<Content>` + `Content::sequence`) e **errava** para qualquer outro tipo.
- Auditoria por grep (`fn eval_if|eval_for|eval_while|eval_closure|last = |output = ` em `01_core/src/rules/eval/*.rs`): os restantes `output =` são destructuring em `bindings.rs` (mecanismo distinto) e o CodeBlock de P728. Não há mais implementações separadas do padrão.

### Critério de fecho da sonda

- [x] Todos os construtos testados contra o vanilla, um a um, não assumidos.
- [x] Lista completa com bug confirmado: `for` e `while`, com file:line.
- [x] Confirmado: `operators::join` (P728) é directamente reaproveitável nos dois casos — sem lógica própria.

## L0 (Prompt)

- `00_nucleo/prompts/rules/eval.md` — entradas `Expr::WhileLoop`/`Expr::ForLoop` reescritas: o valor do corpo de cada iteração acumula com `operators::join` entre iterações (paridade `flow.rs:69,86` e `:120,132`), substituindo a spec antiga ("concatena Content/Str numa sequence").
- `00_nucleo/prompts/rules/eval/ops.md` — nova sub-secção "P729 — `join` também entre iterações de `for`/`while`" na secção P728 (o mecanismo de join é o mesmo; só mudam os consumidores), critérios de verificação P729, linha de histórico.
- `crystalline-lint --fix-hashes .` → headers actualizados; `crystalline-lint .` → **0 violations**.

## Implementação

`01_core/src/rules/eval/control_flow.rs`:

1. `eval_while` — `let mut output = Value::None` antes do ciclo; por iteração `output = operators::join(output, value)?` (span do corpo no erro, paridade `.at(body.span())`); `Ok(output)` no fim (antes: `Ok(Value::None)`).
2. `run_for_loop` — o acumulador `parts: Vec<Content>` + match restritivo (Content/Str/None, resto erro "corpo do for deve ser content") substituído pelo mesmo padrão de join. O erro antigo desaparece: qualquer tipo junta-se pela tabela de P728; combinações inválidas erram "cannot join X with Y" (paridade).
3. Ordem preservada em ambos: eval do corpo → join → saída de scope → match de flow (join **antes** do break/continue, como o vanilla).
4. Import de `Content` removido (deixou de ser usado); `operators` adicionado.

Nota de paridade fina (linguagem, ADR-0107): o `for` com corpo `Str` devolvia antes `Content` (texto por iteração em sequence); passa a devolver `Str` concatenada — exactamente o valor do vanilla (join de Str é Str). O render em markup é idêntico.

## Validação

- `cargo test -p typst-core p729` — fail-first confirmado antes da implementação (**6 failed / 5 passed**: os 4 de join + os 2 de erro inter-iterações falhavam; regressões e já-correctos passavam). Depois: **11 passed, 0 failed**.
- `cargo test --workspace` — **4703 passed, 0 failed** (4008 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Zero regressões.
- `crystalline-lint .` — **0 violations**.
- E2E do passo (`/tmp/p729-join-todos.typ`): cristalino produz agora `(1, 2) (1, 2) (1, 2) (1, 2) (1, 2)` — **idêntico ao vanilla, linha a linha**. Os seis construtos coincidem.

### cetz (registo — não era objectivo do passo)

`line((0,0),(2,1))` + `circle((0,0))`: a compilação continua a parar no bloqueio conhecido `campo desconhecido em array: 'slice'` (exit 1) — P729 não avança o cetz, como o passo antecipava ("não é o objectivo deste passo"). O item `array.slice` permanece o bloqueio actual em `achados-adiados-cetz.md` (candidato natural a P730).

### Achado registado (fora de scope)

Durante a leitura do vanilla para o mirror, encontrado por inspecção: `FlowEvent::Return` é marcado como condicional no fim de `while`/`for` no vanilla (`flow.rs:105-108,183-185`); o cristalino só marca em `eval_conditional` (P635). Mecanismo distinto de join, sem caso medido com comportamento divergente — registado como aberto em `achados-adiados-cetz.md`, prioridade baixa.

## Critério de fecho do passo

- [x] Sonda ampla completa, todos os construtos testados individualmente.
- [x] Cada construto com bug confirmado (`for`, `while`), corrigido e testado.
- [x] Sem regressão em `cargo test --workspace` (4703 passed, 0 failed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p729.md`, com hash do commit.
- [x] Lista de controlo (`achados-adiados-cetz.md`) actualizada (achado Return-condicional; nota de que o bloqueio `array.slice` se mantém).
