# Informational diagnostic review

The full pinned scan at `2026-10-19T11:58:06.809650+00:00` changes V16 errors
from 91 to 83, V19 notes from 1,157 to 1,166, and retains 878 V20 notes.
`lint-state.json.gz` identifies the exact uncommitted source/L0 state and command.
`lint-delta.json` preserves the full positional difference;
`note-content-delta.json` compares rule, level, file and message without positions.

There are nine added V19 messages and no removed informational message by
substance. Nine prior notes move with their source positions. Outside the eight
selected consumers, even note positions remain identical. No error or warning
other than the selected eight V16 findings disappears, and none is introduced.

| Decision | Added grouping | Rationale and controls |
| --- | --- | --- |
| find_cases, find_frac, find_mat | One nominal miss group each | Existing fixture paths retain direct/terminal target, allowed wrappers, first-hit ordering and empty cases. Strong/MathStyled decoys stay opaque. The groups do not assert semantic emptiness. |
| find_root | One nominal miss group | Same preservation controls, with Styled and MathDelimited explicitly remaining outside this narrower observer. |
| collect_custom_into | One opaque group | Retain Styled precedence and Sequence ordering. A hidden Styled decoy contributes nothing, while the following supported sibling remains observable. No claim of global style absence. |
| bottom-up test callback | One no-replacement group | Ok(None) retains the processed node and descendants. Strong still sees transformed Text; Emph, Space, sequence shape and sibling ordering remain checked. |
| same_storage | Two tuple groups | Preserve all 34 positive Arc comparisons. Unqualified-left pairs return false; qualified-left residual pairs return false after same-kind arms. Both axes name the whole relevant current domain. Self/clone, unlike fixture pairs, unqualified shared Sequence, changed parents and unchanged children are checked. |
| fixture_equation_bodies | One opaque group | Actual fixture paths justify the bounded observer. Direct/terminal equation, ordered sequence/style traversal and semantic/causal decoys preserve the boundary and original real-font geometry assertions. |

Each group has one shared outcome because its variants have the same policy at
that specific decision. Splitting identical bodies to hide V19 would not add
coverage or improve the contract. V19 remains visible: neither one executed arm
nor the written alternative count proves that every payload/variant ran.
In particular, tuple alternative counts are syntactic, not pair coverage counts.

The runtime controls cover the stated boundaries and independent witnesses;
nominal inventory audits and attributable compiler-negative experiments address
closed-domain evolution separately. They are not a claim of exhaustive runtime
coverage of every Content payload or complete language parity. No linter rule,
exception, severity, threshold, scan scope, configuration or Nucleus was changed.
