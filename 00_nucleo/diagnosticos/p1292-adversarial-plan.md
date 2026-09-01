# P1292 — plano adversarial selado

## Identidade e regime

- Regime: materializacao Tekt segregada completa; papel `atacante`.
- Executor: agente independente `/root/atacante_p1292`.
- Ambiente: filesystem compartilhado, fontes mutantes somente em `/tmp/p1292-adversarial-work`.
- Inicio: `2026-09-01T11:54:36-03:00`.
- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`.
- Arvore integrada: nao commitada; SHA-256 de `git status --short`:
  `5da3e880819c0d0b1921b45d599e29206f7c334b9cbb33fa24337ef6dcfa4bc6`.
- SHA-256 de `git diff HEAD --stat`:
  `c7c5468427160ec4c28adde8b7fcbce9b16bb474a67218e52c635f116368facc`.
- Contrato selado: `00_nucleo/diagnosticos/p1292-contract-seal.json`, SHA-256
  `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`.
- Oraculo protegido: `04_wiring/tests/p1292_contract.rs`, SHA-256
  `fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.
- Manifesto recebido: `00_nucleo/diagnosticos/p1292-manifest.json`, SHA-256
  `29ecb0b85c9431f55b001a7707c4b92b1146ca36c174cbb94729d89cf75461ba`.
- Baseline vanilla: `a51e02804`; binario `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Politica: todo mutante valido precisa produzir falha discriminante. Falha de
  build alheia ao comportamento, timeout, caso nao executado ou resultado
  ambiguo e `Unknown`, nunca morte. Meta: 23/23 mortos, score `1.0`,
  `Unknown = 0`.

## Capacidades e congelamento

O atacante pode ler L0, contrato, baseline, candidato e testes; pode escrever
somente este plano, `p1292-adversarial-receipt.json` e a copia temporaria. Nao
pode corrigir produto, contrato, L0, seal, testes integrados ou emitir veredito
final do passo. O produto e o oraculo integrado permanecem congelados. Cada
mutacao e aplicada a uma copia versionada em `/tmp`, executada, registrada e
restaurada antes da seguinte. A copia deve terminar byte-identica ao snapshot
inicial e os hashes integrados devem ser conferidos novamente.

Hashes focais congelados antes dos ataques:

| caminho | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/structural/math.rs` | `6b58286a0e99ae4acd7e2a32156c4da7ad091fdcec163ff6c14ed7c71060f241` |
| `01_core/src/compiler/math/layout/cancel.rs` | `6e6035ce00f113eaffd0613baf4d3164d033aaa4001eaa84c20f319568df0611` |
| `01_core/src/compiler/math/layout/underline.rs` | `203370f9bd940a3f61cf79fd3f12149ae75ee26d383e7eafb836bc73ac3700d9` |
| `01_core/src/compiler/math/layout/vec.rs` | `d1368de3e887cdd26fd1ee6277054bdf68bac20b8b0e7ee2915b4b8f554ec98e` |
| `01_core/src/compiler/eval/math.rs` | `e5dde4dfdeec1a07744129b0f5cd59e883f0a2eadca8c29c34a2d7c2b0904a87` |
| `01_core/src/compiler/eval/mod.rs` | `f774abf709988864c2b0ae462f52427215fad599cbe8189453f577ce22422c75` |
| `01_core/src/compiler/stdlib/layout.rs` | `495b7865ddabc2a557ed48ab152ffa07f304fc84cf5562135abdee64f6b47aa2` |
| `01_core/src/compiler/layout/flush.rs` | `54b2d5e97e1d009151efbae116db6058356dcf96cf73fa151e48891acbb5a105` |
| `01_core/src/compiler/layout/cursor.rs` | `55f3ee8c7287e22f5a9f93cbd263d8483be8fa9b1c3fd00a3f45711679e3709f` |
| `01_core/src/compiler/layout/mod.rs` | `9d4aa8e56a4e9157cbab345ddd40cfaae73c4c4c60f7fac85741b4c79e05069d` |
| `01_core/src/compiler/layout/place.rs` | `1bbdc72ee23d7d0d6b79fd1efee7866bf2d5efcadcb0b71957ccad8e05e98a24` |
| `01_core/src/compiler/layout/block.rs` | `635135faa7160030e73e39eff692f47d654023752a4a189f98e2bb980a3d1857` |

