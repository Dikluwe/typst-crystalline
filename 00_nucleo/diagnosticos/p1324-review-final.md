# P1324 — veredito independente final

**Aprovado no recorte do L0; A/B V3 bruto não integralmente GREEN.**
Revisor `/root/p1324_review`, 2026-09-09, regime A/B executado sem atestação
técnica de isolamento e sem selo de refinamento. Root escreveu intenção/L0
e implementação; B escreveu testes e D julgou sem corrigir julgados.
Nenhuma conclusão aqui prova paridade geral de funções ou do compilador.

## Proveniência e transformação julgada

Working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. Manifesto SHA-256
`414bdf11ea6cc34565e8db0dbc4fb7966685d62abbd41195f40a3ae1ea985a2d`.
Baseline P1323 SHA-256 `f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`;
candidato `/tmp/p1324-target.x5jDkw/release/typst` SHA-256
`c5c9aa39c8b7053a19f53bf9f37c8d3731a4081683a51cf8ca5e9e9c0197d2f7`;
vanilla upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Os recibos independentes `p1324-review-preflight.json`,
`p1324-review-prepatch-audit.json`, `p1324-review-candidate-audit.json` e
`p1324-review-final-audit.json` conservam bytes iniciais, hashes e diff/stat.
O último foi medido em 2026-09-09T00:29:28.317Z. Comparação dos inventários
baseline versus `p1324-final-diff-check.json` identifica somente o par
`compiler/eval/bindings/field_access` como alterado. Os seis arquivos dirty
herdados em loading/call_dispatch/wiring e todos os demais arquivos do
inventário produtivo permaneceram iguais.

A causa cabe no owner: projeção nominal das variantes nativas, span AST do
field e mensagem após lookup ausente. Sucesso, None, With e categorias não
nativas são preservados; não há nova API, fase, default ou Núcleo. O L0
precedeu código e sucedeu expressamente a proteção P1311 de Some ausente.
Pareceres ex-ante e do candidato estão em `p1324-review-l0.md` e
`p1324-review-candidate.md`; nenhum achado funcional no delta permanece aberto.

## Gates conferidos

O RED válido R2 executou cinco testes independentes, três controles passaram
e duas asserções de mensagem falharam, exit 101. Source/inventário/diff ficaram
estáveis. O primeiro RED, exit 0 com zero testes, é tentativa inválida retida.
GREEN executou dez testes: cinco novos mais cinco P1311, todos passaram.
O snippet bruto está congelado e seu resultado rustfmt foi comparado
literalmente ao bloco integrado; não houve revisão de expectativa após C.

Sobre o source formatado: build e fmt exit 0; workspace passou 6.672 testes,
zero falhas, três ignorados em 17 grupos, com diff/inventário estáveis
(`p1324-workspace-tests.json`, 00:23:13.619472Z–00:26:01.242214Z).
Lint completo: zero erros, 240 warnings e 1.138 infos, sem alegar zero avisos.
V5/V15/V26 e diff-check finais exit 0.

Linhagem calculada independentemente pelo algoritmo canônico: corpo L0
SHA-256 `4b85d3606fc3dc9e1390449bb856b0fcde5959b644294ba6f728c04b33f807c3`
intacto; header efetivo `740a39f5`; recíproco `c4567498`; Núcleo/pin efetivo
`5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24` íntegro.
O linter sozinho não detectou o recíproco desatualizado; a correção da única
linha metadata ocorreu após funcionais, sem mudar norma, source ou binário.
Essa limitação da ferramenta não foi confundida com garantia bilateral.

## Resultado A/B e julgamento das fronteiras

`p1324-review-ab-final-audit.json` recompõe independentemente todos os canais
raw de `p1324-ab-candidate-v3.json`: 600 processos = 50 casos × quatro perfis
× três ordens; 588 Preserved, 12 Violated, zero Unknown obrigatório e
400 verificações de estabilidade aprovadas. Os 13 artefatos pinados ficaram
íntegros. Estabilidade em três ordens foi medida em C; baseline/oracle são
referências pinadas, sem alegar que baseline foi repetido nas três ordens.

Também recomputei: 204 vetores mudaram baseline→C e todos os 204 coincidem
com vanilla; 396 ficaram iguais ao baseline. Esses números são execuções,
não 204 casos independentes. V3 não pode ser anunciada como 600/600 GREEN.

As 12 falhas são o único caso `json-callee-before-argument`, em quatro perfis
e três ordens. Baseline e C emitem a mesma panic do argumento; vanilla
diagnostica field ausente antes. A fonte causal intacta está em
`01_core/src/compiler/eval/call_dispatch.rs:1331–1348`: tentativa de método
de coleção avalia argumentos antes de chegar ao callee genérico em `:1585`.
O L0 P1324 exige literalmente manter ordem de avaliação. O caso independente
exigiu ordem vanilla e ultrapassou essa proteção; consertá-lo no produto
alteraria outro owner protegido e ampliaria o escopo.

Julgo válido o suplemento B `p1324-ab-order-preservation-supplement.json`,
SHA-256 `042e006c4e04ba46ba645f33d3de2d4550eb8f57fec72c90aae91360b5f06982`:
reconferi suas 12 linhas contra baseline V2 e C V3, todos os canais completos
coincidem. É evidência de preservação exigida pelo L0, não paridade, e foi
produzida após observação de C, sem novos processos. Não altera o resultado
V3 nem seus casos/freeze/comparador. A dívida de ordem em call_dispatch
continua aberta para decisão/owner próprios.

V1 falhou por opção CLI não suportada; V2 corrigiu o adaptador. V3 conservou
dois literais `.body` como controles Content e acrescentou positivos de
chamada, sob parecer independente baseado em baseline/oracle. V3 ocorreu
após C existir, mas antes de o autor observar C. Seu timestamp efetivo é
00:25:27Z, conforme correção explícita retida; não usar o campo `at` incorreto
00:23:56Z como prova. O suplemento de ordem ocorreu após observar C. Essas
revisões impedem alegar cadeia inteiramente congelada antes da implementação.

O veredito aprova a correção nominal/field-span delimitada e sua preservação
sob o L0 vigente, não a obrigação independente excessiva nem paridade de
Content, ordem de chamadas, snapshots em layout, warnings gerais ou demais
dívidas P1322. Nenhum commit, staging ou push foi efetuado pelo revisor.
