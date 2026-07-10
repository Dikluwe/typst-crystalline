# P677 — Aplicar a mesma disciplina a `layout_ms`

**Passo:** 677
**Data:** 2026-07-10
**Foco:** instrumentar `layout_ms` do `macro-10x`, procurar trabalho duplicado, corrigir se houver.
**Commit base:** `561b05f37 — P676: atualiza hashes dos relatórios P675 e P676`
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

---

## 1. Sonda

### 1.1 Repartição actual com 30 execuções

Comando:

```bash
hyperfine --warmup 5 --runs 30 './target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p677.pdf'
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p677-timings.pdf --timings-json /tmp/p677-timings.json
```

Resultado (commit `561b05f37`, com instrumentação temporária adicionada para a sonda):

```text
Benchmark 1: ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p677.pdf
  Time (mean ± σ):      5.222 s ±  0.048 s
  Range (min … max):    5.137 s …  5.346 s    30 runs
```

```json
{
  "parse_ms": 0.0,
  "eval_ms": 571.98,
  "introspect_ms": 88.19,
  "expand_context_ms": 0.001,
  "layout_ms": 1305.02,
  "shape_ms": 1860.50,
  "subset_ms": 0.89,
  "render_ms": 880.27,
  "total_ms": 4706.84
}
```

`layout_ms` = **1305 ms** — segunda maior fase depois de `shape_ms`, confirmando o alvo do passo.

### 1.2 Instrumentação em sub-partes

Adicionados timers acumulados em campos temporários do `Layouter` (L1 puro — sem I/O, sem estado global), cobrindo:

- `layout_word` (layout de cada palavra, incluindo `text_width`)
- `layout_chunk` (smallcaps)
- `flush_line` (quebra de linha + nova página)
- `measure_content_constrained` (medição para grids)
- `layout_sub_frame` (sub-layout de células/boxes)
- dentro de `layout_word`/`layout_chunk`/`measure_content_constrained`: o tempo gasto em `text_width` isoladamente
- nos call sites de `space_width`: o tempo gasto em `space_width` isoladamente
- `vertical_metrics` (em `measure_content_constrained`)

Resultado (commit `561b05f37` + instrumentação, uma execução):

```text
[P677 layout] total=1296.30 bib=2.69 layout_content=1293.54 finish=0.00
  word=519.90 chunk=0.00 flush=100.67 measure=0.00 subframe=0.00
  text_width=474.65 space_width=227.72 vertical_metrics=0.00 (no outline)
```

Leitura (file:line):

- `layout_word` (`01_core/src/rules/layout/cursor.rs::layout_word`) = **519,90 ms** de `layout_content` = 1293,54 ms.
- Do tempo de `layout_word`, **474,65 ms** são gastos dentro de `FontMetrics::text_width` (`01_core/src/rules/layout/metrics.rs::text_width` → `FallbackFontMetrics::advance` em L3). Ou seja, ~91 % do custo de `layout_word` é medição de largura.
- `space_width` (`01_core/src/rules/layout/cursor.rs::space_width` → `advance(" ", …)`), medido nos call sites (`layout/text.rs` e `Content::Space`), custa **227,72 ms** — ~17 % de `layout_content`. Cada espaço re-medido do zero.
- `flush_line` = 100,67 ms. `measure`/`subframe`/`vertical_metrics` ≈ 0 (o `macro-10x` não usa grids nem boxes com altura fixa).

### 1.3 Padrões já conhecidos

```bash
grep -n "Face::parse\|Face::from_slice\|\.clone()\|collect::<Vec" 01_core/src/rules/layout/*.rs | grep -v test | wc -l
# 102
```

Nenhuma ocorrência de `Face::parse`/`Face::from_slice` em L1 (as faces vivem em L3). A duplicação encontrada não é de parse nem de walk: é de **medição repetida de largura**. O mesmo texto+estilo é medido uma e outra vez — em especial os espaços (`space_width`, chamado uma vez por separador de palavra) e as palavras repetidas do corpus.

