# L0 — stdlib/numbering
Hash do Código: ef544b5b

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/numbering.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/numbering.rs:22`, `01_core/src/compiler/stdlib/numbering.rs:55`, `01_core/src/compiler/stdlib/numbering.rs:134` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_numbering, format_pattern, application of entities::numbering::Numbering and conversion of its result to Content. numbering(pattern, ..numbers) requires a pattern and nonnegative integer numbers; wrong types/negative numbers fail. String patterns recognize 1, a, A, i, I, א and ①, preserving prefixes, numeral pieces and suffix. Drop unused pattern pieces; when numbers exceed pieces repeat the last piece using its prefix, or its suffix when its prefix is empty. Patterns without a counting symbol fail.

Hebrew zero warns `the numeral system \`hebrew\` cannot represent zero` and emits decimal 0. Circled zero is ⓪; 1–50 use ①–㊿; larger values warn `the number {n} is too large to be represented with the \`arabic.o\` numeral system` and fall back to decimal. Preserve alphabetic, Roman and Hebrew formatting, including 15 -> טו.

Func patterns invoke apply_func once with numbers as positionals and return any resulting Value; content realization uses normal markup morphology (42 displays 42; none is empty). format_pattern is shared with counter.display, including circled numerals. realize_numbering is exposed for L3 orchestration; layouter/PageStore must not receive Engine or execute callbacks, and consumers must not duplicate Pattern/Func dispatch.

Calculated usize/u32-to-Int callback arguments use Args::from_parts with empty named and absent occurrences; retain aggregate span, order and callback count without invented lexical origins. Do not incidentally fix the existing u32 conversion or claim general diagnostic parity for synthetic callbacks. Preserve existing public helper visibility/signatures.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/stdlib/numbering.rs` nas linhas 168 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `00_nucleo/diagnosticos/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Percorrer os pares prefix/symbol sem enumerar um índice nunca usado. Preservar ordem, consumo de numbers_iter, chamadas ao formatador, erros e interrupção quando faltam números. A emissão de apóstrofo por push(char) preserva o mesmo texto.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.
