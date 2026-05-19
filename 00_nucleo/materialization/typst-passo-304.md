# Passo 304 — `P295.1` nota corpo no rodapé via 2-pass layout

**Frente**: `P295.1 — nota corpo renderizada no rodapé da página`
(pendente desde P295 §8; prioridade #1 P300; reaparece P301-P303).
**Origem**:
- P295 materializou `Content::Footnote` Fase 1 (marker `[N]`
  superscript apenas).
- P295 §3.5 + §8 + §A.5' antecipou P295.1 como sub-passo dedicado
  para body rendering no rodapé.
- P295 §4 teste `p295_footnote_body_nao_renderizado_no_pdf_fase1`
  documenta o gap actual (body armazenado mas ausente do PDF).
- P300 §8 prioridade #1; reaparece em P301/P302/P303 §9.
**Pré-requisitos**: P295 (`Content::Footnote` variant +
`native_footnote` stdlib + walker counter Layouter).
**Tipo declarado**: **1ª magnitude L** pós-cluster math
P296-P298; **1ª aplicação de 2-pass layout** desde P233/P234.
Paradigma genuinamente novo na série P283+.
**Marco numérico**: pendência de **9 passos** (desde P295);
mas critério é factual, não pendência longa (lição P300 §10).

---

## §1 — Objectivo

Materializar renderização do body de `Content::Footnote` no
rodapé da página correspondente ao marker `[N]` inline,
estendendo o Layouter para suportar 2-pass measure→place de
footnotes single-page. P295.2 (overflow multi-página) permanece
sub-passo separado.

### §1.1 — Estado factual pré-P304

**Pós-P295 (Fase 1)**:
- `Content::Footnote { body: Box<Content> }` armazenado.
- Layouter consumer arm produz marker `[N]` superscript via
  walker counter monotónico.
- **Body é descartado** após emit do marker — não renderizado
  no rodapé.
- Teste `p295_footnote_body_nao_renderizado_no_pdf_fase1`
  documenta gap.

**Vanilla typst**:
- Body renderizado no rodapé da página onde o marker aparece.
- Linha horizontal separadora opcional.
- Espaço reservado decrescente para body conforme markers
  acumulam.
- Overflow para páginas subsequentes (P295.2 separado).

**Gap material P304**: conectar body armazenado com rendering
no rodapé via 2-pass layout.

### §1.2 — Precedentes 2-pass no cristalino

| Precedente | Mecanismo | Aplicação |
|---|---|---|
| **`outline()` P65-66** | 2-pass introspection | TOC com page numbers |
| **P233** | two-pass measure→place inaugural N=1 | Layout cluster (Grid/Table per cobertura) |
| **P234** | extensão pattern two-pass | Sub-padrão N=2 cumulativo |

**Sub-padrão "two-pass measure→place"** cumulativo: **P233 N=1
inaugural + P234 N=2** = **N=2 pós-P234**. P304 reaplicação =
**N=3 candidato** se A.0.0 confirma reuso genuíno do pattern.

### §1.3 — Decisão arquitectural fundamental

Footnote 2-pass tem características distintas vs P233 Grid/Table:

| Aspecto | P233 measure→place | P304 footnote |
|---|---|---|
| Constraint principal | Cell sizing dado regions | Body fits no rodapé reservado |
| Pass 1 | Medir todas as células | Medir todos os footnote bodies pendentes |
| Pass 2 | Place com sizes resolvidos | Place markers + bodies no rodapé |
| Page break interaction | Independente | Crítica (body pertence à página do marker) |

**A.0.0 verifica se P233/P234 pattern é reaplicável ou se P304
requer infraestrutura nova**.

### §1.4 — Riscos e razões

**Riscos**:
- Magnitude L pode revelar-se M+/XL se 2-pass infraestrutura
  insuficiente.
- Footnote 2-pass tem **page break coupling** que P233 não tem.
- Pressão de "fechar pendência 9 passos" pode inflacionar
  decisões (P300 §10 lição).

**Razões**:
1. **Pendência registada explícita P295 §8** desde 9 passos.
2. **Primeira magnitude L pós-cluster math** — quebra natural
   do viés magnitude pequena P302/P303.
3. **Reaplicação sub-padrão "two-pass measure→place" N=3**
   candidata — limiar tentativo para promoção.
4. **Paradigma genuinamente novo** — 13º paradigma distinto na
   sequência se A.0.0 confirma 2-pass footnote-aware é
   estrutural novo.
5. **Reaplicação ADR-0098 + ADR-0099** — N=21 + N=16 cumulativos
   condicionais.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 8 secções)

