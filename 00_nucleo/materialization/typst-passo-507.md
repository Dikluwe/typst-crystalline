---

# P507 — DEBT-42 Benchmark: Performance Cristalino vs. Vanilla 0.15.0

> **Passo:** 507
> **Data:** 2026-06-30
> **Foco:** Medir empiricamente o tempo de compilação do cristalino vs. vanilla 0.15.0 em corpus representativo, avaliando o impacto da passagem dupla do eval (P498) e do runtime state (P506). Não declarar conclusão — medir antes de interpretar.
> **Tipo:** Benchmark empírico com análise de regressão.
> **Tamanho:** M (~2h de preparação + execução + análise).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P506 (paridade funcional completa), P498 (passagem dupla do eval), P503 (re-baseline 0.15.0).

---

## 1. Contexto

O projeto cristalino atingiu **paridade funcional completa** com o Typst 0.15.0 (37/37 MATCH em P490 + P500). No entanto, a arquitetura cristalina introduziu **dois custos conhecidos**:

1. **Passagem dupla do eval (P498):** O eval corre duas vezes — uma para introspection (AST original) e outra para layout (pós-show-rules). Isso duplica o trabalho de eval para cada ficheiro.
2. **Runtime state (P506):** O pipeline agora inclui `expand_context_blocks` pós-introspecção, que resolve `state`/`counter`/`context` blocks antes do layout.

Este passo mede o **impacto real** desses custos em comparação com o vanilla 0.15.0.

---

## 2. Metodologia

### 2.1 Corpus de Teste

Usar 3 categorias de documentos:

| Categoria | Descrição | Ficheiros | Tamanho |
|-----------|-----------|-----------|---------|
| **Micro** | Documentos simples (headings, parágrafos, listas) | `test-*.typ` da bateria P490 | 1-5 KB |
| **Médio** | Documentos reais (artigos, relatórios) | Corpus interno do projeto | 10-50 KB |
| **Macro** | Documentos complexos (livros, dissertações) | Corpus interno ou sintético | 100+ KB |

### 2.2 Métricas

| Métrica | Ferramenta | Comando |
|---------|------------|---------|
| **Tempo total** | `time` / `hyperfine` | `time typst compile doc.typ` |
| **Tempo por fase** | Instrumentação interna | `cargo run --release` com profiling |
| **Memória** | `valgrind` / `heaptrack` | `valgrind --tool=massif` |
| **Throughput** | Documentos/segundo | Corpus batch |

### 2.3 Comparação

| Versão | Comando | Notas |
|--------|---------|-------|
| **Vanilla 0.15.0** | `typst compile --timing doc.typ` | Baseline oficial |
| **Cristalino (release)** | `cargo run --release -p typst-wiring -- compile doc.typ` | Otimizado |
| **Cristalino (debug)** | `cargo run -p typst-wiring -- compile doc.typ` | Sem otimizações |

### 2.4 Fases do Cristalino a Medir

Usar instrumentação interna para medir:

1. **Parse:** `typst-wiring` parse do `.typ` → AST.
2. **Eval (passo 1):** Eval puro com `apply_show_rules = false` (introspection).
3. **Introspect:** Construção do `Introspector` e `TagIntrospector`.
4. **Expand context blocks:** Resolução de `state`/`counter`/`context` (P506).
5. **Eval (passo 2):** Eval com `apply_show_rules = true` (layout).
6. **Layout:** Layout engine → frames.
7. **Render:** PDF/HTML/PNG export.

---

## 3. Hipóteses

### 3.1 Hipótese A: Passagem dupla é o gargalo principal

> O tempo de eval duplicado (P498) representa >50% do overhead do cristalino vs. vanilla.

**Se confirmada:** Otimizar a passagem dupla (ex: cache de eval, lazy clone, ou merge dos dois passos).

### 3.2 Hipótese B: Runtime state é o gargalo principal

> O `expand_context_blocks` (P506) adiciona overhead significativo em documentos com muitos `state`/`counter`/`context`.

**Se confirmada:** Otimizar o pipeline de context expansion (ex: batch processing, lazy evaluation).

### 3.3 Hipótese C: O overhead é aceitável (<2x)

> O cristalino é <2x mais lento que o vanilla em todos os corpus, o que é aceitável para a funcionalidade extra (paridade completa + lookahead futuro).

**Se confirmada:** Prosseguir para P508 (Lookahead) sem otimização urgente.

### 3.4 Hipótese D: O overhead é inaceitável (>5x)

> O cristalino é >5x mais lento que o vanilla em corpus médio/grande.

**Se confirmada:** P508 = otimização de performance antes de Lookahead.

---

## 4. Implementação do Benchmark

### 4.1 Script de Benchmark

