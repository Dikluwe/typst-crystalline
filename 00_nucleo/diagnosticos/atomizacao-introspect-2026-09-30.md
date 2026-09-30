# Atomização de introspect — 2026-09-30

## Proveniência

Base: `df72a6c4a701917ab56a7c6169764a92ea775693`, árvore limpa antes da extração. Os estados completos, com SHA-256 e contagem de linhas, estão em `atomizacao-introspect-2026-09-30-before.json.gz` e `atomizacao-introspect-2026-09-30-after.json.gz`. O estado final registrado ainda não estava commitado; o manifesto lista os arquivos alterados.

## Medição e decisão

No estado base, `01_core/src/compiler/introspect.rs` tinha 5.724 linhas, incluindo testes. A medição do código mostra funções independentes para conversão bibliográfica, consultas, entradas, outline, materialização temporal, população de stores, índice de pais, classificação e walk. Decisão: mover esses comportamentos para nove owners L1 com nove Prompts L0 correspondentes. O wrapper público `introspect` e os testes permanecem no pai. O arquivo principal passou a 3.376 linhas, principalmente por manter os testes. Foram deslocadas 2.377 linhas de blocos de produção, sem mudança deliberada de algoritmo nem afirmação de desempenho.

A interface pública existente continua reexportada pelo módulo pai. O walk mantém match exaustivo e despacho estático; Heading e Label continuam delegados aos filhos já existentes. A ordem de `Location`, `Tag::Start` e `Tag::End`, o tratamento de bundles e os pós-processadores runtime não foram alterados.

## Verificação

`cargo build --offline --locked --quiet` passou. `cargo test -p typst-core --lib --offline --locked --quiet` passou: 6.402 testes, 2 ignorados, 0 falhas; inclui um novo teste para ordem causal e unicidade das consultas por tipo. `crystalline-lint .` não emitiu erros nos novos owners; avisos informativos e de drift preexistentes em artefatos de diagnóstico permanecem fora deste recorte. `git diff --check` passou.

Esta operação só prepara a busca de algoritmos alternativos por comportamento. Não mede tempo ou memória e não prova paridade nova contra o vanilla pinado.
