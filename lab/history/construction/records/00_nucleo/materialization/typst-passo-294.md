# Passo 294 — `PathItem::QuadraticTo` + emit + `native_quadratic`

**Frente**: `P-quadratic-curve` (rank 1 candidato P293 §10).
**Origem**: P293 §3.1 — `native_curve` retorna `Err("scope-out P293
ADR-0054 graded")` para `"quadratic"` kind. Frente pendente
registada literalmente em P293 §9 + §10.
**Pré-requisitos**: P293 (caminho `native_curve` constructor +
PathItem variants framework).
**Tipo**: extensão directa P293 **mas estructuralmente diferente**
(ver §1).

---

## §1 — Objectivo e enquadramento factual

Materializar `PathItem::QuadraticTo(control: Point, end: Point)`
no enum `PathItem`, integrar com `native_curve` (P293) removendo
o scope-out de `"quadratic"`, e adicionar emit PDF para operador
quadrático Bézier (`v` ou `y` per PDF spec).

### §1.1 — Crítico: P294 ≠ H6 reaplicado de P293

P293 §10 sugeriu "replica padrão P293 (caminho de entrada + emit
Bézier quadrático `v` ou `y`)" — **mas a inspecção factual
revela que P294 é estructuralmente distinto de P293**:

| Aspecto | P293 (CubicTo) | P294 (QuadraticTo) |
|---|---|---|
| `PathItem` variant | **Existia** desde P277 (inerte) | **Não existe** — precisa materializar |
| Emit PDF operator | **Existia** em `export.rs:2375/2457/2629` (`c`) | **Não existe** — precisa materializar (`v`/`y`) |
| `path_bbox` (P277) suporte | Suportava cubic (DEBT-33 fechou bbox cúbica) | Suportava cubic; **quadratic precisa verificar empiricamente** |
| Hipótese A.0.0 esperada | H6 "activação posterior inerte" | H1 + H2 (cubic operations + PDF emit cubic da spec P293 original — agora aplicáveis a quadratic) |
| Hash `export.rs` | Preservado bit-exact (10º passo) | **Vai mudar intencionalmente** (1ª quebra desde P281) |

Portanto P294 é **materialização from-scratch** de feature, não
activação posterior. P293 H6 não se aplica — H6 exigia variant +
emit pré-existentes inertes.

### §1.2 — Razões

1. **Completar paridade `path_items`** — variants do PathItem:
   `MoveTo`/`LineTo`/`CubicTo`/`ClosePath` actualmente; **falta
   `QuadraticTo`** para paridade vanilla typst.
2. **Marco histórico controlado**: **1ª quebra de hash
   `export.rs` desde P281** (10 passos preservaram). Não é
   regressão — emit novo PDF operator (`v`/`y`) **legitima**
   alteração per ADR-0098 §"alterações justificadas".
3. **Activação cumulativa do `native_curve` stdlib P293** —
   remove o scope-out de `quadratic`; output `Err` actual passa
   a produzir `PathItem::QuadraticTo`.
4. **Reaplicação de ADRs vigentes**:
   - ADR-0098: **N=11 cumulativo** condicional — desta vez com
     alteração justificada, não preservação literal. Documentar
     em A.4.
   - ADR-0099: **N=10 cumulativo** — padrão "activação posterior"
     aplica-se ao scope-out (`Err` → variant funcional). **Mas
     também é materialização de variant novo + emit novo** —
     activação ≠ adição. Esta dupla natureza merece registo em
     A.5'.
5. **§8.3 refutação pragmática N=6** — P293 §7.3 refutou N=6
   candidato; P294 pode promovê-lo se descobrir refutação
   significativa diferente. Mas **provavelmente não**
   (materialização paralela a padrões anteriores).

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

A.0.0 inaugurada em P293; **reaplica N=2 do padrão §8.7'** — mas
com expectativa diferente: P294 sabe-se materialização-from-scratch
ab initio.

### A.0.0 — Verificação que P294 ≠ H6 reaplicado

Inspeccionar literalmente para confirmar §1.1:

1. `01_core/src/entities/geometry.rs` (ou caminho equivalente
   `PathItem`):
   - Variants actuais de `PathItem`: `MoveTo`, `LineTo`, `CubicTo`,
     `ClosePath`. Confirmar **ausência** de `QuadraticTo`.
