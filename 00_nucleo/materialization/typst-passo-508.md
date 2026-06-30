---

# P508 — Diagnóstico de Paridade Real: Reconhecimento das Brechas

> **Passo:** 508
> **Data:** 2026-06-30
> **Foco:** Reconhecer empiricamente que a paridade funcional não foi atingida, mapear as brechas reais, e priorizar o que falta. Não declarar conclusão — medir o que falta.
> **Tipo:** Diagnóstico honesto com priorização rigorosa.
> **Tamanho:** S (~30 min de análise + documentação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P507 (benchmark inválido), P506 (runtime state), P505 (list/enum indent), P504 (novas funcionalidades 0.15.0).

---

## 1. Reconhecimento: A Paridade Não Foi Atingida

O benchmark P507 reportou ratios de 0.03×–0.26× (cristalino "mais rápido" que vanilla). Esse resultado é **inválido** como medida de paridade porque:

1. **O cristalino não faz shaping real** — `FrameItem::Text` é sequencial sem posicionamento de glifo via rustybuzz.
2. **O cristalino não faz subsetting de fontes** — PDF gerado sem embedding correto.
3. **O cristalino não descobre fontes do sistema** — `fontdb` não é inicializado.
4. **O vanilla 0.15.0 falhou em 5 documentos** — funcionalidades que o cristalino aceita mas o vanilla rejeita (ou vice-versa), indicando divergência real.

**Corolário:** Os 37/37 MATCH do P490+P500 são de **sintaxe e semântica de alto nível**, não de **produção de documento real**. A paridade de linguagem (ADR-0107) está parcial, mas a paridade de **produção** está distante.

---

## 2. Brechas Estruturais (Subsistemas Inteiros Ausentes)

| Subsistema | Estado | Tamanho | Bloqueador de quê |
|------------|--------|---------|-------------------|
| **Shaping real (rustybuzz)** | Stub — texto sem posicionamento de glifo | **XL** | Benchmarks reais, PDF correto, qualquer documento com texto |
| **Subsetting de fontes no PDF** | Ausente | L | PDF com fontes reais |
| **Inicialização de fontdb** | Ausente | M | Descoberta de fontes do sistema |
| **HTML export** | 0 % | L | Fora de escopo declarado |
| **SVG export** | 0 % | M | Fora de escopo declarado |
| **Raster render** | 0 % | M | Fora de escopo declarado |
| **IDE** | 0 % | L | Fora de escopo declarado |

**Decisão:** HTML/SVG/raster/IDE são **fora de escopo** do cristalino (que visa PDF). Shaping, subsetting e fontdb são **pré-requisitos** para paridade real.

---

## 3. Brechas Funcionais (Features da Linguagem)

### 3.1 Stdlib — Funções Ausentes ou Incompletas

| Função / Método | Estado | Documento que Falha | Tamanho |
|-----------------|--------|---------------------|---------|
| `str` field access (`.len`, `.first`, etc.) | **Ausente** | `test-str-methods` | M |
| `str.split()`, `str.trim()`, `str.replace()` | **Ausente** | `test-str-methods` | M |
| `dict.insert()`, `dict.len()`, `dict.remove()` | **Ausente** | `test-dict-methods` | S |
| `calc.log10()`, `calc.deg()`, `calc.rad()` | **Ausente** | `test-calc-rest` | XS |
| `json()`, `csv()`, `yaml()`, `toml()`, `xml()`, `cbor()` | **Ausentes** | — | M cada |
| `read()` (ficheiro texto) | **Ausente** | — | S |
| `plugin()` | **Ausente** | — | L |
| `eval()` (eval de string) | **Ausente** | — | M |
| `repr()` | **Ausente** | — | S |
| `panic()` | **Ausente** | — | XS |
| `state.update(callback)` | **Parcial** — só literal | `test-state-counter` | M |
| `state.display()` | **Stub** | `test-state-counter` | S |
| `context` aninhado em `Grid`/`Table` | **Ausente** | `test-state-counter` | M |

### 3.2 Elementos e Argumentos Ausentes

| Feature | Estado | Passo de Identificação | Tamanho |
|---------|--------|----------------------|---------|
| `image(fit:)` | **Ausente** | P500 §4.1 | S |
| `raw(lang:, block:)` | **Ausente** | P500 §4.8 | S |
| `footnote(numbering:)` | **Ausente** | P500 §4.9 | XS |
| `outline(indent:)` | **API diverge** | P500 §4.11 | S |
| `list(marker: array)` por nível | **Scope-out** | P505 §4 | M |
| `numbering: "(a)"` patterns complexos | **Fallback para Decimal** | P505 §4 | M |
| `metadata()` / `query()` API | **Diverge** | `test-metadata-query` | M |

### 3.3 Elementos Granulares Ausentes

**Texto e markup:**
- `SmallcapsElem`, `SubElem`, `SuperElem`, `HighlightElem`, `SymbolElem`, `RawLine`, `ParElem`, `ParLine`

**Math (7 elementos):**
- `BinomElem`, `ClassElem`, `LimitsElem`, `MidElem`, `PrimesElem`, `ScriptsElem`, `StretchElem`

**Math stdlib (12 funções de estilo):**
- `bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright`

**Table/Grid:**
- `TableHLine`, `TableVLine`, `GridHLine`, `GridVLine`

**Curve:**
- `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose`

**Footnote:**
- `FootnoteContainer`, `FootnoteEntry`, `FootnoteMarker`

**PDF tagging / acessibilidade:**
- Cluster de 6 elementos não identificados

---

## 4. Matriz de Prioridade para P509+

### 4.1 Cluster 1: Shaping (Trilha 5) — Pré-requisito de Tudo

| Item | Impacto | Esforço | Desbloqueador |
|------|---------|---------|---------------|
| Shaping rustybuzz | **Crítico** | XL | Benchmarks reais, PDF correto, qualquer documento com texto |
| Subsetting de fontes | Alto | L | PDF com fontes reais |
| Inicialização de fontdb | Alto | M | Descoberta de fontes |

**Decisão:** Trilha 5 é uma **trilha separada**, não um passo. Deve ser iniciada como projeto paralelo, não como continuação da trilha de paridade.

### 4.2 Cluster 2: Stdlib Core (S/M-size, alto impacto)

| Item | Impacto | Esforço | Documento que desbloqueia |
|------|---------|---------|--------------------------|
| `str` field access + 13 métodos | Alto | M | `test-str-methods` |
| `dict` métodos (`insert`, `len`, `remove`) | Alto | S | `test-dict-methods` |
| `calc` funções (`log10`, `deg`, `rad`, `asinh`, etc.) | Médio | XS | `test-calc-rest` |
| `json()`, `csv()`, `yaml()`, `toml()`, `xml()`, `cbor()` | Médio | M cada | Novos testes |
| `read()` | Médio | S | Novos testes |
| `repr()`, `panic()` | Baixo | XS | Novos testes |
| `eval()` (eval de string) | Médio | M | Novos testes |

### 4.3 Cluster 3: Elementos e Args (S/M-size, médio impacto)

| Item | Impacto | Esforço | Documento que desbloqueia |
|------|---------|---------|--------------------------|
| `image(fit:)` | Médio | S | `test-image-fit` |
| `raw(lang:, block:)` | Médio | S | `test-raw-advanced` |
| `footnote(numbering:)` | Baixo | XS | `test-footnote-advanced` |
| `outline(indent:)` API alinhada | Baixo | S | `test-outline-advanced` |
| `state.update(callback)` | Médio | M | `test-state-counter` |
| `state.display()` real | Médio | S | `test-state-counter` |
| `context` aninhado | Médio | M | `test-state-counter` |

### 4.4 Cluster 4: Math Avançado (M-size, médio impacto)

| Item | Impacto | Esforço |
|------|---------|---------|
| 12 funções de estilo math (`bb`, `bold`, `cal`, etc.) | Médio | M |
| 7 elementos math granulares (`BinomElem`, `ClassElem`, etc.) | Médio | M |

### 4.5 Cluster 5: Table/Grid/Curve (M-size, baixo impacto)

| Item | Impacto | Esforço |
|------|---------|---------|
| `TableHLine`, `TableVLine`, `GridHLine`, `GridVLine` | Médio | M |
| `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose` | Baixo | M |
| `FootnoteContainer`, `FootnoteEntry`, `FootnoteMarker` | Baixo | M |

---

## 5. O que o P507 Validou Genuinamente

Apesar das lacunas, o P507 confirma:

| Observação | Fiabilidade |
|------------|-------------|
| `expand_context_ms` (P506) é < 0.1 % do tempo | **Alta** — runtime state implementado, custo medido |
| Passagem dupla do eval consome 42 % no macro | **Alta** — é o gargalo real a monitorizar |
| Introspecção cresce sub-linearmente (7 % no macro) | **Alta** |
| Layout engine cristalino escala bem | **Média** — mas sem shaping real |
| `crystalline-lint` zero violations | **Alta** — já satisfeito |

---

## 6. Definição Operacional de Paridade (Revisada)

Conforme ADR-0107, paridade é com a **linguagem** (semântica, sintaxe, morfologia), não com a mecânica (bytes de PDF, igualdade de glifos).

O cristalino atinge paridade quando:

1. **Zero falhas nos 37 corpora actuais** com documentos válidos para Typst 0.15.0. ✅ (parcial — 5 documentos excluídos)
2. **Zero falhas nos 5 documentos excluídos** do P507. ❌ (funcionalidades ausentes)
3. **Shaping ativo** — `rustybuzz` posiciona glifos; PDF contém fontes reais. ❌ (Trilha 5)
4. **Stdlib core completa** — `str`, `dict`, `calc`, `json`, `csv`, `yaml`, `read`, `repr`, `panic`, etc. ❌
5. **Elementos granulares** — math estilos, table/grid lines, curve elements. ❌

**Conclusão:** O cristalino está em **~60 % da paridade de linguagem** e **~10 % da paridade de produção** (shaping + PDF).

---

## 7. Próximo Passo (P509)

**Recomendação:** P509 = **Materialização de Stdlib Core — Lote 1** (`str` field access + `dict` métodos + `calc` funções + `repr` + `panic`).

**Por que este lote:**
- São as brechas mais frequentes em documentos reais.
- São S/M-size, factíveis em 1-2 dias.
- Desbloqueiam os 5 documentos excluídos do P507.
- Preparam o terreno para Trilha 5 (shaping).

**Alternativa:** Iniciar **Trilha 5 (Shaping)** em paralelo — é XL-size e não bloqueia os passos S/M, mas é o único caminho para paridade de produção.

---

## A. Apêndice — Inventário Completo de Brechas

```
Cluster 1: Shaping (XL) — Trilha 5 separada
Cluster 2: Stdlib Core (S/M) — P509+
  ├── str: field access + 13 métodos (M)
  ├── dict: insert, len, remove (S)
  ├── calc: log10, deg, rad, asinh, acosh, atanh, erf (XS)
  ├── json, csv, yaml, toml, xml, cbor (M cada)
  ├── read() (S)
  ├── repr() (XS)
  ├── panic() (XS)
  └── eval() (M)
Cluster 3: Elementos e Args (S/M) — P510+
  ├── image(fit:) (S)
  ├── raw(lang:, block:) (S)
  ├── footnote(numbering:) (XS)
  ├── outline(indent:) API (S)
  ├── state.update(callback) (M)
  ├── state.display() real (S)
  └── context aninhado (M)
Cluster 4: Math Avançado (M) — P511+
  ├── 12 funções de estilo math
  └── 7 elementos math granulares
Cluster 5: Table/Grid/Curve (M) — P512+
  ├── Table/Grid HLine/VLine
  ├── Curve elements
  └── Footnote internal machinery
```

---

## B. Apêndice — Documentos Excluídos do P507 (a desbloquear)

| Documento | Erro no vanilla | Funcionalidade ausente no cristalino |
|-----------|-----------------|-------------------------------------|
| `test-calc-rest.typ` | `module calc does not contain log10` | `calc.log10` (P504 planejado, não executado) |
| `test-metadata-query.typ` | API divergente | `metadata()` / `query()` API |
| `test-str-methods.typ` | método de string não suportado | `str` field access (P501 planejado, não executado) |
| `medium-combined.typ` | herda falha | Múltiplas |
| `0.15.0-spec.typ` | herda falha | Múltiplas |

**Nota:** O vanilla 0.15.0 compilado localmente (`lab/typst-original/`) pode não ter todas as funcionalidades do release oficial. Verificar se o binário está completo.
