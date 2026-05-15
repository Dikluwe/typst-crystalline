# ⚖️ ADR-0082: Promoções reais de scope-outs ADR-0054 graded — 4 critérios operacionais

**Status**: `PROPOSTO`
**Data**: 2026-05-14 (PROPOSTO P249)
**Autor**: Humano + IA
**Validado**: 8 aplicações cumulativas granular pós-M9d
(P242 radius + P242 clip + P247 outset + P247 fill + P247
stroke + P248 breakable + P248 height + P248 cell overflow;
**N=8 patamar empírico extremamente sólido** — ultrapassa
critério N≥4 mínimo ADR-0065 e N≥5 sólido ADR-0064).
**Reservado para**: meta-documental codifica prática empírica
emergente Categoria A.4 Fase 5 Layout pós-M9d (cumulativa P242
+ P247 + P248).

**Nota numeração**: spec P249 hipótese previa ADR-0067 mas
ADR-0067 já estava ocupado (`attribute-grammar-scoping`);
spec activou cenário `P249.div-2`. ADR-0082 escolhido como
próximo slot disponível após ADR-0081 (M7+ pipeline restructuring
scope).

---

## Contexto

A ADR-0054 formalizou perfil **graded** de paridade vanilla:
permite scope-outs declarados em passos de materialização
("refino futuro per ADR-0054 graded" / "armazenado mas semantic
adiada" / "ignorado em layout"). Estes scope-outs ficaram
historicamente como dívida arquitectural latente — sem regra
operacional sobre **como, quando, e sob que condições** podem
ser promovidos a semantic real.

Entre P242 e P248 (cumulativamente 3 passos pós-M9d com Categoria
A.4 Fase 5 Layout), emergiu prática empírica não-formalizada:
**promoção real de scope-out declarado segue uma sequência
estável de 4 critérios** que assegura backward compat literal
e divergência mínima face vanilla.

**8 aplicações concretas cumulativas granular pós-M9d**:

| # | Passo | Scope-out promovido | Origem (graded) |
|---|-------|---------------------|-----------------|
| 1 | P242 | `radius` (Block + Boxed) | P156G + P156H scope-out |
| 2 | P242 | `clip` (Block + Boxed) | P156G + P156H scope-out |
| 3 | P247 | `outset` semantic real (Block + Boxed) | P156G + P156H + P231 graded |
| 4 | P247 | `fill` (Block + Boxed) | P156G + P156H scope-out |
| 5 | P247 | `stroke` (Block + Boxed) | P156G + P156H scope-out |
| 6 | P248 | `Block.breakable` semantic real | P156G "semantic adiada" |
| 7 | P248 | `Boxed.height` overflow real | P156H "semantic adiada" |
| 8 | P248 | `TableCell.body` overflow clip implícito | P157B "ignorados em layout" |

Em todos os 8 casos, a sequência de 4 critérios foi observada
empíricamente — sem que nenhum passo tenha sido obrigado a
reformular arquitectura mid-passo (0 reformulações cumulativas).
A regra que emergiu é o objecto deste ADR.

---

## Decisão

### Regra vinculativa

**Promoção real de scope-out ADR-0054 graded segue 4 critérios
operacionais.** Passos futuros que promovam scope-outs devem
satisfazer os 4 critérios (ou justificar formalmente desvio
mid-passo via `P-N.div-N`).

### Os 4 critérios

#### Critério 1 — Storage prévio

O scope-out já está **armazenado em campo de variant Content**
(não promoção a variant novo). Distingue **promoção real** de
**materialização nova**.

**Aplicação concreta**: P242 promoveu `radius`/`clip` que estavam
armazenados desde P231 (graded); P247 promoveu `outset`
(armazenado P231) + adicionou `fill`/`stroke` (novos campos mas
ainda dentro de variant existente, não variant novo — ainda
qualifica como aditivo paralelo). P248 promoveu `breakable`
(armazenado P156G), `height` (armazenado P156H), e cell overflow
(detecção via campo existente `regions.cell.height` P246).

#### Critério 2 — Consumer Layouter pre-promoção é graded

O consumer (tipicamente Layouter `mod.rs` ou `grid.rs`) tem
estado pre-promoção em uma de 3 formas:

- **"Armazenado mas ignorado"**: arm tem `field: _` (default
  underscore).
- **"Armazenado mas semantic adiada"**: arm lê o campo mas
  semantic real não é aplicada (paridade graded).
- **"Comportamento parcial via outro caminho"**: o campo está
  activo em algum contexto (ex: Grid cell) mas não no contexto
  primário (ex: Block isolado).

Decisão de promoção é **activar/completar semantic real no
consumer** sem alterar shape do variant.

**Aplicação concreta**: P242 trocou `radius: _, clip: _` → leitura
real; P247 trocou `outset: _` → leitura real cenário A audit
confirmado; P248 trocou `breakable: _` → leitura real + medição
antecipada.

#### Critério 3 — Paridade vanilla referência empírica

A implementação real deve ser confrontada com
`lab/typst-original/crates/typst-*/` antes de cristalizar
(audit C1 obrigatório bloqueante pós-P236.div-1; pattern N=12
cumulativo pós-P249). Divergências graded permitidas per
ADR-0054 são documentadas em **"Limitações conscientes"** do
passo de origem.

**Aplicação concreta**: P248 referência vanilla "overlong atómico
não quebra" para Block.breakable false + body excede página
inteira; P248 paridade "clip default para Boxed.height overflow"
quando clip=true.

#### Critério 4 — Backward compat literal

Defaults pré-promoção preservados literais — output PDF
bit-equivalente para casos default (paridade tests sentinela:
`*_default_preserva_*`). **Adaptações em tests pré-existentes
esperadas N≈0**.

**Aplicação concreta**: P248 atingiu **N=0 adaptações** (defaults
`breakable: true` + `height: None` + cell sem overflow
renderizam idênticos a P247; tests sentinela
`p248_block_breakable_true_preserva_emit_normal`,
`p248_boxed_height_none_preserva_p156h`,
`p248_table_cell_sem_overflow_preserva_p157b` validam).
P247 atingiu N=12 adaptações (acima range estimado por refino
construtores explícitos). P242 atingiu N=0 (paridade P248
sentinela retroactiva).

---

## Justificação empírica

### Tabela cumulativa N=8 (validação pattern)

| # | Passo | Critério 1 (Storage) | Critério 2 (Graded) | Critério 3 (Vanilla ref) | Critério 4 (BC literal) |
|---|-------|---------------------|---------------------|-------------------------|------------------------|
| 1-2 | P242 | radius/clip armazenados P231 | `_` em arms | RoundedRect vanilla path | tests P231 sentinela preservados |
| 3 | P247 | outset armazenado P231 | `outset: _` zero-uso | margin CSS paridade | test `outset+fill+stroke=None_sem_Shape` |
| 4-5 | P247 | fill/stroke novos paralelos | helper `extract_stroke` reusado | Grid/Table fill+stroke pattern | defaults None |
| 6 | P248 | breakable armazenado P156G | `breakable: _` literal | overlong atómico vanilla | default true sentinela |
| 7 | P248 | height armazenado P156H | comentário "adiada" | clip default vanilla | default None sentinela |
| 8 | P248 | cell body via regions.cell.height P246 | sem consumer cell overflow | clip implícito vanilla | sem overflow sentinela |

**Pattern N=8 atinge limiar formalização N=4 sólido (ADR-0065
critério mínimo)** amplamente ultrapassado.

### Contraste pré-P242: ad-hoc manualidade

Pré-P242, promoções de scope-outs eram tratadas caso-a-caso sem
critérios formais explícitos. Resultado: variabilidade na
qualidade do backward compat literal + dependência de
discernimento do executor.

P242+P247+P248 unificaram a sequência operacional implicitamente.
**P249 formaliza explicitamente** para reduzir overhead de
enunciados e aumentar previsibilidade.

---

## Alternativas consideradas

### Alternativa 1 — Manter ad-hoc (não formalizar)

Cada passo continuaria a derivar critérios localmente. Custo:
re-justificação empírica per-passo (~5-10 min/passo); risco de
divergência metodológica entre executores futuros.

**Rejeitada**: padrão N=8 cumulativo justifica overhead único
de formalização vs N×5-10 min cumulativo per-passo futuro.

### Alternativa 2 — Revisão R1 de ADR-0054

Anexar 4 critérios directamente em ADR-0054 (perfil graded).

**Rejeitada**: ADR-0054 cobre **perfil de paridade** (decisão
de adoptar graded sobre estrito). 4 critérios operacionais são
**metodologia downstream** distinta — paridade ADR-0080 (L0
minimal) que também é meta-documental separada.

**Decisão híbrida adoptada**: ADR-0082 PROPOSTO (ADR meta nova)
**+** anotação cruzada secção nova "§Promoções reais
cumulativas" em ADR-0054 §"Perfil de paridade" (refino interno;
ADR-0054 status `EM VIGOR` **preservado literal**).

