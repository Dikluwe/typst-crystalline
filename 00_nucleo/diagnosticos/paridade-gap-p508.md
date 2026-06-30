# Diagnóstico de Paridade — Gap vs Vanilla 0.15.0

**Data:** 2026-06-29
**Referência:** pós-P507 (commit `15e291f65`)
**Propósito:** Mapear o que falta para paridade funcional e estrutural com o Typst vanilla 0.15.0, distinguindo lacunas que distorcem métricas de lacunas reais de linguagem.

---

## 1. O que distorce o benchmark P507

O ratio de 0.03× (cristalino 33× mais rápido) nos documentos micro **não é paridade** — é ausência de trabalho real.

| Fase | Cristalino micro | Vanilla micro | Causa da diferença |
|------|-----------------|---------------|--------------------|
| `render_ms` | 0.01–0.10 ms | ~97 ms (embutido no total) | Cristalino não executa shaping real nem subsetting de fontes |
| `eval_ms` | 0.5 ms | — | Corpus micro é trivial |
| Total | ~3–5 ms | ~97–155 ms | Vanilla inicializa `fontdb`, faz shaping rustybuzz, gera CID PDF |

O vanilla 0.15.0 ao arrancar: (1) descobre e indexa fontes do sistema via `fontdb`, (2) executa shaping por glifo via rustybuzz, (3) gera PDF com embedding de CID fonts subseteadas. O cristalino ainda não replica nenhum destes três pontos em produção.

**Corolário:** qualquer benchmark com documentos pequenos (<50 linhas) é inválido como comparação de throughput até a Trilha 5 (shaping) estar concluída.

---

## 2. Brechas estruturais — camadas inteiras ausentes

Estas são ausências de subsistemas completos, não de features individuais.

| Subsistema | Estado | LOC vanilla estimado | Nota |
|------------|--------|----------------------|------|
| **Shaping real (rustybuzz)** | Stub — `FrameItem::Text` sequencial sem posicionamento | ~épico XL | Trilha 5, não iniciada pós-P480 |
| **Subsetting de fontes no PDF** | Ausente — PDF gerado sem embedding correto | parte do export | Bloqueado pelo shaping |
| **Inicialização de fontdb** | Ausente — cristalino não descobre fontes do sistema | parte de L3 world | Explica 90 ms do overhead vanilla |
| **HTML export** (`typst-html`) | 0 % | ~4 981 LOC | Não planeado |
| **SVG export** (`typst-svg`) | 0 % | ~2 033 LOC | Não planeado |
| **Raster render** (`typst-render`) | 0 % | ~1 127 LOC | Não planeado |
| **IDE** (`typst-ide`) | 0 % | ~4 532 LOC | Não planeado |

> **Nota:** HTML/SVG/raster/IDE estão fora do escopo declarado do cristalino (que visa PDF). A ausência de shaping é o único bloqueador crítico de paridade real.

---

## 3. Brechas funcionais — features da linguagem

### 3.1 Stdlib — funções ausentes ou incompletas

| Função / Método | Estado | Documento que falha |
|-----------------|--------|---------------------|
| `str` field access (`.len`, `.first`, etc.) | Ausente | `test-str-methods` |
| `str.split()`, `str.trim()`, `str.replace()` | Ausente | `test-str-methods` |
| `dict.insert()`, `dict.len()`, `dict.remove()` | Ausente | `test-dict-methods` (passa por acaso?) |
| `calc.log10()`, `calc.deg()`, `calc.rad()` | Ausente | `test-calc-rest` |
| `json()`, `csv()`, `yaml()`, `toml()`, `xml()`, `cbor()` | Ausentes | — |
| `read()` (ficheiro texto) | Ausente | — |
| `plugin()` | Ausente | — |
| `eval()` (eval de string) | Ausente | — |
| `repr()` | Ausente | — |
| `panic()` | Ausente | — |

### 3.2 Elementos e argumentos ausentes

| Feature | Estado | Passo de identificação |
|---------|--------|----------------------|
| `image(fit:)` argumento | Ausente | P500 §4.1 |
| `raw(lang:, block:)` argumentos nomeados | Ausente | P500 §4.8 |
| `footnote(numbering:)` | Ausente | P500 §4.9 |
| `state.update()` com closure/função | Parcial — só literal | P506 §6 |
| `state.display()` real | Stub | P506 §6 |
| `context` aninhado em `Grid`/`Table` | Ausente | P506 §6 |
| `outline(indent:)` | API diverge de vanilla | P500 §4.11 |
| `list(marker: array)` por nível | Scope-out | P505 §4 |
| `numbering: "(a)"` patterns complexos | Fallback para Decimal | P505 §4 |
| `metadata()` / `query()` API | Diverge | `test-metadata-query` |

