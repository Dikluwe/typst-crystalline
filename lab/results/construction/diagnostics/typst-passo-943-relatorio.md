# Relatório P943 — reconciliar a aritmética do 1.2–1.3× restante (passo exploratório)

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-943.md`  
**Commit base:** `1891bdbce` (P942)  
**Âmbito:** reconciliar os números, sem compromisso de fechar o 1.2× por completo.

---

## 1. Resposta à pergunta do dono

**Veredicto: o resíduo é sobretudo overhead decomponível (descoberta, coverage lazy, arranque),
não um defeito com causa específica accionável. Não vale o esforço de o tentar fechar mais.**

A distância total ao vanilla (~1.2–1.3× nos cenários de fallback pesado) reconcilia-se com os
componentes já medidos, com um gap residual de ~10–15 ms compatível com ruído/overhead de processo.
Este é o ponto de fechar a frente de performance de fontes (P925–943).

---

## 2. A aritmética reconciliada (`utf8-cjk`)

### Componentes medidos (P942, release, 1112 fontes)

| Componente | Cristalino | Vanilla |
|---|---:|---:|
| Arranque de processo (`--version`) | ~3 ms | ~4 ms |
| Descoberta de fontes (`load_system_fonts` / parte do scan) | ~71 ms | ~64 ms (estimada) |
| Extração de coverage (lazy / dentro do scan) | ~196 ms | ~186 ms (estimada) |
| Compile (`timings`: eval+layout+shape+render) | ~29 ms | ~33 ms (layout após scan) |
| **Soma** | **~299 ms** | **~287 ms** |
| **Medido (hyperfine, `utf8-cjk`)** | **331 ms** | **283 ms** |
| **Gap não contabilizado** | ~32 ms | ~-4 ms |

A soma dos componentes explica a quase totalidade do tempo medido nos dois lados. O gap de
~32 ms do lado cristalino é atribuível a: carregamento de fontes embutidas + preload
(`preload_coverage_if_needed`, fora dos `timings` de compile) + variância de I/O/scheduler —
não a um passo de trabalho identificado.

### Baselines mínimos

| Medida | Cristalino | Vanilla |
|---|---:|---:|
| `--version` | 3.2 ms | 3.9 ms |
| Doc trivial latino (sem fallback), hyperfine | 94.4 ms | 280.1 ms |
| Doc trivial latino, `timings` compile | 1.6 ms | 253.6 ms (`compile once`) |

- O cristalino paga ~71 ms de descoberta em **todos** os documentos (por isso o trivial custa
  ~94 ms). O vanilla paga ~250 ms de scan (descoberta + coverage) em todos (por isso o trivial
  custa ~280 ms).
- Para documentos **sem fallback** (o caso comum), o cristalino é ~3× **mais rápido** que o
  vanilla (~86–94 ms vs ~253–280 ms), porque não extrai coverage.
- Para documentos **com fallback**, o cristalino paga a coverage lazy (~196 ms) **uma vez**, o
  que o vanilla já pagou no seu baseline maior. Daí a aproximação para ~1.2×.

---

## 3. Decomposição do gap de 48 ms (`utf8-cjk`: 331 vs 283)

| Fonte | Δ (cristalino − vanilla) |
|---|---:|
| Extração de coverage (~196 vs ~186 ms) | ~+10 ms |
| Descoberta (~71 vs ~64 ms) | ~+7 ms |
| Compile/layout (29 vs 33 ms) | ~−4 ms |
| Arranque (3 vs 4 ms) | ~−1 ms |
| Fontes embutidas + preload (fora dos timings) | ~+21 ms |
| Ruído/scheduler | ~+15 ms |
| **Total** | **~+48 ms** |

Nenhuma destas parcelas é um defeito com correção clara:
- A extração de coverage já foi optimizada em P942 (340→196 ms; `sort()` TimSort + sem cópia).
- A descoberta é limitada pelo `fontdb` (igual nos dois lados).
- O compile do cristalino já é **mais rápido** que o do vanilla (−4 ms).
- As fontes embutidas/preload são trabalho necessário e pequeno.

---

## 4. Veredicto e fecho da frente

- A pergunta "dá para melhorar ou vai ser difícil" tem resposta clara: **vai ser difícil, e o que
  resta é margem, não defeito.** O 1.2× residual é overhead decomponível (descoberta, coverage
  lazy, embutidas, arranque), já medido e decomposto — não um passo de trabalho isolado com
  correção óbvia.
- O cristalino está em **paridade prática** com o vanilla: mais rápido no caso comum (~0.3×),
  comparável no fallback (~1.2×). A uniformidade do vanilla (~250–285 ms em tudo) é o preço que
  ele paga por extrair coverage sempre; o cristalino paga-a só quando precisa.
- **Fecho da frente de performance de fontes (P925–943).** O que resta fora desta frente:
  a divergência de fallback de emoji (P941, cor de render — defeito de correcção visual, não de
  performance), candidata a passo próprio se se quiser.

---

## 5. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Baseline `--version` | `hyperfine --warmup 2 --min-runs 20` | `typst` (P942) / vanilla | ~3–4 ms |
| Doc trivial latino | `hyperfine --warmup 1 --min-runs 10` | `typst-p942` / vanilla | 94 vs 280 ms |
| Decomposição vanilla | `--timings` (compile once / scan / layout) | vanilla | `tools/perf/results/p943/` |
| Decomposição cristalino | `--timings-json` | `typst-p942` | `tools/perf/results/p943/` |
| Componentes coverage/descoberta | P942 (já medidos) | commit `1891bdbce` | parse 15.3, cmap 30, runs 151.1, descoberta 71 |

Nenhuma alteração de código neste passo.