### Alternativa 3 — ADR meta única vs múltipla

Sub-padrões relacionados emergentes:
- "Promoção graded → real semantic activação consumer" N=2
  cumulativo (P245 Place float + P248 agregado).
- "Agregar promoções cosméticos visuais ortogonais" N=1 (P247).
- "Agregar promoções multi-consumer via mecanismo comum" N=1
  (P248).

Poderia justificar 3 ADRs separadas. **Rejeitada**: critério
N≥4 não satisfeito em sub-patterns individuais (N=1-2).
**Decisão**: tratar como sub-patterns em ADR-0082 §"Sub-padrões
relacionados" com formalização meta-meta diferida.

---

## Implicações

### Positivas

- **Sessões futuras citam ADR-0082** em vez de re-justificar
  empíricamente cada promoção; redução overhead enunciados.
- **Backward compat literal mantido** uniformemente via Critério
  4 (sentinelas defaults preservados).
- **Audit C1 reforçado** (Critério 3 vanilla ref).
- **Categoria A.4 Fase 5 Layout cumulativa** P242+P247+P248
  documentada cobertura sólida.

### Neutras

- ADR-0054 §"Perfil de paridade" refino interno secção nova
  cumulativa; **status EM VIGOR preservado**.
- Sub-patterns relacionados anotados mas formalização meta-meta
  diferida (limiar N≥4 não satisfeito).

