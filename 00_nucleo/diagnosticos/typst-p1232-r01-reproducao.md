# P1232 — reprodução e decomposição R01

**Veredito:** `ACCEPTED_WITH_EVIDENCE_GAPS / P1229_REOPENED`.

O manifesto executor ausente na primeira tentativa foi materializado antes da
nova execução. Ele pinou o runner FINAL-v2, seus 104 fixtures, imagens e máscaras
vanilla, budgets, Python/Pillow, `rsvg-convert`, binários Typst e conjunto de
fontes. A medição ocorreu no HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, em working tree não commitada,
entre `2026-08-27T15:47:30-03:00` e `2026-08-27T15:54:55-03:00`.

A lista exata dos ficheiros alterados nessa árvore não foi preservada num
artefato versionável. A afirmação histórica de que `git status` e
`git diff HEAD --stat` foram registrados não substitui essa lista. Portanto,
pela ADR-0121, os números abaixo são evidência diagnóstica reproduzida no
ambiente ainda disponível, mas não prova suficiente para fechar o item.

## Medição antes da classificação

O executor P1231 (`/tmp/p1231-isolation/final2-verify/run_candidate.py:28-53`)
define cinco combinações R01 nativas, executa duas rodadas diretas e duas
inversas e mede SVG, raster `1x/2x/4x` e a malha numérica 4096. Duas execuções
integrais P1232 produziram os mesmos hashes:

- numeric: `698e9c34a0b1d77df3682f14933afc1c2dd46b85ebb44faf46290d2796606a95`;
- raster: `6ce7cad22b7d79f341066b277d62d4f3c64c7da98fee7bb81da599f22e85b428`;
- graphs: `8b21517d9ba8a0680823e4f09b1b9ebce722eb571ba1320ec4ac85a81371edfa`.

Numeric e raster também são byte a byte idênticos aos raws protegidos P1231.
Os grafos completos diferem porque o binário existente já contém promoções
posteriores fora de R01; filtradas as 124 linhas R01, o conteúdo é idêntico ao
P1231. Não houve nondeterminismo por repetição ou ordem.

O resultado reproduzido é 14/30 falhas numéricas e 19/30 falhas raster:

| combinação | numéricas | raster |
|---|---:|---:|
| Linear/sRGB | 0/6 | 5/6 |
| Radial/sRGB | 0/6 | 2/6 |
| Linear/Oklab | 6/6 | 5/6 |
| Linear/LinearRgb | 3/6 | 5/6 |
| Radial/Hsv | 5/6 | 2/6 |

## Decomposição causal

P1229 não possuía os “mesmos 30 controles”. Seu construtor reduzido está em
`/tmp/p1229-isolation/oracles/build_oracles.py:9-17,47-51`; P1231 define seis
morfologias por combinação em
`/tmp/p1231-isolation/oracles/scripts/build.py:31-40`. O join normalizado tem
interseção zero. Portanto a mudança de aceitação é primariamente
`FIXTURE-EXPANSION`, não regressão sobre fixtures anteriormente aceitas.

O avaliador direto, sem SVG ou raster, comparou 123.150 amostras públicas. Foram
observadas 68 diferenças codificadas: 63 de um nível u8 e cinco de 209 níveis.
As cinco diferenças grandes são exatamente os casos com offsets coincidentes em
`t=0`: vanilla escolhe o último stop e o cristalino escolhe o primeiro. Isso
classifica essas fixtures como `L1-SAMPLE`. A premissa P1229 estava invertida em
`build_oracles.py:68-70` e excluiu esse ponto do orçamento.

Nas demais fixtures, o avaliador direto diverge no máximo um nível u8 ou
coincide exatamente. Isso permite separar o eixo direto, mas não demonstra por
si só causa exclusiva das falhas SVG/raster. A tabela registra 9 falhas nos
dois eixos, 5 apenas numéricas, 10 apenas raster e 6 que passam ambos; uma das
seis ainda viola a sonda direta em `t=0`, restando cinco fixtures que passam
todos os eixos. A classificação completa em `p1232-r01-reproducao.tsv` separa
estado da sonda, eixos que falharam e força da inferência. Onde a execução não
preservou comparação causal bastante, usa `UNRESOLVED` em vez de promover a
inferência confortável `SVG-APPROXIMATION` a facto.

## Diferenças P1229 ↔ P1231 não fechadas

A execução demonstrou a expansão de fixtures e localizou a premissa invertida
de P1229 sobre `t=0`. Porém não persistiu um diff canônico completo dos
contratos, budgets e algoritmos dos dois pacotes. Assim:

- `FIXTURE-EXPANSION` é medido e preservado como causa da contradição agregada;
- a premissa de stop coincidente é medida nos builders ainda disponíveis;
- `BUDGET-DRIFT`, mudança contratual e mudança algorítmica permanecem
  `Unknown-not-persisted` como explicações adicionais, não como causas negadas.

## Limite do regime Tekt

O manifesto aponta entradas efêmeras em `/tmp` e um contrato em
`/root/p1232_contract`, mas não preserva contrato, recibos, ataques, selo,
mutation score ou referências encadeadas pelo hash do manifesto. A declaração
de capacidades e o hash das entradas não provam isolamento. Consequentemente,
P1232 é uma reprodução diagnóstica executada sem atestação; não é um protocolo
Tekt completo nem um certificado de refinamento.

## Decisão

P1229 continua reaberto. A evidência não autoriza alteração de produção nem
promoção de combinação. P1233 deve derivar novos budgets sobre um corpus comum,
preservar explicitamente a semântica de stops coincidentes e separar tolerância
de quantização de aproximação SVG.

Regime: reprodução diagnóstica; nenhuma implementação candidata foi produzida.
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO** porque o filesystem foi
compartilhado e a cadeia causal completa não foi persistida. Os resultados
autorizam reabrir P1229 e redesenhar P1233, mas não fechar causalidade exclusiva
nem equivalência funcional.
