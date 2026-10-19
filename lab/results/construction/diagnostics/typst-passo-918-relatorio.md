# Relatório P918 — exocitose geométrica: núcleo partilhado de `math/layout/*`

**Commit base (antes de qualquer alteração deste passo):** `024ff9328` (P915, já commitado no
início desta sessão — ver nota de proveniência abaixo sobre o estado em que P915 foi encontrado).
**Commits produzidos por este passo** (4, um por módulo, per disciplina de P899):
- `e561258e7` — (1/4) `stack_tight_above` partilhado, cablado em `underover.rs`/`accent.rs`.
- `9bd10b855` — (2/4) `grid_delim_target_du` cablado em `cases.rs`/`matrix.rs`.
- `8b6ec63d7` — (3/4) `resolve_assembly_repeat` interno a `assembly.rs`.
- `523cf2d2f` — (4/4) `apply_delim_short_fall` interno a `stretchy.rs`.
**Baseline de testes (antes, herdado de P915):** typst-core: 4790 · typst-infra: 743 ·
typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** typst-core: 4790 · typst-infra: 743 · typst-shell: 41
— `0 failed` (contagem idêntica — refactor content-preserving, sem testes novos nem removidos).
**Data:** 2026-07-26.

**Nota de proveniência sobre P915**: no início desta sessão, o Passo 915 (feature `cramped`)
estava implementado, testado e verificado (ver `typst-passo-915-relatorio.md`) mas **não
commitado** — decisão do dono registada nesse relatório. Este passo (P918) exige P915 commitado
como pré-condição explícita (toca os mesmos módulos). Confirmado com o dono no início desta
sessão; commitado como `024ff9328` antes de iniciar a Fase A do P918.

---

## Fase A — inventário dos 5 candidatos (medição directa, não memória de relatórios anteriores)

| # | Candidato | Veredicto | Base (file:line) |
|---|---|---|---|
| 1a | `over_y` (`underover.rs`) / `accent_y` (`accent.rs`) | **Extrair** | Fórmulas byte-idênticas: `-(base_box.ascent + top.descent)` |
| 1b | `frac.rs` (num/den) vs `root.rs` (overline) | Manter separado | Mesma forma (`termo + gap + espessura/2`), mas `frac` desloca a própria caixa deslocada; `root` desloca uma linha fixa sobre o radicando parado em `y=0` — consumidores estruturalmente distintos |
| 1c | `attach.rs` (`compute_script_shifts`) | Manter separado | `.max()` de 4-5 termos, incomparável estruturalmente |
| 2 | Gap de empilhamento (`frac` vs `underover`) | Manter separado | Confirma achado de P906 ainda aberto: `underover.rs` não tem constante de gap explícita |
| 3 | `covering()`/fonte MATH | Nada a extrair | Já centralizado em `font_metrics.rs` desde P912 — 5 consumidores (`math_kern`, `vertical_glyph_variants`, `vertical_glyph_assembly`, `horizontal_glyph_variants`, `horizontal_glyph_assembly`) confirmados sem cópia paralela |
| 4 | `hor_advance` vs `advance`/`full_advance` | Nada a extrair | Já são campos distintos e documentados em `GlyphVariant`/`GlyphPart` (P917) |
| 5 | `compute_math_kern` | Manter separado | Single-consumer confirmado (`attach.rs`, 4 call sites internos, nenhum outro módulo chama) |

**Achados fora do escopo original dos 5 candidatos**, vistos de passagem durante a leitura directa
e incorporados por decisão do dono antes da Fase A.1:

- **`assembly.rs`** — o laço de determinação de `repeat`/`ratio` (algoritmo do vanilla, P913,
  ~53 linhas) idêntico byte-a-byte entre `layout_assembly`/`layout_assembly_horizontal`. A
  duplicação mais forte encontrada em toda a Fase A, maior que qualquer um dos 5 candidatos
  oficiais.
