# Passo 293 — `P-curve-geometry` (scope condicional a Fase A)

**Frente**: `P-curve-geometry` (rank 2 do relatório P292 §9.2).
**Origem**: P282 §6 listou frente como "S-M; ADR-0078 sub-fase b"
— **referência arquitectural ambígua** (verificada em A.0.0:
ADR-0078 cobre column flow, não curve geometry). Origem real:
Tabela A.7 linha 194 `path(...) / curve(...)` é `implementado⁺`
com nota "sem cubic optimisation completa (DEBT-33)" — **mas
DEBT-33 fechou em P277** (Bézier bbox analítica).
**Pré-requisitos**: scope clarificado em Fase A.0.0 obrigatória
antes de A.1.
**Marco**: P293 é o **primeiro passo ortogonal pós-série
cumulativa P288-P292** (per §9.1 P292).

---

## §1 — Objectivo (provisório; depende de A.0.0)

Refinar `path(...) / curve(...)` em `01_core/src/rules/stdlib/`
para alcançar paridade qualitativa adicional vs vanilla typst,
elevando estado actual `implementado⁺` para `implementado` na
Tabela A.7 linha 194.

**Mas o scope concreto é ambíguo** — ver A.0.0. Possibilidades:

1. **Sub-fase qualitativa residual pós-DEBT-33**: P277 fechou
   "Bézier bbox analítica"; mas pode haver operações Bézier
   adicionais (subdivision, length, intersection, etc.) ainda
   por materializar.
2. **Materialização de features visualize adjacentes ausentes**:
   `square` (linha 196), `cmyk/oklab` (linha 198), `gradient`
   (linha 199), `tiling` (linha 200), objecto `Stroke` rico
   (linha 201). Esta interpretação **mudaria** a designação da
   frente.
3. **Refinamento de cubic optimisation noutro plano**: e.g.
   PDF emit operators para Bézier (`c` vs sequência `l`),
   melhoria de precisão numérica, etc.

**A.0.0 obrigatória** decide qual interpretação é correcta antes
da spec se materializar.

Razão de ser:

1. **Primeiro passo ortogonal pós-série cumulativa** — quebra
   natural da sequência P288-P292 (per §9.1 P292).
2. **Refino qualitativo de feature `implementado⁺` para `implementado`** —
   estado actual reflecte aproximação documentada (Tabela A
   metodologia "⁺ = aproximação por ADR-0054").
3. **Sem reaplicação cumulativa de padrão paralelo** — P293 é
   genuinamente novo, não continua sequência cirúrgica. Padrão
   §8.5 P289 (já desqualificado) **não atinge N=6**.
