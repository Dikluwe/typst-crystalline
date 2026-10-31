# Redução do custo de validação JPEG grayscale — 2026-10-26

O caminho elegível por linhas agora usa saída **Luma somente para entradas L8**,
identificadas pelo decoder legado já construído para o preflight. RGB mantém
as opções anteriores. As duas opções são especializações privadas em tempo de
compilação. A decodificação continua até `Complete`, incluindo
entropia e IDCT; os limites, a guarda conservadora e o fallback permanecem.
Nenhuma dependência, API pública, fase, flag ou payload exportado mudou.

## Investigação, L0 e RED→GREEN

No pin `2f9086b07cdf5a36621ce18f79d310f99b751de9`, zune-core
`options/decoder.rs:720` define saída RGB por padrão. Em zune-jpeg
`worker.rs:113,144`, o decoder triplica o canal Luma antes de descartar a faixa.
O caminho Luma (`worker.rs:47`) conserva a decodificação e evita essa expansão.
O probe isolado alternou 35 amostras por modo após três warmups: mediana
35,393 ms em RGB e 28,012 ms em Luma. As primeiras amostras agrupadas oscilaram;
elas permanecem em `probe-gray.log`, sem sustentar a decisão. A medição alternada
com hashes/UTC/comando está em `probe-paired.json`.

A primeira candidata produtiva selecionava a opção dentro de um único helper
com booleano em execução. Melhorou o gray uniforme 12,0%, porém aumentou RGB
9,4%. Um replay separado de 31 pares confirmou cerca de 10% no RGB. Ela foi
descartada, com fonte/L0/binário/recibos preservados em `attempt-runtime/`.
A candidata seguinte especializa privadamente o helper com `const GRAYSCALE`,
permitindo opções constantes em cada variante. Possível efeito da seleção
dinâmica sobre otimização do compilador é uma inferência, não uma causa
comprovada por inspeção de assembly. A tabela abaixo mede a candidata final
novamente; os números da tentativa descartada não são usados como fechamento.

L0 proprietário `00_nucleo/prompts/infra/export/images/scan.md` atualizado antes
dos testes/código, após medição; ownership/Núcleos auditados, resselagem restrita
ao mesmo proprietário. Correção privada em fluxo contínuo ADR-0127.
O trabalho descartado é mecânica; aceitação, erros e conteúdo visível são os
observáveis preservados. A hipótese de ganho foi submetida ao release, não
encerrada apenas pelo probe.

O teste Linux executa o validador real em subprocesso, com allocator System
forwarding e input grayscale 1009×73. RED: contagens
`[faixa RGB, faixa Luma, destino cinza completo] = [1, 0, 0]`.
GREEN: **`[0, 1, 0]`**. O último tamanho distinto protege contra reintroduzir
o destino completo. Os testes de dimensões 1×1, 7×9, 8×8, 9×17 e 1009×73
preservam aceitação e erros legados nas caudas truncadas/adicionadas.
São três novos testes; a instrumentação existe somente no binário de teste.
Os testes anteriores de RGB sem buffer completo, deduplicação, primeiro erro,
nova chamada e limites também passaram na suíte completa.

O replay produtivo repetiu os 14.604 casos congelados anteriores, quatro
cópias RGB da campanha de performance, mais 3256 casos grayscale (3000
mutações com seed 20261028 e 256 valores de transform Adobe APP14) e 18
fixtures desta campanha de performance. Ao todo,
**17882 comparações de aceitação e mensagens exatas sem divergência**;
as fixtures repetidas são contadas como execuções, não como conteúdo único.
Receitas/hashes/UTC em `corpus-gray.json` e `compatibility.json`.
O corpus finito não prova equivalência para toda entrada possível.

## Release: comparação atual com duas referências

`legacy` é o release da alteração 2 (já sem buffer duplicado e com deduplicação).
`antes` é o release por linhas recebido, no qual se mediu a regressão de 7%.
`depois` é esta correção. As três versões rodaram novamente sobre as mesmas
fontes, em seis ordens alternadas, com dois warmups por versão/caso.
Foram **570 compilações medidas em 18 fontes**: 31 amostras por
versão nos três grays grandes, 15 no RGB grande, sete nos demais positivos e
três em cada negativo. Tempo é wall time do processo CLI completo; RSS é pico
em KiB. Medianas:

| Caso | Legacy, ms | Antes → depois, ms | Tempo vs antes | Tempo vs legacy | RSS antes → depois, KiB |
|---|---:|---:|---:|---:|---:|
| gray-large | 42.00 | 44.74 → 39.69 | -11.3% | -5.5% | 17464 → 17424 |
| gray-noise | 58.83 | 51.40 → 50.61 | -1.5% | -14.0% | 23880 → 23824 |
| gray-gradient | 90.51 | 120.58 → 113.37 | -6.0% | +25.3% | 28816 → 28840 |
| gray-progressive | 103.02 | 103.52 → 103.77 | +0.2% | +0.7% | 27396 → 27536 |
| rgb-large | 126.12 | 47.02 → 48.99 | +4.2% | -61.2% | 17684 → 17752 |
| shared-rgb-16 | 229.64 | 156.15 → 156.41 | +0.2% | -31.9% | 24668 → 24612 |
| distinct-rgb-4 | 538.98 | 255.32 → 259.90 | +1.8% | -51.8% | 28316 → 28128 |
| progressive | 67.83 | 67.32 → 66.85 | -0.7% | -1.4% | 60424 → 60616 |
| cmyk | 11.56 | 11.73 → 11.69 | -0.3% | +1.1% | 18488 → 18576 |
| control | 15.62 | 15.86 → 15.46 | -2.5% | -1.0% | 18220 → 18140 |

