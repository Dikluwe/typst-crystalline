# P1400 — relatório da segunda medição oficial

## Veredito

**INCONCLUSIVE / BLOCKED.** A cadeia reaberta distinguiu 9 rotas `ABSENT` e
9 `PARTIAL`, mas 75 rotas permanecem `UNKNOWN`. O P1400 não satisfaz o
critério de fecho e não autoriza implementação de produto.

Este resultado cobre somente as 93 rotas fora da família `format` congeladas
no P1400. Não é alegação de paridade funcional geral da linguagem Typst.

Regime: protocolo completo de materialização segregada, **executado sem
atestação de isolamento**, porque as autoridades compartilharam workspace e
contexto não isolado.

## Cadeia e proveniência

Inputs protegidos da revisão executada:

- contrato `b4be7e313c2772040f8473f0b00e689772060396eec6aa4821f6170c56fbc545`;
- oráculos `75ff90569ca462b819024c186f1705bdd41619c71edb521345549f32fcf1ceb2`;
- mutantes `e3e50c510680d826ab83872bda615342fee8ff9e6ae51b00e9a7c864d4fc9c00`;
- preseal `95c09cbb48fd43bc87271ba6e3747d15063d33b5e87748d97ad478990ceb4f2c`;
- runner `a4bcf6f2807fab8354ce296275d3e3c8b0dd41c7a6a77066e1c2320eb656e016`.

Predecessor causal: HEAD `2ded754d01f36294536d0d0feeea63c31209eba9`,
tree `a5018a56096a48e7dde4f4907b8a80c37b634bec`, working tree não
commitado com somente artefatos P1400 novos. Nenhum Prompt L0, Núcleo Tekt ou
código L1–L4 foi alterado.

A segunda medição ocorreu de `2026-09-14T23:38:01.363881+00:00` a
`2026-09-14T23:39:38.441057+00:00`; resultado SHA-256
`9da156aafbe572a0680f14a6c4b93aeff2b93ef63dde7bca6056a787feb62542`;
brutos em `/tmp/p1400-raw-official-r2-20260914`.

### Primeira tentativa preservada

A tentativa anterior não foi apagada da proveniência: resultado SHA-256
`cf7ca9350d128597985b13c092a6ea613f80d33037f636f7b63eddf7bc40668a`,
janela `2026-09-14T22:58:15.945298+00:00`–
`2026-09-14T22:59:36.840363+00:00`, 93
`UNKNOWN(PARSER_CONTROL_FAILED)`, brutos em
`/tmp/p1400-raw-official-20260914`. Ela foi o motivo público para reabrir a
cadeia; não conta como segunda execução da revisão atual.

## Gates e recursos

Antes da execução, `--validate-only` passou com 93 rotas e 182 probes por
ordem. O gate discriminatório rejeitou 33/33 mutações válidas, preservou 6/6
positivos e manteve 8/8 opacos como `Unknown`; `mutation_score = 1.0`. As
ordens do gate deram o mesmo hash
`1d12572cd774d1d6a41b3a383496511dadb6795796e9ddadb1b0dda1f26f951e`.

A medição registrou 1.092 recibos de subprocesso em 24 lotes de no máximo 12
rotas, paralelismo 1. Pico de RSS: `33764 KiB`; swap máximo: `0 KiB`;
timeouts: 0; truncamentos: 0; maior stdout: 259 bytes; maior stderr: 250 bytes.
As ordens `direct`, `direct-repeat` e `reverse` deram o mesmo vetor SHA-256
`d042f2bc63a921165ce6f8cb8018736622e6f82d28f0743c2ea53b92771b90cc`.

## Classificação das 93 rotas

### Ausência qualificada — 9

`color.spot`, `color.spot.tint`, `enum.item`, `figure.caption`,
`footnote.entry`, `list.item`, `outline.entry`, `polygon.regular` e
`terms.item` receberam `ABSENT(QUALIFIED_PUBLIC_FORM_ABSENT)`.

Esses estados são resultados do instrumento, não autorização automática para
código. A leitura L0 posterior mostrou cláusulas atuais que bloqueiam
explicitamente `color.spot`, `color.spot.tint` e `outline.entry` no gate de
contrato público ADR-0127. Como o runner não aplica L0 pós-medição, essa tensão
fica registrada e não é corrigida silenciosamente no resultado.

### Forma qualificada parcial — 9

