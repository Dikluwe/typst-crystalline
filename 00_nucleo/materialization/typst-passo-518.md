---

# P518 — DEBT-42 Benchmark Revalidado: Performance com Produção Real

> **Passo:** 518
> **Data:** 2026-06-30
> **Foco:** Medir empiricamente o tempo de compilação do cristalino vs. Typst vanilla 0.15.0 em corpus representativo, com shaping + subsetting + system fonts ativos. Não declarar conclusão — medir antes de interpretar.
> **Tipo:** Benchmark empírico com análise de regressão.
> **Tamanho:** M (~2h de preparação + execução + análise).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica; benchmark mede mecânica.
> **ADR-0108 ACEITE** — medir antes de decidir; dados antes de conclusões.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P517 (system fonts + subset marking ativos), P516 (subsetting TrueType), P515 (fontdb + shaping), P514 (paridade de linguagem completa).

---

## 1. Contexto

O P507 executou um benchmark que foi **inválido** como medida de paridade porque o cristalino não fazia shaping real. Agora, com P515-P517 fechados, o cristalino tem:

- ✅ Shaping real (rustybuzz + font fallback)
- ✅ Descoberta de fontes do sistema (fontdb)
- ✅ Subsetting TrueType no PDF
- ✅ System fonts por defeito na CLI
- ✅ Marcação de subset no PDF (`AAAAAA+` prefix)

Este passo **revalida** o benchmark P507 com produção real, medindo o overhead genuíno do cristalino vs. vanilla 0.15.0.

**Nota sobre CFF subsetting:** O P516 documentou que CFF subsetting é **XL-size** e está como **scope-out** com fallback ativo. Este benchmark não inclui CFF — foca em TrueType, que é o caso mais comum.

---

## 2. Metodologia

### 2.1 Corpus de Teste

Usar 3 categorias de documentos:

| Categoria | Descrição | Ficheiros | Tamanho | Fontes |
|-----------|-----------|-----------|---------|--------|
| **Micro** | Documentos simples (headings, parágrafos, listas) | `test-*.typ` da bateria P490+P500 | 1-5 KB | 1-2 fontes |
| **Médio** | Documentos reais (artigos, relatórios) | Corpus interno + `00_nucleo/0.15.0.typ` | 10-50 KB | 2-4 fontes |
| **Macro** | Documentos complexos (livros, dissertações) | `medicao-pre-f-passo-318-corpus.typ` × 10 | 100+ KB | 3-6 fontes |

**Critério:** Todos os documentos devem usar **fontes do sistema** (não fontes embeddadas no projeto) para testar o caminho completo: descoberta → shaping → subsetting → PDF.

### 2.2 Métricas

| Métrica | Ferramenta | Comando |
|---------|------------|---------|
| **Tempo total** | `hyperfine` | `hyperfine --warmup 3 'typst compile doc.typ'` |
| **Tempo por fase** | Instrumentação interna | `--timings-json` (já implementado em P507) |
| **Tamanho do PDF** | `ls -la` | `ls -la out.pdf` |
| **Qualidade do PDF** | `pdffonts` | `pdffonts out.pdf` |
| **Throughput** | Documentos/segundo | Corpus batch |

### 2.3 Comparação

| Versão | Comando | Notas |
|--------|---------|-------|
| **Vanilla 0.15.0** | `typst compile --timing doc.typ` | Baseline oficial |
| **Cristalino (release)** | `cargo run --release -p typst-wiring -- compile doc.typ` | Otimizado |
| **Cristalino (debug)** | `cargo run -p typst-wiring -- compile doc.typ` | Sem otimizações |

### 2.4 Fases do Cristalino a Medir

Usar instrumentação interna (`--timings-json`) para medir:

1. **Parse:** `typst-wiring` parse do `.typ` → AST.
2. **Eval (passo 1):** Eval puro com `apply_show_rules = false` (introspection).
3. **Introspect:** Construção do `TagIntrospector`.
4. **Expand context:** Resolução de `Content::ContextBlock` (P506).
5. **Eval (passo 2):** Eval com `apply_show_rules = true` (layout).
6. **Layout:** Layout engine → frames com glifos posicionados.
7. **Shaping:** rustybuzz + font fallback.
8. **Subsetting:** Criação de subsets TrueType.
9. **Render:** PDF com fontes embeddadas.

---

## 3. Hipóteses

