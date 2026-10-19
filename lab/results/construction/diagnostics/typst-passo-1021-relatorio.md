# Passo 1021 — Relatório de Auditoria Critério D

**Data**: 2026-08-13

## Sumário executivo

- Prompts auditados com achados: **80**
- Total de achados catalogados: **489**
- Achados graves: **134**
- Achados leves: **355**

### Breakdown por bloco do auditar-spec.md

| Bloco | Nome | Total |
|-------|------|-------|
| 1 | Completude estrutural | 95 |
| 2 | Ambiguidade de conteúdo | 140 |
| 3 | Fundamentação | 83 |
| 4 | Zero referência a passo | 171 |

### Distribuição por severidade

| Severidade | Total |
|------------|-------|
| grave | 134 |
| leve | 355 |

## Sumário por prompt

| Prompt | Total | Graves | Leves |
|--------|-------|--------|-------|
| `compiler/eval/call_dispatch.md` | 8 | 6 | 2 |
| `compiler/eval/cast.md` | 7 | 4 | 3 |
| `compiler/eval/show_rule_termination.md` | 10 | 8 | 2 |
| `compiler/eval/table.md` | 10 | 6 | 4 |
| `compiler/introspect/convergence.md` | 6 | 0 | 6 |
| `compiler/introspect/extract_payload.md` | 18 | 0 | 18 |
| `compiler/introspect/fixpoint.md` | 8 | 0 | 8 |
| `compiler/introspect/from_tags.md` | 28 | 0 | 28 |
| `compiler/introspect/locatable.md` | 14 | 0 | 14 |
| `compiler/layout-image.md` | 4 | 1 | 3 |
| `compiler/layout.md` | 14 | 4 | 10 |
| `compiler/layout/bib_csl.md` | 6 | 2 | 4 |
| `compiler/layout/bibliography.md` | 8 | 5 | 3 |
| `compiler/layout/enum_item.md` | 5 | 3 | 2 |
| `compiler/layout/equation.md` | 4 | 0 | 4 |
| `compiler/layout/footnote.md` | 5 | 2 | 3 |
| `compiler/layout/heading.md` | 5 | 2 | 3 |
| `compiler/layout/link.md` | 10 | 3 | 7 |
| `compiler/layout/list_item.md` | 3 | 2 | 1 |
| `compiler/layout/raw_highlight.md` | 11 | 1 | 10 |
| `compiler/layout/shape_block_behaviour.md` | 4 | 1 | 3 |
| `compiler/layout/table.md` | 9 | 0 | 9 |
| `compiler/layout_counters.md` | 5 | 1 | 4 |
| `compiler/layout_figure.md` | 6 | 2 | 4 |
| `compiler/layout_outline.md` | 3 | 1 | 2 |
| `compiler/layout_references.md` | 7 | 3 | 4 |
| `compiler/math/layout/_comum.md` | 4 | 0 | 4 |
| `compiler/math/layout/accent.md` | 3 | 1 | 2 |
| `compiler/math/layout/assembly.md` | 5 | 0 | 5 |
| `compiler/math/layout/attach.md` | 6 | 3 | 3 |
| `compiler/math/layout/cancel.md` | 9 | 0 | 9 |
| `compiler/math/layout/cases.md` | 8 | 3 | 5 |
| `compiler/math/layout/delimited.md` | 5 | 1 | 4 |
| `compiler/math/layout/frac.md` | 7 | 2 | 5 |
| `compiler/math/layout/matrix.md` | 5 | 2 | 3 |
| `compiler/math/layout/op.md` | 7 | 3 | 4 |
| `compiler/math/layout/root.md` | 2 | 2 | 0 |
| `compiler/math/layout/spacing.md` | 6 | 2 | 4 |
| `compiler/math/layout/stretchy.md` | 3 | 0 | 3 |
| `compiler/math/layout/underover.md` | 6 | 1 | 5 |
| `compiler/math/mod.md` | 2 | 0 | 2 |
| `compiler/math/symbols.md` | 11 | 6 | 5 |
| `entities/elements/_comum.md` | 2 | 0 | 2 |
| `entities/elements/align.md` | 4 | 1 | 3 |
| `entities/elements/bibliography.md` | 5 | 2 | 3 |
| `entities/elements/block.md` | 4 | 1 | 3 |
| `entities/elements/boxed.md` | 5 | 1 | 4 |
| `entities/elements/cite.md` | 5 | 2 | 3 |
| `entities/elements/colbreak.md` | 2 | 0 | 2 |
| `entities/elements/columns.md` | 5 | 2 | 3 |
| `entities/elements/counter_display.md` | 3 | 1 | 2 |
| `entities/elements/counter_display_callback.md` | 3 | 1 | 2 |
| `entities/elements/counter_update.md` | 3 | 1 | 2 |
| `entities/elements/curve.md` | 1 | 0 | 1 |
| `entities/elements/divider.md` | 2 | 0 | 2 |
| `entities/elements/enum_item.md` | 4 | 1 | 3 |
| `entities/elements/equation.md` | 4 | 2 | 2 |
| `entities/elements/figure.md` | 4 | 2 | 2 |
| `entities/elements/footnote.md` | 4 | 2 | 2 |
| `entities/elements/grid_cell.md` | 3 | 1 | 2 |
| `entities/elements/grid_footer.md` | 2 | 0 | 2 |
| `entities/elements/math_frac.md` | 4 | 2 | 2 |
| `entities/elements/math_limits_override.md` | 9 | 3 | 6 |
| `entities/elements/math_matrix.md` | 3 | 0 | 3 |
| `entities/elements/math_op.md` | 4 | 3 | 1 |
| `entities/elements/math_root.md` | 3 | 0 | 3 |
| `entities/elements/math_styled.md` | 4 | 0 | 4 |
| `entities/elements/math_underover.md` | 9 | 0 | 9 |
| `entities/elements/metadata.md` | 13 | 9 | 4 |
| `entities/elements/outline.md` | 15 | 1 | 14 |
| `entities/elements/overline.md` | 4 | 0 | 4 |
| `entities/elements/pad.md` | 6 | 3 | 3 |
| `entities/elements/pagebreak.md` | 6 | 2 | 4 |
| `entities/elements/place.md` | 9 | 1 | 8 |
| `entities/elements/quote.md` | 3 | 0 | 3 |
| `entities/elements/raw.md` | 9 | 2 | 7 |
| `entities/elements/ref.md` | 6 | 3 | 3 |
| `entities/elements/repeat.md` | 5 | 2 | 3 |
| `entities/elements/shape.md` | 4 | 0 | 4 |
| `entities/elements/smartquote.md` | 1 | 0 | 1 |

## Catálogo de achados

Abaixo listam-se todos os achados, agrupados por prompt e ordenados por bloco.

### `compiler/eval/call_dispatch.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais** (pureza, imports, invariantes de camada).
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado** (estado/ output final após aplicação).
- **Bloco 2** (grave) — §2, l. 57: "Manter exatamente o comportamento actual" — não define o que é "comportamento actual" nem como medi-lo.
- **Bloco 2** (grave) — Contexto, l. 14: Lista de métodos especiais termina em "`etc.`"; o conjunto de intercepções não está fechado.
- **Bloco 2** (grave) — §2, l. 59: "aplicando intercepções na ordem correcta" — a ordem não é especificada.
- **Bloco 2** (grave) — §2, l. 64: "defaults seguros quando a posição ainda não está disponível" — valor do default não definido.
- **Bloco 3** (grave) — Contexto, l. 14 / §2, l. 59: Afirmações sobre métodos especiais de Typst (`where`, `and`/`or`, `within`, `to-absolute`, `measure`, `layout`, métodos de `state`/`counter`/`color`/`version`/`args`/`content`) sem citação `docs.typst.app` nem medição directa.
- **Bloco 4** (grave) — Contexto, l. 16: Ocorrência de "Passo 1012" fora de `Criado em` / `Histórico de Revisões`.
### `compiler/eval/cast.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — Propósito, l. 16–17: "Consumers em Trilha 7" — termo de roadmap do projecto não definido no prompt.
- **Bloco 2** (grave) — Propósito/API, l. 13 / l. 29: O propósito fala de "Casts implícitos de `Value`" (plural), mas só é especificado `cast_length`; não fica claro se existem outros casts fora do escopo.
- **Bloco 3** (grave) — Propósito, l. 15: Afirmação "`Value::Relative` não pode ser resolvido para `Length` porque falta o contexto de layout" sem citação docs.typst.app nem medição directa.
- **Bloco 4** (grave) — Cabeçalho, l. 6: `Criado em` contém "P469", violando a regra de que este campo só pode ter data + descrição.
- **Bloco 4** (grave) — Histórico de Revisões, l. 50: `Histórico de Revisões` contém "P469", violando a regra de que este campo só pode ter data + descrição.
### `compiler/eval/show_rule_termination.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (grave) — §2, l. 62: `history: Vec<Content> = if full_error { Vec::new() } else { Vec::new() }` — ramos `if`/`else` idênticos; a intenção de alocação condicional não está expressa.
- **Bloco 2** (grave) — §4, l. 140–141: "hint classificatório (cíclico vs não-convergente)" sem especificar o texto exacto do hint.
- **Bloco 2** (grave) — §5 / Critérios, l. 165, 168–169: Critérios referem "18 testes P340" e "Passo 1007 Teste 1/2" sem definição local; o leitor não pode reproduzir a guarda.
- **Bloco 3** (grave) — §3, l. 124: "confirmado por teste empírico" sem registo de proveniência (comando, resultado, commit).
- **Bloco 4** (grave) — Contexto, l. 16: Ocorrência de "Passo 1009" fora de `Criado em` / `Histórico de Revisões`.
- **Bloco 4** (grave) — §3, l. 123: Ocorrência de "Passo 1007" fora de `Criado em` / `Histórico de Revisões`.
- **Bloco 4** (grave) — Critérios, l. 165: Ocorrência de "P340" fora de `Criado em` / `Histórico de Revisões`.
- **Bloco 4** (grave) — Critérios, l. 168–169: Ocorrências de "Passo 1007" fora de `Criado em` / `Histórico de Revisões`.
### `compiler/eval/table.md`

