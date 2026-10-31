# JPEG grayscale com gradiente — custo residual — 2026-10-27

A guarda privada de admissão JPEG agora verifica byte stuffing em blocos de
16 pares adjacentes em x86_64 com SSE2 habilitado. A decodificação completa de
entropia/IDCT, faixas nativas de oito linhas, fallback Luma e decoder legado
permanecem. Outros alvos mantêm o algoritmo escalar recebido. Não há destino
da imagem inteira nem nova dependência, API, modo, fase ou receita de build.

## Investigação, negativos e decisão

A busca solicitada no Bateia foi executada com o código local atual, sem
modificá-lo. O pin zune `2f9086b07cdf5a36621ce18f79d310f99b751de9` completo
ultrapassou o orçamento de leitura de 16 MiB por arquivo/64 MiB total. Uma
segunda consulta usou somente Cargo.toml e src/**/*.rs de zune-jpeg, com mapa
de hashes. Resultado: **INCONCLUSIVE / CATALOG_GAP**. Isso limita o catálogo;
não prova ausência de soluções nem recomenda esta otimização. Proveniência,
comandos, dossier e resultados em `bateia-*`.

Experimentos de perfil foram controles CLI, sem edição de manifestos: infra
opt2, infra opt3 e infra opt2 com zune opt3. Todos foram rejeitados. No ensaio
mesma-fonte, opt2 trouxe gradiente 103,35 → 99,73 ms, mas aumentou o ELF de
51.005.312 para 52.830.664 bytes. No replay R6, sua razão geométrica de RSS
contra R5 foi 0,971941, acima do teto 0,97; a janela basal também variou
3,48%, acima de 3%. Não é aprovação do gate R6. O recibo conserva os dois
problemas e os demais resultados. Nenhum limite foi relaxado e a receita
recebida foi restaurada antes da candidata final. `matrix-results.json`,
`experimental-resources/receipt.json` e `recipe-rejections.json` registram os
estados, amostras e decisões; os recibos anteriores permanecem intactos.

A medição isolada da guarda escalar encontrou 2,135 ms no gradiente contra
0,135 ms com SSE2 (31 pares, 20 iterações por amostra). Isso mede somente
framing, não a validação nem o CLI completo. Foi a evidência para experimentar
a mudança privada em scan.rs, especificada primeiro no L0 e classificada como
mecânica pelo ADR-0127. Ownership/Núcleos foram auditados e somente seu owner
foi resselado. A decisão final depende da campanha CLI abaixo.

## Implementação e RED→GREEN

A guarda preserva o predicado recebido: após SOS, só FF00 no corpo e FFD9
terminal, sem bytes extras. Não valida Huffman e não substitui o decoder.
O helper privado exige EOI, compara current==FF AND next!=00 em blocos de
16 bytes e verifica no máximo 15 bytes finais escalarmente. O cfg exige
x86_64/SSE2; as duas cargas não alinhadas têm limites explícitos e comentário
de segurança. Nenhum ponteiro escapa da entrada. Fallback escalar conserva
o algoritmo anterior nos demais alvos.

O teste instrumenta comparações escalares reais da guarda, sem instrumentação
produtiva: entrada com 8192 pares FF00 passou de **8193 comparações (RED)**
para **0 (GREEN)**, com admissão preservada. Os outros dois testes comparam
o resultado ao escalar recebido: todos os 65.536 pares na fronteira vetorial
e 4096 casos determinísticos variando comprimento, alinhamento e EOI.
Os **25 testes JPEG direcionados passaram**, incluindo alocação, faixas,
deduplicação, limites e falhas anteriores.

O replay produtivo final passou em **17922 comparações de aceitação
e diagnóstico exato**, sem diferenças: corpus anterior e os 20 fixtures atuais.
Esse corpus finito e os testes diferenciais não provam equivalência para todo
byte possível. A otimização não altera a política de rejeição de JPEG.

## Medições atuais

Quatro releases na mesma campanha: legacy = alteração 2; antes = faixas nativas
recebidas; depois = guarda SSE2 final; luma = anterior às faixas nativas.
O +16,4% histórico foi reavaliado, não tomado como resultado desta máquina/janela.
Foram **936 compilações CLI em 20 fontes**, com 24 ordens
alternadas, dois warmups por versão/caso, 61 amostras por versão no gradiente,
31 no uniforme/ruído, 15 no RGB grande, sete nos demais positivos e três nos
negativos. Sem builds/testes concorrentes. Tempo inclui todo o processo CLI;
RSS é pico em KiB. Medianas:

