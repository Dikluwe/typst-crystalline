# P1220 — primeira readjudicação de `svg-morphology`

## Veredito

O `DIFF` histórico de `P1138-X-001` era um **defeito de poder
discriminatório do harness**, não um gap produtivo provado. Após resolver IDs,
referências, transforms e paths em operações de pintura, `plain.typ` é
`Preserved`: canvas dentro de `1e-9pt`, mesma ordem/fundo/outlines/posições e
render comum com zero canais diferentes.

Nenhum código L0/L1–L4 foi alterado, pois imitar path versus rect, hashes de
ID, grupos neutros ou whitespace seria perseguir mecânica proibida pela
ADR-0107. O cluster continua `PARTIAL`: a fixture de forma permanece
deliberadamente `Unknown` até o modelo cobrir shapes.

## Medição antes da decisão

Baseline HEAD `3f0a2638fffd8cddc0fde6e83058f69dedc7d838`, working tree não
commitado, congelado em `2026-08-26T16:27:55-03:00`. Vanilla ratificado
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O runner antigo retornou DIFF por cinco classes mecânicas: precisão do canvas,
path/rect de fundo, grupo neutro dividido, spelling de IDs e whitespace do
path. O modelo focal compôs matrices, resolveu cada `<use>` ao outline e
preservou ordem de pintura. Deltas do canvas: `1.812e-10pt` e `4.724e-10pt`.
Ambos renderizados pelo mesmo `rsvg-convert` produziram PNG `794x1123`
byte-idêntico, SHA-256
`1737b8dcfd7e98c307c7015c21b78eccfe3aafc3737d3f27e6aa0918592dbbc6`.

## Harness e ataques

Foi adicionado `lab/parity/matrix/svg_morphology.py`. O vocabulário focal
aceita canvas, fundo branco, grupos com `matrix`, `<use>` e símbolos com um
path; qualquer pintura não modelada retorna `Unknown`. O runner mapeia
`Unknown` para `PARTIAL`, nunca MATCH. A suíte tem 23 testes e os 16 ataques
foram rejeitados (`mutation_score = 1.0`).

O manifesto agora espera MATCH para `P1138-X-001`, limitado explicitamente ao
resolved-paint model focal. Isso não declara paridade SVG geral.

## DSM e continuação

A correspondência `svg-plain-page-paint` registra consolidação parcial entre
page/text vanilla e o exportador cristalino. `svg-morphology` permanece
`PARTIAL`; o próximo passo deve estender o modelo a shapes (`path`, `rect`,
fill/stroke) e então localizar a primeira diferença real, se houver.

Execução Tekt completa em sessão única: **EXECUTADO SEM ATESTAÇÃO DE
ISOLAMENTO**.

## Gates finais

Passaram 23 testes do harness, duas execuções de `P1138-X-001`, controles
P1219, `cargo test --workspace`, builds workspace/release,
`crystalline-lint .`, fmt, Python bytecode e `git diff --check`. A lente DSM
foi estável nas duas rodadas, SHA-256
`87e49dbfcd8f00b3f863bf58bcbffbaecd0346a6bfcb55f3983cd520101ce89f`.
O certificado reproduzível está em `p1220-tekt-certificado.tsv`.
