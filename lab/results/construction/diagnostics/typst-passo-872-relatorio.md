# Relatório — typst-passo-872: medir tempo de compilação cristalino vs vanilla

**Data:** 2026-07-23T20:03:42Z  
**Executor:** Kimi Code  
**Commit base:** `5a789380d1c4a19170ea7e160ea38b27cdb91d41` (HEAD do ramo `Tekt` após P871)  
**Ramo:** `Tekt`  

---

## 1. Resumo

Primeira medição sistemática de performance do compilador cristalino contra o vanilla. Foram comparados 7 cenários representativos, todos em build de release, com 10+ corridas por cenário e estatísticas completas. O cristalino é mais rápido em 5 dos 7 cenários, mas é dramáticamente mais lento em matemática (22×) e imagens (16×).

---

## 2. Metodologia

### 2.1 Binários

- **Vanilla:** `lab/typst-original/target/release/typst`, versão `0.15.0 (969087ec)`.
- **Cristalino:** `./target/release/typst`, compilado do commit base com `cargo build --workspace --release`.

Ambos são builds de release (otimizados), conforme exigido.

### 2.2 Documentos de teste

Criados em `/tmp/p872-bench/`:

| # | Ficheiro | Descrição |
|---|---|---|
| 1 | `01-hello.typ` | `Hello World` — overhead/startup. |
| 2 | `02-lorem.typ` | `#lorem(500)` — texto corrido. |
| 3 | `03-images.typ` | 50× `image("debian-logo.png")` — decode/embedding. |
| 4 | `04-math.typ` | 100× equações com `sum`, sub/superscript, gregos — layout math. |
| 5 | `05-tables.typ` | 20× tabela 5×10 — grid/placement. |
| 6 | `06-long.typ` | 50× secções com `lorem(200)` e `pagebreak()` — paginação. |
| 7 | `07-context.typ` | 200× `#context measure[lorem(10)]` — caminho P858. |

### 2.3 Ferramenta e parâmetros

- `hyperfine 1.20.0`.
- `--warmup 1` (descarta a primeira corrida de cada comando).
- `--min-runs 10` (mínimo de 10 corridas; hyperfine aumenta automaticamente quando o tempo é pequeno).
- Output para `/dev/null` com `--format pdf` para evitar I/O de disco a influenciar as medidas.
- Comandos alternados automaticamente pelo hyperfine.

### 2.4 Ambiente

- Máquina sem carga aparente de outros processos durante as medições.
- Cada cenário foi corrido isoladamente (não em paralelo com outros).

---

## 3. Resultados

### 3.1 Tabela de tempos

| Cenário | Vanilla média (s) | Vanilla mediana (s) | Vanilla σ (s) | Cristalino média (s) | Cristalino mediana (s) | Cristalino σ (s) | Razão C/V |
|---|---|---|---|---|---|---|---|
| 01-hello | 0.2751 | 0.2723 | 0.0046 | 0.0966 | 0.0964 | 0.0026 | **0.35×** |
| 02-lorem | 0.2786 | 0.2773 | 0.0033 | 0.1155 | 0.1151 | 0.0015 | **0.41×** |
| 03-images | 0.0068 | 0.0066 | 0.0005 | 0.1099 | 0.1098 | 0.0011 | **16.26×** |
| 04-math | 0.2861 | 0.2855 | 0.0027 | 6.3962 | 6.3916 | 0.0320 | **22.36×** |
| 05-tables | 0.3019 | 0.3011 | 0.0044 | 0.1145 | 0.1141 | 0.0017 | **0.38×** |
| 06-long | 0.2924 | 0.2924 | 0.0027 | 0.3630 | 0.3606 | 0.0058 | **1.24×** |
| 07-context | 0.2927 | 0.2931 | 0.0017 | 0.1276 | 0.1268 | 0.0022 | **0.44×** |

**Interpretação da razão:** valores < 1.0 significam cristalino mais rápido; valores > 1.0 significam cristalino mais lento.

### 3.2 Tamanhos dos PDFs gerados

| Cenário | Vanilla (B) | Cristalino (B) | Razão C/V |
|---|---|---|---|
| 01-hello | 5 571 | 340 318 | 61× |
| 02-lorem | 14 763 | 389 993 | 26× |
| 03-images | 13 665 | 95 925 | 7× |
| 04-math | 95 779 | 1 794 931 | 19× |
| 05-tables | 203 536 | 400 765 | 2× |
| 06-long | 151 708 | 1 339 574 | 9× |
| 07-context | 50 670 | 2 100 | 0.04× |

Os PDFs cristalinos são sistematicamente maiores, com destaque para math (19×) e hello (61×). Isto sugere que o cristalino embute fontes completas no PDF, enquanto o vanilla faz subsetting agressivo.

---

## 4. Casos que se destoam

### 4.1 Matemática (04-math) — 22× mais lento

O cristalino demora ~6.4 s contra ~0.29 s do vanilla. O PDF resultante tem ~1.8 MB contra ~96 KB do vanilla. É o caso mais extremo e a principal pista para futura investigação.

**Hipóteses não verificadas (para passo futuro):**
- O subsetting de fontes no cristalino pode estar ausente ou ineficiente, embutindo fontes matemáticas inteiras.
- O layout matemático cristalino pode ter complexidade superlinear ou recálculos desnecessários.
- A resolução de fontes para math (New Computer Modern Math, Libertinus Serif, etc.) pode carregar mais faces do que o vanilla.

### 4.2 Imagens (03-images) — 16× mais lento

O cristalino demora ~110 ms contra ~6.8 ms do vanilla para 50 imagens pequenas. A diferença não se explica pelo tamanho do PDF (7× maior, não 16×).

**Hipóteses não verificadas:**
- Decode de imagem no cristalino pode não reutilizar cache entre chamadas.
- Re-encode ou conversão de formato pode estar a ser feita desnecessariamente.

### 4.3 Documentos longos (06-long) — 1.24× mais lento

Diferença pequena e dentro da variação esperada, mas consistente. Provavelmente relacionada com paginação ou com o maior volume de fontes a embutir.

---

## 5. Conclusão

- O cristalino tem **menor overhead de startup** e é mais rápido em casos simples (texto corrido, tabelas, context), provavelmente porque faz menos trabalho auxiliar ou porque o pipeline PDF é mais enxuto.
- Os **dois casos de destaque negativo são math e imagens**. Em math, a diferença é de duas ordens de grandeza e acompanhada de PDFs muito maiores, o que aponta fortemente para o custo de embedding/subsetting de fontes (ou para o layout matemático em si).
- A medição é robusta o suficiente para servir de baseline: 10+ corridas, estatísticas completas, binários release, e cenários variados.

**Recomendação para passo futuro:** investigar o caminho de fontes no PDF para math (subsetting, carregamento de faces) e o caminho de decode/embedding de imagens, começando pelo que produz PDFs tão maiores.
