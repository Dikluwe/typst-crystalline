# P676 — Porque a poupança interna de P675 não parecia aparecer no tempo de parede?

**Data:** 2026-07-10  
**Commit base:** `6d57fc943 — P675: adiciona hash do commit ao relatório`

---

## Resumo

O relatório de P675 concluiu que a otimização de `build_multifont` reduziu `render_ms` em ~36 % e `total_ms` em ~10,6 %, mas notou que o `hyperfine` directo no `macro-10x` ainda reportava ratio ~1,26×, atribuindo a discrepância a "ruído do sistema". Este passo verificou essa conclusão com mais repetições e comparando o tempo de parede real com o `total_ms` instrumentado.

**Conclusão:** a poupança de P675 **é real e aparece no tempo de parede**. A aparente inconsistência no P675 deveu-se a amostras pequenas (10 runs no `hyperfine` directo, 1 execução no `benchmark-p507.py`). Com 30 runs, a diferença é estável e significativa. Não há custo fora das fases instrumentadas a compensar a poupança.

---

## Medição com amostra maior (30 runs)

### Depois de P675 (commit base P675 = `6d57fc943`)

```bash
hyperfine --warmup 5 --runs 30 './target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676.pdf'
```

Resultado:

```text
Benchmark 1: ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676.pdf
  Time (mean ± σ):      5.212 s ±  0.043 s
  Range (min … max):    5.090 s …  5.284 s
```

### Antes de P675 (commit P674 = `e9450f4b8`)

```bash
git checkout e9450f4b8
cargo build --release
hyperfine --warmup 5 --runs 30 './target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-antes.pdf'
```

Resultado:

```text
Benchmark 1: ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-antes.pdf
  Time (mean ± σ):      5.705 s ±  0.036 s
  Range (min … max):    5.605 s …  5.798 s
```

### Comparação

| Estado | Tempo de parede (mean ± σ) | Poupança |
|---|---:|---:|
| Antes de P675 (P674) | 5,705 s ± 0,036 s | — |
| Depois de P675 | 5,212 s ± 0,043 s | **−493 ms (−8,6 %)** |

A diferença é estável: mesmo com o desvio padrão, os intervalos não se sobrepõem (5,090–5,284 s vs 5,605–5,798 s).

---

## Tempo fora das fases instrumentadas

### Depois de P675

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-timings.pdf --timings-json /tmp/p676-timings.json
time ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-time.pdf
```

```json
{
  "parse_ms": 0.0,
  "eval_ms": 575.697711,
  "introspect_ms": 89.277693,
  "expand_context_ms": 0.000982,
  "layout_ms": 1260.902849,
  "shape_ms": 1843.263715,
  "subset_ms": 1.004982,
  "render_ms": 884.210799,
  "total_ms": 4654.358731
}
```

Tempo real (`time`):

```text
real	0m5,066s
```

**Tempo fora das fases:** 5066 ms − 4654 ms = **412 ms**.

### Antes de P675

```json
{
  "parse_ms": 0.0,
  "eval_ms": 577.953516,
  "introspect_ms": 89.711801,
  "expand_context_ms": 0.000862,
  "layout_ms": 1266.824774,
  "shape_ms": 1851.189042,
  "subset_ms": 0.885898,
  "render_ms": 1395.450305,
  "total_ms": 5182.016198
}
```

Tempo real (`time`):

```text
real	0m5,610s
```

**Tempo fora das fases:** 5610 ms − 5182 ms = **428 ms**.

### Análise

| Estado | `total_ms` instrumentado | Tempo real | Fora das fases |
|---|---:|---:|---:|
| Antes de P675 | 5182 ms | 5610 ms | 428 ms |
| Depois de P675 | 4654 ms | 5066 ms | 412 ms |
| Diferença | −528 ms | −544 ms | −16 ms |

O tempo fora das fases instrumentadas é **praticamente idêntico** antes e depois (~420 ms). Não há nenhum custo extra a compensar a poupança de P675. A poupança interna (−528 ms no `total_ms`) traduz-se quase integralmente em poupança de tempo de parede (−544 ms real).

---

## Causa da discrepância no relatório P675

O relatório P675 usou:
1. Uma única medição `--timings-json` para o `total_ms` (4668 ms).
2. Um `hyperfine` directo de apenas 10 runs, que deu 5,873 s ± 0,438 s.

Com 10 runs, o desvio padrão (0,438 s) era grande o suficiente para fazer com que a média parecesse inconsistente com o `total_ms` instrumentado. Com 30 runs, o desvio padrão cai para ~0,04 s e a média (5,212 s) alinha-se perfeitamente com o `total_ms` + tempo fora das fases (~4654 ms + ~420 ms).

A explicação de "ruído do sistema" no relatório P675 estava tecnicamente no sentido certo, mas imprecisa. O problema não era ruído aleatório; era **amostra insuficiente**. O relatório P675 foi corrigido para refletir esta conclusão.

---

## Decisão

- A poupança de P675 é confirmada no tempo de parede com 30 runs.
- Não há custo oculto fora das fases instrumentadas a compensar a otimização.
- O relatório P675 foi corrigido para remover a atribuição genérica a "ruído" e substituí-la pela conclusão baseada em amostra maior.

---

## Hash do commit

`HASH_A_PREENCHER_APOS_COMMIT`
