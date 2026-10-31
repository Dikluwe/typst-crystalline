# JPEG por linhas com zune-jpeg — 2026-10-26

Implementada a alteração 3 como otimização conservadora: JPEG baseline de oito
bits, um ou três componentes, uma varredura intercalada e estrutura elegível é
decodificado integralmente por faixas descartadas, sem buffer de pixels da
imagem inteira. O fluxo só tem sucesso com `ScanlineStatus::Complete`.
Progressivo, CMYK, múltiplas varreduras, reinícios, trailers e demais formas fora
do subconjunto seguem a decodificação completa anterior. Falha ou incompletude
do zune também executa o decoder anterior; seu diagnóstico permanece soberano.

## Evidência antes da decisão

O experimento usa `zune-jpeg 0.5.16-rc3` e `zune-core 0.5.3`, ambos fixados no
commit `2f9086b07cdf5a36621ce18f79d310f99b751de9` de
<https://github.com/etemesi254/zune-image>. A API `scanline_output().finish()`
consome as faixas restantes; o código upstream conserva coeficientes da imagem
inteira em progressivo/multiscan. Portanto não se afirma memória constante
para todos os JPEGs. O pin é de uma versão candidata, não uma release estável.

A substituição irrestrita em modo estrito não preservou aceitação: o corpus
inicial de 624 mutações encontrou um progressivo corrompido aceito apenas pelo
zune; o corpus expandido de 6000 encontrou 119 aceitações divergentes. Uma
primeira guarda ainda deixou passar duas categorias DC inválidas em 7729
mutações de cabeçalhos. Esses resultados estão preservados em `compat-*.json`
e `compat-*.tsv`, com os arquivos de entrada e sementes congelados.

A decisão posterior foi restringir a otimização e manter fallback: validar
limites de segmentos, SOF0/SOS, quantizadores completos de oito bits não nulos,
categorias DC/AC conservadoras, uma única varredura e EOI no fim da entrada.
O preflight do decoder `image 0.24.9`, incluindo seus limites padrão e reserva
lógica de saída, permanece anterior ao fast path. A guarda não rejeita o
documento: apenas escolhe o decoder completo anterior quando não é elegível.
Isso preserva inclusive o limite lógico padrão, apesar da redução de RAM.

O replay na implementação produtiva comparou **aceitação e mensagens exatas**
em 14.364 casos, mais 240 casos de cauda/entropia: **14.604 sem divergências**.
Os JSONs e logs `product-corpus*` vinculam fonte, binário e contagens. O corpus
finito não prova equivalência para toda entrada possível. Formas excluídas são
uma limitação explícita da otimização; a remoção do fallback requer novo estudo.

## L0, RED→GREEN e preservação

L0 proprietário `00_nucleo/prompts/infra/export/images/scan.md` atualizado antes
do código e resselado após auditoria de ownership/Núcleos. ADR-0127 em fluxo
contínuo: mudança privada de mecânica, sem contrato público, flag, default de
produto ou mudança de fase. A compatibilidade observável é aceitação, erro e
conteúdo visível. Dependências adicionadas apenas em L3, com lockfile fixado.

O RED executou o decoder real em subprocesso isolado: uma alocação de tamanho
RGB completo quando se exigia zero. O GREEN encontrou **zero** em JPEG RGB
6000×4000, preservando também o teste anterior de no máximo uma alocação.
A primeira tentativa GREEN revelou DQT depois de SOF no encoder Rust: o L0 foi
corrigido antes da guarda produtiva para admitir quantizadores válidos em ambas
as ordens. `red.log`, `green.log`, `green-quantizer.log` e `green-zero-frame.log`
preservam essa sequência, incluindo a tentativa falha.

Três novos testes cobrem admissão conservadora, regressões progressivo/tabela
após scan e replay externo genérico. A fixture progressiva própria tem SHA-256
`5936023a53de7c410467d4f56596bfd338a650d20780a932a35ce6461dbf9de9`.
Os testes da alteração 2 agora contam chamadas reais por instrumentação TLS
exclusiva de teste, pois o caminho otimizado já não aloca RGB completo:
um Arc compartilhado valida uma vez; Arcs distintos e novas chamadas continuam
validando separadamente. A deduplicação produtiva local e os bytes exportados
foram preservados. Não há instrumentação no binário de produção.

## Ganhos medidos e custos

