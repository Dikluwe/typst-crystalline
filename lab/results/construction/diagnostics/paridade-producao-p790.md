# P790 — Show rule por string (`#show "texto": ...`) aceita e não aplicada em silêncio

> **Passo:** 790
> **Data:** 2026-07-20, medições entre ~18:45Z e ~19:30Z
> **Commit:** `0774275fe` (+ working tree não commitado: 68 ficheiros com alterações, herdadas de passos anteriores incl. P789 — `git diff HEAD --stat` = 55 files tracked, +1650/−188; a estes somam-se as deste passo: `00_nucleo/prompts/engine/eval.md`, `00_nucleo/prompts/entities/show.md`, `01_core/src/engine/eval/rules.rs`, `01_core/src/engine/eval/tests.rs`, `01_core/src/engine/layout/tests.rs`, headers de hash nos consumidores dos L0)
> **Binários:** vanilla `lab/typst-original/target/release/typst` = typst 0.15.0 (rev `969087ec`); cristalino `./target/release/typst` rebuildado neste passo com a correcção
> **Regras aplicadas:** ADR-0107 (warnings/erros são observáveis ao nível da língua), ADR-0108 (medir antes de decidir — ver §2, que corrige o enquadramento do passo), regra de proveniência de P569.

---

## Resumo em uma linha

**Fechadas as duas divergências de P786 em `eval::rules`**: `#show "texto": [W]` agora substitui de facto ("Hello W.", confirmado por `pdftotext`), e `#show page:`/`#show par: set block(spacing: ..)` emitem os warnings exactos do vanilla com exit 0 — mais dois itens medidos no caminho: selector de texto vazio (erro vanilla reproduzido) e `#show "world": "W"` (já funcionava; confirmado par).

---

## 1. Sonda — mecanismo vanilla (confirmado antes de implementar)

- **Show-by-string**: `#show "texto": …` — a string vira `Selector::text` → `Selector::Regex(regex::escape(text))` (`typst-library/src/foundations/selector.rs:108-113`; string vazia → `bail!("text selector is empty")`). A aplicação acontece na **realização** (`typst-realize/src/lib.rs`): `find_regex_match_in_str` (linha 1321) acha o match leftmost; `visit_regex_match` (linha 1391) **fatia o texto à volta do match** e aplica o recipe ao texto do match, emendando o output no lugar — o texto envolvente é preservado; a regra é revogada para o próprio output (`Style::Revocation`, sem re-varrimento).
- **`#show page:`** — `check_show_page_rule` (`typst-eval/src/rules.rs:67-77`): selector `PageElem` (qualquer transformação) → warning `` `show page` is not supported and has no effect `` + hint `` customize pages with `set page(..)` instead ``. Exit 0.
- **`#show par: set block(spacing: ..)`** — `check_show_par_set_block` (`rules.rs:80-95`): selector `ParElem` + transformação Style com `BlockElem::above`/`below` → warning `` `show par: set block(spacing: ..)` has no effect anymore `` + 2 hints (`` write `set par(spacing: ..)` instead `` / `this is specific to paragraphs as they are not considered blocks anymore`). Exit 0.
- **`#show par: it => …`** — medido: é regra **viva** no vanilla (`#show par: it => [PARA: #it]` erra por recursão — `maximum show rule depth exceeded` — logo o selector funciona). Não há warning genérico de "show par não suportado".

## 2. Medição que corrige o enquadramento do passo (ADR-0108)

O repro sugerido no corpo do passo tem o texto **antes** da regra:

```text
Hello world.
#show "world": [W]
```

Medido no vanilla: renderiza **"Hello world."** — inalterado, exit 0. Show rules são lexicamente confinadas ao conteúdo **seguinte**; o passo descrevia "deveria mostrar 'Hello W.'" para essa ordem, o que não é o comportamento vanilla. A evidência real de P786 (`temp/temp_p786/c_rules_probe_show.typ`) tem a ordem certa (regra primeiro) — vanilla: **"Hello W."**; cristalino (antes): **"Hello world."** (regra aceite e descartada em silêncio). Divergência confirmada nessa base.

**Levantamento do estado cristalino (medido):**

| Caso | vanilla | cristalino (antes) |
|---|---|---|
| `#show "world": [W]` (regra 1ª) | "Hello W." | "Hello world." — Content/Func sobre `Selector::Text` **descartados em silêncio** (`apply_show_rules` só aplicava `Transformation::Str` via `map_text`) |
| `#show "world": "W"` | "Hello W." | "Hello W." — **já par** (medido, sem acção) |
| `#show "": [X]` | erro `text selector is empty`, exit 1 | exit 0, ignorado em silêncio |
| `#show page: it => …` | warning + hint, exit 0 | `error: unknown variable: page`, exit 1 |
| `#show par: set block(spacing: 4em)` | warning + 2 hints, exit 0 | `error: unknown variable: par`, exit 1 |
| `#show par: it => …` | regra viva (erra só por recursão) | `error: unknown variable: par`, exit 1 |

Nota de L0: `entities/show.md` afirmava "Func/Content sobre Text falham explicitamente (DEBT-19 ENCERRADO)" — **falso no código** (eram descartados em silêncio, sem erro). Corrigido em P790: passam a ser suportados (abaixo).

## 3. Decisão de âmbito (registada)

