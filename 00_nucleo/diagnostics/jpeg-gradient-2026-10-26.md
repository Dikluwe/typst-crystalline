# JPEG grayscale com gradiente — faixas nativas — 2026-10-26

JPEG grayscale baseline elegível com uma componente e sampling 1×1 agora é
reconstruído em faixas nativas de oito linhas. A validação não faz a conversão
intermediária para faixas Luma de nove linhas. Ela continua decodificando
entropia e IDCT completos, contando todas as linhas e exigindo `Complete`.
Não há buffer da imagem inteira. O caminho Luma anterior permanece como
fallback antes do decoder legado, preservando inclusive as formas de sampling
excluídas. RGB, outros formatos, limites, guarda de admissão, deduplicação por
Arc e JPEG comprimido exportado mantêm os contratos anteriores.

## Investigação e decisão

No pin zune `2f9086b07cdf5a36621ce18f79d310f99b751de9`,
`decoder.rs:963` consome faixas convertidas; `mcu.rs:1633` processa/converte as
linhas reconstruídas. `decoder.rs:1242` oferece faixas de plano nativo pelo
mesmo caminho de entropia/IDCT, com checkpoints de linha em baseline single-SOS.
A API nativa não significa validação apenas de cabeçalho nem entropia sem IDCT.

Os probes isolados foram inconclusivos em parte: IDCT escalar foi mais lenta;
otimizar somente zune-jpeg não trouxe ganho; perfis opt2/3 do consumidor e busca
com memchr tiveram resultados mistos. Código genérico pode usar o perfil da
crate que o instancia (Cargo 1.92, referência local profiles.html:501–522).
Nenhuma dessas alterações de perfil/dependência foi adotada. As fontes,
manifestações, binários, UTC e negativos estão nos recibos `probe-*`.

O primeiro ensaio produtivo de faixas nativas reduziu o gradiente cerca de 7,8%
em 15 pares e preservou RSS, mas continha um guard composto sinalizado por V17.
Essa etapa está congelada em `attempt-before-style/`, sem certificado final.
O guard foi desdobrado, o owner resselado e o release foi recompilado e medido
novamente. Não houve supressão de warning nem alteração dos critérios.

L0 atualizado antes do código, fluxo contínuo ADR-0127, ownership/Núcleos
auditados, apenas o proprietário de `scan.rs` resselado. É uma mudança privada
de mecânica; aceitação, diagnósticos e conteúdo visível são os observáveis.
Não há API pública, fase, modo, dependência ou receita nova.

## RED→GREEN e compatibilidade

O allocator real em subprocesso com entrada L8 1009×73 passou de
`[RGB9, Luma convertida9, Luma nativa8, destino cinza completo] = [0,1,0,0]`
para **`[0,0,1,0]`**. Isso conserva as garantias anteriores de não expandir
cinza para RGB e não alocar um destino completo; não substitui a medição de RSS.
O helper limita o número de pulls, confere a contagem de cada faixa e aceita
Complete somente com zero linhas restantes. Falha/incompletude/layout não
suportado retorna ao caminho Luma anterior, com estado nativo já descartado.

Três novos testes cobrem gradiente não trivial, sampling incomum e marcadores
stuffed/finais em fronteiras. A primeira expectativa sobre cortes de entropia
estava errada: o legado aceitou os três cortes seguidos por EOI (2404, 4758 e
4803 bytes de um fixture de 4808 bytes). O L0 foi corrigido para comparar os
resultados medidos; nenhuma nova política de rejeição foi introduzida.
As tentativas RED iniciais permanecem nos logs. O RED final falha apenas na
alocação convertida; GREEN final passa nos 22 testes JPEG direcionados.

O validador produtivo final repetiu **17902 comparações de aceitação
e mensagens exatas sem divergências**: os 17.882 casos da etapa anterior e
20 fixtures desta campanha (incluindo repetições). O corpus finito não prova
equivalência para qualquer byte possível. RGB sem buffer completo, dedup,
primeiro erro, novas chamadas e limites passaram também na suíte completa.

## Medições atuais

Três releases no mesmo ensaio: `legacy` = alteração 2, `antes` = release Luma
recebido, `depois` = faixas nativas finais. O antigo +25,3% é histórico; os
números abaixo foram medidos novamente. Foram **612 compilações
CLI em 20 fontes**, com seis ordens alternadas, dois warmups por versão/caso,
31 amostras por versão nos grays originais (gradiente, uniforme e ruído),
15 no RGB grande, sete nos demais positivos (incluindo os novos gradientes
horizontal/vertical) e três nos negativos. Sem builds/testes concorrentes.
Tempo inclui o processo CLI completo; RSS é pico em KiB. Medianas:

