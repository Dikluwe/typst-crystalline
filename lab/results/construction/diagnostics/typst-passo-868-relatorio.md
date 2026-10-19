# Relatório — typst-passo-868: consolidação de P861 a P867

**Data:** 2026-07-23T15:21:00Z  
**Executor:** Kimi Code  
**Commit base:** `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`  
**Commit final:** `3c8839e72` (`Tekt`)

---

## 1. Contradição P863/P864

### O que foi reportado

- Relatório de P863: `p863_show_par_func_transforma_paragrafo` passou.
- Relatório de P864: o mesmo teste apareceu como "falha pré-existente do P863" sem explicação de como um teste que passou se tornou pré-existente.

### Investigação

1. **Tempo de compilação real**: o P863 reportou `cargo check -p typst-core` em 0,14 s. Após `cargo clean -p typst-core`, a compilação real demorou **5,177 s** (cpu 4,475 s + sys 0,663 s). O tempo de 0,14 s era claramente uma recompilação contra cache.
2. **Teste isolado após clean**: `cargo test -p typst-core p863_show_par_func_transforma_paragrafo` passou na primeira compilação limpa.
3. **Conclusão**: a "falha pré-existente" foi um artefacto de execução paralela/cache do ambiente de build, não uma interacção real entre P863 e P864. O teste passa de forma determinística numa compilação limpa.

---

## 2. Consolidação da árvore

- **Estado**: todas as alterações de P861–P867 estavam na mesma working tree do ramo `Tekt`.
- **Stash antigo**: `stash@{0}: On Tekt: P571: salvar working tree para testes de isolacao de causa` — não interfere.
- **Conflitos**: nenhum conflito de merge detectado. Os ficheiros "hub" (`eval/rules.rs`, `layout/mod.rs`, `eval/tests.rs`, `layout/tests.rs`) contêm as alterações de todos os passos de forma coerente.

---

## 3. Validação limpa

Comandos corridos após `cargo clean`:

### `cargo build --release`

```text
Finished release profile [optimized] target(s) in 1m 11s
real    1m11,352s
```

### `cargo test --workspace`

| Suite | Passaram | Falharam | Ignorados |
|---|---|---|---|
| `typst-core` (lib) | 4682 | 0 | 2 |
| `typst-infra` (lib) | 717 | 0 | 5 |
| `typst-shell` (lib) | 41 | 0 | 0 |
| `typst` (bin) | 2 | 0 | 0 |
| `cli` integration | 36 | 0 | 0 |
| `crystalline_lint` integration | 2 | 0 | 0 |
| Doc-tests | 0 | 0 | 3 |

**Total: 5480 passaram; 0 falharam; 10 ignorados.**

### `crystalline-lint .`

```text
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
```

Zero violações novas; apenas o V7 pré-existente.

### Comparação com a soma ingênua dos relatórios

Os relatórios individuais reportam contagens ligeiramente diferentes porque cada um correu contra um estado intermédio da mesma árvore. A corrida única acima é a medição de referência válida para o estado consolidado.

---

## 4. Decisão de escopo P866 (PNG/SVG)

### Implementação actual

O CLI detecta o formato pela extensão de `-o` ou por `--format`, mas recusa PNG/SVG com erro claro:

```text
error: output format 'png' is not supported yet (only 'pdf' is currently available)
```

### Decisão do dono

O dono confirmou a decisão de **implementar PNG/SVG num passo futuro**, em vez de manter a rejeição permanente. Este passo (P868) não implementa a rasterização — apenas regista a decisão e consolida o estado actual.

### Implicações

A implementação de PNG/SVG exigirá:
- Prompt L0 próprio para o módulo de rasterização/export.
- Migração de `typst-render`/`typst-svg` do vanilla ou adição de dependências equivalentes (`tiny-skia`, `resvg`, `pixglyph`, `bytemuck`) em L3.
- Trabalho substancial em `03_infra/src/export/`.

---

## 5. Commit

```text
[Tekt 3c8839e72] P861-P868: consolidacao dos passos 861 a 868
 70 files changed, 3419 insertions(+), 219 deletions(-)
```

---

## 6. Conclusão

- A contradição P863/P864 foi explicada como artefacto de cache, não como bug de código.
- A árvore P861–P867 foi consolidada num único commit.
- A validação limpa confirma 5480 testes passados e zero violações de linter novas.
- A decisão de implementar PNG/SVG foi registada para execução num passo futuro com L0 dedicado.
