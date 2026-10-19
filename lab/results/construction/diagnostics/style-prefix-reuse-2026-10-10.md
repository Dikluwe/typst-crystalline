# Captura direta do prefixo de estilos — experimento rejeitado

## Resultado e decisão

O experimento substituiu clones de deltas, Vec temporário e reconstrução de
nós Arc em `local_delta_since` por verificação emprestada do ancestral e
dobra limitada aos nós locais. A saída permaneceu igual e a suíte do núcleo
passou, mas o controle repetido mostrou aumento de RSS na tabela com estilo
local. **Código e L0 restaurados exatamente ao snapshot anterior.**
A transferência de buffers PDF foi avaliada separadamente e permanece candidata.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`style-prefix-reuse-2026-10-10/before.json.gz` contém UTC, lista de arquivos
alterados, diffstat e os dois owners antes do experimento. `candidate.json.gz`,
`binary-sources.json`, `owner-diff.patch` e `seal.json` identificam o código
experimental e L0. Nenhum arquivo alheio foi restaurado.

Binário anterior: SHA-256
`783572ba02f8b423935179dd7c81f62895bcf21db9144d48d2fa924421249ef0`.
Binário experimental: SHA-256
`d4c9d7e85956e3e1da3345f7588cc999a3eb8794eb6795b09610b701fdb98ef2`.
Oráculo pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O alvo de aceitação neste experimento é preservação do Tekt anterior; não
houve certificação de paridade global com o vanilla.

## Medições antes da decisão

`allocation-red.json.gz` e `allocation-green.json.gz` registram três execuções
por fixture com interposer glibc, hashes, argv e UTC. Inline-set-1600 caiu de
491.584 para 486.784 pedidos (4.800 a menos); local-set-1600 poupou somente
três pedidos; no-fill e famílias alternadas mantiveram as contagens.

`measurements.json.gz` registra três rodadas de quatro pares, warmups
excluídos, ordem embaralhada, CPU fixa, Rayon=1, mesmas fontes e stack=64 MiB.
Mediana dos quocientes pareados de tempo: inline-set −0,08%, local-set −1,07%.
Não há ganho robusto de tempo. RSS local anterior/experimental: medianas
41.042/42.456 KiB; quociente pareado +3,15%.

O controle adicional em
`pdf-font-buffer-reuse-2026-10-10/with-prefix-measurements-controls.json.gz`
compara o original com prefixo+transferência PDF. Mesmo após a transferência,
RSS local sobe 40.896→42.480 KiB, +4,08% pareado; rodadas +4,58%, +3,39%,
+2,74%. Inline-set apresenta −2,09% RSS. Famílias alternadas têm +0,22% tempo
pareado no controle repetido, refutando um custo temporal persistente sugerido
na primeira janela. O PDF poupa cópias somente no export; não explica a
regressão já existente no experimento isolado de prefixo. A causa exata do
aumento de RSS não foi atribuída. Esses ensaios também comparavam nomes
diferentes de executável. A rodada posterior de transferência PDF, com nomes
e comprimentos de argv homogêneos, eliminou a diferença de bytes solicitados
nos controles. Assim, o aumento antigo de RSS não estabelece causa exclusiva
no prefixo lexical; é indicação com variável de confusão. O experimento de
prefixo não foi repetido isoladamente sob essa padronização.

`measurements-preliminary-contended.json.gz` coincidiu com compilações e não
foi usado para aceitação. Os registros são mantidos para auditabilidade.

## Verificação e limites

O teste novo de múltiplos nós verificou false explícito, primeira chave
custom duplicada, exclusão do ancestral e ancestral não pertencente. Passou
antes e depois; a trava RED→GREEN aqui foi a contagem externa de alocações,
pois a semântica foi preservada. Suíte experimental: 6.681 aprovados, cinco
ignorados, em `/tmp/style-prefix-core-tests.log`. A qualificação combinada
em `pdf-font-buffer-reuse-2026-10-10/with-prefix-qualification-*.json.gz`
preservou texto, caixas, páginas e raster de 36 documentos.

Poupar alocações sem um ganho temporal claro não sustentou a adoção. O
aumento de RSS motivou a reversão inicial, mas sua atribuição causal permanece
aberta por causa da variável de argv identificada depois. `decision.json` registra a rejeição; os snapshots e o patch
permitem reproduzir a hipótese sem deixar a implementação experimental no
produto. A reconstrução de TextStyle nos escopos locais continua aberta.
