# Passo 1009 — Fase A: auditoria de `eval::rules` e proposta de nós

**Data**: 2026-08-12  
**Commit de base**: `5bb958bb3` (WIP: outputs e relatórios dos passos 1000-1009)  
**Ficheiro alvo**: `01_core/src/compiler/eval/rules.rs` (2720 linhas)  
**L0 actual do ficheiro**: `00_nucleo/prompts/compiler/eval.md`, hash `74294185` (hub grande de eval)

---

## 1. Pré-condição

`git status` limpo após commit local `5bb958bb3`.

---

## 2. Inventário completo de `rules.rs`

### `pub(crate)`

- `type_mismatch(expected, found, span) -> SourceDiagnostic` (l. 41)
- `expected_length_error(found, span) -> SourceDiagnostic` (l. 53)
- `edge_cast_error(is_top, found, span) -> SourceDiagnostic` (l. 80)
- `VANILLA_TEXT_SET_PROPS: &[&str]` (l. 121)
- `realize_paragraphs(content) -> Content` (l. 441)
- `apply_show_rules(content, rules, ctx, engine) -> SourceResult<Content>` (l. 588)
- `intercept_content(content, ctx, engine) -> SourceResult<Content>` (l. 962)
- `intercept_paragraphs(content, ctx, engine) -> SourceResult<Content>` (l. 994)
- `intercept_labelled(content, ctx, engine) -> SourceResult<Content>` (l. 1032)

### `pub(super)`

- `eval_set_rule(set, scopes, ctx, engine) -> SourceResult<Value>` (l. 1128)
- `eval_show_rule(show, scopes, ctx, engine) -> SourceResult<Value>` (l. 1990)

### Privadas de topo

- `query_selector_to_show_selector` (l. 162)
- `unsupported_property_warn` (l. 233)
- `unsupported_target_warn` (l. 248)
- `is_styled_origin` (l. 260)
- `selector_matches` (l. 293)
- `values_eq_semantic` (l. 377)
- `is_node_rule` (l. 389)
- `splice_text_rule_matches` (l. 411)
- `realize_flow` (l. 447)
- `realize_node` (l. 482)
- `capture_set_styles` (l. 1976)
- `parse_font_dict_named_fields` (l. 2238)
- `parse_font_dict_legacy` (l. 2374)
- `variants_from_value` (l. 2442)

---

## 3. Critérios do método P1002

### Critério 1 — Isolamento de teste

`apply_show_rules` pode ser testada isoladamente se fornecermos:

- `Content` de entrada;
- slice de `ShowRule` (selector + transform);
- `EvalContext` básico (só precisa de `apply_show_rules: true` e `full_error`);
- `Engine` mínimo (route, styles, active_guards vazio).

Já existem testes de guarda (18 de P340 + 3 de P1007) que cobrem o loop α. A extração do mecanismo de paragem num ficheiro próprio torna estes testes independentes do resto do hub (font-dict, eval_set_rule, etc.).

### Critério 2 — Pureza vs estado

`rules.rs` **não** é declarativo como `operators.rs`:

- Recebe `&mut EvalContext` e `&mut Engine<'_>`;
- Toca `engine.route`, `engine.styles`, `engine.active_guards`, `engine.show_rules`;
- `apply_show_rules` chama `closures::apply_func`, que precisa de scopes/engine mutáveis.

Logo este critério **discrimina negativamente** para todo o ficheiro: não pode ser um nó declarativo. Mas isso não impede o fatiamento hub/nó — apenas diz que os nós serão *stateful*, do lado do motor de eval, e não folhas puras.

### Critério 3 — Co-mudança histórica

Análise de `git log --follow --stat` sobre `rules.rs`, excluindo resselos de hash (±1 linha):

