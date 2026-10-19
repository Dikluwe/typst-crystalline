# L0 — Page marginal domain
Hash do Código: 59ce63d3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_running.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-running.toml sha256:88345b9a32ff8678a2ff236d21cbd7c18913820ed151fc704c116ab2542288ed

## Evidence before specification

Historical upstream `a51e02804` evidence is
`typst-library/src/layout/page.rs:315-450` and
`typst-layout/src/pages/run.rs:141-233`, not a current-pin parity probe.

## Owned contract

`PageMarginal` preserves Auto, None and explicit Content, including empty
Content; default is Auto. Auto permits automatic numbering only when numbering
exists and vertical alignment selects this marginal. None suppresses it;
explicit Content takes precedence over automatic numbering in that marginal.
An omitted delta retains the previous value, distinct from all three states.
`PageNumberAlign` carries horizontal alignment and vertical Top/Bottom via
`PageNumberVAlign`; default is center plus bottom. Horizon is not a valid
vertical language value; casting/diagnostics belong to evaluation.
`PageMarginalOffset` is `Rel<Length>`; both offsets default to 30% with zero
absolute component. Header ascent uses the physical top margin and footer
descent bottom; add the absolute component without an empirical constant.
These types remain pure L1 and do not execute callbacks or inspect targets.

## Ownership and delegation

The pinned nucleus carries shared page-running invariants. Composition and rich
Content layout belong to `00_nucleo/prompts/compiler/layout/page_running.md`.
PageConfig belongs to `00_nucleo/prompts/entities/layout_types.md`, Content
transport to `00_nucleo/prompts/entities/content.md`, page-run data to
`00_nucleo/prompts/entities/elements/page_run.md`, numbering to
`00_nucleo/prompts/entities/numbering.md`, and supplement to
`00_nucleo/prompts/entities/page_supplement.md`.
No public page constructor is authorized here. Verify default alignment,
offsets and marginal distinctions. Historical composition sequences and
cross-owner obligations are retained in the diagnostic, not redefined here.

## Default derivado com valor preservado — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/page_running.rs:41` contém a implementação manual de Default sinalizada pelo Clippy. O corpo existente retorna a mesma variante ou inicializa cada campo pelo seu default. O tipo não é genérico, portanto a derivação não acrescenta bounds a parâmetros públicos. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, spans, UTC, status e diffstat estão em `lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`. É evidência de implementação, não nova medição de paridade ou desempenho.

### Decisão e aceitação

Substituir somente a implementação manual de Default por derive(Default), conservando o trait já implementado e seu valor. PageMarginal mantém Auto, anotada com #[default]. PageNumberAlign::default continua manual e center/bottom; não derivá-lo nem alterar offsets, presença ou marginais. Os exemplos anteriores de impl Default explicitam o valor exigido; a derivação equivalente passa a ser a forma desta implementação.

Não alterar assinatura pública, campos, variantes, comportamento padrão do produto ou fase. Correção mecânica interna em fluxo contínuo ADR-0127. Preservar testes e expectativas; validar todos os targets do workspace, suíte core, registro de testes, build, rustdoc, Clippy e linhagem.
