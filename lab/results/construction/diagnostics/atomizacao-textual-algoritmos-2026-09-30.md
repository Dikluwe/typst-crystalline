# Separação algorítmica da realização textual

## Proveniência

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado anterior à movimentação dos algoritmos, com `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos arquivos medidos e hora UTC, está em `atomizacao-textual-algoritmos-2026-09-30-before.json.gz` (2026-09-30 15:43:49.246095 UTC). O estado antes do recorte adicional da projeção está em `atomizacao-textual-projecao-2026-09-30-before.json.gz` (2026-09-30 15:50:59.896744 UTC). O estado final, com o mesmo tipo de inventário e hashes dos oito arquivos L0/L1 tocados, está em `atomizacao-textual-algoritmos-2026-09-30-final.json.gz`. A árvore já continha numerosas alterações não relacionadas; nenhum outro consumer foi ressellado.

## Medição antes da decisão

No primeiro estado, `01_core/src/compiler/eval/rules/textual.rs` continha coordenador, normalização de espaços, agrupamento/matching textual e projeção dos trechos originais. O filtro `cargo test -p typst-core p1494 --lib --offline --locked --quiet` passou 17 testes antes da movimentação. A medição é estrutural e não prova ganho de tempo ou nova paridade com vanilla.

## Separação realizada

`textual.rs` mantém o snapshot causal, a interceptação de nós e a ordem de coordenação: normalização condicional, retorno se show desativado, aplicação de matching. `textual/space.rs` possui exclusivamente a normalização, origem de newline, vizinhos e predicado Unicode. `textual/matching.rs` possui agrupamento causal, precedência das regras, guardas, transformação e restauração do engine. `textual/matching/projection.rs` possui a reconstrução de fatias não casadas e o achatamento das sequências de substituição. Os três algoritmos e o coordenador possuem Prompts L0 e headers exclusivos; os selos estão em `atomizacao-textual-algoritmos-2026-09-30-seals.json`.

Um teste do coordenador verifica que a identidade do grupo depende do mesmo Arc de regras, do estilo colapsado e das guardas. Os testes originais de Unicode, espaços e projeção foram movidos com os algoritmos donos. O recorte não altera assinatura pública, fase, default ou compatibilidade.

## Verificação

No estado final inventariado, o filtro `p1494` passou 17 testes; o teste novo de agrupamento causal passou; a suíte `cargo test -p typst-core --lib --offline --locked --quiet` passou 6393 testes, com 2 ignorados; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `rustfmt --check` passou nos quatro módulos. O lint mantém avisos V16 nos padrões existentes movidos e avisos em outros módulos, sem violação bloqueante. Não há benchmark de desempenho nesta etapa.
