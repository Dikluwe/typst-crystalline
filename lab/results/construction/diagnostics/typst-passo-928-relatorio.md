# Relatório P928 — tentativa da Opção 1 (coverage pré-computada no arranque)

**Precede este passo:** `typst-passo-927-relatorio.md` — Opção 6 (scan condicional)
resolveu o custo de fallback para documentos sem CJK/emoji, mas deixou o custo
absoluto de ~7 s intacto para quem realmente precisa de fallback.

**Objectivo deste passo:** testar a Opção 1 do P925 (pré-computar `coverage` no
`FontInfo` durante o arranque e delegar `candidates_for_char` no `FontBook`)
como forma de reduzir o custo absoluto do fallback para CJK/emoji.

**Data:** 2026-07-28.
**Commit base:** `da18ea9f3` (P922–P924 integrados).
**Resultado:** a abordagem **foi revertida** porque regrediu o caso comum e
não reduziu o custo CJK/emoji de forma significativa.

---

## Resumo executivo

A Opção 1 do P925 falhou nos dois critérios principais:

1. **Regressão no caso comum:** os 7 cenários canônicos ficaram entre
   **1.17× e 1.66×** mais lentos, anulando o zero regressão que P927 tinha
   conseguido (1.01–1.03×).
2. **Ganho insuficiente nos casos CJK/emoji:** `utf8-cjk` melhorou apenas
   **14%** (0.86×) e `utf8-emoji` **piorou 21%** (1.21×). O outlier `05-utf8`
   melhorou só **8%** (0.92×), longe dos ~43× medidos na solução temporária
   completa do P925.

A causa imediata foi que só se implementou a **primeira das três alterações**
que o P925 mediu em conjunto: pré-computar `coverage` no arranque. O shaper e
o `FallbackFontMetrics` continuaram a carregar faces intermediárias, pelo que
o ganho real ficou muito abaixo do esperado. Ao mesmo tempo, a pré-computação
no arranque pagou o custo da iteração do `cmap` para **todos** os documentos,
invalidando o benefício do P927.

Decisão: reverter a Opção 1 e manter o estado P927. As opções futuras que
preservam zero regressão no caso comum são: (a) paralelizar o scan lazy quando
dispara; (b) cache em disco da coverage entre execuções.

---

## Fase A — o que se tentou

### Alterações implementadas

1. `03_infra/src/fonts.rs:348-356` — `font_info_from_bytes` passou a preencher
   `coverage: extract_coverage(&face)` no arranque.
2. `03_infra/src/world.rs:584-600` — `SystemWorld::candidates_for_char` delegou
   no `FontBook::candidates_for_char` em vez de abrir fontes lazy.
3. `03_infra/src/world.rs:156` — `#[allow(dead_code)]` no `coverage_cache` para
   silenciar warning após a delegação.
4. L0s actualizados para descrever P928 (mais tarde revertidos — ver Fase E).

### O que ficou por fazer

O P925 tinha medido uma solução temporária completa com **três** alterações:

1. Pré-computar `coverage` no `FontInfo`.
2. `shaper.rs::covering_all`/`best_covering_run` usarem `FontInfo::coverage`
   para filtrar candidatos sem carregar faces.
3. `FallbackFontMetrics::covering` passar índices filtrados por coverage
   directamente para `FontBook::select_fallback`, sem carregar faces
   intermediárias.

Só a primeira foi implementada. Sem 2 e 3, o fallback continua carregando
muitas faces candidatas.

---

## Fase B — medições

### Metodologia

- **Binário baseline:** `target-original/release/typst`, commit `da18ea9f3`,
  SHA-256 `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`.
- **Binário P928 (tentativa):** `target/release/typst` com P927 + Opção 1,
  SHA-256 `de331e981cf989ef8cc875a48b9b6d3e606930044a1555dd8ec9f03ec96022e9`.
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 (canônicos), warmup 2 /
  min-runs 10 (UTF-8).
- **Scripts:** `tools/perf/benchmark-p928-canonical.py`,
  `tools/perf/benchmark-p928-utf8.py`.
- **Atestações:** `tools/perf/results/p928-canonical/attestation.json`,
  `tools/perf/results/p928-utf8/attestation.json`.

### B.1 — caso comum (7 cenários canônicos)

| Cenário | baseline (ms) | P928 (ms) | rácio |
|---|---:|---:|---:|
| 01-hello | 90.01 | 149.11 | **1.66×** |
| 02-lorem | 113.04 | 169.44 | **1.50×** |
| 03-images | 96.63 | 152.06 | **1.57×** |
| 04-math | 151.80 | 205.94 | **1.36×** |
| 05-tables | 93.94 | 153.40 | **1.63×** |
| 06-long | 301.40 | 351.61 | **1.17×** |
| 07-context | 133.28 | 187.81 | **1.41×** |

**Conclusão:** regressão generalizada no caso comum, com user time a subir
~50–60 ms em documentos latinos simples. Este é exactamente o padrão que P927
fora desenhado para evitar.

### B.2 — casos UTF-8/CJK/emoji

| Cenário | baseline (ms) | P928 (ms) | rácio |
|---|---:|---:|---:|
| 05-utf8 | 7911.06 | 7262.36 | 0.92× |
| utf8-latin | 89.83 | 147.39 | **1.64×** |
| utf8-greek | 90.44 | 146.50 | **1.62×** |
| utf8-cjk | 7219.67 | 6173.12 | 0.86× |
| utf8-emoji | 7284.46 | 8829.32 | **1.21×** |

