# L0 — Isolated projected-buffer allocation regressions
Hash do Código: 7c38c3c8

**Layer:** L3 (integration test executable)
**Exclusive consumer:** `03_infra/tests/projected_buffer_reserve.rs`

## Evidence before organization

`03_infra/src/export/images/scan.rs` already installs a global allocator in the
library's Linux test executable. The C23 lab's allocation-lineage/failure controls
therefore require their own executable, not another library global allocator.
`00_nucleo/prompts/infra/fontdb/selective.md` specifies the private forwarding hook.

## Contract

Compile the actual selective source as a module and use only its cfg(test)
project_for_test facade. Forward metadata extraction to typst_infra and include the
real coverage adapter, forwarding its result; no algorithm stub or public library
API. Provide a local crate::fontdb forwarding module for child parent-loader tests.
Reuse repository fixtures and portable temporary paths. Retain the included real
adapter/source regressions; do not replace them with copied productive logic.

Install only this executable's allocator from the separately owned support module
at `00_nucleo/prompts/infra/tests/support/projected_buffer_alloc.md`. Recover the
returned output pointer's allocation lifetime: the original 12-byte header must
be reused with at most one realloc. Test each injected reservation failure yields
None without partial success, and an over-limit total is rejected before a large
reservation. Do not count temporary Reader/plan allocations as output growth.
Serialize tracing windows or group them in one test so ordinary parallel cargo
test cannot race the trace state. Other test threads must never receive injected
failures. No production flags, dependencies, observers, time thresholds or lab
absolute paths. All source/allocator code here is test-only.