`math.bold`, `math.cal`, `math.display`, `math.italic`, `math.sans`,
`math.scr`, `math.sscript`, `math.text` e `math.upright` receberam
`PARTIAL(QUALIFIED_ABSENT_FUNCTIONAL_ALTERNATIVE)`: a forma qualificada falta,
mas uma forma pública relacionada foi demonstrada bilateralmente em processo
independente. Isso não concede `MATCH` ao path ausente.

### Unknown — 75

- 44 HTML: `TARGET_CONTROL_FAILED(CLI_TARGET_SELECTION)`;
- 22 math: `NON_BILATERAL_EVIDENCE(ROUTE_ALTERNATIVE_PROJECTION)`;
- 3 construções de instância: `UNSUPPORTED_CONSTRUCTION` —
  `outline.entry.body`, `selector.after`, `selector.before`;
- 6 observações ausentes: `OUTPUT_MISSING` — quatro filhos restantes de
  `outline.entry`, `par.line` e `raw.line`.

Não houve `MATCH`, `DIFFERENCE` ou `SCOPE_OUT` no vetor emitido.

## Separação exigida

| Classe | Contagem | Estado probatório |
|---|---:|---|
| rotas públicas ausentes | 9 | conclusivas no runner; ainda sujeitas ao L0/ADR-0127 |
| implementações parciais | 9 | alternativas bilaterais executadas |
| diferenças semânticas/sintáticas/morfológicas | 0 | nenhuma demonstrada |
| lacunas do harness/fixture | 75 | `Unknown`, bloqueiam fecho |
| scope-outs emitidos | 0 | cláusulas L0 posteriores registradas, sem reclassificação silenciosa |
| matches atuais | 0 | nenhum demonstrado |
| rotas sem owner L0 provado | 0 | onze relações 1:1 auditadas |

## Ownership posterior à medição

Os onze L0s candidatos foram conferidos novamente após o resultado. Cada um
possui exatamente um consumer produtivo com header `@prompt`. Dez owners estão
`CURRENT`; `compiler/stdlib/figure_image.md` permanece `STALE`, pois seu próprio
texto declara que `native_figure` existe mas ainda não está especificado.

| Owner | Consumer | Rotas | Estado |
|---|---|---:|---|
| `compiler/stdlib/color.md` | `compiler/stdlib/color.rs` | 2 | CURRENT |
| `compiler/stdlib/html.md` | `compiler/stdlib/html.rs` | 44 | CURRENT |
| `compiler/stdlib/structural/math.md` | `compiler/stdlib/structural/math.rs` | 31 | CURRENT |
| `compiler/stdlib/structural/lists.md` | `compiler/stdlib/structural/lists.rs` | 3 | CURRENT |
| `compiler/stdlib/structural/outline.md` | `compiler/stdlib/structural/outline.rs` | 6 | CURRENT |
| `compiler/stdlib/structural/par.md` | `compiler/stdlib/structural/par.rs` | 1 | CURRENT |
| `compiler/stdlib/foundations/selector.md` | `compiler/stdlib/foundations/selector.rs` | 2 | CURRENT |
| `compiler/stdlib/shapes.md` | `compiler/stdlib/shapes.rs` | 1 | CURRENT |
| `compiler/stdlib/structural/markup.md` | `compiler/stdlib/structural/markup.rs` | 1 | CURRENT |
| `compiler/stdlib/figure_image.md` | `compiler/stdlib/figure_image.rs` | 1 | STALE |
| `compiler/stdlib/structural/footnote.md` | `compiler/stdlib/structural/footnote.rs` | 1 | CURRENT |

O ledger registra 93 linhas, o SHA-256 integral de cada L0 e a classe provável
ADR-0127. Nenhuma rota foi encaminhada diretamente a código.

## Próximo passo

`recommended_next_product_step` é literalmente `null`. Motivo: a segunda
medição oficial ainda é `INCONCLUSIVE`, com 75 `Unknown`; a instrução da cadeia
permite selecionar exatamente um passo de produto apenas se a medição for
conclusiva. Os 18 resultados discriminados permanecem evidência para uma
futura cadeia, mas não anulam esse bloqueio.

O próximo trabalho legítimo é diagnóstico: nova revisão selada deve resolver
o controle de target HTML, as projeções alternativas math e as construções ou
projeções de instância. Isso não é uma recomendação de produto e não autoriza
alterar L0/L1–L4 nesta execução.