- **Bloco 1** (grave) — Cabeçalho, l. 2: `Hash do Código: PENDENTE_HUMAN_CALC` — prompt não selado; impede validação de linhagem.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Contexto/Propósito** (inicia directamente com secções numeradas de comportamento).
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — Cabeçalho, l. 6 / §4, l. 68: Termo "chain léxica" não definido no prompt.
- **Bloco 2** (grave) — §1, l. 17: "Outros tipos → ignorar (herdar)" — não especifica o mecanismo de herança nem o comportamento exacto quando o tipo não casa.
- **Bloco 3** (grave) — §3, l. 27–35 / l. 60–62: Afirmações sobre `table.header`, `table.footer`, `table.cell` e os seus argumentos (`repeat`, `colspan`, `rowspan`, etc.) sem citação `docs.typst.app` nem medição directa.
- **Bloco 4** (grave) — Cabeçalho, l. 6 / §2, l. 23 / §5, l. 83: Ocorrências de "P459" fora de `Criado em` / `Histórico de Revisões`.
- **Bloco 4** (grave) — Cabeçalho, l. 7 / §3, l. 27: Ocorrências de "P493b" fora de `Criado em` / `Histórico de Revisões`.
- **Bloco 4** (grave) — §4, l. 68 / l. 73 / l. 83: Ocorrências de "P661" fora de `Criado em` / `Histórico de Revisões`.
### `compiler/math/layout/accent.md`

- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado** que sintetize o estado final pretendido do `MathAccent`.
- **Bloco 2** (grave) — Secção P988-B (centragem horizontal do acento): Deixa em aberto o momento exacto da medição do `top_accent_attach` do acento: "a investigar na implementação: confirmar se é antes ou depois do stretch". O posicionamento horizontal depende desta escolha.
- **Bloco 4** (leve) — Linhas 5–7, 15, 17, 28, 35, 42, 46, 48, 62, 65, 73, 85, 87, 89, 91, 94, 96–98, 102, 106–107, 120, 132, 135, 145, 152, 177, 192, 210–211, 213, 236–237, 240, 247: Múltiplas referências a passos (`P909`, `P314`, `P296`, `P906`, `P899`, `P905`, `P901`, `P915`, `P918`, `P922`, `P984`, `P988-B`, `P989`, `P987`, `P971`) no corpo técnico, fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/assembly.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com o papel de **Restrições** (ou equivalente). As limitações aparecem esparsas no corpo, mas não há secção dedicada.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com o papel de **Resultado Esperado**. O documento descreve comportamentos e critérios, mas não sintetiza o estado final esperado do módulo.
- **Bloco 3** (leve) — Linhas 25–27: Afirmação sobre comportamento do vanilla ("mesma convenção OpenType usada pelo vanilla") sem citação a `typst.app/docs/` ou medição directa contra o binário vanilla.
- **Bloco 3** (leve) — Linha 51: Afirmação sobre o algoritmo do vanilla (`MAX_REPEATS = 1024`) sem citação a `typst.app/docs/` ou medição directa (`file:line`/comando+resultado).
- **Bloco 4** (leve) — Linhas 5, 11, 15, 17, 48, 61, 67, 97, 99, 111, 139, 157, 203: Ocorrências de referências a passos (`P314`, `P255`, `P906`, `P912`, `P913`, `P914`, `P917`, `P918`, `P945`, `P952b`, `P957`, `P988`, `typst-passo-945`, `typst-passo-952`, `typst-passo-957`) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/attach.md`

- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado**. Os critérios de verificação estão dispersos pelas subsecções, mas não há resumo do estado final do módulo `attach.rs`.
- **Bloco 3** (grave) — Secção "Empilhamento de limites (`is_limits`)", linhas 31–34: Afirmação sobre semântica do Typst (`is_limits` decide empilhamento vertical vs. scripts laterais) sem citação `docs.typst.app/` nem medição directa com proveniência.
- **Bloco 3** (grave) — Secção "P992 — override explícito via `Content::MathLimitsOverride`", linhas 64–70: A reestruturação de `is_limits` é fundamentada numa citação textual da documentação vanilla, mas sem URL `docs.typst.app/` nem `file:line` de origem. A conclusão estrutural depende desta citação não rastreável.
- **Bloco 3** (leve) — Secção "Scripts laterais sub+sup partilham a origem x — P799", linhas 112–113: A paridade vanilla é invocada com `"scripts.rs: tr_x = br_x = pre_width + base_width + kern"`, mas o ficheiro é citado sem `file:line`.
- **Bloco 3** (grave) — Secção "P914 — Deslocamentos Adaptativos de Sub/Sobrescrito", linhas 124–130: Descreve alterações de comportamento geométrico concretas (`shift_up`/`shift_down` dinâmicos, ajuste de gap simultâneo, kerning em duas alturas) sem citação `docs.typst.app/` ou medição directa contra o vanilla.
- **Bloco 4** (leve) — Várias (linhas 5, 12, 13, 20, 29, 43, 56, 62, 97, 107, 118, 124, 132, 142, 155, 167, 180, 200, 217, 244, 275, 290): Dezenas de referências a passos (`P314`, `P772w`, `P799`, `P891`, `P914`, `P915`, `P944`, `P945`, `P952`, `P959`, `P963`, `P971`, etc.) no corpo técnico, fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/cancel.md`

- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado**. O prompt indica o ficheiro alvo e critérios, mas não lista explicitamente os artefactos gerados.
- **Bloco 2** (leve) — Linha 13: A expressão "Heurística minimal per ADR-0054 graded" usa o termo "graded" sem definição ou contexto suficiente.
- **Bloco 2** (leve) — Linha 17: "scope-out para passo futuro dedicado a MathCancel" remete para um passo não identificado.
- **Bloco 2** (leve) — Linha 25: A secção "P986 — a linha usa a convenção baseline-relativa (quarto caso da família)" classifica o problema como "quarto caso da família" sem definir a família nem os casos anteriores.
- **Bloco 3** (leve) — Linhas 14–17: Afirmação "Diagonal default ... 'rising', ângulo padrão do vanilla" é generalização sobre Typst sem citação da documentação oficial nem medição directa.
- **Bloco 3** (leve) — Linhas 16–17: Scope-out de `inverted`/`cross`/`angle`/`stroke` pressupõe que estas opções existem no Typst, mas não cita documentação oficial que as liste.
- **Bloco 3** (leve) — Linhas 27–31: A medição é apresentada por referência indirecta ("achado §7.3 da auditoria 2026-08-06") sem reproduzir o comando ou `file:line` que a produziu.
- **Bloco 4** (leve) — Linhas 5, 7, 8, 25, 36: Referências a passos numerados (`P909`, `P314`, `P296`, `P901`/`P906`/`P919`/`P972`, `P986`) fora de `Criado em`/`Histórico de Revisões`.
- **Bloco 4** (leve) — Linha 17: Referência não numerada a "passo futuro", violando o espírito da regra de zero referências a passos.
### `compiler/math/layout/cases.md`

- **Bloco 1** (grave) — Documento global: Falta secção com o papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado**.
- **Bloco 1** (leve) — Documento global: Secção de **Critérios de Verificação** incompleta. Apenas o sub-item dentro de P919 cobre verificação; não há critérios sistemáticos para P923, P912, P918 e P945.
- **Bloco 2** (grave) — Secções P923 (linha 20) vs. P945 (linhas 69–73): Contradição aparente no cálculo do tamanho das células: P923 prescreve `size: style.size * script_percent_scale_down`, enquanto P945 substitui por `denominator_style` com descida por nível MathSize. Ambas as secções aparecem como vigentes sem que P923 seja marcado como obsoleto.
- **Bloco 2** (leve) — Introdução, linhas 10–11: Limites de escopo implícitos: não indica casos de borda deliberadamente fora de escopo (ex.: `cases()` vazio, com um único ramo, ou encadeado).
- **Bloco 3** (grave) — P912, linhas 26–29: Afirmação "a margem de 10% do vanilla" sem referência a `docs.typst.app` nem medição directa com proveniência.
- **Bloco 3** (leve) — P945, linhas 65–76: Afirma comportamento do vanilla para descida de nível MathSize apenas por referência a `matrix.md` §P945 e `_comum.md` §P945; não apresenta citação directa ao vanilla/docs no próprio prompt.
- **Bloco 4** (leve) — Várias (linhas 5, 13, 19, 24, 26, 33, 35, 38, 40, 44, 46, 50, 60, 65, 67, 69, 70): Múltiplas referências a passos (`P314`, `P923`, `P923b`, `P912`, `P918`, `P919`, `P917`, `P945`, `typst-passo-919-relatorio.md`) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/_comum.md`

- **Bloco 1** (leve) — Estrutura geral (secções presentes: Propósito l. 13, Restrição arquitectural l. 75, Interface pública l. 79, Critérios de verificação gerais l. 317): Não existe secção com o papel de **Resultado Esperado**. Os 5 papéis estruturais do template não estão todos cobertos por secções distintas.
- **Bloco 3** (leve) — "MathPrimes" (linhas 216–222): Afirmação sobre comportamento do Typst vanilla no render de primes é justificada apenas com "Paridade observable vanilla per ADR-0033". ADR interna não substitui citação da documentação oficial nem medição directa com proveniência.
- **Bloco 3** (leve) — "Handler `Content::MathStyled` — Passo 311b.4" (linhas 224–256): Várias afirmações de paridade vanilla (`bb(cal(x))`, `bold(bb(x))`, `upright(italic(x))`, `script(sscript(x))`) são apresentadas sem citação `docs.typst.app/` ou medição directa com comando/resultado.
- **Bloco 4** (leve) — Documento completo; exemplos em Estado actual (l. 16), Baseline x-height (l. 163), Handler `Content::MathStyled` (l. 224), P906/P909 (l. 327), P918 (l. 336), P945 (l. 387), P952 (l. 449), P961 (l. 509), P966 (l. 526), P967b (l. 571), P973 (l. 603), P990-C (l. 639), P991 (l. 668), P992 (l. 722), P994 (l. 764): Inúmeras referências a passos (`P96.8`, `P255`, `P800`, `P813`, `P921`, `P311b.4`, `P772y`, `P893`, etc.) fora de qualquer secção de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/delimited.md`

- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Restrições Estruturais / Invariantes**.
- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — Secções P912 e P919: O prompt especifica o cálculo para delimitadores balanceados e remoção de `apply_axis_offset`, mas não explicita quais casos de borda são deliberadamente fora de escopo (delimitadores desbalanceados, vazios, aninhamento, variação de `size`).
- **Bloco 3** (grave) — "P912 — Cálculo de Altura de Delimitadores Balanceados", linhas 14–20: Afirmação de que a altura-alvo do delimitador balanceado é `2.0 * (ascent - axis).max(descent + axis)` e que espelha o Typst não é fundamentada com citação da documentação oficial nem medição directa contra o binário vanilla.
- **Bloco 4** (leve) — Linhas 5, 10–11, 14, 22, 24, 31: Múltiplas referências a passos (`P314`, `P912`, `P919`) e ficheiro de passo (`typst-passo-919-relatorio.md`) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/frac.md`

- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Documento global: Falta secção com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — Início do documento, linhas 10–11: A frase "Baseline x-height própria" é imprecisa (P919 trata de `axis_height`, não x-height) e a enumeração inicial de constantes (`fraction_rule_thickness + fraction_num_gap + fraction_denom_gap`) está incompleta face a `fraction_numerator_shift_up/down` (P920) e constantes Display (P990-A).
- **Bloco 2** (grave) — Secção P905, linhas 42–46, vs. P920/P990-A/P990-B: P905 afirma que `ascent`/`descent` do `MathBox` resultante "não mudaram" e que a fórmula "já estava correcta"; mais tarde P920/P990-A alteram a fórmula dos gaps/shifts que compõem esses mesmos `ascent`/`descent`. Não há reconciliação explícita sobre qual fórmula é a autoridade final.
- **Bloco 2** (grave) — Secção P920, linha 115: A especificação deixa o "exacto sinal/composição com `axis_pt`" como algo "a confirmar na Fase B" — parte do comportamento de layout está pendente de verificação no próprio L0.
- **Bloco 3** (leve) — Início do documento, linhas 10–11: Afirmação de que `MathFrac` consome `fraction_rule_thickness + fraction_num_gap + fraction_denom_gap` de `MathConstants` não apresenta citação a `docs.typst.app` nem medição directa contra o vanilla.
- **Bloco 4** (leve) — Várias secções, linhas 5–225: Dezenas de referências a passos (`P314`, `P800`, `P901`, `P905`, `P915`, `P919`, `P920`, `P923`, `P944`, `P945`, `P952`, `P952b`, `P972`, `P990`, `P990-A`, `P990-B`, "Passo 9.8/136-137") fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/matrix.md`

- **Bloco 1** (leve) — Documento global: Não contém secções que cubram os papéis de **Restrições Estruturais**, **Critérios de Verificação** e **Resultado Esperado**. O "Critério de regressão" em P919 é um critério específico, não uma secção de Critérios de Verificação.
- **Bloco 2** (leve) — Secção "Delimitadores Customizados e Nulos (`delim`)", linhas 112–117: Especifica que delimitadores nulos (`'\0'`) desactivam a renderização e omitem padding lateral de `0.1em`, mas não fundamenta nem define de onde vem o comportamento de delimitador nulo nem o valor exacto do padding.
- **Bloco 3** (grave) — Secção "P912 — Margem de 10% na Altura dos Delimitadores de Matriz", linhas 68–71: Afirma-se que a altura da grelha aplica "a margem de 10% do vanilla", sem citação do ficheiro/linha do vanilla nem medição directa.
- **Bloco 3** (grave) — Secção "Delimitadores Customizados e Nulos (`delim`)", linhas 112–117: Descreve o comportamento dos delimitadores sem citar a documentação oficial do Typst nem apresentar medição directa contra o binário vanilla.
- **Bloco 4** (leve) — Várias (linhas 5, 13, 35, 39, 44, 46, 49, 56, 68, 73, 75, 77, 80, 87, 91, 98, 102, 105, 119, 121, 122, 124, 133, 142, 143, 146, 151): Dezenas de referências a passos (`P314`, `P923`, `P923b`, `P825`, `P810`, `P912`, `P918`, `P919`, `P921`, `P917`, `P945`, `P944`, `P913`) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/op.md`

- **Bloco 1** (grave) — Documento todo: Falta secção equivalente a **Restrições Estruturais**. Não delimita dependências proibidas, implicações da camada L1, interfaces a implementar ou o que `layout_op` não deve fazer.
- **Bloco 1** (leve) — Documento todo: Falta secção equivalente a **Resultado Esperado**. Não descreve os arquivos/estruturas a gerar, a API pública exposta nem como verificar que as restrições estruturais foram respeitadas.
- **Bloco 2** (leve) — Linha 12: Termo "`math` child standard" não é definido no prompt nem é auto-evidente no domínio sem referência a conceito previamente estabelecido.
- **Bloco 2** (leve) — Linhas 15–28: Os limites do escopo de `layout_op` ficam implícitos: não fica claro o que é responsabilidade deste arquivo vs. `attach.rs` (`limits: false`, modo inline, caracteres especiais).
- **Bloco 3** (grave) — Linhas 20–24: Afirmação "os scripts empilham verticalmente acima/abaixo da base incondicionalmente quando `self.block`, independentemente do carácter" generaliza além do que a citação disponível no corpus-docs justifica, e não inclui citação `docs.typst.app` nem medição directa no próprio prompt.
- **Bloco 3** (grave) — Linhas 23–24: Afirmação sobre a regra geral de `is_limits` depender de `symbols::is_large_operator`/`symbols::is_limit_function` não é acompanhada de citação da documentação oficial nem de medição directa.
- **Bloco 4** (leve) — Linhas 5–7, 13, 21: Ocorrências de `P909`, `P314` e `P298` no corpo do prompt (incluindo campo "Origem"), fora de `Histórico de Revisões`.
### `compiler/math/layout/root.md`

- **Bloco 1** (grave) — Documento global (título + secções P901/P915/P919/P945/P970/P974): O prompt não cobre os 5 papéis estruturais de `template-prompts.md`: não há secção equivalente a **Instrução**, **Restrições**, **Critérios de Verificação** nem **Resultado Esperado**. O documento está organizado como changelog/histórico de passos, não como especificação arquitetural perene.
- **Bloco 4** (grave) — Passim: metadata linha 5, títulos de secção linhas 16, 43, 63, 80, 93, 172, e corpo do texto: Múltiplas referências a passos (`P314`, `P901`, `P915`, `P919`, `P945`, `P970`, `P974`, `P893`, `P800`, `P912`, `P944`, `P952`) fora de qualquer secção `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/spacing.md`

- **Bloco 1** (grave) — Documento global: Falta a secção de **Restrições Estruturais** (ou equivalente). Não há listagem explícita das implicações de ser L1, interfaces a implementar, dependências proibidas nem o que o componente não deve fazer.
- **Bloco 1** (leve) — Final do documento, secção "Integração em mod.rs": Falta a secção de **Resultado Esperado** (ou equivalente). Não há resumo dos ficheiros a criar, das funções/classes/interfaces a expor nem de como verificar que as restrições estruturais foram respeitadas.
- **Bloco 2** (leve) — "Contexto" (linha 17) e "Critérios de verificação" (linha 208): O termo **`mutool trace`** é usado como método de medição sem definição, referência ou comando exemplo, nem é auto-evidente fora do contexto do projecto.
- **Bloco 2** (leve) — Tabela "Classificação por nó" (linhas 61–62): A função **`default_math_class`** é invocada sem definição local ou link explícito para `rules/math/layout/_comum.md`.
- **Bloco 3** (grave) — Cabeçalho (linhas 7–9), tabela de classificação (linhas 63–70), "Tabela de espaçamento" (linha 79), P891 (linhas 91–99), P903 (linhas 116–133), P907 Parte A (linhas 154–163), P907 Parte B (linhas 166–174), "Promoção Vary" (linhas 180–185): Várias afirmações sobre "o que o Typst faz" baseiam-se exclusivamente em leitura do código-fonte do vanilla (ex.: `typst-library/src/math/ir/process.rs:277-319`, `MathItem::is_spaced`, `resolve_op`, `ScriptsItem::create`) e não citam a documentação oficial (`typst.app/docs/`) nem apresentam medição directa contra o binário vanilla com comando+resultado.
- **Bloco 4** (leve) — Dispersas por todo o documento — cabeçalho (linha 5), "Contexto" (linhas 15–17), "Interface pública" (linhas 34–44), P891 (linha 91), P903 (linhas 116–135), P907 (linhas 152–176), "Integração em mod.rs" (linhas 190–192), "Critérios de verificação" (linha 208): Dezenas de referências a passos numerados (`P772y`, `P891`, `P903`, `P907`, `P825`, `P885`, `P889`, `P992`, `P784`, `P897`, etc.) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/stretchy.md`

