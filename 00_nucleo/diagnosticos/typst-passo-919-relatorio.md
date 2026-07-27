# Relatório P919 — desalinhamento vertical entre elementos adjacentes numa sequência matemática

**Commit base (antes deste passo):** `56d85a6f5` (P918, relatório final).
**Commit produzido por este passo:** `cf7f57d1f` (Fase B — 1 commit, tocando `mod.rs`, `frac.rs`,
`root.rs`, `delimited.rs`, `cases.rs`, `matrix.rs`, `tests.rs`, 6 L0s).
**Baseline de testes (antes, herdado de P918):** typst-core: 4790 · typst-infra: 743 ·
typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** typst-core: 4797 (+7, os testes novos do Agente A) ·
typst-infra: 743 · typst-shell: 41 — `0 failed`.
**Data:** 2026-07-26.

---

## Fase A — causa confirmada por leitura + medição (não a hipótese original tal e qual)

**Achado motivador** (`typst-passo-917-relatorio.md`, "Achado registado, não corrigido"):
`x^2_i + (1/2)` mostra `(1/2)` desalinhado verticalmente. Hipótese registada nesse relatório —
"`apply_axis_offset` recentra cada `MathBox` independentemente antes de `hconcat`, sem eixo comum
entre irmãos" — **refutada em parte, refinada em parte** por leitura directa:

- `hconcat_spaced` (`mod.rs`) confirmado: só ajusta `pos.x`, nunca `pos.y` — assume que todas as
  caixas-irmãs já partilham `local_y=0=baseline`.
- `apply_axis_offset` (`mod.rs:343`, antes deste passo): só ajustava `b.ascent`/`b.descent`
  (metadados) — **nunca `b.items`**. Não "recentra cada caixa de forma diferente" (não desloca
  nenhuma) — é um **bug de omissão**: faltava deslocar os items, mesmo padrão que
  `layout_stretchy_delimiter` (`stretchy.rs`) já implementa correctamente.
- Medição real (`mutool trace`, fonte real embutida, `frac(a,b)` ao lado de texto): a barra da
  fracção estava a **0.046pt** da baseline partilhada — praticamente zero, quando devia estar a
  `axis_height` de distância. Confirma quantitativamente o bug de omissão.

**Confirmado por leitura do vanilla que os 5 call sites não são uniformes**
(`lab/typst-original/crates/typst-layout/src/math/`):

| Ficheiro | Vanilla | Decisão |
|---|---|---|
| `fraction.rs:66,69` | `baseline = line_pos.y + axis` — barra fixa a `axis_height`, por construção | `frac.rs` ganha fix próprio (não usa `apply_axis_offset` — fórmula genérica diverge em fracções assimétricas) |
| `table.rs:188` | `set_baseline(height/2.0 + axis)` — centra o meio da altura total | `cases.rs`/`matrix.rs` continuam a usar `apply_axis_offset`, corrigida |
| `radical.rs:110` | `set_baseline(ascent)` — **sem** termo de axis | `root.rs` deixa de chamar `apply_axis_offset` |
| `fenced.rs` | sem centragem do grupo delimitado; delimitadores já pré-centrados | `delimited.rs` deixa de chamar `apply_axis_offset` |

Confirmado também **empiricamente** (`mutool trace`, não só por leitura do vanilla):
`$ x + sqrt(a) + y $` e `$ x + (a) + y $` mostram `x`/`a`/`y` já exactamente ao mesmo Y hoje —
`root`/`delimited` já estavam correctos, "acidentalmente", só porque o bug de omissão nunca
mexia nos items.

## Fase A.1 — gate cumprido (2 rondas de confirmação do dono)

Design inicial apresentado e refinado em 2 rondas (o dono pediu investigação adicional antes de
aprovar):
1ª ronda: design genérico (fix `apply_axis_offset` + manter em frac/cases/matrix + remover de
root/delimited). 2ª ronda, após medição empírica de root/delimited e leitura completa da fórmula
exacta do vanilla para `frac`: refinamento — `frac.rs` não pode reusar a fórmula genérica de
`apply_axis_offset` (diverge em fracções assimétricas), precisa de fix próprio com shift fixo por
`axis_pt`. Aprovado nesta segunda forma.

6 L0s editados e hash sincronizado: `_comum.md`, `frac.md`, `root.md`, `delimited.md`, `cases.md`,
`matrix.md`.

## Fase B — protocolo de dois agentes (mesmo protocolo de P898)

**Agente A** (subagente isolado, sem contexto desta sessão): leu o bug e as 5 fontes vanilla
citadas, compilou o binário actual, mediu com `mutool trace` (confirmou empiricamente o
desalinhamento e os casos "não deve mudar"), e escreveu 7 testes em `tests.rs`:

- `axis_bug_frac_bar_deve_ficar_a_axis_height_da_baseline_vizinha`
- `axis_bug_frac_numerador_e_denominador_acompanham_o_deslocamento`
- `axis_bug_frac_axis_height_generaliza_para_tres_elementos`
- `axis_bug_cases_conteudo_centra_no_axis_height_nao_a_zero`
- `axis_bug_matrix_conteudo_centra_no_axis_height_nao_a_zero`
- `axis_ok_sqrt_radicando_nao_ganha_deslocamento_de_axis_height`
- `axis_ok_delimitado_corpo_nao_ganha_deslocamento_de_axis_height`

5 vermelhos, 2 verdes (guarda de não-regressão), confirmado contra o código pré-P919. Achado
crítico do Agente A, incorporado ao design antes da implementação: em `cases.rs`/`matrix.rs`,
`result.items` mistura os items do(s) delimitador(es) (já auto-centrados por
`layout_stretchy_delimiter`) com os da grelha (ainda não) — deslocar `result` inteiro no fim
deslocaria os delimitadores **duas vezes**. L0s corrigidos antes de avançar (ver Fase A.1).

