# L0 — wiring/tests/math_decoration_and_float_flush
Hash do Código: 9a14f202

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/math_decoration_and_float_flush.rs`

## Current evidence

- `04_wiring/tests/math_decoration_and_float_flush.rs:16` — `Scratch`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:19` — `new`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:32` — `drop`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:37` — `eval`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:44` — `diagnostic`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/math_decoration_and_float_flush.rs`.

## Owned contract

- Own the black-box language/geometry suite only. Use isolated fixtures, propagate subprocess channels on failure and never turn Unknown into success.
- Preserve callbacks/cross/background/spans/no-provisional-output controls. Observe identity, morphology, dimensions, positions, presence and order; JSON/SVG/PDF are transports, not total-byte oracles.
- math.underline equality with underline must be JSON boolean false. SVG automatic-page geometry applies every ancestor translate/matrix in affine order once; no glyph/family/fixture-specific constants or defs/id-order comparisons.
- measure tests execute contextual measurement by rendering a uniquely colored shape, separating finite and height-auto regions; query/metadata/root are not execution substitutes. Preserve delimiter/alignment/repr/default/error controls.
- place.flush observes page, bbox and unique ordered tokens with explicit failure if pdftotext is absent. Cover prefix/suffix, no-flush, top/bottom floats, active-layouter nesting and no-float no-op. Keep tokens out of flow and remove fixtures.

## Verification and boundary

Run `cargo test -p typst-wiring --test math_decoration_and_float_flush -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Explicit code-mode alignment in math fixtures — 2026-10-13

### Measurement before decision

`04_wiring/tests/math_decoration_and_float_flush.rs:706` uses `align:left` in math syntax. Both
the verified vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 and Tekt reject
it with `unknown variable left`; `align:#left` succeeds in both. Sources,
binary hashes and outputs are in
`lab/results/construction/diagnostics/correcoes-producao-2026-10-13/math-bilateral.json`.

The alignment-placement fixture must use `#left`/`#right` as code-mode values
inside math. Preserve the differing-placement assertion and all geometry/repr
controls. Add an explicit negative syntax control so invalid bare alignment
does not become accepted to satisfy the fixture. This repairs the test's input,
not the public language. Internal ADR-0127 continuous correction.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de lab/results/construction/diagnostics/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos Scratch, SvgTransform, seus impls e Drop existentes para `03_infra/tests/support/wiring/math_decoration_and_float_flush.rs`, owner `00_nucleo/prompts/infra/tests/support/wiring/math_decoration_and_float_flush.md`. Conservar em `04_wiring/tests/math_decoration_and_float_flush.rs` todos os cenários e observações finais. Importar somente Scratch, SvgTransform por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/math_decoration_and_float_flush.rs:16 struct_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:18 impl_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:31 impl_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:143 struct_item SvgTransform; 04_wiring/tests/math_decoration_and_float_flush.rs:152 impl_item SvgTransform.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.