| Item | Decisão |
|---|---|
| Text + `Content`/`Func` (splice) | **Implementado** — o achado central de P786 |
| `#show page:` (qualquer transformação) | **Implementado** — warning exacto, sem regra, exit 0 |
| `#show par: set block(spacing/above/below)` | **Implementado** — warning exacto + 2 hints, sem regra, exit 0 |
| Selector de texto vazio | **Implementado** — erro `text selector is empty` (mesma construção `Selector::text`; medido) |
| `#show par: <outra transformação>` | **Scope-out com erro explícito** — no vanilla é regra viva sobre `ParElem`; implementar show-par como element rule é passo futuro. Mensagem cristalina explícita em vez de `unknown variable` |
| Match cross-node (texto partido em vários nós) | **Scope-out registado no L0** — o vanilla agrupa elementos textuais adjacentes na mesma chain antes de casar; o cristalino casa por nó `Content::Text` individual (também afecta `Selector::Regex`, pré-existente) |

## 4. Implementação

Protocolo de Nucleação: L0s actualizados **antes** do código; 9 testes escritos primeiro e confirmados a falhar (8 falhos + 1 controlo já verde).

- **L0** — `eval.md`: dois bullets novos (splice Text+Content/Func; page/par como alvos especiais). `entities/show.md`: descrição de `Selector::Text`, de `Transformation::Str` e a invariante reescritas (a anterior era falsa — ver §2). Hashes via `crystalline-lint --fix-hashes .` (`eval.md` → `db871ac7`, `show.md` → `377d2319`).
- **`01_core/src/engine/eval/rules.rs`**:
  - `eval_show_rule`: intercepta `Expr::Ident` `page`/`par` **antes** de avaliar o selector (não existem como variáveis); `page` → `sink.warn_note` com texto/hint do vanilla; `par` + `set block` com `spacing`/`above`/`below` → `sink.warn_note2` com texto/hints do vanilla; `par` com outra transformação → erro explícito. `Value::Str` vazio no selector → erro `text selector is empty`.
  - `apply_show_rules`: text rules reescritas — `Str` via `map_text` (inalterado); `Content`/`Func` por **splice** via `splice_text_rule_matches` (nova free function): fatia o `Content::Text` nas ocorrências, emenda o replacement por ocorrência, preserva o texto envolvente; `Func` é chamada com o match como `Content::Text` (output Content/Str emendado; outro tipo → erro). `map_content` não reentra no nó substituído — sem re-varrimento (equivalente à `Revocation` do vanilla).
- **Testes** — 4 de layout (`tests_show_rule_integration`): repro de P786, múltiplas ocorrências, func que duplica o match, controlo sem-match; 5 de eval: splice no eval, warnings exactos de page/par com hints, selector vazio, `show par` não suportado.

## 5. Validação (binário release rebuildado)

```text
$ ./target/release/typst temp/temp_p786/c_rules_probe_show.typ → exit 0
pdftotext: "Hello W."                              (vanilla: "Hello W." — PAR)

$ ./target/release/typst temp/temp_p786/c_rules_showpage.typ → exit 0
warning: `show page` is not supported and has no effect (@1:1)
  hint: customize pages with `set page(..)` instead   (idêntico ao vanilla)

$ ./target/release/typst temp/temp_p786/c_rules_showpar.typ → exit 0
warning: `show par: set block(spacing: ..)` has no effect anymore (@1:1)
  hint: write `set par(spacing: ..)` instead
  hint: this is specific to paragraphs as they are not considered blocks anymore

$ ./target/release/typst /tmp/p790-empty.typ → exit 1
error: text selector is empty (@1:6)                (idêntico ao vanilla)

$ ./target/release/typst /tmp/p790-showstring.typ (texto ANTES da regra) → exit 0
pdftotext: "Hello world."                           (inalterado — PAR com o vanilla medido, §2)
```

- `cargo test --workspace` — **verde**: 4289 passed / 0 failed no `typst-core` (inclui os 9 `p790_*`; antes 4280), 655 + 33 + 29 + 2 nas restantes. **Registo de flakiness**: a 1ª corrida teve 1 falha em `export::tests::p269_pdf_bytes_cluster_3_variants_pos_focal_reproduzivel` (determinismo de bytes de PDF de gradientes — não eval, não toca show rules); passou 5/5 isolado e verde na re-corrida completa — flake pré-existente, não regressão (medido, não assumido).
- `crystalline-lint .` — **exit 0, zero violações** (2 warnings V7 de prompts órfãos pré-existentes, não relacionados: `eval/field-access.md`, `infra/package_version_resolution.md`).

## 6. Critério de fecho do passo — checklist

- [x] Mecanismo de show-by-string do vanilla confirmado (§1 — `Selector::text` → Regex escaped; splice em `visit_regex_match`).
- [x] Mensagens de warning para `page`/`par` confirmadas palavra por palavra (§1 — medidas por execução, reproduzidas byte a byte).
- [x] `#show "texto": ...` de facto substitui o conteúdo (§5 — `pdftotext` "Hello W.", não só exit code).
- [x] `#show page`/`#show par` aceite com warning, não erro fatal (§5).
- [x] `cargo test --workspace` verde (§5 — com registo do flake p269).
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p790.md`.

## 7. Próximo passo (candidatos)

- Conforme indicado no passo: selectors por label (`#show <lbl>: ...` — P786 §5, `foundations::selector`), context/layout eval (`#layout`, `text.lang`, `here().position()`), numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes.
- Novos registados neste passo: (a) `show par` como element rule viva (§3 — scope-out); (b) match cross-node de texto para `Selector::Text`/`Regex` (§3); (c) flake de `p269_pdf_bytes_cluster_3_variants_pos_focal_reproduzivel` (§5 — determinismo de export, candidato a investigação própria).