- **`stretchy.rs`** — achado revisto após leitura directa (uma primeira passagem, mais grosseira,
  tinha marcado como "quase idênticos"): só a subtracção de `DELIM_SHORT_FALL` (2 linhas) é
  idêntica byte-a-byte; o resto diverge de propósito (vertical centra em `axis_height`, horizontal
  assenta na baseline via `vertical_metrics`) — não unificado além dessas 2 linhas.
- **`cases.rs`/`matrix.rs`** — bloco de 5 linhas (`grid_height_pt`, margem de 10%, conversão
  pt→du) byte-idêntico, mesma constante mágica `1.1`. Tratado como extensão do candidato 1
  (mesma categoria "geometria tipográfica", ADR-0123) — ver `grid_delim_target_du` abaixo.

**Achado lateral, não investigado** (fora do escopo desta fase, registado para o dono decidir):
`fraction_denom_gap` (`math_constants.rs`) parece não ter nenhum consumidor — `frac.rs` usa
`fraction_num_gap` tanto para ascent quanto descent. Pode ser bug; não confirmado.

## Fase A.1 — núcleo geométrico desenhado (gate cumprido, aprovado pelo dono)

4 extracções, registadas em `_comum.md` §P918 antes de qualquer `.rs` tocado, hash sincronizado
(`crystalline-lint --fix-hashes .`, 8 ficheiros, 0 drift):

1. **`stack_tight_above(base_ascent, top_descent) -> f64`** — free function em `mod.rs` (mesmo
   padrão de `offset_item`). Cross-módulo (`underover.rs`, `accent.rs`) — categoria "geometria
   tipográfica" (ADR-0123).
2. **`grid_delim_target_du(&self, grid_box, style) -> f64`** — método `pub(super)` em `mod.rs`.
   Cross-módulo (`cases.rs`, `matrix.rs`) — categoria "geometria tipográfica" (ADR-0123).
3. **`resolve_assembly_repeat(assembly, scale, target_pt) -> (Vec<&GlyphPart>, f64)`** — função
   privada local a `assembly.rs`. Mesmo ficheiro, categoria "mecânica" (ADR-0107) — não precisa
   de viver em `mod.rs`.
4. **`apply_delim_short_fall(target_du, upem) -> f64`** — função privada local a `stretchy.rs`.
   Mesmo ficheiro, categoria "mecânica" (ADR-0107).

**Não extraído** (decisão registada para não repetir a pergunta em passo futuro): `frac.rs` vs
`root.rs`, `attach.rs`, gap de `underover.rs` (achado de P906 continua em aberto), resto de
`stretchy.rs` além do short-fall — ver tabela da Fase A acima para a razão de cada um.

**7 L0s escritos**: `_comum.md`, `underover.md`, `accent.md`, `cases.md`, `matrix.md`,
`assembly.md`, `stretchy.md`.

## Fase B — implementação, módulo a módulo (4 commits, suite verde a cada um)

| Commit | Módulo | Extracção | Suite (core/infra/shell) |
|---|---|---|---|
| `e561258e7` | `mod.rs` + `underover.rs` + `accent.rs` | `stack_tight_above` (+ `grid_delim_target_du` adicionado, ainda não cablado) | 4790/743/41, 0 failed |
| `9bd10b855` | `cases.rs` + `matrix.rs` | `grid_delim_target_du` cablado | 4790/743/41, 0 failed |
| `8b6ec63d7` | `assembly.rs` | `resolve_assembly_repeat` | 4790/743/41, 0 failed |
| `523cf2d2f` | `stretchy.rs` | `apply_delim_short_fall` | 4790/743/41, 0 failed |

`crystalline-lint .` após cada commit: 0 drift novo (só o V7 pré-existente e não relacionado,
`infra/package_version_resolution.md`).

## Prova final — PDF de 30 secções byte-idêntico (mesmo padrão de P909)

