# Relatório P733 — Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) é erro

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-733.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir).
**Commit:** `0cfa0c8a15adf031c7f879a83f8a868aef97687c`
**Proveniência das medições (regra de proveniência):** commit base `58158cdd0ea5235698057617579957bf87d93453` ("P732: preenche hash do commit no relatório"), branch `Tekt`, working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/eval.md`, `01_core/src/rules/eval/closures.rs`, `01_core/src/rules/eval/tests.rs` + 8 ficheiros `eval/*.rs` só com o header `@prompt-hash` sincronizado `a3904d9a` → `45bc9822`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T22:52Z).

---

## Sonda — medições antes de decidir (ADR-0108)

Caso irmão de P708 (posicional extra, já corrigido): um argumento **nomeado** sem parâmetro correspondente era aceite silenciosamente. Era scope-out explícito de §P708 no L0 (`eval.md`), registado em `achados-adiados-cetz.md`.

### Comportamento exacto no vanilla (medido, binário `lab/typst-original/target/release/typst`)

| Caso | Vanilla | Cristalino (antes) |
|---|---|---|
| `#let f(a) = a; f(1, z: 2)` | **erro "unexpected argument: z"** (exit 1) | exit 0 — **aceite silenciosamente** |
| `#let f(a, named: 10) = (a, named); f(1, named: 20)` | `(1, 20)` | `(1, 20)` ✓ |
| `#let f(a, ..rest) = rest; f(1, z: 2, y: 3)` | `arguments(z: 2, y: 3)` — sink absorve | absorve ✓ (repr difere: `arguments(...)`) |
| `f(1, 2, z: 3)` (posicional + nomeado extra) | erro "unexpected argument" | erro "unexpected argument" ✓ (P708) |
| `f(1, z: 2, y: 3)` (múltiplos extras) | erro "unexpected argument: z" — **o primeiro** | exit 0 |
| `#let f(a, named: 10, ..rest) = rest; f(1, named: 20, z: 3)` | `arguments(z: 3)` — **sink exclui o nomeado consumido** | sink recebia `args.named` inteiro (incluía `named: 20`) |
| `f(1, z: 2, 3)` (ordem entre tipos) | erro "unexpected argument: z" — primeiro na ordem original | — |

### Mecanismo vanilla (fonte, não assumido)

`lab/typst-original/crates/typst-eval/src/call.rs:650-694` + `foundations/args.rs`:
- `args.named::<T>(name)` (`args.rs:218`) **consome** o nomeado durante o binding.
- O sink recebe `args.take()` — só os argumentos não consumidos.
- `args.finish()` (`args.rs:259-268`) reporta o **primeiro** argumento não consumido na ordem original: com nome → `"unexpected argument: {name}"`; sem nome → `"unexpected argument"`.

### Localização exacta no cristalino

`01_core/src/rules/eval/closures.rs` — `apply_closure`: linha 231 lia `args.named.get(param.name)` **sem consumir**; linha 265 passava `args.named` inteiro ao sink; não existia verificação de nomeados remanescentes no braço sem sink (só a posicional de P708, linhas 268-276).

### Critério de fecho da sonda

- [x] Mensagem de erro exacta confirmada: `unexpected argument: z` (com o nome do argumento).
- [x] Caso de não-regressão confirmado: `f(1, named: 20)` → `(1, 20)`.
- [x] Localização exacta confirmada (file:line acima).

## L0 (Prompt)

`00_nucleo/prompts/rules/eval.md` — nova secção **§P733** com o mecanismo vanilla (file:line), a tabela de medições completa, a correcção (consumo por `shift_remove`; verificação posicional-primeiro; sink só com não consumidos) e a divergência de canto registada (ordem entre tipos). O scope-out de §P708 foi marcado como **fechado em P733**. `crystalline-lint --fix-hashes .` → headers sincronizados (`@prompt-hash` `a3904d9a` → `45bc9822` nos ficheiros vinculados); `crystalline-lint .` → **0 violations**.

## Implementação

`01_core/src/rules/eval/closures.rs` (`apply_closure`):

1. `args: Args` → `mut args: Args` (a assinatura já recebia por valor).
2. Loop de binding: `args.named.get(param.name.as_str())` → `args.named.shift_remove(param.name.as_str())` — o nomeado consumido sai do mapa (paridade `args.named()` do vanilla). `shift_remove` preserva a ordem dos restantes (necessário para reportar o primeiro remanescente).
3. Braço do sink: sem alteração de código — passa a receber automaticamente só os nomeados não consumidos (paridade `args.take()`, medido: `arguments(z: 3)`).
4. Braço sem sink: `else if pos_idx < ...` → bloco com duas verificações na ordem medida — posicional primeiro (`"unexpected argument"`, P708), depois o primeiro nomeado remanescente na ordem de inserção (`format!("unexpected argument: {k}")`).

**Divergência de canto registada** (não corrigida): `f(1, z: 2, 3)` — o vanilla reporta o nomeado (primeiro na ordem original, lista única `Args.items`); o cristalino, com `items`/`named` separados, reporta o posicional primeiro. Ambos erram; registado em `achados-adiados-cetz.md`.

## Validação

- Fail-first confirmado: `cargo test -p typst-core p733` antes da implementação → **2 failed / 3 passed** (falhavam os dois testes do comportamento novo; passavam os três de não-regressão).
- Depois: **5 passed, 0 failed** — `p733_nomeado_extra_sem_parametro_e_erro` (mensagem exacta), `p733_nomeado_valido_sem_regressao`, `p733_sink_absorve_nomeados_extra`, `p733_sink_exclui_nomeado_consumido_por_parametro`, `p733_ordem_posicional_reportado_primeiro` (todos em `eval/tests.rs`).
- `cargo test --workspace` — **4732 passed, 0 failed** (4037 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Pré-P733: 4727; +5 = os novos testes do passo.
- Testes dos passos que tocam o mesmo mecanismo (atenção pedida pelo passo): **55 testes P504/P707/P708/P715/P724 verdes**, sem nenhuma alteração.
- `crystalline-lint .` — **0 violations**.
- E2E do passo (ficheiros exactos):
  - `/tmp/p733-named-extra.typ` → **`error: unexpected argument: z`** (exit 1) — mensagem idêntica ao vanilla.
  - `/tmp/p733-named-ok.typ` → **`(1, 20)`** — idêntico ao vanilla.
  - `/tmp/p733-sink.typ` → compila (exit 0); o conteúdo do sink está correcto (`rest.named()` → pares, coberto por teste); a repr (`arguments(...)` vs `arguments(z: 2, y: 3)`) fica registada como achado cosmético.

## Critério de fecho do passo

- [x] Sonda completa, mensagem de erro e caso de não-regressão confirmados (tabela acima, incluindo sink e ordem de reporte).
- [x] Implementado e testado — erro com a mensagem exacta do vanilla; sink continua a absorver nomeados extra sem erro (e agora exclui consumidos, como o vanilla); nomeados válidos sem regressão.
- [x] Sem regressão em `cargo test --workspace` (4732 passed, 0 failed); testes P504/P707/P708/P715/P724 confirmados intactos (55 verdes).
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p733.md`, com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (e dois novos achados registados: repr lossy de `Value::Args`; ordem entre tipos no erro).
