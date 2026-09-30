# L0 — compiler/eval/observation/value
Hash do Código: 7c1e388e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/value.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/value.rs:11` — `impl ObservationEq for crate::entities::value::Value {`.
- `01_core/src/compiler/eval/observation/value.rs:249` — `impl ObservationEq for crate::entities::path::VirtualRoot {`.
- `01_core/src/compiler/eval/observation/value.rs:265` — `impl ObservationEq for crate::entities::package_spec::PackageSpec {`.
- `01_core/src/compiler/eval/observation/value.rs:274` — `impl ObservationEq for crate::entities::package_spec::PackageVersion {`.

## Contract

Own the exhaustive Value match and runtime, selection, argument, diagnostic,
function and module carriers outside the other observation families. Compare
variant, presence, order and payload. Scope retains binding order and values.
Native functions compare call identity, name and namespace; closures are Same
only by Arc identity. Plugins and services lacking proof remain Unproven.
Diagnostics distinguish success, errors, spans and causal metadata. Never
execute a compared value. Signed zero and NaN retain their bit distinctions.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
