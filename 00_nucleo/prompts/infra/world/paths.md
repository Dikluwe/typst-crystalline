# L0 — infra/system-world/paths
Hash do Código: 0ec358ed

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/paths.rs`

## Current evidence

- `03_infra/src/world/paths.rs:19` — `root`.
- `03_infra/src/world/paths.rs:25` — `directory_of`.
- `03_infra/src/world/paths.rs:37` — `path_of`.
- `03_infra/src/world/paths.rs:41` — `realize_rooted`.
- `03_infra/src/world/paths.rs:56` — `resolve_physical_path`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/paths.rs`.

## Owned contract

- Own root/directory/path access, rooted realization, physical-path resolution, bibliography and read/include path helpers. Use the cache/package owners, not parallel state.
- Legacy absolute paths use the current package root or project root; relative paths use the current file's directory with existing root fallback for an unregistered id.
- Rooted paths preserve VirtualRoot::Project or full Package identity; lexical normalization stays in L1 DTOs. Unknown ids, bases outside root, escape and backslash keep their existing errors.
- Join the selected physical root and vpath with the existing prefix check. Do not add canonicalization or incidental sandbox hardening.
- Register canonical read/include ids and use shared caches. Preserve diagnostics including `searched at`, access denied and `include: ficheiro não encontrado`.
- Read bibliography via World, validate UTF-8 and delegate BibTeX parsing to L1, preserving error formatting and current-file relative base.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.


## Approved physical path error contract — approved 2026-10-22

Change public `path_of(&self, id: FileId)` to `FileResult<Option<PathBuf>>`.
A healthy slots registry returns Ok(Some(the original path)) for a known id
and Ok(None) for an unknown id; poison returns existing
`FileError::Other("source slots lock is poisoned".into())`.
Private directory/physical-resolution helpers become fallible and use the
existing String Result channels of World path/read/include operations. Preserve
true unknown-id root fallback, package/project selection, lexical path rules,
NotFound and valid diagnostics. Poison never means unknown id, root fallback or
missing file. Construct the known root VirtualPath through its fallible existing
API instead of an unwrap; propagate an unexpected failure without inventing a
path. No new World trait method, source re-evaluation or resource I/O is added.


Approval of this exact metadata correction is retained in `lab/results/world-metadata-errors-2026-10-22/approval.json`, review SHA-256 `31d632a54d40ca7df4139081c5297af774970a93b08cd605986b53e75e0ccc62`. The approved clauses supersede prior infallible signature/return preservation only for the two public APIs and the described private composition interfaces.
