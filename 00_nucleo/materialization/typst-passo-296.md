# Passo 296 — Math Accent + Cancel

**Frente**: `P-math-accent-cancel` (rank 1 frente original P292
§9.2; recomendada P294 §10 e P295 §9).
**Origem**:
- Tabela A.4 linha 118: `accent(c, mark)` → **parcial** (math
  passos; heurístico; sem todas as variantes Unicode).
- Tabela A.4 linha 119: `cancel`, `underover`, `op` → **parcial**
  (math passos; parcial).
**Pré-requisitos**: nenhum bloqueador formal.
**Tipo declarado**: frente ortogonal **genuinamente nova** — não
extensão directa de P293-P295. **Scope: Accent + Cancel apenas**
(scope-out `underover` e `op`; cada com passo próprio).
**Marco**: 3º passo ortogonal pós-série cumulativa cirúrgica
P288-P294.

---

## §1 — Objectivo (provisório; depende fortemente de A.0.0)

Refinar `accent(c, mark)` e `cancel(body)` de `parcial` para
`implementado` na Tabela A.4. **Scope concreto depende
significativamente de A.0.0** porque o estado `parcial` não está
detalhado factualmente — Tabela B Content variants (linhas
291-301) **não contém `MathAccent` nem `MathCancel`** apesar de
ambos serem listados como `parcial` em A.4.

**Possíveis interpretações do "parcial"** (A.0.0 obrigatória
clarifica):

| Hipótese | Descrição |
|---|---|
| **HI** — Variant ausente, tratamento ad-hoc | Stdlib aceita `accent(...)` mas resolve via `MathSequence` heurístico (sem variant dedicado) |
| **HII** — Variant existe mas é interno | `MathAccent` existe mas não aparece em Tabela B (não-documentado) |
| **HIII** — Stdlib função existe sem variant | `native_accent`/`native_cancel` materializadas mas retornam approximação via outros variants |
| **HIV** — Ambos ausentes; tratamento via fallback | `accent`/`cancel` no stdlib retornam erro descritivo ou ignoram com fallback |
| **HV** — Mix | Combinação parcial das anteriores |

**A.0.0 inspecciona literalmente** `01_core/src/entities/content.rs`
+ `01_core/src/rules/stdlib/math.rs` (ou caminho equivalente) +
vanilla `lab/typst-original/.../math/accent.rs` e
`lab/typst-original/.../math/cancel.rs` para decidir hipótese.

**Resultado esperado**: A.0.0 produz scope concreto P296 baseado em
hipótese factual, **não em pressuposto da spec**. Se A.0.0 descobre
discrepância significativa entre Tabela A.4 e código real,
**refutação genuína** registada — **§8.3 N=8 candidato** (P294 §7.1
adiou N=6; P295 §6.2 adiou N=7; P296 conta sobre +1).

Razão de ser metodológica:

1. **Frente recomendada repetidamente** (P292 §9.2 #1; P294 §10;
   P295 §9). Backlog histórico merece atenção.
2. **Magnitude controlada (XS+S)** — bom equilíbrio para 3º
   ortogonal sem repetir cluster M (P295) nem voltar a XS-XS
   cumulativo.
3. **Possível refutação significativa A.0.0** — Tabela B variants
   omite `MathAccent`/`MathCancel`; significância de "parcial"
   genuinamente ambígua. **Risco P295 §10 "sequência reflexa"
   empiricamente testado** — se A.0.0 P296 revelar refutação
   significativa (não factual-modesta), §8.7' N=4 + promoção
   robusta viável.
4. **Math: 40% → 50%** — incremento substancial (2 features
   `parcial` → `implementado` em categoria com 12 entries).