2. `03_infra/src/export.rs:2375/2457/2629` (sítios `c` operator
   cubic):
   - Inspeccionar emit cubic existente.
   - Confirmar **ausência** de `v`/`y` operator quadrático.
3. `01_core/src/entities/shape.rs` `path_bbox`:
   - Verificar se algoritmo de bbox suporta `QuadraticTo` ou se é
     extensão necessária.
4. `lab/typst-original/.../curve.rs`:
   - Como vanilla typst expressa quadratic Bézier? Como armazena
     control point único?

**Decisão A.0.0**:
- Se §1.1 confirmado → P294 procede com plano H1+H2 (materialização).
- Se inspecção revela variant `QuadraticTo` já existente (improvável)
  → reclassificação para H6 reaplicado de P293; spec adapta.

### A.0 — Potencial de reuso ADR-0098 (alteração justificada)

| Verificação | Esperado |
|---|---|
| Hash `export.rs` esperado pós-P294 | **Muda** intencionalmente — emit `v`/`y` operator novo |
| Mudança preserva ADR-0098? | **Sim** — ADR-0098 §"alterações justificadas" cobre adição de operator novo paralelo ao `c` operator existente; **não** é violação do single source of truth |
| Casos não-quadratic (cubic, line, move, close) preservam bytes? | **Bit-exact preserved** — emit existente inalterado |

**A.0 não-trivial pela primeira vez na sequência cirúrgica**:
P282-P293 preservaram hash literalmente. P294 quebra
intencionalmente. ADR-0098 é robusta a esta quebra **se** a
verificação for explícita.

Documentar em A.0 secção dedicada:
- Hash antes e depois.
- Quais bytes do `export.rs` mudam (esperado: novo bloco match
  arm `PathItem::QuadraticTo => ...` paralelo ao `CubicTo`
  existente).
- Confirmação que ramos pré-existentes (`MoveTo`/`LineTo`/`CubicTo`/
  `ClosePath`) produzem bytes idênticos vs pré-P294 (validação
  por hash de PDFs de teste em cenários sem quadratic).

### A.1 — Inventário literal do caminho `PathItem`

8 sub-secções:

1. **A.1.1 — Variants actuais** — confirmar 4: `MoveTo`, `LineTo`,
   `CubicTo`, `ClosePath`.
2. **A.1.2 — Estrutura `PathItem::CubicTo`** — `CubicTo(c1: Point,
   c2: Point, end: Point)` ou similar; verificar literalmente.
3. **A.1.3 — `match` exaustivos sobre `PathItem`** —
   `grep -rn "PathItem::" 01_core/ 03_infra/ 02_layout_types/`.
   Listar todos os sítios. Adicionar `QuadraticTo` arm a cada um
   (defesa cumulativa compilador).
4. **A.1.4 — Constructors actuais** — `native_curve` P293 só.
5. **A.1.5 — Layouter consumer** — `path_bbox` (P277) extensão
   necessária? Verificar.
6. **A.1.6 — Emit consumer** — `export.rs:2375/2457/2629` para
   cubic. **Paradigma consumer P294**: replica `c` operator emit
   adicionando `v` ou `y` paralelo.
7. **A.1.7 — Vanilla typst comparação** — como `curve.quadratic`
   é definido em `lab/typst-original/.../curve.rs`?
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente.

### A.2 — Estrutura do variant `QuadraticTo`

Decisão arquitectural:

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `QuadraticTo(control: Point, end: Point)` — paralelo `CubicTo` mas com 1 control point | Simétrico arquitecturalmente | — |
| **(b)** `QuadraticTo { control: Point, end: Point }` — struct variant nomeado | Nomes explícitos | Inconsistente com `CubicTo` tuple variant |
| **(c)** `QuadraticTo(Point, Point)` — paralelo `CubicTo(Point, Point, Point)` posicional | Convenção idêntica | Sem nomes — risco ambiguidade |

Default sugerido: **(a)** se `CubicTo` tem campos nomeados (struct
variant); **(c)** se `CubicTo` é tuple variant posicional. Decisão
**estructuralmente forçada por A.1.2**.

**Não-trivial em si** — diferentemente de P293 H6 (que reusou
variant existente), aqui há decisão estructural genuína.

### A.3 — Integração com `path_bbox` e Layouter

