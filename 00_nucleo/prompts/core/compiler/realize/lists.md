# L0 — realize/lists
Hash do Código: 0c04d67e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/realize/lists.rs`
**Governance:** ADR-0107, ADR-0109, ADR-0127, ADR-0129, ADR-0132.

## Contract

Own the formation of list containers and the preparation of their labels.

- Consecutive loose items of one kind form one container. Spaces and paragraph breaks
  between them are absorbed, nested sequences are looked into, and a styled item stays
  styled inside the container. Any other content, or an item of another kind, ends the
  group. The container is tight when no paragraph break was absorbed, and records it
  as its lexical `tight`. Leading and trailing breaks stay outside.
- Grouping is idempotent, keeps every item once and in order, and does not enter
  completed visuals when it applies show rules.
- A label that is a function, or content other than plain text, is produced once per
  item, with the styles of the container, the nesting level for markers and the number
  or numbers for numberings, and appended to the container.

## Acceptance

Unit controls for grouping and idempotence; CLI controls for loops, show rules and
function labels.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`.
