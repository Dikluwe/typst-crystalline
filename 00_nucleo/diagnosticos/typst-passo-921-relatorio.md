# Relatório P921 — `assembly` não atinge a altura-alvo em matrizes/casos de 6+ linhas

**Commit base (antes deste passo):** `1e9b7c94f` (P920, relatório final).
**Commit produzido por este passo:** `2ff9218da` (1 commit — `mod.rs`, `tests.rs`, `_comum.md`).
**Baseline de testes (antes, herdado de P920):** typst-core: 4800 · typst-infra: 743 ·
typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** typst-core: 4800 (mesma contagem — nenhum teste novo,
3 ajustados) · typst-infra: 743 · typst-shell: 41 — `0 failed`.
**Data:** 2026-07-27.

---

## Fase A — a premissa do passo estava errada; causa real é mais funda e mais ampla

**Premissa refutada por medição directa com o vanilla real** (`lab/typst-original/target/release/
typst`, `typst 0.15.0`, já compilado): `mat(1,2;3,4;5,6;7,8;9,10;11,12)` a 20pt — tracei com
`mutool trace` e confirmei que o vanilla **também usa `assembly`** (glifo topo + 11 repetições de
peça extensora + glifo fundo) para este caso, **não** uma variante única como o achado de
P916/917 (que motivou este passo) assumia.

**Causa real, encontrada por comparação directa dos dois PDFs** (mesmo `.typ`, mesmo tamanho):
`mediabox` final — **141.693×210.165** (vanilla) vs **145.69×298.33** (cristalino, antes deste
passo) — o cristalino produzia uma página muito MAIS alta, o oposto do sintoma "não atinge a
altura" que o nome do passo descrevia. Duas causas reais, confirmadas por leitura directa:

