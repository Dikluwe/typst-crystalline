# Relatório P920 — gap de `underover`/acento sem constante; `fraction_denom_gap` sem consumidor

**Commit base (antes deste passo):** `54328a52b` (P919, relatório final).
**Commit produzido por este passo:** `5b32f0c14` (Parte B — 1 commit; Parte A destacada, sem
código tocado além dos headers `@prompt-hash`).
**Baseline de testes (antes, herdado de P919):** typst-core: 4797 · typst-infra: 743 ·
typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** typst-core: 4800 (+3, os testes `p920_frac_*`) ·
typst-infra: 743 · typst-shell: 41 — `0 failed`.
**Data:** 2026-07-26/27.

---

## Fase A — os dois achados são mais fundos do que o registo original assumia

O passo tratava dois achados pequenos juntos, por vizinhança de código
(`typst-passo-906-relatorio.md`, gap de `underover`; `typst-passo-918-relatorio.md`, achado
lateral de `fraction_denom_gap`). Leitura directa da fórmula real do vanilla revelou que ambos
são mais fundos do que "falta uma constante":

### Parte A — `underover.rs`/`accent.rs`: premissa original errada, mecanismo real é acento

`resolve.rs:1277-1472` do vanilla (`resolve_underoverspreader`): `underbrace`/`overbrace`/
`underbracket`/etc. resolvem como `AccentItem` — o MESMO mecanismo de `hat`/`tilde`
(`accent.rs:56-71` do vanilla) — **não** como `LineItem` (`underbar_vertical_gap`/
`overbar_vertical_gap`, exclusivos de `underline()`/`overline()`, que o cristalino nem
implementa como `MathUnderover`). A fórmula real tem dois ramos:

- **Abaixo** (`underbrace`, `under_y`): `gap = -accent.ascent()` — sem `accent_base_height`,
  equivale a empilhamento justo puro. `under_y` do cristalino **já implementa exactamente isto**
  — parte do achado original de P906 estava certa sobre o sintoma, errada sobre onde se aplicava.
- **Acima** (`overbrace`/`hat`/`tilde`, `over_y`/`accent_y`): `gap = -accent.descent() -
  base.ascent().min(accent_base_height)` — um cap, não constante aditiva.

**Correcção de uma leitura inicial errada**: uma primeira versão desta investigação (registada
nos L0s, depois corrigida) tinha concluído "bases altas ganham espaço extra" — o oposto do que o
comentário do próprio vanilla diz (`accent.rs:57-60`): *"Only if the base is very small, we need a
larger gap so that the accent doesn't move too low"* — são as bases pequenas que ganham mais
espaço.

**Bloqueio real, não resolvido neste passo**: `-accent.descent()` no vanilla pode ser **negativo**
(tinta do acento inteiramente acima da própria baseline, comentário explícito do vanilla). O
contrato `FontMetrics::text_ink_bounds` do cristalino garante `ascent`/`descent` sempre `>= 0`
(`engine/layout/metrics.rs:59-61`) — perde exactamente essa informação. Tentativa de aproximar
(`accent.descent() ≈ 0`) produz sobreposição (gap negativo), não uma aproximação inofensiva.
Tentativa de resolver por medição directa com o binário vanilla real (`lab/typst-original/target/
release/typst`, já compilado) confirmou a direcção do comentário (bases pequenas → mais gap) mas
não resolveu como representar o termo com sinal no modelo do cristalino sem uma extensão de
contrato. **Destacado para passo dedicado** — pergunta a resolver na Fase A desse passo: estender
`FontMetrics` para expor extensões com sinal (ou mecanismo equivalente), decisão arquitectural
própria.

### Parte B — `frac.rs`: a fórmula toda diverge, não só um campo por ler

`fraction.rs:30-53` do vanilla: `num_gap = (shift_up - axis - thickness/2 -
num.descent()).max(num_min)`; `denom_gap = (shift_down + axis - thickness/2 -
denom.ascent()).max(denom_min)`. `fraction_num_gap`/`fraction_denom_gap` (campos já existentes no
cristalino) servem de **piso mínimo**, não de gap directo — o cristalino usava
`fraction_num_gap` directamente como gap aditivo, o MESMO valor duplicado nos dois lados. Requer 2
campos novos (`fraction_numerator_shift_up`/`fraction_denominator_shift_down`) — sem o bloqueio de
sinal da Parte A (`num.descent()`/`denom.ascent()` são de conteúdo normal, sempre `>= 0`).

## Fase A.1 — gate, escopo dividido a meio da investigação

Aprovação inicial do dono foi para as duas partes com escopo completo. Investigação da Parte A
(própria Fase B, antes de qualquer código tocado) revelou o bloqueio de sinal descrito acima —
não resolvível como "adicionar uma constante". Apresentado ao dono; decisão: avançar só com a
Parte B, destacar a Parte A para passo dedicado. L0s corrigidos (`accent.md`, `underover.md`,
`math_constants.md`, `font_metrics.md`) para reflectir a decisão antes de qualquer implementação.

## Fase B — protocolo de dois agentes (mesmo protocolo de P898/P919)

**Agente A** (subagente isolado): escreveu 10 testes cobrindo as duas partes (na altura, escopo
completo ainda não tinha sido dividido), com ground-truth medido via `fontTools`
(`NewCMMath-Regular.otf`) e leitura directa do vanilla. Reportou explicitamente, no cabeçalho do
seu módulo de testes, a mesma ambiguidade de direcção da Parte A que o orquestrador confirmou
depois — não comprometeu a direcção nos testes (`assert_ne!`, não `>`/`<`), recomendando
confirmação contra o vanilla real antes de implementar.