5. **Reaplicação ADR-0098 + ADR-0099** — N=13 + N=12 cumulativos
   condicionais; padrão "single source of truth" deve manter
   porque math layout consome `Content::*` agnóstico ao emit.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal do estado "parcial" (N=4 cumulativo §8.7')

**Crítico**: significância de "parcial" linhas 118-119 é ambígua;
ausência de `MathAccent`/`MathCancel` em Tabela B é evidência
indirecta. Inspecção literal obrigatória:

1. **`grep -rn "Math.*Accent\|Math.*Cancel\|MathAccent\|MathCancel"
   01_core/`** — confirmar existência ou ausência de variants.
2. **`grep -rn "native_accent\|native_cancel" 01_core/`** —
   verificar stdlib functions.
3. **Inspeccionar `01_core/src/entities/content.rs`** — listar
   variants `Math*` actuais; comparar com Tabela B linhas 291-301.
4. **Inspeccionar `01_core/src/rules/stdlib/math.rs`** (ou
   caminho equivalente) — APIs expostas para math.
5. **Inspeccionar `lab/typst-original/crates/typst-library/src/math/accent.rs`** —
   estrutura `AccentElem` vanilla (atributos, layout).
6. **Inspeccionar `lab/typst-original/crates/typst-library/src/math/cancel.rs`** —
   estrutura `CancelElem` vanilla.
7. **Inspeccionar `01_core/src/rules/layout/math.rs`** (ou caminho
   equivalente) — `layout_math_*` que tratam accent/cancel
   actualmente.
8. **Comparar com `frac`/`attach`/`root`** (precedentes math
   `implementado`) — paradigma esperado: variant dedicado +
   stdlib function + Layouter consumer + emit via
   `FrameItem::Text` (ADR-0098 vigente).

**Decisão A.0.0 sobre hipótese HI-HV**:
- Documentar literalmente em
  `00_nucleo/diagnosticos/diagnostico-math-accent-cancel-passo-296.md`.
- Reformular §3 materialização conforme hipótese real.
- **Permitir interrupção honesta** se HV descobre tarefas
  fragmentadas em sub-passos múltiplos.

**Vigilância anti-reflexão** (P295 §10 + §6.6):

Comparar magnitude da refutação A.0.0 P296 com sequência
P293→P294→P295:

- P293: Hipótese H6 não-listada (**alta**).
- P294: Spec inteira invalidada (**máxima**).
- P295: Linha administrativa desactualizada (**baixa**).

P296 deve registar **explicitamente** se A.0.0 descobre:
- **Refutação significativa** (variants existem mas variant para
  feature parcial não está documentado em Tabela B; ou stdlib
  function existe parcialmente; ou paradigma genuinamente
  divergente do precedente frac/attach/root) → §8.7' template
  valida com refutação real.
- **Refutação factual-modesta** (Tabela A nota imprecisa) →
  P296 inaugura **4ª A.0.0 consecutiva sem ganho empírico
  significativo** → desformalização do template candidata.
- **Refutação nula** (Tabela A factualmente correcta; código
  reflecte exactamente "parcial heurístico") → A.0.0 não gerou
  ganho em P296 → reforça candidato a desformalização.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "accent\|cancel" 03_infra/src/export.rs` | **Zero hits funcionais esperados** — math layout consome via `FrameItem::Text`/`FrameItem::Glyph`/`FrameItem::Line` agnóstico | Inspecção literal |
| Materialização P296 toca emit? | **Não esperado** — accent é glyph + posicionamento; cancel é stroke linha + posicionamento; ambos via FrameItem existentes | A.1.7 confirma |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 13º passo consecutivo | Verificação final |

### A.1 — Inventário literal

8 sub-secções condicionais a A.0.0 (hipótese HI-HV):

1. **A.1.1 — `Content` Math* variants pós-P295** — listar
   literalmente; confirmar (ou refutar) ausência `MathAccent`/`MathCancel`.
2. **A.1.2 — Tipos relacionados** — `Accent` struct? Algum tipo
   helper?
3. **A.1.3 — Match exaustivo sobre `Content::Math*`** — sítios
   onde adicionar arms se HI escolhida.
4. **A.1.4 — Vanilla `AccentElem` + `CancelElem`** — atributos:
   `AccentElem { base, accent, size?, dotless?, ... }`;
   `CancelElem { body, length?, inverted?, cross?, angle?, stroke?, ... }`.
5. **A.1.5 — Stdlib actual** — `native_accent` / `native_cancel`
   existem? Retornam o quê?
6. **A.1.6 — Layouter consumer** — `layout_math` ou similar;
   como tratam accent/cancel hoje (paralelo a frac/attach/root).
7. **A.1.7 — Emit** — verificar literalmente que accent/cancel
   produzem `FrameItem::Glyph` + `FrameItem::Line` standard.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente
   conforme hipótese.

### A.2 — Estrutura dos variants (condicional a A.0.0)

Se HI escolhida (variants ausentes):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `MathAccent { base, accent }` + `MathCancel { body }` mínimos | Paralelo `MathFrac{num, den}` literal | Sem atributos cosméticos (`size`/`length`/`stroke`/etc.); ADR-0054 graded |
| **(b)** `MathAccent { base, accent, dotless: Option<bool> }` + `MathCancel { body, length: Option<Length>, inverted: bool, ... }` | Paridade vanilla com cosméticos | Reaplica padrão "variant rico" N=5 — **dispara qualificação genuína** se a) não trivializa, b) cosméticos realmente usados |
| **(c)** Apenas `MathAccent` materializar; `MathCancel` adiar | Scope reduzido | Refuta intent de P296; melhor abrir 2 sub-passos |

Default sugerido: **(a) minimal** salvo se A.1.4 mostrar que
cosméticos vanilla são essenciais (e.g. `inverted: bool` em cancel
é binary toggle, não cosmético).

**Implicação gatilho "variant rico"**:
- (a) preserva padrão N=4 inalterado.
- (b) qualifica **N=5 candidato genuíno** se cosméticos não
  triviais — primeira qualificação desde P287 refutação e P295
  refutação consciente.

### A.3 — Integração com sistema math actual

| Opção | Comportamento |
|---|---|
| **(α)** Variants novos + `layout_math` arm novo (paralelo frac/attach/root) | Paradigma single source of truth aplicado a math layout |
| **(β)** Reuso integral de `MathSequence` + transformações inline | Sem variants novos; menos isolamento |
| **(γ)** Stdlib só (sem variant) — accent/cancel retornam `MathSequence` construído | Híbrido |

Default sugerido: **(α)** se HI/HII confirmadas. **(β)** se HIII
revela accent/cancel actualmente operam via MathSequence (refino
qualitativo no consumer existente). **(γ)** rejeitada salvo
limitação estrutural.

### A.4 — Impacto em emit (aplicação ADR-0098)

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** Math layout produz `FrameItem::Glyph` + `FrameItem::Line` standard; emit lê paradigma P136 | `export.rs` **preservado** — 13º passo consecutivo |
| **(ii)** Cancel diagonal stroke exige operator PDF novo (path com transform?) | Improvável — `FrameItem::Line` suporta linhas em qualquer ângulo |
| **(iii)** Híbrido | Caso edge |

Default sugerido: **(i)** quase certo. Verificar A.1.7.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- **Accent**: base com ascendente (e.g. `f` com circumflexo) —
  posicionamento Y; vanilla typst tem heurística.
- **Accent**: accent com glyph não-existente em font corrente —
  fallback.
- **Cancel**: body vazio — comportamento?
- **Cancel**: body com width zero — ângulo da linha?
- **Cancel inverted**: linha em ângulo oposto — paridade.
- **Cancel cross**: 2 linhas em X — overlay.

### A.5' — Verificação anti-reflexão (N=5 do padrão §8.6)

**Crítica especial pós-P295**: P295 §10 registou *"P296+ deve
testar se A.0.0 ainda gera valor empírico ou se degenerou em
ritual procedimental"*.

P296 é o **teste empírico explícito** desta hipótese.

4 verificações:

1. **Comparação literal A.1.6 P288-P296** — paradigma novo
   esperado:
   - P288 lang: cross-module.
   - P289 weight: TextStyle method.
   - P290 tracking: per-glyph + Tc emit.
   - P291 leading: per-line via peek.
   - P292 font: FontBook indirect resolution.
   - P293 cubic: activação posterior variant inerte.
   - P294 quadratic: transform-on-build (construct-time).
   - P295 footnote: walker counter no Layouter.
   - **P296 math accent/cancel: ??? — depende A.0.0**.
2. **A.0 produzido empiricamente** — A.0.0 magnitude registada.
3. **Elementos estructuralmente novos identificados**:
   - **Magnitude A.0.0 vs P293-P295** — registar honestamente.
   - **Hipótese HI-HV decidida** — primeira spec onde A.0.0 não
     antecipa hipótese principal com confiança (P293 antecipou
     H1-H5; P294 antecipou paralelismo P293; P295 antecipou
     bloqueador desactualizado). P296 **antecipa ambiguidade
     mas não hipótese preferida**.
   - **Math layout consumer** — paradigma diferente de
     P288-P295 (todos foram Style/Text/Frame).
   - **Possível qualificação "variant rico" N=5** se A.2 → (b).
4. **Decisão sobre promoção ADR meta**:
   - **§8.7' N=4** se A.0.0 refutação **significativa** (não
     factual-modesta como P295) → **promoção robusta candidata**.
   - **§8.7' N=4 mas refutação factual-modesta** → **adiar
     promoção; considerar desformalização do template** (lição
     P295 §10).
   - **§8.3 N=8** se refutação pragmática genuína.
   - **"variant rico" N=5** se A.2 → (b).
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério explícito anti-reflexão**: se A.0.0 P296 descobre
apenas refutação factual-modesta (paralela P295), **registar
inversão de §6.6 P295**:

> *4 reaplicações A.0.0 consecutivas com magnitude decrescente ou
> factual-modesta. Template degenera em ritual procedimental.
> Considerar desformalização — A.0.0 deixa de ser secção
> obrigatória; passa a ser secção condicional ("aplicar quando há
> ambiguidade arquitectural genuína").*

---

## §3 — Materialização (condicional a A.0.0)

Plano genérico per hipótese HI/HII/HIII/HIV/HV.

**Cenário HI default + (a) minimal + (α) layout integration**:

1. Adicionar `MathAccent { base: Box<Content>, accent: Box<Content> }`
   e `MathCancel { body: Box<Content> }` em
   `01_core/src/entities/content.rs`.
2. Match arms exhaustive em todos os sítios (paralelo P295
   estratégia: defesa cumulativa via compiler errors).
3. Stdlib functions `native_accent(base, mark)` e
   `native_cancel(body)` — paralelo `native_frac` precedente.
4. Layouter consumer `layout_math` arms novos:
   - Accent: posiciona glyph `accent` acima de bbox de `base`.
   - Cancel: emite `FrameItem::Line` diagonal sobre bbox de `body`.
5. Testes ~10-15 (5-8 unit + 5-7 integração L3).

**Cenário HIII (stdlib parcial existente)**:

1. Localizar stdlib `native_accent`/`native_cancel` actuais.
2. Refactor para produzir variants novos em vez de
   `MathSequence` heurístico.
3. Layouter arms novos.
4. Testes incluem regression bit-exact para PDFs pré-P296 (se
   stdlib produz variant novo, output PDF pode mudar).

**Outros cenários**: adaptados.

Aplicação ADR-0098 obrigatória — A.0 documenta inspecção emit.

Promoção ADR meta condicional per A.5':
- **§8.7' N=4 promoção** se refutação A.0.0 significativa.
- Default: **sem promoção** (anti-padrão P273.17 §0).

Actualizar L0:
- `entities/content.md` (+2 variants se HI).
- Tabela A.4 linhas 118-119 — `parcial` → `implementado` ou
  `implementado⁺` conforme scope materializado.
- Tabela B — +2 variants `MathAccent`, `MathCancel`.
- Propagar hashes.

**Sem caps** (per P282 §7). Estimativa de testes: 10-20.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P295: 2 817 testes.
  Esperado: ~2 827-2 837.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` muda (+2 variants se HI).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `export.rs` **preserved bit-exact** pelo **13º passo
  consecutivo** se A.4 → (i) (esperado quase certo).
- **Regressão bit-exact validada** para math features pre-P296
  (frac/attach/root/lr/matrix/cases inalteradas).
- Tabela A.4 linha 118 (`accent`) e 119 (`cancel`) reclassificadas
  conforme hipótese.
- Tabela B Math variants — +2 entradas.
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **A.0.0 magnitude registada honestamente** — significativa,
  factual-modesta, ou nula. Decisão sobre desformalização
  template §8.7' condicional.
- **Sem promoção ADR meta** default; condicional a A.5'
  conclusiva.

---

## §5 — Não-objectivos

- **Não** materializar `underover` (Tabela A.4 linha 119; vanilla
  `UnderoverElem`). Passo dedicado P296.1 candidato se P296 fecha
  com sucesso.
- **Não** materializar `op` (linha 119; vanilla `OpElem`). Passo
  dedicado P296.2 candidato.
- **Não** materializar atributos cosméticos completos vanilla
  (e.g. `dotless`, `cross`, `angle`) se A.2 → (a). ADR-0054
  graded justifica.
- **Não** estender paradigma math layout. Reuso integral do
  existente (`MathSequence` + alignment helpers).
- **Não** promover ADR meta **a menos que A.0.0 produza refutação
  genuinamente significativa**. Critério estrito anti-padrão.
- **Não** desformalizar §8.7' (A.0.0 template) imediatamente.
  Decisão fica para P297+ se A.0.0 P296 confirmar tendência
  factual-modesta cumulativa.
- **Não** confundir `MathAccent` com vanilla `AccentElem`
  (estructuras diferentes; cristalino simplifica per ADR-0054).

---

## §6 — Pendências relacionadas

Resolve (condicional a hipótese):
- Tabela A.4 linha 118 (`accent`) — `parcial` → `implementado`/`implementado⁺`.
- Tabela A.4 linha 119 parcialmente (`cancel`; `underover` e `op`
  ficam fora).

Não resolve:
- `underover` (P296.1 candidato).
- `op` (P296.2 candidato).
- Math shaping completo (ADR-0054 perfil graded).
- `equation.numbering` linha 122 (parcial).

---

## §7 — Risco residual

Risco principal: **A.0.0 magnitude factual-modesta** (paralela
P295) confirma degenerescência §8.7' template. Mitigação: §A.5'
critério explícito anti-reflexão; relatório registar honestamente.
Não tentar inflacionar magnitude artificialmente.

Risco secundário: **HIV** (ambos features ausentes apesar de
"parcial") revela trabalho maior que XS+S esperado. Mitigação:
considerar interrupção honesta (paralelo P293 §A.0.0.3); abrir
P296.0 (Accent only) + P296.1 (Cancel only) se complexidade
revela-se.

Risco terciário: **A.2 → (b) qualifica "variant rico" N=5**. Se
disparar simultaneamente com §8.7' N=4 promoção, **uma ADR meta
por passo** (P273.17 §0). Preferência: §8.7' (mais cumulativa) ou
"variant rico" (mais directa) conforme magnitude.

Risco quaternário: **emit muda inesperadamente** (HII/HIII revela
acoplamento). Mitigação: A.4 inspecção literal; documentar como
alteração justificada se ocorrer.

Risco quinário: **regressão bit-exact em math pre-P296**.
Mitigação: testes regression obrigatórios para frac/attach/root/etc.
inalterados.

Risco senário (anti-meta): "sequência reflexa" §8.6 A.5' N=5
desformaliza-se acidentalmente em P296 sem decisão consciente.
Mitigação: §5 não-objectivo explicita "decisão fica para P297+";
A.0.0 P296 é teste empírico, não decisão automática.

Risco septenário (cluster math): refino accent/cancel revela
divergências em outros math features (e.g. attach scripts em
combinação com accent). Mitigação: A.5 cenários fronteira;
regression robusta.

---

## §8 — Ponteiros

- Tipo a inspeccionar: `01_core/src/entities/content.rs`
  (`Content::Math*` variants; pós-P295: número exacto a confirmar
  em A.1.1).
- Função stdlib: `01_core/src/rules/stdlib/math.rs` (ou caminho
  equivalente).
- Layouter consumer: `01_core/src/rules/layout/math.rs` (ou
  similar).
- Vanilla: `lab/typst-original/crates/typst-library/src/math/accent.rs`
  + `cancel.rs`.
- Precedentes arquitecturais math `implementado`:
  - P37 (`frac`) — variant `MathFrac` + Layouter arm.
  - P35 (`attach`) — variant `MathAttach` + Layouter arm.
  - `MathRoot` (math passos) — variant `MathRoot` + Layouter arm.
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded (justifica simplifications cosméticas).
- Padrão §8.7' A.0.0 template (N=4 reaplicação; **teste empírico
  P295 §10 hipótese degenerescência**).