Timings internos para `05-utf8`:

| | baseline | P928 |
|---|---:|---:|
| `layout_ms` | 6365.45 | 5740.99 |
| `shape_ms` | 521.57 | 500.75 |
| `total_ms` | 7171.32 | 6527.27 |

**Conclusão:** o ganho em CJK é marginal (14%) e emoji piorou. A redução de
`layout_ms`/~shape_ms` mostra que a filtragem por bitmap ajuda marginalmente,
mas o shaper e o `FallbackFontMetrics` ainda carregam faces em excesso — o
ganho real fica muito abaixo dos ~43× da solução temporária completa do P925.

---

## Fase C — análise

### Por que a Opção 1 falhou

1. **Custo de arranque pago por todos.** Pré-computar `coverage` para todas as
   fontes do sistema (~1112 slots) aumenta o trabalho de CPU no arranque. Para
   documentos latinos simples, que P927 conseguia manter abaixo de ~90 ms, isso
   adicionou ~60 ms de user time.
2. **Ganho incompleto em CJK/emoji.** Sem alterar `shaper.rs` e
   `font_metrics.rs` para usarem `FontInfo::coverage` sem carregar faces, o
   fallback continua a carregar dezenas de candidatas. A filtragem por bitmap
   no `candidates_for_char` sozinha não é suficiente.
3. **Emoji piorou.** A coverage por bloco de 256 codepoints é demasiado
   grosseira para emoji (muitos blocos partilhados), e o shaper acabou por
   carregar mais faces do que o caminho original.

### Por que a solução temporária do P925 foi diferente

O P925 mediu **0.20 s** para `05-utf8` quando aplicou as três alterações em
conjunto, incluindo a mudança no shaper e no `FallbackFontMetrics`. A tentativa
actual só aplicou a primeira, pelo que não é comparável.

---

## Fase D — decisão e reversão

Decisão: **reverter a Opção 1** e ficar com o estado P927.

Motivos:
- A regressão no caso comum é inaceitável e anula o trabalho de P927.
- O ganho em CJK/emoji é marginal e emoji piora.
- Implementar as alterações 2 e 3 do P925 manteria a regressão no caso comum
  (custo de arranque pago por todos) e aumentaria o risco de mudanças no shaper
  e nas métricas.

O que foi revertido:
- `03_infra/src/fonts.rs` — `coverage: Coverage::new()` no arranque.
- `03_infra/src/world.rs` — `candidates_for_char` volta ao cache lazy P880/P927.
- `03_infra/src/world.rs` — removido `#[allow(dead_code)]` do `coverage_cache`.
- L0s de P928 removidos de `font-book.md`, `fontdb.md`, `shaper.md`,
  `font_metrics.md`, `system-world.md`.
- Hashes sincronizados via `crystalline-lint --fix-hashes`.

---

## Fase E — estado da árvore após reversão

- `cargo test --workspace`: passou em todos os crates.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`).
- Benchmark canônico re-corre após reversão — rácios de volta à banda de ruído
  (P927 vs baseline):

| Cenário | rácio pós-reversão |
|---|---:|
| 01-hello | 1.02× |
| 02-lorem | 1.03× |
| 03-images | 1.06× |
| 04-math | 0.97× |
| 05-tables | 1.02× |
| 06-long | 1.00× |
| 07-context | 1.00× |

Binário pós-reversão: `target/release/typst`, SHA-256
`d440fc998ddc415e8dd12e54d930173ff8e6d6d37244c821383423bd232ba1b3`.

---

## Fase F — opções futuras

A frente do custo absoluto de fallback CJK/emoji continua aberta. As opções
registadas em P926/P927 que não prejudicam o caso comum são:

1. **Paralelizar o scan lazy quando dispara.** Em vez de abrir as ~1112 fontes
   em sequência, usar threads para processar múltiplos slots em paralelo. O
   ganho depende do número de núcleos e da largura de banda de I/O.
2. **Cache em disco da coverage entre execuções.** Guardar os bitmaps de
   coverage num ficheiro de cache e invalidá-lo quando as fontes instaladas
   mudarem. A primeira execução continuaria lenta, mas execuções subsequentes
   seriam quase instantâneas.

Ambas requerem medição própria antes de qualquer decisão.

---

## Proveniência

- Commit base: `da18ea9f3`.
- Binário baseline: `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`.
- Binário P928 (tentativa): `target/release/typst`, SHA-256
  `de331e981cf989ef8cc875a48b9b6d3e606930044a1555dd8ec9f03ec96022e9`.
- Binário pós-reversão: `target/release/typst`, SHA-256
  `d440fc998ddc415e8dd12e54d930173ff8e6d6d37244c821383423bd232ba1b3`.
- Scripts: `tools/perf/benchmark-p928-canonical.py`,
  `tools/perf/benchmark-p928-utf8.py`.
- Atestações:
  - Tentativa P928 UTF-8: `tools/perf/results/p928-utf8/attestation.json`
    (binário `de331e981c…`).
  - Pós-reversão canônico: `tools/perf/results/p928-canonical/attestation.json`
    (binário `d440fc998d…`).
  - Nota: a atestação canônica da tentativa P928 foi sobrescrita pela
    validação pós-reversão; os números da tentativa no caso comum constam do
    output do script e deste relatório.
