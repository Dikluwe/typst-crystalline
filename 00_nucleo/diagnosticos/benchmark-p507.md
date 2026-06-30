# Benchmark de Performance — Passo 507

**Data:** 2026-06-30  
**Passo:** 507  
**Foco:** Medir empiricamente o tempo de compilação do cristalino vs. Typst vanilla 0.15.0, avaliando o impacto da passagem dupla do eval (P498) e do runtime state (P506).  
**Estado:** Concluído.

---

## 1. Metodologia

### 1.1 Ferramentas

- **Vanilla 0.15.0:** binário compilado a partir de `lab/typst-original/` (`cargo build --release -p typst-cli`).
- **Cristalino release:** `target/release/typst` (`cargo build --release -p typst-wiring`).
- **Benchmark externo:** [hyperfine](https://github.com/sharkdp/hyperfine) v1.x (instalado via `cargo install hyperfine`).
- **Instrumentação interna:** flag `--timings-json <file>` adicionada ao CLI cristalino; mede as fases:
  - `eval_ms` — eval com passagem dupla (P498);
  - `introspect_ms` — construção do `TagIntrospector`;
  - `expand_context_ms` — expansão de `Content::ContextBlock` (P506);
  - `layout_ms` — layout engine → `PagedDocument`;
  - `render_ms` — shaping + export PDF.

### 1.2 Corpus

| Categoria | Documentos | Fonte |
|-----------|------------|-------|
| Micro | 37 ficheiros `.typ` | `lab/parity/corpus/p490/` (20) + `lab/parity/corpus/p500/` (17) |
| Médio | 2 documentos | `benches/corpus/*.typ` concatenados + `00_nucleo/0.15.0.typ` |
| Macro | 1 documento | `00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ` × 10 |

Foram gerados em `tools/perf/corpus/`:
- `medium-combined.typ`
- `macro-10x.typ`

### 1.3 Script

- `tools/perf/benchmark-p507.py` — orquestra hyperfine, recolhe fases internas e produz JSON.
- `tools/perf/results/benchmark-p507-summary.json` — dados brutos.
- `tools/perf/results/benchmark-p507.log` — log de execução.

Cada documento foi corrido com `--warmup 1 --runs 5` no hyperfine. As fases internas do cristalino foram medidas em 5 runs com `--timings-json`.

---

## 2. Resultados Agregados

### 2.1 Tempo Total (vanilla vs cristalino)

| Categoria | N | Vanilla médio | Cristalino médio | Ratio médio | Ratio mediana | Min | Max |
|-----------|---|---------------|------------------|-------------|---------------|-----|-----|
| Micro | 34 | 96.85 ms | 4.70 ms | **0.067×** | 0.030× | 0.021× | 0.565× |
| Macro | 1 | 4750.11 ms | 1233.16 ms | **0.260×** | 0.260× | 0.260× | 0.260× |

**Interpretação:** o cristalino foi mais rápido que o vanilla em todos os documentos comparáveis. A razão média é fortemente puxada por 3 documentos em que o vanilla tem overhead maior (bibliografia, imagem, stroke-sides); a mediana micro é 0.030×.

> **Caveat:** o vanilla 0.15.0 pode ter overhead de inicialização (descoberta/indexação de fontes do sistema) que o cristalino ainda não replica. Os números refletem o comportamento observado no ambiente de teste, não uma comparação teórica de throughput puro.

### 2.2 Fases Internas do Cristalino

#### Micro (média de 34 docs)

| Fase | Média (ms) | % do total |
|------|------------|------------|
| `eval_ms` | 0.525 | 24.0 % |
| `introspect_ms` | 0.013 | 0.6 % |
| `expand_context_ms` | 0.003 | 0.1 % |
| `layout_ms` | 1.557 | 71.3 % |
| `render_ms` | 0.086 | 3.9 % |
| **total_ms** | **2.184** | 100 % |

#### Macro (`macro-10x.typ`)

| Fase | Média (ms) | % do total |
|------|------------|------------|
| `eval_ms` | 397.52 | 41.7 % |
| `introspect_ms` | 66.33 | 7.0 % |
| `expand_context_ms` | 0.01 | <0.1 % |
| `layout_ms` | 226.54 | 23.8 % |
| `render_ms` | 262.05 | 27.5 % |
| **total_ms** | **952.45** | 100 % |

**Observações:**
- `expand_context_ms` (P506) é negligenciável em todos os casos (<0.1 % no micro, <0.1 % no macro).
- A passagem dupla do eval é visível, mas no micro domina o `layout_ms`; no macro o `eval_ms` representa ~42 % do tempo, o que o torna o maior candidato a otimização futura.
- A fase de introspecção cresce com o tamanho do documento, mas permanece abaixo de 10 % do total.

---

## 3. Documentos Não Comparáveis

5 documentos falharam no vanilla 0.15.0 por usarem funcionalidades não presentes nesta baseline:

| Documento | Erro no vanilla |
|-----------|-----------------|
| `test-calc-rest.typ` | `module calc does not contain log10` |
| `test-metadata-query.typ` | API de query/metadata divergente |
| `test-str-methods.typ` | método de string não suportado |
| `medium-combined.typ` | herda falha de um dos ficheiros incluídos |
| `0.15.0-spec.typ` | herda falha de um dos ficheiros incluídos |

Estes documentos foram excluídos da comparação vanilla/cristalino, mas o cristalino compilou-os com sucesso.

---

## 4. Avaliação das Hipóteses

| Hipótese | Estado | Justificação |
|----------|--------|--------------|
| **A:** Passagem dupla é o gargalo principal | **Parcialmente confirmada no macro** | No `macro-10x.typ` o eval consome 41.7 % do tempo, tornando-se a fase mais cara. No micro o layout ainda domina. |
| **B:** Runtime state é o gargalo principal | **Rejeitada** | `expand_context_ms` é sempre <0.1 % do tempo. |
| **C:** Overhead é aceitável (<2×) | **Confirmada** | O cristalino é *mais rápido* que o vanilla em todos os documentos comparáveis; nenhum ratio >1. |
| **D:** Overhead é inaceitável (>5×) | **Rejeitada** | Não há nenhum caso de slowdown. |

---

## 5. Decisão para P508

De acordo com a tabela do Passo 507:

| Cenário | Ratio Cristalino/Vanilla | Próximo Passo |
|---------|--------------------------|---------------|
| **A** | < 2× | P508 = Lookahead Layout Engine (inovação) |
| **B** | 2-5× | P508 = Otimização de Performance |
| **C** | > 5× | P508 = Refactor Arquitetural |

Os dados colocam o projeto no **cenário A**: ratio < 2× (de facto << 1×). Recomenda-se:

> **P508 = Lookahead Layout Engine** — prosseguir com a inovação arquitetural, mantendo a otimização do eval duplo como débito técnico monitorizado (DEBT-42).

---

## 6. Débito Técnico Identificado

- **DEBT-42:** overhead da passagem dupla do eval em documentos grandes. Quando P508 estabilizar, considerar cache de eval entre as duas passagens ou merge lazy das fases de introspection e layout.
- **DEBT-43:** diferença de overhead de inicialização vanilla vs cristalino (fontes, etc.) pode distorcer ratios em documentos muito pequenos. Futuros benchmarks devem usar documentos ≥ médios para comparações mais estáveis.

---

## 7. Anexos

### 7.1 Comandos de Reprodução

```bash
# Compilar vanilla 0.15.0
cd lab/typst-original
cp Cargo.toml.original Cargo.toml
cargo build --release -p typst-cli

# Compilar cristalino release
cd ../..
cargo build --release -p typst-wiring

# Executar benchmark
python3 tools/perf/benchmark-p507.py
```

### 7.2 Ficheiros Gerados

- `tools/perf/results/benchmark-p507-summary.json`
- `tools/perf/results/benchmark-p507.log`
- `tools/perf/results/hyperfine-*.json`

### 7.3 Hash do Commit de Referência

Benchmark executado sobre o commit `969087ecf` (P506 finalizado).
