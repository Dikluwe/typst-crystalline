# Passo 291 — `Style::Leading` variant

**Frente**: `P-style-leading-variant` (rank 1 do relatório P290 §9;
P290.1 candidato).
**Origem**: P290 §5.6 — assimetria residual 2/5 fechados; restam
`leading`/`font`. P291 = P290.1 (sequência directa).
**Pré-requisitos**: nenhum bloqueador.
**Origem secundária**: Tabela B.3 lista 8 variants pós-P290 (sem
`Leading`). Tabela A.3 linha 70 lista `#set par(...)` como
`parcial` P138 com nota "`leading` capturado em text por
conveniência; sem `par` propriamente". Tabela B.4 linha 352 lista
`StyleDelta.leading` como `implementado` desde P138.

---

## §1 — Objectivo

Adicionar variant `Style::Leading(Length)` ao enum `Style` em
`01_core/src/entities/style.rs`, integrar com a cascade existente
(parse `#set text(leading: 0.8em)` → `Style::Leading(...)` →
acumula em `StyleDelta.leading`), e fechar 1/2 da assimetria
residual P290 §5.6 (2 fields → 1 restante pós-P291: apenas
`font`).

Razão de ser:

1. **Fecha 1/2 da assimetria residual** — `leading` é o 1º dos 2
   fields restantes. Pós-P291 resta apenas `font`.
2. **Reaplicação de ADR-0099 sem promoção** — limiar N=5 atingido
   em P289; este passo é **N=7 cumulativo do padrão** sem
   promoção adicional.
3. **Aplicação directa de ADR-0098** — Fase A.0 obrigatória.
   **A.0 prevista trivial** (zero hits prováveis em `export.rs`
   per P290 §5.3 e §9: *"leading é layout-time only, sem `Tc`
   operator equivalente"*) — mas verificar **literalmente**, não
   assumir.
4. **Confirmar ADR-0098 vigente pelo 8º passo consecutivo** se A.4
   → (i) — hash `export.rs 66cb8ac3` preservado.
5. **Mitigação activa do risco "sequência reflexa"** (P290 §7
   risco quinário). P291 é o 4º passo cumulativo paralelo da série
   P288-P291 — patamar onde a automatização sem reflexão fica
   visível. Esta spec inclui §2.5 dedicada à protecção activa
   contra rubber-stamping.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; protecção activa anti-reflexão)

Cinco secções A.0-A.5 paralelas a P290, **mais uma sub-secção A.5'
nova** dedicada à verificação de que a Fase A não é rubber-stamp
do P290.