A.0.0 + A.0.0' (subdivision se complexidade revela) + A.0-A.5
+ A.5'.

### A.0.0 — Verificação literal estado 2-pass + footnote (N=11 cumulativo §8.7')

Inspecção literal:

1. **`grep -rn "two.pass\|measure_then_place\|two_pass" 01_core/`** —
   localizar implementações 2-pass actuais.
2. **Inspeccionar P65-66 `outline()`** — mecanismo 2-pass
   introspection; reaplicável a footnote?
3. **Inspeccionar P233/P234** — sub-padrão "two-pass measure→place"
   estructura; reaplicável a footnote?
4. **Inspeccionar `Content::Footnote` consumer arm pós-P295** —
   sítio onde marker é emitido; body actualmente descartado.
5. **Inspeccionar Layouter page break mechanism** — como cristalino
   actual decide page breaks; pode reservar espaço rodapé?
6. **Inspeccionar `Introspector::query` (P208C)** — pode ser
   usado para coletar todos os `Content::Footnote` antes de
   layout?
7. **Inspeccionar vanilla typst footnote**:
   `lab/typst-original/.../model/footnote.rs` + integration
   com layouter.
8. **Cross-check com `outline()` 2-pass**: como outline reserva
   espaço para entries; aplicável a footnote rodapé?

**Decisão A.0.0 sobre hipóteses**:

| Hipótese | Acção |
|---|---|
| **HU** (P233/P234 pattern directamente reaplicável) | P304 procede com reuso máximo; magnitude L confirmada |
| **HV** (2-pass existe mas footnote requer extensão) | P304 procede com refactor minor; subdivisão A.0.0' considerável |
| **HW** (2-pass insuficiente; nova infraestrutura) | P304 magnitude M+; abrir P304.0 dedicado para infraestrutura |
| **HX** (subset reduzido viável: single-page only) | P304 single-page; P295.2 overflow separado |

**Default sugerido**: **HV ou HX** — extensão minimal de
infraestrutura existente. **HW** rejeitado salvo evidência
empírica clara.

**Magnitude esperada A.0.0**: **alta** — primeira modificação
estrutural significativa em Layouter na série P283+. P233/P234
precedente existe mas footnote tem page break coupling distinto.

### A.0.0' — Decisão de subdivisão (paralelo P299/P302)

| Subset | Scope | Magnitude |
|---|---|---|
| **P304.A** | Single-page only (sem overflow); reuso P233 pattern | M |
| **P304.B** | A + reserved space dynamic conforme markers | M+ |
| **P304.C** | A + B + separator line (linha horizontal entre body e rodapé) | M+ |
| **P304.D** | A + B + C + customization (height, separator style) | L |

**Default sugerido**: **P304.A** — minimal viable; P304.B/C/D
em sub-passos.

**Permitir interrupção honesta** se A.0.0 revelar P304.A já é
mais que M; abrir P304.0 dedicado para infraestrutura.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado |
|---|---|
| `grep "Footnote.*body\|footnote_body" 03_infra/src/export.rs` | **Zero hits** — emit agnóstico esperado |
| `FrameItem::Text` suporta posicionamento no rodapé arbitrário? | **Sim** — posicionamento Y livre |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo **21º passo consecutivo** se body é layout-time only |

**Risco**: se P304 requer novo `FrameItem` variant para footnote
body specifically, hash `export.rs` muda. A.1.7 verifica
literalmente.

### A.1 — Inventário literal Layouter + 2-pass

8 sub-secções:

1. **A.1.1 — Layouter actual** — estrutura `Layouter` pós-P298
   (cluster math handlers, walker counter footnote P295).
2. **A.1.2 — `Content::Footnote` arm actual** — sítio exacto
   onde body é descartado.
3. **A.1.3 — `Introspector::query` ou similar** — coletar
   footnotes pendentes pré-layout.
4. **A.1.4 — Page break mechanism** — como cristalino decide
   quando quebrar página.