| Caso | Legacy, ms | Antes → depois, ms | Tempo vs antes | Tempo vs legacy | RSS antes → depois, KiB |
|---|---:|---:|---:|---:|---:|
| gray-gradient | 86.94 | 110.61 → 101.24 | -8.5% | +16.4% | 28864 → 28884 |
| gray-horizontal | 42.88 | 42.76 → 36.08 | -15.6% | -15.9% | 17980 → 18008 |
| gray-vertical | 42.14 | 41.13 → 35.78 | -13.0% | -15.1% | 17808 → 17784 |
| gray-large | 41.26 | 39.30 → 33.04 | -15.9% | -19.9% | 17440 → 17500 |
| gray-noise | 57.56 | 49.15 → 46.54 | -5.3% | -19.1% | 23824 → 23892 |
| rgb-large | 127.38 | 48.41 → 47.83 | -1.2% | -62.4% | 17688 → 17776 |
| shared-rgb-16 | 228.60 | 156.36 → 152.05 | -2.8% | -33.5% | 24660 → 24744 |
| shared-rgb-pages | 227.77 | 156.21 → 152.23 | -2.6% | -33.2% | 24732 → 24744 |
| distinct-rgb-4 | 549.67 | 256.80 → 262.21 | +2.1% | -52.3% | 28208 → 28004 |
| gray-progressive | 102.34 | 103.96 → 102.02 | -1.9% | -0.3% | 27532 → 27548 |
| progressive | 67.17 | 67.29 → 67.29 | -0.0% | +0.2% | 60468 → 60356 |
| cmyk | 11.12 | 11.08 → 11.05 | -0.2% | -0.7% | 18640 → 18344 |
| control | 15.65 | 15.23 → 15.26 | +0.2% | -2.5% | 18088 → 18020 |

No gradiente original, tempo **-8.5%** contra o recebido.
A diferença frente à alteração 2 foi de **+27.2% para +16.4%**
no mesmo ensaio. RSS final **28884 KiB**,
+0.1% contra o recebido e -45.8% contra a alteração 2.
O resultado reduz o custo medido; não transforma a observação histórica em
certificado de ganho universal. Valores residuais/negativos permanecem na tabela.

Os gates de recursos fixados no L0 antes do release passaram: gradiente pelo
menos 5% mais rápido que recebido; RSS de todos os nove cenários grandes no
máximo 103% do recebido. `resource-gates.json` vincula esses limites ao SHA do
release final. Os controles adicionais horizontal/vertical são 6000×4000 L8;
ruído é 2048×1536, com progressivo correspondente. Receitas/hashes, todas as
amostras, min/max e ambiente estão em `fixture-receipt.json` e
`paired-results.json`. São fixtures sintéticas e uma máquina; não se promete
melhoria uniforme em qualquer imagem real.

Executável: 50,990,016 → 51,005,312 bytes
(+15,296, +0.030%).

## Validação e preservação

- Status e stderr iguais nas três versões em todas as amostras.
- qpdf aprovou os 16 PDFs positivos. Todas as páginas foram rasterizadas;
  166 comparações de raster,
  texto e JPEG extraído permaneceram idênticas. Bytes JPEG comprimidos originais
  e páginas são verificados em `payload-and-pages.json`.
- **9969 testes Rust aprovados, 7 ignorados; 42 Python aprovados**.
  Os 12 gates passaram, incluindo build, testes, doctests, all-targets,
  Clippy, fmt, rustdoc e arquitetura/linhagem. Linter completo sem warning/error.
- Só `03_infra/src/export/images/scan.rs` e seu L0 mudaram nos inputs recebidos.
  Alterações anteriores, manifests, lockfile, fixture progressiva e estudos
  anteriores foram preservados; índice recebe uma entrada aditiva. Sem commit/push.

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitada. UTC, lista exata de
alterações/diffstat e 2165 hashes em `before.json`;
materialização congelada em `after-source-state.json`; fechamento `2026-10-27T02:47:27.680450+00:00`.
Build/validação: um job, cgroup 12 GiB, limite alto 10 GiB, zero swap.
Data do título em America/Sao_Paulo; recibos mantêm UTC de 27 de outubro.
Não se atualizou o oráculo nem se recertificaram campanhas antigas de vanilla.

Legacy SHA-256 `e2fdcf17339d5cb16e5f62da6a38e04797758a7d56159e1a57cbdcdeb9a32711`.
Recebido SHA-256 `74c8a978ee5c326c217951f9fab36e35d7b601fbc496524f9fb302abe3f28cb8`.
Final SHA-256 `a424337b066631ee0da841687f0c9804739531c74b87954e13502dad97d888ae`.

Evidências: `lab/results/construction/diagnostics/jpeg-gradient-2026-10-26/`.