### 3.3 Elementos granulares ausentes

**Texto e markup:**

| Elemento | Nota |
|----------|------|
| `SmallcapsElem` | — |
| `SubElem`, `SuperElem` | Subscript / superscript inline |
| `HighlightElem` | — |
| `SymbolElem` | — |
| `RawLine` | Linha individual de bloco raw |
| `ParElem`, `ParLine` | Texto flui mas sem wrapper dedicado |

**Math:**

| Elemento | Nota |
|----------|------|
| `BinomElem` | — |
| `ClassElem` | — |
| `LimitsElem` | — |
| `MidElem` | — |
| `PrimesElem` | — |
| `ScriptsElem` | — |
| `StretchElem` | — |

**Math stdlib (funções de estilo):**

`bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright` — 12 funções de estilo matemático ausentes.

**Table/Grid:**

| Elemento | Nota |
|----------|------|
| `TableHLine`, `TableVLine` | Linhas horizontais/verticais declarativas |
| `GridHLine`, `GridVLine` | Idem para Grid |

**Curve:**

| Elemento | Nota |
|----------|------|
| `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose` | Path granular |

**Footnote:**

| Elemento | Nota |
|----------|------|
| `FootnoteContainer`, `FootnoteEntry`, `FootnoteMarker` | Maquinaria interna |

**PDF tagging / acessibilidade:** cluster de 6 elementos não identificados no cristalino.

---

## 4. O que o benchmark P507 valida genuinamente

Apesar das lacunas acima, o P507 confirma o seguinte:

| Observação | Fiabilidade |
|------------|-------------|
| `expand_context_ms` (runtime state P506) é < 0.1 % do tempo | Alta — feature implementada, custo medido |
| Layout engine cristalino escala bem com documento grande (macro 0.26×) | Média — mas macro não tem shaping real |
| Passagem dupla do eval consome 42 % no macro | Alta — é o gargalo real a monitorizar |
| Introspecção cresce sub-linearmente (7 % no macro) | Alta |
| `test-bibliography-csl` tem ratio 0.39× (cristalino 2.6× mais rápido) | Alta — usa o mesmo ficheiro CSL, overhead do vanilla é GC do compilador OCaml-style |

---

## 5. Matriz de prioridade para P508+

| Cluster | Impacto em paridade | Esforço estimado | Desbloqueador de quê |
|---------|--------------------|--------------------|----------------------|
| Shaping rustybuzz (Trilha 5) | **Crítico** — valida todos os benchmarks | XL | Benchmarks reais, PDF correto |
| `str` / `dict` métodos ausentes | Alto — falha em testes comuns | M | `test-str-methods`, `test-dict-methods` |
| `calc` funções ausentes | Médio | S | `test-calc-rest` |
| `state.update(callback)` | Médio — caso real de uso | M | documentos com state dinâmico |
| Math elementos granulares (7) | Médio | M | corpus math avançado |
| Table/Grid HLine/VLine | Médio | M | documentos com tabelas com linhas |
| Stdlib data (json/csv/yaml/toml) | Baixo a médio | M cada | documentos que carregam dados |
| HTML/SVG/raster export | Fora de escopo | — | — |

---

## 6. Definição operacional de "paridade"

Conforme ADR-0107, paridade é com a **linguagem** (semântica, sintaxe, morfologia), não com a mecânica (bytes de PDF, igualdade de glifos, estrutura de Rust).

O cristalino atinge paridade quando:

1. **Shaping ativo** — `rustybuzz` posiciona glifos; PDF contém fontes reais.
2. **Zero falhas nos 37 corpora actuais** com documentos válidos para Typst 0.15.0.
3. **Zero falhas nos 5 documentos excluídos** do P507 por razões de feature (não por bug de corpus).
4. **`crystalline-lint` zero violations** — já satisfeito pós-P507.

O ponto 1 (shaping) é o pré-requisito de todos os outros — sem ele os benchmarks são indicativos de arquitetura, não de desempenho real.

---

## 7. Referências

| Documento | Localização |
|-----------|-------------|
| Benchmark P507 | `00_nucleo/diagnosticos/benchmark-p507.md` |
| Auditoria vanilla 2026-05-19 | `00_nucleo/diagnosticos/auditoria-paridade-codigo-vanilla-2026-05-19.md` |
| Cobertura vanilla vs cristalino | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` |
| Série paridade funcional | `00_nucleo/diagnosticos/paridade-funcional-p490.md` … `p506.md` |
| ADR-0107 (definição de paridade) | `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md` |
| ADR-0115 (infra benchmark) | `00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md` |