**Revisão do orquestrador, antes do Agente B**: confirmação adicional (compilação do vanilla real,
já existente em `lab/typst-original/target/release/typst`; render visual; leitura do comentário
`accent.rs:57-60`) resolveu a DIRECÇÃO do efeito mas expôs o bloqueio de sinal descrito acima.
Removidos os 6 testes da Parte A de `tests.rs` (não compilavam sem o campo, e a implementação foi
adiada) — mantida a nota de investigação como registo histórico. L0s corrigidos.

**Agente B** (subagente isolado, só Parte B): mediu de novo os valores reais via `fontTools`
(confirmou 394/345, batendo com o que os testes do Agente A já assumiam), adicionou os 2 campos a
`MathConstants` + `font_metrics.rs`, reescreveu `frac.rs` com a fórmula completa. Encontrou e
corrigiu 1 teste pré-existente (`axis_bug_frac_numerador_e_denominador_acompanham_o_deslocamento`,
de P919) cuja premissa "caso simétrico" deixou de valer com a fórmula real (gaps legitimamente
diferentes mesmo com num/den do mesmo tamanho, por `fraction_numerator_shift_up` ≠
`fraction_denominator_shift_down` serem constantes distintas por desenho) — investigado per
ADR-0108 antes de ajustar, não "arranjado às cegas".

**Revisão do orquestrador, depois do Agente B**:
- Diagnóstico de compilação reportado durante a sessão (`E0063`, campos em falta num
  `MathConstants{...}` de `font_metrics.rs`) investigado e confirmado como diagnóstico
  transitório/desactualizado — `cargo build --workspace` limpo, sem erros, confirmado
  repetidamente.
- Valores `fontTools` (394/345) confirmados de forma independente pelo orquestrador, batem exacto.
- `cargo test --workspace` mostrou 1 falha isolada em `typst-infra` (742/1 failed) numa execução;
  3 execuções seguintes deram 743/0 failed — flake não reproduzível, quase certamente pré-existente
  (testes sensíveis a descoberta de fontes do sistema), não atribuído a este passo.
- Confirmação geométrica real (`mutool trace`, binário release, `$x + frac(a_1, b)$` — numerador
  com subscrito): barra a 4.12pt do numerador (com subscrito, descent real) vs 7.58pt do
  denominador (plano) — gaps genuinamente assimétricos, confirmando a correcção; antes do fix, os
  dois gaps eram forçados a ser iguais.

## Fase C — Benchmark (7 cenários, `hyperfine --warmup 5 -N -m 20`)

Binários `antes` (`54328a52b`) / `depois` (`5b32f0c14`), release, mesma sessão.

| Cenário | Razão (depois vs antes) |
|---|---|
| 01-hello | 1.00× |
| 02-lorem | 1.00× |
| 03-images | 1.00× |
| 04-math | 1.01× (depois mais rápido — ruído) |
| 05-tables | 1.00× |
| 06-long | 1.01× (depois mais rápido — ruído) |
| 07-context | 1.00× |

**Nenhuma regressão atribuível a P920** — banda 1.00–1.01×, mesma faixa de ruído já estabelecida
em P909/P915/P917/P918/P919.

---

## Resultado final

| Item | Estado |
|---|---|
| Fase A — causa real confirmada por leitura do vanilla (não a premissa original) | ✅ Parte A e Parte B |
| Fase A.1 — gate, escopo corrigido a meio da investigação, L0s consistentes | ✅ |
| Protocolo de dois agentes (P898) — Agente A testes, revisão do orquestrador antes do Agente B | ✅ |
| Parte B implementada (2 campos novos, fórmula completa) | ✅ |
| Parte A destacada para passo dedicado, com a pergunta arquitectural precisa registada | ✅ |
| Confirmação geométrica real (`mutool trace`) | ✅ Gaps assimétricos confirmados |
| Suíte completa, verificada independentemente | ✅ core 4800 (+3) · infra 743 · shell 41, `0 failed` |
| `crystalline-lint .` | ✅ 0 drift novo |
| Benchmark 7 cenários | ✅ Sem regressão, banda 1.00–1.01× |
| Commit | ✅ `5b32f0c14` |

### Achados em aberto desta frente, não tocados por este passo

- **Parte A — `accent_base_height`** (destacada, passo dedicado): decidir se/como estender
  `FontMetrics`/`text_ink_bounds` para representar extensões com sinal (ou mecanismo alternativo)
  antes de portar `gap = -accent.descent() - base.ascent().min(accent_base_height)` fielmente.
- Desalinhamento de elementos em matrizes/casos de 6+ linhas (herdado de P917).
- Flake isolado em `typst-infra` (742/1 failed numa execução, 743/0 nas 3 seguintes) — não
  investigado a fundo, provavelmente ambiental, registado para se voltar a aparecer.

### Proveniência

- Commit: `5b32f0c14` (`git log -1`), working tree limpa nestes ficheiros no fim da sessão.
- Valores de fonte real confirmados independentemente três vezes (Agente A, Agente B, orquestrador)
  via `fontTools` sobre `NewCMMath-Regular.otf` (`~/.cargo/git/checkouts/typst-assets-
  525e6d15ef7950cb/c0ae970/files/fonts/`).
- Vanilla real compilado e usado directamente (`lab/typst-original/target/release/typst`,
  binário pré-existente, `typst 0.15.0`) para a investigação da Parte A — não só leitura de código.
- Todos os números reproduzíveis via `cargo test --workspace`, `crystalline-lint .`, e o par de
  binários release `54328a52b`/`5b32f0c14` para o benchmark.
