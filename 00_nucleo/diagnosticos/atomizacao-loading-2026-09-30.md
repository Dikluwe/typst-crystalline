# Atomização de loading — 2026-09-30

## Proveniência

Base: `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa`, árvore limpa. Estados completos, SHA-256, horário e contagem por arquivo: `atomizacao-loading-2026-09-30-before.json.gz` e `atomizacao-loading-2026-09-30-after.json.gz`. O estado final foi medido antes do commit; o manifesto identifica cada arquivo.

## Medição e decisão

Na base, `01_core/src/compiler/stdlib/loading.rs` tinha 3.458 linhas. Seu código de produção agrupava onze comportamentos separáveis: projeção textual/validação de Args, encoders JSON/TOML/YAML, decoders JSON/YAML/TOML/CBOR/XML, CSV e leitura/resolução de fonte. Foram criados onze módulos L1 com onze Prompts L0 proprietários. O módulo pai conserva helpers mínimos, macro estática de composição das quatro nativas e os testes históricos; agora tem 2087 linhas, das quais a maior parte são testes.

A API pública existente é reexportada pelo módulo pai. Os contratos de Value, spans, ordem de validação, World injetado e distinção path/Bytes foram preservados. O decode continua puro em L1; não foi adicionado acesso direto a filesystem nem mudança de parser.

## Verificação

Baseline: `cargo test -p typst-core --lib --offline --locked --quiet` — 6.402 passaram, 2 ignorados. Após a separação: 6.413 passaram, 2 ignorados, 0 falhas; onze novos testes cobrem as unidades extraídas. `cargo build --offline --locked --quiet` passou. `crystalline-lint .` não emitiu erros; avisos existentes em outros arquivos não foram tratados neste recorte. `git diff --check` passou.

O recorte facilita investigar algoritmos por formato. Não houve medição de tempo/memória nem nova comparação bilateral com o vanilla pinado.