- **Bloco 3** (leve) — Secção `P912 — Subtração de DELIM_SHORT_FALL (0.1em)`, linhas 47–52: Afirma-se que delimitadores verticalmente esticáveis subtraem `DELIM_SHORT_FALL = 0.1em` da dimensão alvo antes de consultar variantes. Afirmação sobre comportamento do Typst/vanilla sem citação a `docs.typst.app/` nem medição directa contra o binário vanilla.
- **Bloco 3** (leve) — Secção `P914 — Centralização no Eixo Matemático (axis_height)`, linhas 71–75: Descreve a centralização do delimitador em torno de `axis_height` como comportamento a implementar, sem citação à documentação oficial nem medição directa.
- **Bloco 4** (leve) — Linhas 5, 12, 16, 18, 22, 37, 47, 54, 56–57, 62, 71, 77, 81–83, 96, 117–118, 122, 140, 151, 163–165, 170, 177, 184–188, 195, 202: Dezenas de referências a passos (`P314`, `P255`, `P906`, `P912`, `P914`, `P917`, `P918`, `P952b`, `P984`, `P985`, `P974`, `P911`, `P916`, `P945`, `P949`, "typst-passo-952") fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/layout/underover.md`

- **Bloco 1** (leve) — Documento global (linhas 1–226): O prompt não adopta a estrutura de 5 papéis do `template-prompts.md`; está organizado cronologicamente por passos. Os papéis de **Restrições** e **Resultado Esperado** não têm secção dedicada nem equivalente explícito.
- **Bloco 2** (leve) — Linha 101, secção P920/P922: Fragmento "— cap. **Nota de correcção**": "cap." fica incompleto/ambíguo (possível "capped", "capping" ou erro tipográfico), deixando a nota sem contexto sintáctico claro.
- **Bloco 2** (leve) — Linha 220, secção P985 (`**Critério**`): O critério de verificação usa o carácter `➞` (U+279E) no par `➞ ink (783du, 0)` do stub de tinta; não corresponde a nenhuma das peças under/over identificadas no prompt (`⏟`/`⏞`/`⎵`/`⎴`) e não é definido.
- **Bloco 3** (leve) — Linhas 14–16, introdução: Afirmação sobre o vanilla ("vanilla typst fragmenta em 12 elements (`underbrace`/`overbrace`/`underbracket`/etc., dispatch em `eval/math.rs`)") é baseada em ADR-0054/leitura de código-fonte, sem citação `docs.typst.app` nem medição directa.
- **Bloco 3** (grave) — Linhas 92–97 (P920/P922) e linhas 155–157 (P984): Afirmações mecanísticas sobre o vanilla — "resolvem no vanilla como `AccentItem` … **não** como `LineItem`" e "`StretchInfo::new(Rel::one(), Em::zero())` — short_fall zero" — citam apenas ficheiros fonte do vanilla (`resolve.rs:1277-1472`, `accent.rs:56-71`, `resolve.rs:1430`), não `docs.typst.app` nem medição directa; o corpus local também não é cruzado.
- **Bloco 4** (leve) — Linhas 5–7, 18, 29, 42, 45, 50, 52, 78, 80, 82, 84, 87, 94, 97, 105–106, 113, 116, 119, 124, 126, 129, 138–139, 151, 153, 160, 162, 185, 190–191, 194, 203: Múltiplas referências a passos (`P909`, `P314`, `P297`, `P906`, `P905`, `P901`, `P918`, `P920`, `P922`, `P944`, `P945`, `P952b`, `P961`, `P984`, `P985`) em títulos de secção, corpo e referências a outros prompts, fora de `Criado em`/`Histórico de Revisões`.
### `compiler/math/mod.md`

- **Bloco 1** (leve) — Secções do documento (global): Falta o papel **Resultado Esperado**. O documento cobre Contexto, Estrutura dos Submódulos, Regras de Arquitetura, Interface Pública e Critérios de Verificação, mas não há secção equivalente a "Resultado Esperado" do template.
- **Bloco 2** (leve) — Título (linha 1), "Ficheiro alvo" (linha 5), "Interface Pública" (linhas 37–38), "Critérios de Verificação" (linhas 59–60): Ambiguidade/contradição interna sobre o caminho/namespace do módulo. O título usa `rules/math/mod`, o ficheiro alvo é `compiler/math/mod.rs`, os comentários da interface referem `rules::math::layout`/`rules::math::symbols`, enquanto os critérios de verificação usam `crate::compiler::math::layout`/`crate::compiler::math::symbols`.
### `compiler/math/symbols.md`

- **Bloco 1** (grave) — Estrutura global: Falta secção com o papel de **Restrições Estruturais** (ou equivalente).
- **Bloco 1** (grave) — Estrutura global: Falta secção com o papel de **Resultado Esperado** (ou equivalente).
- **Bloco 2** (grave) — "Interface Pública" (linhas 33–34) vs. adenda `§P958` (linhas 216–221): Inconsistência no inventário de letras gregas. A Interface Pública afirma "23 letras gregas minúsculas" e "13 letras gregas maiúsculas", mas a adenda `§P958` adiciona `digamma`, `omicron` (minúsculas) e 11 maiúsculas (`Chi`, `Eta`, `Iota`, `Kappa`, `Mu`, `Nu`, `Omicron`, `Rho`, `Tau`, `Upsilon`, `Zeta`). O leitor não sabe se a lista original já as inclui ou se são adições pós-P958.
- **Bloco 2** (leve) — Secção `§P958` (linhas 204–230): Mistura linguagem de especificação com linguagem de diagnóstico/processamento ("Medição", "Inventário contra o vanilla", "Correcção"), confundindo o propósito de um prompt L0 perene com o registo de um passo concreto.
- **Bloco 3** (grave) — Lista de identificadores em `is_math_function` (linhas 76–79): Afirma quais nomes Typst são funções matemáticas a renderizar em upright, mas não cita documentação oficial (`typst.app/docs/`) nem medição directa contra o vanilla.
- **Bloco 3** (grave) — Regra de `is_single_letter_var` (linhas 83–84): Afirma que variáveis de uma única letra ASCII "devem ser renderizadas em itálico matemático", sem citação da documentação oficial ou medição directa.
- **Bloco 3** (grave) — Lista de operadores em `is_large_operator` (linhas 95–99): Afirma que estes caracteres pertencem à classe `Large` vanilla, mas a lista não é fundamentada por citação ou medição directa.
- **Bloco 4** (leve) — Cabeçalho (linha 6): Contém "Passo 36" e "Passo 49" fora da secção de histórico de revisões.
- **Bloco 4** (leve) — Corpo do prompt: Múltiplas referências a passos: `P780` (linha 36), `P902` (linha 38), `P894` (linha 45), `P812-D` (linha 52), `P772w` (linhas 101, 147, 148, 178) e `P780` (linha 149).
- **Bloco 4** (leve) — Secção "Histórico de Revisões" (linhas 200–201): Inclui números de passo (`P772w`, `P780`) na coluna Motivo — o histórico só pode conter data e descrição, nunca número de passo.
- **Bloco 4** (leve) — Secção `§P958` (linhas 204–230): Contém referências a passos no título e no corpo (`P958`, `P820`, `P303`, `P780`, `P772w`).
### `compiler/layout/bib_csl.md`

- **Bloco 1** (leve) — Estrutura geral: Falta uma secção com o papel de **Critérios de Verificação** (testes/asserts de saída CSL por `CitationForm`).
- **Bloco 2** (leve) — “Integração no layout”, ~l.58: “quando … o cache falha” não especifica condições de falha (parse, style não encontrado, locale inválido, etc.).
- **Bloco 3** (grave) — “API pública”, ~l.42: Afirmação “default `"ieee"`” sem citação `docs.typst.app` nem medição directa.
- **Bloco 3** (grave) — “API pública”, ~l.38: Afirmação sobre as 4 forms de citação (`Normal`, `Prose`, `Author`, `Year`) sem fonte de linguagem.
- **Bloco 4** (leve) — Cabeçalho, ~l.6: Referência `P418` no campo “Origem”, fora de histórico de revisões.
- **Bloco 4** (leve) — Secção “P420 (M) — CSL customizado via path”, ~l.68: Título contém referência `P420`.
### `compiler/layout/bibliography.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção de **Critérios de Verificação**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção de **Resultado Esperado**.
- **Bloco 2** (grave) — “`format_bib_entry_body`”, l.138: Comentário “campos opcionais: year, volume, pages, etc.” deixa por especificar quais campos, ordem e separadores.
- **Bloco 3** (grave) — “Propósito”, l.17: “estilo numérico como default: `[1]`, `[2]` ordenados por primeira aparição” sem citação/medida.
- **Bloco 3** (grave) — “`cite.rs` — P472 ibid.”, l.74-75: Comportamento `ibid.` para estilo numérico sem citação/medida.
- **Bloco 3** (grave) — “`cite.rs` — P468 CitationStyle::Numeric”, l.113-119: Formatação de `Prose`/`Author`/`Year` sem citação/medida.
- **Bloco 3** (grave) — “`cite.rs` — P468 CitationStyle::Numeric”, l.127: Fallback “Entry `None` + Numeric/Normal → `[key]`” sem citação/medida.
- **Bloco 4** (leve) — Várias (l.6, 17, 24, 46, 55, 72, 96, 125, 143, 158, 167, 168): Múltiplas referências a passos (`P468`, `P472`, `P159G`, `P420`) fora de `Criado em`/`Histórico de Revisões`; incluindo entradas de histórico que contêm números de passo.
### `compiler/layout_counters.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais** (camada, dependências proibidas).
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — “Critérios de verificação”, ~l.20: “texto contém o número formatado” não define qual formato.
- **Bloco 2** (leve) — “Regras de negócio”, secção inteira: Limites de escopo não explícitos: resets, contadores aninhados, numbering personalizado.
- **Bloco 3** (grave) — “Regras de negócio”, l.12-17: Comportamento de `SetHeadingNumbering`/`CounterUpdate`/`CounterDisplay` sem citação docs/medida.
### `compiler/layout/enum_item.md`

- **Bloco 2** (grave) — “Validação”, l.96-97: Usa `line_advance` sem definição no prompt (não se sabe se é `line_height`, `paragraph_advance`, etc.).
- **Bloco 2** (grave) — “Semântica”, pontos 2-3 vs. “Validação”, l.95-97: Inconsistência: ponto 2 reseta `last_was_loose_item=false`, impedindo soma com espaçamento de itens soltos; validação afirma gap de `2×line_advance` com `Parbreak`.
- **Bloco 2** (leve) — “Semântica”, ponto 3, l.38-42: `line_height` não é formalizado (de onde vem?).
- **Bloco 3** (grave) — “Semântica” e “Validação”: Comportamento de parâmetros `indent`/`body-indent`/`tight` e reinício após `Parbreak` sem citação docs/medida.
- **Bloco 4** (leve) — Cabeçalho e corpo (l.5, 27, 32, 36, 79, 95): Referências `P380`, `P505`, `P864`, `P762` fora de histórico.
### `compiler/layout/equation.md`

