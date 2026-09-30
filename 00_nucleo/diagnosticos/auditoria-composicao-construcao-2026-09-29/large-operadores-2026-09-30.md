# Operadores Large: variante máxima e montagem vertical

Medição: 2026-09-30 03:16:00 UTC; HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`; working tree não commitado, com lista exata de arquivos alterados no instante da medição em [`large-operadores-status-2026-09-30.txt`](large-operadores-status-2026-09-30.txt). O checkout é compartilhado; as alterações de outras frentes não são atribuídas a esta correção.

## Fonte e classificação

- Vanilla pinado: `lab/typst-original/target/release/typst` SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; `lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:268-312` conserva a última variante quando nenhuma chega ao alvo e não existe assembly; com assembly, monta peças. `resolve.rs:350-354` ativa o stretch em Display; `glyph.rs:450-454` usa `DisplayOperatorMinHeight`. Isto é comportamento de forma/geometria da linguagem, não igualdade mecânica.
- Fonte reproduzível: `/usr/share/texmf/fonts/opentype/public/tex-gyre-math/texgyrepagella-math.otf` SHA-256 `1f9e010f60e947d0e925910009b2ea85ad54edc7cefb106f8cdefb9ffd1d5f2f`. Sua tabela MATH dá `DisplayOperatorMinHeight=1500du` para `∑`; variantes `summation=1001du` e `summation.v1=1443du`, sem assembly. Logo a maior variante fica abaixo do alvo. Estes valores foram extraídos via fontTools 4.46.0 do arquivo indicado.
- Tekt antes da correção: `01_core/src/compiler/math/layout/mod.rs:1712-1742` escolhia somente variante suficiente e, nesse caso, retornava ao glifo base. O teste com métricas injetadas confirmou RED: glifos `[]` contra `[(101, 17.328)]` esperados. A causa está na política de fallback do Large, não no owner textual `op.rs`.

## Correção e limites da evidência

O L0 `_comum.md` agora especifica primeira variante suficiente, senão assembly ou maior variante, e o L0 `tests.md` exige os dois casos. `layout_large_operator_display` usa o compositor vertical já existente para assembly e mantém o caminho de emissão do próprio Large para variantes. Sem construção, permanece o glifo base. Não mudou contrato público, modo padrão, fase do pipeline nem compatibilidade declarada; aplica-se o fluxo contínuo ADR-0127.

A sonda bilateral PDF com a fonte Pagella não serve como prova end-to-end: o vanilla aceitou `#show math.equation: set text(font: "TeX Gyre Pagella Math")` e incorporou essa fonte, enquanto o CLI cristalino rejeitou `math.equation` como seletor. Os PDFs inicialmente compilados com `#set text(font: ...)` usaram NewCMMath em matemática e por isso não exercitam esta diferença. O fechamento nesta frente combina a política da fonte pinada, a tabela OpenType e regressões de L1; a seleção de fonte por seletor é frente distinta.

Testes: `cargo test -q -p typst-core large_sem_variante_suficiente --lib` passou 2/2; `cargo test -q -p typst-core p952op_tests --lib` passou 14/14; `cargo build -q` passou; `git diff --check` nos quatro arquivos Large passou. V5 está limpo nos dois consumers desta frente. `crystalline-lint .` completo retornou 12 erros V26 de pin obsoleto em `paragraph-selection.toml` no checkout compartilhado; não decorrem do Large.