### 3.1 Hipótese A: Overhead é aceitável (<2×)

> O cristalino é <2× mais lento que o vanilla em todos os corpus, o que é aceitável para a funcionalidade extra (paridade completa + lookahead futuro).

**Se confirmada:** Prosseguir para P519 (Lookahead) ou P520 (publicação).

### 3.2 Hipótese B: Shaping é o gargalo principal

> O shaping (rustybuzz) representa >30% do tempo total.

**Se confirmada:** Otimizar shaping (cache de faces, batch processing, etc.).

### 3.3 Hipótese C: Subsetting é o gargalo principal

> O subsetting representa >30% do tempo total.

**Se confirmada:** Otimizar subsetting (cache de subsets, paralelização, etc.).

### 3.4 Hipótese D: Overhead é inaceitável (>5×)

> O cristalino é >5× mais lento que o vanilla em corpus médio/grande.

**Se confirmada:** P519 = otimização de performance antes de Lookahead.

---

## 4. Implementação do Benchmark

### 4.1 Script de Benchmark

```bash
#!/bin/bash
# benchmark-p518.sh

CORPUS_DIR="lab/parity/corpus"
VANILLA="typst"
CRISTALINO="cargo run --release -p typst-wiring --"
RESULTS_DIR="tools/perf/results/p518"

mkdir -p $RESULTS_DIR

for doc in $CORPUS_DIR/p490/*.typ $CORPUS_DIR/p500/*.typ; do
    echo "=== $(basename $doc) ==="

    # Vanilla
    hyperfine --warmup 3 --runs 10         "$VANILLA compile $doc /tmp/vanilla.pdf"         --export-json "$RESULTS_DIR/vanilla_$(basename $doc .typ).json"

    # Cristalino
    hyperfine --warmup 3 --runs 10         "$CRISTALINO compile $doc /tmp/cristalino.pdf"         --export-json "$RESULTS_DIR/cristalino_$(basename $doc .typ).json"

    # Tamanho do PDF
    ls -la /tmp/vanilla.pdf /tmp/cristalino.pdf >> $RESULTS_DIR/sizes.txt

    # Fontes no PDF
    pdffonts /tmp/cristalino.pdf >> $RESULTS_DIR/fonts.txt
done
```

### 4.2 Instrumentação Interna

```rust
// Adicionar em src/infra/pipeline.rs
pub fn compile_with_timings(doc: &str) -> (Document, Timings) {
    let t0 = Instant::now();
    let ast = parse(doc);
    let t1 = Instant::now();

    let (introspection_content, _) = eval(&ast, EvalMode::Introspection);
    let t2 = Instant::now();

    let introspector = Introspector::new(introspection_content.clone());
    let t3 = Instant::now();

    let expanded = expand_context_blocks(introspection_content, &introspector);
    let t4 = Instant::now();

    let (content, _) = eval(&ast, EvalMode::Layout);
    let t5 = Instant::now();

    let layout = layout(&content);
    let t6 = Instant::now();

    let shaped = shape_layout(&layout, &ctx.font_db);
    let t7 = Instant::now();

    let subsets = create_font_subsets(&shaped, &ctx.font_db);
    let t8 = Instant::now();

    let pdf = render_pdf(&shaped, &subsets);
    let t9 = Instant::now();

    (pdf, Timings {
        parse: t1 - t0,
        eval_introspection: t2 - t1,
        introspect: t3 - t2,
        expand_context: t4 - t3,
        eval_layout: t5 - t4,
        layout: t6 - t5,
        shaping: t7 - t6,
        subsetting: t8 - t7,
        render: t9 - t8,
        total: t9 - t0,
    })
}
```

### 4.3 Análise de Resultados

```python
# analyze-p518.py
import json
import pandas as pd

results = []
for f in glob("tools/perf/results/p518/*.json"):
    data = json.load(open(f))
    for run in data["results"]:
        results.append({
            "doc": f,
            "tool": run["command"].split()[0],
            "mean_ms": run["mean"] * 1000,
            "stddev_ms": run["stddev"] * 1000,
            "pdf_size_kb": get_pdf_size(f),
        })

df = pd.DataFrame(results)
pivot = df.pivot(index="doc", columns="tool", values="mean_ms")
pivot["ratio"] = pivot["cristalino"] / pivot["typst"]
pivot["pdf_ratio"] = pivot["cristalino_pdf_size"] / pivot["vanilla_pdf_size"]

print("=== Tempo ===")
print(pivot[["typst", "cristalino", "ratio"]])
print("
=== Tamanho PDF ===")
print(pivot[["vanilla_pdf_size", "cristalino_pdf_size", "pdf_ratio"]])
print("
=== Estatísticas ===")
print(f"Ratio médio: {pivot['ratio'].mean():.2f}x")
print(f"Ratio mediana: {pivot['ratio'].median():.2f}x")
print(f"Ratio min: {pivot['ratio'].min():.2f}x")
print(f"Ratio max: {pivot['ratio'].max():.2f}x")
```