- **Bloco 2** (leve) — “Regras de negócio” › “Numeração automática”, l.72: “à direita da página” é impreciso; P987 corrige para `content_end + gutter`, mas a regra geral não foi actualizada.
- **Bloco 2** (leve) — “P967”, l.275: Termo `THICK` usado sem definição local.
- **Bloco 2** (leve) — “Critérios de verificação” vs “Critérios de aceitação”: Duas secções com papéis equivalentes; pode induzir dúvida sobre o gate oficial.
- **Bloco 4** (leve) — Várias (l.18, 25, 30-62, 78, 89, 103, 113, 141, 164, 232, 241, 268, 290): Dezenas de referências a passos (`P800`, `Passo 48`, `P813`, `P451`, `P784`, `P893`, `P895`, `P896`, `P944`, `P945`, `P952`, `P967`, `P987`, etc.).
### `compiler/layout_figure.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — “Regras de negócio”, l.17: “fallback para arábico” não define se é pattern `"1"`, função, etc.
- **Bloco 2** (leve) — “§P488”, l.42: Acrónimo “LoF” não definido na primeira menção.
- **Bloco 2** (grave) — “Regras de negócio” l.23 vs “§P488” l.44: Relação entre `numbering` e `caption` não coberta: figura com `numbering` mas sem `caption`.
- **Bloco 3** (grave) — “Regras de negócio”, l.18-21: Prefixos i18n (`"Figure"`, `"Figura"`, `"Abbildung"`) sem citação docs/medida.
- **Bloco 4** (leve) — “Regras de negócio” l.18; “Import adicionado (P470)” l.28; “§P488” l.42, 53: Referências `P470` e `P488` fora de histórico.
### `compiler/layout/footnote.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 3** (grave) — “Instrução” › “Aplicar `numbering`”, l.30-31: Default vanilla `"1"` sem citação docs/medida.
- **Bloco 3** (grave) — “Instrução” › “Renderizar em superscript”, l.37-38: Generalização “o vanilla usa apenas o número formatado (ex.: `¹`, `*` )” sem citação/medida.
- **Bloco 3** (leve) — “Critérios de verificação”, l.68: Formato das notas no rodapé apresentado como invariante sem fonte.
- **Bloco 4** (leve) — “Contexto” l.15-16; “Instrução” l.23, 29, 34; “Critérios de verificação” l.76: Referências `P381`, `P1016`, `P793`, `P844`, `P448`, `Passo 1016` fora de histórico.
### `compiler/layout/heading.md`

- **Bloco 2** (grave) — “Comportamento”, item 2, l.19: “Se `cursor_x` já passou da margem” não define o limite exacto.
- **Bloco 2** (leve) — “Patterns suportados”, l.36 vs “Tests canónicos”, l.49: Lista `"I."` vs teste `"I.I"` hierárquico — relação não clara.
- **Bloco 2** (leve) — “Comportamento”, item 2, l.19: `flush_line()` invocado sem definição local.
- **Bloco 3** (grave) — “Contexto” l.12; “Comportamento” item 1, l.18: `bold=true`, `italic=false` para headings sem citação docs/medida.
- **Bloco 4** (leve) — Cabeçalho l.6; “Scope-outs” l.41; título “P978” l.57; corpo P978 l.61: `P451` em `Criado em`; `P241` nos scope-outs; `P978`/`P975` no corpo.
### `compiler/layout-image.md`

- **Bloco 2** (grave) — “Contrato de comportamento” ponto 6 vs “`calculate_dimensions`” passo 5: Contradição/incoerência: contrato promete DPI real (EXIF/JFIF/PNG), mas passo 5 descreve conversão com `PX_TO_PT = 1.0` (72 DPI).
- **Bloco 2** (leve) — “Critério de aceitação”, l.180: “não regressom” (sic) não define baseline nem critério de regressão.
- **Bloco 2** (leve) — “`layout`”, passo 8, l.134: Termo `in_main_flow` usado sem introdução/definição.
- **Bloco 4** (leve) — Várias (l.15, 19, 20, 29, 33, 67, 178): Referências `P768`, `P769`, `P770`, `P771`, `P773`, `P776` fora de histórico.
### `compiler/layout/link.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — “Algoritmo”, passo 1: “Renderizar `body` normalmente” não especifica modo/método.
- **Bloco 2** (leve) — “Algoritmo”, passo 2: Referência a `current_line`/`current_items`/`flush` sem definição no contrato do `Layouter`.
- **Bloco 2** (leve) — “Algoritmo”, passo 3: “bbox acumulada” sem definição de como se combina.
- **Bloco 2** (leve) — “Scope-out”: Termo `QuadPoints` não definido.
- **Bloco 2** (grave) — “Decisão arquitetural” vs “Algoritmo”, passo 4: `LinkTarget::Destination(Label)` é definido, mas o algoritmo só descreve `Url(url)` para `Content::Link`.
- **Bloco 3** (grave) — Documento como um todo: Nenhuma afirmação sobre hiperligações do Typst acompanha citação docs/medida.
- **Bloco 3** (grave) — “Algoritmo”, passo 4: Generalização “`Content::Link` mapeia sempre para URL” sem fonte.
- **Bloco 4** (leve) — Cabeçalho, l.6: Referências `P422` e `P424` no campo “Origem”.
### `compiler/layout/list_item.md`

- **Bloco 2** (grave) — “Semântica” pontos 2-3 e “Validação”, l.32-33, 39, 76: Inconsistência terminológica: `paragraph_advance`, `line_height`, `line_advance` usados sem clarificar se são distintos ou sinónimos.
- **Bloco 3** (grave) — “Semântica”, ponto 4 (l.42-48) e ponto 9 (l.64-65): Significado de `indent`/`body-indent`/`tight`/`marker-align` sem citação docs/medida.
- **Bloco 4** (leve) — Cabeçalho e corpo (l.5, 27, 32, 36, 64, 75, 81): Referências `P380`, `P505`, `P864`, `P762`, `P504` fora de histórico.
### `compiler/layout.md`

- **Bloco 1** (leve) — Final do documento: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — “Comportamento”, l.39; “P867”, l.45-69: Constante `MARGIN` usada em regras de wrap/paginação sem definição.
- **Bloco 2** (grave) — “Tipos e interface”, l.33 vs “P181H”, l.328: Contradição interna: `layout(content: &Content)` vs `layout(content: &Content, initial_state: CounterStateLegacy)`.
- **Bloco 2** (grave) — “Tipos e interface”, l.18-25 vs “metrics.rs”, l.707-719: Contradição interna na definição do trait `FontMetrics` (`char_width`/`line_height`/`font_size` vs `advance`/`vertical_metrics`/`text_edges`/etc.).
- **Bloco 2** (grave) — “Referências e Contadores Automáticos”, l.264 vs “P184D”, l.406: Contradição: P59 afirma “`Figure`: variante não existe em `Content`”; P184D descreve `Figure-arm` de `Content::Figure`.
- **Bloco 2** (leve) — “measure_content_constrained”, l.823: Referência a `FontMetrics::text_width` não consta das definições do trait no prompt.
- **Bloco 2** (grave) — “P864”, l.1426-1437, 1462, 1475 vs “Reset de contador de enum”, l.1492-1496: Contradição sobre reinício de enum por `Parbreak`: tabela diz que não reinicia; subsecção diz o oposto.
- **Bloco 2** (leve) — “§P842”, l.1382: Termo `Nfr` em `h(Nfr)` não definido.
- **Bloco 2** (leve) — “Comportamento”, l.39: `leading` usado no avanço de `Parbreak` sem definição inicial.
- **Bloco 3** (leve) — “P867 — `#set page(...)`”, l.45-69: Comportamento de `auto` em `width`/`height` sem citação docs/medida.
- **Bloco 3** (leve) — “Avanço vertical e cálculo de line advance”, l.71-99: Fórmula `line_advance = top_edge + |bottom_edge| + leading` e defaults sem citação docs/medida (fontes vanilla citadas só suportam `TextEdge`).
- **Bloco 3** (leve) — “Segmentação de linha para scripts sem espaços (P756)”, l.189-237: Afirmações sobre CJK/Thai/Lao/Myanmar/Khmer e tailoring de aspas sem citação docs/medida.
- **Bloco 3** (leve) — “Referências e Contadores Automáticos (Passo 59)”, l.247-276: Comportamento de `Labelled`, `Ref`, numeração automática sem citação docs/medida.
- **Bloco 4** (leve) — Distribuído por todo o documento: 273 ocorrências do padrão de passo (`Pxxx`, `Passo N`) fora de `Criado em`/`Histórico de Revisões`.
### `compiler/layout_outline.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 3** (grave) — “Regras de negócio”, l.29-32: Afirmação de paridade vanilla (entrada usa `number` sem supplement; headings não numerados → `number=None`) sem citação docs/medida.
- **Bloco 4** (leve) — Várias (l.17, 18, 29, 47, 73, 75, 81, 83, 88, 102, 123, 133): Referências `P457`, `P502`, `P359`, `P472`, `P488`, `§P488` fora de histórico.
### `compiler/layout/raw_highlight.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (grave) — “Comportamento de Layout”, item 1, ~l.38: “estilos `bold`/`italic` conforme a cor de primeiro plano do `Style`” confunde dimensões (em `syntect`, `font_style` é independente da cor).
- **Bloco 2** (leve) — “Comportamento de Layout”, item 1, ~l.39: Termo “palavra/segmento” não definido.
- **Bloco 2** (leve) — “Comportamento de Layout”, item 1, ~l.39: “preservando espaços” não especifica espaços iniciais/finais/tabs/múltiplos.
- **Bloco 2** (leve) — “Comportamento de Layout”, item 1: Comportamento quando linguagem não é encontrada em `RAW_SYNTAXES` não especificado.
- **Bloco 2** (leve) — “Comportamento de Layout”: Diferença entre `raw` inline e bloco não explicitada.
- **Bloco 3** (leve) — “Contexto e Objetivo”, ~l.12: Afirmação sobre syntax highlighting do Typst vanilla via `syntect`/`two-face` sem citação docs/medida.
- **Bloco 3** (leve) — “Paleta de Cores e Temas”, tabela: Cores hex por escopo apresentadas sem indicar fonte (tema `two-face` ou medição).
- **Bloco 4** (leve) — Cabeçalho, ~l.6: Referência `P785a` no campo “Origem”.
### `compiler/layout_references.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (grave) — “Regras de negócio”, l.26 vs “§P788”, l.49: Contradição: label inexistente → `"?"` vs erro fatal.
- **Bloco 2** (grave) — “§P788”, l.53-57 vs critérios “§P856”, l.122: Contradição: ref a heading sem numbering → erro vs “continua a funcionar (sem regressão)”.
- **Bloco 2** (leve) — “Regras de negócio”, l.27: “Abordagem A” mencionada sem definição.
- **Bloco 2** (grave) — “§P788”, l.67: “mensagem genérica equivalente ao vanilla” não especificada.
- **Bloco 4** (leve) — Várias (l.9, 13, 21, 45, 51, 56, 57, 70, 85, 87): Referências `P462`, `Passagem 1`, `P463`, `§P788`, `P786 A10`, `P786 A8`, `§P856`, `P856` fora de histórico.
### `compiler/layout/shape_block_behaviour.md`

