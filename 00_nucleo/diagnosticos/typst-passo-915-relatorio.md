# Relatório P915 — "cramped" em modo matemático (adiado de P914)

**Commit de referência (working tree no início e no fim desta sessão):** `0e98008a2` (working
tree modificada, não commitada — ver `git diff --stat` na secção final; nenhum commit feito
durante este passo).
**Baseline de testes (antes de qualquer alteração):** typst-core: 4786 · typst-infra: 743 ·
typst-shell: 41 (herdado de P917, já commitado em `0e98008a2`).
**Baseline de testes (fim deste passo):** typst-core: 4790 · typst-infra: 743 · typst-shell: 41
— todos `0 failed`.
**Data:** 2026-07-26.

---

## Fase A — o que o vanilla de facto implementa (per ADR-0123, não presumido)

Localizados **todos** os pontos onde o vanilla define/propaga `cramped`, por leitura directa
(`grep -rni cramped lab/typst-original/crates/`, cobertura exaustiva confirmada — toda a lista
abaixo, nada mais):

**Onde `cramped=true` é aplicado:**

| Ponto | `file:line` | Nota |
|---|---|---|
| Subscrito | `style.rs:333`, `style_for_subscript` | `[style_for_superscript, style_cramped()]` — **superscrito nunca é forçado** |
| Denominador | `style.rs:362`, `style_for_denominator` | `[style_for_numerator, style_cramped()]` — **numerador nunca é forçado** |
| Base de accent | `resolve.rs:373`, `resolve_accent` | só quando `position == Above` (accent "abaixo" não é cramped) |
| Radicando | `resolve.rs:1229-1231`, `resolve_root` | sempre cramped |
| Índice de raiz | `resolve.rs:1233-1240`, `resolve_root` | cramped **e** scriptscript, os dois juntos |
| Base de `overline()` | `resolve.rs:1270`, `resolve_overline` | `underline()` **não** é cramped |

**Onde é consumido:** `compute_script_shifts` (`scripts.rs:318-382`) — `cramped` substitui **só**
o termo `superscript_shift_up` por `superscript_shift_up_cramped`, quando há superscrito
presente (`tl`/`tr`). Confirmado por leitura literal: nenhum outro termo da fórmula
(`shift_down`, `sup_bottom_min`, `sup_drop_max`, `gap_min`, etc.) é afectado por `cramped`.

**Achado de âmbito, não corrigido**: o cristalino não tem `overline()`/`underline()` como
construção matemática — esses nomes resolvem sempre para decoração de texto
(`DecoKind::Overline`/`Underline`, `stdlib/text.rs`), nunca para `Content::MathUnderover`. Não
há call site de "overline" a tocar — confirmado por leitura de `eval/mod.rs` (`scope.define`
único, global, sem dispatch math-específico).

## Fase A.1 — decisão de arquitectura (gate cumprido, confirmado pelo dono)

**Decisão**: campo novo `cramped: bool` em `TextStyle`, mesmo padrão exacto de `math_script`
(P891) — default `false` via `#[derive(Default)]`, propagado por `..style.clone()`.

**4 call sites de propagação** (código actual, não vanilla):

1. `attach.rs` — `script_style` partilhado dividido em `top_style` (tl/sup, herda `cramped`) e
   `bottom_style` (bl/sub, força `true`); `compute_script_shifts` lê `style.cramped` do estilo
   **ambiente** (não de `top_style`/`bottom_style`).
2. `frac.rs` — `sub_style` partilhado dividido em `num_style` (herda) e `den_style` (força `true`).
3. `root.rs` — radicando (`style` usado directo) e índice (`script_style`) passam a `cramped: true`.
4. `accent.rs` — base (`style` usado directo) passa a `cramped: true`, **incondicional** — o
   cristalino só implementa accent "acima" (`MathAccentElem` sem campo de posição), logo a
   condição do vanilla é trivialmente sempre verdadeira aqui.

**Constante nova**: `MathConstants::superscript_shift_up_cramped` (15º campo) — método já existia
em `ttf_parser` 0.25, só nunca tinha sido lido. Fallback: mesmo valor de `superscript_shift_up`
(362.0) — não foi possível confirmar uma fonte STIX Two Math cujos 13 campos batessem com o
fallback actual do cristalino (testada `typst-dev-assets`; `axis_height` mediu 258 vs `500`
hardcoded — divergência que já não bate certo, não só neste campo), e o fallback só é exercitado
sem fonte real, onde "sem efeito de cramped" é o mais defensável.

**7 L0s escritos e hash sincronizado ANTES do código** (`crystalline-lint --fix-hashes .`, 7
ficheiros `.rs`), per Trava Arquitectural: `entities/layout_types.md`, `entities/math_constants.md`,
`infra/font_metrics.md`, `engine/math/layout/attach.md`, `frac.md`, `root.md`, `accent.md`.
Decisão apresentada ao dono com os call sites já mapeados; aprovada antes de qualquer `.rs` tocado.

## Fase B — implementação (TDD, fonte real desde o primeiro teste)

**Testes novos** (`01_core/src/engine/math/layout/tests.rs`), ground-truth sintético mas
extremo (300 vs 700du) para detectar regressão alta e clara, mais teste de fonte real:

- `p915_attach_superscript_usa_shift_cramped_quando_estilo_ambiente_e_cramped` — mesma base+sup,
  `style.cramped` diferente → `shift_up` diferente. Vermelho antes (`y_normal=y_cramped=0`,
  `compute_script_shifts` não lia `cramped`), verde depois.
