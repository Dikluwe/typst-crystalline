# P1232 — reproduzir e decompor a contradição R01

**Estado:** EXECUTADO — DIAGNÓSTICO ACEITE COM LACUNAS DE EVIDÊNCIA (`P1229_REOPENED`)  
**Predecessor:** P1231 `REJECTED`  
**Saída:** classificação reproduzível de cada falha R01, sem alterar produção.

## Objetivo

Reexecutar os 30 controles sRGB/P1229 do verificador final P1231 e separar,
por fixture, quatro eixos: grafo SVG, amostra numérica unidimensional, geometria
local e raster externo. Congelar binários, scripts, fixtures, fontes, renderer,
escala, bbox, máscara e hashes. Comparar diretamente os artefatos P1229 e P1231
e explicar por que P1229 aceitou três combinações enquanto o universo expandido
agora falha 14/30 probes numéricos e 19/30 probes raster.

## Execução

1. Registrar HEAD, hora, árvore suja integral e SHA-256 de todas as entradas.
2. Reproduzir duas vezes, em ordem direta/inversa, `raw-numeric.tsv`,
   `raw-raster.tsv` e `raw-graphs.tsv` da cadeia FINAL-v2.
3. Rodar os mesmos SVGs apenas no avaliador numérico, sem layout/raster.
4. Fazer diff entre contrato, budgets, fixtures e algoritmo dos pacotes P1229
   e P1231; nenhuma diferença pode ser descrita apenas como “mais rigor”.
5. Classificar cada falha como `HARNESS`, `BUDGET-DRIFT`, `FIXTURE-EXPANSION`,
   `L1-SAMPLE`, `SVG-APPROXIMATION`, `SERIALIZATION` ou `EXTERNAL-LAYOUT`.

## Gate

Não editar L0 ou código. Produzir
`00_nucleo/diagnosticos/p1232-r01-reproducao.tsv` e relatório com `file:line`.
Se a reprodução não for determinística, P1233 recebe primeiro a causa da
nondeterministicidade; nenhuma correção de cor é autorizada.

## Resultado

A retomada criou o manifesto executor que faltava e reexecutou integralmente o
FINAL-v2 duas vezes. Cada execução já contém duas ordens diretas e duas inversas.
`raw-numeric.tsv` e `raw-raster.tsv` são byte a byte idênticos ao P1231 e as 124
linhas de grafo pertencentes a R01 também são idênticas. O resultado reproduz
14/30 falhas numéricas e 19/30 falhas raster.

A contradição é primariamente `FIXTURE-EXPANSION`: P1229 mediu três casos
reduzidos; P1231 introduziu seis morfologias novas por combinação, com
interseção normalizada zero. A sonda direta separa 5 fixtures com
`L1-SAMPLE-OUT-OF-BUDGET` em `t=0` (delta máximo de 209 níveis u8), 21 com
diferença direta de um nível dentro do orçamento e 4 sem diferença direta.
Os eixos SVG/raster registram 9 falhas conjuntas, 5 apenas numéricas e 10
apenas raster; a execução não preservou evidência suficiente para atribuir
causa exclusiva `SVG-APPROXIMATION` a todas elas. Cinco fixtures passam todos
os eixos, incluindo a sonda direta. P1229 também excluía `t=0` sob uma premissa
invertida sobre qual stop coincidente o vanilla escolhe.

Não houve edição de L0 ou produção. P1229 permanece reaberto; P1233 precisa ser
reexecutado sobre esta decomposição, sem herdar sua retificação rejeitada.

Esta execução sustenta uma **reprodução diagnóstica**, não um protocolo Tekt
completo: a lista histórica exata da árvore suja, o contrato original, os
recibos encadeados, o selo e o gate discriminatório não foram preservados em
artefatos versionáveis. Os números servem para reabrir P1229 e orientar P1233,
mas não para fechar causalidade exclusiva nem equivalência funcional.

**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO:** as capacidades e allowlists foram
declaradas, mas os papéis compartilharam filesystem e a cadeia causal completa
não é auditável a partir dos artefatos persistidos.
