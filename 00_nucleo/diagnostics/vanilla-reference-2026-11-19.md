# Upstream vanilla reference selected on 2026-11-19

The upstream default branch is `main`. Its selected commit is
`a6dcd82927b2a44bb089bc9151e29566e84e7d5a`, with tree
`20464de0800655e5b1b4f796f4d5aa912c683517`.
[Official upstream commit](https://github.com/typst/typst/commit/a6dcd82927b2a44bb089bc9151e29566e84e7d5a).

The delta from `64c5e8c02fe0394d913ead36e42ef8c1334d2107` covers 96 paths.
The former pin and its measurements remain documented in the local
`lab/results/construction/diagnostics/vanilla-resync-2026-10-26/relatorio.md`.

Use [the oracle manifest](../../lab/parity/vanilla.json) for the current
commit, tree and verified local binary identity. CI builds the pinned
upstream source independently and records the resulting binary identity.
The laboratory vendor tree carries the local inventory extractor and its
one-entry lockfile overlay. The official oracle build uses the clean
upstream lockfile.

The local binary SHA-256 is
`e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`.
The build used Rust 1.92.0, `--release --locked -p typst-cli`, and one
Cargo job with a 12 GiB address-space limit. The verified binary printed
`typst 0.15.1 (a6dcd829)` and evaluated `sys.version` to
`version(0, 15, 1)`.

The prior local binary is preserved as
`lab/typst-original/target/release/typst-64c5e8c0-2026-10-26`.
The new build receipt, vendor verification, matrix outputs and issue
probe are in `lab/results/construction/diagnostics/vanilla-resync-2026-11-19/`.