### Negativas (mitigadas)

- **Overhead criar ADR meta**: mitigado por estrutura canónica
  reusada (ADR-0065/ADR-0080 template; ~30-60 min puramente
  documental).
- **Risco "regra rígida sufoca casos novos"**: mitigado por
  cláusula "justificar formalmente desvio via `P-N.div-N`"
  (paridade pattern divergências documentadas P244 +
  P247.div-N + P248 zero divergências).

---

## Sub-padrões relacionados (formalização meta-meta diferida)

### Sub-padrão A — "Promoção graded → real semantic activação consumer"

**N=2 cumulativo pós-P248**:
- N=1 (P245 Place float real — primeiro storage P223 → semantic
  P245 cross-passo).
- N=2 (P248 agregado 3 sub-activações Block.breakable +
  Boxed.height + TableCell overflow via mecanismo comum
  `measure_content_constrained`).

Distingue-se de Promoção real scope-out (ADR-0082 objecto)
porque o storage prévio era em campo `Content::Place` (P223)
mas a semantic real depende de mecanismo cross-passo (defer
buffer floats vs medição antecipada). Granularidade N=4
contando 3 sub-activações P248 + 1 P245.

**Formalização meta-meta diferida**: limiar N≥4 conceptual
não-claro (mistura agregação vs cross-passo).

