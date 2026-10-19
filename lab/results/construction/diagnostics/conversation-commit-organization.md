# Conversation work: organized commit index

Conversation corrections and evidence are recorded by topic. Existing changes from other work streams remain preserved and uncommitted. This is not a global sanitization completion claim. No push was performed.

| Commit | Scope |
| --- | --- |
| `3dee39ead07f359fcfaeff2e35d16ea12068cb9e` | Infra cache identity, registration and metadata |
| `0fea906e176acf93ae6301e4cd17ace6aa4fe349` | Matching font axes and instancer gate |
| `392c4ef0730973efb82380982bc371bd3253640c` | Shaping language and cache identity |
| `f1ff689fe4f3c26d102011136d77b11d6e00d17f` | Initial nontext blocks and empty flush |
| `7cd0213420bd14ccc9c47176854a43e943da7af2` | Unused subframe cap-height requests |
| `08812c0f01965209a3adc5f8d859d0df0a2871e7` | Benchmark guards and export classification |
| `fc8e82abb98e68267c593079a9deaceaa1b8cbea` | Lazy per-world font catalog |
| `a58dfe5a6c29bf9836a902cfa2e9741d5ff2d149` | Pinned vanilla font measurements |
| `a9dfed1dcc10b8ee28bb0d5961e2fa4758095947` | Shaper composition, reuse and fallback |
| `d2540371f14f3243d556886dd8e9b1b235881459` | Performance baseline and phase investigation |

Font lifecycle and shaper commits include current owner/consumer files and private descendants required by composition. Previously untracked files retain their full context. Other atomization and documentation changes remain in the working tree; this operation does not establish a clean checkout build.

All sixteen dataset manifests and accompanying report digests were verified before and after organization. Manifest-listed raw outputs, fixtures, scripts, executables and receipts are tracked. Historical measured HEADs and working-tree inventories remain unchanged. `conversation-commit-organization/commits.json` records exact staged paths for the five archive/composition commits; Git trees record all correction paths. Ignored Python bytecode caches remain excluded.

The latest recorded checks include 938 passing layout tests (`subframe-font-demand-followup/completion-audit.json`) and 57 passing shaper tests (`shaper-language-fix/`), plus builds and lint with existing warnings. These checks belong to their recorded working-tree states. Benchmark compilation and guards are documented in `benchmark-suite-review.md`; no new measurements were taken for committing.

Vanilla remains pinned to `9f2b6e8715237cb086899a42873660fe744622e8`. Reports retain distributions, stability failures and attribution limits. Archiving does not promote exploratory samples into stable results or make different microbenchmark fixtures additive.

Ordinary image and shape subframes no longer request unused cap-height. Initial font catalog acquisition through the shared subframe/caller path remains a follow-up candidate. No new latency improvement or post-fix accessor count is claimed.

`conversation-commit-organization/remaining-status.txt` records remaining work before committing this index. Source fingerprints outside the latest correction's four files were preserved.
