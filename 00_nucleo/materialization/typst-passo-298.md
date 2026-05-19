# Passo 298 — `MathOp` (P296.2 — fecho cluster math)

**Frente**: `P296.2 — op` (frente pendente P296 §8 + P297 §8).
**Origem**: P296 fechou accent+cancel; P297 fechou underover.
Resta `op` para fechar cluster math 4/4 (Tabela A.4 linha 119).
**Pré-requisitos**: P296 + P297 — precedentes arquitecturais
directos.
**Tipo declarado**: **3ª aplicação do paradigma "cluster math
handler dedicado"** (N=3 cumulativo se A.3 → α confirma).
**Magnitude**: XS-S esperado, mas A.0.0 pode revelar diferença
factual material (paralelo P297 vs P296).
**Marco**: fecho cluster math 4/4 (accent + cancel + underover +
op).

---

## §1 — Objectivo

Materializar `Content::MathOp` no `Content` enum com stdlib
`native_op(text, limits: ?)` e Layouter handler dedicado
`layout_op` em `rules/math/layout/mod.rs`. Reclassificar Tabela
A.4 linha 119 — `op` permanece `parcial` pós-P297; objectivo é
`implementado`.

### §1.1 — Sequência e expectativas factuais

P296 inaugurou "cluster math handler dedicado" N=1; P297
consolidou N=2; P298 (este) atinge **N=3 cumulativo** se A.3 →
(α) confirma. **Limiar tentativo para promoção sub-padrão**
candidata.

### §1.2 — Diferença factual material vs P296/P297

**`op` é tipologicamente diferente** de accent/cancel/underover:

| Aspecto | P296 accent/cancel | P297 underover | **P298 op** |
|---|---|---|---|
| Estrutura content | `Box<Content>` fields | `Box<Content>` + Options | **`EcoString` text + `bool` flag** |
| Semântica | Wrapper visual | Wrapper composicional | **Anotação tipológica** sobre identificador |
| Interaction com attach | Independente | Independente | **Modifica comportamento attach** (limits vs scripts) |
| Layout | Posicionamento estático | Empilhamento condicional | **Sem layout dedicado** — afecta layout de attach posterior |

Esta diferença **antecipa A.0.0 magnitude potencialmente alta**:
- Se cristalino trata `lim`/`sin`/`cos` via `MathIdent` simples
  (P50 Unicode mapping), `op` introduz mecanismo novo não
  existente.
- Se cristalino tem mecanismo limits-aware já parcial (linha 418
  "layout aproximado"), pode ser HIV''/HV'' (refactor).
- Se cristalino implementa `op` heuristicamente via attach com
  detecção de identifiers (e.g. lookup "lim" → limits-style), é
  HIII''/HV''.

**Diferença com P296.1 (P297)**: P297 também antecipou diferença
estrutural genuína (Option fields) e descobriu refutação alta
(vanilla fragmentação 12-elements). P298 antecipa diferença
estrutural genuína (text+bool vs body); **A.0.0 magnitude
esperada é média-alta**, não baixa.

### §1.3 — Razões

1. **Fecho cluster math 4/4** — Tabela A.4 linha 119 4/4
   features implementadas.
2. **Sub-padrão "cluster math handler dedicado" N=3 cumulativo
   candidato** — limiar tentativo para promoção ADR meta
   independente de §8.7'/§8.3/"variant rico".
3. **Diferença estrutural genuína** — A.2 decisão não-trivial
   (text+bool vs body); paralelo conceptual a P297 mas com
   tipos diferentes.
4. **Reaplicação ADR-0098 + ADR-0099** — N=15 + N=14 cumulativos
   se hash preservado.
5. **§8.7' template testado pela 6ª vez** — magnitudes
   acumuladas P293-P298 confirmam ou refutam tendência
   não-decrescente.

### §1.4 — Disparo simultâneo P297: o desbloqueio

P297 §6.4 + §10 registou disparo simultâneo §8.7' N=5 + "variant
rico" N=5 — ambos adiados per P273.17 §0. P298 tem 4 candidatos
possíveis para promoção:

