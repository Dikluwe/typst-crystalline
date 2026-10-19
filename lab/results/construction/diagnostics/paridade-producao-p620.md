# P620 — Bissecção da regressão de `macro-10x` entre P518 e P543

**Data da medição:** 2026-07-08  
**Commit base:** `1149271908cf10be458b0f2722316668f1e292db`  
**Passo:** P620  
**ADR-0108 em vigor:** medição precede a decisão.

## Resumo executivo

A bissecção entre P518 (`macro-10x` a 0,30× — cristalino mais rápido) e P543 (`macro-10x` a 11,52× — cristalino mais lento) identificou o primeiro commit onde `shape_ms` cresceu massivamente:

- **Commit:** `46c75cc66b2349671346ea0805c6e88bce894f72`
- **Passo:** P538e — "fallback padrão quando fonte default não existe"

A "regressão" não é um bug: P538e fez o shaper assumir fontes de fallback reais (DejaVu Sans, Noto Sans, Liberation Sans, FreeSans, Arial) quando a fonte declarada — tipicamente a default "Helvetica" — não existe no FontBook. Antes de P538e, `try_shape` devolvia `None` e o texto não era shapeado por esse caminho. A partir de P538e, o shaping passou a ser executado realmente, com o custo que isso implica.

A medição de P518 não é comparável às posteriores: o cristalino parecia "mais rápido" porque fazia menos trabalho (desistia do shaping quando a fonte default faltava), não porque o algoritmo fosse mais eficiente.

## Baselines de fronteira

| Momento | Commit | `shape_ms` | `total_ms` | Rácio `macro-10x` | Nota |
|---|---|---:|---:|---:|:---|
| P518 | `7bd72a2ce` | 142,33 | 1 012,03 | 0,30× | "Good" — shaping mínimo. |
| P538d | `b06f9a365` | 142,43 | — | — | Último commit "good" antes da mudança. |
| **P538e** | **`46c75cc66`** | **44 777,66** | — | — | **Primeiro commit "bad".** |
| P543 | `14a6d0628` | 55 812,18 | 58 044,01 | 11,52× | "Bad" confirmado. |
| P619 | `114927190` | 32 577,36 | 35 933,21 | 6,78× | Estado actual, pós-optimizações. |

Valores em milissegundos. A medição de fronteira usou `--timings-json` numa única run.

## Metodologia da bissecção

```bash
git bisect start
git bisect bad 14a6d0628   # P543
git bisect good 7bd72a2ce  # P518
git bisect run /tmp/p620-bisect.sh
```

O script `/tmp/p620-bisect.sh`:

1. `cargo build --release --bin typst`
2. Corria `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p620-bisect.pdf --timings-json /tmp/p620-timings.json`
3. Classificava o commit como **good** se `shape_ms < 1000 ms`, **bad** se `shape_ms ≥ 1000 ms`.

Resultado do `git bisect`:

```text
46c75cc66b2349671346ea0805c6e88bce894f72 is the first bad commit
P538e: fallback padrão quando fonte default não existe
```

## O que mudou em P538e

Diff relevante em `03_infra/src/shaper.rs`:

- Adicionou `DEFAULT_FALLBACK_FONTS: &[&str]` com DejaVu Sans, Noto Sans, Liberation Sans, FreeSans, Arial.
- Alterou `try_shape` para, quando `resolve_candidates` devolve vazio, iterar por essas fontes padrão e usar a primeira que exista no FontBook.
- Antes, `try_shape` devolvia `None` quando a fonte declarada (incluindo a default "Helvetica") não existia.

Isto significa que, em sistemas onde "Helvetica" não está instalada, o shaper passou a fazer shaping real de todo o texto em vez de desistir. O documento `macro-10x` não declara fonte explicitamente, pelo que usa a fonte default — e, como "Helvetica" não está no FontBook, P538e activa o fallback para DejaVu Sans/Noto Sans, com o custo de shaping associado.

## Verificação da hipótese de metodologia

A hipótese levantada em P546 — de que P518 media a frio e isso inflacionava artificialmente a vantagem do cristalino — não é a explicação principal. A explicação principal é comportamental:

- **P518 fazia menos trabalho:** o shaper desistia quando a fonte default faltava, deixando o texto por shapear por outro caminho (ou não shapeado de todo).
- **P538e em diante faz o trabalho correcto:** assume fallback padrão e shapeia o texto, tal como o vanilla faz.

A diferença de tamanho dos PDFs confirma o aumento de trabalho:

| Versão | Páginas | Tamanho do PDF | Linhas `pdftotext` |
|---|---:|---:|---:|
| P518 | 1042 | 19,6 MB | 188 962 |
| P543 | 1042 | 34,7 MB | 118 540 |

O P518 produziu um PDF menor, com texto extraído fragmentado/corrompido (`Sec??o`, `Par?grafo`), consistente com um caminho de renderização que não usava o shaper completo. O P543 produziu um PDF maior e com texto shapeado correctamente.

## Decisão

A "regressão" de P518 para P543 é, na realidade, **uma correção de comportamento com custo associado**:

1. O shaper deixou de desistir quando a fonte default não existe.
2. Passou a usar fontes de fallback reais, alinhando o cristalino com o comportamento do vanilla.
3. O custo de `shape_ms` (~90% do tempo no `macro-10x`) é o preço de fazer shaping real.

**Decisão:** aceitar com razão. Não é uma regressão a corrigir; é o preço de ter paridade semântica/morfológica com o vanilla no tratamento de fallback de fontes. O trabalho subsequente (P548 cache de `Face`, etc.) reduziu o custo de 11,52× (P543) para 6,78× (P619), mantendo o comportamento correcto.

A medição histórica de P518 (rácio 0,30×) deve ser reclassificada como **não comparável** com P543/P619 por metodologia/comportamento diferente: o cristalino de P518 não estava a fazer o mesmo trabalho.

## Fecho da pergunta de P546

A pergunta registada em P546 — "fica registada para bisecção futura dedicada" — está agora respondida. A causa da inversão entre P518 e P543 é P538e, e a explicação é comportamental, não algorítmica.

## Proveniência da medição

- Commit base (relatório): `1149271908cf10be458b0f2722316668f1e292db`
- Commits de fronteira: P518 `7bd72a2ce`, P543 `14a6d0628`
- Script de bissecção: `/tmp/p620-bisect.sh`
- Comando de medição: `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p620-bisect.pdf --timings-json /tmp/p620-timings.json`
- Data/hora aproximada: 2026-07-08, após P619.
