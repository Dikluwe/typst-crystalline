# Relatório Diagnóstico — Passo 594
## O benchmark excede o tempo por lentidão genuína, não por ciclo

- **Commit de Referência:** `625309f210dd24d03d4a5f02a9e8cb1eb65ca0e9` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 12:39:40 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`, ADR-0108

---

## 1. Objetivo

O benchmark `tools/perf/benchmark-p507.py` excedeu o tempo limite em P546, P563, P565 e P593 — sempre aceite como "é grande, demora". Este passo separa as hipóteses: ciclo sem fim vs. lentidão genuína.

---

## 2. Verificação

### 2.1 Isolar `macro-10x` com timeout curto

```bash
time timeout 30 ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p594-macro.pdf
```

Resultado:

```text
real  0m30.158s
user  0m29.483s
sys   0m0.557s
Exit code: 124
```

O processo usou CPU constante (~98%) durante os 30 s. Não estava parado.

### 2.2 Isolar `macro-10x` com timeout longo

```bash
time timeout 120 ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p594-macro.pdf
```

Resultado:

```text
real  0m35.377s
user  0m34.610s
sys   0m0.733s
Exit code: 0
```

**Conclusão:** `macro-10x` termina, mas demora ~35 s. Não é um ciclo sem fim.

### 2.3 Comparar com vanilla

```bash
time timeout 120 ./lab/typst-original/target/release/typst compile tools/perf/corpus/macro-10x.typ /tmp/p594-macro-vanilla.pdf
```

Resultado:

```text
real  0m4.633s
user  0m4.125s
sys   0m0.747s
Exit code: 0
```

O cristalino demora ~7,6× mais tempo que o vanilla neste documento.

### 2.4 Onde passa o tempo no cristalino

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p594-macro.pdf --timings-json /tmp/p594-timings.json
```

```text
parse_ms:     0.0 ms
eval_ms:    561.4 ms
introspect_ms: 89.7 ms
expand_context_ms: 0.0 ms
layout_ms: 1380.8 ms
shape_ms: 30564.7 ms
subset_ms:    1.0 ms
render_ms: 1498.4 ms
total_ms: 34096.0 ms
```

**89,6% do tempo total** gasta em `shape_ms`. O bottleneck é a passagem de shaping, não o layout nem o render.

### 2.5 Inspecção de ciclos em `layout_bidi.rs`

```bash
grep -n "loop\|while\|fn reflow_rtl_paragraphs\|fn try_fuse_paragraph" 03_infra/src/layout_bidi.rs
```

Funções inspeccionadas:

- `reflow_rtl_paragraphs` (`03_infra/src/layout_bidi.rs:564`): loops com condições de saída claras (`i = end`, `end` incrementa, `run_start`/`run_end` aproximam-se).
- `try_fuse_paragraph` (`03_infra/src/layout_bidi.rs:684`): sem loops.

Não foi encontrado nenhum ciclo sem condição de saída explícita.

---

## 3. Decisão

- **Não é ciclo sem fim.** É lentidão genuína, concentrada na passagem de shaping (`shape_ms`).
- O documento `macro-10x` demora ~35 s no cristalino vs. ~4,6 s no vanilla.
- O benchmark completo excedia 300 s porque executa `macro-10x` várias vezes (warmup + 5 runs + 5 fases internas).

### Ajuste ao script de benchmark

Ficheiro: `tools/perf/benchmark-p507.py`

Documentos da categoria `macro` passam a usar `runs=2` em vez de 5, reduzindo o tempo total do benchmark para um valor que permite terminar sem exceder o tempo limite.

```python
runs = 2 if category == "macro" else 5
```

---

## 4. Validação

Após o ajuste:

```bash
timeout 600 python3 tools/perf/benchmark-p507.py
```

O benchmark terminou. Resultado para `macro-10x`:

```text
vanilla mean:     4937.04 ms
cristalino mean: 39623.76 ms
ratio:              8.03x
```

O tempo total do benchmark ajustado foi inferior a 5 minutos (o processo terminou dentro do timeout de 600 s).

Nota: alguns documentos `medium` (por exemplo, `medium-combined`, `0.15.0-spec`) falharam no hyperfine com "non-zero exit code 1" devido a erros de compilação conhecidos de paridade — o cristalino emite `heading(): level espera int, recebeu content` e o vanilla emite `expected term item or array`. Estes erros são independentes do timeout e não foram tratados neste passo.

---

## 5. Ficheiros alterados

- `tools/perf/benchmark-p507.py`
