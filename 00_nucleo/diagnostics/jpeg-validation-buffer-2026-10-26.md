# JPEG: validação completa sem destino duplicado — 2026-10-26

Implementada a alteração 1 autorizada nesta conversa: usar o resultado completo
do JpegDecoder existente sem construir a DynamicImage que seria descartada.
O caminho continua decodificando pixels e convertendo cores; não virou validação
de cabeçalhos nem processamento por linhas. Não foram adicionados dependências,
cache, deduplicação, contratos públicos ou mudanças de fase.

## Proveniência e L0

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**. Estado recebido,
lista exata de arquivos alterados, diffstat, UTC e 2165 hashes
em `lab/results/construction/diagnostics/jpeg-validation-buffer-2026-10-26/before.json`.
Estado materializado em `after-source-state.json`; somente
`03_infra/src/export/images/scan.rs` e seu proprietário
`00_nucleo/prompts/infra/export/images/scan.md` mudaram nesses inputs.
Alterações recebidas de notas, geometria e referência vanilla foram preservadas.
Fechamento `2026-10-26T21:45:57.526854+00:00`; os comandos têm timestamps próprios nos recibos.

L0 atualizado antes da implementação, fluxo contínuo ADR-0127 para correção
interna, ownership/Núcleos verificados e selos recalculados somente no proprietário.
Mantidos `Limits::default`, reserva de `total_bytes` antes de `set_limits`,
decodificação completa, erros e envelope `failed to decode image (...)`.
PNG/GIF/WebP, travessia e transporte dos JPEG originais não mudaram.

## RED→GREEN e observação que mudou o teste

A regressão isolada encontrou **duas alocações RGB completas antes e uma depois**.
O alocador System instrumentado pertence apenas aos testes Linux, encaminha suas
operações sem modificar argumentos e ativa contagem somente no subprocesso da
validação, após leitura da fixture. O runtime produtivo não recebe esse alocador.
Logs `allocation-red.log` e `green-allocation-count.log`, mais o teste GREEN de
compatibilidade, retêm os resultados. Testados RGB/gray, truncamentos e dimensões
que ultrapassam o limite de alocação, comparando aceitação e erro ao caminho anterior.

O primeiro guard de VmHWM falhou nos dois caminhos no caso de teste RGB 4:4:4:
buffers internos do decoder dominavam o pico. Ele foi rejeitado, sem relaxar seu
limiar para forçar GREEN. A contagem testa a remoção da alocação; RSS/tempo foram
medidos separadamente em release. Os logs preliminares foram preservados.

## Melhoria medida

Foram executadas **96 compilações alternadas** do antes/depois sobre 12 fontes:
oito válidas e quatro rejeitadas. As três imagens grandes receberam sete amostras
por binário; os outros casos, três. Ambiente, comandos, UTC, hashes dos arquivos
e amostras estão em `paired-results.json`. As medianas abaixo incluem o processo
CLI completo; RSS é o máximo observado por processo, em KiB.

| Caso | RSS antes → depois, KiB | Variação RSS | Tempo antes → depois, ms | Variação tempo |
|---|---:|---:|---:|---:|
| RGB 6000×4000 | 157,096 → 121,832 | -22.4% | 170.9 → 130.2 | -23.8% |
| RGB progressivo 3000×2000 | 69,084 → 60,128 | -13.0% | 79.3 → 68.2 | -13.9% |
| Grayscale 6000×4000 | 63,400 → 62,572 | -1.3% | 55.5 → 40.2 | -27.5% |

No RGB grande, as faixas de RSS foram 156.884–157.452 KiB antes e
121.660–122.248 KiB depois. No grayscale, as faixas se sobrepõem; não declarar
redução conclusiva de RSS nesse caso. Casos pequenos/PNG oscilaram e não sustentam
uma conclusão geral de desempenho. Fixtures são imagens uniformes geradas com
PIL, congeladas por hash, não um corpus de fotografias. Medições locais repetidas,
sem inferir garantia para todas as imagens ou arquiteturas.

O baseline ficou preservado como `typst-before`, SHA-256
`c964cf75e8483b0efcea691cba57d5683229989c553fc94cc3d65c747d793567`. Release corrigido SHA-256
`f88539918c3ad59491e3203d3241128b807ce3037c955909313d5bd4a1629332`. Receita, Rust, tamanho e recursos em
`release-build-receipt.json`; build com um job, teto de 12 GiB e zero swap.
O oráculo continua no pin `64c5e8c02fe0394d913ead36e42ef8c1334d2107`.

## Preservação e validação

- Mesmos status e stderr em todas as amostras pareadas, inclusive JPEG com
  cabeçalho/dados truncados, ausência de EOI e dimensões excessivas.
- PDF válido em todos os oito casos positivos verificados com qpdf.
- 23 arquivos de renderização, texto extraído e JPEG extraído idênticos entre
  versões; os sete streams JPEG também coincidem com os arquivos comprimidos
  originais. Inclui CMYK, progressivo, grayscale, ICC/EXIF e controle PNG.
- Três testes novos; 9957 testes Rust aprovados, 7 ignorados;
  42 testes Python de CI aprovados. Os 12 gates passaram: build, suites,
  doctests, all-targets, Clippy, fmt, rustdoc e verificações arquiteturais.
  Linter completo sem warning/error; notas informativas permanecem.
- Sem commit/push. Índice de diagnósticos atualizado apenas com nova entrada;
  os demais arquivos recebidos permanecem intactos.

## Limitação técnica

`image` 0.24.9 marca `into_reader` como deprecado para remoção futura. A exceção
é local a essa chamada, documentada no L0; atualizar a dependência exigirá rever
esse caminho. Seu comportamento eager foi conferido no fonte da versão travada.
Essa alteração não promete memória constante: pixels e buffers internos continuam
existindo, e JPEG progressivo mantém necessidades adicionais de coeficientes.

Recibos e scripts: `lab/results/construction/diagnostics/jpeg-validation-buffer-2026-10-26/`.