- Padrão §8.3 refutação pragmática (N=8 candidato).
- Padrão "variant rico" N=4 (P156G/H/I + P284); N=5 candidato se
  A.2 → (b).

---

*Spec P296 produzida 2026-05-19 pós-P295 (footnote Fase 1 fechado;
A.0.0 N=3 refutação factual-modesta registou risco "sequência
reflexa"). Frente `P-math-accent-cancel` — refino 2 features
`parcial` em Math (linhas 118-119 Tabela A.4). **Magnitude
controlada (XS+S)**; **scope reduzido**: accent + cancel apenas
(underover e op em passos próprios futuros). Fase A obrigatória
com **7 secções** A.0.0+A.0-A.5+A.5'. **A.0.0 P296 é teste
empírico explícito** da hipótese degenerescência do template §8.7'
levantada em P295 §10: se refutação for factual-modesta ou nula,
desformalização candidata; se refutação significativa, promoção
§8.7' N=4 robusta. **Hipótese principal não antecipada** (HI-HV
todas plausíveis) — primeira spec onde A.0.0 não tem hipótese
preferida com confiança. **A.2 → (b) qualifica "variant rico" N=5**
genuíno se cosméticos vanilla não-triviais (`inverted`/`cross` para
cancel) — gatilho diferente de §8.7'/§8.3. **Uma ADR meta por
passo no máximo** (P273.17 §0). Honestidade epistémica reforçada:
spec **aceita não saber qual hipótese se vai confirmar** mas
documenta critério estrito anti-reflexão para evitar inflação
ritualística. Sem caps LOC ou magnitude (P282 §7).*