| Padrão | N pós-P298 | Condição |
|---|---|---|
| §8.7' A.0.0 template | N=6 | Se A.0.0 magnitude média/alta |
| §8.3 refutação pragmática | N=10 candidato | Adiado P294-P297 |
| **Sub-padrão "cluster math handler"** | **N=3 candidato** | **Se A.3 → α e fecho cluster genuíno** |
| "Variant rico" | N=5 inalterado | **NÃO qualifica P298** — A.2 antecipada com `bool` toggle (não `Option<Box<Content>>` estrutural) |

**Sub-padrão "cluster math handler dedicado" é candidato
preferencial** porque:
- Surge **organicamente** da sequência P296+P297+P298.
- É **arquitecturalmente concreto** (handler funcs em mesma
  região de código).
- **Não disputa** com §8.7'/§8.3 (gatilho independente).
- Tem **3 aplicações cumulativas** (limiar tentativo N≥3).

§5 explicita prioridade: se sub-padrão N=3 dispara genuíno,
preferir essa promoção sobre §8.7'/§8.3 adiados.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal estado actual `op` (N=6 cumulativo §8.7')

Inspecção literal:

1. **`grep -rn "MathOp\|OpElem" 01_core/`** — confirmar ausência
   ou presença.
2. **`grep -rn "native_op\|\"op\"\b" 01_core/src/rules/stdlib/`** —
   verificar stdlib actual.
3. **Inspeccionar `01_core/src/entities/content.rs`** — listar
   `Content::Math*` variants pós-P297 (esperado 13 incluindo
   `MathAccent`/`MathCancel`/`MathUnderover`).
4. **Inspeccionar `01_core/src/rules/math/layout/mod.rs`** — ver
   se há tratamento heurístico actual de operadores; em
   particular, como `MathAttach` decide entre limits-style vs
   scripts-style.
5. **Inspeccionar `lab/typst-original/.../math/op.rs`** —
   `OpElem` estrutura literal:
   - Campos: `text: EcoString`, `limits: bool`.
   - Atributos extra?
6. **Inspeccionar lista de operadores vanilla pre-definidos** —
   `lab/typst-original/.../math/op.rs` define `lim`/`sin`/`cos`/etc.
   como instâncias `OpElem` no scope math.
7. **Cross-check com `MathIdent` actual** — como cristalino trata
   `lim`/`sin` hoje (P50 simbólicos)?
8. **Cross-check com `MathAttach`** — como `^`/`_` actualmente
   interagem com identifiers; existe path limits-style?

**Decisão A.0.0 sobre hipóteses HI''-HV''**:

| Hipótese | Decisão se confirmada |
|---|---|
| **HI''** (variant ausente; stdlib ausente; sem heurística) | P298 procede com materialização from-scratch (paralelo P296 HIV) |
| **HII''** (variant existe não-documentado) | Improvável; A.0.0 magnitude baixa |
| **HIII''** (stdlib parcial sem variant) | Refactor stdlib + variant novo |
| **HIV''** (vanilla é wrapper sobre attach) | Verificar A.1.4 conceptualmente; decidir scope |
| **HV''** (heurística `MathAttach` existe parcialmente para limits) | A.0.0 magnitude alta — refactor + variant novo |

