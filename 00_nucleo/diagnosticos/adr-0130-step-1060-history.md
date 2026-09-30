# ADR-0130 — Step 1060 inventory and historical context

## Provenance and interpretation

This diagnostic preserves the inventory removed from the translated ADR-0130.
It is historical evidence, not a new measurement or a current requirement.
The original report is `00_nucleo/diagnosticos/typst-passo-1060-relatorio.md`,
section 2. Its 7-occurrence directory list differs from the summary reproduced
below; that discrepancy is retained rather than silently rewriting history.
No current conclusion relies on the historical counts.

The ADR was originally approved by the owner in Step 1062 under ADR-0127,
Option (b), explicit full paths. Its former prose linked this to Step 1054's
preference for explicit canonical references over ambiguous short references.
The former introductory paragraph also cited seven `mod.md` files and
shared `layout.md`, `heading.md`, `footnote.md`, and `bibliography.md` names
across `compiler/`, `entities/`, and `infra/`. These were contextual claims,
not a current inventory; the current ADR uses only existing examples.

The former Markdown rule described paths relative to `00_nucleo/prompts/`,
while Rust annotations were already repository-relative. Reconciliation adopts
the explicit repository-relative form for prose and leaves Rust syntax intact.
Former prose examples included `entities/layout.md` and
`compiler/eval/mod.md`, which do not exist at the reconciliation checkpoint.
They are recorded here as historical strings, not canonical current examples.

## Inventory formerly embedded in ADR-0130 (verbatim)

The Step 1060 inventory catalogued 35 filenames repeated across directories, totalling 83 files. When citing any of these files, attention to the full path is critical:

1. **7 occurrences**: `mod.md` (`compiler/`, `compiler/eval/`, `entities/`, `entities/ast/`, `entities/types/`, `infra/`, `contracts/`).
2. **3 occurrences** (8 names, 24 files):
   - `layout.md` (`compiler/`, `entities/`, `infra/`)
   - `heading.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `footnote.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `bibliography.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `table.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `label.md` (`compiler/`, `entities/`, `infra/`)
   - `color.md` (`compiler/`, `entities/`, `infra/`)
   - `_comum.md` (`compiler/`, `entities/`, `infra/`)
3. **2 occurrences** (26 names, 52 files):
   - `page.md`, `math.md`, `stroke.md`, `image.md`, `state.md`, `counter.md`, `context.md`, `shape.md`, `text.md`, `align.md`, etc.
