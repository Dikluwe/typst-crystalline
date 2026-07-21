# P791 — Show rule por seletor de label (`#show <lbl>: ...`)

> **Passo:** 791
> **Data:** 2026-07-20, medições entre ~19:40Z e ~22:30Z
> **Commit:** `0774275fe` (+ working tree não commitado: 69 ficheiros com alterações, herdadas de passos anteriores incl. P789/P790 — `git diff HEAD --stat` = 55 files tracked, +1944/−189; a estes somam-se as deste passo: `00_nucleo/prompts/engine/eval.md`, `00_nucleo/prompts/entities/show.md`, `01_core/src/entities/show.rs`, `01_core/src/engine/eval/rules.rs`, `01_core/src/engine/eval/mod.rs`, `01_core/src/engine/eval/tests.rs`, `01_core/src/engine/layout/tests.rs`, headers de hash)
> **Binários:** vanilla `lab/typst-original/target/release/typst` = typst 0.15.0 (rev `969087ec`); cristalino `./target/release/typst` rebuildado neste passo (22:25)
> **Regras aplicadas:** ADR-0107 (paridade do observável), ADR-0108 (medir antes de decidir — ver §1 e §2, que mudaram o enquadramento duas vezes), regra de proveniência de P569.

---

## Resumo em uma linha

**`#show <lbl>: …` implementado e a evidência completa de P786 (`b_selector.typ`) reproduz byte-idêntica ao vanilla** (`HEAD=Alpha [origLBL=] found=1`) — com duas divergências adjacentes medidas e registadas fora do âmbito (label-em-texto não indexado pelo introspector; função `#label()` sem intercepção de show).

---

## 1. Sonda — vanilla (com correção de enquadramento ADR-0108)

**Partilha mecanismo com P790? Parcialmente.** É o mesmo pipeline (`eval_show_rule` → `ShowRule` → aplicação), mas a validação do selector era o ponto de falha: `Value::Label` caía no braço `other` de `eval_show_rule` (`selector inválido para show rule: label`). O vanilla aceita via `ShowableSelector` → `Selector::Label`, com match por `target.label()` (`foundations/selector.rs:140`) na mesma realização dos element rules.

**Medição que mudou o enquadramento (1):** o probe do passo (heading **antes** da regra) **não dispara nem no vanilla** — renderiza "Alpha" (escopo léxico de show rules, mesmo achado de P790). A forma que dispara é regra-primeiro.

**Medição que mudou o enquadramento (2):** em markup, `[`/`]` são **texto literal** no vanilla 0.15 (`typst-syntax/src/parser.rs:91-98`, `convert_and_eat(Text)` — bracket não abre content block em markup). Confirmado: `Hello [world]!` → `Hello [world]!` **nos dois motores** (cristalino já tinha paridade). Isto não é modificação local da quarentena — é upstream 0.15.0.

Decomposição medida do vanilla (`#show <sp>: it => [LBL=#it]` declarado antes):

| Entrada | vanilla |
|---|---|
| `= Alpha <sp>` | `LBL=` + heading (`LBL=\nAlpha`) — regra dispara sobre o elemento rotulado |
| `ABC <sp>` | `LBL=ABC` — label casa o elemento de texto inteiro |
| `[orig] <sp>` | `[origLBL=` + `]` — o label casa só o `Text("]")` final (o vanilla separa cada bracket noutro `Text`) |
| `[orig] <sp>` + `#context query(<sp>)` | `found=1` |

## 2. Estado cristalino (medido antes de implementar)

- `#show <sp>: …` → `error: selector inválido para show rule: label`, exit 1 (a divergência do passo). ✓ confirmada.
- `[orig] <sp>` + `query(<sp>)` → **`found=0`** (vanilla: `found=1`) — label em nó de texto não é indexado pelo introspector. **Divergência separada pré-existente**, fora do subsistema de show rules.
- Associação retroactiva de `<label>` (Passo 56, `eval/mod.rs:565-580`) cria `Content::label_auto(name, last)` **sem** passar por `intercept_content` — o wrapper nunca viajava pela maquinaria de show rules.
- `entities::show::Selector` não tinha variante `Label`; `entities::selector::Selector::Label` (query) já existia.