**Default sugerido**: HI'' provável (paralelo P296 HIV + P297
HV'.a indicam consistentemente "variant ausente"); mas se A.0.0
revelar HV'' (heurística limits-style já no `MathAttach`),
abrir sub-passo P298.0 para isolar o refactor.

**Vigilância anti-reflexão**: P298 é 6ª A.0.0 consecutiva. P297
§10 inverteu hipótese degenerescência (magnitude alta valida).
P298 testa **se tendência não-decrescente persiste** ou se
sequência cumulativa volta a magnitudes modestas.

Se P298 A.0.0 magnitude **baixa-modesta** após P297 alta:
- Cluster math reaplicação "confirmação esperada" legítima
  (P297 inaugurou categoria).
- **Não é degenerescência** — sub-padrão "cluster math handler"
  N=3 valida o paradigma sem necessidade de descoberta nova.
- §8.7' template robusto: magnitude **variada** é saudável.

Se P298 A.0.0 magnitude **alta** após P297 alta:
- Tendência não-decrescente fortemente confirmada.
- §8.7' N=6 promoção candidata muito robusta.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "MathOp\|op_text\|op_limits" 03_infra/src/export.rs` | **Zero hits** — emit agnóstico | Inspecção literal |
| `FrameItem::Text` suficiente para emit `op` text? | **Sim** — text com font math (paralelo `MathIdent`) | A.1.7 confirma |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 15º passo consecutivo | Verificação final |

### A.1 — Inventário literal

8 sub-secções (paralelo P297 §A.1):

1. **A.1.1 — `Content` Math* variants pós-P297** — confirmar 13
   variants.
2. **A.1.2 — Tipos relacionados** — `EcoString` em outros
   variants math (`MathIdent`, `MathText`); paradigma string-based.
3. **A.1.3 — Match exaustivo sobre `Content::Math*`** — sítios
   onde adicionar `MathOp` arm.
4. **A.1.4 — Vanilla `OpElem` estrutura literal** — campos +
   defaults + interaction com attach.
5. **A.1.5 — Lista operadores vanilla** — `lim`/`sin`/`cos`/etc.
   pré-definidos no scope math.
6. **A.1.6 — `MathAttach` actual** — como cristalino decide
   limits-style vs scripts-style? Existe lookup, heurística, ou
   sempre scripts?
7. **A.1.7 — Emit verification** — `FrameItem::Text` para op
   text.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente
   conforme hipótese.

**Crítico A.1.6**: se cristalino sempre usa scripts-style (P50
heurístico simples), P298 muda comportamento de `^`/`_` quando
applied a `op` — **mudança semântica visível user-facing**.
Testes regression obrigatórios.

### A.2 — Estrutura do variant `MathOp`

**Decisão arquitectural** (per ADR-0065 critério #1):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `MathOp { text: EcoString }` minimal sem `limits` | Mínimo; sem flag estrutural | **Perde semântica vanilla** — `limits` é o que distingue `lim` de `sin` |
| **(b)** `MathOp { text: EcoString, limits: bool }` paralelo vanilla | Paridade vanilla; flag estrutural genuíno | **Qualifica "variant rico" N=5 candidato genuíno** se `limits` for considerado estrutural |
| **(c)** Duas variants separadas: `MathOpLimits { text }` + `MathOpScripts { text }` | Cada minimal; sem `bool` | Sobre-engenharia; perde elegância |

Default sugerido: **(b)** — `limits: bool` é flag **estrutural**
genuíno (afecta layout fundamental do attach), não cosmético.
**(a)** rejeitada — perde semântica essencial vanilla. **(c)**
rejeitada — sobre-complica.

**Implicação gatilho "variant rico" N=5**:

P297 §6.4 estabeleceu que `Option<Box<Content>>` qualifica como
fields estruturais (não cosméticos). **`bool` toggle** é caso
diferente:
- P156G Block (bool defaults) — refutado P287 (cosméticos).
- P284 Underline (Option<Color>) — refutado P287 (cosméticos).
- P297 MathUnderover (Option<Box<Content>>) — **qualifica N=5
  genuíno** (estruturais).
- **P298 MathOp (`bool limits`)** — caso intermédio:
  - **Afecta layout fundamental** (não cosmético).
  - **Mas não é Option estrutural** (sempre presente; é
    discriminador).
  - **Pode qualificar N=5** se interpretado como "discriminador
    estrutural"; ou **pode não qualificar** se restrito a Options.

**Decisão sobre qualificação**: registar em A.5' honestamente.
Se ambíguo, **adiar promoção** (preferir conservador) — não
diluir o gatilho com casos limítrofes (anti-padrão
over-formalização P273.17 §0).

### A.3 — Integração com layout_math handlers

| Opção | Comportamento |
|---|---|
| **(α)** Handler dedicado `layout_op` paralelo P296/P297 | Confirma sub-padrão N=3 — **promoção candidata** |
| **(β)** Inline em `layout_node` match arm | Sem handler dedicado; rejeita sub-padrão |
| **(γ)** Reuso de `layout_node` text path (paralelo `MathIdent`) sem handler novo | Compromisso |

Default sugerido: **(α)** — confirma sub-padrão. **(γ)**
considerar se A.1.6 mostra que op é apenas annotation sobre
identifier (não-trivial layout).

### A.4 — Interaction com `MathAttach` (CRÍTICO)

**Decisão estrutural distintiva P298**: `MathAttach` precisa de
ler `limits` flag do `MathOp` base.

| Opção | Mecanismo |
|---|---|
| **(i)** `MathAttach` arm verifica se `base` é `Content::MathOp { limits: true, .. }` e renderiza limits-style; senão scripts-style | Paradigma single source of truth — flag lido em runtime |
| **(ii)** `MathOp` produz internamente um wrapper que `MathAttach` reconhece | Indirecção |
| **(iii)** Reescrever `MathAttach` para aceitar discriminador limits-style explícito | Refactor MathAttach |

Default sugerido: **(i)** se A.1.6 mostra `MathAttach` actualmente
single-style (P50 simples). **(iii)** rejeitada salvo se A.1.6
revelar acoplamento complexo.

**Crítico**: este passo é distintivo da sequência P296/P297 —
ambas tinham handlers **independentes**. P298 introduz
**interaction cross-variant** (`MathOp` afecta layout de
`MathAttach`). Paradigma **genuinamente novo** dentro do cluster
math.

### A.4-bis — Impacto em emit

Standard: emit agnóstico via `FrameItem::Text` (paralelo P296/P297).
Hash `export.rs` preservado bit-exact pelo **15º passo
consecutivo**.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- `op("lim", limits: true)` + attach `_(x → 0)` — limits-style
  positioning (abaixo, centrado).
- `op("sin", limits: false)` + attach `^2` — scripts-style
  (lateral superior).
- `op("max", limits: true)` sem attach — só o text.
- `op("", limits: true)` — text vazio (degenerate).
- `op("Σ", limits: true)` — caractere Unicode operator.
- `op("lim")` sem `limits` arg — default? `false`? `true`?
  Verificar A.1.4.

### A.5' — Verificação anti-reflexão (N=7 do padrão §8.6)

**6ª reaplicação A.0.0** consecutiva.

4 verificações:

1. **Comparação literal A.1.6 P288-P298** — paradigma novo?
   - P293-P297 nove paradigmas distintos.
   - **P298**: paradigma novo "cross-variant interaction"
     (`MathOp` afecta `MathAttach`) — **estructuralmente novo**
     mesmo se A.0.0 magnitude modesta.
2. **A.0 produzido empiricamente** — A.0.0 P298 magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **Cross-variant interaction** (op affects attach layout) —
     **paradigma genuinamente novo** no cluster math.
   - **`bool` discriminador estrutural** — caso intermédio para
     "variant rico" N=5 (registar honestamente).
   - **Fecho cluster math 4/4** — sub-padrão "cluster math
     handler" N=3 atinge limiar tentativo.
4. **Decisão sobre promoção ADR meta**:
   - **Preferência principal**: **sub-padrão "cluster math
     handler dedicado" N=3** — promoção candidata cumulativa.
   - **§8.7' N=6** se A.0.0 magnitude alta — secundária.
   - **§8.3 N=10** candidato adiado — terciária.
   - **"Variant rico" N=5** — **adiado** se `bool` ambíguo.
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério escolha**: preferir promoção que **emerge naturalmente
do trabalho concreto P298** (sub-padrão "cluster math handler")
sobre promoções genéricas (§8.7'/§8.3 metodológicas). Sub-padrão
tem 3 aplicações arquitecturalmente concretas e fecho-natural do
cluster.

---

## §3 — Materialização

Após Fase A (com decisões A.2/A.3/A.4):

**Cenário default (HI'' + A.2 → (b) + A.3 → α + A.4 → i)**:

1. Adicionar `MathOp { text: EcoString, limits: bool }` em
   `entities/content.rs`.
2. Match arms exhaustive (paralelo P296/P297; 9 sítios).
3. Stdlib `native_op(text, limits: bool = false)` em
   `structural.rs`.
4. Handler `layout_op` em `rules/math/layout/mod.rs`:
   - Emite `FrameItem::Text` com text (font math).
   - **Não posiciona attach** — Layouter exterior usa flag.
5. Modificar `layout_attach` (ou equivalente) para verificar se
   `base` é `MathOp { limits: true, .. }`:
   - Se sim: renderiza super/sub abaixo/acima (limits-style).
   - Se não: scripts-style (default actual).
6. Registar `op` em `eval/mod.rs` scope math.
7. Lista operadores vanilla pré-definidos (`lim`/`sin`/`cos`/etc.)
   — **scope-out P298**: passo dedicado P298.X se A.1.5 mostrar
   integração com scope module (e.g. `math::op::lim`).
8. Testes ~10-15.

Aplicação ADR-0098 obrigatória — A.0 documenta inspecção emit.

Promoção ADR meta condicional per A.5':
- **Preferência: sub-padrão "cluster math handler dedicado"
  N=3** se A.3 → (α) confirma e fecho cluster genuíno.
- Default secundário: **sem promoção** anti-padrão.
- "Variant rico" N=5 **adiado** se `bool` ambíguo.

Actualizar L0:
- `entities/content.md` (+1 variant `MathOp`).
- Tabela A.4 linha 119 — `op` `parcial` → `implementado`.
- Tabela B — +1 variant.
- Propagar hashes.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-15.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P297: 2 841 testes.
  Esperado: ~2 849-2 856.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` muda (+1 variant).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `export.rs` **preserved bit-exact** pelo **15º passo
  consecutivo** se A.4-bis → (i) (esperado quase certo).
- **Regressão bit-exact** para math features pre-P298
  (frac/attach/root/lr/matrix/cases/accent/cancel/underover
  inalteradas).
- **Regressão visível em attach**: se A.4 → (i) modifica
  `layout_attach`, **testes regression específicos**:
  - `x^2` (scripts-style sem op) preservado bit-exact.
  - `sum_(i=1)^n` (scripts-style com `sum` actualmente?
    verificar A.1.5).
- Tabela A.4 linha 119 (`op`) reclassificada para `implementado`.
- Tabela B Math variants — +1 entrada.
- **Cluster math 4/4 features fechadas** — Tabela A.4 linha 119
  completa.
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **Promoção ADR meta condicional**:
  - **Sub-padrão "cluster math handler dedicado" N=3 promoção
    preferencial** se A.3 → (α) confirma.
  - **§8.7' N=6** secundária se A.0.0 magnitude alta.
  - **Uma ADR meta por passo** (P273.17 §0).
- "Confirmação esperada" registada como categoria honesta se
  aplicável.

---

## §5 — Não-objectivos

- **Não** materializar operadores vanilla pré-definidos
  (`lim`/`sin`/`cos`/etc.) como funções stdlib distintas. Passo
  P298.X candidato se A.1.5 mostrar integração com scope `math`
  module (e.g. `math.lim`).
- **Não** alterar `MathAttach` semanticamente além do mínimo
  para detectar `MathOp` base. Refactor maior é sub-passo
  P298.0.
- **Não** materializar atributos cosméticos extra de `OpElem`
  vanilla se existirem (`size`/`color`/etc.). ADR-0054 graded.
- **Não** promover múltiplas ADRs meta. **Uma por passo**
  (P273.17 §0). Preferência: sub-padrão "cluster math handler"
  N=3 (concreta) sobre §8.7'/§8.3 (metodológicas).
- **Não** confundir P298 com P296/P297 — paradigma
  cross-variant interaction é distintivo (A.4 explicita).
- **Não** considerar P298 como rubber-stamp — diferença
  estrutural genuína registada §1.2 + paradigma cross-variant
  novo §A.5'.

---

## §6 — Pendências relacionadas

Resolve:
- Tabela A.4 linha 119 — `op` parcial → implementado.
- **Cluster math 4/4 fechado** — accent+cancel+underover+op.

Não resolve (continua aberto):
- Operadores vanilla pré-definidos (`lim`/`sin`/etc.) como
  scope module — P298.X candidato.
- P297.X discriminator `UnderoverKind` — cosmético.
- P296.X toggles cancel — cosmético.
- Cosméticos accent/cancel/underover/op — ADR-0054 graded.
- Math shaping completo — ADR-0054 perfil graded.

---

## §7 — Risco residual

Risco principal: **A.4 → (i) modifica `layout_attach`** — risco
de regressão em features attach pre-P298. Mitigação: testes
regression bit-exact obrigatórios em critério §4; se regressão
detectada, abrir sub-passo P298.0 dedicado a isolar refactor.

Risco secundário: **A.0.0 magnitude alta inesperada** revelando
HV'' (heurística limits-style já parcial). Mitigação:
interrupção honesta permitida; sub-passo P298.0.

Risco terciário: **`bool` discriminador ambíguo "variant rico"
N=5**. Mitigação: §A.2 explicita registo honesto; adiar promoção
se ambíguo (anti-padrão sobre-formalização).

Risco quaternário: **sub-padrão "cluster math handler" N=3
promoção forçada** sem maturidade real. Mitigação: §A.5' critério
"emerge naturalmente do trabalho concreto"; se promoção parece
forçada (e.g. handlers são triviais wrappers), adiar.

Risco quinário: **6ª A.0.0 consecutiva** sem novo paradigma
empírico significativo. Mitigação: cross-variant interaction
em A.4 é paradigma novo factual; mesmo se A.0.0 magnitude
modesta, P298 não é rubber-stamp.

Risco senário: **fecho cluster math 4/4 cria pressão para nova
sequência cumulativa**. Mitigação: §5 explicita P298 é **fecho
natural**, não trampolim; P299 será ortogonal por construção
(paralelo P293 pós-série Style).

Risco septenário: **regressão silenciosa em features math
existentes**. Mitigação: §4 testes regression bit-exact
obrigatórios.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/content.rs`
  (`Content::Math*` variants; 13 pós-P297).
- Função stdlib: `01_core/src/rules/stdlib/structural.rs`
  (paralelo `native_accent`/`native_cancel`/`native_underover`).
- Layouter consumer: `01_core/src/rules/math/layout/mod.rs`
  (`layout_node` arm + `layout_op` handler + modificação
  `layout_attach`).
- Vanilla: `lab/typst-original/crates/typst-library/src/math/op.rs`.
- Precedente arquitectural directo: **P296** + **P297**.
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded (justifica simplifications
  cosméticas).
- Padrão §8.7' A.0.0 template (N=6 reaplicação).
- Padrão §8.3 refutação pragmática (N=10 candidato adiado).
- **Padrão "cluster math handler dedicado" N=3 cumulativo
  candidato** — preferência preferencial.
- Padrão "variant rico" N=5 — caso ambíguo `bool` discriminator.

---

*Spec P298 produzida 2026-05-19 pós-P297 (math underover fechado
com agregação; A.0.0 N=5 magnitude alta refutou degenerescência).
Frente `P296.2 — op` — fecha cluster math 4/4. **Diferença
estrutural genuína vs P296/P297**: `op` tem `text: EcoString +
limits: bool` (não `body: Content`); **paradigma cross-variant
interaction** novo (`MathOp` afecta layout `MathAttach`).
Magnitude XS-S esperada mas A.0.0 pode revelar HV'' (heurística
limits-style já parcial) → magnitude alta. Fase A obrigatória
com **7 secções** A.0.0+A.0-A.5+A.5'. **A.0.0 P298 é 6ª
reaplicação consecutiva** — testa persistência tendência
não-decrescente. **Promoção ADR meta preferencial**: sub-padrão
"cluster math handler dedicado" **N=3 cumulativo** (P296+P297+P298)
— emerge naturalmente do trabalho concreto, distinto de §8.7'/§8.3
metodológicos. **"Variant rico" N=5 caso ambíguo**: `bool
limits` é discriminador estrutural mas não Option estrutural —
**adiar promoção** se ambíguo (anti-padrão sobre-formalização).
**Uma ADR meta por passo no máximo** (P273.17 §0). **Cluster
math 4/4 fechado pós-P298** — P299 ortogonal por construção
(paralelo P293 pós-série Style). Sem caps LOC ou magnitude
(P282 §7).*
