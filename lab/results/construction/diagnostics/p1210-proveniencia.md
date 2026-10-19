# P1210 — proveniência reproduzível

**Início da medição:** 2026-08-26T10:47:25-03:00 (`America/Sao_Paulo`)  
**Estado reconfirmado:** 2026-08-26T10:51:50-03:00  
**HEAD cristalino:** `7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568`  
**Baseline vanilla ratificado:** `upstream/main a51e02804`

## Estado da árvore medido

- working tree não commitada;
- 706 ficheiros tracked modificados: 737 inserções e 699 remoções;
- digest SHA-256 de `git diff HEAD`: `d06adc2164f11cc6d4ada9094753ae5ab84f6560a31a2556a9cc035bf5182dab`;
- índice vazio (`git diff --cached --quiet` terminou com status 0);
- antes das saídas P1210: 6 ficheiros untracked, nominalmente os diagnósticos/passos P1207–P1210 já presentes;
- os artefatos P1210 são untracked e, portanto, não alteram o digest acima.

## Binários

| Papel | Caminho | SHA-256 | versão observada |
|---|---|---|---|
| cristalino | `target/release/typst` | `24162117e869c056884e3576921c7f8b3c687e5dbc62bd56b721b2178c8ec09d` | `typst 0.15.1 (45b54707)` |
| vanilla | `lab/typst-original/target/release/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | `typst 0.15.1 (e0e8ca4d)` |
| vanilla instalado | `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | idêntico ao binário do lab |

A string de versão não foi usada para provar a revisão. A identidade vanilla vem do pin ratificado e da igualdade SHA-256 dos dois binários de referência. O hash curto exibido pelo cristalino permaneceu antigo apesar do rebuild; por isso a proveniência usa HEAD, diff e SHA-256 do binário, não `--version` isoladamente.

## Ambiente

- `LC_ALL=C.UTF-8`, `LANG=pt_BR.UTF-8`, `LANGUAGE=pt_BR:pt:en`;
- timezone da medição: `America/Sao_Paulo` (`-03:00`);
- Rust `1.92.0`, Cargo `1.92.0`, Python `3.12.3`;
- Poppler `pdftotext 24.02.0`, `pdftoppm 26.05.0`, qpdf `11.9.0`;
- ferramentas externas presentes: `pdftotext`, `pdftoppm`, `pdfinfo`, `qpdf`.

## Comandos principais

```text
cargo build --workspace --release --quiet
cargo run --manifest-path lab/surface-inventory/Cargo.toml --release --quiet -- /tmp/p1210-crystalline.json /tmp/p1210-vanilla-historical.json
python3 lab/surface-inventory/run_probes.py /tmp/p1210-probes.json
python3 lab/surface-inventory/merge.py /tmp/p1210-vanilla-historical.json /tmp/p1210-crystalline.json /tmp/p1210-surface-merged.json --generated-at 2026-08-26T10:47:25-03:00
python3 -m unittest lab/parity/matrix/test_runner.py
python3 lab/parity/matrix/runner.py --validate-only
python3 lab/parity/matrix/runner.py --output /tmp/p1210-matrix-results.json
cargo test --manifest-path lab/parity/Cargo.toml --test parse_parity -- --nocapture
```

## Universos nominais

- superfície runtime cristalina: 1.151 entradas;
- probes binárias de superfície: 29 casos listados no JSON bruto;
- matriz diferencial declarada: 19 casos em `lab/parity/matrix/manifest.yaml`;
- P1: 50 testes, incluindo o corpus completo versionado;
- corpus vanilla amplo: não executável honestamente; não existe sampler determinístico P1–P4 no harness vigente;
- scope-outs: 420 ocorrências textuais em 112 L0s, listadas nominalmente no TSV.

## Digests das saídas

- `p1210-superficie-publica.json`: `8897c4d9058ef61428faa20345005f82a25aaa74206618b685a388954ef7e9a7`;
- `p1210-divergencias.tsv`: `c1a70cde09aa6fb54d9d6ee37afed39e9fa97fcf968b59eca58d28970b51c680`;
- `p1210-matriz-pipeline.tsv`: `898e325bb2551eeffcaa7265b0f8ab082f7e51c1bcb9354e29f7f690d2f8ee88`;
- `p1210-scope-outs.tsv`: `5ff2970b6308c7e8ec279d2b8b44e9e20c59f40b28819b1622c03d1ba9f68cad`;
- resultado bruto temporário da matriz: `91de0fc79a04ac6d05b3540e2e803d9b9093144ea1a08fdb024f9b950159c7d1`.

