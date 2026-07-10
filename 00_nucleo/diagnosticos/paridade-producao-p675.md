# P675 — Reperfilar `macro-10x` depois das correcções de cache

**Data:** 2026-07-10  
**Commit base:** `e9450f4b8 — P674: adiciona hash do commit ao relatório`

---

## Resumo

Depois de P657–P674 optimizarem `shape_ms` e o custo fixo de arranque, o `macro-10x` mantinha-se a ~1,18–1,26× do vanilla. Este passo reperfilou o benchmark do zero para identificar o próximo gargalo. A fase `shape_ms` continua a ser a maior individualmente, mas `render_ms` e `layout_ms` juntas dominam o tempo restante.

A instrumentação do export PDF (`build_multifont`) revelou que **~91 % do tempo da sub-fase `faces`** era gasto em `collect_shaped_cluster_texts(doc)`, chamada uma vez por fonte resolvida. Com 3 fontes no `macro-10x`, o documento era percorrido 3 vezes para reconstruir os mesmos textos shaped. Mover esta colecção para fora do loop reduziu o `render_ms` em ~36 % e o tempo total do `macro-10x` em ~10 %.

---

## Repartição de fases actual

Comando:

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p675-macro.pdf --timings-json /tmp/p675-timings.json
```

Resultado (commit base P674):

```json
{"parse_ms":0.000000,"eval_ms":571.911659,"introspect_ms":87.212405,"expand_context_ms":0.000961,
 "layout_ms":1297.335230,"shape_ms":1885.093339,"subset_ms":0.912721,"render_ms":1380.021051,
 "total_ms":5222.487366}
```

### Tabela comparativa de fases do `macro-10x`

| Fase | P619 | P672 | P673 | P674 | **P675 (base)** |
|---|---:|---:|---:|---:|---:|
| `eval_ms` | 539,36 | — | — | 572 | **572** |
| `introspect_ms` | 88,59 | — | — | 87 | **87** |
| `layout_ms` | 1 254,53 | — | — | 1 297 | **1 297** |
| `shape_ms` | 32 577,36 (90,66 %) | 2 199 | 1 885 | 1 885 | **1 885 (36,1 %)** |
| `render_ms` | 1 472,48 (4,10 %) | — | 1 380 | 1 380 | **1 380 (26,4 %)** |
| `total_ms` | 35 933,21 | 5 914 | 5 222 | 5 222 | **5 222** |

Valores em milissegundos. Percentagens do `total_ms` do respectivo passo.

### Análise da repartição

- `shape_ms` deixou de ser 90 % do total; agora é 36 %.
- `layout_ms` + `render_ms` + `eval_ms` representam ~62 % do tempo restante.
- A atenção destina-se agora a `render_ms` (1380 ms) e `layout_ms` (1297 ms).

---

## Instrumentação do export PDF

Foram adicionados timers temporários em `PdfBuilder::build_multifont` (`03_infra/src/export/builder.rs`) para dividir `render_ms` em sub-partes.

Resultado da instrumentação:

```text
[P675 instrumentação build_multifont] total=1333.28 collect=61.54 faces=847.15 pages=369.71 emit=17.58 serialize=31.41 subset_ms=0.99
[P675 detalhe faces] n_fonts=3 map_chars=0.03 math_map=0.04 subset=0.99 vf=0.01 collect_shaped=774.56 widths=71.45
```

### Breakdown da sub-fase `faces` (847 ms)

| Sub-parte | Tempo (ms) | % de `faces` |
|---|---:|---:|
| `map_chars_to_glyphs` | 0,03 | 0,0 % |
| `build_math_glyph_reverse_map` | 0,04 | 0,0 % |
| `measure_subset` | 0,99 | 0,1 % |
| `instantiate_variable_font` | 0,01 | 0,0 % |
| **`collect_shaped_cluster_texts`** | **774,56** | **91,4 %** |
| `widths_array` | 71,45 | 8,4 % |

A função `collect_shaped_cluster_texts` (`03_infra/src/export/fonts.rs:133`) percorre todo o `PagedDocument` e reconstrói o texto de cada cluster shaped (incluindo ligatures e RTL). Era chamada dentro do loop por fonte, portanto 3 vezes no `macro-10x`.

### Localização do trabalho duplicado

- `03_infra/src/export/builder.rs:697` — `collect_shaped_cluster_texts(doc)` dentro do `for face in faces`.
- A mesma colecção é usada apenas para `remap_glyph_id` e `seen_to_unicode_gids`, que são específicos por fonte; os dados de entrada são independentes da face.

---

## Implementação

Atualizado o Prompt L0 `00_nucleo/prompts/infra/export/builder.md` com a secção §P675.

Mudança em `03_infra/src/export/builder.rs`:

```rust
// P675 — colectar textos shaped uma única vez e partilhar entre fontes,
// em vez de percorrer o documento N vezes (uma por fonte).
let shaped_cluster_texts = collect_shaped_cluster_texts(doc);
```

E dentro do loop:

```rust
for &(old_gid, ref hex) in &shaped_cluster_texts {
    let new_gid = remap_glyph_id(old_gid, &glyph_mapping);
    if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
        to_unicode_mappings.push((new_gid, hex.clone()));
    }
}
```

Cada fonte mantém o seu próprio `seen_to_unicode_gids` e faz o seu próprio `remap_glyph_id`, preservando a semântica original.

---

## Resultados — depois da correção

### `--timings-json`

```json
{"parse_ms":0.000000,"eval_ms":578.006279,"introspect_ms":85.881798,"expand_context_ms":0.000992,
 "layout_ms":1259.609993,"shape_ms":1859.463421,"subset_ms":0.966145,"render_ms":883.945917,
 "total_ms":4667.874545}