### A.0 — Potencial de reuso ADR-0098 (esperado trivial; verificar literalmente)

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "leading" 03_infra/src/export.rs` | **Zero hits funcionais** (per P290 §5.3 antecipação) | Listar todos os hits; classificar |
| `FrameItem::Text` precisa de novo field `leading`? | Esperado **não** — leading é layout-time only | Inspecção literal |
| `Frame` ou `Region` consultam leading directamente? | Possível — leading afecta vertical spacing entre frames | Verificar A.1.5 |
| Hash `export.rs` esperado | **Preservado bit-exact** pelo 8º passo se A.0 confirmar antecipação | Verificação final |

**Diferença material vs P290 A.0**: P290 esperava hits e
descobriu 2 via `style.tracking`. P291 espera zero hits e deve
**confirmar empiricamente** (não assumir per P290 §5.3). Se hits
aparecerem, classificar como em P290:
- Via `FrameItem::Text.style.leading` ou `Frame.layout.leading`
  (paradigma P136 — OK).
- Via `chain.leading()` directo em emit (violação nominal
  ADR-0098).

**Caso especial leading**: vertical spacing entre linhas pode
manifestar-se em emit como diferença em coordenadas Y entre
`FrameItem::Text` consecutivos. **Não é** consumo de leading em
emit — é consumo no Layouter que produz coordenadas distintas
em `FrameItem::Text`. Esta distinção deve estar explícita em A.0.

### A.1 — Inventário literal do caminho actual `leading`

8 sub-secções paralelas a P290 §A.1, **com inspecção genuína nova**:

1. **A.1.1 — `Style` enum pós-P290** — confirmar 8 variants.
2. **A.1.2 — `StyleDelta.leading`** — verificar tipo (esperado
   `Option<Length>` per Tabela B.4 linha 352).
3. **A.1.3 — `push_styles` cascade** — match exaustivo sobre 8
   variants; `Style::Leading(Length)` adiciona 9º arm.
4. **A.1.4 — `delta.leading` write site único actual** — esperar
   `eval/rules.rs:???` (precedente P138).
5. **A.1.5 — `delta.leading` read sites** — `chain.leading()`
   walk up-the-chain. **Verificar onde**: Layouter consome para
   calcular vertical advance? Em que ponto exacto do flow?
6. **A.1.6 — Consumers actuais** — listar **literalmente**, não
   assumir paralelo a P290. Esperar:
   - Layouter consumer P138 (advance entre linhas/parágrafos).
   - `TextStyle.leading` capture? — **verificar literalmente**
     (P138 pode ter implementado diferente de P137; leading é
     conceptualmente "between glyphs vertically", tracking é
     "between glyphs horizontally" — paradigmas distintos
     possíveis).
7. **A.1.7 — `FrameItem::Text`/`Frame` emit** — verificar
   literalmente `grep "leading" export.rs`.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente,
   **não copy-paste de P290**. Se for idêntico, registar a
   redundância como evidência factual; se divergir, registar a
   divergência.

### A.2 — Estrutura do variant `Style::Leading`

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Leading(Length)` — paralelo P290 `Tracking(Length)` | `Length` é `Copy` (P127); `Style: Copy` preserved | Trivial paralelo — risco "rubber-stamp" se A.1.2 confirmar |
| **(b)** `Leading(f64)` — armazena valor em pontos | Storage raw | Perde semântica `Em` vs `Pt` |
| **(c)** `Leading(LengthVariant)` — wrapper | Domínio restrito | Indirecção sem ganho |

Default sugerido: **(a)** — paridade com `StyleDelta.leading`.
**Verificar A.1.2 literalmente**: se `StyleDelta.leading: Option<Length>`
confirmado, (a) é a escolha estructuralmente forçada (não preferência).

**Atenção anti-reflexão**: se A.1.2 revelar tipo diferente (e.g.
`Option<Em>` ou `Option<f64>`), opção (a) está estructuralmente
errada — registar e adaptar. **Não assumir paralelo a tracking
sem verificação**.

### A.3 — Integração com `StyleDelta`

| Opção | Comportamento |
|---|---|
| **(α)** `delta.leading = Some(*l)` — last-write wins | Paridade com 8 arms anteriores |
| **(β)** Merge específico (e.g. leading multiplicativo) | Improvável; vanilla é last-write |

Default sugerido: **(α)** — sem evidência empírica contra.

### A.4 — Impacto em `FrameItem::Text`/`Frame` e emit

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** Leading consumido apenas no Layouter para calcular vertical advance entre `FrameItem::Text` consecutivos; emit lê coordenadas Y resultantes (não leading directo) | `export.rs` **preservado bit-exact** (8º passo consecutivo) → ADR-0098 vigente |
| **(ii)** `FrameItem::Text` ou `Frame` consulta leading directamente em emit | Improvável — leading não tem analogon directo de `Tc` operator |
| **(iii)** Híbrido — reflector trivial | Caso edge |
| **(iv)** Emit lê `chain.leading()` directamente — violação | Improvável; investigar se ocorrer |

Default sugerido: **(i)** se A.0 confirmar zero hits.

**Implicação gatilho meta**: nenhuma ADR meta promovida em P291.
ADR-0099 reaplica (N=7 cumulativo); ADR-0098 reaplica (N=8
cumulativo). §8.5 atinge N=4 mas **continua desqualificado** per
P290 §8.5 — anti-inflação.

