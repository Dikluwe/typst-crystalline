# Atomização dos braços Heading e Label do walk de introspecção

## Proveniência e medição

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado antes da intervenção, com `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos seis arquivos L0/L1 inspecionados e hora UTC, está em `atomizacao-introspect-heading-label-2026-09-30-before.json.gz` (2026-09-30 16:07:23.504268 UTC). O estado final, com o mesmo inventário e hashes, está em `atomizacao-introspect-heading-label-2026-09-30-final.json.gz`. O filtro `cargo test -p typst-core introspect --lib --offline --locked --quiet` passou 263 testes antes do movimento.

## Decisão e mudança

O `match` em `01_core/src/compiler/introspect.rs` permanece exaustivo e delega estaticamente os braços Heading e Label a `heading::walk_heading` e `labelled::walk_label`. Cada filho já possuía um Prompt L0 exclusivo para seus cálculos; os três L0 envolvidos foram atualizados antes do código e os três pares L0–consumer foram ressellados individualmente. Os selos estão em `atomizacao-introspect-heading-label-2026-09-30-seals.json`.

O pai ainda aloca a Location real, popula o registro inicial e emite Start antes do braço; emite End real após o braço. Heading conserva no filho o incremento sequencial de auto-label, o gate de numbering da StyleChain causal, a materialização temporal do corpo, uma descida recursiva e as tags derivadas Labelled, TOC e bookmarks nessa ordem. Label conserva no filho a descida com label causal, a escolha da primeira Location real, a precedência da linguagem da chain, a associação derivada e a classificação de alvos não referenciáveis. Nenhum dispatch dinâmico, I/O ou contrato público novo foi introduzido.

## Verificação

No estado final inventariado, o filtro `introspect` passou 263 testes; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6393 testes e ignorou 2; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `git diff --check` passou nos seis arquivos tocados; `rustfmt --check` passou nos dois filhos. Avisos não bloqueantes já existentes permanecem na árvore. Essa separação melhora ownership e leitura isolada dos comportamentos; nenhum ganho de CPU foi medido.
