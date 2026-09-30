# Fechamento dos casos RTL restantes — 2026-09-29

Estado: working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`. A proveniência integral dos binários, fontes, ficheiros fonte e estado da árvore está em `final/provenance.json`; o recibo compacto está em `closure.json`. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 do binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt SHA-256 `208de722f2312f0dad639300a057556b6fab854829eb300aa9594007d7b39190`.

## Medição antes da decisão

O `after-leading` fechou os seis casos de espaços literais no início de linhas RTL. Restavam seis casos hebraicos com diferença de 7,68 pt. `pdffonts` mostrou Noto Sans Hebrew no Tekt e Libertinus Serif no vanilla. O código pinado `lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101` lista Libertinus Serif e fontes emoji como fallback explícito depois da família declarada; `03_infra/src/shaper/font_selection.rs` passava diretamente da primária resolvida à busca global. A fonte diferente é causa mensurável anterior ao posicionamento.

## Correção

`01_core/src/compiler/layout/cursor.rs` reserva a largura de prefixos `TextSeparator::Literal` na borda visual direita de linhas RTL, sem somar esse avanço duas vezes. `03_infra/src/fallback_fonts.rs` declara a cadeia comum do vanilla; `03_infra/src/shaper/font_selection.rs` acrescenta essa cadeia depois da família declarada e antes do fallback global. `03_infra/src/font_metrics/resolution.rs` consulta a mesma cadeia para as métricas. Os owners L0 foram atualizados primeiro e selados em `scoped-seals.json`. Não há seleção por língua, número compensatório nem alteração de contrato público.

## Aceitação

- RED→GREEN: `red-leading.log`, `green-leading.log`, `red-hebrew.log`, `green-hebrew.log`.
- Corpus RTL final: 66 casos, texto e páginas iguais; maior diferença de origem de glifo 0.00003100 pt (< 0,001 pt). A face hebraica emitida pelo Tekt passou a Libertinus Serif. `final/results.json` guarda comandos, texto, glifos e PDFs dos três modos.
- Replay histórico: 401 entradas, nenhuma regressão de texto, geometria, estado ou erro; tolerância 0,001 pt.
- Controles LTR: 36 casos, diferença máxima 0,00003 pt. Corpus mixed: 42 casos, nenhuma diferença acima de 0,001 pt. Corpus adicional: 42 casos, texto/páginas/caixas de palavras coincidentes dentro de 0,001 pt.
- `cargo test -p typst-core --lib`: 6379 passaram, 2 ignorados. `cargo test -p typst-infra --lib`: 1138 passaram. `cargo build` e build release passaram. `crystalline-lint .` terminou com código 0; nenhum aviso V5/V15/V26 nos cinco consumers alterados. A saída contém avisos informativos/históricos de outros arquivos.

Este fechamento comprova os corpora medidos, não paridade geral para todas as combinações de fonte e escrita.