---

## 5. Critério de Fecho

- [ ] Corpus de 37+ documentos preparado (P490 + P500 + documentos reais com fontes do sistema).
- [ ] Benchmark executado: vanilla 0.15.0 vs cristalino (release).
- [ ] Benchmark executado: cristalino (release) vs cristalino (debug).
- [ ] Instrumentação interna funcionando: 9 fases medidas.
- [ ] Tabela de resultados com tempo médio, desvio padrão, ratio cristalino/vanilla.
- [ ] Tabela de tamanhos de PDF comparando vanilla vs cristalino.
- [ ] Hipóteses A-D avaliadas com dados.
- [ ] Decisão documentada: P519 = Lookahead (se overhead aceitável) ou P519 = Otimização (se overhead inaceitável).
- [ ] `00_nucleo/diagnosticos/benchmark-p518.md` produzido com análise completa.
- [ ] Gráficos de performance gerados (se aplicável).

---

## 6. Nota sobre CFF Subsetting (XL-size)

O P516 documentou que **CFF subsetting é XL-size** e está como **scope-out** com fallback ativo. Este benchmark:

- **Não inclui** documentos com fontes CFF (PostScript outlines).
- **Foca** em TrueType, que é o caso mais comum em documentos reais.
- **Documenta** CFF como débito técnico futuro (DEBT-44).

**Se o corpus incluir fontes CFF:** O cristalino usará fallback (fonte completa embeddada), o que distorce o benchmark de tamanho. Filtrar documentos CFF ou documentar a distorção.

---

## 7. Próximo Passo (P519)

Dependendo dos resultados do benchmark:

| Cenário | Ratio Cristalino/Vanilla | Próximo Passo |
|---------|--------------------------|---------------|
| **A** | < 2× | P519 = Lookahead Layout Engine (inovação) |
| **B** | 2-5× | P519 = Otimização de Performance (shaping/subsetting) |
| **C** | > 5× | P519 = Refactor Arquitetural (merge de passos, cache) |

---

## A. Apêndice — Comandos de Reprodução

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Instalar hyperfine
cargo install hyperfine

# Vanilla 0.15.0
typst --version  # confirmar 0.15.0

# Cristalino release
cargo build --release -p typst-wiring

# Benchmark single file
hyperfine --warmup 3 --runs 10   'typst compile doc.typ /tmp/vanilla.pdf'   'cargo run --release -p typst-wiring -- compile doc.typ /tmp/cristalino.pdf'

# Benchmark corpus completo
for f in lab/parity/corpus/**/*.typ; do
  hyperfine --warmup 3 --runs 10     "typst compile $f /tmp/vanilla.pdf"     "cargo run --release -p typst-wiring -- compile $f /tmp/cristalino.pdf"     --export-json "tools/perf/results/p518/$(basename $f .typ).json"
done

# Análise
python3 tools/perf/analyze-p518.py
```

---

## B. Apêndice — Fases do Pipeline Cristalino (para instrumentação)

```
[Parse] → [Eval 1] → [Introspect] → [Expand Context] → [Eval 2] → [Layout] → [Shaping] → [Subsetting] → [Render]
   │         │            │                │               │          │          │           │          │
   ▼         ▼            ▼                ▼               ▼          ▼          ▼           ▼          ▼
  t1        t2           t3               t4              t5         t6         t7          t8         t9

Total = t9 - t1
Shaping = t7 - t6
Subsetting = t8 - t7
Eval duplicado = (t2 - t1) + (t5 - t4)
```

**Overhead conhecido:**
- Passagem dupla do eval: ~2× o tempo de eval.
- Shaping: proporcional ao número de runs de texto.
- Subsetting: proporcional ao número de fontes e glifos.
