# Relatório — P878: O filtro de fallback de P875 não foi aplicado em `FallbackFontMetrics::covering`

**Data:** 2026-07-23T19:29:18-03:00  
**Commit de trabalho:** `3f15cc50ec1dc40e852b41bc92e9ee2895ecaec6`  
**Estado:** fechado (diagnóstico, sem correções)  
**L0s afetados:** nenhum

---

## 1. Resumo

P877 concluiu, via `--timings-json`, que a maior parte do tempo de `04-math` estava em `layout_ms`. Isso parecia contradizer P873, que tinha localizado ~79% do tempo em I/O de `.ttc` CJK dentro do fallback. A medição deste passo reconcilia as duas coisas: **o I/O de CJK ainda domina o tempo, mas a instrumentação `--timings-json` agrupa esse I/O sob `layout_ms` porque acontece dentro de `FallbackFontMetrics::covering`, chamado pelo layout.**

A causa imediata: P875 aplicou o filtro de coverage apenas no shaper (`shaper.rs::covering_all`); `FallbackFontMetrics::covering` (`font_metrics.rs:734`) continua a iterar `for slot_idx in 0..book.len()` e carregar **todas** as fontes do sistema para cada caractere de fallback.

---

## 2. Repetição da medição de P873 no estado atual

### 2.1 `/usr/bin/time -v`

| Métrica | P873 (base) | P878 (atual) |
|---|---|---|
| User time | 1.36 s | 1.35 s |
| **System time** | **5.04 s** | **5.37 s** |
| Elapsed | 6.40 s | 6.73 s |
| Maximum RSS | 9.41 GB | 9.41 GB |

Os números são essencialmente os mesmos de P873. O filtro de P875 **não reduziu** o system time nem o RSS.

### 2.2 `strace -f -c`

| syscall | P873 | P878 (atual) |
|---|---|---|
| `read` (chamadas / tempo) | 2343 / 2.19 s | 2343 / 2.25 s |
| `% tempo em read` | 92.88% | 93.55% |

A proporção de tempo em `read()` mantém-se. O I/O ainda domina.

### 2.3 Contagem de aberturas de `.ttc` CJK

| Ficheiro | baseline (01-hello) | 04-math atual |
|---|---|---|
| `NotoSansCJK-Regular.ttc` | 11 | **21** |
| `NotoSansCJK-Bold.ttc` | 11 | **21** |
| `NotoSerifCJK-Regular.ttc` | 6 | **11** |
| `NotoSerifCJK-Bold.ttc` | 6 | **11** |

A diferença é exactamente o número de faces de cada coleção (10, 10, 5, 5), tal como em P873. O filtro de P875 não evitou que o layout carregasse cada face CJK uma vez.

---

## 3. Localização exacta do problema

### 3.1 Onde o filtro de P875 foi aplicado

`03_infra/src/shaper.rs:667`, `CandidateSet::covering_all`:

```rust
for slot_idx in self.world.book().candidates_for_char(c) {
    ...
}
```

### 3.2 Onde o filtro NÃO foi aplicado

`03_infra/src/font_metrics.rs:734`, `FallbackFontMetrics::covering`:

```rust
for slot_idx in 0..book.len() {
    if primary.iter().any(|cand| cand.slot_idx == slot_idx) {
        continue;
    }
    let Some(cached) = self.cached_face(slot_idx) else { continue };
    if cached.face().glyph_index(c).is_some() {
        ids.push(slot_idx);
    }
}
```

Este loop percorre **todo o `FontBook`** (~1086 fontes + 30 faces CJK) para cada caractere que as primárias não cobrem. `cached_face(slot_idx)` chama `self.world.font(slot_idx)`, que invoca `FontSlot::get()` — lendo o ficheiro inteiro do disco se ainda não estiver em cache.

### 3.3 Outros loops semelhantes em `font_metrics.rs`

Pelo menos três métodos adicionais iteram `0..book_len()` carregando faces via `cached_face`:

- `line_metrics` (~linha 909)
- `cap_height` (~linha 957)
- `edge_metrics` (~linha 994)

Estes são usados para métricas de linha/cap/edge quando a fonte primária não fornece os valores. Também podem contribuir para o custo fixo, mas o volume de chamadas em `04-math` é dominado por `covering`.

---

## 4. Confirmação por isolamento temporário

### 4.1 Alteração temporária

Em `FallbackFontMetrics::covering`, substituí:

```rust
for slot_idx in 0..book.len() {
```

por:

```rust
for slot_idx in book.candidates_for_char(c) {
```

### 4.2 Resultado

| Métrica | Sem filtro (atual) | Com filtro temporário |
|---|---|---|
| Razão C/V | 23.64× | **19.22×** |
| Tempo cristalino | ~6.65 s | ~5.50 s |
| Maximum RSS | 9.41 GB | **7.08 GB** |

Apenas esta alteração isolada reduziu o tempo em **~1.1 s** (~17%) e o RSS em **~2.3 GB** (~25%).

---

## 5. Por que o filtro de P875 não foi suficiente mesmo no shaper

A análise dos blocos marcados por `NotoSansCJK-Regular.ttc` mostra que as fontes CJK marcam os blocos `0x00`, `0x01`, `0x02`, `0x03`, `0x04`. Isso significa:

- Caracteres ASCII/latinos básicos (bloco `0x00`) ainda fazem com que `candidates_for_char` devolva as fontes CJK.
- O documento `04-math.typ` contém muitos caracteres ASCII (`k`, `n`, `=`, `+`, dígitos) dentro das equações.
- A granularidade de 256 codepoints por bit gera **falsos positivos** para blocos amplos onde a fonte CJK tem apenas alguns codepoints.

No entanto, o ganho medido com o filtro temporário (19× vs 23×) mostra que mesmo com falsos positivos por bloco, o filtro já ajuda significativamente.

---

## 6. Recomendação para o próximo passo

**P879 — Aplicar o filtro de coverage em `FallbackFontMetrics::covering` (e possivelmente nos outros loops `0..book_len()` de `font_metrics.rs`).**

### 6.1 Escopo mínimo

Substituir o loop `0..book.len()` em `covering` por `book.candidates_for_char(c)`. Esta é a alteração que o isolamento temporário provou dar ganho real (~1.1 s, ~2.3 GB RSS).

### 6.2 Escopo alargado (opcional)

Avaliar se os loops em `line_metrics`, `cap_height` e `edge_metrics` também devem usar `candidates_for_char` ou outro tipo de cache. Estes são chamados menos vezes, mas carregam o book inteiro quando nenhuma primária fornece a métrica.

### 6.3 O que NÃO fazer

- Não investigar o "algoritmo de layout matemático" como causa principal — a medição refuta isso.
- Não refinar a granularidade do bitmap como primeira medida — o ganho com a granularidade actual já é significativo; refinamento pode vir depois.

### 6.4 Critério de fecho

Reproduzir o benchmark P872 e confirmar que a razão de `04-math` cai de ~23× para ~19× (ou melhor). Só depois se deve considerar P878c (layout math propriamente dito) se o tempo ainda for inaceitável.

---

## 7. Procedimento de validação

```text
/usr/bin/time -v ./target/release/typst /tmp/p872-bench/04-math.typ /tmp/out.pdf
hyperfine --warmup 1 --min-runs 10 \
  "lab/typst-original/target/release/typst compile 04-math.typ /dev/null --format pdf" \
  "target/release/typst 04-math.typ /dev/null"
```

Métricas a reportar: `system time`, `Maximum RSS`, razão C/V.