## 3. Implementação

Protocolo de Nucleação: L0s primeiro (`eval.md` bullet P791; `entities/show.md` variante + invariantes); 6 testes escritos primeiro, confirmados a falhar com o erro antigo.

- **`entities/show.rs`** — nova variante `Selector::Label(Label)`.
- **`rules.rs`**:
  - `eval_show_rule`: braço `Value::Label(l) => Selector::Label(l)`.
  - `selector_matches` / `is_node_rule`: `Selector::Label(_) => false` — regras de label **não** viajam pela travessia principal (o wrapper é criado pós-intercepção do corpo; viajar causaria dupla aplicação e `it` errado).
  - Nova `intercept_labelled`: aplicação dedicada só de regras de label no wrapper. `it` = **corpo** (paridade `target.label()` — label é metadado); saída substitui o wrapper (label consumido); aplicação única, última-declarada primeiro (P358); `Content` substitui; `Str` erro (paridade `NodeKind`); show-set (`Style`) dobra e embrulha em `Content::Styled` sem consumir passe (P352).
- **`eval/mod.rs`** (Passo 56, braço `SyntaxKind::Label`): o wrapper passa por `rules::intercept_labelled`.

## 4. Validação (binário release 22:25)

```text
$ cristalino /tmp/p791-test.typ   (= Alpha <sp> antes da regra; probe do passo)
exit 0 → "Alpha"                              (vanilla: "Alpha" — PAR; escopo léxico)

$ cristalino (regra primeiro) = Alpha <sp>
exit 0 → "LBL=\nAlpha"                        (vanilla: idem — PAR)

$ cristalino temp/temp_p786/b_selector.typ
exit 0 → "HEAD=Alpha [origLBL=] found=1"      (vanilla: "HEAD=Alpha [origLBL=] found=1" — BYTE-IDÊNTICO)
```

Surpresa medida: o `[origLBL=]` também é par — o cristalino reproduz o mesmo observável do vanilla (a L0 chegou a registar divergência de micro-segmentação prevista antes da medição; **corrigida após medir** — ADR-0108: o observável manda; o mecanismo interno exacto fica marcado como inferência).

**Não-regressão:** `b_selector_show.typ` → `HEAD=Alpha Body.` ✓; `b_selector_query.typ` → `Alpha found=1` ✓; `cargo test --workspace` — **verde**: 4295 passed / 0 failed no `typst-core` (+6 `p791_*`; antes 4289), 655 + 33 + 29 + 2 nas restantes, 0 falhas. `crystalline-lint .` — **exit 0, zero violações** (2 warnings V7 órfãos pré-existentes, não relacionados).

## 5. Critério de fecho do passo — checklist

- [x] Mecanismo do vanilla para `Selector::Label` em show rule confirmado (§1 — `Selector::Label`, match por `target.label()`).
- [x] Confirmado se compartilha código com P790 ou é caminho separado — **partilha o pipeline, falhava na validação do selector; aplicação é caminho dedicado novo** (`intercept_labelled`, §3).
- [x] `#show <lbl>: ...` funciona, comportamento idêntico ao vanilla (§4 — incl. evidência completa byte-idêntica).
- [x] Sem regressão em `#show <tipo>: ...` nem em `query(<lbl>)` (§4).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p791.md`.

## 6. Divergências registadas (candidatas a passos futuros)

1. **Label em nó de texto não é indexado pelo introspector** — `query(<sp>)` sobre `[orig] <sp>`: cristalino `found=0` vs vanilla `found=1` (medido). Subsistema de introspecção, não de show rules.
2. **`#label("nome")` (função, `stdlib/label.rs`) não passa por `intercept_labelled`** — show-by-label cobre só a sintaxe `<lbl>` em markup.
3. Match de regras de label sobre wrappers `Content::Label` criados por outros caminhos (introspect/auto) — não viajam a travessia principal por decisão documentada (§3); se um caso real exigir, reavaliar.

## 7. Próximo passo (candidatos)

Conforme o passo: context/layout eval (`#layout`, `text.lang`, `here().position()`), numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5. Novos: os itens de §6 acima.