### Sub-padrão B — "Agregar promoções cosméticos visuais ortogonais"

**N=1 inaugurado P247** (3 cosméticos visuais ortogonais:
outset + fill + stroke em passo único; magnitude M; coesão
visual; tests cross-multiplicados).

### Sub-padrão C — "Agregar promoções graded → real multi-consumer via mecanismo comum"

**N=1 inaugurado P248** (3 sub-activações com mecanismo
partilhado `measure_content_constrained`; magnitude L
controlada; coesão semantic via medição antecipada).

Distingue-se de Sub-padrão B (P247 = ortogonais aditivos;
P248 = mecanismo comum).

---

## Aplicações citantes (validação empírica pós-PROPOSTO)

Sub-secção criada para registar aplicações concretas que citam
ADR-0082 explicitamente (em vez de re-justificar empíricamente
os 4 critérios). Promoção a EM VIGOR pendente N=3 aplicações
consecutivas citantes (paridade ADR-0065 EM VIGOR pós-P156K
validada P156J/P157A/P157B sequente).

### N=1 — P250 (2026-05-14): Block.spacing + Block.above + Block.below + Block.sticky

**Primeira aplicação citante.** P250 promove 4 scope-outs
originais P156G (spacing + above + below + sticky) com semantic
real cumulativa; **fecha Block A.4 COMPLETO 10/10**.

Os 4 critérios operacionais ADR-0082 verificados explicitamente:

1. **Storage prévio** ✓ — 4 fields scope-out P156G "Limitações
   conscientes" declarados originalmente (não variants novos).
2. **Consumer Layouter pre-promoção graded** ✓ — 4 args
   actualmente "rejeitados em `native_block` com erro hard"
   P156G; arm Block layout consumer ignora literal.
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.4
   P250 confirmou
   `lab/typst-original/crates/typst-library/src/layout/
   container.rs`: vanilla `Em::new(1.2)` default; `above.or(
   spacing)` fallback; `max(prev.below, curr.above)` collapse;
   sticky default false.
4. **Backward compat literal** ✓ — defaults (None×3 + false)
   produzem output PDF bit-equivalente para Block sem estes
   args; sentinela `p250_block_defaults_preserva_output_pre_
   p250` valida.

**Granularidade**: 4 sub-promoções (1 fila tabela cumulativa
ADR-0054 §"Promoções reais"). Total cumulativo pós-P250: **N=12
granular** (P242 ×2 + P247 ×3 + P248 ×3 + P250 ×4).

**Pattern adicional inaugurado P250**: "Refactor Sequence
consumer cross-arm via peekable + neighbour context" N=1 —
distinto P250-específico (não meta a ADR-0082; mas relacionado
porque viabiliza sticky lookahead). Formalização meta-meta
diferida (N≥4 não satisfeito).

### N=2 — P251 (2026-05-14): TableCell.body overflow row break real cell-level γ-Items

**Segunda aplicação citante.** P251 promove scope-out P157B
TableCell.body overflow de "clip implícito P248" para "row
break vertical real cell-level" via γ-Items (slice frame items
por threshold + buffer pending + flush em new_page chain);
**activa Categoria C.2 Fase 5 Layout parcialmente** (cell-level
apenas; multi-region completo diferido).

Os 4 critérios operacionais ADR-0082 verificados explicitamente:

1. **Storage prévio** ✓ — TableCell.body já armazenado P157B
   (scope-out original "ignorados em layout" graded); semantic
   actual P248 "clip implícito" não é variant novo.