**Agente B** (subagente isolado separado, recebeu os testes já escritos — sem poder editá-los —
mais a especificação de implementação): implementou os 5 pontos (fix de `apply_axis_offset`,
`frac.rs` próprio, remoção em `root.rs`/`delimited.rs`, `apply_axis_offset` aplicada a `grid_box`
isolado em `cases.rs`/`matrix.rs`). **Encontrou e corrigiu um erro de sinal na especificação**
que lhe dei: eu tinha escrito "desloca `items` por `Pt(0.0, shift)`"; a derivação correcta (e os
próprios testes) exigem `Pt(0.0, -shift)` — verificado algebricamente (o midpoint pré-offset tem
de aterrar em `-axis_pt` sob a convenção `y` cresce para baixo) e confirmado pelos testes de
`cases`/`matrix` (falhavam com `+shift`, offset na direcção errada).

**Revisão do orquestrador** (caso composto, não coberto pelos testes dos dois agentes):
`$ x^2_i + (a/b) + sqrt(c) + mat(1,2;3,4) $` — sequência com attach, delimitado-com-fracção,
raiz e matriz, todos ao mesmo tempo. Confirmado por `mutool trace`: `x` e `c` (radicando)
partilham exactamente o mesmo Y mesmo dentro da sequência composta (root continua correcto em
contexto, não só isolado); barra da fracção e centro da matriz deslocados do eixo em direcções
correctas (antes seriam ~0). Re-derivação algébrica independente do sinal de `apply_axis_offset`
(secção separada, confirmando o trabalho do Agente B) — consistente.

**Suíte completa** (verificada independentemente pelo orquestrador, não só relatada pelo Agente
B): `cargo test --workspace` → core 4797, infra 743, shell 41, `0 failed`. `crystalline-lint .`:
0 drift novo (só o V7 pré-existente).

## Fase C — Benchmark (7 cenários, `hyperfine --warmup 5 -N -m 20`)

Binários `antes` (`56d85a6f5`) / `depois` (`cf7f57d1f`), release, mesma sessão, mesmos fixtures
reconstruídos de P918.

| Cenário | Razão (depois vs antes) |
|---|---|
| 01-hello | 1.01× |
| 02-lorem | 1.00× |
| 03-images | 1.00× |
| 04-math | 1.02× (depois mais rápido — ruído) |
| 05-tables | 1.02× (depois mais rápido — ruído, outlier reportado pelo hyperfine) |
| 06-long | 1.01× (depois mais rápido — ruído) |
| 07-context | 1.02× |

**Nenhuma regressão atribuível a P919** — todos os deltas na mesma banda 1.00–1.02× já
estabelecida em P909/P915/P917/P918.

---

## Resultado final

| Item | Estado |
|---|---|
| Causa confirmada por leitura do vanilla + medição, não suposição | ✅ |
| Fase A.1 — gate, 2 rondas de confirmação do dono, 6 L0s | ✅ |
| Protocolo de dois agentes (P898) seguido: Agente A testes, Agente B implementação | ✅ |
| Testes cobrindo 2 e 3+ elementos, `cases`/`matrix`, guardas de não-regressão para `root`/`delimited` | ✅ 7 testes |
| Revisão do orquestrador — caso composto (attach+delimited(frac)+sqrt+matrix simultâneos) | ✅ Geometricamente coerente, `mutool trace` |
| Recibo de posição Y antes/depois, comparado ao vanilla | ✅ (Fase A: 0.046pt→confirmado bug; Fase B: offsets não-zero nas direcções correctas) |
| Suíte completa verde, verificada independentemente | ✅ core 4797 (+7) · infra 743 · shell 41, `0 failed` |
| `crystalline-lint .` | ✅ 0 drift novo |
| Benchmark 7 cenários, atestado | ✅ Sem regressão, banda 1.00–1.02× |
| Commit | ✅ `cf7f57d1f` |

### Achados em aberto desta frente, não tocados por este passo

- Gap de empilhamento de `underover.rs` sem constante explícita de `MathConstants` (P906, ainda
  aberto).
- `fraction_denom_gap` sem consumidor confirmado em `frac.rs` (achado lateral de P918, não
  investigado).
- `assembly` não atinge a altura-alvo total em matrizes/casos de 6+ linhas (herdado de P917).
- Interacção `delimited(frac(...))`: a fórmula P912 de dimensionamento do delimitador
  (`(ascent-axis).max(descent+axis)`) agora recebe `body_box.ascent`/`descent` de `frac.rs` já
  correctamente axis-referenciados (antes recebia valores parcialmente ajustados pelo bug de
  P919) — mudança de input, não de fórmula; comportamento observado como coerente na revisão do
  orquestrador, não isolado num teste dedicado. Candidato a teste explícito num passo futuro se
  surgir um achado visual de tamanho de delimitador em `(frac(...))`.

### Proveniência

- Commit: `cf7f57d1f` (`git log -1`), working tree limpa nestes ficheiros no fim da sessão.
- Todos os números reproduzíveis via `cargo test --workspace`, `crystalline-lint .`, e o par de
  binários release `56d85a6f5`/`cf7f57d1f` (`cargo build --release -p typst-wiring` em cada) para
  o benchmark. Medições `mutool trace` reproduzíveis com os `.typ` de teste descritos nas secções
  Fase A/B acima (não persistidos em git — reconstruídos por sessão, mesma prática de P909+).