1. **`layout_text_node` (`mod.rs:607-628`) usava a fonte de métricas errada.** Construía
   `ascent`/`descent` de QUALQUER folha de texto em modo matemático via
   `FontMetrics::vertical_metrics` (métricas OS/2 globais — `sTypoAscender`/`sTypoDescender`/
   `sTypoLineGap`, pensadas para altura de linha de texto corrido) em vez de
   `FontMetrics::text_ink_bounds` (bbox real do glifo). Isto contradizia uma regra já documentada
   desde **P813** (comentário de `text_ink_bounds`: "paridade vanilla: o ascent/descent de um
   frame math vem das bboxes dos glyphs, não das métricas globais") — só que essa correcção nunca
   tinha chegado ao caminho usado por CADA folha, só à extensão agregada da equação inteira.
   Medido (`fontTools`, `NewCMMath-Regular.otf`): um dígito sem descendente ("1"/"2") ganhava
   `descent=0.394em` fabricado, quando a tinta real desce `0em`.
2. **`layout_grid_boxes` não tinha o piso de altura por linha via `(` sintético** que o vanilla
   aplica (`typst-layout/src/math/table.rs:67-85`: "we pad ascent and descent with the ascent and
   descent of the paren to ensure that normal matrices are aligned with others unless they are way
   too big"). Sem isto, uma grelha com conteúdo mais curto que um `(` (ex.: só dígitos) fica mais
   rasa que o vanilla mesmo depois de corrigido o achado 1.

Achados laterais confirmados, não a causa principal, registados: `math_leading` do cristalino
(lido da tabela MATH real, ~0.154em) vs `DEFAULT_ROW_GAP` do vanilla (constante fixa, 0.2em) —
mecanismo diferente, magnitude pequena.

## Fase A.1 — gate em duas rondas (blast radius medido antes de decidir)

1ª ronda: achado apresentado como correcção sistémica de grande risco (afecta potencialmente todo
o motor de layout matemático, não só matrizes) — o dono pediu para medir o raio de acção antes de
decidir. Experimentei a correcção (1) isoladamente, medi: **3 de 4800 testes afectados**, todos
sintéticos com `FixedMetrics`/`StubHorizontalMetrics` (mudança de comportamento já documentada
desde P813 — o `default` de `text_ink_bounds` para stubs sem bbox real, `cap_height`/zero, é
diferente do `default` de `vertical_metrics`, proporção fixa 0.8/0.4 — não é regressão com fonte
real). Visual (30 secções, `.typ/typst-math-comprehensive-test.typ`): página mais compacta
(13186px→12106px a 150dpi), sem sobreposição, coerente. 2ª ronda: aprovado avançar; medição
geométrica revelou o segundo mecanismo (piso de `(` sintético) em falta — apresentado, aprovado
incluir na mesma sessão.

## Fase B — TDD directo (não protocolo de dois agentes — causa é ajuste de fonte de métrica, não
lógica de decisão, per critério do próprio `typst-passo-921.md`)

1. `layout_text_node`: `ascent`/`descent` passam a vir de `self.metrics.text_ink_bounds(text,
   style.size, style)` em vez de `vertical_metrics`.
2. `layout_grid_boxes`: `paren_ascent`/`paren_descent` de um `(` sintético em estilo de
   denominador (`size: style.size * script_percent_scale_down`, `cramped: true` — mesmo padrão de
   `den_style` em `frac.rs`), calculado uma vez; `row_ascent`/`row_descent` de cada linha (e da
   próxima linha, no cálculo de `advance`/`total_descent`) passam a `.max(paren_ascent)`/
   `.max(paren_descent)`.
3. 3 testes sintéticos pré-existentes ajustados (não aceites cegamente — valores re-derivados da
   fórmula com o novo `leaf_ascent`/`leaf_descent` de `FixedMetrics`/`StubHorizontalMetrics`):
   `p905_frac_numerador_tem_gap_acima_da_linha`,
   `axis_bug_frac_numerador_e_denominador_acompanham_o_deslocamento` (P919),
   `p915_cramped_e_sup_sub_simultaneos_interagem_sem_cancelar_piso_cramped` (P915/P916 — ajuste de
   tolerância de ponto flutuante, o contrato semântico do teste continua a valer).
4. Suíte completa verde, 2 execuções consecutivas para confirmar estabilidade (`cargo test
   --workspace`: 4800/743/41, `0 failed`, ambas as vezes).

**Confirmação geométrica** (`mutool trace`, `mat(...)` 6 linhas, 20pt, vanilla real):

| Estado | Espaçamento de linha medido | Δ vs vanilla (23.92pt) |
|---|---|---|
| Antes de P921 | 27.08pt | +3.16pt (13% acima) |
| Depois do achado 1 só | 16.84pt | -7.08pt (30% abaixo) |
| Depois de achado 1 + 2 | 19.96pt | -3.96pt (17% abaixo) |

**Achado residual, registado, não resolvido**: mesmo com os dois mecanismos implementados
fielmente (cada um confirmado por leitura directa do vanilla, `file:line`), sobra uma diferença de
~4pt/linha não isolada nesta sessão. Não bloqueia o fecho do passo — os dois mecanismos são,
individualmente, portes correctos da fórmula real do vanilla (paridade é com a fórmula, per
ADR-0107/0123, não com o resultado numérico bit-a-bit de uma implementação mecanicamente
diferente); a causa do resíduo fica como candidato a investigação futura.

## Fase C — Benchmark (7 cenários, `hyperfine --warmup 5 -N -m 20`)

Binários `antes` (`1e9b7c94f`) / `depois` (`2ff9218da`), release, mesma sessão.

| Cenário | Razão (depois vs antes) |
|---|---|
| 01-hello | 1.00× |
| 02-lorem | 1.02× (outlier reportado pelo próprio hyperfine, σ alto) |
| 03-images | 1.00× |
| 04-math | 1.00× |
| 05-tables | 1.01× |
| 06-long | 1.00× |
| 07-context | 1.00× |

**Nenhuma regressão atribuível a P921** — banda 1.00–1.02×, mesma faixa de ruído já estabelecida
em P909/P915/P917/P918/P919/P920.

---

## Resultado final

| Item | Estado |
|---|---|
| Causa confirmada por medição com o vanilla real, não a premissa original | ✅ |
| Blast radius medido antes de decidir (a pedido do dono, 2 rondas) | ✅ 3/4800 testes, sintéticos |
| Fase B — TDD directo, 2 mecanismos implementados, `file:line` do vanilla citado | ✅ |
| Suíte completa, 2 execuções consecutivas confirmando estabilidade | ✅ core 4800 · infra 743 · shell 41, `0 failed` |
| `crystalline-lint .` | ✅ 0 drift novo |
| Confirmação geométrica (`mutool trace` contra vanilla real) | ✅ Melhoria de 74pt→18pt de página, resíduo registado |
| Benchmark 7 cenários | ✅ Sem regressão, banda 1.00–1.02× |
| Commit | ✅ `2ff9218da` |

### Achados em aberto desta frente, não tocados por este passo

- **Resíduo de ~4pt/linha** em matrizes/grelhas, não isolado (candidato a passo futuro se for
  considerado relevante).
- **`math_leading` (cristalino, tabela MATH real) vs `DEFAULT_ROW_GAP` (vanilla, constante fixa)**
  — mecanismo diferente, magnitude pequena, registado.
- **`accent_base_height`** (destacado em P920 para passo dedicado) — bloqueio arquitectural de
  sinal de `descent` ainda por resolver.
- Decisão variante-vs-assembly em `stretchy.rs` (Fase A ponto 3 do passo original) — não chegou a
  ser comparada em detalhe com o vanilla, dado que a causa real (achados 1/2 acima) tornou essa
  comparação menos urgente; ambos os sistemas usam `assembly` para 6 linhas, confirmado, mas a
  condição exacta de troca variante↔assembly não foi auditada linha a linha.

### Nota — fecho da frente de geometria matemática (P885-921)

O ficheiro do passo perguntava explicitamente se, no fim de P919/P920/P921, seria hora de fechar
esta frente. Cada um dos três revelou achados mais fundos do que o registo original assumia
(P919: bug de omissão em `apply_axis_offset`, não "falta de eixo comum"; P920: duas fórmulas
inteiras divergentes do vanilla, uma delas com bloqueio arquitectural, não "campos por ler"; P921:
bug de fonte de métricas afectando todo o motor, não "assembly incompleto") — um padrão
consistente de a investigação real encontrar mais do que o achado catalogado inicialmente
descrevia. Isto sugere que a frente **não deve ser fechada por decreto** só porque a rodada P919-
921 terminou — os achados residuais registados acima (sobretudo o resíduo de ~4pt/linha e a
`accent_base_height` destacada) são candidatos concretos a continuar, não hipotéticos. Decisão de
handoff fica para o dono.

### Proveniência

- Commit: `2ff9218da` (`git log -1`), working tree limpa nestes ficheiros no fim da sessão.
- Vanilla real compilado e usado directamente (`lab/typst-original/target/release/typst`,
  binário pré-existente, `typst 0.15.0`) para toda a Fase A e a confirmação geométrica final.
- Todos os números reproduzíveis via `cargo test --workspace`, `crystalline-lint .`, e o par de
  binários release `1e9b7c94f`/`2ff9218da` para o benchmark.
