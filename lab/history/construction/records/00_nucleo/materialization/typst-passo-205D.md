# Passo 205D — `label_pages` trackable (condicional)

**Série**: 205 (sub-passo `D` = label_pages após P205C
position_of consumer integration).
**Tipo**: implementação condicional (decide no
inventário se materializa ou adia).
**Magnitude planeada**: S (se materializar) ou nula (se
adiar — encerramento documental no próprio passo).
**Pré-condição**: P205C concluído; impl real de
`position_of` activa via `inject_positions`; pendência
ADR-0073 §C6a fechada estruturalmente; tests 1860
verdes; 0 violations; ADR-0074 PROPOSTO em vigor com
§P205B + §P205C anotados ✅ MATERIALIZADO.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código se C2 = materializar).

---

## §1 Propósito

Decidir e (se decidir afirmativamente) materializar
trackagem de `label_pages` em paralelo a
`SealedPositions`, completando F3 minimal per ADR-0074
plano de materialização.

A decisão de materializar ou adiar é fixada no inventário
inicial com base em consumers reais e benefício
arquitectural observado. ADR-0074 declara P205D
**condicional** — não é obrigatório (contraste com
P205B/P205C que fixam infraestrutura + impl real).

P205D respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Material de partida verificado em P205C

Antes de qualquer alteração, confirmar empíricamente:

- `SealedPositions` em
  `01_core/src/entities/sealed_positions.rs` com
  `#[comemo::track] impl` aplicado.
- `TagIntrospector` ganhou `pub positions:
  SealedPositions` + método `inject_positions`
  (P205C).
- `Introspector::position_of` impl real activa via
  injecção.
- `runtime.label_pages: HashMap<Label, NonZeroUsize>`
  em `LayouterRuntimeState` populated single-pass.
- `PagedDocument.extracted_label_pages` populated em
  `Layouter::finish` (precedente Passo 63).
- Tests 1860 verdes; 0 violations.

Sem isto, recuar para P205C.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **Consumers de `label_pages`** — confirmar:
   - Onde é lido em produção (não-tests)?
   - Tests existentes que invocam `label_pages` ou
     `extracted_label_pages`.
   - Padrão de acesso actual (HashMap directo via
     `runtime.label_pages` ou `doc.extracted_label_pages`).
2. **Trait `Introspector` e label_pages** — confirmar:
   - Existe método trait que expõe label → page?
   - Esperado: `query_by_label` retorna `Option<Location>`,
     não `Option<page>`. Distinto.
   - Há método "label_to_page" actual ou potencial?
3. **Pipeline pré vs pós-layout para label_pages** —
   confirmar:
   - Pre-layout: ausente (label → page é resultado de
     layout).
   - Durante layout: populated single-pass via
     `runtime.label_pages.insert(label, page)`.
   - Pós-layout: disponível via
     `doc.extracted_label_pages` ou via injecção em
     `TagIntrospector` (análogo a P205C).
4. **Consumers actuais de `extracted_label_pages`** —
   confirmar:
   - PDF export (provável).
   - PDF outline / TOC (provável).
   - Outros call sites empíricos.
5. **Benefício potencial de tracking** — análise
   qualitativa:
   - Queries label_pages são raras durante layout
     (single-pass) ou frequentes em queries pós-layout?
   - Cache hits via comemo seriam significativos?
   - Vanilla typst aplica tracking em label_pages? (per
     A6 P205A — confirmar empíricamente).
6. **Aliasing entre `runtime.label_pages` e
   `doc.extracted_label_pages`** — confirmar:
   - São o mesmo HashMap movido em sealing? (per Passo
     63 padrão).
   - Ou há cópia / divergência?

Output: 6 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

### C2 — Decisão sobre materializar ou adiar

Com base em C1, fixar:

- **Materializar — Caminho A** — `SealedLabelPages`
  análogo a `SealedPositions`; campo em `TagIntrospector`
  enriquecido; método trait dedicado
  (`label_to_page` ou similar) ou consumer query
  expandida.
- **Adiar — Caminho B** — P205D fecha sem materializar
  trackagem. ADR-0074 §P205D anotado como
  `✅ DEFERIDO` com fundamento empírico. P205E é o
  passo seguinte (encerramento série).

