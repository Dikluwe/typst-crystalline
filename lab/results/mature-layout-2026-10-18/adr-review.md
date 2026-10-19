# ADR organization and translation review

Review basis: checkpoint tag `tekt-construction-2026-10-18`; current files under `00_nucleo/adr/`; complete source→destination map `/tmp/adr-path-map.tsv`. This was a read-only review. No repository files were changed during this review and no build or test suite was run.

## Identity, status, catalog, and links

- Compared every one of the 116 mapped current ADR files with its checkpoint source path using `git show <tag>:<old-path>`; all 116 source/current pairs were available.
- All primary `**Status:**` values match the checkpoint exactly: ACCEPTED 68, IMPLEMENTED 44, PROPOSED 4. No status transition was introduced.
- Heading ADR numbers match the checkpoint. The two records sharing ADR-0026 remain distinct in the map and catalog: the base decision and its R1 revision.
- The canonical ORDER catalog contains 116 ADR entries for 116 numbered files. Every entry resolves once; there are no duplicate or omitted records. All numbered ADR filenames follow `adr-NNNN-short-english-title.md` (with `r1` retained in the ADR-0026 revision filename).
- Checked 125 Markdown links in the ADR directory; all 125 targets resolve in the current working tree, including links to the preserved construction-history/evidence locations.

## Normative review of translated ADRs

The following translations were compared with the Portuguese checkpoint text, including status, approval history, numbers, requirements, exceptions, alternatives, and references:

- ADR-0018: dependency authorization, parser replacement, L1 hashing policy, and Scope migration condition retained.
- ADR-0029: L1 purity definition, prohibited/allowed I/O tables, supersession of ADR-0028, typographic type inventory/exclusions, V13 examples, and deferred `Length` review retained.
- ADR-0030: RAM/I/O boundary, all L1/L3 examples, deep-clone versus `Arc::clone` rules, and string/array guidance retained.
- ADR-0032: all three `unsafe` classes and alternatives, temporary/permanent exception conditions, occurrence counts, debt plans, absolute prohibitions, and alternatives retained.
- ADR-0107: semantics/syntax/morphology distinction, examples, mechanical non-goals, vanilla-oracle condition, precedent reconciliation, and deferred `==` correction retained.
- ADR-0108: all six verifiable rules, residual risks, foundational self-check, consequences, and precedents retained.
- ADR-0109: atomization definition, four non-goals, Option B implementation form, Option A rejection reasons, ADR relationships, historical metric error, and application scope retained.
- ADR-0115: accepted status, framework, API, corpus sizes, ns/byte metric, reproducibility rule, alternatives, and consequences retained. The sole authorized decision change is the benchmark crate/corpus location under `lab/benches/`; no benchmark behavior or acceptance threshold changed.
- ADR-0119: four verification rules and all five typographic-output checks retained.
- ADR-0121: required commit/working-tree provenance and conditional timestamp requirement retained.
- ADR-0125: vanilla probe gate, deliberate-extension branch, rollback branch, and relationships to verification ADRs retained.
- ADR-0127: four mandatory-stop categories, four continuous-flow categories, historical application table, boundary distinction, and consequences retained.
- ADR-0129: owner-consumer bijection, nucleus schema/DAG/pins/cardinality, V15/V26 gates, migration categories, rollback, and owner approval retained.

ADR-0130 was already in English at the checkpoint; it was not counted as a new translation. The README names the records revised in English and explicitly says the whole corpus is not translated.

## Remaining Portuguese-prose count

Method: examined each of the 116 numbered ADR Markdown files. Before tokenization, removed fenced code blocks, inline-code spans, blockquote lines, and URLs. Tokenized the remaining prose with Unicode word boundaries and matched against a curated set of Portuguese lexical markers. Counted a file as containing Portuguese prose markers only when it had at least 5 marker occurrences and at least 3 distinct markers. The set intentionally excludes common English overlaps such as `a`, `as`, `de`, `do`, `no`, `actual`, and `se`.

Marker set used:

`não também está estão são será serão foram havia haver existem onde qualquer apenas somente pelo pela pelos pelas uma umas sua suas seu seus nesse nessa nisso desta deste destas destes próprio própria próprios próprias porque portanto desde após durante entre nunca deve devem deverá deverão pode podem poderiam seria seriam ficaria ficou feita feito feitos feitas decisão decisões seção consequência consequências regra regras escolha escolhido escolhida rejeitado rejeitada rejeitados autorizado autorizada projeto código removido removida implementar implementação passos passo ficheiro ficheiros pasta directório diretório relatório relatórios medida medição medições número números linha linhas fonte fontes saída entradas entrada valor valores trabalho mudança mudanças alterar alteração alterações usar utilizado utilização aplica aplicação aplicável vigente vigentes anterior anteriores seguinte seguintes actual atual atualmente completo completa completos completas verdadeiro verdadeira falso falsa ambos ambas dentro através contrário contrária aceite aceito aceita foram haver existir tornar toda todos todas nenhum nenhuma quaisquer cujo cuja cujos cujas outra outro outras outros então portanto neste nesta estes estas aquele aquela aqueles aquelas deverá deverão deve devem sendo feita dito dita tais enquanto embora antes depois posterior posteriormente urgente necessária necessário novo nova novos novas consiste consistem contém refere respectivo respectiva`

Result: **102 of 116 ADRs** meet this lexical criterion; the other 14 are the 13 ADRs listed above plus the pre-existing English ADR-0130. This is a reproducible marker count, not a claim that a language detector or human clause-by-clause review has certified every remaining document. It is also conservative for short Portuguese passages: a file below the 5-hit/3-distinct threshold is not counted even if a brief Portuguese fragment remains.

## Limitations and follow-up

- This review validates references and links inside `00_nucleo/adr`; repository-wide incoming references from prompts, source files, diagnostics, and other directories remain for the parent task’s integration audit.
- Historical narrative that states the old filename form (for example, `typst-adr-0125-*` in a retrospective slot scan) was retained as historical evidence. It is not a live catalog path.
- The 102 remaining records are still Portuguese-prose candidates by the defined method. They require clause-by-clause translation review before the migration can be described as complete.
- The new preserved history/evidence paths are expected to resolve under the parent’s mature-layout migration. Link resolution was checked against the current shared working tree at review time.
