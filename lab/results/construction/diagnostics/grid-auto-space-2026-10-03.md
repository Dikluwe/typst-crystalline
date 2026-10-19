# Ablação do espaço final na medição de colunas Auto

**Data:** 2026-10-03. Hipótese testada contra vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`. O [probe](grid-auto-space-2026-10-03/probe.py)
e o resultado integral em `/tmp/tekt-grid-auto-space/results.json` permitem
repetir a comparação.

## Medição antes da decisão

O estudo anterior em `grid-vanilla-phase-2026-10-03.md` encontrou um desvio de
2,5 pt por coluna Auto de texto de uma palavra; a sonda `measure` da linguagem
mediu 2,5 pt para um espaço na mesma fonte. `measurement.rs:70-96` somava
`word_w + space_w` em toda palavra, incluindo a última. `grid/tracks.rs:79`
utiliza esse resultado para Auto. No vanilla pinado,
`lab/typst-original/crates/typst-layout/src/grid/layouter.rs:967-1100`
determina a largura pelo layout da célula. A observação inicial era causalmente
plausível, mas a ablação abaixo era necessária para confirmá-la.

O teste `auto_text_measurement_counts_only_interword_spaces` falhou antes da
correção em `palavra única sem espaço final` (exit 101, 1 falha) e passou após
a correção (1 passou). Também verifica duas palavras com exatamente um espaço
intermediário e uma quebra com duas linhas.

Às **2026-10-03 15:07:25 UTC**, o probe mediu a árvore não commitada sobre
HEAD `09da7fa67c93c5adef2a25d7b78b72e3529eb667`. `git diff HEAD --stat`
registrava `measurement.md` (12 linhas alteradas), `tests.md` (12),
`measurement.rs` (15) e `tests.rs` (39); o diretório
`00_nucleo/diagnosticos/grid-auto-space-2026-10-03/` estava não rastreado.
Os binários SHA-256 eram:

| Motor | SHA-256 |
| --- | --- |
| Tekt antes | `b0207af549b16e85001f1e2b571fd8d620d07406ccdb3a6cdd850474694ffa10` |
| Tekt depois | `94a084ecc1768549d180ecf65faf6ca3c269f7aed3ff0b85b0166c9fffbc6e50` |
| Vanilla | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |

Mesma fonte Libertinus Serif 10pt, PDF sem tags, `SOURCE_DATE_EPOCH=0`,
`RAYON_NUM_THREADS=1`, CPU 0, `--jobs 1` no vanilla. Quatro fixtures:
`auto1` SHA-256 `d5739642bf8c6da1967d8665bb9b45f8b0c522ca4e1a463eb59725447e63e9d2`,
`auto80` `902986bd28ace4b39144ce567592229c5a5621453cbb661a408414c803a58d6d`,
`fixed80` `8f25fa3c91dfbb6885251f18fa156eb56e86d0bb887d144a18e6a6e2d5ffe5c8`,
`multi1` `96be9d926d52c72dfd4930fb6b39a6c3ba33b2e79e4135d21b5b28e8a8eec698`.

| Fixture | Palavras comuns | x diferente do vanilla antes | x diferente depois | Máximo desvio antes → depois |
| --- | ---: | ---: | ---: | ---: |
| Auto, 1 linha | 4 | 3 | 0 | 7,5 → 0 pt |
| Auto, 80 linhas | 320 | 240 | 0 | 7,5 → 0 pt |
| Fixa 100pt, 80 linhas | 320 | 0 | 0 | 0 → 0 pt |
| Auto, 4 células de duas palavras | 8 | 6 | 0 | 7,5 → 0 pt |

Diferença de x usa `pdftotext -bbox` com limite de 0,001 pt. O efeito de
2,5 pt por coluna desapareceu completamente nesses controles. Assim, a
hipótese do espaço artificial após a última palavra está **confirmada para
células textuais simples nas condições medidas**. Não se infere daí paridade
automática para insets, spans, estilos mistos ou quebra entre páginas.

## Tempo

Doze execuções por motor, duas warmups, ordem alternada; tempo de CLI inclui
carregamento, compilação e escrita do PDF da tabela de 80 linhas. Medianas:
Tekt antes **221,65 ms**, depois **223,38 ms**, vanilla **144,27 ms**.
Diferença depois−antes: **+1,73 ms** nesta coleta, pequena diante do custo
total e sem prova de regressão causal ou ganho. A correção é geométrica; a
duplicação de layout paginado e a diferença restante de tempo continuam
questões separadas.

## Verificação e decisão

`cargo build --locked --offline`, `cargo build --release --locked --offline
-p typst-wiring --bin typst` e `crystalline-lint .` saíram com código zero.
O linter ainda emitiu avisos preexistentes, inclusive V5 em cópias históricas
de `.rs` sob diagnósticos; não houve V5 nos dois consumers alterados. A suíte
focada de grid passou com **154 testes, zero falhas**. A decisão é manter a fórmula sem espaço final: corrige a
geometria observável dos controles sem alterar API, default ou fase do
pipeline. Não reivindicar melhora de desempenho com esta mudança.