| Opção | Comportamento |
|---|---|
| **(α)** `path_bbox` (P277) estende-se para `QuadraticTo` — algoritmo análitico paralelo ao cubic | Paridade arquitectural |
| **(β)** `QuadraticTo` reduz a `CubicTo` interno (control quadratic → 2 controls cubic via fórmula matemática) | Reuso máximo; sem novo algoritmo bbox |

Default sugerido: **(α)** salvo se A.1.5 revelar que (β) é vanilla
typst paradigm.

**(β)** tem precedente histórico — TrueType fonts armazenam outlines
em quadratic Bézier mas PDF emit-os como cubic; típicamente fontes
desktop fazem esta conversão. Verificar A.1.7 se vanilla também.

### A.4 — Impacto em emit (alteração justificada ADR-0098)

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** Match arm novo `PathItem::QuadraticTo(c, e) => write!("{} {} {} {} v\n", ...)` ou `y` | Hash muda intencionalmente; **ramos existentes preservados bit-exact** |
| **(ii)** Conversão para cubic antes do emit (via β em A.3) | Hash preservado se conversão ocorre em layout-time, não emit-time |

Default sugerido: **(i)** se A.3 → (α); **(ii)** se A.3 → (β).

**Crítico**: se opção (i), §A.4 deve documentar **literalmente**:
- Que bytes mudam.
- Que ramos pré-existentes (cubic/line/move/close) produzem PDF
  byte-exact em ausência de quadratic.
- ADR-0098 §"alterações justificadas" é a clausura que cobre.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- Quadratic com control point coincidente com start ou end (degenerate).
- Quadratic com control fora da linha start-end (caso normal).
- Quadratic com control colinear (caso degenerate visual).
- Conversão quadratic → cubic (se A.3 → β) preserva geometria.
- Sequência mista `move + quadratic + cubic + close` produz PDF coerente.

### A.5' — Verificação anti-reflexão (N=4 do padrão §8.6)

Reaplicação P291+P292+P293. **N=4 cumulativo** — aproxima limiar
tentativo N≥3-4 mas pré-P294 já estava N=3 em P293 não-promovido.

4 verificações:

1. **Comparação literal A.1.6 P288-P294** — esperar paradigma
   genuinamente novo (P294 é "materialização de variant novo +
   emit novo paralelo a sequência existente"):
   - P288 lang: cross-module.
   - P289 weight: TextStyle method.
   - P290 tracking: per-glyph + Tc emit.
   - P291 leading: per-line via peek.
   - P292 font: FontBook indirect resolution.
   - P293 cubic: activação posterior variant inerte.
   - **P294 quadratic: materialização nova de variant + emit
     paralelo (NÃO activação posterior)**.
2. **A.0 produzido empiricamente** — em P294 inclui análise
   explícita de mudança de hash justificada.
3. **Elementos estructuralmente novos identificados**:
   - **A.0 não-trivial** (hash muda) — primeira vez na sequência.
   - **P294 ≠ H6 reaplicado** — explícito.
   - **A.2 decisão estructural** — variant novo.
   - **A.3 escolha (α) vs (β)** — quadratic-as-cubic ou nativo.
4. **§8.7' A.0.0 template reaplica N=2**:
   - P293 inaugurou.
   - P294 reaplica com **decisão genuína anti-H6**.
   - Aproxima N≥3 tentativo para promoção ADR meta — registar.

**Promoção ADR meta condicional**:
- §8.7' N=2 reaplicação — aproxima limiar.
- §8.6 N=4 — aproxima limiar.
- §8.3 N=6 — P293 refutou; pode reaplicar se A.0 alteração
  justificada conta como refutação genuína.
- **Uma ADR meta por passo no máximo** (P273.17 §0). Se múltiplas
  disparem, escolher uma — preferir §8.7' (gatilho cumulativo mais
  recente) ou §8.6 (mais evidência cumulativa).

---

## §3 — Materialização

Após Fase A:

1. Adicionar `QuadraticTo(...)` ao enum `PathItem` per A.2.
2. Estender todos os match arms exhaustive identificados em A.1.3:
   - Layouter (`path_bbox`, transformações).
   - Emit (`export.rs`).
   - Tests existentes (se algum faz match sobre `PathItem`).
