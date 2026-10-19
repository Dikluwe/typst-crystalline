# Reuso da cadeia primária do shaper — melhoria aceita

## Resultado

A resolução repetida da mesma cadeia de fontes foi substituída por reuso
limitado a uma entrada no FaceCache existente. Nos casos grandes com
estilos locais, controle e famílias alternadas, o tempo caiu cerca de 9%;
inline-set caiu 7,64%. As buscas no catálogo passaram de 9.619 para 19
(99,80% a menos), sem redução da quantidade de chamadas de primary_candidates.
A saída anterior/final foi preservada em 36 documentos. Não há ganho de
memória residente certificado; o RSS dos seis casos ficou próximo da base.

## Medição antes da decisão e classificação

`03_infra/src/shaper/font_selection.rs:16-91` antes do reparo resolvia
novamente a cadeia declarada, a recuperação por classe e os complementos
MATH/comum. `03_infra/src/shaper/cache.rs:64-87` já retinha as faces por slot.
A sonda `resolution-calls-before.json.gz` deste diagnóstico mediu, para
local-set/no-fill-1600, 1.602 chamadas de primary_candidates, 9.612 de
resolve_candidates e 9.619 de FontBook::select_pattern. A implementação
usava as mesmas entradas, mas repetia a seleção.

A escolha das famílias, sua ordem, variante, fallback e tabela MATH são
semântica de linguagem; repetir as buscas é mecânica interna. Hipótese
refutável: guardar a última resolução completa evita esse trabalho sem
aumentar retenção por número de pedidos. RED de consultas, refutadores de
chave/falha/escopo, alocações, comparação de saída e tempos/RSS verificaram
a hipótese. O reparo não reproduz vtable nem muda o enum Content.

## Implementação e fronteiras

- `03_infra/src/shaper/cache.rs` possui o carrier privado PrimaryResolution e a entrada opcional inicialmente vazia de FaceCache.
- `03_infra/src/shaper/font_selection.rs` possui lookup, reconstrução vigente no miss e publicação do resultado completo.
- A chave compara toda a FontList ordenada, FontVariant efetiva e math. Nenhuma comparação só por hash ou identidade de ponteiro.
- O valor contém slots e units-per-em, incluindo os complementos MATH/comum já existentes. Não conserva novos buffers de fontes.
- Tamanho, cor, idioma, direção, tracking e eixos explícitos continuam tratados nas fases que já os usavam. Cobertura global, similaridade, segmentação de script e chaves do cache de runs continuam independentes.
- Resolução vazia não é publicada. Qualquer None na map de faces impede conservadoramente uma nova publicação; a última entrada completa pode permanecer. A política anterior de ausência de face por slot continua: os testes comprovam novas consultas ao catálogo, não prometem releitura de um slot que já tinha negativo no FaceCache.
- A entrada dura o escopo da instância e do mesmo World/catálogo imutável de suas faces; uma instância nova começa sem resolução. Sem globais, novo unsafe ou eviction da map de faces.

Ownership estrito 1:1: prompts infra/shaper/cache.md e
infra/shaper/font_selection.md possuem somente seus respectivos consumers.
As invariantes comuns pertencem ao núcleo
`00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml`, com pin efetivo
`24dff6d00f015a26b2fdc0b84c18aa7debac4a43d5b0380b861cf5ac43403584`
nos dois owners. Código aponta somente aos prompts proprietários.
Mudança interna ADR-0127: nenhuma API pública, default, flag ou fase mudou.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Before/candidate JSON gzip, owner-diff.patch, nucleus.json, seal.json e
final-validation.json.gz neste diretório conservam UTC, lista exata de
arquivos alterados, diffstat, fontes, L0/núcleo, hashes e logs. A comparação
das 966 fontes produtivas identificou somente os dois consumers do shaper
como alterações desta rodada. L1/L2/L4 e a transferência de buffers PDF
anterior permanecem com os mesmos bytes de fonte.

- Tekt anterior SHA-256: `5e66c09d44e2b59545c98ce85dc8fb6aedb1df165d4641a07c1bfb100abb358a`.
- Tekt final SHA-256: `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`.
- Vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; receita/árvore ratificadas no diagnóstico P1489.