No grayscale uniforme 6000×4000, a variação de tempo foi
**-11.3% contra o release recebido** e
**-5.5% contra a alteração 2**, medidos nesta execução.
RSS final 17424 KiB:
-0.2% contra o recebido e
-72.2% contra a alteração 2.
Os valores atuais devem ser comparados entre si; os 7% anteriores pertencem
ao recibo histórico da campanha anterior.

A especialização reduziu o custo adicional no controle RGB frente à primeira
candidata, mas a medição final ainda registra +4,2% contra o release recebido
(cerca de 2 ms). O grayscale com gradiente melhorou 6,0% contra o recebido,
porém permanece 25,3% mais lento que a alteração 2. Esses limites ficam abertos;
a correção não representa vitória universal de latência. Os ganhos de memória
de grayscale e RGB permanecem na comparação atual.

`gray-noise` é 2048×1536 com ruído pseudoaleatório; `gray-gradient` é 6000×4000
com gradiente/várias faixas; `gray-progressive` usa o mesmo ruído em progressivo.
Progressivo, CMYK e demais formas excluídas conservam fallback. RGB não recebe
a seleção Luma. Variações dos controles são registradas como medições locais;
nenhuma diferença é automaticamente descartada como ruído.
Comandos, todas as amostras, faixas min/max, hashes de fontes/binários e
ambiente fixo estão em `paired-results.json`; receitas de imagem em
`fixture-receipt.json`. São entradas sintéticas e uma máquina, sem promessa
de ganho uniforme em qualquer documento ou foto real.

Executável: 50,989,568 → 50,990,016 bytes
(+448, +0.001%).

## Gates, saídas e preservação

- Status e stderr iguais nas três versões em todas as amostras.
- qpdf aprovou os 14 PDFs positivos; todas as páginas foram rasterizadas.
  154 comparações de arquivos
  de raster, texto e JPEG extraído foram idênticas contra o recebido nas duas
  outras versões. Payloads JPEG originais e páginas em `payload-and-pages.json`.
- **9966 testes Rust aprovados, 7 ignorados, 42 Python aprovados**.
  Os 12 gates passaram: build, testes, doctests, all-targets, Clippy, fmt,
  rustdoc e arquitetura/linhagem; linter completo sem warning/error.
- Apenas `scan.rs` e seu L0 mudaram nos inputs produtivos recebidos; todos os
  demais arquivos recebidos preservados, inclusive manifestos/lockfile, fixture
  progressiva e os estudos anteriores. Índice recebe uma entrada aditiva.
  Sem commit/push.

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**.
UTC, status, diffstat e 2165 hashes de inputs em `before.json`;
estado materializado em `after-source-state.json`; fechamento `2026-10-27T02:02:44.845868+00:00`.
Data do título em America/Sao_Paulo; recibos conservam UTC de 27 de outubro.
Build release e validação final com um job e cgroup de 12 GiB, limite alto de
10 GiB, zero swap. Validação final fora do sandbox explicitamente autorizada
pelo usuário; `user-authorization.json` registra o escopo e a resposta.
A revisão automática das duas tentativas de iniciar a validação pelo serviço
expirou antes de criar processos. A tentativa alternativa dentro do sandbox,
com `/tmp`, um job e `RLIMIT_AS` de 12 GiB por processo, passou em 11 gates,
mas falhou em 34 testes que exigem `/dev/shm` e quatro que usam sockets locais.
Ela não certificou a integração. `validation-sandbox/`, `validation-remainder/`
e `partial-completion.json` preservam essa etapa; o limite de endereçamento
por processo difere do teto agregado por cgroup usado no fechamento final.
Após a autorização, os 12 comandos originais foram executados no ambiente
adequado e passaram, inclusive os 38 testes antes impedidos. Os inputs
produtivos permaneceram congelados. Não se alteraram testes nem critérios para
contornar as restrições; `full-validation-resources.json` registra o serviço.
As antigas campanhas de paridade com vanilla não foram recertificadas.

Legacy SHA-256 `e2fdcf17339d5cb16e5f62da6a38e04797758a7d56159e1a57cbdcdeb9a32711`.
Antes SHA-256 `c6ff873c095d21fa21a4fbb397e9e4551370454c7301b92bb33020c60d86a417`.
Depois SHA-256 `74c8a978ee5c326c217951f9fab36e35d7b601fbc496524f9fb302abe3f28cb8`.

Evidências: `lab/results/construction/diagnostics/jpeg-grayscale-2026-10-26/`.
