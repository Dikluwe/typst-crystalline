# P672 — Relatório de Paridade de Produção

**Passo:** 672  
**Data:** 2026-07-10  
**Foco:** Explicar e corrigir a discrepância entre 99,76 % de acerto na cache de shaping (P657) e apenas 29 % de redução de `shape_ms`.  
**Commit base:** `2cf78ce0532df184da74b2d9bd5b951ad083cc3c` (P671)  
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

---

## 1. Sonda

### 1.1 Hipótese

P657 reportou:

- Hit ratio na cache de shaping: **99,76 %** (428 948 repetições / 430 000 chamadas).
- Redução de `shape_ms`: apenas **29 %** (34 211 ms → 24 147 ms).

Se quase todas as chamadas acertam na cache, a redução devia aproximar-se de 99 %. A discrepância indica trabalho caro a acontecer **antes** da consulta à cache.

### 1.2 Instrumentação temporária

Adicionou-se instrumentação em `03_infra/src/shaper.rs` para medir, por sub-run:

1. `setup_ms`: resolução de fontes primárias/fallback (`resolve_candidates`) + criação do `CandidateSet`.
2. `bidi_split_ms`: `bidi_runs` + `split_run_by_font` + `candidates.get`.
3. `key_ms`: construção da chave da cache.
4. `lookup_ms`: consulta `cache.map.get`.
5. `miss_ms`: trabalho real em cache miss (`rustybuzz::Face::from_slice` + `rustybuzz::shape`).
6. `post_ms`: construção dos `FrameItem::TextShaped` após a cache.
7. Número de chamadas a `face_covers_char`.

Comando:

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p672-instrumentado.pdf
```

Resultado (antes da correcção):

```text
[P672 instrumentação] setup_ms=4761.31 bidi_split_ms=18552.16 key_ms=118.29 lookup_ms=84.23 miss_ms=25.68 post_ms=315.81 hits=428951 misses=1049 face_covers=1857840
```

| Parte | Tempo (ms) | % do shape_ms observado |
|---|---|---:|
| setup | 4 761 | ~16 % |
| bidi + split | 18 552 | ~62 % |
| key | 118 | <1 % |
| lookup | 84 | <1 % |
| miss (shape real) | 25 | <1 % |
| post | 316 | ~1 % |
| **Total instrumentado** | **~23 857** | **~80 %** |

O trabalho real de `rustybuzz::shape` em cache miss é apenas **25 ms** — confirmando o hit ratio de 99,76 %. O gargalo está na segmentação por fonte (`bidi_split_ms`) e, dentro dela, na função `face_covers_char`, que foi chamada **1 857 840 vezes**, cada uma re-parseando a face `ttf-parser` correspondente.

### 1.3 Confirmação do padrão P546

O padrão é exactamente o mesmo de P546: `ttf_parser::Face::parse` a ser chamado repetidamente sem cache. Em `face_covers_char` (`03_infra/src/shaper.rs:577`):

```rust
let Some(face) = ttf_parser::Face::parse(font.as_slice(), 0).ok() else { return false };
face.glyph_index(c).is_some()
```

A cache de `Face` de P548 existe em `FallbackFontMetrics` (`03_infra/src/font_metrics.rs`), mas o `shaper.rs` não a usava. A cache de resultados de shaping (P657) não cobria este trabalho anterior.

---

## 2. Implementação

### 2.1 `03_infra/src/shaper.rs`

Adicionou-se uma cache local de faces `ttf-parser` por documento (`FaceCache`), análoga à de `FallbackFontMetrics`, mas scoped ao `shape_document`:

- `FaceCache`: mapeia `slot_idx -> Arc<CachedFace>`.
- `CachedFace`: guarda os bytes da fonte (`Font`) e a face parseada (`ttf_parser::Face<'static>`), usando o mesmo truque de lifetime de P548 (`from_raw_parts` sobre slice owned).

A cache é criada em `shape_document` e passada por `shape_page` → `shape_item` → `try_shape` → `CandidateSet`. Todas as funções que antes parseavam faces de novo passaram a usar a cache:

- `resolve_candidates` (obtém `units_per_em`).
- `CandidateSet::load_fallback`.
- `face_covers_char`.

A cache de resultados de shaping (P657) manteve-se inalterada.

### 2.2 `shaped_width`

