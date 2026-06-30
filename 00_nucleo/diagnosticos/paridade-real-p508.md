# Diagnóstico de Paridade Real — Passo 508

**Data:** 2026-06-30  
**Passo:** 508  
**Foco:** Verificar empiricamente o estado real de paridade do cristalino com Typst 0.15.0, corrigir diagnósticos desatualizados do P507/P508 e priorizar brechas restantes.  
**Estado:** Concluído.

---

## 1. Correção ao Diagnóstico P507/P508

### 1.1 O benchmark P507 não é inválido — é incompleto

O P507 mediu corretamente o tempo de execução observado no ambiente de teste. A razão cristalino/vanilla < 1 reflete principalmente:

- **Ausência de descoberta de fontes do sistema** — o cristalino não inicializa `fontdb`; só carrega fontes via `--font-path`. Isso evita o overhead de indexação de fontes do vanilla.
- **Ausência de subsetting de fontes** — o cristalino embebe a fonte completa (linha 243 de `03_infra/src/export/builder.rs`), mas como geralmente não há fontes resolvidas, o impacto é nulo.
- **Shaping real existe** — `03_infra/src/shaper.rs` usa `rustybuzz` com runs bidirecionais (`unicode-bidi`) e o export `stream.rs` emite `FrameItem::TextShaped` via operador `TJ`.

**Conclusão:** O cristalino *faz* shaping real, mas *não* descobre fontes do sistema, pelo que o shaping raramente é ativado em documentos sem `--font-path` explícito.

### 1.2 Estado real do shaping / fontes / PDF

| Subsistema | Estado Real | Observação |
|------------|-------------|------------|
| Shaping (rustybuzz) | **Implementado** | `shaper.rs` + `TextShaped` + export `TJ`. |
| Subsetting de fontes | **Ausente** | Fonte completa embutida (`builder.rs:243`). |
| Descoberta de fontes do sistema | **Ausente** | `SystemWorld` não usa `fontdb`; só `--font-path`. |
| Font embedding | **Parcial** | Funciona quando `--font-path` aponta para TTF/OTF. |

---

## 2. Paridade Funcional — Testes Empíricos

### 2.1 Corpus P490 + P500 no Cristalino

Comando: `target/release/typst <doc> <out.pdf>` para todos os 37 documentos.

| Resultado | Quantidade |
|-----------|------------|
| OK | 36 / 37 |
| Falha | 1 / 37 (`test-metadata-query.typ`) |

**Brecha confirmada:** `query(<tag>)` sem argumento selector falha. O cristalino exige string/location:

```
error: query() requer string ou location, recebeu none.
Tipos suportados: "kind", "<label>", Value::Location.
```

### 2.2 Funcionalidades Testadas Individualmente

| Funcionalidade | Estado | Nota |
|----------------|--------|------|
| `str.len` | ❌ Ausente | `field access não suportado em str` |
| `str.first` | ❌ Ausente | |
| `str.last` | ❌ Ausente | |
| `str.at(i)` | ❌ Ausente | |
| `str.slice(a,b)` | ❌ Ausente | |
| `str.rev` | ❌ Ausente | |
| `str.clusters()` | ❌ Ausente | |
| `str.split("l")` | ✅ Implementado | |
| `str.contains` / `starts-with` / `ends-with` | ✅ Implementado | |
| `str.trim` / `replace` | ✅ Implementado | |
| `dict.len` | ❌ Ausente | `campo 'len' não existe` |
| `dict.insert(k,v)` | ❌ Ausente | |
| `dict.remove(k)` | ❌ Ausente | |
| `calc.log10` | ✅ Implementado | |
| `calc.deg` / `calc.rad` | ✅ Implementado | |
| `calc.asinh` / `calc.erf` | ✅ Implementado | |
| `repr()` | ✅ Implementado | |
| `panic()` | ✅ Implementado | |
| `json("...")` / `csv("...")` | ⚠️ Path-only | Interpretam string como path, não conteúdo inline. |
| `image("x.png", fit: "contain")` | ❌ Ausente | `unknown variable: fit` |
| `raw(lang: "rust", block: true, "...")` | ❌ Ausente | `unknown variable: lang` |
| `footnote[note][numbering: "1."]` | ⚠️ Aceite | Sintaxe aceite; efeito visual não verificado. |
| `outline(indent: 2em)` | ✅ Implementado | |
| `state("x",0).update(x => x+1)` | ✅ Implementado | |
| `context` aninhado em `table` | ✅ Implementado | |
| `counter(heading).get()` | ✅ Implementado | |

### 2.3 Documentos que Falharam no Vanilla 0.15.0 (P507)

| Documento | Erro no vanilla | Estado no cristalino |
|-----------|-----------------|----------------------|
| `test-calc-rest.typ` | `calc.log10` ausente no vanilla | ✅ Compila no cristalino |
| `test-metadata-query.typ` | API divergente | ❌ Também falha no cristalino (query selector) |
| `test-str-methods.typ` | método de string não suportado | ✅ Compila no cristalino |
| `medium-combined.typ` | herda falha | ✅ Compila no cristalino |
| `0.15.0-spec.typ` | herda falha | ✅ Compila no cristalino |

**Nota:** O binário vanilla compilado localmente (`lab/typst-original/target/release/typst`) reporta-se como `0.15.0` mas falta-lhe `calc.log10`. Isto pode indicar que a cópia em `lab/typst-original/` não é exatamente o release oficial 0.15.0 ou que algumas funcionalidades foram scope-out naquele snapshot.

---

## 3. Matriz de Brechas Reais (Priorizada)

### 3.1 Cluster 1: Infra de Fontes (Trilha 5)