Metodologia (idêntica a P909): `git worktree add --detach <tmp> 024ff9328` (estado antes deste
passo) + build release isolado (`cargo build --release -p typst-wiring`), comparado ao binário do
working tree actual (`523cf2d2f`, depois deste passo), mesma sessão.
`CRYSTALLINE_PDF_FIXED_EPOCH=1` nos dois lados (fixa `InstanceID`/`DocumentID`/timestamp XMP,
P615/P617). Fixture: `.typ/typst-math-comprehensive-test.typ` (30 secções, já presente no
repositório, mesmo ficheiro referenciado por P909).

```
sha256(antes)  = ba83e03b9e7be51155fe686293305576f5d1379232cc16672b7c90473e2e6f01
sha256(depois) = ba83e03b9e7be51155fe686293305576f5d1379232cc16672b7c90473e2e6f01
cmp antes.pdf depois.pdf → idênticos
```

PDF de saída **byte-idêntico** confirmado, separado do commit único (não houve correcção
"real" neste passo — as 4 extracções são puro refactor, ao contrário de P915 que teve uma
feature nova).

## Fase C — Benchmark (7 cenários, `hyperfine --warmup 5 -N -m 20`)

Ficheiros canónicos (`01-hello` … `07-context`) reconstruídos equivalentes às mesmas 7
categorias de passos anteriores (não persistem em git, mesma prática de P909/P915/P917).
Binários `antes` (`024ff9328`) / `depois` (`523cf2d2f`), release, mesma sessão.

| Cenário | Razão (depois vs antes) |
|---|---|
| 01-hello | 1.00× |
| 02-lorem | 1.01× |
| 03-images | 1.00× (depois ligeiramente mais rápido, dentro do ruído) |
| 04-math | 1.02× |
| 05-tables | 1.02× (outlier reportado pelo hyperfine, mesma ordem de grandeza dos outros) |
| 06-long | 1.01× |
| 07-context | 1.00× |

**Nenhuma regressão atribuível a P918** — todos os deltas na mesma banda de ruído 1.00–1.02×
já estabelecida em P909/P915/P917 (reorganização de código, não lógica nova).

---

## Resultado final

| Item | Estado |
|---|---|
| Fase A — 5 candidatos + 3 achados extra, `file:line` dos dois lados | ✅ |
| Fase A.1 — núcleo desenhado, 7 L0s, hash sincronizado, gate aprovado pelo dono | ✅ |
| Fase B — 4 extracções, módulo a módulo, 4 commits, suite verde a cada um | ✅ |
| PDF de 30 secções byte-idêntico (sha256 igual) | ✅ |
| Suite completa final | ✅ core 4790 · infra 743 · shell 41, `0 failed` |
| `crystalline-lint .` | ✅ 0 drift novo |
| Benchmark 7 cenários | ✅ Sem regressão, banda 1.00–1.02× |
| Commits | ✅ 4, um por módulo (`e561258e7`, `9bd10b855`, `8b6ec63d7`, `523cf2d2f`) |

### Achados em aberto desta frente, não tocados por este passo

- Gap de empilhamento de `underover.rs` sem constante explícita de `MathConstants` (P906, ainda
  aberto).
- `fraction_denom_gap` sem consumidor confirmado em `frac.rs` (achado lateral desta Fase A, não
  investigado).
- Desalinhamento vertical entre elementos adjacentes numa sequência matemática (herdado de P917).
- `assembly` não atinge a altura-alvo total em matrizes/casos de 6+ linhas (herdado de P917).

### Proveniência

- Commits: `e561258e7`, `9bd10b855`, `8b6ec63d7`, `523cf2d2f` (`git log`), working tree limpo
  nestes ficheiros no fim da sessão (`git status --short` só mostra backlog não relacionado:
  `p916-work/`, PDFs soltos na raiz, pré-existentes ao início desta sessão).
- Todos os números reproduzíveis via `cargo test --workspace`, `crystalline-lint .`, e o par de
  binários release `024ff9328`/`523cf2d2f` (`cargo build --release -p typst-wiring` em cada) para
  o PDF byte-idêntico e o benchmark.