```bash
#!/bin/bash
# benchmark.sh

CORPUS_DIR="lab/parity/corpus"
VANILLA="typst"
CRISTALINO="cargo run --release -p typst-wiring --"

for doc in $CORPUS_DIR/p490/*.typ $CORPUS_DIR/p500/*.typ; do
    echo "=== $doc ==="

    # Vanilla
    hyperfine --warmup 3         "$VANILLA compile $doc /tmp/vanilla.pdf"         "$CRISTALINO compile $doc /tmp/cristalino.pdf"         --export-json "benchmark_$(basename $doc .typ).json"
done
```

### 4.2 Instrumentação Interna

Adicionar em `01_core/src/engine/eval/mod.rs` e `03_infra/src/pipeline.rs`:

```rust
use std::time::Instant;

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

    let pdf = render_pdf(&layout);
    let t7 = Instant::now();

    (pdf, Timings {
        parse: t1 - t0,
        eval_introspection: t2 - t1,
        introspect: t3 - t2,
        expand_context: t4 - t3,
        eval_layout: t5 - t4,
        layout: t6 - t5,
        render: t7 - t6,
        total: t7 - t0,
    })
}
```

### 4.3 Análise de Resultados

```python
# analyze.py
import json
import pandas as pd

results = []
for f in glob("benchmark_*.json"):
    data = json.load(open(f))
    for run in data["results"]:
        results.append({
            "doc": f,
            "tool": run["command"].split()[0],
            "mean_ms": run["mean"] * 1000,
            "stddev_ms": run["stddev"] * 1000,
        })

df = pd.DataFrame(results)
pivot = df.pivot(index="doc", columns="tool", values="mean_ms")
pivot["ratio"] = pivot["cristalino"] / pivot["typst"]
print(pivot)
```

---

## 5. Critério de Fecho

- [ ] Corpus de 20+ documentos preparado (P490 + P500 + documentos reais).
- [ ] Benchmark executado: vanilla 0.15.0 vs cristalino (release).
- [ ] Benchmark executado: cristalino (release) vs cristalino (debug).
- [ ] Instrumentação interna funcionando: 7 fases medidas.
- [ ] Tabela de resultados com tempo médio, desvio padrão, ratio cristalino/vanilla.
- [ ] Hipóteses A-D avaliadas com dados.
- [ ] Decisão documentada: P508 = Lookahead (se overhead aceitável) ou P508 = Otimização (se overhead inaceitável).
- [ ] `00_nucleo/diagnosticos/benchmark-p507.md` produzido com análise completa.
- [ ] Gráficos de performance gerados (se aplicável).

---

## 6. Próximo Passo (P508)

Dependendo dos resultados do benchmark:

| Cenário | Ratio Cristalino/Vanilla | Próximo Passo |
|---------|--------------------------|---------------|
| **A** | < 2x | P508 = Lookahead Layout Engine (inovação) |
| **B** | 2-5x | P508 = Otimização de Performance (P498 passagem dupla, P506 context expansion) |
| **C** | > 5x | P508 = Refactor Arquitetural (merge de passos, lazy eval) |

---

## A. Apêndice — Comandos de Benchmark

```bash
# Instalar hyperfine
cargo install hyperfine

# Vanilla 0.15.0
typst --version  # confirmar 0.15.0

# Cristalino release
cargo build --release -p typst-wiring

# Benchmark single file
hyperfine --warmup 3   'typst compile doc.typ /tmp/vanilla.pdf'   'cargo run --release -p typst-wiring -- compile doc.typ /tmp/cristalino.pdf'

# Benchmark corpus completo
for f in lab/parity/corpus/**/*.typ; do
  hyperfine --warmup 3     "typst compile $f /tmp/vanilla.pdf"     "cargo run --release -p typst-wiring -- compile $f /tmp/cristalino.pdf"     --export-json "benchmark_$(basename $f .typ).json"
done

# Análise
python3 analyze.py
```

---

## B. Apêndice — Fases do Pipeline Cristalino (para instrumentação)

```
[Parse] → [Eval 1: Introspection] → [Introspect] → [Expand Context] → [Eval 2: Layout] → [Layout] → [Render]
   │            │                        │                │                │           │          │
   │            │                        │                │                │           │          │
   ▼            ▼                        ▼                ▼                ▼           ▼          ▼
  t1           t2                       t3               t4               t5          t6         t7

Total = t7 - t1
Eval duplicado = (t2 - t1) + (t5 - t4)
Context expansion = t4 - t3
```

**Overhead conhecido:**
- Passagem dupla: ~2x o tempo de eval (se eval for 30% do total, overhead = +30%).
- Context expansion: proporcional ao número de `context` blocks (tipicamente <5% do total).