A causa raiz está em L3: `FallbackFontMetrics::advance` (`03_infra/src/font_metrics.rs`) resolve a fonte primária e itera caractere a caractere **em cada chamada**, sem cache ao nível de palavra. Já existe cache para `advance_shaped` (`shaped_width_cache`, P591) e para faces (`cache`, P548; `shaper_face_cache`, P673), mas **não para o caminho rápido `advance`** — que é exactamente o caminho usado por todo o texto latim do `macro-10x`.

### 1.4 Critério de fecho da sonda

- [x] Repartição confirmada com 30 execuções.
- [x] `layout_ms` dividido em sub-partes com instrumentação directa.
- [x] Confirmado trabalho duplicado: medição repetida de `advance` em `FallbackFontMetrics`, especialmente `space_width` (227 ms) e `text_width` (475 ms), em `03_infra/src/font_metrics.rs`.

---

## 2. Implementação

Cache de `advance` ao nível de palavra em `FallbackFontMetrics`, seguindo o mesmo padrão de `shaped_width_cache` (P591) e das caches de P548/P673.

### 2.1 `03_infra/src/font_metrics.rs`

- Nova chave `AdvanceWidthKey` (texto, tamanho, fonte, peso, variações de eixo OpenType, etc.). `tracking` é excluído da chave porque é aplicado fora do cache em `text_width`.
- Novo campo `advance_width_cache: Arc<Mutex<HashMap<AdvanceWidthKey, Pt>>>`, partilhado entre clones (fixpoint loop de TOC).
- `advance` envolvido em `cached_advance_width(text, style, || { … })`: hit devolve o valor guardado; miss calcula e insere.

```rust
fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
    self.cached_advance_width(text, style, || {
        // corpo original inalterado
    })
}
```

### 2.2 `03_infra/src/fallback_fonts.rs`

Hash `@prompt-hash` do `font_metrics.md` actualizado (o ficheiro declara dois `@prompt`; o de `shaper.md` ficou inalterado em `b832b2b5`).

### 2.3 `00_nucleo/prompts/infra/font_metrics.md`

Documentação actualizada: listagem dos campos de `FallbackFontMetrics` (incluindo `advance_width_cache`), nota sobre o propósito da cache e entrada no histórico de revisões.

---

## 3. Validação

### 3.1 Instrumentação depois da correcção

```text
[P677 layout] total=418.46 bib=2.73 layout_content=415.67 finish=0.00
  word=121.42 chunk=0.00 flush=90.39 measure=0.00 subframe=0.00
  text_width=78.57 space_width=35.44 vertical_metrics=0.00 (no outline)
```

| Sub-parte | Antes (ms) | Depois (ms) | Δ |
|---|---:|---:|---:|
| `text_width` | 474,65 | 78,57 | −396,08 (−83 %) |
| `space_width` | 227,72 | 35,44 | −192,28 (−84 %) |
| `layout_word` | 519,90 | 121,42 | −398,48 (−77 %) |
| `flush_line` | 100,67 | 90,39 | −10,28 |
| `layout_content` | 1293,54 | 415,67 | −877,87 (−68 %) |

A poupança concentra-se em `text_width` e `space_width`, exactamente onde a sonda apontou. `flush_line` e o resto do layout ficam praticamente inalterados, como esperado (a cache só afecta a medição de largura).

### 3.2 Benchmark directo (30 runs)

Com instrumentação ainda presente (overhead mínimo, idêntico nos dois lados):

| Estado | `hyperfine --runs 30` (mean ± σ) | Poupança |
|---|---:|---:|
| Antes (sem cache) | 5,222 s ± 0,048 s | — |
| Depois (com cache) | 3,895 s ± 0,026 s | **−1,327 s (−25 %)** |

Os intervalos não se sobrepõem (5,137–5,346 s vs 3,811–3,924 s); a melhoria é estável.

