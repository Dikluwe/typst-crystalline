# Memo da resolução de fontes em StyleChain — experimento rejeitado

## Evidência anterior e hipótese

O baseline SHA256 `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8` é idêntico ao de style-scope-transfer-2026-10-10. Sua coleta de stacks inclui normalização de nomes e construção de vetores em StyleChain::font (`style_chain.rs:633–654`). Há 400 amostras de pedidos480bytes em Vec::from_iter sob esse resolver no caso local1600, além de normalizações; amostragem periódica inclusiva não fornece custo exclusivo. Set text.font persiste Array em `compiler/eval/rules/set/text.rs:269`. Hipótese: memo local elimina reconstrução repetida sem mudar a linguagem.

## Candidato e testes

L0 editado primeiro. Um OnceLock<Arc<Option<FontList>>> privado por nó cacheava a conversão na primeira leitura, inclusive None que bloqueia herança; delta tipado, ordem, primeira chave e filtro somente Str eram preservados. Não havia normalização no push, cache global, I/O, catálogo ou fase nova. O carrier padrão mede16bytes em `carrier-probe.json` (stand-in Vec no payload); isso não mede classe do alocador ou RSS.

O teste de backing strings longas foi RED→GREEN. 29 testes do módulo e 6.683 testes L1 passaram,5 ignorados. Semântica: precedência tipada, regex/variants, herança, arrays vazios/mixed/dict, duplicatas, collapse e captura original. 25 tabelas+8 escopos preservaram texto/páginas/bbox/raster36dpi; seis query(table).len() deram1 nos três motores. Build dev/release e lint saíram0; avisos preexistentes, sem V13/V15/V26 ou drift deste owner. Esses resultados não certificam paridade total nem corrigem o limite histórico de dict custom.

## Medição e rejeição

Três execuções por caso para pedidos glibc; o gate estrito de menos alocações em **todos** os casos falhou. Duas chamadas adicionais apareceram em no-fill e famílias; ganhos dos alvos são modestos. Tempo/RSS sem interposer:3rodadas×4pares por caso, warm-up excluído, CPU fixa, argv before/after com basename e comprimento iguais, Rayon1/stack64MiB. Testes/build/profiling terminaram antes.

| Caso | Alocações antes → candidato | Tempo pareado | RSS pareado |
|---|---:|---:|---:|
| local-set-400 | 154,052 → 153,254 | +0.01% | +0.78% |
| local-set-1600 | 328,505 → 325,307 | +0.83% | +0.72% |
| no-fill-1600 | 253,234 → 253,236 | +0.94% | +0.52% |
| inline-set-1600 | 459,588 → 456,390 | -0.22% | +0.94% |
| variants-1600 | 1,121,236 → 1,116,438 | -0.85% | +0.09% |
| families-1600 | 391,060 → 391,062 | +0.61% | +0.38% |

Rodadas de tempo:

- `local-set-400`: -0.64%, +8.45%, -0.24%
- `local-set-1600`: +1.56%, -1.61%, +1.28%
- `no-fill-1600`: +0.71%, +1.07%, +0.76%
- `inline-set-1600`: -0.37%, -0.82%, +0.93%
- `variants-1600`: -2.20%, -0.01%, +0.28%
- `families-1600`: +10.66%, +0.44%, +0.61%

Controles separados,3rodadas×4pares:

- `plain`: tempo +0.17%; RSS -0.01%.
- `unused-font`: tempo -0.77%; RSS +0.84%.
- `unused-style`: tempo -0.04%; RSS -0.47%.
- `default-scopes`: tempo +0.58%; RSS -0.01%.

**Decisão:** rejeitar. Não houve ganho consistente de tempo nos casos locais; RSS dos seis alvos subiu na mediana pareada. A redução de chamadas não compensa o cache residente neste experimento. A causa exata da latência não foi isolada. Fonte e L0 foram restaurados exatamente e o baseline reposto, comprovados em restoration.json, antes de iniciar a alternativa sem memo `style-font-capacity-2026-10-10`. O teste mecânico de backing compartilhado não foi incorporado.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before.json.gz e candidate-validation.json.gz guardam lista exata/diffstat, todos os consumers e bytes dos owners; medições guardam argv/fixtures/fontes/CPU/ambiente/script hashes. Candidato alterou somente style_chain.rs.

- Baseline `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`.
- Candidato rejeitado `e362db641be9808894bb2dbb1c6bffb1d9337539a6865bb4cebaa0e8d649dc3d`.
- Benchmark UTC `2026-10-10T04:38:58.227158+00:00` → `2026-10-10T04:40:02.015750+00:00`; controles UTC `2026-10-10T04:40:02.074787+00:00` → `2026-10-10T04:40:17.190948+00:00`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, receita/tree P1489. Variants não tem equivalência linguística com vanilla certificada.
- Artefatos: allocation-equal-red/green.json.gz, measurements-equal-argv.json.gz, measurements-cost-controls.json.gz, stacks-after.json.gz, qualification-*.json.gz, owner-diff.patch, seal.json e logs completos em candidate-validation.json.gz. A amostragem antes/depois mudou sua fase de alinhamento; não converter sua diferença diretamente em pedidos exclusivos.

## Continuação

A sonda seguinte testa capacidade exata do vetor, preservando a resolução vigente e sem adicionar memória a cada nó. Relatório: `00_nucleo/diagnosticos/style-font-capacity-2026-10-10.md`.