Baseline é o release da alteração 2, já com as alterações 1 e 2; não o binário
anterior a elas. Foram 178 compilações CLI alternadas em
15 fontes: sete amostras por versão nos positivos, três nos quatro negativos.
Wall time inclui o processo inteiro; RSS é pico em KiB. Medianas:

| Caso | Tempo antes → depois, ms | Variação tempo | RSS antes → depois, KiB | Variação RSS |
|---|---:|---:|---:|---:|
| rgb-large | 126.5 → 47.2 | -62.6% | 122100 → 17752 | -85.5% |
| gray-large | 41.7 → 44.6 | +7.0% | 62524 → 17296 | -72.3% |
| shared-rgb-16 | 228.8 → 154.8 | -32.4% | 128644 → 24728 | -80.8% |
| shared-rgb-pages | 229.0 → 155.3 | -32.2% | 128744 → 24800 | -80.7% |
| distinct-rgb-4 | 547.3 → 260.3 | -52.4% | 187416 → 28080 | -85.0% |
| progressive | 67.7 → 67.3 | -0.6% | 60232 → 60460 | +0.4% |
| cmyk | 10.8 → 11.1 | +3.7% | 18372 → 18576 | +1.1% |
| control | 15.2 → 15.3 | +0.6% | 17968 → 18024 | +0.3% |

RGB e grayscale grandes são 6000×4000. `shared-rgb-16` reutiliza uma imagem
16 vezes; `shared-rgb-pages` distribui essas ocorrências em quatro páginas;
`distinct-rgb-4` usa quatro arquivos com Arcs distintos. Progressivo e CMYK
permanecem no fallback. O controle é PNG.

No RGB grande o RSS caiu 85,5% e o tempo 62,6%. No grayscale grande o RSS caiu
72,3%, porém o tempo subiu 7,0% (cerca de 2,9 ms). O executável cresceu de
50,751,560 para 50,989,568 bytes: **+238,008 bytes,
+0.47%**. Há ganho expressivo de memória no
subconjunto medido, com custo de tamanho e uma regressão local de latência.
Fixtures sintéticas, uma máquina e poucas amostras não sustentam promessa de
melhoria universal em fotografias/documentos reais. Pequenas diferenças dos
controles e fallbacks são variação local; não significam otimização desses casos.

## Validação completa e proveniência

- Status e stderr iguais em todas as amostras pareadas, inclusive corrompidos,
  truncado sem EOI e imagem acima do limite lógico de saída.
- qpdf validou os 11 PDFs positivos. Todas as páginas foram rasterizadas;
  68 arquivos de raster,
  texto e JPEG extraído são idênticos. Os dez cenários JPEG positivos contêm
  seus bytes comprimidos originais; páginas/payloads em `payload-and-pages.json`.
- **9963 testes Rust passaram, 7 ignorados; 42 Python passaram.**
  Os 12 gates passaram: arquitetura/linhagem, build, testes, doctests,
  all-targets, Clippy, fmt e rustdoc. Linter completo sem warning/error.
- Alterações recebidas preservadas byte a byte fora dos quatro inputs
  autorizados; fixture nova e diagnóstico são adições. Índice recebeu somente
  uma entrada. Sem commit/push.

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**.
`before.json` guarda UTC, status, lista exata de alterações/diffstat e hashes
de 2165 inputs, além dos arquivos recebidos.
`after-source-state.json` congela os quatro inputs alterados neste trabalho
(L0, scan.rs, Cargo.toml de L3 e Cargo.lock) e a fixture adicional.
Fechamento `2026-10-27T01:13:20.841227+00:00`. A data no título é local (America/Sao_Paulo); os recibos
conservam UTC, inclusive as execuções já em 27 de outubro UTC.

Baseline SHA-256 `e2fdcf17339d5cb16e5f62da6a38e04797758a7d56159e1a57cbdcdeb9a32711`.
Novo release SHA-256 `c6ff873c095d21fa21a4fbb397e9e4551370454c7301b92bb33020c60d86a417`.
Oráculo preservado no pin `64c5e8c02fe0394d913ead36e42ef8c1334d2107`,
sem nova certificação das antigas campanhas de paridade.
Receita de release e validação: um job, teto 12 GiB, zero swap;
`release-build-receipt.json`, `validation/` e `completion.json` registram
comandos, estados, recursos, hashes e fechamento.

Evidências e scripts:
`lab/results/construction/diagnostics/jpeg-scanline-2026-10-26/`.