3. Implementar emit per A.4 (escolha (i) ou (ii)):
   - Se (i): novo bloco match arm em cada sítio de emit cubic
     (linhas 2375/2457/2629 esperadas).
   - Se (ii): conversão em layout-time; emit inalterado.
4. Estender `path_bbox` (P277) per A.3:
   - Se (α): algoritmo análitico quadratic (mais simples que
     cubic — derivada é linear).
   - Se (β): conversão para cubic; reusa algoritmo existente.
5. Activar `quadratic` em `native_curve` (P293) — remover scope-out
   `Err`:
   ```rust
   "quadratic" => path_items.push(PathItem::QuadraticTo(
       point_from(&arr[1])?,
       point_from(&arr[2])?,
   )),
   ```
6. Testes:
   - L1 unitário variant: ctor; PartialEq.
   - L1 unitário catalog: 4 → 5 PathItem variants.
   - L1 unitário match exhaustive: defesa compilador.
   - L1 unitário `path_bbox`: quadratic bbox calculation paridade
     vanilla (5 cenários A.5).
   - L1 unitário `native_curve`: `"quadratic"` activado retorna
     `Ok(Content::Shape{...QuadraticTo...})`.
   - L3 integração: `#curve(("move", (0, 0)), ("quadratic", (5, 10),
     (10, 0)))` produz PDF com `v` (ou `y`) operator.
   - L3 regression bit-exact: PDFs pré-P294 (cubic, line, polygon)
     produzem bytes idênticos pós-P294.
   - **L3 hash export.rs**: comparar `before_hash` vs `after_hash` —
     muda; documentar.
7. Promoção ADR meta condicional per A.5':
   - **Sem promoção** se A.5' identifica paradigmas mas nenhum
     atingiu limiar claro N≥3 não-trivial cumulativo.
   - **Considerar §8.7' N=2** se A.0.0 reaplicação for genuinamente
     usada (não mecânica) — mas N=2 ainda longe N≥3.
   - **Default: sem promoção**.
8. Actualizar L0:
   - `entities/geometry.md` (PathItem +1 variant).
   - `infra/export.md` (se existir secção curve emit).
   - Propagar hashes.
   - Tabela A.7 linha 194 — nota adicional sobre `quadratic`
     activado.

**Sem caps** (per P282 §7). Estimativa de testes: ~15-20.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P293: 2 802 testes.
  Esperado: ~2 817-2 822.
- `crystalline-lint` zero violations.
- Hash L0 `geometry.md` muda (+1 PathItem variant).
- Hash L0 `stdlib.md` **preserved** (política única).
- **Hash L0 `export.rs` muda intencionalmente** — **1ª quebra
  desde P281** (10 passos preservaram: P282-P293):
  - Documentar mudança em diagnóstico + relatório + footnote
    cobertura.
  - Validar com hash de PDFs de teste para cubic/line/polygon
    pre-P294 vs pós-P294 → bytes idênticos para ramos não-quadratic.
  - Confirma ADR-0098 §"alterações justificadas".
- **Regressão bit-exact validada** para call sites sem quadratic.
- Tabela A.7 linha 194 nota actualizada.
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **§8.7' reaplica N=2 sem promoção**.
- **§8.6 atinge N=4 sem promoção**.
- Bug latente colateral (se descoberto em A.5) registado.

---

## §5 — Não-objectivos

- **Não** materializar `native_quadratic` standalone — `native_curve`
  P293 já é o constructor universal; activar `"quadratic"` kind
  basta.
- **Não** alterar paradigma cubic emit existente. P294 **acrescenta**
  ramo paralelo; **não substitui**.
- **Não** estender `path_bbox` para outras operações (curve length,
  intersection) — fora de scope.
- **Não** materializar `curve.move`/`curve.cubic`/`curve.quadratic`
  como scope methods (sintaxe vanilla fiel). Bloqueado por scope
  methods em stdlib — frente independente.
- **Não** materializar parser SVG path string. Frente vanilla
  separada per P293 §9.
- **Não** promover ADR meta sem gatilho genuíno. §8.7' N=2 e §8.6
  N=4 são candidatos mas ainda não disparam claramente.
- **Não** confundir `QuadraticTo` cristalino (control point único
  vanilla typst) com TrueType quadratic (control points implícitos
  via interpolação) — fora de scope.

---

## §6 — Pendências relacionadas

Resolve:
- **P293 §9 frente `native_quadratic` / `PathItem::QuadraticTo`** —
  scope-out explícito P293 → activado P294.

