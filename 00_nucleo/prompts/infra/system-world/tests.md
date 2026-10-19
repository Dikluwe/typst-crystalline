# L0 — infra/system-world/tests
Hash do Código: c26947c4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a


**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/tests.rs`

## Current evidence

- `03_infra/src/world/tests.rs:18` — `MockWorld`.
- `03_infra/src/world/tests.rs:26` — `new`.
- `03_infra/src/world/tests.rs:38` — `library`.
- `03_infra/src/world/tests.rs:41` — `book`.
- `03_infra/src/world/tests.rs:44` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/tests.rs`.

## Owned contract

- Own World mocks, filesystem/font/path/bibliography/package fixtures and unit cases under world::tests, not production implementations.
- Characterize synthetic main without I/O, memoized source/error, path/id shared successful bytes, retry after byte error, canonical identity/dependencies (including concurrent registration of the same path), lexical-escape rejection and book coverage without source access.
- Characterize a valid project font added after configuring the world and before its first catalog request, retained catalog identity under later/concurrent requests, and old coverage cleared when replacing a configuration. Keep the precomputed-coverage-without-font-bytes regression.
- Use local deterministic temporary fixtures and no network. Execute serially because a legacy test mutates XDG_DATA_HOME. Arc-identity assertions characterize cache mechanics, not language parity.

## Verification and boundary

Run `cargo test -p typst-infra --lib world::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Testar ids distintos de documento e expressões, Source code resolúvel,
world.file com bytes UTF-8 exatos, expressões iguais com ids independentes,
stdin carregado sem arquivo e read/import relativos. Arquivos físicos reais
com nomes <input-expression> e <stdin> não podem aliasar transientes; dependencies
não contém os marcadores, mas contém esses arquivos quando efetivamente usados.
Testar registro concorrente físico/transiente, última identidade disponível e
exaustão sem sobrescrita: sources/caches/dependencies anteriores permanecem
resolúveis, inclusive depois da falha física capturada. Manipular contador em
fixture interna para alcançar fronteira sem criar milhares de arquivos.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

## Adaptação interna de fixture — retenção de fontes ratificada

Medição anterior à decisão: /tmp/watch-green.log (cópia preservada no relatório
de execução) contém E0596 em 03_infra/src/world/tests.rs:493-498 depois do estado
Arc ratificado para SystemWorld. O fixture de cobertura pré-computada precisa
mutar somente seu Arc exclusivo, usando Arc::get_mut; preservar cobertura, slot
inexistente e todas as assertions. Adaptação interna de teste, sem contrato ou
comportamento público novo, em fluxo contínuo ADR-0127. Owner continua exclusivo.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/world/tests.rs:206 (clippy::len_zero). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

No teste do catálogo de fontes do projeto, usar !world.font_catalog().0.is_empty() em vez de len >= 1. Conservar o acesso, fontes válidas e o requisito de catálogo não vazio.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.
