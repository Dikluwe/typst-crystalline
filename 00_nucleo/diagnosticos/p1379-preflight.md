# P1379 — preflight de materialização

Protocolo completo executado **sem atestação de isolamento**: uma autoridade
compartilha contrato, testes, candidato, ataques e veredito. Hashes provam
identidade e ordem, não independência.

A fonte vanila cria uma região finita repetida para cada coluna e distribui
frações dentro de cada flow regional. O bloqueio P1376 no cristalino distingue
apenas colunas contínuas por `page_columns.is_some()`; segmentos explícitos
ficam sem parcela e os Blocks caem para altura intrínseca.

Foi rejeitada a mutação cômoda de liberar todo `column_mode`: o mesmo sinal é
usado temporariamente pelo flush de footnotes e abriria comportamento fora do
contrato. A solução congelada é uma entrada interna contextual de Sequence,
sem sentinela persistente no Layouter.

Ordem congelada: L0/contrato/oráculos/mutantes → RED focal → entrada contextual
→ GREEN focal → matriz bilateral → resselo/lint. Budget: duas revisões focais;
duas falhas consecutivas da mesma causa reabrem o desenho.

Calibração v2: a primeira candidata calculou corretamente `679,257480...pt`
de parcela entre cursor `70,866141...pt` e fundo `771,023622...pt`, mas a
recomposição excedeu o fundo por arredondamento e paginou a cauda. O contrato
passa a exigir uma guarda proporcional à magnitude; o pré-selo v1 fica
invalidado antes da segunda candidata.

Reabertura arquitetural v3: a matriz completa rejeitou a segunda candidata
por duas testemunhas distintas dos testes unitários. Prefixo `Text` era
submedido em `1,188pt`; page columns entregava o body inteiro sob um `Styled`
exterior, ocultando `colbreak` do splitter. Foram adicionados controles E2E
congelados para ambos. O pré-selo v2 fica invalidado; a nova hipótese trata as
duas fronteiras explicitamente, sem nova afinação do mesmo predicado.

Reabertura arquitetural v4: a testemunha E2E de prefixo continuou vermelha e
o trace focal separou as grandezas. O preflight reservava `13,2pt`; ao fechar
`Lead`, o cursor ia do topo regional `10pt` ao início do Block em `32,55pt`,
isto é, `22,55pt`. A hipótese v3 confundia altura de linha com deslocamento
completo desde o topo de entrada. O contrato passa a espelhar os dois termos
já proprietários de `ensure_initial_baseline`/`flush_line`: top-edge inicial
quando pendente e avanço `top-edge + descent + leading`. O pré-selo v3 fica
invalidado antes desta alteração de implementação.

Reabertura arquitetural v5: a matriz bilateral mostrou que v4 resolveu a
paginação mas não a morfologia completa. `After=220,166pt` passou a coincidir,
porém o corpo `One` permaneceu em `21,792pt` contra `27,842pt` no vanila. A
diferença exacta `6,05pt` é `par.spacing - par.leading` em fonte 11pt, a mesma
fronteira já medida e materializada por P1377 para o contentor Columns no flow.
O contrato agora exige essa fronteira dentro do segmento quando texto direto
precede o primeiro Block fracionário, e exige descontá-la da parcela. O teste
E2E passa a verificar também a coordenada do corpo; o pré-selo v4 fica
invalidado antes da implementação.
