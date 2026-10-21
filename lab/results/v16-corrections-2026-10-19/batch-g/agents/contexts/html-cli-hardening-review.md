# Independent HTML verifier hardening review

The archived old runner/policy hashes exactly match the initial independent review. All16 source strings/digests, oracle payloads and target modes remain unchanged;13 required cases and3 explicit Link gaps are preserved.

Independently reran the actual AST-extracted acceptance fixtures: the archived verifier accepts6 invalid mutants; the hardened verifier passes all8 fixtures. Missing/wrong doctype, mandatory feature-warning loss, extra/changed warnings and error diagnostics with successful exit now fail. Only the exact previously measured deep-heading warning omission is waived.

The diagnostic field is now accurately called parsed_diagnostic_fields. Raw stderr remains retained; this does not certify every origin/trace/highlight/path. The known warning omission and Link gaps must remain visible in strict results. These fixtures validate the verifier, not the compiler: source-bound candidate CLI and structural/lint reviews remain separate gates. No productive or L0 edits and no Cargo were performed.
