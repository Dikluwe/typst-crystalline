# Passo 1011 — Continuação do fatiamento `eval::rules`: `font_dict` e `selector_matching`

**Tipo**: Continuação do método (P1002), segundo lote de nós de `rules.rs`, já
identificados e justificados na Fase A do Passo 1009 — não repetir a auditoria, só
materializar.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1009 (inclui `show_rule_termination`).

---

## Nós a materializar (per tabela da Fase A do Passo 1009)

### Nó `font_dict`

- `parse_font_dict_named_fields` (l. 2238, na numeração pré-P1009 — reconfirmar linha
  actual antes de mover, `rules.rs` já mudou de tamanho)
- `parse_font_dict_legacy`
- `variants_from_value`

### Nó `selector_matching`

- `selector_matches`
- `query_selector_to_show_selector`
- `is_node_rule`
- `splice_text_rule_matches`

---

## Fase A — Confirmar antes de mover (o ficheiro já mudou desde a Fase A original)

1. `rules.rs` perdeu o loop α para `show_rule_termination.rs` no Passo 1009 — reconfirmar
   números de linha actuais de todos os símbolos acima antes de cortar.
2. Aplicar o Critério 1 (isolamento de teste) aos dois nós propostos — a Fase A do P1009
   não o fez em detalhe para estes dois (só para o mecanismo de paragem). Confirmar:
   - `font_dict` — as 3 funções são `Value`/`Dict` → `FontVariant`/erro, sem tocar
     `Engine`/`EvalContext`? Se sim, é nó declarativo, diferente da natureza stateful do
     resto de `rules.rs` — registar isto como achado (mistura de naturezas dentro do
     mesmo ficheiro original, mesma classe de observação já feita para `stdlib::foundations`
     no Passo 1004).
   - `selector_matching` — toca `EvalContext`/`Engine`, ou é `Selector`/`Content` →
     `bool` puro? Confirmar por leitura, não presumir a partir do nome.
3. Critério 3 (co-mudança) — a Fase A do P1009 já mostrou: font-dict muda em commits
   distintos do resto (`P836`), e não há sobreposição registada com selector_matching nos
   commits listados. Confirmar que isto se mantém como dois nós separados (não juntar os
   dois só porque ambos "sobraram") — critério 3 decide, não conveniência.

## Fase B — Materializar

1. L0 de cada nó (`compiler/eval/font_dict.md`, `compiler/eval/selector_matching.md`),
   sem referência a passo, com o campo **Técnica** preenchido só se houver algo nomeável
   (parsing de dict com fallback legacy pode valer a nota; matching de selector contra
   `Content` pode corresponder a "pattern matching estrutural" — confirmar se vale a pena
   nomear ou se é simples de mais para o campo).
2. Ficheiros `.rs` próprios (`font_dict.rs`, `selector_matching.rs`), V15 obrigatório.
3. Actualizar `rules.rs`/`eval.md` (hub) para reflectir que estas responsabilidades saíram.
4. Código cortado e colado, não reescrito — mesma disciplina de sempre.

## Fase C — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão. Confirmar contagem de testes antes/depois.

## Fase D — Estado de `rules.rs` depois deste passo

Registar o que sobra no hub (`eval_set_rule`, `eval_show_rule`, `intercept_*`,
`realize_*`, `capture_set_styles`, helpers de erro) e se algum destes ainda justifica
fatiamento próprio ou se o que sobra já é do tamanho certo para ficar como está — não
fatiar por fatiar, só porque ainda há linhas.

---

## Resultado esperado

`rules.rs` reduzido aos dispatchers de set/show-rule e intercept/realize; `font_dict` e
`selector_matching` como nós próprios, cada um com a natureza (declarativo/stateful)
confirmada, não presumida. `eval::rules` fica no estado que o P1008 previu como candidato
"prosseguir" — agora efectivamente tratado, antes de seguir para `closures` (próximo da
ordem do P1008).
