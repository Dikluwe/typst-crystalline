# Canonical ADR reading order

This file orders the active ADR corpus by authority and architectural reach.
It does not renumber decisions. Every numbered file in 00_nucleo/adr/ must
appear exactly once below. Revoked, superseded, and misclassified records are
listed through the archive manifest instead. Classification uses the canonical
English primary statuses defined in README.md; historical qualifiers do not
promote a decision. ADR-0130 governs L0 references: use full repository-relative
paths including `00_nucleo/prompts/` in prose. Rust `@prompt` remains unchanged.

## 1. Constitutional constraints

- [ADR-0129](typst-adr-0129-nucleos-tekt-l0-compartilhado.md)
- [ADR-0130](typst-adr-0130-referencias-canonicas-l0.md)
- [ADR-0127](typst-adr-0127-gate-l0-paragem-vs-fluxo.md)
- [ADR-0029](typst-adr-0029-pureza-fisica-revoga-adr-0028.md)
- [ADR-0030](typst-adr-0030-performance-dominio-l1.md)
- [ADR-0018](typst-adr-0018-rustc-hash.md)
- [ADR-0032](typst-adr-0032-unsafe-em-l1.md)
- [ADR-0107](typst-adr-0107-paridade-linguagem-nao-mecanica.md)
- [ADR-0033](typst-adr-0033-paridade-funcional-vanilla.md)
- [ADR-0108](typst-adr-0108-disciplina-anti-deriva.md)
- [ADR-0109](typst-adr-0109-atomizacao.md)
- [ADR-0037](typst-adr-0037-coesao-por-dominio.md)
- [ADR-0100](typst-adr-0100-coesao-por-dominio-l3.md)
- [ADR-0104](typst-adr-0104-atomicidade-para-agentes.md)
- [ADR-0119](typst-adr-0119-disciplina-verificacao.md)
- [ADR-0121](typst-adr-0121-proveniencia-medicao.md)
- [ADR-0125](typst-adr-0125-implementacao-vs-linguagem.md)

## 2. Evidence and decision process

- [ADR-0034](typst-adr-0034-diagnostico-tipos-vanilla.md)
- [ADR-0065](typst-adr-0065-inventariar-primeiro.md)
- [ADR-0084](typst-adr-0084-auditoria-condicional.md)
- [ADR-0085](typst-adr-0085-diagnostico-imutavel.md)
- [ADR-0114](typst-adr-0114-sonda-antes-da-spec.md)
- [ADR-0117](typst-adr-0117-sonda-a0-mecanismo.md)
- [ADR-0122](typst-adr-0122-paridade-defeitos-testes.md)
- [ADR-0124](typst-adr-0124-checklist-sublayouts.md)
- [ADR-0093](typst-adr-0093-meta-metodologia-evolucao-adrs.md)
- [ADR-0094](typst-adr-0094-meta-operacional-specs.md)
- [ADR-0098](typst-adr-0098-single-source-of-truth-invariante-anti-bug.md)
- [ADR-0113](typst-adr-0113-stub-transparente-vs-fallback-software.md)
- [ADR-0080](typst-adr-0080-l0-minimal-para-refactors.md)

## 3. Core domain and representation

- [ADR-0001](typst-adr-0001-estrategia-migracao.md)
- [ADR-0005](typst-adr-0005-packagespec-world.md)
- [ADR-0009](typst-adr-0009-math-class.md)
- [ADR-0024](typst-adr-0024-ecow-value.md)
- [ADR-0025](typst-adr-0025-int-eq-float.md)
- [ADR-0026](typst-adr-0026-content-divergencia.md)
- [ADR-0026-R1](typst-adr-0026-R1-content-arc.md)
- [ADR-0031](typst-adr-0031-early-hashing-source.md)
- [ADR-0036](typst-adr-0036-atomizacao-progressiva.md)
- [ADR-0038](typst-adr-0038-sistema-estilos-l1.md)
- [ADR-0039](typst-adr-0039-frameitem-style.md)
- [ADR-0052](typst-adr-0052-lang-tipo-semantico.md)
- [ADR-0053](typst-adr-0053-font-tipo-composto.md)
- [ADR-0058](typst-adr-0058-value-type-simplificado.md)
- [ADR-0059](typst-adr-0059-args-tipo-separado.md)
- [ADR-0064](typst-adr-0064-smart-para-option-default.md)
- [ADR-0101](typst-adr-0101-ieee754-restricao-stdlib.md)
- [ADR-0102](typst-adr-0102-math-style-mechanism.md)
- [ADR-0103](typst-adr-0103-math-style-composition.md)
- [ADR-0105](typst-adr-0105-modelo-elemento-d-agora-f-destino.md)
- [ADR-0106](typst-adr-0106-fronteira-de-extensao-e1.md)
- [ADR-0112](typst-adr-0112-rust-decimal-l1.md)

## 4. Compiler runtime and pipeline

