# P1335 — recomendação causal para P1336

## Medição antes da decisão

Estado: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado, com os 16 ficheiros produtivos/L0 alterados já pinados em `p1335-baseline.json`. Vanilla ratificado: upstream `a51e02804`. Hora, comandos, fontes, binários e canais completos estão nos recibos P1335; esta análise não executou produto nem mutantes.

As três ordens principais são estáveis em 18.872 células/4.718 probes. Há 18.220 coincidências brutas (96,545146%); o ajustado é 18.220/18.716 (97,349861%), excluindo 156 células de 39 extensões com retenção pública explicitamente documentada no L0 atual. Não há crédito adicional no numerador. Os três bindings `calc.deg`, `calc.rad` e `calc.log10` permanecem sem intenção autorizada e dentro do denominador. Todos os canais principais são literalmente iguais aos medidos em P1322: esta matriz de lookup/repr não demonstra avanço funcional.

Os suplementos medem 4.422 células/1.125 casos canônicos, também estáveis em três ordens. A reconciliação temporal distingue 64 fechamentos com fonte comparável de 12 observações atualmente fechadas sob fixture relocalizada, sem alegação de delta literal. Os 1.376 checkpoints históricos de preservação produzem 1.332 preservações literais, 32 correções posteriores autorizadas por mensagens/spans exatos e 12 preservações literais confirmadas na localização original. Cada fechamento P1323–P1334 e seus resíduos está enumerado em `p1335-classification-functional-reconciliation-r1.json`.

Testemunhas atuais, todas as quatro configurações e três ordens:

| Expressão | Vanilla | Cristalino |
|---|---|---|
| `(1).nope` | `cannot access fields on type integer`; span só `nope` | `cannot access fields on type int`; span da expressão inteira |
| `"abc".nope` | `cannot access fields on type string`; span só `nope` | `cannot access fields on type str`; span da expressão inteira |

Origem literal: `p1335-sentinels-{normal,repeat,reverse}-r3.json`, IDs `p1325.integer-boundary` e `p1325.string-boundary`. Nenhuma normalização dos canais foi usada para constatar essas diferenças.

## Causa atual e refutação

O L0 `00_nucleo/prompts/compiler/eval/bindings/field_access.md:320` exige explicitamente que o erro de `#(1).foo` nomeie `integer`. O consumer proprietário `01_core/src/compiler/eval/bindings/field_access.rs:533` exclui Int/Str da escolha de span de campo e `:836` usa `other.type_name()` no fallback. O helper de nomes vanilla já está importado em `:28`; `operators/error_formatting.rs:58` e `:60` já mapeiam Int/Str para integer/string. O despacho de métodos reais antecede o fallback (`field_access.rs:469`). Não é necessária uma nova entidade, um novo helper nem alteração a outro owner para a hipótese estreita de acesso direto ausente Int/Str.

Esta é uma inferência causal refutável: um canal completo que continue incorreto após limitar a mudança ao fallback/spans desses acessos, uma dependência produtiva adicional ou uma mudança de método válido refutariam a suficiência de um owner. `Type::Int` como namespace, chamadas de campo com argumentos, outras variantes e precedência de avaliação não estão incluídos.

## Decisão recomendada, sem implementação

Recomendar somente **`primitive-instance-field-diagnostic`** para P1336: prioridade 2 (contradição canônica), um owner completo, dois caminhos de linguagem da mesma causa, risco relativo 1. O conjunto de owners é apenas `compiler/eval/bindings/field_access.md` → `01_core/src/compiler/eval/bindings/field_access.rs`.

`math-lexical-callee-resolution` é a alternativa seguinte: também prioridade 2, um owner e dois caminhos (abs/sqrt), mas risco relativo 2 por alterar resolução matemática. Os controles com funções Content-returning e nome multigrafema `custom` isolam a causa lexical sem confundir o retorno Content-only nem as diferenças de serialização da equação. Aliases, quatro perfis e três ordens não aumentam a contagem de caminhos. O controle inicial de `f` de um grafema foi refutado como teste de callable e permanece registrado.

O desempate foi refeito após objeção independente: a cláusula `integer` do L0:320 impede classificar o primeiro candidato como mero diagnóstico de prioridade 3. Os scope-outs históricos de P1324–P1326 preservam aqueles reparos estreitos, não revogam a cláusula canônica. A tabela completa, inclusive coortes inelegíveis por owner/risco não provados, está em `p1335-classification-cohorts-r1.md`; a ordenação reproduzível está em `p1335-classification-selection-r1.json`.

P1336 deve primeiro atualizar e ressellar o L0 proprietário para a correção interna de paridade (ADR-0127, sem autorização de implementação por esta auditoria). A obrigação futura deve cobrir literalmente mensagem, span e canais dos dois acessos, e preservar métodos válidos Int/Str, namespace Type, Module/Dict/Content/Float, nativas/closures/With e precedência de argumentos. Especificação, código, ataques e veredito devem continuar segregados segundo `tekt-materializacao-segregada`; RED→GREEN e mutantes produtivos pertencem ao passo futuro, não a P1335. Qualquer necessidade de ampliar o contrato/owner exige reclassificação, não expansão silenciosa.

## Limites e artefatos ativos

Esta recomendação aguarda o veredito do revisor independente; não certifica os próprios ledgers. A dívida adversarial continua em `p1335-classification-certification-debt.json`: 37 famílias históricas P1322 mais o registro sobreposto de seis famílias P1308; zero mutantes produtivos P1335 e nenhum mutation score novo.

Artefatos ativos: `owner-ledger-r1.tsv`, `transitions-r1.tsv`, `supplemental-ledger.tsv`, `supplemental-transitions-final-r1.tsv`, `transversal-ledger-r1.tsv`, `summary-r1.json`, `selection-r1.json`, `functional-reconciliation-r1.json`, `source-lineage-r1.json`, todos com prefixo `p1335-classification-`. O suplemento mantém a preservação detalhada em `preservation-reconciliation.json`; os demais snapshots anteriores permanecem imutáveis.

Incidentes do classificador: `finalize.cjs` parou antes de emitir artefatos por tratar a chave documental `input` como caminho; `finalize-r1.cjs` parou antes de emissão ao exigir recibos de testes explicitamente desativados pelo perfil. `finalize-r2.cjs` emitiu a base; `finalize-r3.cjs` emitiu os sucessores ativos, separando os 12 fechamentos relocalizados e completando o mapa de ownership de wiring ausente do inventário parcial principal. Nenhum incidente alterou expectativas, fontes produtivas ou os canais medidos.

O ledger transversal retém seis recibos exit-2 inicialmente Unknown e referencia o overlay estrito posterior `p1335-cli-capability-overlay.json`: são ausência pública da forma CLI no vanilla, sem crédito de exportação/paridade. Os dois controles de serialização são C-only. Diferença de nome de recurso PDF e raster marginal não foi promovida a dívida de língua; texto/estrutura/geometria e diagnósticos permanecem eixos distintos.
