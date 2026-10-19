> Pre-retirement review snapshot. Final dispositions and open obligations are in
> `00_nucleo/diagnostics/retired-nonowners-2026-10-18.md`.

# Ownerless prompt classification (2026-10-18)

Read-only classification of the five non-procedural files from the inventory. No source prompt, owner, code, or `crystalline.toml` entry was edited in this classification. Line references below were checked against the current worktree.

## `00_nucleo/prompts/compiler/show-regex.md`

**Disposition:** Retire from active L0 only after checking and updating active references; preserve the full document as historical materialization evidence. It is marked “MATERIALIZADO — P393 + confirmado P473 + P474” at its end. `00_nucleo/prompts/entities/show.md:9-11` says its ratified R3-T1 section supersedes historical Text/Regex Style restrictions, node-by-node matching, and eager realization. The current data contract says `Transformation::Style` is valid for Text and Regex (`entities/show.md:273-288`), and the current eval owner accepts Style for literal/regex selectors and delegates matching to the matcher (`compiler/eval/rules/show.md:272-274`). Therefore the old whole-node transformation, no-splitting scope-out, and Style-rejection clauses are stale, not live requirements.

The historical prompt’s remaining useful context is the original Regex constructor/wiring scope and old test evidence; these do not confer current ownership. Its scope also says no layout/render and no non-text application. Verify whether any still-live behavioral requirement is missing from current owners before archival; do not import the old restrictions into the current contract.

## `00_nucleo/prompts/compiler/stdlib/text/regex.md`

**Disposition:** Retire from active L0 and preserve its redirect/history. The document explicitly identifies itself as a historical redirect with no productive consumer or behavior and points to `compiler/stdlib/foundations/regex.md` / the `regex_constructor.rs` consumer. Its `crystalline.toml` exception at line 214 describes it as a historical redirect. It contains no independent live normative requirement to retain as an owner.

## `00_nucleo/prompts/compiler/style/font-dict.md`

**Disposition:** Candidate for retirement from active L0 after confirming exact legacy diagnostics/representation against the owner and preserving any missing material as history or transferring it under authorized ownership. `compiler/eval/font_dict.md:10-14,18-34` owns the active consumer and covers named/legacy parsing, accepted types, regex-call keys, error diagnostics, empty dict/spread rejection, evaluation order, fallback behavior, and signatures/visibility.

The old prompt still records specific details that should be checked before retiring it: the custom `StyleChain` channel `"text.font"` as `Array[Dict]`; zero new `Value` types; exact legacy errors (“font dict key must be string or regex”, “font dict value must be string or array of strings”, “font variants must be strings”, “font dict must not be empty”); first-regex-match behavior; and explicit scope-outs for variant-aware selection, regex literal optimization, named-field `stretch`, and dict spread. Current owner mentions preservation of diagnostic literals but does not spell out those exact strings, channel representation, or all scope-outs. The old prompt is marked as integrated in `rules.rs` by `crystalline.toml:197`; active implementation moved to `compiler/eval/font_dict.rs` per the owner.

## `00_nucleo/prompts/infra/package_version_resolution.md`

**Disposition:** It is a reference with unresolved ownership, not an owner. Preserve its ownership-gap statement/history. Its live version-selection contract is covered by `shell/init.md:28,79` (explicit local roots first; omitted preview uses remote index, not local cache; non-preview selects the greatest parseable data-root version and ignores malformed names; tests vary preview cache contents) and `infra/package_downloader.md:32` (index parsing, exact name filter, valid three-component numerical max, absence vs error, no per-World cache, init call path). The reference adds the useful constraint that package versions have three unsigned components, prereleases are invalid, malformed local entries are skipped silently, and import syntax still requires a version; verify that these constraints remain represented in their actual owners or history.

Do not remove it until the canonical active link at `shell/init.md:11` is changed by the parent to a retained diagnostic/history target. Current `crystalline.toml:217` describes a future L3 resolution specification. It records no independent consumer and must not be converted into a fictitious owner.

## `00_nucleo/prompts/package-spec-dto.md`

**Disposition:** Unmaterialized normative proposal; do not delete or claim implemented. Preserve its whole API/obligations as an unresolved historical proposal unless a later authorized task establishes an L3 owner and consumer. The file specifies a `PackageSpecDto` with `namespace`, `name`, and string `version`; `TryFrom<PackageSpecDto> for PackageSpec` with typed `InvalidVersion` / `InvalidFormat`; `FromStr` for `PackageSpec` and `VersionlessPackageSpec` using `unscanny`; serde/unscanny confined to L3; and six DTO/parsing/isolation verification cases. `crystalline.toml:206` says it materializes in L3 but the `typst-infra` crate does not yet exist.

The active L1 entity prompt says serde must never enter L1 and describes serialization in L3 through the DTO pattern (`entities/package-spec.md:49-51`), but does not supply an L3 consumer or own the DTO / `TryFrom` / `FromStr` implementation. Its historical generated path `03_infra/dto/package_spec_dto.rs` is not evidence of an existing consumer. Preserve the separation-of-layer obligation and the proposed API without inferring current implementation status.
