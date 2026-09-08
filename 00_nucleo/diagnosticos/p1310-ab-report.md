# P1310 — evidência independente A/B

O candidato `/dev/shm/p1310-target.VeBP6Q/release/typst`, SHA-256
`7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146`, satisfaz
as 1036 células R2 em normal, repetição e ordem integralmente inversa. Não há
Unknown nem instabilidade. Isso é evidência do fragmento contratado, não
equivalência geral dos decoders nem selo do protocolo completo.

Executor: `/root/p1310_tests`, testador A/B. Foram lidos skill/referências,
CLAUDEs, passo P1310 explicitamente autorizado, L0 loading inteiro, fonte
vanilla e adapters históricos públicos. Não foram lidos o código candidato,
o diff Rust ou os testes locais do owner. As escritas deste executor ficaram
em novos `p1310-ab-*`, além das fixtures temporárias feitas via apply_patch.
O filesystem é compartilhado: executado sem atestação de isolamento técnico.

## Escopo e fontes dos resultados

São 259 expressões, nos perfis default, html, a11y-extras e html+a11y-extras:
195 casos com expectativa literal vanilla; cinco casos Symbol com efeito
normativo explícito; 59 controles de preservação do baseline. O total é
1036 células por ordem. A comparação conserva stdout/stderr integrais,
mensagem completa, hints, range primário e traces resolvidos ao texto ASCII.
Nenhuma normalização de mensagem, espaços ou caminhos foi aplicada.

Os cinco loaders cobrem integer, float, boolean, none, auto, dictionary,
array, function, content, length, angle, ratio, fraction, color, label,
type, version, duration, datetime, decimal, symbol, direction, alignment,
regex, state, counter, arguments e module. As rotas incluem alias, With,
Args, spread de array, sink, filter, map que mantém ou troca o valor, join,
multilinha, primeiro posicional e named anterior. Path/Str/Bytes válidos,
missing/named/excesso, read/csv e encoders têm controles baseline. Args
sintético sem occurrence requer a evidência interna do coordenador: a suíte
CLI não alega construir esse estado Rust.

Vanilla ratificado upstream `a51e02804`, binário `/usr/local/bin/typst`, SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline `/dev/shm/p1309-r2-target.R2ZoSj/release/typst`, SHA
`e54dcc9bb6a4c45cdc710066027ea255af6fb13f17b3715583ba4e9f6f3287c7`.
O baseline e o vanilla foram medidos bilateralmente antes do candidato; as
três ordens finais reexecutam o candidato contra essas expectativas, sem
alegar três novas execuções das referências. O recibo baseline fornecido
foi validado por SHA `3a19c3b842c5bdcc2d4e6df3ea8778a407a9843b2fc5ec183f3b07620c25d65f`.

Todas as medições guardam argv, cwd, UTC, custo, SHA dos executáveis, stdout
e stderr em base64 e seus hashes. HEAD durante as medições:
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado;
diff/stat e git status exatos estão em cada recibo. A verificação final de
estabilidade ocorreu em `2026-09-08T00:32:49.410080+00:00`.

## Duas correções de especificação da suíte, preservadas

A tentativa inicial encontrou Symbol: vanilla faz coerção Symbol→Str e
chega à leitura de arquivo; o cristalino já o rejeitava. O L0 documentou essa
fronteira antes do patch. R1 preservou a medição inicial e congelou a rejeição
cristalina com a nova mensagem e value_span. A expectativa Symbol usa o
diagnóstico vanilla da mesma expressão, conservando sua origem e substituindo
somente a mensagem pela obrigação L0. Não é igualdade com vanilla.

R1 também classificou incorretamente 15 casos named-prefix como paridade
vanilla. Isso conflitava com o L0 anterior, que preservava a ordem de
validação e excluía named do reparo. A revisão apontou o erro; a execução
R1 ficou preservada com 976 Preserved e 60 Violated. Nenhuma alteração do
produto foi feita para acomodar essa expectativa incorreta.

R2 mudou somente essas 15 políticas, em 60 células, para preservação. As
expectativas são copiadas integralmente das observações baseline R1
anteriores ao candidato; nenhuma saída candidata foi usada para escrever
um expected. **A classificação R2 foi escrita depois da existência e da
primeira execução do candidato. Não se alega congelamento integral R2
anterior ao patch.** O runner original e todos os artefatos R1 permanecem
intactos. O focal R2 passou 120/120 (60 afetadas e 60 controles), antes das
três execuções integrais. As duas revisões tratam causas distintas e
produziram ganho observável; não houve ciclo sem ganho nem Unknown promovido.

Com a política R2 correta, há 800 células RED no baseline: 780 de paridade
de rejeição e 20 do efeito normativo Symbol. As outras 236 são preservação.

## Recibos e limites

- `p1310-ab-frozen-r1.json`: congelamento pré-candidato,
  SHA `306257272b75bc2aaaa6dc8920f2da7c4f571f52dd434bef368af25c67d02515`.
- `p1310-ab-frozen-r2.json`: sucessor de política pós-candidato,
  SHA `50641e69dd555f5599ff94ec82c23df947bff237dca9168dbe71a6e3776d1b68`.
- `p1310-ab-normal.json`: resultado R1 original, inclusive 60 falhas.
- `p1310-ab-r2-{focal,normal,repeat,reverse}.json`: saídas integrais R2.
- `p1310-ab-r2-stability.json`: 1036 células estáveis nas três ordens,
  SHA `09aaaf3de40153dca198daf4168ff653a0efed93cf6f5d2cd750de248defe55b`.
- `p1310-ab-l0-pin.json`: L0 bruto pré-patch e pin normalizado, removendo
  exclusivamente a linha canônica `Hash do Código`. O hash sem essa linha
  é `446c56d88674caaf471a634d91b4ba105f992d4b4efd11e1425575d0d18c07a2`.

Os recibos principais registram 6336 processos de suíte: 2072 observações
de referência pré-candidato, 1036 da primeira tentativa R1, 120 focais R2
e 3108 finais R2. Tempos por fase estão nos JSON; essas contagens não incluem
sondas manuais de calibração nem ferramentas de leitura. Não houve mutações
de produto, score de mutação ou atestação técnica de independência.

O juízo final de escopo, linhagem, testes internos, build/lint e encerramento
é responsabilidade do revisor e coordenador; este relatório não os substitui.
