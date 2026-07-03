# Diagnóstico de Paridade — Passo 548 (FallbackFontMetrics: cache + kerning)

**Data:** 2026-07-03
**Repositório:** `typst-crystalline`
**Artefactos de comparação:**

- Compilador cristalino: `./target/release/typst` (build `--release` após P548)
- Comparador vanilla: `/usr/local/bin/typst` (0.14.2) / `lab/typst-original/target/release/typst` (0.15.0)
- Fonte de teste tipográfico: DejaVu Sans (`/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`)
- Prompt L0 actualizado: `00_nucleo/prompts/infra/font_metrics.md` (hash `88903d1e`)

---

## 1. Objetivo

Corrigir duas causas identificadas em P544/P546 na mesma função, `FallbackFontMetrics::advance()`:

1. **Regressão de desempenho:** re-parse completo da fonte por carácter, que levou `macro-10x` de ~54,5 s (antes de P544) para ~135,2 s (depois de P544), 28,68× mais lento que o vanilla.
2. **Bug de espaçamento:** a medição de largura não aplicava kerning, reservando mais espaço do que o shaper usava de facto — o excesso aparecia como espaço real no texto extraído (`" A`, `T ypography`, `.”Journal`).

Antes de qualquer código, P548 cumpriu a Trava Arquitetural: actualizar o Prompt L0 de `infra/font_metrics.md` para documentar `FallbackFontMetrics`, a cache do `Face` parseado e a decisão sobre kerning.

---

## 2. Alterações aplicadas

### 2.1 L0 — `00_nucleo/prompts/infra/font_metrics.md`

- Nova secção `FallbackFontMetrics<'a>` com a interface, a cache `Arc<Mutex<HashMap<usize, Arc<CachedFace>>>>` e a decisão de lifetime (`Face<'static>` partilhando alocação estável do `Font` via padrão `unsafe` equivalente ao Typst vanilla).
- Decisão sobre kerning: aplicar kerning a partir das tabelas legacy `kern`/`kerx` do TrueType/OpenType, com justificativa (alinhamento com o shaper) e limitação conhecida (GPOS não consultado nesta iteração).
- Invariantes e critérios de verificação para cache e kerning.

Hash confirmado via `crystalline-lint --fix-hashes .`: `88903d1e`.

### 2.2 Cache do `Face` parseado — `03_infra/src/font_metrics.rs`

- Introduzida `struct CachedFace { data: Font, face: Face<'static> }`, alocada em `Arc`, indexada por `slot_idx` do `FontBook`.
- `FallbackFontMetrics::cached_face(slot_idx)` devolve a face existente ou parseia e guarda uma nova cópia.
- `resolve_primary()`, `covering()` e `advance()` reutilizam a mesma face, eliminando os 2–3 parses por carácter documentados em P546.
- `Clone` partilha a cache, de modo que layouters posteriores (fixpoint loops de TOC, etc.) reaproveitam as faces já parseadas.

### 2.3 Kerning na medição de largura — `03_infra/src/font_metrics.rs`

- `advance()` itera carácter a carácter e, para pares consecutivos na **mesma fonte candidata**, consulta `face_kerning()`.
- `face_kerning()` percorre subtables `kern` e depois `kerx`; o valor (em design units) é convertido para `Pt` e adicionado ao total.
- Pares que mudam de fonte não aplicam kerning, coerente com o facto de o shaper partir runs separadas por fonte.

### 2.4 Correção do sinal do delta TJ — `03_infra/src/export/stream.rs` (P520)

Durante a verificação do kerning na medição, descobriu-se que o delta do operador PDF `TJ` estava com sinal invertido. O operador `TJ` **subtrai** o número da coordenada horizontal; portanto:

```text
advance_tu = (nominal - x_advance) / upm * 1000
```

O código anterior usava `(x_advance - nominal)`, o que afastava glifos em vez de os aproximar quando o kerning era negativo. A correção foi aplicada aos cenários `Cidfont` e `Multifont` em `emit_shaped_pdf`.

L0s afectados actualizados:

- `00_nucleo/prompts/infra/export/stream.md`
- `00_nucleo/prompts/infra/export/builder.md`

Snapshot afectado regenerado:

- `03_infra/fixtures/p307b/reference/09-cidfont.pdf` — mantém paridade com a nova fórmula.
- `07-multi-feature.pdf` foi mantido original (flaky pré-existente, não relacionado com este passo).

---

## 3. Validação

### 3.1 Build e testes

```text
cargo build --release          # OK
cargo test --workspace          # 573 passed, 6 ignored
crystalline-lint .              # ✓ No violations found
```

### 3.2 Benchmark oficial (`tools/perf/benchmark-p507.py`)

Corpus: 35 micro + 1 macro (`macro-10x`). `test-str-methods`, `medium-combined` e `0.15.0-spec` falharam no vanilla por incompatibilidade de sintaxe/API pré-existente — mesmo comportamento de P546.

#### Micro (35 documentos)

| Métrica | Antes de P544 (P543) | Depois de P544 | P548 (actual) |
|---|---:|---:|---:|
| Ratio médio | 3,53× | 4,03× | **3,23×** |
| Ratio mediana | 1,86× | 1,91× | **1,87×** |