| Item | Impacto | Esforço | Estado |
|------|---------|---------|--------|
| Descoberta de fontes do sistema (`fontdb`) | **Crítico** | M | ❌ |
| Subsetting de fontes no PDF | Alto | L | ❌ |
| Shaping real já ativo | — | — | ✅ |

### 3.2 Cluster 2: Stdlib Core

| Item | Impacto | Esforço | Estado |
|------|---------|---------|--------|
| `str` field access (`.len`, `.first`, `.last`, `.at`, `.slice`, `.rev`, `.clusters`) | Alto | M | ❌ |
| `dict` métodos (`.len`, `.insert`, `.remove`) | Alto | S | ❌ |
| `image(fit:)` | Médio | S | ❌ |
| `raw(lang:, block:)` | Médio | S | ❌ |
| `query()` com selector/location | Médio | M | ⚠️ |
| `json()` / `csv()` / `yaml()` / `toml()` / `xml()` / `cbor()` | Médio | M cada | ✅ Decode existe; path-only behavior a alinhar. |
| `read()` | Médio | S | ✅ Implementado em `loading.rs`. |
| `repr()`, `panic()` | Baixo | XS | ✅ |

### 3.3 Cluster 3: Elementos e Features Avançadas

Conforme inventário do P508, permanecem não verificados neste passo:
- Math: 12 funções de estilo, 7 elementos granulares.
- Table/Grid: `HLine`/`VLine`.
- Curve: `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose`.
- Footnote interno: `FootnoteContainer`, `FootnoteEntry`, `FootnoteMarker`.
- PDF tagging / acessibilidade.

---

## 4. Avaliação das Hipóteses de Paridade

| Declaração P508 | Veredito | Justificação |
|-----------------|----------|--------------|
| "Cristalino não faz shaping real" | **Falso** | `shaper.rs` implementa shaping rustybuzz; export usa `TextShaped`. |
| "Cristalino não faz subsetting" | **Verdadeiro** | `builder.rs:243` embebe fonte completa. |
| "Cristalino não descobre fontes do sistema" | **Verdadeiro** | `SystemWorld` só usa `--font-path`. |
| "`str` field access ausente" | **Parcialmente verdadeiro** | `.len`/`.first`/etc ausentes; `.split`/`.contains`/`.trim` existem. |
| "`dict.insert/remove/len` ausentes" | **Verdadeiro** | Confirmado empiricamente. |
| "`calc.log10/deg/rad` ausentes" | **Falso** | Implementados e testados. |
| "`state.update(callback)` parcial" | **Falso** | Funciona com callback. |
| "`context` aninhado ausente" | **Falso** | Funciona em `table`. |
| "`repr()`/`panic()` ausentes" | **Falso** | Implementados. |

---

## 5. Definição Operacional de Paridade (Revisada)

De acordo com ADR-0107, paridade é com a **linguagem** (semântica, sintaxe, morfologia), não com a mecânica (bytes de PDF, glifos idênticos).

Critérios atualizados:

1. **Zero falhas nos 37 corpora actuais** com documentos válidos para Typst 0.15.0. ⚠️ 1 falha (`test-metadata-query`).
2. **Stdlib core completa** — `str` field access, `dict` métodos. ❌
3. **Args de elementos** — `image(fit:)`, `raw(lang:, block:)`. ❌
4. **Shaping ativo** — necessita de `fontdb` system. ❌
5. **Subsetting de fontes** — não bloqueia paridade de linguagem, mas bloqueia produção real. ❌

**Estimativa:** paridade de linguagem ~80 %; paridade de produção ~40 %.

---

## 6. Recomendação para P509

Duas trilhas são possíveis:

| Trilha | Foco | Esforço | Desbloqueia |
|--------|------|---------|-------------|
| **A — Stdlib Core (recomendada)** | `str` field access + `dict` métodos + `image(fit:)` + `raw(lang:, block:)` + `query()` selector | S/M | 1 falha restante no corpus; documentos reais comuns |
| **B — Fontdb System** | Integrar `fontdb` no `SystemWorld` e ativar shaping por defeito | M | Paridade de produção; benchmarks realistas |

**Recomendação:** P509 = **Trilha A — Stdlib Core**. São brechas pequenas, de alto impacto na linguagem, e desbloqueiam o corpus P490/P500 a 100 %. A Trilha B (fontdb) deve ser iniciada em paralelo como projeto separado (Trilha 5), pois é pré-requisito para paridade de produção.

---

## 7. Anexos

### 7.1 Comandos de Reprodução

```bash
# Corpus no cristalino
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  out=$(mktemp /tmp/p508-XXXX.pdf)
  target/release/typst "$f" "$out" >/dev/null 2>&1 \
    && echo "OK: $(basename $f)" \
    || echo "FAIL: $(basename $f)"
  rm -f "$out"
done

# Testes individuais
target/release/typst /tmp/p508-one.typ /tmp/p508-one.pdf
```

### 7.2 Referências de Código

- Shaping: `03_infra/src/shaper.rs`
- Export TextShaped: `03_infra/src/export/stream.rs:328`, `:746`
- Fonte completa sem subsetting: `03_infra/src/export/builder.rs:243`
- `SystemWorld` sem fontdb: `03_infra/src/world.rs:102-140`
- `str.split` etc.: `01_core/src/rules/stdlib/structural.rs`
- `loading.rs`: `01_core/src/rules/stdlib/loading.rs`
- `panic.rs`: `01_core/src/rules/stdlib/panic.rs`

### 7.3 Hash do Commit de Referência

Diagnóstico baseado no commit `15e291f65` (P507 finalizado).
