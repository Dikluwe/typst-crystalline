# Passo 963 — P959 corrigiu o limite inferior de operador grande, mas o superior continua ~5× errado

**Precede este passo**: auditoria externa, terceira rodada (2026-08-04) — depois de P959, limite
inferior de operador grande bate quase exato com o vanilla (mediana 3.57 vs 3.55pt). **Limite
superior continua muito divergente**: mediana 11.39pt (cristalino) vs 2.37pt (vanilla), sobre as
mesmas 37 ocorrências pareadas. P959 implementou fórmulas simétricas para `t_shift`/`b_shift`
(ambas com termo `max()`) e corrigiu os extents de tinta da base — mas só o lado inferior
melhorou.

**Pré-condição de árvore**: `git status`. Confirmar P962 (se já executado) presente.

---

## Fase A — confirmar por que só um lado corrigiu

1. Reler `typst-passo-959-relatorio.md` — confirmar exatamente quais termos/constantes foram lidos
   e aplicados para `t_shift` (superior) vs `b_shift` (inferior) — a fórmula documentada era
   simétrica, mas a implementação pode não ter aplicado os dois lados de forma simétrica de
   verdade.
2. Isolar um caso mínimo com limite só acima (`limits: #true` num operador com só sobrescrito, ou
   `lim` com seta acima) e medir directamente (`mutool trace`/`pdftotext -bbox`) o shift superior
   no binário atual — confirmar a divergência isoladamente, fora do documento de 30 seções.
3. Ler o código actual de `attach.rs` para o braço que trata o limite superior especificamente —
   confirmar se ele consome `UpperLimitGapMin`/`UpperLimitBaselineRiseMin` correctamente, ou se há
   uma diferença sutil (por exemplo, usar `descent` onde devia usar `ascent`, ou aplicar a extensão
   de tinta da base só ao cálculo do lado inferior por engano).
4. Confirmar os valores reais de `UpperLimitGapMin`/`UpperLimitBaselineRiseMin` na fonte
   (`fontTools`, mesmo método de P959) e recalcular manualmente o valor esperado para o caso
   isolado, para comparar com o que o cristalino produz.

## Fase B — Implementação (TDD directo, correção pontual — mesma fórmula de P959, só aplicada ao
lado que falta)

1. Teste com o valor esperado do limite superior, derivado da fórmula e dos valores reais da Fase
   A — cobrir pelo menos um caso só-superior e um caso com os dois (superior+inferior juntos, para
   confirmar que a correção não desequilibra o que já estava certo no inferior).
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir as 37/38 ocorrências de novo (mesmo método da auditoria externa) — confirmar que o
   limite superior cai para a banda do vanilla (~1.9-3.8pt), sem regredir o inferior já corrigido.
2. `compare.py` nas seções com somatórios/integrais.
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta de por que só o lado inferior foi corrigido por P959.
- Limite superior corrigido, inferior preservado, com números reais antes/depois.
- Benchmark sem regressão.