2. **Consumer Layouter pre-promoção graded** ✓ — P248 "clip
   implícito" é graded (`FrameItem::Group { clip_mask: Some(Rect),
   .. }`); não é semantic real "row break vertical cross-page".
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.1
   P251 confirmou `layout_sub_frame_with_width` retorna items
   `pos.y` local; γ-Items viável magnitude L face γ-Content L+.
   Vanilla `lab/typst-original/crates/typst-layout/src/grid/`
   reference disponível (limitações atomic + recursive overflow
   3 iter).
4. **Backward compat literal** ✓ — cells sem overflow + cells
   em rows `TrackSizing::Fixed` preservam P248 clip implícito
   bit-equivalente; só cells Auto/Fraction com overflow ganham
   semantic nova (sentinelas
   `p251_cell_sem_overflow_preserva_p248_output_literal` +
   `p251_table_cell_overflow_row_fixed_preserva_p248_clip`).

**Granularidade**: 1 sub-promoção (1 fila tabela cumulativa
ADR-0054 §"Promoções reais"). Total cumulativo pós-P251: **N=13
granular** (P242 ×2 + P247 ×3 + P248 ×3 + P250 ×4 + P251 ×1).

**Patterns adicionais inaugurados P251** (não meta a ADR-0082;
relacionados):
- "Slice frame items at height via filter + rebase pos.y" N=1
  (novo módulo `layout/slicing.rs`).
- "DeferredX buffer + flush em new_page" N=1 → N=2 cumulativo
  (P245 floats + P251 cell tails).

**Marco**: Categoria C.2 Fase 5 Layout activada parcialmente
cell-level; promoção ADR-0082 PROPOSTO → EM VIGOR pendente N=3
citantes (P252 candidato Boxed stroke-overhang).

---

## Plano de promoção (status PROPOSTO → EM VIGOR)

ADR-0082 mantém status `PROPOSTO` até satisfação de **ambas**:

1. **Próxima aplicação N=9 cumulativa cita explícitamente
   ADR-0082** (em vez de re-justificar empíricamente os 4
   critérios).
2. **N=3 aplicações consecutivas citantes** atingidas (paridade
   limiar formalização interno ADR-0065 EM VIGOR pós-P156J +
   P157A + P157B sequente).

**Decisão de promoção é humana** — não automática per passo
materialização.

Candidatas próximas N=9 (recomendação subjectiva pós-P249):

- **A.4 Block 4 scope-outs restantes** (spacing + above + below
  + sticky agregados paridade P247) — 4 sub-promoções em passo
  único cumulariam N=9-12 directamente.
- **A.4 Boxed 1 scope-out restante** (stroke-overhang) — 1
  sub-promoção atinge N=9.

---

## Referências

- **ADR-0033** — Convenção L1 perfil paridade observacional.
- **ADR-0054** — Critério fecho DEBT-1 perfil graded (este
  ADR refina); §"Promoções reais cumulativas" anotação cruzada
  P249.
- **ADR-0065** — Inventariar primeiro (pattern N=5 formalização
  precedente; template estrutural ADR meta).
- **ADR-0080** — L0 minimal para refactors (pattern N=9
  formalização precedente; aplicação paralela à anti-inflação).
- **ADR-0079** — Layout Fase 5 roadmap (Categoria A.4
  cumulativa P242+P247+P248 anotada).
- **Passos**: P156G, P156H, P157B (origem scope-outs); P231
  (outset armazenado); P242 (radius + clip promovidos);
  P246 (`regions.cell.height` migrado); P247 (outset semantic
  + fill + stroke); P248 (breakable + height overflow + cell
  overflow); **P249** (este ADR meta).
- **Spec administrativos XS precedentes**: P156A historiograma
  + P156K ADRs meta + ADR-0062-create + P160A + P238 + P244 +
  **P249** (sétima aplicação cumulativa).

---

## Próximos passos

1. **Aplicação concreta N=9 candidata** (recomendação A.4 Block
   spacing+above+below+sticky) cita ADR-0082 explicitamente.
2. **N=3 aplicações consecutivas citantes** atingidas.
3. Humano promove ADR-0082 `PROPOSTO` → `EM VIGOR` em passo
   administrativo XS futuro (paridade ADR-0080 P229 promoção
   pós-P227+P228 validações).

Anotação cumulativa preserva contexto histórico para retomada
futura.
