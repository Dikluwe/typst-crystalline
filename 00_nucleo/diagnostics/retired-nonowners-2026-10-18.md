# Retired non-owner specifications — 2026-10-18

## Evidence and disposition

The construction checkpoint is `8ec827fd975bf14cd849bebb9ede245e49033337`.
The [classification review](../../lab/results/mature-layout-2026-10-18/nonowner-classification.md)
records the source anchors and coverage limitations before retirement. The
[migration map](../../lab/results/mature-layout-2026-10-18/nonowner-migration.json)
records destinations. No new product API, behavior, or consumer was introduced.

Three process documents now live in `lab/tools/workflows`: module splitting
audit, L0 audit, and the L0 template. They guide work but do not legitimize code.
Five other documents are retained under
[construction/retired-l0](../../lab/history/construction/retired-l0):

| Former prompt | Status and current authority |
|---|---|
| `compiler/show-regex.md` | Historical materialization; its Style rejection and whole-node/no-splitting restrictions were superseded by R3-T1. Current data/eval owners are `00_nucleo/prompts/core/entities/show.md` and `00_nucleo/prompts/core/compiler/eval/rules/show.md`. |
| `compiler/stdlib/text/regex.md` | Historical redirect only. The current owner is `00_nucleo/prompts/core/compiler/stdlib/foundations/regex_constructor.md`. |
| `compiler/style/font-dict.md` | Historical broad specification. Parsing is owned by `00_nucleo/prompts/core/compiler/eval/font_dict.md`; its owner covers named/legacy forms, output dictionaries, diagnostic preservation, spread/empty rejection and current signatures. Exact historical literals and render representation remain listed in the audit obligations below. The old render-selection scope-outs are not new requirements for this parser. |
| `infra/package_version_resolution.md` | Non-owning reference. Initialization and remote lookup remain owned by `00_nucleo/prompts/infra/project_init.md` and `00_nucleo/prompts/infra/package_downloader.md`; package parsing belongs to `00_nucleo/prompts/core/entities/package_spec.md`. |
| `package-spec-dto.md` | Unimplemented proposal with no exclusive consumer; the complete proposed DTO, conversions, errors, and test obligations are retained. Retirement does not declare the proposal implemented or rejected. |

## Open obligations

The former font-dictionary specification records exact `text.font` representation
and older render-selection limitations. Its parsing owner preserves the current
consumer contract, but this organizational change does not certify current font
selection parity or decide those older render proposals. The classification
retains the exact diagnostic strings and scope-outs for a later substantive
owner audit; it must not be read as proof that those proposals are implemented.

The version-resolution reference preserves a previously recorded ownership/cache
investigation. Removing a redundant non-owner document does not resolve that
investigation. The active consumers keep their existing owners, including the
historical `shell/init.md` name for the L3 initialization consumer. Renaming that
owner by layer requires a separate, explicit mapping rather than a fictitious
new consumer.

The DTO proposal conflicts with later pure-parser ownership if interpreted as a
new L3 `FromStr` implementation. Keep serde outside L1; do not implement the old
proposal without a fresh decision and an actual L3 owner. The old entity prompt's
DTO path was an architectural intention, not evidence that the file exists.

No orphan exception is needed for these archived documents. Future implementation
must begin with an up-to-date materializable L0 owner and follow ADR-0127.