| Caso | Legacy, ms | Antes → depois, ms | Tempo vs antes | Tempo vs legacy | RSS antes → depois, KiB |
|---|---:|---:|---:|---:|---:|
| gray-gradient | 86.18 | 100.19 → 97.53 | -2.6% | +13.2% | 28888 → 28856 |
| gray-horizontal | 42.03 | 35.75 → 35.68 | -0.2% | -15.1% | 18076 → 18000 |
| gray-vertical | 41.88 | 34.93 → 34.98 | +0.1% | -16.5% | 17792 → 17860 |
| gray-large | 41.16 | 32.76 → 32.19 | -1.7% | -21.8% | 17420 → 17492 |
| gray-noise | 57.16 | 46.75 → 45.99 | -1.6% | -19.6% | 23844 → 23768 |
| rgb-large | 124.41 | 47.35 → 46.57 | -1.7% | -62.6% | 17700 → 17612 |
| shared-rgb-16 | 225.14 | 148.16 → 148.37 | +0.1% | -34.1% | 25040 → 24564 |
| shared-rgb-pages | 224.41 | 149.47 → 151.28 | +1.2% | -32.6% | 24700 → 24620 |
| distinct-rgb-4 | 537.48 | 252.39 → 250.60 | -0.7% | -53.4% | 28092 → 28044 |
| gray-progressive | 100.40 | 101.15 → 100.96 | -0.2% | +0.6% | 27600 → 27384 |
| progressive | 66.76 | 67.02 → 66.54 | -0.7% | -0.3% | 60376 → 60432 |
| cmyk | 11.40 | 11.49 → 11.56 | +0.6% | +1.5% | 18588 → 18516 |
| control | 15.69 | 15.66 → 15.82 | +1.0% | +0.8% | 18088 → 17944 |

Gradiente: **-2.6%** no tempo contra o estado recebido.
A diferença frente à alteração 2 foi de **+16.3% para +13.2%**
na mesma janela. RSS final **28856 KiB**,
-0.1% contra o recebido e -45.9% contra alteração 2.
Permanece custo residual; não se declara eliminação da regressão nem ganho
universal. Os controles negativos e variações permanecem expostos na tabela.

Gates congelados antes do release: mediana do gradiente menor que recebido,
IC bootstrap pareado de 95% inteiramente abaixo de 1, preservação do ganho
anterior de pelo menos 5% contra luma e RSS até 103% do recebido nos nove
casos grandes. Todos passaram. Razão das medianas: 0.973502;
mediana das razões pareadas: 0.974673;
IC95% **[0.969795, 0.978128]**, 10.000 resamples, seed 20261027.
Razão final/luma: 0.894347.
Todos os tempos, intervalos, fixtures, hashes e RSS estão em
`paired-results.json`/`resource-gates.json`. São entradas sintéticas e uma
máquina. Os fixtures foram copiados sem alteração da campanha anterior.

ELF: **51,005,312 → 51,005,664 bytes**
(+352, +0.001%).

## Controle final R6

Após os 12 gates, sem builds concorrentes, o release final repetiu as sete
cargas R6, três rodadas × três processos por papel/carga e dois warmups.
Os cinco papéis foram R5 recebido, corrente, faixas nativas recebidas, oráculo
fixado e z+shared validado. Todos os critérios fixos passaram: executável
normalizado 0.860247 de R5 (teto 0,95), abaixo do oráculo;
RSS geométrico 0.953157 de R5 (teto 0,97); cada carga
até 1,03; variação das medianas por rodada até 3%; matemática abaixo do
controle z+shared. Conteúdo textual e geometria ≤0,001 pt foram preservados.
`final-resources/receipt.json` conserva todas as amostras, fontes congeladas,
hashes, tempos e critérios. Esse resultado não apaga o ensaio opt2 rejeitado.

## Validação e preservação

- Status e stderr iguais nos quatro releases em todas as amostras.
- qpdf aprovou os 16 PDFs positivos; todas as páginas foram rasterizadas.
  **249 comparações** de
  raster, texto e JPEG extraído idênticas. Payloads JPEG comprimidos originais
  e páginas verificados em `payload-and-pages.json`.
- **9972 testes Rust aprovados, 7 ignorados; 42 Python aprovados**.
  Todos os 12 gates passaram: build, testes, doctests, all-targets, Clippy,
  fmt, rustdoc, arquitetura e linhagem. Linter completo sem warning/error.
- Só `03_infra/src/export/images/scan.rs` e seu L0 mudaram nos inputs recebidos.
  Manifestos, lockfile, receita, fixture progressiva e todas as alterações
  anteriores foram preservados. Índice recebe uma entrada aditiva.
  Não houve commit/push nem atualização do oráculo.

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitada. `before.json` contém
UTC, diffstat exato e 2165 hashes; `after-source-state.json`
congela o código final. Fechamento `2026-10-27T04:12:01.865018+00:00`. Build/validação: um job,
cgroup MemoryMax 12 GiB, MemoryHigh 8 GiB, zero swap; autorização persistente
registrada. A pasta conserva a data local inicial 2026-10-26; este fechamento
é de 27 de outubro em America/Sao_Paulo, recibos em UTC.

Oráculo recebido preservado: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`,
tree `483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256
`22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`.
Esta campanha não recertifica comparações antigas com vanilla.

legacy: `/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/jpeg-scanline-2026-10-26/typst-before`, SHA-256 `e2fdcf17339d5cb16e5f62da6a38e04797758a7d56159e1a57cbdcdeb9a32711`.

before: `/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/jpeg-gradient-residual-2026-10-26/typst-before`, SHA-256 `a424337b066631ee0da841687f0c9804739531c74b87954e13502dad97d888ae`.

after: `/repos/Antigravity/typst-crystalline/lab/target/product/release/typst`, SHA-256 `2941a5bae502951c2028d3e6a218349e45a1a89512f43124fad854cb1dc47bce`.

luma: `/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/jpeg-gradient-2026-10-26/typst-before`, SHA-256 `74c8a978ee5c326c217951f9fab36e35d7b601fbc496524f9fb302abe3f28cb8`.

Evidências: `lab/results/construction/diagnostics/jpeg-gradient-residual-2026-10-26/`.