## Matriz de mutacoes selada antes da execucao

| id | mutacao negativa real | teste/oraculo discriminante primario |
|---|---|---|
| A1 | `math.cancel` aponta para a funcao historica textual `native_cancel_line` | `p1292_a_cancel_surface_repr_and_errors` |
| A2 | `stroke` e aceito mas descartado no layout | `p1292_a_cancel_surface_repr_and_errors` e SVG de stroke explicito |
| A3 | defaults de `length`/angulo deixam de ser os defaults canonicos | `p1292_a_cancel_callback_cross_context_span_and_background` |
| A4 | callback e resolvido uma unica vez e reutilizado no `cross` | `p1292_a_cancel_callback_cross_context_span_and_background` |
| A5 | erro do callback usa `Span::detached()` | `p1292_a_cancel_callback_cross_context_span_and_background` |
| B1 | `math.underline` vira alias do underline textual | `p1292_b_math_underline_surface_identity_and_errors` |
| B2 | gap sob a regra e fixado em pt | `p1292_b_math_underline_math_constants_styles_and_italic_correction` |
| B3 | espessura ignora `underbar_rule_thickness` MATH | `p1292_b_math_underline_math_constants_styles_and_italic_correction` |
| B4 | body ausente vira `Content::Empty` | `p1292_b_math_underline_surface_identity_and_errors` |
| B5 | dispatcher nao delega ao owner `underline::layout` | teste owner/dispatcher focal mais oraculo de layout B |
| C1 | sintaxe `vec` degrada para a forma de matriz | convergencia em `p1292_c_vec_gap_finite_auto_absolute_and_mixed` |
| C2 | zero filhos vira erro | `p1292_c_vec_cardinality_defaults_repr_and_errors` |
| C3 | `align` e aceito mas ignorado | SVG left/right em `p1292_c_vec_gap_finite_auto_absolute_and_mixed` |
| C4 | percentual de gap usa largura da regiao | casos finitos 10% em `p1292_c_vec_gap_finite_auto_absolute_and_mixed` |
| C5 | altura infinita participa diretamente e produz nao-finito/panico | casos `auto` em `p1292_c_vec_gap_finite_auto_absolute_and_mixed` |
| C6 | sintaxe e funcao constroem formas canonicas diferentes | pares repr em `p1292_c_vec_gap_finite_auto_absolute_and_mixed` |
| D1 | `native_flush` devolve `Content::Empty` | `p1292_d_place_flush_namespace_repr_and_errors` |
| D2 | marcador e no-op e so `finish()` drena | `p1292_d_flush_prefix_suffix_clearance_nested_and_noop` |
| D3 | fronteira inclui floats posteriores | prefixo/sufixo em `p1292_d_flush_prefix_suffix_clearance_nested_and_noop` |
| D4 | admissao top/bottom usa ordem invertida | mistura top/bottom no oraculo D |
| D5 | drenagem duplica ou descarta uma ocorrencia | contagem/posicoes FLOAT_BEFORE/FLOAT_AFTER no oraculo D |
| D6 | `place.with(...)` perde namespace | `p1292_d_place_flush_namespace_repr_and_errors` |
| D7 | argumentos extras sao silenciosamente ignorados | erros posicionais/named em `p1292_d_place_flush_namespace_repr_and_errors` |

Cada linha deve possuir comando executado, exit code, testemunha de falha e
restauracao no recibo. O plano fica selado neste ponto; alteracao posterior de
seu conteudo invalida a campanha.
