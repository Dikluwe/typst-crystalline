# L0 — outline
Hash do Código: 1955655e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/outline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/outline.rs:23`, `01_core/src/compiler/layout/outline.rs:78`, `01_core/src/compiler/layout/outline.rs:192`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

For Figures and Tables targets retain the existing LoF/LoT paths, optional title or English default, numbered caption text and page suffix from the corresponding known-page vector by document-order index. For Headings, dispatch an available realized body rather than rebuilding entries; close a material exterior line before it and the realized terminal line after it, rearming the block boundary. The current consumer also retains a legacy fallback when no realized body exists: localized title, depth/active-indent filtering, stored heading number without supplement, cloned rich body and known label page, under the existing read-only flag. Do not remove this fallback under the guise of documentation. The private destination-link helper dispatches its body, requests fractional/repeat expansion through `00_nucleo/prompts/compiler/layout/line_commit.md` before closing the private capture scope; terminal geometry computes fragment bounds through `00_nucleo/prompts/compiler/layout/link.md`, preserving all copies and their internal destination. It does not implement repeat allocation itself. The realized-outline fail-closed proposal and the surviving fallback are a contract ambiguity requiring separate resolution.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

O helper de link interno usa o escopo privado comum. Preservar a expansão já exigida de fractions/repeats antes do fechamento do escopo e todos os destinos, cópias e regras de outline. Não mover emissão comitada para a linha pendente.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.
