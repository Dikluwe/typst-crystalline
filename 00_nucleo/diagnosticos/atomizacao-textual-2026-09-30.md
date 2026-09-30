# Atomização da realização textual de eval/rules

## Proveniência

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado antes da movimentação do código foi capturado em `atomizacao-textual-2026-09-30-state.json.gz` às 2026-09-30 15:09:28.988179 UTC, depois da edição L0 inicial. O estado final, incluindo `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos consumers e prompts afetados, e horário UTC está em `atomizacao-textual-2026-09-30-final-state.json.gz`. O estado já continha numerosas alterações externas a este recorte; não foram resselladas globalmente.

## Medição antes da decisão

No estado capturado antes da movimentação, `01_core/src/compiler/eval/rules.rs:3932-4427` continha snapshot causal, interceptação dos nós de texto, agrupamento, recortes literal/regex, normalização de espaços e testes Unicode. `compiler/eval/mod.rs:1371,1408,1416,1470,1550` chamava as três entradas. O filtro focal `cargo test -p typst-core p1494 --lib --offline --locked --quiet` passou 17 testes antes da movimentação, no mesmo HEAD e working tree não commitado, com a edição L0 inicial como única alteração interposta antes da captura de proveniência. Esta medição verifica a base estrutural, não paridade bilateral nem desempenho.

## Mudança

O comportamento foi movido integralmente para `01_core/src/compiler/eval/rules/textual.rs` e ganhou o Prompt L0 exclusivo `00_nucleo/prompts/compiler/eval/rules/textual.md`. O L0 pai agora delega essa responsabilidade; o L0 de `compiler/eval.md` aponta a normalização ao novo owner. `eval/mod.rs` chama o módulo filho diretamente. O despacho estático de set/show e a aplicação de show a um nó permanecem no pai. Não houve alteração intencional de algoritmo, assinatura pública, fase, default ou compatibilidade. Os três pares L0–consumer foram ressellados individualmente; hashes em `atomizacao-textual-2026-09-30-seals.json`.

## Verificação

No estado final capturado, `cargo test -p typst-core p1494 --lib --offline --locked --quiet` passou 17 testes; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6392, com 2 ignorados; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0, sem violações bloqueantes. O lint ainda emite avisos em outras partes da árvore e três avisos V16 no bloco textual movido, já presentes na lógica anterior. `rustfmt --edition 2021 --check --config skip_children=true` passou no novo módulo. Nenhum ganho de desempenho é atribuído a esta extração sem benchmark controlado.

## Próximas medições

A auditoria de atomização indica os braços Heading/Label da introspecção como próximo recorte estrutural. Para velocidade, instrumentar separadamente o agrupamento textual, a montagem da stdlib por passagem, o layout de grid e a fase de fontes; o custo inicial de fontes medido na comparação com vanilla pertence principalmente a L3. Evitar inferir ganho de CPU da divisão de arquivo.
