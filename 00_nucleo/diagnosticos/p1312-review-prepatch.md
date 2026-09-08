# P1312 — GO prépatch

GO para o recorte read-only do L0 P1312, após revisão independente do
congelamento às `2026-09-08T01:39:22.046026+00:00` e do RED unitário.
O regime permanece A/B sem atestação de isolamento técnico.

Entradas recalculadas no namespace do host, por leitura dos artefatos:

- Freeze `p1312-ab-freeze.json`: SHA-256
  `440d7e744067dffab23371700bf421bf6dd2830c7d8a60cf26ccc3b6ea54f725`.
- Medição `p1312-ab-baseline-sealed.json`: SHA-256
  `e6dcbc57b13bada6ab8aa319dc2928ba19823c8142b920d8bf18ab99664f83d9`.
- L0 normativo, removendo somente a linha `Hash do Código`:
  `1abef2ea2a2a18076534a43d8a1e74197b14fab40afd60e8649acd0d04965807`.
- RED `p1312-unit-red.json`: SHA-256
  `0f17547837ac174b899a073ac97101876fc81141cfd6d254549ab7f4fbe6c8e5`.

Recalculei todos os pins do freeze, incluindo scripts/fixtures/medição/baseline.
Recompus 280 expectativas a partir das 560 observações finais: 70 casos nos
quatro perfis, 29 alvos de paridade, três casos Symbol normativos e 38 controles.
Para targets/control a origem do oráculo é respectivamente vanilla/baseline;
Symbol conserva o span e trace vanilla medido e substitui somente a primeira
linha pelo texto normativo literal. Não há política alterada após candidato.

Amostras distinguem Bytes/read versus Bytes/CSV, tipo longo, named antes do
positional, Args/With/sink/spread e detached por map. A revisão sugeriu antes
do freeze dois casos concorrentes: unknown named precede cast; cast precede
encoding inválido. Ambos entraram na medição final. Controles Str e Path
simples passam; o path importado preserva erro sandbox cristalino já existente,
explicitamente documentado como dívida, não sucesso de paridade.

O RED é funcional: exit 101, três falhas assertivas (texto/nome de tipo/origem)
e um controle de preservação passando. Não usei como RED as tentativas anteriores
de CLI/fixture/sintaxe. Ao remover somente o bloco de testes P1312 e restaurar
o header de linhagem prévio, recompus exatamente loading.rs do baseline:
`a4c574a7fcf28ee579f079a0473d56a81ebf8692aab91fd56e68bd957c407ca1`.
Logo ainda não havia mudança funcional no instante deste GO.

Proveniência de números: HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`,
working tree não commitado; estado e diff/stat no baseline, RED e medição
referenciados. Histórico P1310/P1311 preservado: read.wrong e read-loader-error
mudam deliberadamente apenas para os respectivos observáveis vanilla prépatch;
esses deltas não permitem reescrever oráculos antigos nem alegar preservação
literal integral dos replays. P1308 readwrongtype é delta esperado equivalente.

O GO não é veredito final. Ainda exige inspeção do candidato, GREEN, comparação
literal normal/repeat/reverse, replays com deltas delimitados, gates, hashes
e escopo preservado. Apenas metadata de Hash do Código pode mudar no L0 congelado.