Critério para escolha:

- Se C1.5 mostrar **benefício real** (queries
  frequentes pós-layout ou paridade com vanilla) e C1.4
  identificar consumer concreto: Caminho A.
- Se C1.5 mostrar **zero benefício observável** (queries
  são raras; consumers actuais usam `doc.extracted_label_pages`
  directamente sem fricção): Caminho B.

C2 fixa **uma** alternativa. **Caminho B é resultado
empírico legítimo**, análogo ao "Caminho C — adiar" da
spec P205C — só que aqui é **mais provável** porque ADR-0074
declara P205D condicional, não fixa materialização (ao
contrário de §P205B + §P205C que fixaram).

### C3 — `SealedLabelPages` (se C2 = A)

Se Caminho A:

```text
pub struct SealedLabelPages(HashMap<Label, NonZeroUsize>);

#[comemo::track]
impl SealedLabelPages {
    fn page_of(&self, label: Label) -> Option<NonZeroUsize>;
}
```

Mesma forma estrutural de `SealedPositions` (P205B):
- Newtype com field privado.
- Sem Arc.
- `#[comemo::track] impl` directo.
- Localização: `01_core/src/entities/sealed_label_pages.rs`.

L0 prompt em
`00_nucleo/prompts/entities/sealed-label-pages.md` per
Protocolo de Nucleação (lição P204D/E/G/P205B).

### C4 — Sealing point + integração (se C2 = A)

Se Caminho A:

- `Layouter::finish` produz
  `doc.extracted_label_pages_sealed: SealedLabelPages`
  ou estende `extracted_label_pages` para conter
  ambos os formatos.
- `TagIntrospector` ganha campo `pub label_pages_sealed:
  SealedLabelPages` + método
  `inject_label_pages(sealed)`.
- Trait `Introspector` ganha método novo
  `label_to_page` ou similar (decide em C5).

Se C2 = B: skip C3/C4.

### C5 — API pública (se C2 = A)

Decisão entre:

- **C5a** — Novo método trait `label_to_page(label) ->
  Option<NonZeroUsize>` no `Introspector`.
- **C5b** — Sem método trait novo; consumer acede via
  `intr.label_pages_sealed.page_of(label)` directamente
  (similar ao padrão actual de
  `doc.extracted_label_pages`).

Critério: se há plano de tornar isto trait-tracked via
comemo, C5a (método trait). Se não, C5b é mais leve.

C5 fixa **uma** se C2 = A.

### C6 — Tests dedicados

Se C2 = A: adicionar 2–4 tests (sentinelas + unit + E2E).

Se C2 = B: sem tests novos.

### C7 — Compilação

Se C2 = A:

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde.

Se C2 = B: skip (P205D é puro documental).

### C8 — Tests workspace

Se C2 = A:

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1860+ tests verdes.

Se C2 = B: 1860 mantém-se.

### C9 — Linter

```
crystalline-lint .
```

Critério: 0 violations independentemente de C2.

### C10 — Documentação ADR-0074

ADR-0074 mantém PROPOSTO. Anotação cirúrgica em §P205D:

- Se C2 = A: `✅ MATERIALIZADO 2026-05-07` + sumário.
- Se C2 = B: `✅ DEFERIDO 2026-05-07 — fundamento
  empírico: zero benefício observável; consumers actuais
  usam acesso directo sem fricção. F3 minimal completo
  via P205B+P205C.`

A transição final ADR-0074 PROPOSTO → ACEITE fica para
P205E.

### C11 — Sentinelas

Se C2 = A: 1–2 sentinelas dedicadas.

Se C2 = B: sem sentinelas novas.

### C12 — Critério de fecho de P205D

P205D concluído quando:

- C1 inventário completo (6 sub-secções).
- C2 caminho fixado com justificação empírica.
- (Caminho A) C3–C7 implementação aplicada.
- C8 tests workspace verdes.
- C9 linter 0 violations.
- C10 ADR-0074 anotada (MATERIALIZADO ou DEFERIDO).
- (Caminho A) C11 sentinelas.
- Inventário registado.
- Relatório escrito.

### C13 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa empírica.
C3–C11 executam o caminho fixado.

