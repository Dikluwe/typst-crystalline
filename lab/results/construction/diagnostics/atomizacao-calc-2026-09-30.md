# Atomização de calc — 2026-09-30

## Proveniência

Base: `d9e851a850c66905bdb0c3ea4b155a6f36881106`, árvore limpa. Os estados completos, SHA-256, horário e contagem por arquivo estão em `atomizacao-calc-2026-09-30-before.json.gz` e `atomizacao-calc-2026-09-30-after.json.gz`. O estado final foi medido antes do commit; o manifesto identifica cada arquivo.

## Medição e decisão

Na base, `01_core/src/compiler/stdlib/calc.rs` tinha 4.512 linhas. O código de produção agrupava valor absoluto, potência e raiz, arredondamento, extremos, trigonometria, hiperbólicas, logaritmos, aritmética inteira, combinatória, norma, erf e conversão/guarda numérica. Foram criados doze módulos L1 com doze Prompts L0 proprietários. O módulo pai conserva o registro estático de `make_calc_module`, os testes históricos e o harness dos novos testes; passou a 3162 linhas, majoritariamente testes.

Os nomes e bindings nativos, tipos de retorno, prioridades de validação, spans, mensagens e política local para não finitos foram preservados. A atomização não introduziu despacho dinâmico nem mudou a fórmula da aproximação de erf.

## Verificação

Baseline: `cargo test -p typst-core --lib --offline --locked --quiet` — 6.420 testes passaram, 2 ignorados. Depois: 6.432 passaram, 2 ignorados, 0 falhas; doze novos testes exercitam as famílias extraídas. `cargo build --offline --locked --quiet` passou. `crystalline-lint .` não emitiu erros. `git diff --check` passou.

A divisão facilita pesquisar algoritmos alternativos por família. Não houve medição de tempo/memória nem nova comparação bilateral com o vanilla pinado.
