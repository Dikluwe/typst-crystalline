# Item 4 — geometria e raster, 2026-10-08

## Proveniência

A medição final está em `measurements.json`, gerada por `measure.py` em
2026-10-09T01:27:37.034943 UTC contra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`
com working tree não commitado. O estado exato dos arquivos alterados e não
rastreados no momento da medição consta de `git-diff-head-stat.txt` e
`git-status-short.txt`; seus SHA-256 estão em `measurements.json`. O binário
Tekt release tem SHA-256 `3d6b4b21b85518c82433d48bfc64fb61dab8a1afda43708a3d3793568161201f`;
o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` tem binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Fontes controladas em `00_nucleo/diagnosticos/performance-baseline/fonts`;
`RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`, `CRYSTALLINE_PDF_FIXED_EPOCH=0`,
`TZ=UTC`, `--jobs 1` no vanilla. Os hashes das cinco entradas também estão no JSON.
O script mede caixas `pdftotext -bbox` e diferenças RGB do Poppler na primeira
página a 36, 72, 144 e 288 dpi. Isto mede o raster do Poppler, não todos os renderers.

## Resultados separados

| Caso | Geometria final Tekt vs vanilla | Pixels diferentes a 36/72/144/288 dpi | Estado |
|---|---:|---:|---|
| `context` | máximo 0 pt, 2 palavras | 0 / 0 / 0 / 0 | Corrigido |
| `text-dense` | máximo 0,00003 pt, 672 palavras | 697 / 1853 / 4590 / 13974 | Deslocamento corrigido; raster residual aberto |
| `table-alias` | máximo 0,00002 pt, 100 palavras | 24 / 24 / 0 / 0 | Resíduo só em baixa resolução |
| `table-local` | máximo 0,00002 pt, 100 palavras | 149 / 449 / 925 / 3400 | Resíduo de glifo aberto |
| `image` | sem palavras | 0 / 0 / 0 / 0 | Corrigido |

### Deslocamento vertical em contexto e texto denso

Antes, `context` deslocava a linha em 0,658 pt e 376 pixels a 144 dpi;
`text-dense` deslocava apenas as dez palavras da última linha em 0,658 pt.
A inspeção causal mostrou itens textuais de 10 pt contra o máximo 11 pt
herdado do estilo exterior em `line_commit.rs`. O ascent excedente era 0,658 pt.
O cálculo agora usa o máximo dos itens reais nas linhas normais e conserva
as exceções de singleton forçado e modo de colunas. O teste novo de dois
fragmentos menores que o exterior foi RED→GREEN. O controle P1381 de colunas
falhou na primeira implementação e voltou a passar após preservar o fallback
exterior nesse modo. A suíte completa do núcleo terminou com 6649 testes
aprovados, 5 ignorados.

A geometria final de `text-dense` coincide até o arredondamento mostrado, mas
persistem 4590 pixels distintos de 960000 a 144 dpi. Como a contagem era a
mesma antes da correção, ela não constitui prova de deslocamento vertical
remanescente. A causa exata do raster de texto denso segue aberta.

### Imagem JPEG

Antes, o XObject JPEG era byte a byte idêntico e a matriz de 80×80 pt também,
mas o Tekt aplicava um clip igual ao próprio retângulo. Isso gerava 321 pixels
diferentes a 144 dpi nas bordas direita/inferior. Agora o exporter só aplica
clip se alguma aresta da imagem transformada ultrapassar o retângulo.
O controle sem crop coincide em quatro resoluções; o teste de crop real
continua positivo. O snapshot controlado 08 mudou de 3557 para 3541 bytes,
foi gerado duas vezes com SHA-256 idêntico, validado com `qpdf --check`, e os
outros oito snapshots ficaram intactos. A suíte de infraestrutura com epoch
fixo terminou com 1255 testes aprovados, 2 ignorados. Sem epoch fixo, dois
testes históricos de igualdade de bytes variam pelo identificador PDF;
ambos passaram isoladamente sob o mesmo controle da suíte final.

### Tabelas

`table-alias` tem somente 24 pixels de antialiasing distintos a 36 e 72 dpi;
a 144 e 288 dpi o raster coincide. `table-local` mantém 925 pixels distintos
em 960000 a 144 dpi (bbox x=224–229, y=57–868), concentrados no último
`l` de `Local` repetido nas linhas. Caixas de texto coincidem a 0,00002 pt;
na inspeção de SVG os contornos e posições pareceram equivalentes. Exportação
compacta e verbosa não removeu a diferença; alterar isoladamente `/StemV` e
`/Flags` do FontDescriptor também não. Estes experimentos **não** identificam
a causa. Investigar mapeamento/posicionamento subpixel do glifo e rasterização
antes de mudar o exporter; a diferença de baixa resolução de `table-alias`
deve ser tratada separadamente. Nenhuma equivalência de raster das tabelas é
reivindicada além das resoluções e condições medidas.

## Validação

`cargo test --offline -p typst-core --lib -- --test-threads=4` passou;
`CRYSTALLINE_PDF_FIXED_EPOCH=0 cargo test --offline -p typst-infra --lib -- --test-threads=4`
passou; `cargo build --offline --release` passou; `crystalline-lint .` não
reportou violações (somente avisos informativos existentes). Os testes da
correção de geometria e do clip tiveram RED→GREEN antes do build final.
