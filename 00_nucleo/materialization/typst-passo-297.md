# Passo 297 — `MathUnderover` (P296.1)

**Frente**: `P296.1 — underover` (frente pendente P296 §8).
**Origem**: P296 fechou `MathAccent` + `MathCancel`; deixou
`underover` + `op` para sub-passos próprios.
**Pré-requisitos**: P296 — precedente arquitectural directo.
**Tipo declarado**: **extensão directa P296 com decisão A.2
genuinamente distinta** (campos opcionais vs obrigatórios).
**Marco**: continuação cluster math (3/4 features Tabela A.4 linha
119 após P297).

---

## §1 — Objectivo

Materializar `Content::MathUnderover` no `Content` enum, com
stdlib `native_underover(base, under?, over?)` e Layouter handler
dedicado `layout_underover`. Reclassificar Tabela A.4 linha 119
para `underover`: `parcial` (provavelmente `ausente` real) →
`implementado`.

### §1.1 — Sequência metodológica e expectativa

P296 inaugurou padrão **"cluster math handler dedicado"** (paradigma
N=9 cumulativo P288-P296). P297 reaplica:

- `Content::MathUnderover { ... }` (estrutura per A.2).
- `native_underover(base, under?, over?)` em stdlib.
- `layout_underover` handler em `rules/math/layout/mod.rs`.
- Emit agnóstico via `FrameItem` standard (ADR-0098 N=14
  esperado preservado).

**Diferença factual material vs P296** (anti-rubber-stamp):

| Aspecto | P296 (`accent`/`cancel`) | P297 (`underover`) |
|---|---|---|
| Estrutura vanilla | `AccentElem { base, accent }` / `CancelElem { body }` — **fields obrigatórios** | `UnderoverElem { base, under?, over? }` — **fields opcionais** |
| Campos required | 2 (accent) / 1 (cancel) | 1 (base) — under/over opcionais |
| Decisão A.2 | Trivial — paralelo `MathFrac` | **Genuína** — Option fields qualifica "variant rico" N=5 |
| Layout | Posicionamento estático vertical (accent acima) ou diagonal stroke (cancel) | Posicionamento condicional (under abaixo se presente; over acima se presente; ambos se ambos presentes) |

Esta diferença é **factual, não inflada** — `under`/`over`
opcionais não são cosméticos (são scope estrutural), portanto A.2
→ (b) qualifica genuinamente o padrão N=5 "variant rico com
cosméticos opcionais" se interpretado liberamente. **Mas P297
default sugere A.2 → (a) com required fields** (paralelo P156G
Block fields opcionais que **não** qualificaram) — verificar em
A.2.

### §1.2 — Vigilância anti-reflexão (P296 §9 + §6.6 P295)

P297 é **5ª reaplicação A.0.0** consecutiva (P293-P297). P296 §6.6
refutou hipótese degenerescência via magnitude média; P297 testa
empiricamente:

- **Cenário esperado**: A.0.0 modesta (similar a P295/P296 baixa)
  — variant ausente, paradigma idêntico P296. **Não é
  degenerescência** se reconhecido como **confirmação esperada
  do paradigma** (P296 §10 distinção entre rubber-stamp vs
  confirmação).
- **Cenário possível**: A.0.0 alta — se `underover` tem
  precedente parcial real em `MathSequence` heurístico (linha 418
  "layout aproximado"). Refutaria assunção HI' directa.
- **Decisão**: se A.0.0 P297 é **5ª consecutiva sem refutação
  significativa** (3 modestas + 2 altas), **inverter §6.6 P296**
  — desformalização §8.7' candidata revisitada.

Razão de ser:

1. **Cluster math 3/4 features fechadas** (Tabela A.4 linha 119).
2. **Decisão A.2 genuinamente distinta** vs P296 — campos
   opcionais qualificam ou não "variant rico" N=5.