- **Bloco 2** (leve) — “Propósito”, l.19; “Critério de aceitação”, l.83-84: Acrónimo **“AE”** usado como métrica sem definição no prompt.
- **Bloco 2** (leve) — “1. Contrato de comportamento”, l.31; “Critério de aceitação”, l.85: Comportamento de formas dentro de sub-layouts (`#box`, `#grid`, etc.) não explicitado.
- **Bloco 3** (grave) — “1. Contrato de comportamento”, l.31-32: Afirmação específica sobre ancoramento vertical do vanilla sem citação docs/medida nem medição directa.
- **Bloco 4** (leve) — Cabeçalho l.6; “Propósito” l.16; “1. Contrato de comportamento” l.30-32; “2. Ponto de intercepção” l.43, 48; “3. Espaçamento” l.58; “4. Impacto em `place()`” l.66-67; “5. Impacto nos testes” l.71, 77; “Critério de aceitação” l.81: Múltiplas referências `P767`, `P763h`, `P748`, `P750`, `P767c`, `P767a`, `P250`, `P763f`, `P745-P762` fora de histórico.
### `compiler/layout/table.md`

- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — Estrutura geral: Falta secção com papel de **Resultado Esperado**.
- **Bloco 2** (leve) — Instrução, l.13-15: Comportamento quando `table.numbering` não é `Value::Str` não especificado.
- **Bloco 2** (leve) — Instrução, l.20-22: Comportamento quando `caption` existe mas não há pattern não especificado.
- **Bloco 2** (leve) — Instrução, l.24-32: Casos de borda de `header`/`footer` não delimitados (múltiplos, vazios, repetição em quebras).
- **Bloco 2** (leve) — Scope-out, l.70: “vanilla default é acima” assume contexto sem precisar.
- **Bloco 3** (leve) — Scope-out, l.70: Afirmação sobre vanilla default do caption sem citação docs/medida.
- **Bloco 3** (leve) — “§P661”, l.66-67: Afirmação “no vanilla, numeração de tabelas passa sempre por figure” sem citação docs/medida.
- **Bloco 4** (leve) — Cabeçalho e corpo (l.6-8, 23-32, 34-46, 48-73): Referências `P380`, `P459`, `P157A`, `P224`, `P772v`, `P772i`, `P488`, `P661` fora de histórico.
### `compiler/introspect/convergence.md`