4. **Potencial gatilho §8.3 N=6** — se A.0.0 revelar refutação
   significativa do scope antecipado P282 §6 ("ADR-0078 sub-fase
   b" — referência ambígua), refutação genuína documentada.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; A.0.0 inaugurada)

Seis secções: A.0.0 inaugural + A.0-A.5 paralelas a P292.

### A.0.0 — Clarificação de scope (NOVA, inaugural)

**Inaugurada** porque a frente `P-curve-geometry` tem **referência
arquitectural ambígua** registada literalmente em P282 §6
("ADR-0078 sub-fase b") que **não corresponde** ao que ADR-0078
efectivamente cobre (column flow, sub-fase b DEBT-56 FECHADA P220).

#### A.0.0.1 — Verificação literal do estado actual

1. **Inspeccionar literalmente** `01_core/src/rules/stdlib/curve.rs`
   (ou caminho equivalente): que operações estão materializadas?
   Que estão ausentes?
2. **Inspeccionar literalmente** `01_core/src/entities/shape.rs`:
   estrutura de `ShapeKind::Path`.
3. **Inspeccionar literalmente** `03_infra/src/export.rs`: como
   `ShapeKind::Path` é emitido em PDF (`c` operators para cubic
   Bézier? sequência de `l` para linear approximation?).
4. **Inspeccionar literalmente** `lab/typst-original/.../curve.rs`:
   que operações vanilla expõe? Comparar com cristalino.

#### A.0.0.2 — Hipóteses possíveis para scope P293

Após inspecção literal, classificar Fase A em uma das interpretações:

| Hipótese | Scope concreto | Magnitude esperada |
|---|---|---|
| **H1 — Cubic operations** | Materializar operações Bézier ausentes (subdivision, length, intersection) | XS-S |
| **H2 — PDF emit cubic** | Melhorar emit PDF para usar `c` operators em vez de linear approximation | S |
| **H3 — Features visualize ausentes** | Mudar scope para `square`/`gradient`/`cmyk` (frente diferente) | S-M cada |
| **H4 — Refino numérico** | Precisão de bbox/transformações | XS |
| **H5 — Mix** | Combinação parcial | S-M |

**Decisão A.0.0.2 obrigatória** com:
- Citação literal de ficheiros e linhas inspeccionadas.
- Justificação da hipótese escolhida.
- Reclassificação da frente se necessário (e.g. H3 → renomear para
  `P-square-materialization` ou `P-gradient-materialization`).

#### A.0.0.3 — Decisão sobre continuação P293

| Cenário | Acção |
|---|---|
| A.0.0.2 → H1/H2/H4 | P293 prossegue com scope clarificado |
| A.0.0.2 → H3 | **P293 interrompido**; abrir P293' renomeado conforme a feature concreta |
| A.0.0.2 → H5 | Decidir qual sub-componente é P293; os outros são passos próprios |

#### A.0.0.4 — Honestidade epistémica registada

**P293 spec assume que não sabe ainda o que vai materializar**.
A.0.0 inaugura template para casos onde a frente tem referência
arquitectural ambígua na origem. Não tentar adivinhar; inspeccionar
literalmente.

### A.0 — Potencial de reuso ADR-0098

Após A.0.0 clarificar scope, executar A.0 normal:

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "curve\|path" 03_infra/src/export.rs` | Hits prováveis (emit operators) | Listar e classificar paradigma |
| Materialização P293 (per A.0.0) toca `FrameItem::Shape` emit? | Depende H1/H2/H4 | Per cenário |
| Hash `export.rs 66cb8ac3` esperado | **Condicional**:<br>- H1 (cubic ops layout-only): preserved<br>- H2 (emit cubic): **muda** intencionalmente<br>- H3: depende feature<br>- H4: provavelmente preserved | A.0 esclarece pré-materialização |

**Crítico**: se A.0.0.2 → H2 (PDF emit cubic), hash `export.rs`
**muda** — quebra a sequência de 9 passos consecutivos preservando.
Esta seria a **primeira quebra** desde P281; deve ser registada como
evento histórico (não regressão; mudança intencional alinhada com
ADR-0098 §"alterações justificadas").

### A.1 — Inventário literal do caminho actual (per H da A.0.0)

Adaptado conforme hipótese A.0.0:

1. **A.1.1 — `Content::Shape{Path(...)}` actual** — variants do
   `PathSegment` (ou equivalente).
2. **A.1.2 — Operações stdlib actualmente expostas** —
   `path(...)`, `curve(...)`, métodos auxiliares.
3. **A.1.3 — Emit actual em `export.rs`** — PDF operators usados.
4. **A.1.4 — Vanilla typst comparação** — operações que faltam.
5. **A.1.5 — DEBT-33 fecho P277 detalhe** — verificar literalmente
   o que P277 materializou (Bézier bbox analítica) e o que ficou
   fora.
6. **A.1.6 — Consumers actuais** — Layouter consume `Content::Shape`,
   layout calcula bbox via P277 algoritmo, emit produz PDF.
7. **A.1.7 — Paradigma consumer** — variável conforme A.0.0.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente.

### A.2 — Decisão estrutural

Depende de A.0.0 + A.1. Possibilidades genéricas:

- Adicionar funções stdlib novas (`native_curve_subdivide`, etc.).
- Estender `PathSegment` variants.
- Refactor emit para usar `c` operator PDF.
- Mix.

**Decisão genuína** condicional a A.0.0.

### A.3 — Integração com tipos existentes

Conforme A.2 + A.0.0. Padrão "extensão de stdlib sem novos variants
em `Content`" preferido se aplicável (paralelo P283 calc trig).

### A.4 — Impacto em `FrameItem::Shape` e emit (aplicação ADR-0098)

Crítico para gatilho "9º passo consecutivo preservando export.rs":

| Cenário | Impacto |
|---|---|
| H1 (cubic ops layout-only) | Hash preserved → ADR-0098 N=10 cumulativo |
| H2 (emit cubic) | **Hash muda** — **quebra sequência consecutiva**; primeira quebra desde P281; documentar honestamente |
| H4 (refino numérico) | Hash provavelmente preserved (operações layout-time) |

**Registar em A.4 explicitamente** se P293 quebra a sequência de
preservação. Isto **não é regressão** — ADR-0098 permite alterações
justificadas; o que ela invariante é "reuso de single source of
truth", não "preservação do hash em si".

### A.5 — Detecção de bugs latentes

Conforme A.0.0. Cenários fronteira de Bézier:

- Caso degenerado: curve com 2 control points coincidentes.
- Curve fechada (start = end).
- Curve com loop (auto-intersecção).
- Precisão numérica em extremos (very small / very large coordinates).

### A.5' — Verificação anti-reflexão (N=3 do padrão §8.6)

Reaplicação P291+P292. **P293 é o primeiro passo ortogonal pós-série
cumulativa** — A.5' tem que avaliar:

1. **Comparação literal A.1.6 P288-P293** — esperar paradigma
   genuinamente novo (P293 é ortogonal por construção, não
   cumulativo).
2. **A.0 produzido empiricamente** — em P293 inclui A.0.0
   clarificação de scope.
3. **Elemento estructuralmente novo identificado**:
   - **A.0.0 em si é novo** — inaugural; primeiro passo onde scope
     da spec é genuinamente ambíguo e tem que ser clarificado
     empiricamente.
   - Adicionalmente: paradigma consumer da hipótese A.0.0 escolhida.
4. **Decisão sobre passo seguinte**:
   - Se A.0.0 → H3, P293 redirige para feature concreta; P294 é
     consequência directa.
   - Se H1/H2/H4, P293 procede normal; P294 é livre escolha.

**Padrão §8.6 N=3 cumulativo** — aproxima limiar tentativo N≥3-4.
**Mas não promover** — limiar exige reaplicações cumulativas, e
P293 é qualitativamente distinto dos anteriores (ortogonal vs
cumulativo).

---

## §3 — Materialização (condicional a A.0.0)

**Plano genérico**:

1. Após A.0.0 clarificar hipótese H1/H2/H3/H4/H5:
   - H1: materializar operações stdlib novas + testes.
   - H2: refactor emit `export.rs` para usar `c` operator + bit-exact
     comparison antes/depois para casos não-cubic preserved + tests
     novos.
   - H3: interromper P293 e abrir passo renomeado; relatório regista.
   - H4: refino numérico + testes precisão.
   - H5: subset concreto.

2. Aplicar ADR-0098 obrigatória — documentar em A.4 se hash
   preservado ou alterado intencionalmente.

3. Promoção ADR meta:
   - **Sem promoção** se cenário H1/H4 (refinos pequenos paralelos
     ao paradigma estabelecido).
   - **Considerar promoção** se H2 dispara §8.3 N=6 (refutação
     pragmática genuína da sequência "hash sempre preservado"
     significa que ADR-0098 ganha clausura "alteração justificada").
     Mas **uma ADR meta por passo**.

4. Actualizar L0:
   - Tabela A.7 linha 194 — actualizar conforme H escolhida.
   - Footnote nova com marco P293 (primeiro passo ortogonal).
   - Propagar hashes.

5. Actualizar diagnóstico:
   - `diagnostico-curve-geometry-passo-293.md` com **7 secções**
     A.0.0 + A.0-A.5 + A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: **incerta** —
H1 ~5-10, H2 ~10-20, H4 ~5-10. Reformula-se após A.0.0.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P292: 2 793 testes.
  Esperado: ~2 798-2 815 (depende de H).
- `crystalline-lint` zero violations.
- Hash L0 conforme A.0.0:
  - `style.md` **preserved** (P293 não toca em `Style` enum).
  - Outros L0 mudam conforme scope.
- Hash L0 `export.rs` **condicional a A.0.0.2**:
  - H1/H4: preserved → 10º passo consecutivo; ADR-0098 N=10.
  - H2: muda intencionalmente → **primeira quebra desde P281**;
    documentar como evento histórico em diagnóstico + relatório
    + footnote cobertura.
  - H3: P293 redireccionado — critério muda conforme feature
    concreta.
- **Regressão bit-exact validada** para call sites sem alteração
  intencional (e.g. se H2, casos não-cubic devem produzir bytes
  idênticos).
- Tabela A.7 linha 194 actualizada (estado e/ou nota).
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **A.0.0 produzida genuinamente** — inspecção literal antes de
  decidir hipótese; não citar P282 §6 como prova.
- A.5' N=3 cumulativo.

---

## §5 — Não-objectivos

- **Não** materializar `square` / `cmyk` / `oklab` / `gradient` /
  `tiling` / `Stroke` rico se A.0.0 → H1/H2/H4. Estas são features
  distintas. Se A.0.0 → H3, P293 redirige para uma delas mas as
  outras ficam para passos próprios.
- **Não** alterar `ShapeKind::Path` semanticamente se A.0.0 → H1/H4.
- **Não** promover múltiplas ADRs meta. Se §8.3 N=6 e §8.6 N=3
  ambos disparam, escolher uma (preferir §8.3 — mais evidência
  cumulativa estável).
- **Não** considerar `P293` como "continuação da série P288-P292".
  P292 §9.1 explicita: série terminou; P293 é ortogonal por
  construção.
- **Não** assumir scope sem A.0.0 — primeira spec onde A.0.0 é
  obrigatória por construção factual (ambiguidade real na origem).
- **Não** materializar refactor de helpers Bézier internos se A.0.0
  → H4 (refino numérico). Mudanças cirúrgicas apenas.

---

## §6 — Pendências relacionadas

Resolve (condicional a A.0.0):
- Estado actual `implementado⁺` linha 194 → `implementado` se
  scope clarificado e materializado.

Não resolve (continua aberto):
- `square` (Tabela A.7 linha 196) — passo dedicado.
- `cmyk/oklab` (linha 198) — passo dedicado.
- `gradient` (linha 199) — passo dedicado (M).
- `tiling` (linha 200) — passo dedicado (M).
- `Stroke` rico (linha 201) — passo dedicado.

---

## §7 — Risco residual

Risco principal **NOVO** P293: **scope ambíguo na origem**.
Frente referenciada como "ADR-0078 sub-fase b" em P282 §6 — mas
ADR-0078 cobre column flow, não curve geometry. Mitigação: A.0.0
clarifica antes de materializar; **template inaugural para casos
similares futuros**.

Risco secundário: A.0.0 → H3 (scope diferente que esperado) força
redirecção P293. Mitigação: §A.0.0.3 permite interrupção honesta
e renomeamento.

Risco terciário: H2 (emit cubic) quebra sequência de 9 passos
consecutivos preservando `export.rs`. Mitigação: §A.4 + §3 ponto
2 explicitam que ADR-0098 permite alterações justificadas;
preservação do hash **não é a invariante** — reuso de single
source of truth **é**.

Risco quaternário: §8.6 N=3 (A.5' anti-reflexão) candidato a
promoção mas ainda longe do tentativo N≥3-4 + P293 é
qualitativamente diferente dos anteriores (ortogonal vs
cumulativo). Mitigação: §5 não-objectivo; relatório regista.

Risco quinário: P293 introduz dependências novas (e.g. crate para
cubic Bézier operations). Mitigação: A.0.0 verifica
literalmente; se H1 exige nova dependência, registar como
sub-decisão arquitectural (paralelo a ADR-0018 / DEBT-libm
mencionada em P283 spec).

Risco senário (anti-meta): "sequência reflexa" persiste apesar
de P293 ser ortogonal. Mitigação: A.5' verifica genuinamente; se
mesmo ortogonal degenera em automatização, abrir auditoria meta.

---

## §8 — Ponteiros

- Tipo a inspeccionar: `01_core/src/entities/shape.rs`
  (`ShapeKind::Path` + segmentos).
- Caminho stdlib: `01_core/src/rules/stdlib/` (curve/path funções).
- Caminho emit: `03_infra/src/export.rs` (PDF operators para shape).
- Vanilla: `lab/typst-original/crates/typst-library/src/visualize/curve.rs`.
- Precedente fecho graded: P277 (DEBT-33 Bézier bbox analítica
  CLOSED).
- Precedente refino qualitativo: P156A-J (Layout Fase 2; refino
  cumulativo).
- Precedente scope ambíguo resolvido: **nenhum** — A.0.0 é
  inaugural.
- ADR aplicável: **ADR-0098** + **ADR-0099** (ambas vigentes).
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções
  A.0.0+A.0-A.5+A.5').
- ADR cultural: P273.17 §0 (anti-padrão).
- ADR visualize: nenhuma específica para curve (verificar A.0.0.1).

---

*Spec P293 produzida 2026-05-19 pós-P292 (série cirúrgica
P288-P292 terminada). Frente `P-curve-geometry` — **primeiro
passo ortogonal pós-série cumulativa** (per §9.1 P292). **Spec
parcialmente provisória**: scope concreto depende de A.0.0
inaugural ("clarificação de scope") porque frente foi referenciada
como "ADR-0078 sub-fase b" em P282 §6 mas ADR-0078 cobre column
flow (não curve geometry); referência arquitectural ambígua na
origem. A.0.0 obrigatória inspecciona literalmente
`stdlib/curve.rs` + `entities/shape.rs` + `export.rs` + vanilla,
classifica scope em hipóteses H1-H5, decide qual é P293, e
**permite interrupção honesta + renomeamento se H3**. Fase A
expandida com **7 secções**: A.0.0 (nova) + A.0-A.5 (paralelo
P292) + A.5' (N=3 cumulativo padrão §8.6). Critério de fecho
**condicional**: H1/H4 preserva hash `export.rs` (10º passo
consecutivo); H2 quebra intencionalmente — primeira quebra desde
P281; documentar como evento histórico (não regressão). **A.0.0
inaugural** estabelece template para casos futuros com scope
ambíguo na origem — protege contra materialização sob
pressuposto falso. **Sem promoção ADR meta antecipada** — A.0.0
pode revelar gatilhos genuínos (§8.3 N=6 se refutação pragmática
significativa em H2), mas **uma ADR meta por passo no máximo**
(P273.17 §0). Honestidade epistémica reforçada: spec **aceita não
saber ainda** o que vai materializar. Sem caps LOC ou magnitude
(P282 §7).*