3. **Teste empírico final do template §8.7'** — 5ª reaplicação.
4. **Reaplicação ADR-0098 + ADR-0099** — N=14 + N=13 cumulativos
   se hash preservado.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal estado actual (N=5 cumulativo §8.7')

Inspecção literal:

1. **`grep -rn "MathUnderover\|UnderoverElem" 01_core/`** —
   confirmar ausência ou presença.
2. **`grep -rn "native_underover\|under\|over" 01_core/src/rules/stdlib/`** —
   verificar stdlib actual.
3. **Inspeccionar `01_core/src/entities/content.rs`** — listar
   `Content::Math*` variants pós-P296 (esperado 12 incluindo
   `MathAccent`/`MathCancel` novos).
4. **Inspeccionar `01_core/src/rules/math/layout/mod.rs`** — ver
   se há tratamento heurístico actual de underover (linha 418
   sugere "layout aproximado").
5. **Inspeccionar `lab/typst-original/.../math/underover.rs`** —
   `UnderoverElem` estrutura literal:
   - Campos: `base`, `under: Option<Content>`, `over: Option<Content>`.
   - Campos cosméticos: `style: Smart`, `over_style`, etc.
6. **Cross-check com `\overbrace`/`\underbrace`/`\overline`/`\underline`**
   — vanilla typst expõe estes como aliases para `underover`?
   Verificar.

**Decisão A.0.0**:
- **Cenário "modesta esperada"** (HI' confirmado): variant ausente;
  stdlib ausente; nenhum tratamento heurístico significativo.
  P297 procede paralelo P296. **A.0.0 magnitude baixa**.
- **Cenário "alta inesperada"** (HIV'): "layout aproximado" linha
  418 revela código real — refactor não trivial. A.0.0 magnitude
  média/alta.
- **Cenário "nula"** (HII' improvável): variant existe não
  documentado. A.0.0 nula → **inverter §6.6 P296** se 5ª A.0.0
  consecutiva sem ganho significativo cumulativo (P295 baixa +
  P296 média + P297 nula → tendência mista).

**Critério estrito anti-rubber-stamp**:
- Se A.0.0 P297 magnitude baixa esperada (HI'), registar
  **honestamente como confirmação esperada do paradigma**, não
  como descoberta.
- Distinguir entre "confirmação esperada" (legítimo) e
  "rubber-stamp" (procedimento sem valor).
- Critério: se A.0.0 P297 produz output **estructuralmente
  idêntico** a P296 A.0.0 (substituição mecânica de nome),
  registar como **template valida sem nova descoberta** —
  não é degenerescência se reconhecido como tal.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "underover\|under_over" 03_infra/src/export.rs` | **Zero hits** — emit agnóstico via FrameItem | Inspecção literal |
| `FrameItem::Text/Line` suporta posicionamento under+over? | **Sim** — paralelo accent (P296) que empilha verticalmente | A.1.7 confirma |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 14º passo consecutivo | Verificação final |

### A.1 — Inventário literal

8 sub-secções (paralelo P296 §A.1):

1. **A.1.1 — `Content` Math* variants pós-P296** — confirmar
   12 variants (10 pré-P296 + MathAccent + MathCancel).
2. **A.1.2 — Tipos relacionados** — `MathBox`, helpers de
   posicionamento.
3. **A.1.3 — Match exaustivo sobre `Content::Math*`** — sítios
   onde adicionar `MathUnderover` arm.
4. **A.1.4 — Vanilla `UnderoverElem` estrutura literal**:
   - Campos required vs optional.
   - Atributos cosméticos vs estruturais.
5. **A.1.5 — Stdlib actual** — `native_underover` ausente
   esperado.
6. **A.1.6 — Layouter consumer** — `layout_node`/`layout_math`
   actual; precedente directo `layout_accent` P296.
7. **A.1.7 — Emit verification** — `FrameItem` standard.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente;
   provavelmente quase idêntico a P296 (registar como
   "confirmação esperada" se for o caso).

### A.2 — Estrutura do variant `MathUnderover` (decisão genuína)

**Decisão arquitectural genuína** vs P296 (que tinha decisão
trivial paralela `MathFrac`):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `MathUnderover { base, under, over }` com **all required** (`Content::Empty` como default semântico para "ausente") | Simétrico P296 `MathAccent`; sem `Option`; padrão "variant rico" N=4 preservado | **Não vanilla-fiel** — vanilla usa `Option`; precisa convenção `Content::Empty == None` |
| **(b)** `MathUnderover { base, under: Option<Box<Content>>, over: Option<Box<Content>> }` | Paridade vanilla; "ausente" explícito | **Qualifica "variant rico" N=5 candidato** — primeiros fields genuinamente Option na sequência P156G/H/I + P284 |
| **(c)** Duas variants separadas: `MathUnder { base, under }` + `MathOver { base, over }` + combinação composicional | Cada variant minimal e paralela P296 | Sobre-complica; perde semântica "ambos" |

Default sugerido: **(b)** se A.1.4 confirma vanilla `Option`
fields **estruturais** (não cosméticos). **(a)** se A.1.4 mostra
que `Content::Empty` é convenção cristalina já estabelecida em
outros variants. **(c)** rejeitada salvo se A.1.4 revelar
constraints inesperadas.

**Implicação gatilho "variant rico" N=5**:
- (a) preserva N=4 inalterado.
- (b) **qualifica genuinamente N=5** — primeira qualificação
  desde P287 refutação (P287 era `bool` flags; P296 era required;
  P297 (b) seria primeiro Option estrutural).
- (c) preserva N=4 mas é arquitecturalmente pior.

**Crítico**: se (b) escolhida, gatilho ADR meta "variant rico"
N=5 **dispara genuinamente**. Decisão sobre promoção em §3 ponto
6 condicional.

### A.3 — Integração com layout_math handlers (paralelo P296)

| Opção | Comportamento |
|---|---|
| **(α)** Handler dedicado `layout_underover` paralelo `layout_accent`/`layout_cancel` P296 | Paradigma cluster math handler dedicado N=10 cumulativo |
| **(β)** Reusa `layout_accent` para over; novo `layout_under` simétrico; composição interna | Reuso máximo |
| **(γ)** Inline em `layout_node` match arm | Sem handler dedicado |

Default sugerido: **(α)** — confirma paradigma P296. **(β)**
considerar se A.1.4 mostra que vanilla typst usa composição
literal accent+algo. **(γ)** rejeitada (perdia handler isolamento).

### A.4 — Impacto em emit (paralelo P296)

| Opção | Mecanismo | Implicação |
|---|---|---|
| **(i)** `FrameItem::Text/Glyph` agnóstico (paralelo P296) | Hash preserved 14º passo |
| **(ii)** Caso edge inesperado | Improvável dado P296 paralelo |

Default sugerido: **(i)** quase certo.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- `under` ausente, `over` presente — comportamento equivalente a
  accent? (Verificar paridade A.1.4).
- `under` presente, `over` ausente — under simétrico.
- Ambos ausentes — degenerate: comportamento vanilla?
- Ambos presentes — empilhamento triplo (under abaixo, base meio,
  over acima).
- `under`/`over` com glyph que ultrapassa width da base — clipping
  vs expansão.
- `MathUnderover` aninhado dentro de `MathAccent` — composição.

### A.5' — Verificação anti-reflexão (N=6 do padrão §8.6)

**Crítica especial 5ª reaplicação A.0.0 consecutiva**.

4 verificações:

1. **Comparação literal A.1.6 P288-P297** — paradigma novo?
   - P293-P296 nove paradigmas distintos identificados.
   - **P297**: se HI' confirmado, paradigma **idêntico P296
     "math layout handler dedicado"** — N=10 do padrão (não
     novo). Registar como **N=10 do padrão "cluster math handler
     dedicado"** inaugurado em P296 (gatilho cumulativo distinto
     de §8.6 anti-reflexão).
2. **A.0 produzido empiricamente** — A.0.0 P297 magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **A.2 → (b) qualifica "variant rico" N=5** se escolhida —
     primeira qualificação genuína. Mesmo se A.2 → (a), a
     **decisão A.2** é estructuralmente nova (P296 não tinha
     decisão genuína; P297 tem).
   - **"Confirmação esperada" como categoria distinta de
     "descoberta"** — paradigma metodológico novo que pode
     desbloqueir desformalização ordenada do template §8.7'.
4. **Decisão sobre promoção ADR meta**:
   - **§8.7' N=5** se A.0.0 magnitude **média ou alta** —
     primeira promoção robusta possível (P294 e P296 já adiaram).
   - **§8.7' N=5 adiado** se A.0.0 magnitude **baixa/nula** —
     reconhece "confirmação esperada" como legítima sem
     promoção.
   - **§8.3 N=9** candidato adiado mesma razão.
   - **"variant rico" N=5** se A.2 → (b) — **promoção independente
     dos outros gatilhos**.
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério estrito anti-reflexão pós-P296**:

P296 §6.6 revisado: *"§8.7' template valida com refutação real
genuína"*. P297 testa se essa validação **persiste** ou se P296
foi excepção. Critério:

- Se A.0.0 P297 **modesta-esperada** (confirmação paradigma):
  registar honestamente; template continua viável como
  ferramenta de **verificação empírica**, mesmo sem descoberta
  nova.
- Se A.0.0 P297 **nula** ou rubber-stamp: 4ª A.0.0 sem ganho real
  em 5 reaplicações → **desformalização candidata**.
- Se A.0.0 P297 **significativa**: §8.7' robustez confirmada
  cumulativamente; **N=5 promoção candidata genuína**.

---

## §3 — Materialização

Após Fase A (com decisões A.2 e A.3):

**Cenário default (HI' + A.2 → (a) all required + A.3 → α handler
dedicado)**:

1. Adicionar `MathUnderover { base: Box<Content>, under: Box<Content>,
   over: Box<Content> }` em `entities/content.rs`.
2. Match arms exhaustive (paralelo P296 9 sítios).
3. Stdlib `native_underover` em `structural.rs` com 3 posicionais.
4. Handler `layout_underover` em `rules/math/layout/mod.rs`:
   - Calcular bbox under, over, base.
   - Empilhar verticalmente: over (top) + base (middle) + under
     (bottom).
   - Centrar horizontalmente cada por width-máximo.
5. Registar em `eval/mod.rs`.
6. Testes ~10-15.

**Cenário alternativo (A.2 → (b) Option fields)**:

1. Variant com `Option<Box<Content>>` fields.
2. Handler conditional layout (skip under/over se None).
3. Stdlib `native_underover(base, under: ?, over: ?)` com named
   args opcionais.
4. **Promoção candidata "variant rico" N=5** em §3 ponto 7
   condicional.

**Cenário alternativo (HIV' "layout aproximado" descoberto)**:

1. Refactor não-trivial — abrir sub-passo P297.0 se complexidade
   > XS+S.

Aplicação ADR-0098 obrigatória — A.0 documenta inspecção emit.

Promoção ADR meta condicional per A.5':
- Default: **sem promoção**.
- "variant rico" N=5 se A.2 → (b) com Option estrutural genuíno.
- §8.7' N=5 só se A.0.0 magnitude média/alta.

Actualizar L0:
- `entities/content.md` (+1 variant `MathUnderover`).
- Tabela A.4 linha 119 — `underover` `parcial` → `implementado`.
- Tabela B — +1 variant.
- Propagar hashes.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-15.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P296: 2 830 testes.
  Esperado: ~2 838-2 845.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` muda (+1 variant).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `export.rs` **preserved bit-exact** pelo **14º passo
  consecutivo** se A.4 → (i).
- **Regressão bit-exact** para math features pre-P297
  (frac/attach/root/lr/matrix/cases/accent/cancel inalteradas).
- Tabela A.4 linha 119 (`underover`) reclassificada.
- Tabela B Math variants — +1 entrada.
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **A.0.0 magnitude registada honestamente** — esperado baixa
  (confirmação) ou média (refutação inesperada).
- **Promoção ADR meta condicional**:
  - **"variant rico" N=5** se A.2 → (b) genuinamente.
  - **§8.7' N=5** apenas se A.0.0 magnitude média/alta.
  - **Uma ADR meta por passo** (P273.17 §0).
- "Confirmação esperada" registada como categoria honesta se
  aplicável.

---

## §5 — Não-objectivos

- **Não** materializar `op` (Tabela A.4 linha 119; vanilla
  `OpElem`). P296.2 candidato dedicado.
- **Não** materializar `\overbrace`/`\underbrace`/`\overline`/
  `\underline` como aliases stdlib se vanilla os tem. Frentes
  dedicadas (depende A.1.6).
- **Não** materializar atributos cosméticos vanilla
  (`style: Smart`, etc.) se A.2 → (a)/(b) minimal. ADR-0054
  graded.
- **Não** promover ADR meta **sem gatilho genuíno disparar**.
  Critério estrito anti-padrão.
- **Não** confundir P297 com P296 — A.2 decisão genuína distingue;
  paradigma layout idêntico mas estrutura variant diferente.
- **Não** considerar P297 como "rubber-stamp" automaticamente —
  "confirmação esperada" é categoria distinta legítima.
- **Não** materializar `inverted`/`cross` toggles cancel (P296.X
  candidato).

---

## §6 — Pendências relacionadas

Resolve (condicional):
- Tabela A.4 linha 119 — `underover` parcial → implementado.

Não resolve (continua aberto):
- `op` (P296.2 candidato — extensão directa P297).
- Toggles funcionais cancel (`inverted`/`cross`) — P296.X.
- Cosméticos accent (`size`/`dotless`) + cancel
  (`length`/`angle`/`stroke`) — ADR-0054 graded.
- Underover style customisation — cosmético.
- Math shaping completo — ADR-0054 perfil graded.

---

## §7 — Risco residual

Risco principal: **A.0.0 P297 modesta esperada confunde-se com
rubber-stamp**. Mitigação: §A.5' critério estrito —
"confirmação esperada" categoria honesta; **distingue-se**
empíricamente de rubber-stamp por:
- Inspecção literal genuína (não copy-paste documental).
- Decisão A.2 não-trivial registada (campos opcionais vs
  obrigatórios).
- Output diagnóstico **não idêntico** a P296 (campos diferentes).

Risco secundário: **A.2 → (b) qualifica "variant rico" N=5 sem
gatilho legítimo**. Mitigação: §A.2 estrito — A.1.4 confirma se
vanilla `Option` é estrutural ou cosmético; só qualifica se
estrutural genuíno.

Risco terciário: **HIV' "layout aproximado" descoberto** força
refactor maior que XS+S. Mitigação: A.0.0 interrupção honesta
permitida; abrir P297.0 dedicado.

Risco quaternário: **promoção ADR meta múltipla** se "variant
rico" N=5 e §8.7' N=5 disparam simultaneamente. Mitigação:
P273.17 §0 — uma por passo; preferir "variant rico" (mais
directo) se A.2 → (b) genuíno; preferir §8.7' (cumulativa) se
A.0.0 significativa.

Risco quinário: **5ª A.0.0 consecutiva** torna template ritual.
Mitigação: §A.5' critério "confirmação esperada" desbloqueia
**desformalização ordenada** sem necessidade de degenerescência
factual — template pode ser **opcionalmente aplicado** pós-P297.

Risco senário: **inconsistência com P296 cancel handler**. P296
`layout_cancel` emite `FrameItem::Line` directo; P297
`layout_underover` provavelmente reusa `layout_node` recursivo.
Mitigação: A.3 explicita estratégia; documentar consistência.

Risco septenário: **regressão bit-exact em accent/cancel
P296**. Mitigação: testes regression obrigatórios.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/content.rs`
  (`Content::Math*` variants; 12 pós-P296).
- Função stdlib: `01_core/src/rules/stdlib/structural.rs`
  (paralelo `native_accent`/`native_cancel` P296).
- Layouter consumer: `01_core/src/rules/math/layout/mod.rs`
  (`layout_node` arm + `layout_underover` handler).
- Vanilla: `lab/typst-original/crates/typst-library/src/math/underover.rs`.
- Precedente arquitectural directo: **P296** (`MathAccent` +
  `MathCancel`).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded (justifica simplifications
  cosméticas).
- Padrão §8.7' A.0.0 template (N=5 reaplicação; **teste final de
  robustez ou inauguração "confirmação esperada"**).
- Padrão §8.3 refutação pragmática (N=9 candidato adiado).
- Padrão "cluster math handler dedicado" (N=2 cumulativo
  P296+P297).
- Padrão "variant rico" N=4 (P156G/H/I + P284); **N=5 candidato
  se A.2 → (b)**.

---

*Spec P297 produzida 2026-05-19 pós-P296 (math accent+cancel
fechado; A.0.0 N=4 magnitude média refutou hipótese
degenerescência). Frente `P296.1 — underover` — extensão directa
P296 mas **decisão A.2 genuinamente distinta**: campos opcionais
vanilla (`under?`/`over?`) vs obrigatórios P296 (accent/body).
Magnitude XS+S; cluster math 3/4 após P297. Fase A obrigatória
com **7 secções** A.0.0+A.0-A.5+A.5'. **A.0.0 P297 é 5ª
reaplicação consecutiva** — teste empírico final da robustez do
template §8.7'. **"Confirmação esperada" inaugurada como
categoria honesta** distinta de "descoberta" — desbloqueia
desformalização ordenada do template se sucessivas reaplicações
forem todas-modestas-confirmações. **A.2 → (b) `Option` fields
qualifica "variant rico" N=5 candidato genuíno** se A.1.4 confirma
estrutural vanilla — primeira qualificação possível desde P287
refutação. **Uma ADR meta por passo no máximo** (P273.17 §0):
"variant rico" N=5 e §8.7' N=5 não disparam simultaneamente.
Honestidade epistémica reforçada: P297 **explicitamente aceita**
que pode ser "confirmação" sem ser "descoberta" — registar
honestamente em vez de inflar magnitude artificialmente. Sem
caps LOC ou magnitude (P282 §7).*