- **Bloco 4** (leve) — Cabeçalho, linha 6 (`P174`): `Criado em` contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 13 (`P174`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Contexto, linha 15 (`P163`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Semântica, linha 43 (`P162`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Consumers, linha 76 (`P174`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Histórico de Revisões, linha 95 (`P174`): Histórico contém número de passo.
### `compiler/introspect/extract_payload.md`

- **Bloco 4** (leve) — Cabeçalho, linha 6 (`P162`): `Criado em` contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 17 (`P162` (×2)): Duas referências a passos no corpo do prompt.
- **Bloco 4** (leve) — Secção “Mapeamento…”, linha 28 (`P162`): Título contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 33 (`P168`): Nota de linha de tabela contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 35 (`P169`): Nota de linha de tabela contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 36 (`P171`): Nota de linha de tabela contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 37 (`P171`): Nota de linha de tabela contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 38 (`P182C`, `P182B`): Nota de linha de tabela contém números de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 39 (`P178`): Nota de linha de tabela contém número de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 40 (`P181D`, `P181`): Nota de linha de tabela contém números de passo.
- **Bloco 4** (leve) — Tabela mapeamento, linha 41 (`P186C`, `P186D`): Nota de linha de tabela contém números de passo.
- **Bloco 4** (leve) — “Tests obrigatórios”, linha 74 (`P162`): Título da secção contém número de passo.
- **Bloco 4** (leve) — Consumers actuais, linha 86 (`P162`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Consumers planeados, linha 90 (`P162`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Histórico, linha 114 (`P162`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 115 (`P181D`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 116 (`P182C`, `P171`, `P173`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 117 (`P186C`, `P186D`): Histórico contém números de passo.
### `compiler/introspect/fixpoint.md`

- **Bloco 4** (leve) — Cabeçalho, linha 6 (`P174`): `Criado em` contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 21 (`P174`, `P175`): Corpo contém números de passo.
- **Bloco 4** (leve) — Interface pública, linha 67 (`P175`): Doc comment da função contém número de passo.
- **Bloco 4** (leve) — Consumers, linha 148 (`P174`, `P175`): Corpo contém números de passo.
- **Bloco 4** (leve) — Sobre paridade, linha 158 (`P163`, `P173`): Corpo contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 173 (`P174`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 174 (`P175`): Histórico contém número de passo.
- **Bloco 4** (leve) — Secção “P844…”, linha 178 (`P844`, `P831`): Secção inteira referencia passos fora de histórico.
### `compiler/introspect/from_tags.md`

- **Bloco 4** (leve) — Cabeçalho, linha 6 (`P165`): `Criado em` contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 13 (`P162`): Referência a passo no corpo do prompt.
- **Bloco 4** (leve) — Restrições Estruturais, linha 21 (`P173`): Corpo contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 33 (`P170`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 34 (`P184B`, `P184A` (×2), `P168`): Item de lista contém múltiplos números de passo.
- **Bloco 4** (leve) — Lógica, linha 36 (`P169`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 37 (`P171`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 38 (`P178`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 39 (`P181E`, `P181A` (×2)): Item de lista contém números de passo.
- **Bloco 4** (leve) — Lógica, linha 40 (`P186E`, `P186D`, `P186B`, `P184B`, `P186A`, `P188`): Item de lista contém múltiplos números de passo.
- **Bloco 4** (leve) — Lógica, linha 43 (`P182C`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 44 (`P171`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 45 (`P173`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Lógica, linha 48 (`P171`): Item de lista contém número de passo.
- **Bloco 4** (leve) — Interface pública, linha 68 (`P173`, `P171`): Nota pós-assinatura contém números de passo.
- **Bloco 4** (leve) — “Tests obrigatórios”, linha 88 (`P165`): Título da secção contém número de passo.
- **Bloco 4** (leve) — Consumers actuais, linha 100 (`P165`): Corpo contém número de passo.
- **Bloco 4** (leve) — Sobre paridade, linha 113 (`P162`): Corpo contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 130 (`P165`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 131 (`P173`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 132 (`P182C`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 133 (`P181E`, `P181C`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 134 (`P184B`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 135 (`P186B` (×2)): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 136 (`P186D`, `P185D`, `P186E`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 137 (`P186E`, `P186B`, `P183C`, `P188`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 138 (`P195B`, `P195C`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 139 (`P195C`, `P195B`, `P195D`): Histórico contém números de passo.
### `compiler/introspect/locatable.md`

- **Bloco 4** (leve) — Cabeçalho, linha 6 (`P164`): `Criado em` contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 15 (`P164`): Corpo contém número de passo.
- **Bloco 4** (leve) — Contexto, linha 17 (`P162`): Corpo contém número de passo.
- **Bloco 4** (leve) — Secção “Cobertura”, linha 32 (`P164`, `P169`, `P171`, `P178`, `P181D`): Título da secção e nota contêm números de passo.
- **Bloco 4** (leve) — Cobertura, linha 37 (`P169`, `P171`, `P178`): Lista contém números de passo.
- **Bloco 4** (leve) — Cobertura, linha 38 (`P181D`): Lista contém número de passo.
- **Bloco 4** (leve) — Cobertura, linha 39 (`P182C`): Lista contém número de passo.
- **Bloco 4** (leve) — Cobertura, linha 41 (`P186D`, `P183C`, `P186C`): Lista contém números de passo.
- **Bloco 4** (leve) — “Tests obrigatórios”, linha 80 (`P164`): Título da secção contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 146 (`P164`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 147 (`P181D`, `P181A`, `P181`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 148 (`P182C`): Histórico contém número de passo.
- **Bloco 4** (leve) — Histórico, linha 149 (`P186D`, `P186C`, `P185D`, `P183C`): Histórico contém números de passo.
- **Bloco 4** (leve) — Histórico, linha 150 (`P992`): Histórico contém número de passo.
### `entities/elements/align.md`

- **Bloco 1** (grave) — Final do ficheiro (após secção `eq`): Falta a secção de **Critérios de Verificação** (e/ou Resultado Esperado); só existem `Struct`, `impl Element` e `eq`.
- **Bloco 2** (leve) — Linha 6: “**Não-locatável**”: Termo técnico do projecto usado sem definição local nem referência que o defina.
- **Bloco 2** (leve) — Linha 7: “Contentor de prosa”: Expressão não definida; papel do elemento é descrito informalmente.
- **Bloco 4** (leve) — Linhas 5–6: “Lote 7 **P322**”, “confirmado **P322**”: Referências a passo fora do `Histórico de revisões`.
### `entities/elements/bibliography.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linha 8: “**Fronteira: LOCATÁVEL**”: Termo não definido no prompt.
- **Bloco 2** (leve) — Secções P418/P419/P420: “CSL built-in”, “hayagriva crate”, “path relativo/absoluto”, “erros legíveis” — termos/valores assumidos sem enumeração.
- **Bloco 3** (grave) — Secções P418/P419/P420: Afirmações sobre semântica Typst (paridade linguística de `.bib`/`@key`, resolução de path, built-in vs ficheiro `.csl`) sem citação a `docs.typst.app` nem medição directa.
- **Bloco 4** (leve) — Várias (linhas 8, 21–25, 33, 56, 60, 64, 67, 75, 80, 85, 91, 99–100): Referências a passos fora do histórico: **P181C**, **P159A**, **P325**, **P418**, **P419**, **P420**.
### `entities/elements/block.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linha 7: “**Não-locatável**”: Termo não definido no prompt.
- **Bloco 2** (leve) — Linha 51: “preserva os 13 cosméticos”: “Cosméticos” não é definido; lista implícita dos campos.
- **Bloco 4** (leve) — Linhas 5–6: “Lote 15 **P330**”: Referência a passo fora do histórico.
### `entities/elements/boxed.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linha 7: “**Não-locatável**”: Termo não definido no prompt.
- **Bloco 2** (leve) — Linha 5: “família densa do L12”: “L12” não é definido localmente.
- **Bloco 2** (leve) — Linha 46: “preserva os 9 cosméticos”: Termo “cosméticos” não definido.
- **Bloco 4** (leve) — Linhas 5–6: “Lote 14 **P329**”: Referência a passo fora do histórico.
### `entities/elements/cite.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linha 9: “**Fronteira: LOCATÁVEL**”: Termo não definido.
- **Bloco 2** (leve) — Linhas 64–65: “Introspector”, “forward references”: Conceitos de infraestrutura não definidos no prompt.
- **Bloco 3** (grave) — Secção P418: Afirmações sobre Typst/CSL (“renderização CSL real via hayagriva”, forward references, formas `Normal`/`Prose`/`Author`/`Year`) sem citação nem medição.
- **Bloco 4** (leve) — Linhas 5–6, 9, 31, 60, 64, 67, 74: Referências a passos fora do histórico: **P324**, **M1**, **P320**, **P418**.
### `entities/elements/colbreak.md`

- **Bloco 2** (leve) — Linha 6: “**Não-locatável**”: Termo não definido.
- **Bloco 4** (leve) — Linhas 5–6: “Lote 5 **P320**”, “confirmado **P320**”: Referências a passo fora do histórico.
### `entities/elements/columns.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linha 7: “**Não-locatável**”: Termo não definido.
- **Bloco 2** (leve) — Linhas 8, 21–23: “P552”, “wrap_page_columns”: A transformação e a flag `page_columns` não são explicadas o suficiente para saber quando exactamente ocorrem.
- **Bloco 3** (grave) — Linhas 8, 21–23: Afirmação sobre a distinção entre `#columns(N)[...]` e `#set page(columns: N)` no Typst sem citação docs.typst.app nem medição.
- **Bloco 4** (leve) — Linhas 5–6, 8, 21: Referências a passos fora do histórico: **P323**, **P552**.
### `entities/elements/_comum.md`

- **Bloco 2** (leve) — Linhas 95–100, 105, 109: “locatável/não-locatável”, “S1”, “F-1/F-2+”, “spike-2” usados sem definição local.
- **Bloco 4** (leve) — Linhas 5, 52, 104: Referências a passos fora do histórico: **P316**, **P334**.
### `entities/elements/counter_display_callback.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 8–11: “LOCATÁVEL”, “apply_counter_displays pós-fixpoint”, “consumo por payload” — termos não definidos localmente.
- **Bloco 4** (leve) — Linhas 5–6: Referências a passos fora do histórico: **P321**, **P241**.
### `entities/elements/counter_display.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 8–9: “legacy single-pass”, “DEBT-10”, “resolve no Layouter directo” — termos não explicados.
- **Bloco 4** (leve) — Linhas 5–6: Referências a passos fora do histórico: **P321**.
### `entities/elements/counter_update.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 9–10: “LOCATÁVEL”, “CounterRegistry”, “apply_at/apply_hierarchical_at” — termos não definidos.
- **Bloco 4** (leve) — Linhas 5–6: Referências a passos fora do histórico: **P321**, **P198C**.
### `entities/elements/curve.md`

- **Bloco 4** (leve) — Linha 4: “**Passo 513**”: Referência a passo no cabeçalho.
### `entities/elements/divider.md`

- **Bloco 2** (leve) — Linha 15: “singleton estrutural (**P154B**)”: Jargão de lote/projecto; o termo “singleton estrutural” não é definido.
- **Bloco 4** (leve) — Linhas 5, 15, 34: Referências a passos fora do histórico: **P316**, **P154B**.
### `entities/elements/enum_item.md`

- **Bloco 2** (leve) — Linha 8: “**Não-locatável**”: Termo não definido.
- **Bloco 2** (leve) — Linhas 19–26: `EnumNumbering`, comportamento exacto de `indent`/`body_indent`/`tight` não são definidos.
- **Bloco 3** (grave) — Linhas 65–70: Afirmação sobre Typst vanilla (`indent`/`body-indent`/`tight` como propriedades do container `enum`) e divergência mecânica pretendida sem citação docs.typst.app nem medição.
- **Bloco 4** (leve) — Linhas 5–8, 20–24, 36, 40, 60, 63: Referências a passos fora do histórico: **P318**, **P470**, **P505**.
### `entities/elements/equation.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 8, 11: “LOCATÁVEL”, “M6 eixo 2 ADR-0068”, “counter_update: Step” — termos não explicados.
- **Bloco 3** (grave) — Linhas 15–19, 37–39: Afirmações sobre numeração de equações no Typst (padrão vive na chain, `equation.numbering`, gate `block && pattern.is_some()`) sem citação docs.typst.app nem medição.
- **Bloco 4** (leve) — Linhas 5–8, 15, 17, 37: Referências a passos fora do histórico: **P325**, **P186B**, **P456**, **P365/P454**, **P364/P456**.
### `entities/elements/figure.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 9, 45, 53: “Fronteira: LOCATÁVEL”, “is_counted”, “infer_kind_from_body” — termos não definidos.
- **Bloco 3** (grave) — Linhas 24–25, 45: Afirmações sobre semântica de `figure` (caption opcional, numeração automática, condição `numbering.is_some() && caption.is_some()`) sem citação docs.typst.app nem medição.
- **Bloco 4** (leve) — Linhas 5–6, 9: Referências a passos fora do histórico: **P328**, **M1**.
### `entities/elements/footnote.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 6–7, 40: “Locatável desde P1016”, “scope-out de P326 (P295 Fase 1 marker-only)”, “ao contrário de TableElem” — dependem de contexto externo não explicado.
- **Bloco 3** (grave) — Linhas 7–9, 40: Afirmações sobre Typst vanilla (counter flat `footnote`, `counter(footnote)`, contagem sem gate) sem citação docs.typst.app nem medição.
- **Bloco 4** (leve) — Linhas 5–7, 27, 29, 40: Referências a passos fora do histórico: **P326**, **P1016**, **P502**, **P295**.
### `entities/elements/grid_cell.md`

- **Bloco 1** (grave) — Final do ficheiro: Falta a secção de **Critérios de Verificação** / Resultado Esperado.
- **Bloco 2** (leve) — Linhas 6–8, 42: “Não-locatável”, “estruturas gémeas”, “9 cosméticos” — termos não definidos.
- **Bloco 4** (leve) — Linhas 5–6: Referência a passo fora do histórico: **P327**.
### `entities/elements/grid_footer.md`

- **Bloco 2** (leve) — Linhas 6–7: “Não-locatável”, “par simétrico de GridHeader” — termos não definidos.
- **Bloco 4** (leve) — Linhas 5–6: Referência a passo fora do histórico: **P320**.
### `entities/elements/math_frac.md`

- **Bloco 2** (grave) — linha 7, contexto: "Comportamento idêntico ao braço atual do hub" não identifica o referente concreto (ficheiro/função), inviabilizando verificação.
- **Bloco 2** (grave) — linhas 28, 29, 30, 31, 37: Referências `content.rs:<linha>` sem caminho absoluto; ambiguidade na proveniência da medição.
- **Bloco 2** (leve) — linha 37, `eq estrutural`: Uso de "paridade" para descrever `PartialEq` cria ambiguidade com o conceito técnico de ADR-0107.
- **Bloco 4** (leve) — linhas 5, 7: Referências a `P317` fora do histórico de revisões.
### `entities/elements/math_limits_override.md`

- **Bloco 3** (grave) — Contexto, linhas 17–22: Comportamento de `scripts()`/`limits()` no vanilla descrito sem citação a `docs.typst.app` nem medição directa.
- **Bloco 3** (grave) — Contexto, linhas 25–27: Mapeamento `inline=true/false` para `Limits::Always/Display` atribuído ao vanilla sem citação ou `file:line`.
- **Bloco 3** (grave) — Contexto, linhas 38–40: Afirmação sobre `resolve_scripts`/`resolve_limits` não afetarem `MathClass`/espaçamento sem citação ou medição.
- **Bloco 4** (leve) — cabeçalho, linha 5: Referência a "Passo 992" fora do histórico de revisões.
- **Bloco 4** (leve) — cabeçalho, linha 7: Referências a `P296`, `P298`, `P772y` fora do histórico de revisões.
- **Bloco 4** (leve) — Contexto, linha 34: Referência a `P899 Parte D` fora do histórico de revisões.
- **Bloco 4** (leve) — Contexto/Despacho, linhas 36, 44, 93: Múltiplas referências a `§P992` fora do histórico de revisões.
- **Bloco 4** (leve) — Despacho, linha 96: Referência a `P311b.4` fora do histórico de revisões.
- **Bloco 4** (leve) — Secção `P992b`, linhas 109–111: Título e texto da secção contêm referências a `P992` fora do histórico de revisões.
### `entities/elements/math_matrix.md`

- **Bloco 4** (leve) — cabeçalho, linhas 5, 7: Referências a `P317` fora do histórico de revisões.
- **Bloco 2** (leve) — cabeçalho, linha 7: "Comportamento idêntico ao braço atual do hub" não define o referente nem permite verificação.
- **Bloco 2** (leve) — linha 31, `impl Element`: Termo "math structural" não é definido nem citado.
### `entities/elements/math_op.md`

- **Bloco 1** (grave) — estrutura geral: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 3** (grave) — cabeçalho, linhas 7–8: Afirmação sobre mecanismo vanilla e efeito de `limits` em `MathAttach` sem citação docs ou medição.
- **Bloco 3** (grave) — linha 30, tabela `impl Element`: Generalização "`limits` é layout-only" apoiada só em `content.rs:1655`, sem citação docs.
- **Bloco 4** (leve) — cabeçalho, linhas 5 e 8: Referências a `P317` e `P298` fora do histórico de revisões.
### `entities/elements/math_root.md`

- **Bloco 4** (leve) — cabeçalho, linhas 5 e 7: Referências a `P317` fora do histórico de revisões.
- **Bloco 1** (leve) — estrutura geral: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — cabeçalho, linha 7: "Comportamento idêntico ao braço atual do hub" não define o referente.
### `entities/elements/math_styled.md`

- **Bloco 1** (leve) — final do documento, após `Critério`: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — tabela `impl Element`, linha ~30: "Comportamento (idêntico ao braço atual)" não define o referente nem como verificar.
- **Bloco 4** (leve) — cabeçalho/contexto, linha 5: Referência a `P316` fora do histórico de revisões.
- **Bloco 4** (leve) — cabeçalho/contexto, linha 8: Referência a `P314` fora do histórico de revisões.
### `entities/elements/math_underover.md`

- **Bloco 1** (leve) — estrutura geral: Falta secção/equivalente funcional com o papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — final do documento, linhas 42–45: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — cabeçalho, linhas 5–7: "Idêntico ao braço atual do hub" não define o referente.
- **Bloco 2** (leve) — cabeçalho, linha 6; tabela, linha 36: Termo "não-locatável" usado sem definição.
- **Bloco 2** (leve) — cabeçalho, linha 8: Termos "graded" e "cluster vanilla under/over" não definidos nem auto-evidentes.
- **Bloco 2** (leve) — tabela `impl Element`, linha 34: Termo "math structural" usado sem definição nem citação.
- **Bloco 2** (leve) — documento todo: Limites de escopo implícitos; não explicita scope-outs deliberados.
- **Bloco 3** (leve) — várias secções: Afirmações sobre Typst sem citação `docs.typst.app` nem medição directa contra o binário vanilla.
- **Bloco 4** (leve) — cabeçalho, linhas 5, 7, 8: Referências a `P317` e `P297` fora do histórico de revisões.
### `entities/elements/metadata.md`

- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Restrições**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (grave) — linha 6: "Comportamento idêntico ao braço atual" não identifica o ponto de comparação.
- **Bloco 2** (leve) — linhas 38–39: Termo "terminais" aplicado a `map_content`/`map_text` não é definido.
- **Bloco 2** (grave) — linhas 43–48: Expressão "marcadores efectivos" e natureza do quirk de `eq` não são definidos.
- **Bloco 3** (grave) — linhas 18–30, secção `Struct`: Representação interna/construtor descritos sem citação docs ou medição vanilla.
- **Bloco 3** (grave) — linha 36: `plain_text` → `String::new()` referenciado só como `content.rs:1655`, sem proveniência vanilla.
- **Bloco 3** (grave) — linhas 43–48, secção `eq`: Afirmação de que `Metadata` cai em `_ => false` sem citação ou medição.
- **Bloco 3** (grave) — linha 54: Alegação de paridade vanilla (`MetadataElem.value`) sem citação `docs.typst.app` nem medição.
- **Bloco 4** (leve) — linha 5: Referência a `P321` fora do histórico de revisões.
- **Bloco 4** (leve) — linha 8: Referência a `P169` fora do histórico de revisões.
- **Bloco 4** (leve) — linha 52: Referências a `P844` e `P831` fora do histórico de revisões.
### `entities/elements/outline.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Restrições**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — linha 6, cabeçalho: "Comportamento idêntico ao braço atual" não define o referente.
- **Bloco 2** (leve) — linha 10, bloco Modelo D: "Migração aterrada neste módulo sem tocar o hub" usa "hub" sem definição.
- **Bloco 2** (grave) — linhas 102–104, secção `with_target`: Exemplo inválido: campo `indent` do tipo `OutlineIndent` recebe `true` (bool).
- **Bloco 3** (leve) — linhas 33–35, secção `OutlineTarget`: Defaults "List of Figures" / "List of Tables" sem citação docs ou medição.
- **Bloco 3** (leve) — linhas 53–58, secção `OutlineIndent`: Tipos aceites por `outline(indent:)` no vanilla sem link docs nem medição.
- **Bloco 3** (leve) — linha 73, secção `Struct`: Defaults de título sem citação docs ou medição.
- **Bloco 3** (leve) — linhas 74–75, secção `Struct`: Afirmação de que `depth`/`indent` só são relevantes para `Headings` sem citação ou medição.
- **Bloco 4** (leve) — linha 5, cabeçalho: Referências a `P323` e `P457` fora do histórico de revisões.
- **Bloco 4** (leve) — linha 14, bloco Fronteira: Referência a `P178` fora do histórico de revisões.
- **Bloco 4** (leve) — linha 22, secção `OutlineTarget`: Referência a `P472` no título, fora do histórico de revisões.
- **Bloco 4** (leve) — linha 41, secção `OutlineIndent`: Referência a `P502` no título, fora do histórico de revisões.
- **Bloco 4** (leve) — linhas 69, 82–83, 99: Múltiplas referências a `P472` fora do histórico de revisões.
### `entities/elements/overline.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — final do documento: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 3** (leve) — linha 7: Afirmação de `OverlineElem` como não-locatável sem citação docs ou medição vanilla.
- **Bloco 4** (leve) — linhas 5, 7 e 8: Referências a `P319` e `P284` fora do histórico de revisões.
### `entities/elements/pad.md`

- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Restrições Estruturais**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — tabela `impl Element`, linha ~30: "Comportamento (idêntico ao braço atual)" não define o referente.
- **Bloco 2** (leve) — cabeçalho, linhas ~6–7: Termos "Não-locatável" e "Contentor" não definidos no prompt.
- **Bloco 4** (leve) — cabeçalho, linha 5: Referência a `P325` fora do histórico de revisões.
### `entities/elements/pagebreak.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Restrições Estruturais**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (grave) — cabeçalho, linhas 5–7: "Comportamento idêntico" é ambíguo face às diferenças de campos entre `PagebreakElem` e `Divider`.
- **Bloco 2** (leve) — secção `Struct`, linha 22: "Construtor ergonómico preservado" não especifica localização nem assinatura exacta.
- **Bloco 2** (grave) — secção `Struct` vs. secção `Critério`: Contradição interna: `#[derive(..., Hash)]` vs. "`Hash` manual via Debug".
- **Bloco 4** (leve) — cabeçalho, linhas 5–7 e linha 26: Referências a `P320` fora do histórico de revisões.
### `entities/elements/place.md`

- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — secção `impl Element`, linha ~37: "Comportamento (idêntico ao braço atual)" não define o referente.
- **Bloco 2** (leve) — cabeçalho, linha ~8: Termo "Não-locatável" usado sem definição.
- **Bloco 2** (leve) — secção `impl Element`, linhas ~41–42: Expressão "os 6 campos cosméticos" não define quais são nem o critério.
- **Bloco 2** (leve) — documento todo: Limites de escopo implícitos (layout/render, `PlaceScope`, etc.).
- **Bloco 3** (leve) — cabeçalho, linha ~8: Afirmação "`Place` é Não-locatável" sem citação docs ou medição vanilla.
- **Bloco 3** (leve) — cabeçalho, linha ~8: Afirmação "`Place` é Contentor — `map_*` recursam no `body`" só apoiada em `content.rs`, sem docs.
- **Bloco 4** (leve) — cabeçalho, linha ~5: Referência a `P324` fora do histórico de revisões.
### `entities/elements/quote.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 4** (leve) — cabeçalho, linha 5: Referência a `P323` fora do histórico de revisões.
### `entities/elements/raw.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Restrições Estruturais**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (grave) — linhas 6–7 e 29–31: "Comportamento idêntico ao braço atual" não define o referente.
- **Bloco 2** (leve) — linha 6: Termo "Não-locatável" não definido.
- **Bloco 2** (leve) — linha 23: "Construtor ergonómico preservado" não define assinatura/comportamento.
- **Bloco 2** (leve) — linha 24: "syntax highlighting real continua scope-out" é ambíguo quanto ao que fica excluído.
- **Bloco 4** (leve) — linhas 5–6: Referências a `P322` fora do histórico de revisões.
- **Bloco 4** (leve) — linha 24: Referência a `P502` fora do histórico de revisões.
### `entities/elements/ref.md`

- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — secção "Notas P462", linhas 42–44: Lista de supplements default não deixa claro se é exaustiva ou exemplificativa.
- **Bloco 3** (grave) — secção "Notas P462", linhas 40–41: Afirmação sobre resolução do número no layout via `Introspector` sem citação docs.
- **Bloco 3** (grave) — secção "Notas P462", linhas 42–44: Supplements default sem citação docs ou medição.
- **Bloco 4** (leve) — linhas 5 e 37: Referências a `P462` fora do histórico de revisões.
### `entities/elements/repeat.md`

- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (grave) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — cabeçalho, linha 6: Termo "Não-locatável" usado sem definição.
- **Bloco 2** (leve) — secção `impl Element`, linha 31: "Comportamento (idêntico ao braço atual)" não define o referente.
- **Bloco 4** (leve) — cabeçalho, linhas 5 e 6: Referências a `P322` fora do histórico de revisões.
### `entities/elements/shape.md`

- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Critérios de Verificação**.
- **Bloco 1** (leve) — documento global: Falta secção/equivalente funcional com o papel de **Resultado Esperado**.
- **Bloco 2** (leve) — cabeçalho, linha 7: Termo "Não-locatável" usado sem definição.
- **Bloco 4** (leve) — cabeçalho, linha 5: Referência a `P326` fora do histórico de revisões.
### `entities/elements/smartquote.md`

- **Bloco 4** (leve) — cabeçalho, linha 5: Referência a `P324` fora do histórico de revisões.

## Metodologia e escopo

Aplicou-se o workflow `00_nucleo/prompts/auditar-spec.md` a cada prompt L0 do corpus, usando os 4 blocos:

1. Completude estrutural (5 papéis do template-prompts.md).
2. Ambiguidade de conteúdo (afirmações verificáveis, definições de termos, limites de escopo, contradições internas).
3. Fundamentação (citação docs.typst.app ou medição directa contra o vanilla).
4. Zero referência a passo (números de passo fora de `Criado em`/`Histórico de Revisões`).

Foram auditados 301 prompts, distribuídos por 17 partições (ver `temp/p1021/partition_*.txt`).
Foram excluídos desta primeira passagem os ~25 prompts já fatiados desde P999:

- `compiler/eval/operators/*`
- `compiler/eval/bindings/*`
- `compiler/eval/closures.md`
- `compiler/eval/font_dict.md`
- `compiler/eval/selector_matching.md`
- `compiler/stdlib/structural*`

Também foram excluídos os meta-prompts `auditar-spec.md`, `auditar-fatiamento.md` e `template-prompts.md`.

## Notas

- Este relatório é um catálogo; não propõe correcções.
- A classificação de severidade é indicativa: `grave` quando a ambiguidade pode afectar comportamento gerado; `leve` quando se limita a prosa ou organização do documento.
- O critério de exclusão dos prompts já fatiados não implica aprovação desses prompts — apenas adia a auditoria para passo futuro.
- Alguns achados de Bloco 4 (zero referência a passo) são herdados de prompts escritos antes da regra vigente; a correcção sistemática fica para passo dedicado.
