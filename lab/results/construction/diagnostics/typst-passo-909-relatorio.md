# Relatório — Passo 909: atomizar `layout_accent`/`layout_cancel`/`layout_underover`/`layout_op`

**Data:** 2026-07-25
**Commit de partida:** `5cfb12430` (P908)
**Estado da medição:** working tree não commitado no momento de todas as medições abaixo
(`git diff HEAD --stat` — ver Resumo por item).

---

## Resumo executivo

Refactor puro (ADR-0107, content-preserving), replicando o padrão já aplicado a
`frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/`delimited` em P314 (ADR-0104): os quatro
handlers `layout_accent`/`layout_cancel`/`layout_underover`/`layout_op` — adicionados em P296-298,
depois de P314, e nunca movidos — foram fatiados de `math/layout/mod.rs` para arquivos próprios
(`accent.rs`/`cancel.rs`/`underover.rs`/`op.rs`), forma B (ADR-0109): `impl MathLayouter` dividido
entre arquivos, arms de despacho em `mod.rs` inalterados (já eram magros — `self.layout_x(...)`).

## Fase A — decisões registadas antes da Fase B

1. **Guard partilhado `layout_stretchy_or_node`** (usado por `layout_accent` e `layout_underover`):
   **fica em `mod.rs`**, promovido de `fn` privado a `pub(super) fn` — não duplicado nos dois
   arquivos novos. Mesmo tratamento já dado a outros helpers partilhados entre submódulos
   (`layout_stretchy_delimiter`, `apply_axis_offset`, `layout_grid_rows`). Decisão registada em
   `_comum.md` §P906/P909 e em `accent.md`/`underover.md` §P906.
2. **`layout_op`/`MathAttach` (cross-variant interaction)**: confirmado que a lógica real de P298
   (`limits: true` força empilhamento incondicional de scripts) vive em `attach.rs:78,143`, não em
   `layout_op` (delegate puro). Documentado explicitamente em `op.md` para não se perder de vista
   por estar fisicamente noutro arquivo.

## Fase B — implementação

- 4 arquivos novos: `accent.rs` (69 linhas), `cancel.rs` (46), `underover.rs` (95), `op.rs` (26) —
  lógica movida (não reescrita) do `mod.rs` anterior.
- 4 L0s novos: `accent.md`/`cancel.md`/`underover.md`/`op.md`, mesmo formato de `frac.md`/`attach.md`.
  Conteúdo das duas secções P906 de `_comum.md` (esticamento horizontal + correcção de convenção
  baseline-relativa) migrado para `accent.md`/`underover.md` (cada handler mantém o seu próprio
  achado/correcção documentado no arquivo do handler, mesmo padrão de `frac.md` §P905 / `root.md`
  §P901).
- `mod.rs`: `-192` linhas líquidas (4 corpos de função + secção de cabeçalho obsoleta "Passo 296"
  removidos; guard promovido a `pub(super)`; declarações de módulo alfabetizadas).

## Testes e build

```
cargo build -p typst-core            → limpo (só warnings pré-existentes)
cargo test -p typst-core --lib       → 4778 passed; 0 failed; 3 ignored
cargo test --workspace               → typst_core 4778, typst_shell 734, typst_infra 41,
                                        typst (bin) 2, cli 37 — todos 0 failed
```

Os 6 testes P906 (`p906_layout_accent_*`, `p906_layout_underover_*`) continuam a passar sem
alteração, chamando `ml.layout_accent(...)`/`ml.layout_underover(...)` via `pub(super)` através de
`math::layout::tests`, agora um módulo irmão de `accent`/`underover` (não descendente) — a
promoção de visibilidade no ponto 1 da Fase A era estritamente necessária para isto continuar a
compilar.

## `crystalline-lint`

`--fix-hashes .`: 6 ficheiros (`accent.rs`, `cancel.rs`, `underover.rs`, `op.rs`, `mod.rs`,
`tests.rs` — os dois últimos partilham `@prompt _comum.md`). `crystalline-lint .`: 0 drift novo, só
o warning V7 pré-existente e não relacionado (`infra/package_version_resolution.md`).

## `.typ` de 30 secções — comparação byte-a-byte

Metodologia: `git worktree add --detach <tmp> HEAD` (commit `5cfb12430`, estado antes deste passo)
+ build release isolado, comparado ao binário do working tree actual (depois deste passo), mesma
sessão. `InstanceID`/`DocumentID`/timestamp XMP são intencionalmente aleatórios por processo
(P615/P617) — fixados via `CRYSTALLINE_PDF_FIXED_EPOCH=1` nos dois lados para tornar a comparação
possível.

```
sha256(antes) = e584ee79b6a3081c7891fe6d90de739c95df17f77eb7ba4c1aa88b59e57f135b
sha256(depois) = e584ee79b6a3081c7891fe6d90de739c95df17f77eb7ba4c1aa88b59e57f135b
cmp antes.pdf depois.pdf → idênticos
```

PDF de saída **byte-idêntico** confirmado — prova mais forte que "suíte verde" de que o refactor é
content-preserving.

## Benchmark (Fase C)

7 cenários (`01-hello`/`02-lorem`/`03-images`/`04-math`/`05-tables`/`06-long`/`07-context`,
construídos nesta sessão — os fixtures efémeros de P906-908 não persistem em git),
`hyperfine --warmup 5 -m 20`, binários release comparados A/B na mesma sessão (mesmo método de
controlo de deriva ambiental usado em P907): `04-math.typ` exercita deliberadamente os 4 handlers
movidos (`hat`/`tilde`, `cancel`, `underbrace`/`overbrace`, `op(...)`/`op(..., limits: #true)`).

| Cenário | antes (P908 HEAD) | depois (P909) | Δ |
|---|---|---|---|
| 01-hello | 90.0ms | 90.4ms | +0.4ms |
| 02-lorem | 104.2ms | 103.8ms | -0.4ms |
| 03-images | 311.2ms | 308.4ms | -2.8ms |
| 04-math | 164.6ms | 163.8ms | -0.8ms |
| 05-tables | 92.3ms | 92.3ms | 0.0ms |
| 06-long | 138.9ms | 139.3ms | +0.4ms |
| 07-context | 104.8ms | 105.7ms | +0.9ms |

Todos os deltas dentro do ruído de `hyperfine` (σ ~1-4ms por cenário, sem direcção consistente) —
sem regressão atribuível a P909, como esperado (reorganização de código, não lógica nova).

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| Guard partilhado `layout_stretchy_or_node` — decisão de onde vive | Fica em `mod.rs`, `pub(super)`, não duplicado | ✅ |
| `layout_op`/`MathAttach` cross-variant interaction | Confirmada em `attach.rs`, documentada em `op.md` | ✅ |
| 4 arquivos novos + 4 L0s novos | Implementado, forma B, hashes sincronizados | ✅ |
| `mod.rs` reduzido | -192 linhas líquidas, arms de despacho inalterados | ✅ |
| Suite completa verde | 4778+734+41+2+37 testes, 0 falhas | ✅ |
| `crystalline-lint` 0 drift novo | Confirmado, só V7 pré-existente | ✅ |
| PDF de 30 secções byte-idêntico | sha256 idêntico confirmado | ✅ |
| Benchmark 7 cenários sem regressão | Todos os deltas dentro do ruído | ✅ |