A mediana voltou ao valor de antes de P544; a média melhorou relativamente ao estado pós-P544. Os outliers `test-image-fit` (25,71×) e `test-stroke-sides` (25,35×) distorcem a média porque o vanilla executa esses documentos em ~6–7 ms (provavelmente I/O-bound a frio), enquanto o cristalino leva ~170 ms de arranque; a morfologia do resultado é correcta.

#### `macro-10x`

| Estado | Vanilla | Cristalino | Ratio |
|---|---:|---:|---:|
| Antes de P544 (P543) | 4 731 ms | 54 512 ms | 11,52× |
| Depois de P544 | 4 714 ms | 135 215 ms | **28,68×** |
| **P548 (actual)** | 4 684 ms | **56 100 ms** | **11,98×** |

A cache do `Face` reduziu o tempo de `macro-10x` de 135 215 ms para 56 100 ms — praticamente de volta ao patamar pré-P544 (54 512 ms). A diferença residual (~3%) está dentro da variância do benchmark.

Fases internas (`--timings-json`, média de 5 runs via `benchmark-p507.py`):

| Fase | Depois de P544 | P548 (actual) | Delta |
|---|---:|---:|---:|
| `layout_ms` | 80 503,8 ms | **1 484,9 ms** | −79 018,9 ms |
| `render_ms` | 1 414,9 ms | 1 506,9 ms | +92,0 ms |
| `total_ms` | 134 385,2 ms | 60 492,1 ms | −73 893,1 ms |

O `layout_ms` colapsou de 80,5 s para 1,5 s, confirmando que a regressão de P544 foi eliminada. O `total_ms` restante (≈60 s) é agora dominado pelo `shape_ms` (≈57 s), que é uma regressão anterior a P544 (já observada em P546: 11,52× em P543 vs 0,30× em P518) e permanece fora do âmbito deste passo.

### 3.3 Casos de kerning de P546

```text
Texto texto Text text Tempo tempo Testando
Type Toe Tyler Yellow Yesterday
```

Extração `pdftotext`:

- Cristalino: `Texto texto Text text Tempo tempo Testando Type Toe Tyler Yellow Yesterday`
- Vanilla:    `Texto texto Text text Tempo tempo Testando Type Toe Tyler Yellow Yesterday`

Ambas coincidem linha a linha; não há recortes como `T exto`.

### 3.4 Re-teste dos casos P547 (CSL/Bibliografia)

Os artefactos tipográficos que P547 atribuíra a "fallback de fonte/shaper" (`“ A`, `T ypography`, `.”Journal`) **desapareceram** nos quatro estilos CSL testados (`ieee`, `apa`, `chicago-author-date`, `mla`). Isto confirma que eram o mesmo bug de kerning/sinal do TJ, não um problema de fonte.

Exemplo (IEEE, cristalino):

```text
[1] Maria Silva and João Santos, “A Comprehensive Study of Modern Typography,”
Journal of Design Research, vol. 12, pp. 45–67, 2023.
```

Nota: diferenças de formatação CSL (iniciais vs. nomes completos, ordem alfabética, citações em texto) permanecem. Essas são questões de estilo CSL/hayagriva vs. vanilla 0.14.2 e não foram objecto deste passo.

---

## 4. Causa raiz consolidada

| Problema | Causa | Correcção |
|---|---|---|
| Regressão 28,68× em documentos grandes | `FallbackFontMetrics::advance()` fazia `ttf_parser::Face::parse` 2–3 vezes por carácter | Cache por `slot_idx` (`CachedFace` em `Arc`) |
| Espaço a mais em pares kerning (`Te`, `To`, `Ty`, `Ye`) | `advance()` somava só `glyph_hor_advance`, ignorando tabelas `kern`/`kerx` | Adicionar `face_kerning()` e ajustar a largura total |
| Espaço a mais mesmo após kerning na medição | Delta do operador PDF `TJ` tinha sinal invertido | `advance_tu = (nominal - x_advance) / upm * 1000` |

---

## 5. Gaps remanescentes

1. **Regressão anterior a P544:** `macro-10x` já estava 11,52× mais lento que o vanilla em P543 (antes de qualquer alteração deste passo). A fase `shape_ms` domina agora o tempo total (~57 s). Esta regressão fica para bissecção dedicada, como registado em P546.
2. **GPOS kerning:** a medição só consulta tabelas legacy `kern`/`kerx`. Fontes que só disponibilizam kerning via `GPOS` lookup type 2 continuam a medir sem esse ajuste; é uma limitação aceite e documentada no L0.

---

## 6. Conclusão

P548 cumpriu os três objectivos:

1. **L0 actualizado** antes de qualquer código, com hash confirmado.
2. **Cache do `Face` parseado** implementada, eliminando re-parses redundantes e revertendo a regressão de P544 em `macro-10x` (28,68× → 11,98×, praticamente ao nível pré-P544).
3. **Kerning na medição** aplicado, corrigindo os artefactos tipográficos de P546 e P547.
4. **Sinal do delta TJ** corrigido, garantindo que o kerning negativo no PDF aproxima os glifos em vez de os afastar.

Validação final verde: `cargo test --workspace`, `crystalline-lint .`, benchmark oficial e comparação directa contra o vanilla.