- `p915_attach_subscript_nao_afectado_por_cramped_ambiente` — confirma que `shift_down` **não**
  muda com `style.cramped` (achado do vanilla: só `shift_up` é afectado).
- `p915_frac_denominador_e_cramped_numerador_nao` — mesmo `x^2` dentro do numerador vs.
  denominador de uma fracção → geometria diferente.
- `p915_cramped_e_sup_sub_simultaneos_interagem_sem_cancelar_piso_cramped` — **revisão do
  orquestrador**, caso composto não coberto pelos testes acima: cramped **e** ajuste de gap
  simultâneo de P914 ao mesmo tempo — confirma que o piso cramped sobrevive como mínimo, o
  ajuste de P914 só soma, nunca cancela o piso.
- `p914_fallback_font_metrics_math_constants_le_tabela_math_real` (extensão) — `superscript_
  shift_up_cramped` extraído da tabela MATH real bate byte-a-byte com `ttf_parser` directo
  (mesma fonte `NewCMMath-Regular.otf`, mesmo `rev` pinado, confirmado por P917 — não a fixture
  `Book`, já confundida uma vez nessa frente).

**Confirmação visual/geométrica** (`frac(a^2, b^2)`, 20pt, `mutool trace`, fonte real): medi as
posições reais dos glifos "2" (sobrescrito) em relação às suas bases "a"/"b" no PDF exportado.
Razão denominador/numerador medida no PDF: `4.046/5.082 = 0.7961`. Razão esperada a partir da
tabela MATH real (`superscript_shift_up_cramped/superscript_shift_up` = `289/363 = 0.7961`,
`NewCMMath-Regular.otf`, medido via `fontTools`) — **bate a 4 algarismos significativos**,
confirmando o mecanismo completo (fonte real → `math_constants()` → `compute_script_shifts` →
posição no PDF) end-to-end, não só a unidade isolada.

**Suíte completa verde**: typst-core 4790 (+4), typst-infra 743 (extensão de teste existente, sem
net-new), typst-shell 41 — `0 failed`.

**`crystalline-lint .`**: 0 novo (só o V7 pré-existente, não relacionado).

## Fase C — regressão

7 cenários (reconstruídos, mesma prática de P917 — os ficheiros canónicos não persistem no
repositório), `hyperfine --warmup 5 -N -m 20`, binário release antes (`typst_after2`, P893 sem
P915) vs. depois (`typst_p915`):

| Cenário | Razão (depois vs antes) |
|---|---|
| 01-hello | 1.00× |
| 02-lorem | 1.00× |
| 03-images | 1.00× |
| 04-math | 1.08±0.22× (outlier — remedido com `--warmup 10 -m 30`: **1.01×**, dentro do ruído) |
| 05-tables | 1.00× |
| 06-long | 1.00× |
| 07-context | 1.00× |

**Nenhuma regressão** — `04-math` teve uma leitura inicial ruidosa (σ=27.8ms num run de 20),
remedida com mais warmup/runs para confirmar ruído, não efeito real.

---

## Resultado final

| Item | Estado |
|---|---|
| Fase A — pontos de propagação do vanilla, `file:line` dos dois lados | ✅ Confirmados por leitura, não presumidos |
| Fase A.1 — decisão de arquitectura (campo `cramped` em `TextStyle`) | ✅ Aprovada pelo dono antes da Fase B |
| 7 L0s escritos e hash sincronizado antes do código | ✅ |
| `superscript_shift_up_cramped` — campo novo em `MathConstants`, extraído da fonte real | ✅ |
| 4 call sites de propagação (attach/frac/root/accent) | ✅ Implementados |
| Testes com fonte real desde o início (lição de P912/913/917) | ✅ |
| Caso composto (revisão do orquestrador: cramped + gap simultâneo de P914) | ✅ Testado, sem cancelamento |
| Confirmação visual/geométrica contra a fórmula real do vanilla | ✅ Razão medida bate a 4 sig. figs. com a fonte real |
| Suíte completa verde | ✅ core 4790 · infra 743 · shell 41, `0 failed` |
| `crystalline-lint .` | ✅ 0 novo |
| Benchmark 7 cenários | ✅ Sem regressão |
| Commit | ❌ Não commitado — decisão do dono |

### Achados em aberto desta frente, ainda não tocados (herdados de P917, não deste passo)

- Desalinhamento vertical entre elementos adjacentes numa sequência matemática.
- `assembly` não atinge a altura-alvo total em matrizes/casos de 6+ linhas.
- Reconciliação de numeração de ADRs (P910) — ainda sem confirmação contra o repositório real.
- Se o cristalino vier a suportar accent "abaixo": a condicional do vanilla (`position == Above`)
  terá de ser reintroduzida em `accent.rs` — não codificada agora por não haver caso que a exercite.

### Proveniência

- Commit base: `0e98008a2` (`git log -1`), working tree modificada (17 ficheiros, `git diff
  --stat`: `447 insertions(+), 27 deletions(-)`), nenhum commit feito nesta sessão.
- Fonte usada para ground truth: `NewCMMath-Regular.otf` embutida via `typst_assets`
  (`~/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts/`), mesmo `rev`
  pinado em `Cargo.toml` — confirmado, não a fixture `03_infra/fixtures/fonts/NewCMMath-Book.otf`.
- Todos os números reproduzíveis via `cargo test --workspace` e `hyperfine --warmup 5 -N -m 20`
  sobre binários release da working tree actual vs. `git stash` (estado pré-P915).