`shaped_width` (P591) também usa `CandidateSet`, mas é chamado a partir do layout, fora do `shape_document`. Criou-se uma `FaceCache` local dentro de `shaped_width` para evitar re-parsear faces durante a medição de larguras de fallback.

---

## 3. Validação

### 3.1 Correcção do output

Comparação do texto extraído do PDF antes e depois da correcção:

```bash
pdftotext /tmp/p672-baseline.pdf /tmp/p672-baseline.txt
pdftotext /tmp/p672-final.pdf /tmp/p672-final.txt
diff -u /tmp/p672-baseline.txt /tmp/p672-final.txt
```

Resultado: `0` linhas de diferença — output idêntico.

### 3.2 `shape_ms` do `macro-10x`

| Estado | `shape_ms` (ms) | `total_ms` (ms) |
|---|---:|---:|
| P657 (com cache de shaping) | 24 147 | 27 445 |
| P671 (baseline P672) | ~30 134 | ~34 111 |
| **P672 (com cache de faces)** | **2 199** | **5 914** |

A cache de faces reduziu `shape_ms` em mais de **10×** relativamente a P657/P671.

### 3.3 Benchmark completo

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
```

Tempo total da execução: ~2 min 39 s.

#### Macro

| Passo | Vanilla (ms) | Cristalino (ms) | Rácio |
|---|---|---:|---:|---:|
| P618 | 5303,54 | 35958,10 | 6,78× |
| P657 | 5035,43 | 29097,89 | 5,78× |
| P670 | 6047,79 | 36129,28 | 5,97× |
| P671 | 5728,07 | 36357,03 | 6,35× |
| **P672** | **5973,80** | **7974,72** | **1,33×** |

O `macro-10x` passou de ~36 s para ~8 s no cristalino, reduzindo o rácio de 6,35× para **1,33×**.

#### Micro (seleção)

| Documento | Rácio P672 | Rácio P671 |
|---|---:|---:|
| test-array | 1,74× | 1,89× |
| test-calc | 1,89× | 1,87× |
| test-columns | 1,77× | 1,94× |
| test-footnote | 1,84× | 1,89× |
| test-math | 1,85× | 1,85× |
| test-raw | 1,46× | 1,47× |
| test-stroke-sides | 28,86× | 29,63× |
| test-image-fit | 28,27× | 29,31× |
| test-bibliography-csl | 1,54× | 1,59× |
| test-raw-advanced | 1,17× | 1,23× |

Os documentos micro mantiveram-se estáveis ou melhoraram ligeiramente. Os outliers (`test-stroke-sides`, `test-image-fit`) continuam altos por causa do tempo de arranque fixo do cristalino contra documentos muito pequenos, mas não pioraram.

### 3.4 Testes e linter

- `cargo test --workspace`: passou.
- `crystalline-lint .`: `✓ No violations found`.

---

## 4. Decisão

- A discrepância de P657 foi **explicada e corrigida**: o gargalo não era a cache de shaping, mas o re-parse contínuo de faces `ttf-parser` durante a segmentação por fonte (`face_covers_char`), que acontecia **antes** da consulta à cache.
- A cache local de faces por documento elimina esse custo, reduzindo `shape_ms` de ~30 s para ~2,2 s no `macro-10x`.
- O output PDF mantém-se idêntico; testes e linter passam.
- O benchmark completo mostra uma melhoria drástica no caso macro (rácio de 6,35× para 1,33×).

Ação: P672 fecha a pendência de desempenho levantada por P657/P670 e actualiza o registo histórico de benchmark.

---

## 5. Proveniência da medição

- Commit base: `2cf78ce0532df184da74b2d9bd5b951ad083cc3c` (P671).
- Ficheiro alterado: `03_infra/src/shaper.rs`.
- Comando de instrumentação: `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p672-instrumentado.pdf`.
- Comando de timings: `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p672-final.pdf --timings-json /tmp/p672-final-timings.json`.
- Comando de diff: `pdftotext` + `diff -u` (ver secção 3.1).
- Comando de testes: `cargo test --workspace`.
- Comando de linter: `crystalline-lint .`.
- Comando de benchmark: `time timeout 900 python3 tools/perf/benchmark-p507.py`.
- Ficheiro de resultados: `tools/perf/results/benchmark-p507-summary.json`.
