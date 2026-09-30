# Identidade textual da montagem Large até PDF

Medição em 2026-09-30 12:03:04 UTC, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. A lista exata de alterações presentes no momento está em `working-tree-status.txt`, com `git diff HEAD --stat` em `working-tree-stat.txt`. Este checkout é compartilhado; os dois recibos contêm alterações de outras frentes. O script `seal_scope.py` e `scoped-seals.json` identificam somente os sete consumers ressellados nesta correção.

## Medição antes da decisão

A sonda `probe.typ` usa o vanilla pinado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` e NewCMMath-Book com somente `DisplayOperatorMinHeight=3000du` SHA-256 `7309e4593ac90f72cb7bf0e3648fafd17dd107167f68876bfe013d3f9c1feb48`. Esse alvo obriga `∫` a uma assembly de três peças. Antes, `large_operator.rs` entregava três `FrameItem::Glyph` sem portador de ocorrência. O PDF Tekt extraía `⌡⎮⌠` de ToUnicode; o vanilla agrupava as peças em `ActualText=∫`. O RED em `layout-red.log` confirmou a ausência de TextReplacement no L1. Isto é perda de identidade morfológica, não exigência de reproduzir o mesmo GID ou os mesmos bytes PDF.

Depois de envolver as peças no TextReplacement existente, o PDF já continha `ActualText=∫` e `pdftotext` simples devolvia `∫`, mas a marcação exterior aos estados gráficos dava caixa de altura zero no Verbose e ordem invertida em `pdftotext -layout` nos dois modos. O experimento QDF temporário abriu BDC após BT da primeira peça e fechou EMC antes de ET da última, sem mudar GIDs/posições; `pdftotext -bbox` passou a medir caixa positiva e `-layout` devolveu a ordem `∑`, `∫`. A hipótese de simples falta de marcação foi, portanto, insuficiente.

## Correção

O owner `large_operator.rs` envolve exclusivamente a montagem vertical escolhida para Large em `TextReplacement { text: c, items: peças }`. `assembly.rs` continua dono da geometria, sem alteração. `attach.rs` lê a correção itálica dos glifos visuais dentro do envelope quando posiciona scripts, preservando o cálculo anterior. O exportador reutiliza o mapeamento de fontes e GIDs dos emissores Glyph existentes; os emissores marcados posicionam BDC/EMC dentro de BT/ET, e o dispatcher os usa somente para TextReplacement cujos filhos são Glyph. Outros envelopes, como SHY, seguem o caminho original. Nenhum carácter é inferido de GID no PDF.

Os L0 de `large_operator`, `attach`, testes, `layout_types`, `stream`, `stream/text` e `stream/draw` foram atualizados antes do código e ressellados de forma escopada. Não houve campo de entidade, método de trait, assinatura pública, default ou fase novos; o fluxo é contínuo segundo ADR-0127.

## Verificação e limite

- RED→GREEN: `layout-red.log` falhou porque a assembly eram Glyphs diretos; `layout-green.log` passou. A suíte P952 passou 16/16, incluindo montagem com scripts e correção itálica. Os dois testes PDF de hífen discricionário passaram.
- `cargo build -q` e `crystalline-lint .` saíram 0. O linter emite avisos V16 preexistentes, sem violações bloqueantes; `git diff --check` nos arquivos afetados passou.
- Verbose e Compact: `pdftotext -layout` passou a devolver `∑` seguido de `∫`; `pdftotext -bbox` mede caixa positiva em ambos (`*-bbox.html` e `*-layout.txt`). `mutool` mostra um `ActualText=∫`. O raster 144 dpi antes/depois tem SHA-256 idêntico `254a7886d95712770dffe1eb0918d3f1e754338709c0ab927327488ca1ee7546` nos dois modos, confirmando tinta preservada. `qpdf --check` não encontrou erro sintático.
- PDF/UA1 da mesma sonda mantém a única falha veraPDF preexistente: duas fórmulas sem Alt/ActualText no StructElem. `ua1-before.json` e `ua1-after.json` mostram 105 regras aprovadas e 1 falhada em ambos; o ActualText do content stream não substitui esse contrato do StructElem. A caixa do `∫` no Tekt ainda não coincide exatamente com a do vanilla e o Compact mantém espaço de linhas diferente; esta correção fecha identidade extraível, ordem e caixa não degenerada, sem declarar paridade geométrica completa.
