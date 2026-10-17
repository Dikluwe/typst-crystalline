# Candidato de estilo de Cell sob demanda — 2026-10-10

## Mudança e decisão

A realização construía dois candidatos de Cell em passagens distintas mesmo sem regra Style elegível. Agora cada passagem cria seu snapshot somente quando encontra a primeira regra Style não guardada, e reutiliza o mesmo snapshot nas demais. A ordem de matching e aplicação de estilos permanece a anterior. Decisão: manter a mudança, com ganho de alocações e tempo modesto confirmado em duas janelas. O RSS subiu 104–312 KiB nos controles; é um custo observado, não explicado como ruído nem compensado por uma alegação de menor heap vivo. O sinal do tempo inline e local-400 foi favorável nas seis rodadas; local-1600/no-fill tiveram sinais mistos na primeira janela. Nenhuma fase, contrato público, estrutura de Content ou despacho foi alterado. L0 atualizado antes de código; resselo restrito ao owner.

## Alocações

O gate passou em todas as cinco fixtures. Local/inline de 1.600 células caíram 3.200 pedidos e 1.945.600 bytes solicitados (1,86 MiB); no-fill de 1.600 caiu 1.600 pedidos e 972.800 bytes. Em 400 células, local caiu 800 e no-fill 400 pedidos. Isto mede tráfego acumulado do alocador, não heap vivo nem RSS. As contagens são glibc malloc/calloc/realloc/memalign, três repetições com interpositor; os tempos com interpositor não fundamentam desempenho.

## Tempo e RSS sem instrumentação

Duas janelas independentes, cada uma com três rodadas de oito pares antigo/candidato por fixture, warmup excluído, ordem alternada e CPU fixa. Diferença de tempo é mediana de razões pareadas; RSS é diferença das medianas em KiB.

| Caso | Tempo janela 1 | Tempo janela 2 | RSS janela 1 (KiB) | RSS janela 2 (KiB) |
|---|---:|---:|---:|---:|
| local-set-400 | -1.18% | -1.30% | +178 | +252 |
| local-set-1600 | -0.10% | -0.87% | +248 | +312 |
| no-fill-1600 | -0.57% | -0.29% | +234 | +104 |
| inline-set-1600 | -1.76% | -0.83% | +238 | +192 |

O ganho é pequeno e específico; não declarar aceleração geral nem redução de RSS. Preservar as duas janelas e os sinais por rodada para auditar variação. A equivalência visual certificada aqui é antigo/candidato; as diferenças anteriores de raster contra vanilla continuam abertas.

## Verificação

Teste unitário RED de execução, com falha `ineligible rule must not clone the cell`, seguido de GREEN; também verifica reutilização do mesmo snapshot nas regras elegíveis. Suíte core: 6.683 passaram, cinco ignorados. Cargo build debug e release passaram. Crystalline-lint retornou zero com avisos preexistentes. Vinte e cinco documentos mantiveram texto, palavras, páginas, caixas, raster 36 dpi e avisos idênticos entre baseline e candidato, incluindo Cell show-set correspondente/não correspondente. Seis consultas table retornaram 1 nos três motores. Apenas um arquivo Rust produtivo mudou, verificado por SHA-256 contra snapshot dos 966 arquivos.

## Proveniência e reprodução

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b; working tree não commitado. before/final.json.gz registram status e diff HEAD --stat completos, UTC, hashes de fontes e conteúdo dos owners. Baseline Tekt 584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba; candidato 71e968c20d5ba32c652b2b66c4fd444c672158ba9bb27b0272f59cfaa5530c78. Oracle pin 9f2b6e8715237cb086899a42873660fe744622e8; binary 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea (receita/árvore ratificadas P1489). Não inferir proveniência por versão.

Receipts allocation-equal-red/green, qualification-tables/query, measurements-first/risk-repeat guardam argv, ambiente, fixtures, fontes tipográficas, CPU, stack, UTC e hashes de scripts/binários. Slots old/new foram congelados em /tmp/tekt-table-candidate. Scripts reproduzíveis estão na pasta deste diagnóstico; executar allocation-equal-argv.py red/green, qualify_tables.py, qualify_query.py e measure-risk-repeat.py. Nova execução sobrescreve recibos: guardar a janela anterior primeiro. Patchs textuais do único owner e logs de testes/build/lint ficam preservados. Nenhum commit, reset ou reparo global de hashes foi feito.

Janelas UTC: 2026-10-10T16:43:07.190803+00:00 a 2026-10-10T16:43:40.402240+00:00; 2026-10-10T16:43:49.529405+00:00 a 2026-10-10T16:44:21.463139+00:00.
