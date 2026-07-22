# Relatório — typst-passo-816: `typst_library::diag` — `#set` com propriedade inválida vira warning silencioso em vez de erro (achado #3 de P810, prioridade alta)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-816.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado**. Estado nas medições "antes" (`git diff HEAD --stat`, 2026-07-21 23:17 -03): 31 ficheiros alterados vs HEAD (+1329/-166) — alterações de P823/P824/P827/P814/P815/P821 (entre eles `engine/eval/tests.rs` +364, `engine/stdlib/loading.rs` +285, `03_infra/src/integration_tests.rs` +80, `crystalline.toml`). Estado nas medições "depois": os anteriores + os ficheiros de P816 (§Passo 2; `git diff HEAD --stat` final: 32 ficheiros, +1708/-200). Binário cristalino rebuildado (`cargo build --release`) após a implementação.
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`; **sem** subcomando `compile`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`).

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p816/`:

- `a.typ`: `#set text(nonexistent-prop: 12pt)\nHello`
- `b.typ`: `#set text(font: "FamiliaQueNaoExiste")\nHello`
- `c.typ`: `#set text(size: 12)\nHello`
- `ctrl.typ`: `#set text(size: 12pt, font: "New Computer Modern")\nHello *world*`

Comandos exactos: `lab/typst-original/target/release/typst compile <f>.typ <f>.pdf; echo $?` e `./target/release/typst <f>.typ -o <f>.cris.pdf; echo $?`. Saída literal, ANTES da implementação:

**(a)** `#set text(nonexistent-prop: 12pt)`

```text
vanilla:
error: unexpected argument: nonexistent-prop
  ┌─ a.typ:1:10
  │
1 │ #set text(nonexistent-prop: 12pt)
  │           ^^^^^^^^^^^^^^^^^^^^^^
exit=1

cristalino (ANTES):
.../a.typ:1:10: warning: text: propriedade 'nonexistent-prop' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

**(b)** `#set text(font: "FamiliaQueNaoExiste")`

```text
vanilla:
warning: unknown font family: familiaquenaoexiste
  ┌─ b.typ:1:16
  │
1 │ #set text(font: "FamiliaQueNaoExiste")
  │                 ^^^^^^^^^^^^^^^^^^^^^
exit=0

cristalino (ANTES):
(sem qualquer saída)
exit=0
```

**(c)** `#set text(size: 12)`

```text
vanilla:
error: expected length, found integer
  ┌─ c.typ:1:16
  │
1 │ #set text(size: 12)
  │                 ^^
  │
  = hint: a length needs a unit - did you mean 12pt?
exit=1

cristalino (ANTES):
(sem qualquer saída)
exit=0
```

**Controlo (ANTES):** `ctrl.typ` → exit 0 nos dois binários (sem saída).

**Sondas complementares de classificação (executadas no vanilla para decidir o desenho):**

| Documento | Vanilla | Leitura |
|---|---|---|
| `#set text(hyphenate: true)` | exit 0 | `hyphenate` é propriedade **válida** do `TextElem` → fica warning de scope-out |
| `#set text(stroke: 1pt)` | exit 0 | idem |
| `#set text(alignment: "center")` | `error: unexpected argument: alignment` @1:10, exit 1 | `alignment` **não existe** no `TextElem` → erro |
| `#set text(leading: 0.65em)` | `error: unexpected argument: leading` @1:10, exit 1 | `leading` é de `par`, não de `text` → erro |
| `#set text(baseline: 3pt)` | exit 0 | válida no vanilla → warning de scope-out |
| `#set text(tracking: 1)` | `error: expected length, found integer` @1:20 + hint `did you mean 1pt?`, exit 1 | mesmo defeito de tipo de (c) |

**Código identificado:**

- Vanilla (a): `lab/typst-original/crates/typst-library/src/foundations/args.rs:257-266` — `Args::finish` faz `bail!("unexpected argument: {name}")` para qualquer argumento nomeado não consumido pelo `#[elem]`.
- Vanilla (b): `lab/typst-original/crates/typst-library/src/text/mod.rs:1577-1588` — `check_font_list` emite `warning!("unknown font family: {}", family)` por família literal ausente do `FontBook`; chamada **no parse do argumento `font`** (`text/mod.rs:170-176`, `#[parse({ ... })]` do campo `font`), ou seja, na avaliação do set rule — **não** no shaping. O nome sai lowercased (normalização de `FontFamily`).
- Vanilla (c): `lab/typst-original/crates/typst-library/src/foundations/cast.rs:325-343` — mensagem `expected length, found integer` + hint `a length needs a unit - did you mean {i}pt?` para `Int`.
- Lista de propriedades settable do `TextElem` vanilla: `text/mod.rs:182-788` (campos `pub` do `#[elem]`).
- Cristalino (a): `01_core/src/engine/eval/rules.rs` — arm `_` do `set text` (ANTES: `:1533-1539`) emitia warning via `unsupported_property_warn` (`:125-136`) para **qualquer** nome desconhecido, sem distinguir "válido no vanilla mas não capturado" de "inexistente no vanilla".
- Cristalino (b): ausência — o arm `font` construía a lista e empurrava para a chain sem consultar o `FontBook`; o fallback silencioso acontecia mais tarde em `03_infra/src/shaper.rs:178-195` (`shaped_width`/`resolve_candidates`, `:533-554`).
- Cristalino (c): `01_core/src/engine/eval/rules.rs` — arms `size` e `tracking` (ANTES: `:1337-1342`, `:1411-1416`) ignoravam silenciosamente valores não-`Length`.