- [ADR-0017](typst-adr-0017-adiamento-eval-typst-library.md)
- [ADR-0040](typst-adr-0040-activacao-set-rule.md)
- [ADR-0041](typst-adr-0041-activacao-show-rule.md)
- [ADR-0042](typst-adr-0042-sink-materializado.md)
- [ADR-0043](typst-adr-0043-canal-sink-saida.md)
- [ADR-0044](typst-adr-0044-engine-agregador.md)
- [ADR-0068](typst-adr-0068-location-aware-layouter.md)
- [ADR-0069](typst-adr-0069-post-recursion-tag-emission.md)
- [ADR-0070](typst-adr-0070-eliminacao-counter-state-legacy.md)
- [ADR-0071](typst-adr-0071-walk-pipeline-redesign.md)
- [ADR-0072](typst-adr-0072-m7-fixpoint-runtime-fechado.md)
- [ADR-0073](typst-adr-0073-comemo-introspector.md)
- [ADR-0074](typst-adr-0074-f3-layouter-substores-trackable.md)
- [ADR-0076](typst-adr-0076-introspector-completion.md)
- [ADR-0099](typst-adr-0099-activacao-posterior-feature-graded.md)
- [ADR-0120](typst-adr-0120-textshaped-rustybuzz.md)

## 5. Layout, infrastructure, and export

- [ADR-0019](typst-adr-0019-ttf-rustybuzz.md)
- [ADR-0020](typst-adr-0020-fontdb.md)
- [ADR-0022](typst-adr-0022-fontbook.md)
- [ADR-0027](typst-adr-0027-cidfont-subsetting.md)
- [ADR-0054](typst-adr-0054-criterio-fecho-debt-1.md)
- [ADR-0055](typst-adr-0055-font-consumer-cidfont.md)
- [ADR-0057](typst-adr-0057-lang-hyphenation.md)
- [ADR-0075](typst-adr-0075-vanilla-integration.md)
- [ADR-0078](typst-adr-0078-column-flow-algorithm.md)
- [ADR-0082](typst-adr-0082-promocoes-reais-scope-outs-graded.md)
- [ADR-0083](typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md)
- [ADR-0086](typst-adr-0086-paint-wrapper-solid-only.md)
- [ADR-0087](typst-adr-0087-gradient-linear-only.md)
- [ADR-0088](typst-adr-0088-gradient-radial-only.md)
- [ADR-0089](typst-adr-0089-gradient-conic-only.md)
- [ADR-0091](typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md)
- [ADR-0092](typst-adr-0092-conic-coons-patches-rgb-cmyk.md)
- [ADR-0095](typst-adr-0095-dedup-arc-as-ptr-resources.md)
- [ADR-0115](typst-adr-0115-infra-benchmark-scanner.md)
- [ADR-0116](typst-adr-0116-excecao-get-unchecked-scanner.md)
- [ADR-0126](typst-adr-0126-modo-verboso-primeiro.md)
- [ADR-0128](typst-adr-0128-html-target-semantico.md)

## 6. Shell, CLI, and diagnostics

- [ADR-0045](typst-adr-0045-formato-diagnosticos.md)
- [ADR-0046](typst-adr-0046-cli-minima.md)
- [ADR-0047](typst-adr-0047-cli-clap.md)
- [ADR-0048](typst-adr-0048-diagnosticos-com-cor.md)
- [ADR-0049](typst-adr-0049-cli-em-l2.md)
- [ADR-0050](typst-adr-0050-formatter-em-l2.md)
- [ADR-0051](typst-adr-0051-flags-funcionais.md)

## 7. Parsing and dependency decisions

- [ADR-0006](typst-adr-0006-typst-timing.md)
- [ADR-0008](typst-adr-0008-defer-inline.md)
- [ADR-0010](typst-adr-0010-unicode-ident.md)
- [ADR-0011](typst-adr-0011-unicode-math-class.md)
- [ADR-0012](typst-adr-0012-unicode-script.md)
- [ADR-0013](typst-adr-0013-unicode-segmentation.md)
- [ADR-0014](typst-adr-0014-unscanny.md)
- [ADR-0015](typst-adr-0015-ecow.md)
- [ADR-0016](typst-adr-0016-lazyhash.md)
- [ADR-0021](typst-adr-0021-datetime.md)
- [ADR-0023](typst-adr-0023-indexmap.md)
- [ADR-0035](typst-adr-0035-ecovec-autorizacao.md)
- [ADR-0062](typst-adr-0062-hayagriva-bibliography-parsing.md)
- [ADR-0077](typst-adr-0077-regex-l1.md)
- [ADR-0111](typst-adr-0111-crates-parsing-loading.md)

## 8. Proposed or pending decisions

- [ADR-0067](typst-adr-0067-attribute-grammar-scoping.md)
- [ADR-0110](typst-adr-0110-marcador-vanilla.md)
- [ADR-0118](typst-adr-0118-runtime-state-context.md)
- [ADR-0123](typst-adr-0123-geometria-tipografica.md)

## Historical records

Historical records and redirects are indexed in
../diagnosticos/adr-archive/redirects.tsv. They are evidence, not current
authority.