5. **A.1.5 — `outline()` 2-pass P65-66** — algoritmo literal.
6. **A.1.6 — P233/P234 sub-padrão** — algoritmo literal
   `measure_then_place`.
7. **A.1.7 — `Frame`/`FrameItem`** — variants suficientes para
   posicionamento rodapé?
8. **A.1.8 — Diagrama de fluxo 2-pass footnote** — produzir
   genuinamente.

### A.2 — Estrutura do 2-pass footnote

Decisão arquitectural:

| Opção | Mecanismo | Prós | Contras |
|---|---|---|---|
| **(a)** Reuso integral P233 pattern: Pass 1 measure todos os footnote bodies; Pass 2 place markers + bodies | Sub-padrão N=3; consistência arquitectural | Page break coupling requer adaptação |
| **(b)** Custom 2-pass específico footnote: introspect + layout-with-reserved-space | Optimizado | Divergência com P233 pattern |
| **(c)** 1-pass com lookahead via `Introspector::query` | Sem 2-pass formal | Pode falhar overflow cases |

Default sugerido: **(a)** se A.0.0 confirma HV (extensão pattern
P233). **(b)** se HW (pattern P233 inaplicável). **(c)**
rejeitado salvo P304.A minimal.

### A.3 — Integração com page break

| Opção | Comportamento |
|---|---|
| **(α)** Page break consume footnotes pendentes antes de fechar página | Single-source — todos os markers de uma página resolvem na mesma página |
| **(β)** Reserved space dynamic conforme markers acumulam | Mais flexível; mais complexo |
| **(γ)** Linha separadora horizontal entre body e footnote area | Cosmético; pode ser P304.C |

Default sugerido: **(α)** para P304.A; **(β)** para P304.B;
**(γ)** para P304.C.

### A.4 — Impacto em emit (ADR-0098)

| Opção | Mecanismo | Implicação |
|---|---|---|
| **(i)** Body emit via `FrameItem::Text` standard no rodapé Y position | Hash preserved 21º passo |
| **(ii)** Novo `FrameItem::FootnoteBody` variant | **Hash muda** intencionalmente — alteração justificada |
| **(iii)** Híbrido | Caso edge |

Default sugerido: **(i)** quase certo. **(ii)** apenas se A.1.7
revela necessidade absoluta.

### A.5 — Detecção de bugs latentes + matrix cross-construct (P302 §6.7 lição)

Cenários fronteira:

- 1 footnote por página — caso típico.
- 0 footnotes na página — comportamento normal.
- 2+ footnotes na mesma página — empilhamento bodies.
- Footnote no fim da página — espaço pode ser insuficiente.
- Footnote com body multi-linha — wrap dentro do rodapé.
- Footnote com body que excede página inteira — **overflow
  excluído P304** (P295.2).

**Matrix cross-construct** (lição P302 §6.7 / aplicada P303):

```
                  Sintaxe A          Sintaxe B           Sintaxe C
                  #footnote[body]    Inline em texto    Em math mode
Sem nesting       ✓ P304             ✓ P304             ? A.0.0
Com nesting       Vanilla proíbe?    Vanilla proíbe?    n/a
Antes do final    ✓ (caso típico)    ✓                  ?
No final página   espaço suficiente? overflow → P295.2  ?
```

### A.5' — Verificação anti-reflexão (N=14 do padrão §8.6)

**11ª reaplicação A.0.0** consecutiva.

4 verificações:

1. **Comparação literal A.1.6 P288-P304** — paradigma novo?
   - P293-P303 doze paradigmas distintos.
   - **P304**: paradigma **"2-pass layout footnote-aware"** —
     genuinamente novo se A.0.0 confirma HV/HW (reaplica P233
     pattern com extensão). 13º paradigma.
2. **A.0.0 produzido empiricamente** — magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **Primeira reaplicação sub-padrão "two-pass measure→place"
     N=3** desde P233/P234 — **limiar tentativo atingido se
     reaplicação genuína**.
   - **Page break coupling** — paradigma novo cross-constraint.
   - **Magnitude L primeira vez na série P283+ não-cluster**
     (P295 cluster foi M).
   - **Matrix cross-construct continuada** (lição P302 §6.7).