## Passo 2 — Implementação

Tudo em L1 (`01_core`) — o ponto lógico do vanilla para os três sub-achados é a **avaliação do set rule**, que no cristalino vive em `01_core/src/engine/eval/rules.rs`. O `FontBook` é acessível em L1 via `engine.world.book()` (contrato `contracts/world.rs:37`, entidade pura `entities/font_book.rs`) — pureza de L1 preservada (zero I/O novo). `03_infra/src/shaper.rs` **não** foi tocado: o warning no ponto de shaping seria tarde de mais e fora do sítio onde o vanilla o emite (medido acima).

1. **`01_core/src/engine/eval/rules.rs`**:
   - Nova constante `VANILLA_TEXT_SET_PROPS` — as 32 propriedades nomeadas settable do `TextElem` vanilla (`text/mod.rs:182-788`), em kebab-case.
   - Nova helper `expected_length_error(found, span)` — `expected length, found {type}` com nome de tipo no vocabulário do vanilla (`int`→`integer`, `str`→`string`, `bool`→`boolean`; o `type_name()` cristalino diverge, P636) + hint `a length needs a unit - did you mean {i}pt?` para `Int`.
   - **(a)** Arm `_` do `set text`: nome fora de `VANILLA_TEXT_SET_PROPS` → **erro hard** `unexpected argument: {name}` (span no nome, mesmo formato do arm `bold`/`italic` de §P665); nome dentro da lista mas não capturado → mantém o warning de scope-out do Passo 107 (hint ADR-0040).
   - **(b)** Arm `font`: após construir o array de famílias, cada nome literal (`Value::Str` de topo ou `name` Str dentro de dict) ausente de `engine.world.book()` (`select_family`) emite warning `unknown font family: {nome lowercased}` com span no valor do argumento. Nomes `Value::Regex` não verificados (como no vanilla, que só avisa sobre `FontFamily` literais).
   - **(c)** Arms `size` e `tracking`: valor não-`Length` → `expected_length_error` (antes: ignorado em silêncio).
2. **`00_nucleo/prompts/engine/eval.md`**: nova secção **§P816** (medição vanilla + regras + critérios de verificação), precedendo §P665. `crystalline-lint --fix-hashes .` executado (actualizou `engine/eval/tests.rs` para `54460ea7`; `rules.rs` manteve `2f2e3e80` — o lint confirma 0 drift V5).
3. **Testes** (escritos contra o comportamento ANTES medido na sonda — warning+exit 0 / silêncio — que eles contradizem):
   - `01_core/src/engine/eval/tests.rs` (+6): `p816_set_text_propriedade_inexistente_erro`, `p816_set_text_propriedade_vanilla_nao_implementada_warning` (controlo, `baseline`), `p816_set_text_size_int_erro` (mensagem + hint), `p816_set_text_tracking_int_erro`, `p816_set_text_font_desconhecida_warning`, `p816_set_text_font_conhecida_sem_warning` (controlo com `FontBook` populado).
   - `01_core/src/engine/eval/tests.rs` (migrado): `eval_set_text_leading_emite_warning_passo_134` — o warning do Passo 134 para `#set text(leading:)` foi promovido a erro hard (paridade medida: vanilla `unexpected argument: leading`); o teste passa a esperar `Err`.
   - `03_infra/src/integration_tests.rs` (+3): `p816_set_text_propriedade_inexistente_erro`, `p816_set_text_size_int_erro`, `p816_set_text_font_desconhecida_warning`.
   - `03_infra/src/integration_tests.rs` (migrados): `debt49_set_text_alignment_emite_warning` e `debt49_set_text_multiplas_propriedades_desconhecidas` — o canary `alignment` virou erro hard (não existe no vanilla); rotação para `baseline` (válida no vanilla, não capturada).

Diff (resumo `git diff HEAD --stat`, só ficheiros de P816):

```text
 00_nucleo/prompts/engine/eval.md  |  +81  (secção §P816; inclui alterações anteriores da tree)
 01_core/src/engine/eval/rules.rs  | +142/-~10 (constante, helper, 4 arms)
 01_core/src/engine/eval/tests.rs  |  +6 testes novos + teste P134 migrado (+163/-~20 deste passo)
 03_infra/src/integration_tests.rs |  +3 testes novos + 2 canários migrados (+82/-~20 deste passo)
```

## Passo 3 — Validação (medição DEPOIS)

Mesmos comandos da sonda, com o binário rebuildado:

**(a)** `#set text(nonexistent-prop: 12pt)`

```text
cristalino (DEPOIS):
.../a.typ:1:10: error: unexpected argument: nonexistent-prop
exit=1
```

