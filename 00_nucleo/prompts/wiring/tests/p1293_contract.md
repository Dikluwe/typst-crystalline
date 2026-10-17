# L0 — wiring/tests/p1293_contract
Hash do Código: 253ea869

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1293_contract.rs`

## Current evidence

- `04_wiring/tests/p1293_contract.rs:24` — `Transcript`.
- `04_wiring/tests/p1293_contract.rs:31` — `from_output`.
- `04_wiring/tests/p1293_contract.rs:39` — `success`.
- `04_wiring/tests/p1293_contract.rs:44` — `trim_transport`.
- `04_wiring/tests/p1293_contract.rs:50` — `run`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1293_contract.rs`.

## Owned contract

- Own only black-box language/export integration: float.is-nan identity/bound calls/casts/diagnostics; math attach/binom/mono/script morphology and geometry (no binom bar); HTML fields/DOM/attributes/escaping/void/nesting/feature-target separation; grid/table short names/public aliases/with/arity and absent nonpublic aliases.
- Compare public language observations, not implementation hashes, administrative catalogs, authorship or execution ordering. Any upstream comparison requires the independently verified current pin; archived earlier runs are not proof against it.
- Use isolated temporary fixtures and finite process limits. Default PNG requires safe-memory success and correct IHDR at 144 PPI (2 pixels/pt); missing output, signal or timeout is failure, not a tolerated result.

## Verification and boundary

Run `cargo test -p typst-wiring --test p1293_contract -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## CI oracle provenance — 2026-10-13

Measurement: `04_wiring/tests/p1293_contract.rs:21` hardcodes
`/usr/local/bin/typst`. The P1489 provenance report establishes that this path
still identifies the previous pin, while the current language reference is
`9f2b6e8715237cb086899a42873660fe744622e8`, upstream tree
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`. A fresh CI runner does not supply
that local installation. This is a harness provenance defect, not a language
change or a reason to relax the observable comparisons.

Decision: obtain the oracle executable from the required `TYPST_VANILLA`
environment variable. The caller must independently establish the current pin,
source tree, build recipe and executable SHA-256 before running these tests.
CI builds a clean checkout of the pinned upstream tree and retains that receipt.
Local runs may use the P1489 ratified lab binary after checking its documented
SHA-256. Missing configuration fails explicitly; never fall back to PATH,
`/usr/local/bin/typst` or the crystalline binary. Keep all public-language,
diagnostic and morphology assertions. This changes only test infrastructure;
product APIs, defaults and pipeline phases remain unchanged.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

04_wiring/tests/p1293_contract.rs:375 (clippy::needless_range_loop); 04_wiring/tests/p1293_contract.rs:580 (clippy::ptr_arg). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

No controle viewBox, iterar expected.iter().enumerate() sobre o array de quatro coordenadas, conservando índices, comparação com summary.0[index], tolerância e mensagens. Não usar zip que trunque silenciosamente uma observação curta. Em push_dom_node, somente stack passa de &mut Vec<DomNode> a &mut [DomNode], pois só last_mut é usado; roots permanece Vec para push. Todos os chamadores privados e DOM/children ficam iguais.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos Transcript, Scratch, DomNode, ParsedDom, seus impls e Drop existentes para `03_infra/tests/support/wiring/p1293_contract.rs`, owner `00_nucleo/prompts/infra/test-support/wiring/p1293_contract.md`. Transferir também os helpers genéricos de observação trim_transport, decode_entities, tag_end, parse_open_tag, push_dom_node, parse_dom, visit_elements, sem mudar seus algoritmos. Conservar em `04_wiring/tests/p1293_contract.rs` todos os cenários e observações finais. Importar somente ParsedDom, Scratch, Transcript, parse_dom, visit_elements por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/p1293_contract.rs:28 struct_item Transcript; 04_wiring/tests/p1293_contract.rs:34 impl_item Transcript; 04_wiring/tests/p1293_contract.rs:48 function_item trim_transport; 04_wiring/tests/p1293_contract.rs:126 struct_item Scratch; 04_wiring/tests/p1293_contract.rs:128 impl_item Scratch; 04_wiring/tests/p1293_contract.rs:141 impl_item Scratch; 04_wiring/tests/p1293_contract.rs:519 enum_item DomNode; 04_wiring/tests/p1293_contract.rs:525 struct_item ParsedDom; 04_wiring/tests/p1293_contract.rs:530 function_item decode_entities; 04_wiring/tests/p1293_contract.rs:538 function_item tag_end; 04_wiring/tests/p1293_contract.rs:550 function_item parse_open_tag; 04_wiring/tests/p1293_contract.rs:580 function_item push_dom_node; 04_wiring/tests/p1293_contract.rs:588 function_item parse_dom; 04_wiring/tests/p1293_contract.rs:650 function_item visit_elements.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.


## Referência atual do harness — re-sync 2026-10-17

### Medição anterior à decisão

`tools/ci/build_oracle.py:12-13` e `.github/workflows/ci.yml:37` foram
auditados no checkpoint `e2d7f6cdc93c5f6b948ca80167411d390ccc714d`.
A consulta oficial `git ls-remote upstream HEAD refs/heads/main` e o fetch
registrados em `00_nucleo/diagnosticos/vanilla-resync-2026-10-17/` identificam
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`.
`04_wiring/tests/p1293_contract.rs:28` ainda nomeia o pin anterior na mensagem
de configuração ausente. A variável obrigatória e os comparadores não mudam.

### Decisão e aceitação

Atualizar somente a mensagem de `TYPST_VANILLA` para o pin atual explícito;
exigir receita, fonte/árvore e SHA-256 registrados antes de usar o executável.
Os parágrafos anteriores registram medições históricas; não certificam o novo
pin. Reexecutar o harness com o novo oráculo, sem relaxar esperados ou tolerâncias.
Fluxo contínuo ADR-0127: infraestrutura de teste, sem mudança pública de produto,
default, compatibilidade ou fase. L0 primeiro e resselo restrito deste owner.