Não resolve (continua aberto):
- `curve.move`/`curve.cubic`/`curve.quadratic` scope methods —
  bloqueado por scope methods stdlib.
- `native_path` parser SVG — frente independente.
- Outras operações Bézier (length, intersection, subdivide) —
  passos próprios.
- `path_bbox` para sequências mistas complexas (já testadas em
  P277 + P293; verificar A.3 se P294 expõe edge cases novos).

---

## §7 — Risco residual

Risco principal: **A.4 → (i)** quebra hash `export.rs` pela 1ª vez
desde P281. Mitigação: ADR-0098 §"alterações justificadas" cobre
explicitamente; validação bit-exact para ramos não-quadratic
**obrigatória** em critério de fecho. Não é regressão.

Risco secundário: A.3 → (β) (conversão quadratic → cubic) introduz
divergência numérica vs vanilla typst. Mitigação: A.5 testes
fronteira; se divergência > epsilon razoável, escolher A.3 → (α).

Risco terciário: `path_bbox` (P277) algoritmo para cubic não
generaliza trivialmente para quadratic. Mitigação: quadratic
bbox é **mais simples** que cubic (derivada é linear, não
quadrática) — A.3 → (α) implementação directa.

Risco quaternário: ADR-0099 N=10 "activação posterior" aplica-se
**parcialmente** a P294. Activar `quadratic` em `native_curve` é
activação posterior do scope-out P293; mas adicionar variant
novo + emit novo é **materialização**, não activação. Esta dupla
natureza pode confundir interpretação cumulativa. Mitigação: A.5'
explicita as duas componentes; relatório registar.

Risco quinário: gatilhos meta múltiplos (§8.7' N=2, §8.6 N=4, §8.3
N=6 reaplicável) podem disparar simultâneamente. Mitigação: §5
não-objectivo "uma ADR meta por passo"; se ambíguo, **adiar todas**
e registar em relatório para passo subsequente.

Risco senário: A.0.0 reveal que variant ou emit **já existem** parcialmente
(improvável dado P293 §9 explicit; mas verificar). Mitigação:
inspecção literal antes de materializar.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/geometry.rs`
  (`PathItem` enum, 4 variants pós-P293).
- Constructor: `01_core/src/engine/stdlib/shapes.rs:native_curve`
  (P293 §3.1).
- Emit: `03_infra/src/export.rs:2375/2457/2629` (cubic emit P293
  identificado).
- `path_bbox`: P277 (Bézier bbox analítica CLOSED).
- Vanilla: `lab/typst-original/crates/typst-library/src/visualize/curve.rs`.
- Precedente directo cubic: **P293**.
- ADR aplicável: **ADR-0098** §"alterações justificadas" + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded (justifica scope-out outras Bézier
  operations).
- Padrão §8.7' A.0.0 template (P293 inaugural; P294 N=2).

---

*Spec P294 produzida 2026-05-19 pós-P293 (P-curve-geometry fechado;
H6 inaugural; PathItem::CubicTo activado). Frente
`P-quadratic-curve` — **extensão directa P293 mas estructuralmente
diferente**: P293 foi activação posterior (variant + emit
existentes inertes); **P294 é materialização from-scratch** (variant
+ emit ausentes). Diferença factual confirma que P294 ≠ rubber-stamp
de P293. Fase A com **7 secções** A.0.0+A.0-A.5+A.5'. **A.0
não-trivial pela 1ª vez na sequência cirúrgica P282-P294**: hash
`export.rs 66cb8ac3` **muda intencionalmente** — 1ª quebra desde
P281. Não é regressão — ADR-0098 §"alterações justificadas" cobre
adição de operator PDF novo (`v`/`y`) paralelo ao cubic existente
(`c`). Critério de fecho **obrigatório**: validação bit-exact para
ramos não-quadratic (cubic/line/move/close) preserva bytes;
**apenas o ramo quadratic** introduz mudança. **§8.7' A.0.0
template reaplica N=2** (não promovido; limiar N≥3 não atingido).
**§8.6 atinge N=4** (não promovido). Honestidade epistémica: P294
tem **dupla natureza** (activação do scope-out P293 + materialização
de variant/emit novos); A.5' explicita. Sem caps LOC ou magnitude
(P282 §7).*