4. **Decisão sobre promoção ADR meta**:
   - **Sub-padrão "two-pass measure→place" N=3** —
     **candidato robusto** se A.2 → (a) confirma reuso genuíno;
     primeira promoção concreta possível em 11 passos.
   - **§8.7' N=11** — adiamento standard P300-P303.
   - **§8.3 N=12** candidato.
   - **§8.4 N=2 subcat A; N=1 subcat B** — não aplicável P304.
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério estrito**: promover **sub-padrão "two-pass" N=3**
apenas se A.2 → (a) e reaplicação **genuína não-trivial**.
Default conservador (P300 standard): **sem promoção** se
ambíguo.

---

## §3 — Materialização

**Cenário default (HV + P304.A + A.2 → (a) + A.3 → α + A.4 → (i))**:

1. Estender Layouter para 2-pass com footnote-awareness:
   - **Pass 1**: percorrer página corrente coletando
     `Content::Footnote` markers + medindo bodies.
   - **Pass 2**: layout corpo da página + reserved space para
     footnotes + bodies posicionados no rodapé.
2. Modificar `Content::Footnote` arm:
   - Emite marker `[N]` (preservado P295).
   - **Adicionalmente** regista body para Pass 2 rendering.
3. Adicionar campo `Layouter` para footnote bodies pendentes:
   ```rust
   pending_footnote_bodies: Vec<(u32, Box<Content>)>,
   ```
4. Page-close logic: antes de fechar página, layout bodies
   no rodapé via `FrameItem::Text` standard com posicionamento
   Y bottom.
5. Testes:
   - L1 unitário: body presente no PDF após P304 (invalida
     `p295_footnote_body_nao_renderizado_no_pdf_fase1`).
   - L1 unitário: marker `[N]` continua presente (regressão
     P295).
   - L1 unitário: 2 footnotes na mesma página produzem 2
     bodies empilhados.
   - L1 unitário: footnote single body multi-linha.
   - L3 integração: PDF com 1+ footnote produz body no rodapé.
   - L3 regressão bit-exact: documentos sem footnote produzem
     bytes idênticos pré-P304.
6. **Substituir teste P295** `p295_footnote_body_nao_renderizado_no_pdf_fase1`:
   - Documenta gap que P304 fixa.
   - Renomear/substituir para `p304_footnote_body_renderizado_no_rodape`.
7. Promoção ADR meta condicional per A.5':
   - **Sub-padrão "two-pass measure→place" N=3** se A.2 → (a)
     genuinamente reaplica P233.
   - Default conservador: **sem promoção**.
8. Actualizar L0:
   - Tabela A.6 linha 178 — `ausente`/`parcial` → `parcial⁺`
     ou `implementado` conforme P304 + P295.2 pendente.
   - Hash `entities/content.md`: preservado (sem variants novos).
   - Hash `rules/layout.md`: pode mudar conforme A.0.0.
   - Propagar via `crystalline-lint --fix-hashes`.
9. Diagnóstico produzido com 8 secções
   A.0.0+A.0.0'+A.0+A.1+A.2+A.3+A.4+A.5+A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: 15-25 (magnitude L
primeira vez pós-cluster math).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P303: 2 881 testes.
  Esperado: ~2 896-2 906.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved** (sem variants novos).
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `rules/layout.md` **condicional** — muda se
  Layouter struct ganha campo `pending_footnote_bodies` (esperado).
- Hash L0 `export.rs` **preserved bit-exact** pelo **21º passo
  consecutivo** P282-P304 se A.4 → (i).
- **Regressão bit-exact validada** — PDFs sem footnote idênticos.
- **Teste P295 invariante invalidado intencionalmente** —
  documentar transição.
- Tabela A.6 linha 178 reclassificada.
- Diagnóstico 8 secções produzido.
- **Promoção ADR meta condicional**:
  - **Sub-padrão "two-pass" N=3** se reaplicação genuína
    inequivocamente confirmada.
  - Default: sem promoção (anti-padrão honrado 12ª vez).
- Frente P295.1 resolvida; P295.2 (overflow) permanece pendente.

---

## §5 — Não-objectivos

- **Não** materializar P295.2 (overflow multi-página). Sub-passo
  dedicado.
- **Não** materializar P295.X (footnote.entry / multi-ref).
- **Não** alterar `Content::Footnote` semanticamente — variant
  pós-P295 preservado.
- **Não** materializar customization (separator line, height,
  style) — P304.C/D em sub-passos.
- **Não** promover múltiplas ADRs meta. Uma por passo (P273.17
  §0).
