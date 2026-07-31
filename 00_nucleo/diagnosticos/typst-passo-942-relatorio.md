# Relatório P942 — o mistério de P939: código "equivalente" mas ~3× mais lento

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-942.md`  
**Commit base:** `ad3372c53` (P941)  
**Binário cristalino P942:** `target/release/typst-p942` (strings `Typst compiler (crystalline)`)

---

## 1. Resumo executivo

O mistério de P939 (código de coverage "equivalente" mas ~3× mais lento que o vanilla) está
**resolvido**. A causa não era a construção de `Coverage` em si — era uma **re-verificação
redundante de `glyph_index`** no caminho de fallback de `font_metrics::covering` (P838), que
**carregava a face de cada candidato** (dezenas de `.ttc` de CJK, ~10–20 MB cada) só para
confirmar o que a coverage exacta já garantia.

**Correções aplicadas:**

1. `Coverage::from_codepoints`: `impl Into<Vec<u32>>` (sem cópia do Vec, ~115 MB evitados) e
   `sort()` (TimSort) em vez de `sort_unstable()` — a cmap já vem ordenada, e o TimSort é O(n)
   em input ordenado. Breakdown da extração de coverage: 340→196 ms.
2. `Coverage::contains`: terminação antecipada (pára quando `cursor > codepoint`).
3. **`font_metrics::covering`: deixa de carregar a face de cada candidato para re-verificar
   `glyph_index`** — a coverage exacta (P937/P938) já é definitiva, e o vanilla confia nela em
   `select_fallback` (`book.rs:106-114`). Só a face da fonte vencedora é carregada.

**Resultado:** os cenários de fallback pesado caíram de ~4.4–6.1× para **~1.2–1.3× do vanilla**.

| Cenário | P938 (ms) | P941 (ms) | **P942 (ms)** | Vanilla (ms) | P942/Vanilla |
|---|---:|---:|---:|---:|---:|
| `05-utf8` | 1549 | 1294 | **363** | 285 | **1.28×** |
| `utf8-cjk` | 1233 | 1248 | **331** | 283 | **1.17×** |
| `utf8-emoji` | 1540 | 1258 | **319** | 271 | **1.18×** |

O caso comum mantém-se ~0.3× do vanilla (mais rápido que o vanilla), sem regressão.

---

## 2. Fase A — diferenças de build (descartadas)

- **Perfil release:** o vanilla usa `lto = "thin", codegen-units = 1`; o cristalino usava defaults.
  Testado com build LTO: `layout_ms` 724 vs 727 ms — **não explica a distância**. Revertido.
- **Alocador:** nenhum dos dois usa `jemalloc`/`mimalloc` — ambos usam o alocador do sistema.
- **Paralelismo:** nenhum dos dois usa `rayon` na descoberta de fontes (`typst-kit/fonts.rs` e
  `fontdb` são sequenciais).

## 3. Fase B — perfil (instrumentação manual; `perf` bloqueado)

`perf_event_paranoid=4` neste ambiente — `perf record` não é permitido. Usámos instrumentação
manual com `std::time::Instant` (revertida depois):

### 3.1 Breakdown da extração de coverage (release, 1112 fontes)

| Etapa | Antes | Depois da correcção (1) |
|---|---:|---:|
| `Face::parse` | 15.3 ms | 15.3 ms |
| iteração da `cmap` | 31.2 ms | 30.0 ms |
| `Coverage::from_codepoints` (sort+dedup+runs) | **293.4 ms** | **151.1 ms** |
| **total** | 339.8 ms | **196.4 ms** |

A construção de `Coverage` era dominada pelo sort/cópia — corrigida (1). Mas o `layout_ms` não
desceu (724→728 ms), provando que a extração de coverage **não** era o custo principal.

### 3.2 `candidates_for_char` (contagem e tempo total)

17 chamadas, **249 ms total** em `utf8-cjk` (inclui a extração de coverage na primeira chamada).
Itera as ~1112 fontes por caractere — mas mesmo isto era só ~53 ms de iteração após a extração.

### 3.3 A causa real: re-verificação redundante em `font_metrics::covering`

`covering` (P838) chamava `cached_face(slot_idx)` + `glyph_index(c)` para **cada** candidato
devolvido por `candidates_for_char` — carregando dezenas de faces `.ttc` de CJK (~10–20 MB cada,
via `extract_collection_face`) só para confirmar cobertura que a coverage exacta já dava. Medido
como o custo dominante: `layout_ms` **728→25 ms** em `utf8-cjk` após a correcção (3).

O vanilla confia na coverage em `select_fallback` (`book.rs:106-114`) e **não** re-verifica
`glyph_index`. A re-verificação era um resíduo de quando a coverage era aproximada (bitmap, com
falsos positivos) — deixou de ser necessária com a coverage exacta de P937/P938.

---

## 4. Fase D — medição final

### 4.1 `layout_ms` isolado

| Cenário | P941 (ms) | **P942 (ms)** |
|---|---:|---:|
| `05-utf8` | 722 | **26.6** |
| `utf8-cjk` | 728 | **25.2** |
| `utf8-emoji` | 740 | **3.7** |
| `01-hello` | 0.2 | **0.2** |

### 4.2 Benchmark completo (11 cenários, `hyperfine --warmup 1 --min-runs 10`)

| Cenário | P942 (ms) | P938 (ms) | Vanilla (ms) | P942/P938 | P942/Vanilla |
|---|---:|---:|---:|---:|---:|
| `01-hello`   |  85.9 |  86.5 | 257.7 | 1.00 | 0.33 |
| `02-lorem`   |  88.0 |  89.8 | 253.6 | 1.00 | 0.35 |
| `03-math`    |  93.5 |  92.4 | 255.7 | 1.00 | 0.37 |
| `04-code`    |  87.2 |  88.5 | 254.6 | 1.00 | 0.34 |
| `05-utf8`    | 363.2 | 1549.0 | 284.7 | 0.23 | 1.28 |
| `06-matrix`  |  87.6 |  88.3 | 252.2 | 1.00 | 0.35 |
| `07-cases`   |  87.8 |  88.4 | 252.9 | 1.00 | 0.35 |
| `utf8-latin` |  85.8 |  86.7 | 253.1 | 1.00 | 0.34 |
| `utf8-greek` |  86.4 |  86.6 | 253.2 | 1.00 | 0.34 |
| `utf8-cjk`   | 331.0 | 1232.5 | 282.7 | 0.27 | 1.17 |
| `utf8-emoji` | 318.5 | 1539.7 | 270.5 | 0.21 | 1.18 |

### 4.3 Leitura

- **Caso comum:** zero regressão (~86–93 ms, ~0.3× do vanilla — mais rápido que o vanilla).
- **Fallback pesado:** ~4.3–4.8× mais rápido que P938; agora só **~1.2–1.3× do vanilla**.
- **Uniformidade do vanilla (~250–285 ms em tudo):** o vanilla paga a descoberta+coverage no
  arranque para todos os documentos. O cristalino paga-a só quando há fallback (e agora sem o
  custo redundante), sendo mais rápido no caso comum e comparável no fallback.

### 4.4 Confirmação de output

- `utf8-cjk.pdf` — texto CJK correcto (日本語の文章、中文文本、한국어 텍스트).
- `utf8-emoji.pdf` — emojis a cores (como em P941), tamanho ~52 KB.

---

## 5. O que foi investigado e descartado (registo para não repetir)

- **Build profile (LTO/codegen-units):** testado, não explica (revertido).
- **Alocador:** igual nos dois lados.
- **Paralelismo (rayon):** não usado na descoberta de fontes em nenhum dos lados.
- **Duplicação de I/O do `FontSlot`:** testada em P939 (mmap partilhado), revertida — não explica.
- **`Coverage::from_codepoints` (sort/cópia):** era um custo real mas secundário (340→196 ms);
  corrigido, mas não era o factor de ~3×.
- **`candidates_for_char` por caractere:** custo real (~53 ms de iteração em `utf8-cjk`), mas
  secundário face à re-verificação de faces.

---

## 6. Distância restante ao vanilla

- Fallback pesado: **~1.2–1.3×** (antes ~4.4–6.1×). O resíduo vem da descoberta+coverage lazy
  (~267 ms vs ~256 ms do vanilla) e do arranque do processo — margem pequena.
- Caso comum: **~0.3× do vanilla** (cristalino mais rápido).
- Próximos alvos possíveis (se se quiser fechar o 1.2×): reduzir o custo da extração de coverage
  lazy (196 ms) ou da descoberta (71 ms) — ambos já medidos e decompostos neste passo.

---

## 7. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Breakdown coverage | teste temporário `p942_coverage_breakdown` (release) | working tree | parse/collect/runs, 1112 fontes |
| `candidates_for_char` | contadores atómicos temporários + `eprintln!` (revertidos) | working tree | 17 chamadas, 249 ms |
| `layout_ms` | `--timings-json` | `typst-p942` | secção 4.1 |
| Benchmark 11 cenários | `hyperfine --warmup 1 --min-runs 10` | `typst-p942` | JSONs em `tools/perf/results/p942/` |
| Build LTO | `lto=thin, codegen-units=1` (revertido) | working tree | `layout_ms` 724 vs 727 ms |
| Output | `mutool draw` | `typst-p942` | CJK correcto, emoji a cores |
| Suíte | `cargo test -p typst-infra --lib`, `typst-core` | working tree | 748 + 4806 passed |
| Linter | `crystalline-lint .` | working tree | 0 drift (V7 pré-existente) |

---

## 8. Validação final

- [x] Causa exacta confirmada por instrumentação (re-verificação redundante de `glyph_index`).
- [x] Correção implementada (3 mudanças) e medida (`layout_ms` 728→25 ms).
- [x] Benchmark 11 cenários: fallback ~1.2–1.3× do vanilla; caso comum ~0.3× sem regressão.
- [x] `cargo test` — 748 (infra) + 4806 (core) passed.
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Output CJK/emoji confirmado correcto.