### A.5 — Detecção de bugs latentes (padrão P288 §8.4)

Cenários fronteira:

- `Length::pt(0.0)` — leading zero (linhas colapsam? texto sobrepõe?).
- `Length::pt(11.0)` — leading próximo do default (typst default
  é ≈ 0.65em — verificar A.1).
- `Length::em(0.5)` — leading em em-units relativo.
- `Length::pt(50.0)` — leading grande (linhas muito espaçadas).
- `Length::pt(-1.0)` — leading **negativo**: vanilla typst aceita?
  Comportamento esperado é colapso/sobreposição. Verificar paridade.

**Atenção particular ao leading negativo** — vanilla typst pode
saturar a 0 (não permitir negativo). Verificar empiricamente
durante A.5.

### A.5' — Verificação anti-reflexão (NOVA, P290 §7 risco quinário)

P290 §7 registou risco quinário: *"P290 e seus sucessores
P290.1/P290.2 entrarem em 'sequência reflexa' — automatização sem
reflexão genuína."* P291 é a primeira spec onde esta mitigação se
manifesta activamente.

Sub-secção A.5' adicional **obrigatória**:

1. **Comparar literalmente os diagnósticos A.1.6 de P288 + P289 +
   P290 + P291 (este)** — se forem todos cópias estruturais com
   substituição mecânica de nome, registar como evidência factual
   de que a sequência atingiu o seu limite metodológico genuíno.
2. **Verificar que A.0 produziu hits/zero-hits empiricamente** —
   não apenas citou a antecipação de P290 §5.3. Se A.0 apenas
   confirmar a antecipação sem inspecção genuína, registar a
   degenerescência.
3. **Identificar pelo menos 1 elemento estructuralmente novo em
   P291 vs P290** — pode ser:
   - Comportamento de A.5 com leading negativo (saturação? aceite?
     diferente de tracking negativo P290).
   - Caminho de consumo de leading no Layouter (vertical advance
     entre frames, não dentro de glyph runs como tracking).
   - Divergência arquitectural registada (Tabela A.3 linha 70:
     "leading capturado em text por conveniência" — não está em
     par.rs como em vanilla).
4. **Se nada estructuralmente novo emerge**, registar honestamente
   em diagnóstico A.5' e **considerar adiar P292 (font)** para
   passo ortogonal anterior (math-accent-cancel ou outro) — quebra
   da sequência cumulativa para evitar inflação reflexa.

---

## §3 — Materialização

Após Fase A (incluindo A.5' anti-reflexão) produzir todos os
inventários:

1. Adicionar `Leading(Length)` ao enum `Style`.
2. Implementar match-exhaustive arm em `StyleChain::push_styles`:
   `Style::Leading(l) => delta.leading = Some(*l)`.
3. Match exaustivo continua — defesa cumulativa P288-P291.
4. Testes:
   - L1 unitário variant: ctor; PartialEq.
   - L1 unitário cascade: `push_styles` projecta no delta.
   - L1 unitário injection: `Content::Styled(body,
     Styles::from_iter([Style::Leading(Length::em(0.65))]))`.
   - L1 unitário last-write: 2 `Style::Leading` consecutivos.
   - L1 unitário catalog: 8 → 9 variants.
   - L1 unitário fronteiras: 5 cenários A.5.
   - L3 integração: verificar que dois `FrameItem::Text`
     consecutivos têm Y-delta consistente com leading aplicado.
   - L3 integração regression: PDFs pré-P291 (P138 parse-driven)
     produzem bytes idênticos pós-P291.
5. Aplicação ADR-0098 obrigatória — diagnóstico A.0 documenta
   verificação empírica.
6. Promoção ADR meta: **não promover**. Razões em §5.
7. Actualizar L0:
   - `00_nucleo/prompts/entities/style.md` (+9º variant).
   - Tabela B.3 — `Leading(Length)` 9º variant.
   - Tabela B.4 linha 352 — nota cruzada P291.
   - Propagar hashes.
8. Actualizar diagnóstico:
   - `diagnostico-style-leading-passo-291.md` com 5 secções
     A.0-A.5 + secção A.5' anti-reflexão.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-12 (paralelo
P290 +11; expectativa similar).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P290: 2 773 testes.
  Esperado: ~2 781 a ~2 785.