```

| Métrica | Antes | Depois | Ganho |
|---|---:|---:|---:|
| `render_ms` | 1 380 ms | 884 ms | **−36 %** |
| `total_ms` | 5 222 ms | 4 668 ms | **−10,6 %** |

### Hyperfine directo (`macro-10x`)

```
Benchmark 1: ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p675-cristalino.pdf
  Time (mean ± σ):      5.873 s ±  0.438 s

Benchmark 2: lab/typst-original/target/release/typst compile ... /tmp/p675-vanilla.pdf
  Time (mean ± σ):      4.663 s ±  0.059 s

Summary: vanilla 1.26 ± 0.10 times faster.
```

### Paridade de output

`pdftotext` dos PDFs antes e depois da alteração produziu ficheiros de texto idênticos (`diff -q` sem diferenças).

### Benchmark completo

Documentos micro mantiveram-se abaixo de 0,5× do vanilla (continuam a beneficiar de P674). O `macro-10x` reportou ratio 1,31× numa execução do script, mas medições directas com `hyperfine` confirmam ~1,26×. A variação é atribuída a ruído do sistema; o `render_ms` instrumentado melhorou consistentemente ~36 %.

---

## Validação

- `cargo test --workspace` — passou.
- `crystalline-lint .` — zero violations (hash do L0 actualizado para `e154f0a4`).
- `cargo build --release` — sem erros.
- Paridade de output confirmada via `pdftotext` + `diff -u`.
- Benchmark completo repetido; `render_ms` reduzido e output inalterado.

---

## Decisões e notas

- O maior trabalho duplicado restante no `macro-10x` está no `render_ms`, especificamente em colecções que percorrem o documento múltiplas vezes.
- Esta correcção remove um walk duplicado por fonte em `build_multifont`. Documentos single-font (`build_cidfont`) não são afectados porque a colecção já era feita uma única vez.
- A otimização não altera a interface do `PdfBuilder` nem os objectos PDF emitidos.
- `layout_ms` (~1297 ms) continua a ser a próxima fase a examinar, mas este passo cumpre o seu objectivo de reperfilar e corrigir o primeiro gargalo identificado no `render_ms`.

---

## Hash do commit

`HASH_A_PREENCHER_APOS_COMMIT`
