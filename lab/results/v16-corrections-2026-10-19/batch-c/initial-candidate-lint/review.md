# Rejected candidate scan

The pinned full scan removed exactly the nineteen selected V16 findings, but
introduced V14 for a direct regex::escape call in the selector adapter. This
candidate is rejected; no allowlist, severity, rule or suppression change is
permitted. Replace the forbidden direct external dependency with pure literal
escaping specified by the owning L0 and preserve language behavior with controls.
Full exact state, commands, UTC and raw diagnostics are retained here.