A possibilidade de C2 = Caminho B (adiar) **não é ramo
condicional** — é resposta empírica honesta a "zero
benefício observável detectado". ADR-0074 declarou
P205D **condicional**; honrar a condicionalidade
afirmativamente ou negativamente não é ramo, é
resposta.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205D-inventario.md`.

Conteúdo:
- §1 C1 — inventário (6 sub-secções).
- §2 C2 — caminho fixado (A ou B) com justificação.
- §3 (Caminho A) C3–C5 alterações literais.
- §4 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-205D-relatorio.md`.

Conteúdo:
- O que foi feito.
- Caminho escolhido (A ou B).
- Tempo de execução.
- Métricas (tests pre/post; LOC delta — possível +0 se
  Caminho B).
- Decisões.
- Sugestão para próximo sub-passo (P205E em ambos os
  casos).

### Ficheiro 3 — Alterações em código (se C2 = A)

Não é ficheiro discreto. Conjunto de:

- 1 ficheiro novo (`sealed_label_pages.rs`).
- L0 prompt novo.
- `01_core/src/entities/mod.rs` (export).
- `Layouter::finish` adapta-se.
- `TagIntrospector` ganha field + método.
- Trait `Introspector` ganha método (se C5a).
- Tests dedicados.
- Sentinelas.
- Anotação cirúrgica em ADR-0074.

Se C2 = B: apenas anotação cirúrgica em ADR-0074
(`✅ DEFERIDO`).

---

## §5 Critério de progressão para P205E

P205D fechado quando C12 cumprido.

Em ambos os caminhos (A ou B), P205E é o sucessor
imediato.

Em caso de divergência empírica relevante, registar em
`P205D.div-N` e:

- Resolver dentro de P205D (preferido).
- Recuar para P205A re-fixar C1 (escopo F3) se
  obstrução for estrutural — improvável.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P205D não:

- Transita ADR-0074 para ACEITE (P205E).
- Cria ADR nova além de ADR-0074 já PROPOSTO.
- Toca em loop fixpoint.
- Modifica `runtime.label_pages` populated single-pass.
- Modifica `extracted_label_pages` em `PagedDocument`
  (mantém-se per Passo 63 + P205B).
- Modifica `SealedPositions` ou impl `position_of`
  (esses foram P205B + P205C).
- Endereça outros sub-stores Categoria B além de
  `label_pages` (per ADR-0074 escopo F3 minimal).
- Endereça vanilla integration (DEBT-53/54).

---

## §8 Erro a não repetir

P205C teve uma lição importante: a minha spec abriu a
porta a "Caminho C — adiar" como honestidade, mas
ADR-0074 já tinha fixado materialização — o Claude Code
rejeitou C correctamente.

P205D **inverte essa lição**: ADR-0074 declara P205D
**condicional**, não fixa materialização. Caminho B
(adiar) é **legítimo se C1.5 mostrar zero benefício
observável**.

Risco específico: **inflar P205D por simetria com
P205B/P205C** (replicar `SealedLabelPages` análogo
porque a infraestrutura é trivial). Se C1.5 mostrar
zero benefício, simetria sem benefício é over-engineering.

Outro risco: **assumir benefício sem verificar**.
Vanilla pode tracker label_pages, mas vanilla tem
arquitectura assimétrica (per `P205A.div-1`). Cristalino
single-pass pode não beneficiar do mesmo.

Hipótese mais provável: C1.5 mostra **zero benefício
empírico observável** porque consumers actuais usam
`doc.extracted_label_pages` directamente em PDF
export/outline sem fricção. **Caminho B (adiar) é
resultado provável**.

Mas é hipótese, não decisão. C2 fixa-se com base em
C1.

---

## §9 Particularidade — execução

P205D é trabalho focado mas com magnitude variável:

- Caminho B (adiar): trabalho documental S (apenas
  inventário + ADR anotação). ~15–20 min.
- Caminho A (materializar): trabalho de código S
  (paralelo a P205B). ~30–40 min.

Recomendado Claude Code dado:

- Investigação de consumers + benefício (C1.5) é
  empírica.
- Decisão Caminho A vs B é honesta — não inflar por
  simetria.

Sessão actual viável especialmente se Caminho B for
escolhido (trabalho mínimo).