- **Não** confundir P304 com P295 — P295 marker only; P304 body
  rendering.
- **Não** confundir reaplicação sub-padrão "two-pass" com criação
  nova — se HV/HW confirmam reuso, N=3; se HW confirma
  infraestrutura nova, **NÃO** reaplica (categoria diferente).

---

## §6 — Pendências relacionadas

Resolve:
- **P295.1 nota corpo no rodapé** — pendente 9 passos.

Não resolve:
- **P295.2** overflow multi-página — sub-passo dedicado.
- **P295.X** footnote.entry / multi-ref.
- Cosméticos footnote (separator line, height customisation).
- Outras pendências catálogo P300.

---

## §7 — Risco residual

Risco principal: **magnitude L revela-se M+/XL**. Mitigação:
A.0.0' subdivisão honesta; abrir P304.0 dedicado se infraestrutura
2-pass insuficiente.

Risco secundário: **pressão de "fechar pendência 9 passos"
inflaciona decisões**. Mitigação: P300 §10 lição — critério é
factual; A.0.0 inspecção literal antes de assumir scope.

Risco terciário: **sub-padrão "two-pass" promovido prematuramente
N=3**. Mitigação: §A.5' critério estrito — promover apenas se
reaplicação genuinamente não-trivial. Se P304 reaplica
literalmente P233 código, qualidade do 3º caso ambígua;
adiar (paralelo P298 "cluster math handler" §6.5).

Risco quaternário: **page break coupling introduce bugs em
features pre-P304**. Mitigação: §4 regressão bit-exact obrigatória
para documentos sem footnote.

Risco quinário: **Layouter struct ganha campo novo** —
modificação estrutural maior que cluster math handlers (que só
acrescentaram funcs). Mitigação: A.1.1 verificação literal.

Risco senário: **`Introspector::query` insuficiente para
footnote collection** (pode requerir pre-walk dedicado).
Mitigação: A.1.3 verificação; HV/HW decide.

Risco septenário: **footnote multi-linha overflow dentro de
single page**. Mitigação: cenário A.5; documentar.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/rules/layout/mod.rs`
  (Layouter struct + page-close logic).
- Variant pré-existente: `Content::Footnote` (P295).
- Stdlib: `native_footnote` (P295) — preservado.
- Precedente 2-pass directo: **P233 + P234** ("two-pass
  measure→place" N=2 cumulativo).
- Precedente outline: **P65-66** (TOC via 2-pass introspection).
- Introspector: `Introspector::query` (P208C).
- Vanilla: `lab/typst-original/crates/typst-library/src/model/footnote.rs`.
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 8 secções).
- ADR cultural: P273.17 §0 (anti-padrão).
- ADR processual: ADR-0017 (Introspection) — materializada
  P208B+C.
- Padrão §8.7' A.0.0 template (N=11 reaplicação).
- Sub-padrão "two-pass measure→place" — N=2 cumulativo
  (P233+P234); P304 candidato N=3.

---

*Spec P304 produzida 2026-05-19 pós-P303 (bug irmão $undef(x)$
fixed; subcategoria §8.4 B inaugurada; 20º passo `export.rs`
preservado). Frente **P295.1 nota corpo no rodapé** — pendente
desde P295 §8 (9 passos). **Primeira magnitude L pós-cluster
math** P296-P298; **paradigma genuinamente novo** se A.0.0
confirma 2-pass footnote-aware reaplica sub-padrão "two-pass
measure→place" P233/P234 com extensão page break coupling.
Fase A obrigatória com **8 secções** A.0.0+**A.0.0'
(subdivision)**+A.0-A.5+A.5'. **Sub-padrão "two-pass" N=3
candidato robusto** se A.2 → (a) confirma reuso genuíno —
primeira promoção concreta possível em 12 passos. **Subdivisão
A.0.0' P304.A/B/C/D** evita inflação magnitude. Hash `export.rs`
preservado pelo 21º passo consecutivo se A.4 → (i). **Anti-padrão
P273.17 §0 honrado pela 12ª vez consecutiva** se sem promoção
(default). Lição P300 §10 aplicada: pressão "fechar pendência
9 passos" não é critério metodológico; A.0.0 inspecção literal
antes de assumir scope. Sem caps LOC ou magnitude (P282 §7).*