| Commit | Regiões tocadas |
|--------|-----------------|
| `3c8839e72` P861-P868 | realize_paragraphs, apply_show_rules, intercept_content/paragraphs, eval_set_rule, eval_show_rule |
| `f0db5cd20` P792/P792a | selector_matches, splice_text_rule_matches, apply_show_rules, intercept_labelled, eval_set_rule, eval_show_rule |
| `ee02d0b2b` P837 | edge_cast_error, expected_length_error, eval_set_rule (text props) |
| `2ae3ff53f` P836 | variants_from_value, parse_font_dict_*, eval_show_rule |
| `af4887e7e` P813-P827 | eval_show_rule, parse_font_dict_*, intercept_*, realize_* |
| `070c1cec7` P793/P794 | eval_show_rule, eval_set_rule |

Padrão: `apply_show_rules` e o loop de show-rules mudam separadamente das funções de font-dict (`parse_font_dict_*`, `variants_from_value`) e separadamente do dispatcher `eval_set_rule`/`eval_show_rule` em alguns commits, mas há sobreposição em commits grandes. O mecanismo de **paragem do loop α** é uma unidade coesa: as suas alterações concentram-se em `apply_show_rules` (P348, P350c, P863) e não nas funções de font-dict ou intercept.

### Critério 4 — Correspondência vanilla

Confirmado no Passo 1003: o vanilla separa a lógica em:

- `typst_eval::rules` — pequeno, definição de `ShowRule`/`Selector` e despacho;
- `typst_realize` — aplicação real das show rules, loop α e terminação.

O cristalino actualmente agrega tudo em `eval::rules.rs`. O fatiamento aproxima a fronteira vanilla: o mecanismo de paragem/realização passa para um nó próprio, enquanto `rules.rs` mantém o dispatcher e os helpers de eval.

---

## 4. Proposta de nós (Fase A)

Baseada nos critérios, propomos os seguintes nós. **Este passo materializa apenas o primeiro; os restantes ficam como scope-out futuro**, a documentar no L0 do hub.

| Nó | Ficheiro L1 | L0 | Conteúdo |
|----|-------------|-----|----------|
| `show_rule_termination` | `compiler/eval/show_rule_termination.rs` (novo) | `compiler/eval/show_rule_termination.md` (novo) | Loop α de aplicação de show rules, detecção de ponto-fixo/ciclo via `morph_canon`, limite MAX_SHOW_RULE_DEPTH. |
| `rules_hub` | `compiler/eval/rules.rs` (existente) | `compiler/eval.md` (atualizado) | Dispatcher `eval_set_rule`/`eval_show_rule`, intercept_*, realize_*, helpers de cast/erro, font-dict. |
| `font_dict` (futuro) | `compiler/eval/font_dict.rs` | `compiler/eval/font_dict.md` | `parse_font_dict_named_fields`, `parse_font_dict_legacy`, `variants_from_value`. |
| `selector_matching` (futuro) | `compiler/eval/selector_matching.rs` | `compiler/eval/selector_matching.md` | `selector_matches`, `query_selector_to_show_selector`, `is_node_rule`, `splice_text_rule_matches`. |

**Decisão do mecanismo de paragem**: tornar a detecção de ponto-fixo e ciclo o comportamento base (não só sob `full_error`). A flag `full_error` mantém-se apenas para o hint classificatório extra (cíclico vs não-convergente). Desfecho 3 (`maximum show rule depth exceeded`) preserva mensagem byte-idêntica ao vanilla (ADR-0033). Desfecho 2 (ciclo) introduz mensagem nova `ShowRuleCycle` — decidir forma no L0.

---

## 5. Bloqueios resolvidos / pendentes

- ✅ `git status` limpo (commit `5bb958bb3`).
- ✅ Inventário completo.
- ✅ Critérios aplicados com evidência.
- ⏳ L0 para `show_rule_termination` não existe — necessário antes de código (Protocolo de Nucleação + ADR-0127).
- ⏳ Decisão sobre a mensagem de ciclo (Desfecho 2).