**(b)** `#set text(font: "FamiliaQueNaoExiste")`

```text
cristalino (DEPOIS):
.../b.typ:1:16: warning: unknown font family: familiaquenaoexiste
exit=0
```

**(c)** `#set text(size: 12)`

```text
cristalino (DEPOIS):
.../c.typ:1:16: error: expected length, found integer
  hint: a length needs a unit - did you mean 12pt?
exit=1
```

Mensagens, spans (1:10 no nome / 1:16 no valor) e exit codes **batim com o vanilla** nos três sub-achados (formato de envelope cristalino `path:line:col:` vs carets do vanilla — apresentação, fora de âmbito).

**Controlos de regressão (DEPOIS):**

| Documento | Cristalino | Esperado |
|---|---|---|
| `#set text(hyphenate: true)` | warning `text: propriedade 'hyphenate' ainda não suportada` + hint ADR-0040, exit 0 | scope-out mantido ✓ |
| `#set text(weight: "bold", style: "italic", fill: rgb(255,0,0))` | exit 0, sem saída | válido ✓ |
| `#set text(font: "DejaVu Sans")` (família presente no sistema) | exit 0, sem saída | sem falso warning ✓ |
| `#set text(size: 12pt, font: "New Computer Modern")` (ctrl) | exit 0, **com** warning `unknown font family: new computer modern` | ver nota abaixo |
| `#set text(leading: 0.65em)` | `error: unexpected argument: leading` @1:10, exit 1 | paridade vanilla ✓ |
| `#set text(tracking: 1)` | `error: expected length, found integer` @1:20 + hint `did you mean 1pt?`, exit 1 | paridade vanilla ✓ |
| `#set text(font: ("FamiliaQueNaoExiste", "FamiliaTambemNao"))` | 2 warnings (um por família, ambos @1:16), exit 0 | paridade vanilla ✓ |

**Nota sobre o controlo `ctrl.typ`:** o vanilla não avisa sobre `New Computer Modern` porque a **embebe** no binário; o cristalino não tem fontes embutidas (divergência de inventário já existente — a pipeline cai em Helvetica quando a família não resolve) e agora avisa, correctamente face ao **seu** `FontBook`. O warning é verdadeiro para o cristalino (a família não lhe está disponível); documentos que nomeiem as fontes embutidas do vanilla (`New Computer Modern`, `Libertinus Serif`) passam a emiti-lo. Registado como consequência consciente, não como regressão: o observável "família desconhecida → warning" é o comportamento da linguagem; o inventário de fontes é mecânica/ambiente (ADR-0107).

**Suítes de testes (contagens ANTES/DEPOIS):**

- `cargo test -p typst-core` — ANTES: `4400 passed; 0 failed; 2 ignored` (estado da tree no arranque, confirmado por filtragem: 4408 total − 6 novos = 4402 = 4400+2). DEPOIS: **`test result: ok. 4406 passed; 0 failed; 2 ignored`** (+6 testes P816; o teste P134 migrado mantém a contagem).
- `cargo test -p typst-infra` — ANTES: `664 passed; 0 failed; 5 ignored` (672 total − 3 novos = 669 = 664+5). DEPOIS: **`test result: ok. 667 passed; 0 failed; 5 ignored`** (+3 testes P816; 2 canários migrados mantêm a contagem).
  - Incidente: a primeira execução DEPOIS da suíte infra (na mesma corrida de shell imediatamente a seguir ao `cargo build --release`) reportou `666 passed; 1 failed`. Três re-execuções independentes reportaram `667 passed; 0 failed` de forma estável — tratado como flake de ambiente/paralelismo (nome do teste não capturado; a saída da corrida original foi truncada pelo `grep` de resumo). Registado para proveniência; não reproduzível em 3 tentativas.
- `crystalline-lint .` — **0 violations novas**: 0 V5 (drift) após `--fix-hashes`; restam apenas 6 warnings V7 ("prompt órfão") pré-existentes e alheios a este passo (`structural.md`, `package_version_resolution.md`, etc.), presentes antes da alteração.

## Passo 4 — Estado final

| Sub-achado | ANTES (cristalino) | DEPOIS (cristalino) | Vanilla | Estado |
|---|---|---|---|---|
| (a) prop inválida | warning + exit 0 | `unexpected argument: nonexistent-prop` + exit 1 | erro + exit 1 | **fechado** |
| (b) fonte desconhecida | silêncio | `unknown font family: {nome}` + exit 0 | warning + exit 0 | **fechado** |
| (c) `size: <int>` | aceite em silêncio | `expected length, found integer` + hint + exit 1 | erro + hint + exit 1 | **fechado** |

Follow-ups registados (fora de âmbito, não medidos neste passo): arms `fill`, `top-edge`, `bottom-edge`, `dir` e `lang` ainda ignoram silenciosamente tipos errados (o vanilla erra via cast); os outros targets de `#set` (`par`, `smartquote`, `heading`, …) continuam a tratar qualquer nome desconhecido como warning — a regra "nome inexistente → erro" foi aplicada apenas a `text`, o alvo medido no achado.