- `crystalline-lint` zero violations.
- Hash L0 `style.md` muda (+1 variant em B.3).
- Hash L0 `content.md` **preserved**.
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `export.rs` **condicional**:
  - Preserved se A.4 → (i) → confirma ADR-0098 vigente; **8º passo
    consecutivo** (P282+P285+P286+P287+P288+P289+P290+P291).
  - Muda se A.4 → (ii)/(iv) → investigação obrigatória.
- **Regressão bit-exact validada** — todos os testes leading-aware
  pré-P291 (P138 parse-driven) verdes byte-exact.
- Tabela B.3 actualizada com `Leading(Length)` (9º variant) + nota
  assimetria residual (1 field restante: `font`).
- Tabela B.4 linha 352 com nota cruzada P291.
- Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5+**A.5'** produzido.
- **Sem promoção ADR meta nova**.
- Bug latente colateral (se descoberto em A.5) registado e fixado.
- **A.5' anti-reflexão produzida** — relatório regista
  honestamente se a sequência atingiu o seu limite metodológico
  (decisão sobre P292 condicional ao resultado de A.5').

---

## §5 — Não-objectivos

- **Não** materializar `Style::Font` simultaneamente. P291 é
  cirúrgico (1 variant); P292 (font) é passo próprio **condicional
  ao resultado de A.5'**.
- **Não** promover nova ADR meta. ADR-0098 + ADR-0099 cobrem;
  §8.5 desqualificado per P290; §8.3 N=5 estável.
- **Não** alterar relação arquitectural `text`↔`par` para
  `leading`. Tabela A.3 linha 70 regista divergência consciente
  ("leading capturado em text por conveniência"); preservar.
- **Não** validar input leading range. Vanilla typst pode aceitar
  ou saturar negativo — paridade preservada (verificar A.5).
- **Não** materializar `Style::Region`/`Style::Script` ou outros
  styles `text` ainda ausentes (Tabela A.3 linhas 96-98).
  Bloqueados por shaping XL.
- **Não** alterar `StyleDelta.leading` semanticamente.
- **Se A.5' revelar sequência reflexa**, **não materializar P292
  imediatamente**. Quebrar sequência com passo ortogonal.

---

## §6 — Pendências relacionadas

Resolve:
- **Assimetria Tabela B.3 vs B.4 para `leading`** — 1/2 da
  assimetria residual P290 §5.6.

Não resolve (continua aberto):
- `font: Option<FontList>` — passo próprio candidato P292
  (condicional a A.5').
- Show rules sobre leading — bloqueada por regex em L1.
- `Content::Par` propriamente (Tabela A.3 linha 184 `parcial`) —
  passo distinto.

---

## §7 — Risco residual

Risco principal (NOVO P291): **sequência reflexa materializada**.
P291 é o 4º passo cumulativo paralelo (P288+P289+P290+P291). Se
A.5' confirmar que a Fase A é rubber-stamp:

1. Materializar P291 com honestidade explícita (registar em
   relatório que a sequência atingiu o seu limite metodológico).
2. **Adiar P292 (font)** para passo posterior; intercalar passo
   ortogonal antes.
3. Considerar formalização explícita do padrão §8.5 se A.5'
   revelar que P288-P291 são genuinamente "reaplicação não-trivial"
   (improvável dado P290 §8.5 desqualifica) ou desqualificá-lo de
   forma estável.

Risco secundário: A.0 antecipação (zero hits) revela-se falsa —
leading tem consumo emit não previsto. Mitigação: A.0 é
verificação literal, não assumpção; se hits aparecerem, classificar
per paradigma P290.

Risco terciário: leading negativo causa bug. Mitigação: A.5 inclui
cenário; verificar paridade vanilla (saturar a 0 ou aceitar
sobreposição).

Risco quaternário: A.1.2 revela tipo de `StyleDelta.leading`
diferente do esperado `Option<Length>`. Mitigação: opção A.2 (a)
condicional a A.1.2; se tipo diferente, adaptar variant
estructura.

Risco quinário (anti-meta): A.5' detecta degenerescência mas
relator decide ignorá-la e continuar P292 mecânicamente. Mitigação:
critério de fecho §4 inclui A.5' como obrigatória; relatório deve
documentar a decisão. P273.17 §0 anti-padrão estende-se de
"over-formalização" a "over-automatização" — invariante cultural
implícito.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/style.rs` (`Style` enum,
  8 variants pós-P290).
- Tipo relacionado: `StyleDelta.leading: Option<Length>` (P138).
- Caminho parse `#set text(leading: ...)`: P138 (Tabela A.3 linha
  70 + linha 184); precedente directo.
- Caminho consumer Layouter: P138 (vertical advance entre linhas).
- Precedente directo (mesma assimetria): P288 (`Style::Lang`) +
  P289 (`Style::Weight`) + P290 (`Style::Tracking`).
- Divergência arquitectural consciente: Tabela A.3 linha 70 —
  "leading capturado em text por conveniência; sem `par`
  propriamente". Linha 184: `par` é `parcial`. P291 preserva esta
  divergência (não-objectivo §5).
- ADR aplicável: **ADR-0098** + **ADR-0099** (ambas vigentes
  pós-P289).
- ADR processual: ADR-0065 (inventariar-primeiro; 5+1 secções
  A.0-A.5 + A.5').
- ADR cultural: P273.17 §0 (anti-padrão over-formalização **e**
  anti-automatização implícita); P290 §7 risco quinário (sequência
  reflexa).
- ADR estilo: ADR-0038 (`Style` enum divergência intencional);
  ADR-0040 (`#set text` activation P102); ADR-0054 (`leading`
  graded — mas P138 marca `implementado`; verificar A.1).
- Padrão §8.5 P289 — P291 atinge N=4 mas continua desqualificado
  per P290.

---

*Spec P291 produzida 2026-05-19 pós-P290 (assimetria 3/5 fechada;
ADR-0098 validada com emit consumer real). Frente
`P-style-leading-variant` — extensão simétrica do enum `Style` com
`Leading(Length)` paralela a P288/P289/P290. Fecha 1/2 da
assimetria residual P290 §5.6 (1 field restante pós-P291: `font`).
Fase A obrigatória com **6 secções**: A.0-A.5 paralelas a P290
**+ A.5' nova obrigatória** dedicada à mitigação activa do risco
"sequência reflexa" registado em P290 §7 risco quinário. Critério
de fecho condicional em A.4 + A.5': se A.4 → (i) (esperado),
hash `export.rs 66cb8ac3` preservado pelo 8º passo consecutivo →
confirma ADR-0098 robusta. **Sem promoção ADR meta** — ADR-0098
(N=8 cumulativo) e ADR-0099 (N=7) reaplicam sem promoção; §8.5
N=4 mas desqualificado per P290. Diferença material vs P290:
**A.5' detecta sequência reflexa** se a Fase A degenera em
rubber-stamp; se positivo, P292 (font) **adiado** para passo
ortogonal intercalado. Honestidade epistémica preservada: variant
atómico (não rico); P273.17 §0 estende-se a "anti-automatização"
implicita. Divergência arquitectural consciente preservada
(`leading` em `text` não em `par` per Tabela A.3 linha 70). Sem
caps LOC ou magnitude (P282 §7).*