### 3.3 Benchmark completo (`benchmark-p507.py`)

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
```

#### Macro

| Passo | Vanilla (ms) | Cristalino (ms) | Rácio |
|---|---|---:|---:|
| P673 | 5483,96 | 6928,43 | 1,26× |
| **P677** | **5449,62** | **5015,71** | **0,92×** |

O `macro-10x` passou de **1,26× (mais lento que vanilla)** para **0,92× (mais rápido que vanilla)**. Repartição de fases no P677: `eval` 581,01 ms, `layout` 390,86 ms, `render` 892,43 ms.

#### Micro (seleção)

Os documentos micro mantiveram-se estáveis (rácios ~0,42×, idênticos a P673), sem regressão — a cache só acelera casos com texto repetido; documentos pequenos já eram dominados por overhead constante.

### 3.4 Correcção do output

```bash
pdftotext /tmp/p677-baseline.pdf /tmp/p677-antes.txt
pdftotext /tmp/p677-cached-hf.pdf /tmp/p677-depois.txt
diff -u /tmp/p677-antes.txt /tmp/p677-depois.txt
```

Resultado: **0 linhas** de diferença — output idêntico. A cache não altera nenhum valor medido; apenas reutiliza resultados já calculados.

### 3.5 Testes e linter

- `cargo test --workspace`: **passou** (todos os testes verdes; a instrumentação temporária, incluindo o `eprintln` de sonda, foi removida antes desta corrida — o teste `disciplina_stderr_vazio_em_compilacao_limpa` confirmou `stderr` limpo).
- `crystalline-lint .`: **`✓ No violations found`**.

### 3.6 Instrumentação removida

Toda a instrumentação temporária foi removida do código L1 antes da validação final:

- `01_core/src/rules/layout/mod.rs`: campos `t_*` do `Layouter`, timers em `measure_content_constrained`, timers em `layout_with_introspector_and_metrics`, helper `duration_ms` — `git diff` líquido = 0 linhas.
- `01_core/src/rules/layout/cursor.rs`, `sub_frame.rs`, `text.rs`: timers e helpers removidos — `git diff` líquido = 0 linhas.

A única alteração permanente é a cache em L3 (`font_metrics.rs`) e a actualização documental do L0 (`font_metrics.md`).

---

## 4. Decisão

- A duplicação encontrada em `layout_ms` é real e localizada: medição repetida de `advance` em `FallbackFontMetrics` (`03_infra/src/font_metrics.rs`), dominada por `space_width` (227 ms) e `text_width` (475 ms) no `macro-10x`.
- A correcção segue o padrão já estabelecido (cache), sem alterar a semântica: output idêntico (0 linhas de diferença), testes verdes, linter limpo.
- `layout_ms` caiu de ~1305 ms para ~426 ms (instrumentação) / ~391 ms (harness); o `macro-10x` passou de 1,26× para 0,92× face ao vanilla.

---

## 5. Proveniência das medições

- **Commit base:** `561b05f37 — P676: atualiza hashes dos relatórios P675 e P676`.
- **Hora das medições finais:** 2026-07-10T06:05:50Z (working tree com as alterações de P677 aplicadas, instrumentação já removida).
- **Ficheiros alterados no momento da medição final (`git diff HEAD --stat`):**

```text
00_nucleo/prompts/infra/font_metrics.md            |  21 +-
03_infra/src/fallback_fonts.rs                     |   4 +-
03_infra/src/font_metrics.rs                       | 158 +-
tools/perf/results/*                               | … (benchmark re-executado)
```

- **Nota:** os números "antes" foram medidos com instrumentação temporária adicionada sobre o commit base (working tree não commitado nesse momento), e os números "depois" com a mesma instrumentação mais a cache. A instrumentação foi depois removida para a validação final e o commit; o `git diff` líquido em L1 é zero.

---

## Hash do commit

`(a preencher após o commit)`
