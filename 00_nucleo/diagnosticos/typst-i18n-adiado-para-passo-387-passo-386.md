# Eixo 2 (prontidão i18n) — adiado para Passo 387

**Tipo**: Nota de adiamento (registro de decisão; não materializa nada).
**Data**: 2026-06-21.
**Origem**: Passo 386 §C + §156 (gatilho de split) + decisão do dono no início da execução.

---

## Decisão

O Passo 386 tinha dois eixos. O **eixo 2 — prontidão para i18n** destaca-se como **Passo
387 dedicado**, por decisão do dono no início da execução, acionando o gatilho previsto no
§156 do passo ("se a superfície de strings for grande, §C vira Passo 387").

O **eixo 1** (Listas A e B — dívida de feature) foi entregue neste passo:
- `typst-falta-migrar-lista-A-passo-386.md`
- `typst-falta-migrar-lista-B-passo-386.md`
- `lab/parity/tools/falta_migrar.py` (+ teste)

## Por que o split (dado que motivou)

Medição inicial da superfície de strings user-facing (gatilho objetivo, não narrativa):

| Medida | Valor (só L1) |
|--------|--------------:|
| Sítios de construção de erro/format (`SourceDiagnostic`/`error!`/`bail!`/`eco_format!`/`format!`) | ~1062 |
| Literais string em contexto de validação/erro | ~482 |

A classificação L/H/N sítio-a-sítio de ~1000+ sítios é, ela própria, escopo de um passo
dedicado — exatamente o que o §156 antecipa. Fazê-la diluída no 386 inflaria o passo e
daria à i18n menos granularidade do que merece.

## O que o Passo 387 deve produzir (herdado do §C do 386)

1. **Lista C** — inventário da superfície de strings user-facing em L1/L2/L3, cada sítio
   `ficheiro:linha` classificado:
   - **(L)** já lang-aware (passa por `rules/lang/` ou catálogo indexado por `Lang`);
   - **(H)** literal hardcoded (não-localizável);
   - **(N)** não-user / interno (`panic!`/debug/trace) — fora de i18n.
   Três categorias de sítio: **erro/diagnóstico**, **supplement/rótulo**, **saída formatada**.
   Saída: `typst-i18n-superficie-strings-passo-387.md`.
2. **ADR i18n PROPOSTA** (varrer número livre — ADR-0110 ocupada; 0111 candidato):
   catálogo indexado por chave + `Lang`, construído sobre `figure_supplement_for_lang` /
   `localize_quotes` / `rules/lang/`; parametrização translation-safe; roadmap de passos;
   **política de default declarada** (separar língua do *conteúdo do documento* da língua das
   *mensagens do compilador*).
3. **DEBT i18n** (rastreador, EM ABERTO, XL).

## Pista já levantada pelo eixo 1 (insumo para o 387)

A Lista B mediu **`typst_library::diag` = 24** itens no resíduo — maquinaria de diagnóstico,
território direto da Lista C (categoria erro/diagnóstico). O ponto de partida do inventário
do 387 é o construtor de erro do cristalino: `01_core/src/entities/source_result.rs`
(`SourceDiagnostic`) e os ~1062 sítios que o invocam.
