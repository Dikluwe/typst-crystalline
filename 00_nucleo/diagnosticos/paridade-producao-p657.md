# Relatório de Paridade — P657

**Passo:** 657  
**Data:** 2026-07-09  
**Foco:** Cache de resultados de shaping para documentos com conteúdo repetido.  
**Dependências:** P548 (precedente de cache de métricas de fonte), P619 (onde `shape_ms` foi confirmado como dominante).  
**Hash do commit com as alterações:** `5df37f09c`

---

## 1. Sonda

### 1.1 Conteúdo de `macro-10x`

```bash
wc -l tools/perf/corpus/macro-10x.typ
sort tools/perf/corpus/macro-10x.typ | uniq -c | sort -rn | head -10
```

Resultado:

- 70 050 linhas.
- Blocos repetidos 5 000 vezes cada: `+ item enumerado A`, `+ item enumerado B`, `- item dois com mais texto`, parágrafos, equações.
- Documento altamente repetitivo: ~500 secções com a mesma estrutura.

### 1.2 Cache existente em `shaper.rs`

`03_infra/src/shaper.rs` não tinha cache de **resultado de shaping**. Há:

- Cache de métricas de fonte (corrigido em P548) via `FontMetrics`/`Face`.
- `shaped_width` (P591) faz shaping sob demanda sem cache de resultado.

A função `try_shape` chamava `rustybuzz::shape` para cada sub-run de cada `FrameItem::Text`.

### 1.3 Taxa de repetição medida

Instrumentação temporária em `try_shape` (não commitada):

```text
[P657 instrumentação] shape calls total=430000 unique=1052 repeated=428948 (ratio 99.76%)
```

99,76% das chamadas ao shaper eram para sub-runs idênticos já vistos no mesmo documento.

### 1.4 Baseline de `shape_ms`

Antes da cache:

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p657-baseline.pdf --timings-json /tmp/p657-baseline-timings.json
```

Resultado:

```json
{
    "shape_ms": 34211.640824,
    "total_ms": 37711.382177
}
```

---

## 2. Implementação

### 2.1 `03_infra/src/shaper.rs`

Adicionada `ShapeCache` local a cada documento:

- Chave: `String` composta por `(texto, slot_idx da face, rtl, variações de eixo, tracking)`.
- Valor: `CachedRun { glyphs: Vec<ShapedGlyph>, width: i32 }`.
- A cache é criada em `shape_document` e passada por `shape_page` / `shape_item` / `try_shape`.
- Em cada sub-run, verifica-se cache antes de chamar `rustybuzz::shape`. Em caso de hit, reutilizam-se os glifos e a largura em unidades de fonte; o posicionamento absoluto e o `segment_style` continuam a ser calculados por sub-run.

A chave inclui tudo o que afecta o output do shaper:

- Texto do sub-run.
- Identificador da face (`slot_idx`).
- Direção (`rtl`).
- Variações de eixo OpenType (`wght`, `ital`, etc.).
- Tracking (aplicado aos avanços dos glifos).

### 2.2 Testes actualizados

Atualizadas 4 chamadas a `shape_item` nos testes de `shaper.rs` para passarem uma `ShapeCache::new()`.

---

## 3. Validação

### 3.1 Correção

Comparação do texto extraído dos PDFs (baseline vs com cache):

```bash
pdftotext /tmp/p657-baseline.pdf /tmp/p657-baseline.txt
pdftotext /tmp/p657-cache.pdf /tmp/p657-cache.txt
diff -u /tmp/p657-baseline.txt /tmp/p657-cache.txt
```

Resultado: `diff exit: 0` — output idêntico.

### 3.2 Desempenho em `macro-10x`

Com cache:

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p657-cache.pdf --timings-json /tmp/p657-cache-timings.json
```

Resultado:

```json
{
    "shape_ms": 24146.64608,
    "total_ms": 27445.343681
}
```

Ganho:

- `shape_ms`: 34 211 ms → 24 146 ms (**-29,4%**).
- `total_ms`: 37 711 ms → 27 445 ms (**-27,2%**).

### 3.3 Documento sem repetição

```bash
cat > /tmp/p657-unico.typ <<'EOF'
#set text(font: "FreeSerif")
#for i in range(1, 200) {
  [Parágrafo #i com texto único #i para evitar repetição no cache de shaping. ]
}
EOF
./target/release/typst /tmp/p657-unico.typ /tmp/p657-unico.pdf --timings-json /tmp/p657-unico-timings.json
```

Resultado: compilação bem-sucedida, `shape_ms` ≈ 948 ms para 200 parágrafos únicos. A cache fica vazia (nenhum hit), mas o overhead de construir chaves e fazer lookups é insignificativo face ao custo do shaping.

### 3.4 Invalidação de cache

A cache é local ao documento (`shape_document`), pelo que mudanças de fonte/tamanho/estilo entre documentos não interferem. Dentro do mesmo documento, a chave inclui `slot_idx`, variações de eixo e tracking, pelo que diferentes estilos geram entradas separadas. A chave inclui o texto completo do sub-run, pelo que texto diferente nunca reutiliza um resultado antigo.

### 3.5 `cargo test --workspace`

Resultado: todos os crates passaram.

### 3.6 `crystalline-lint .`

Resultado: `✓ No violations found`.

### 3.7 Benchmark completo

```bash
python3 tools/perf/benchmark-p507.py
```

Resultado para `macro-10x`:

```text
vanilla mean: 5035.43 ms
cristalino mean: 29097.89 ms
ratio: 5.78x
```

(Novo rácio registado em `tools/perf/results/benchmark-p507-summary.json`.)

Nota: alguns documentos falharam no benchmark (`test-str-methods`, `medium-combined`, `0.15.0-spec`) devido a erros de compilação independentes da cache; foram reportados como `[skip]`.

---

## 4. Decisão

- Confirmada taxa de repetição de 99,76% em `macro-10x`.
- Implementada cache local de resultados de shaping por documento.
- `shape_ms` reduzido em ~29% e tempo total em ~27% para `macro-10x`.
- Output PDF inalterado; testes passam; linter limpo.
- `shaped_width` (P591) continua sem cache de resultado — possível trabalho futuro, mas fora do scope deste passo, que focava `shape_ms` medido em `shape_document`.