Tempos em measurements-equal-argv.json.gz: três rodadas com quatro pares
por fixture, warmups excluídos, ordem embaralhada, mesma CPU fixa, Rayon=1,
stack=64 MiB, mesmas fontes e SOURCE_DATE_EPOCH=0. Cópias dos dois Tekt têm
nome typst e caminhos de mesmo comprimento, com identidade SHA conferida.
UTC da janela: 2026-10-10T04:10:29.613882+00:00 → 2026-10-10T04:11:32.310560+00:00. Todos os argv,
fixtures/fontes/scripts e hashes estão nos receipts. Contadores GDB medem
chamadas, não tempo; ptrace foi autorizado somente para subprocessos de teste
criados pela tarefa. Nenhum profiler/debugger rodava durante o benchmark.

## Medições finais

Tempos são medianas; percentuais são medianas dos quocientes pareados.

| Caso | Tempo anterior → final (ms) | Tempo pareado | Pedidos de alocação anterior → final | Pico RSS pareado |
|---|---:|---:|---:|---:|
| local-set-400 | 113.88 → 110.86 | -2.60% | 162,048 → 154,052 | +0.16% |
| local-set-1600 | 159.91 → 145.29 | -8.84% | 360,501 → 328,505 | +0.11% |
| no-fill-1600 | 144.44 → 130.21 | -9.08% | 285,230 → 253,234 | +0.13% |
| inline-set-1600 | 182.33 → 168.78 | -7.64% | 491,584 → 459,588 | -0.11% |
| variants-1600 | 789.82 → 780.00 | -1.60% | 1,132,436 → 1,121,236 | -0.03% |
| families-1600 | 161.52 → 146.51 | -9.20% | 423,073 → 391,060 | +0.19% |

Local-set-1600 teve ganhos por rodada de 8,84%, 8,65% e 9,61%; no-fill
9,28%, 9,99% e 8,78%; inline-set 7,43%, 7,65% e 8,12%. O ganho menor em
variantes não foi extrapolado para equivalência contra vanilla. O RSS do
caso de variantes segue aproximadamente 286 MiB; este cache não fecha a
frente de buffers originais repetidos por FontKey.

Em local-set-1600, alocações caíram 8,88% (31.996 pedidos a menos); no-fill
11,22%; inline-set 6,51%. Famílias alternadas e variantes também caíram.
Os arquivos allocation-equal-red/green conservam três execuções por caso,
interposer, bytes solicitados e estado. O gate passou nos seis casos.

As sondas finais registram 1.602 primary_candidates (igual à base),
12 resolve_candidates (antes 9.612) e 19 select_pattern (antes 9.619)
em ambos os controles. A consulta das faces e a seleção global de glifos
não foram removidas como requisito: o reuso cobre a cadeia primária.

Na mesma janela, inline-set final ficou 5,12% mais rápido que vanilla,
local-set 14,81% e no-fill 22,30%. Estes números pertencem às fixtures e
a este estado; não certificam paridade global nem vantagem em todo documento.

## Verificação

- RED real: consultas de catálogo 54 contra 6 esperadas no teste de repetição; três refutadores já passavam. GREEN: quatro testes aprovados.
- Refutadores comparam reuso e instância fresca, em ambas as ordens: famílias declaradas/invertidas, regex, bold/italic, primeiro nome inexistente, math e face MATH; leitura bloqueada, bytes inválidos e catálogo vazio; novo World com slots invertidos.
- Suíte infra final: 1.279 aprovados, zero falhas, dois ignorados, serial e epoch fixo. L1 não mudou; sua suíte não foi repetida nesta rodada.
- 36 documentos preservaram texto, palavras, páginas/tamanhos, caixas extraídas com delta zero e raster a 36 dpi. Incluem tabelas, context, source/include, espaços, counter/query, fontes reais, matemática/RTL pequenos e VF. Seis sondas query(table).len retornaram 1 nos três motores.
- cargo build e cargo build --release passaram. crystalline-lint retornou zero, sem V5 nos owners alterados, V15 ou V26. Permanecem avisos anteriores em outras unidades; nenhum fix-hashes global foi aplicado.

## O que permanece aberto

A diferença local-set/no-fill é agora cerca de 15,1 ms nas medianas desta
janela. Esse delta inclui avaliação e layout; não é atribuição exclusiva a
layout_styled. A próxima hipótese é a reconstrução de estilos/propriedades
nos escopos locais, com medição direta antes de escolher uma representação
ou cache. A memória residual dos recursos de fontes e divergências de
raster/paginação em matemática repetida continuam frentes separadas.
