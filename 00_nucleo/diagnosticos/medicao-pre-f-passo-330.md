# Medição pré-F (P330 Parte 2) — baseline de performance pós-lotes

**Zero decisão, zero otimização.** Só os números que o diagnóstico do F vai
consumir (medir ≠ mexer). Tirado com o hub no **estado final pós-lotes** (commit
`acf64534b`, Lote 15 / Block fechado — 65 variantes migradas).

Substitui o baseline **M3/P318** como referência do F (o M3 estava quantizado;
ver § Comparação). Referência cruzada adicionada em
`medicao-pre-f-passo-318.md`.

---

## Ferramenta e ambiente

- **`hyperfine` indisponível** neste ambiente → **fallback**: `/usr/bin/time -f
  "%e"`, **12 execuções por cenário, 1ª descartada (warmup)**, média ± σ sobre
  as 11 restantes. Caminho usado: fallback (registrado por exigência da receita).
- **rustc** `1.92.0 (ded5c06cf 2025-12-08)`, profile `release` (optimized).
- **Binário**: `./target/release/typst` (de `04_wiring`).

## Corpus e comando (os mesmos do M3/P318, + 10×)

- **1×**: `00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ` — **7003
  linhas** (idêntico ao M3, comparabilidade).
- **10×**: o corpus 1× concatenado 10 vezes — **70030 linhas** (afasta-se da
  granularidade de 0.01 s, onde o número 1× é informativo).

```bash
# por execução (×12, 1ª descartada):
/usr/bin/time -f "%e" ./target/release/typst <CORPUS> -o /tmp/out.pdf
```

Mede o pipeline **completo** eval→layout→**export PDF** (mesma ressalva do M3:
sem isolamento por fase).

## Medições (commit `acf64534b`, pós-L15)

| Cenário | Linhas | Baseline | Nota |
|---------|--------|----------|------|
| **1×** | 7 003 | **0.0600 s ± 0.0000** (n=11) | quantizado (granularidade 0.01 s) — todas as runs idênticas; só para regressão grosseira |
| **10×** | 70 030 | **0.6518 s ± 0.0057 s** (n=11; min 0.64, max 0.66) | **o baseline real** — σ ≈ 0.9%, longe da granularidade |

**Extras** (números que o F pode afetar; baratos):

- **`cargo build --release` incremental** (após `touch content.rs`): **7.06 s**.
- **Suíte `typst-core`** (2697 testes): **0.37 s**.

## Comparação com M3/P318

- **M3** (P318): baseline **0.07 s** (mediana, 5 runs) no corpus 1× (7003 linhas),
  com **caveat C1/P319**: a granularidade de 0.01 s do `/usr/bin/time` ⇒
  quantização **~15%** — serve só para regressões grosseiras.
- **P330 1×**: **0.06 s** — dentro do ruído de quantização do M3 (Δ 0.01 s = 1
  tick). **Não conclusivo** por si só (a quantização domina).
- **P330 10×**: **0.6518 s ± 0.0057** — o **primeiro baseline com σ útil** do
  roteiro (a quantização deixa de dominar a 70k linhas). É **este** que o F deve
  usar como antes.
- **Nada foi otimizado** entre M3 e P330; a migração D não tinha performance como
  objetivo (medir ≠ mexer). A diferença 0.07→0.06 (1×) é ruído de quantização,
  não um ganho medido.

## Regra para o "depois" do F (já gravada, repetida aqui)

O "depois" do F **refaz antes+depois no par de commits** do diagnóstico, com a
**mesma ferramenta/corpus** (preferencialmente `hyperfine` se disponível; senão
este fallback, corpus 10×). Comparar 1× contra 1× é inválido (quantização);
usar o **10×** como métrica.

## Caveat conhecida

Stack default estoura em `recursao_infinita_*` — não é regressão; suíte corre com
`RUST_MIN_STACK=33554432`. (Não afeta este baseline — é compile/render de corpus,
não os testes de recursão.)
