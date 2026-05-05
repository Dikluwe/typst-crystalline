# Relatório Consolidado — Série P199

**Data**: 2026-05-04
**Sub-passos**: A ✅ B ✅ C ✅
**Magnitude agregada**: S diagnóstico (P199A) + M materialização (P199B) + S documental (P199C)
**Estado**: Série fechada
**Pattern arquitectural**: ADR-0069 stylesheet — **5ª variante operacional consolidada** (cenário α por construção autonomizado).

---

## §1 Resumo executivo

Série P199 materializa **`Content::SetEquationNumbering`** —
Reserva 1 desde P189B, em espera há mais de 12 séries.
Distinção arquitectural: P199B introduz **cenário α por
construção** (5ª variante operacional ADR-0069 consolidada
como autónoma) — variante nova adicionada cuja infraestrutura
downstream estava planeada antecipadamente e activa
imediatamente.

**Marco arquitectural**: pela primeira vez desde P189B há
**0 excepções M5 activas**:

| Excepção | Estado |
|----------|--------|
| E1 (Equation/SetEquationNumbering) | ✅ fechada (P199B — cenário α por construção) |
| E2 (Heading auto-toc) | parcialmente fechada (P196B — 3/4 mutações; resta E2-residuo) |
| E3 (Figure) | ✅ fechada (P197B — cenário α) |
| E4 (Labelled) | ✅ fechada (P195D — pattern original ADR-0069) |
| E5 (SetHeadingNumbering) | ✅ fechada (P198B — cenário α) |
| E6 (CounterUpdate) | ✅ fechada (P198C — cenário β-promote 1ª aplicação) |

**M5 universal a 1 passo paralelo do fecho** — sub-store
`intr.headings_for_toc` (E2-residuo, lacuna #3) é único
restante.

**Output observable em produção**: ligeiramente alterado
(activação por construção do Layouter `equation.rs:32-33`
first branch antes adormecida) mas **paridade preservada**
via write paralelo legacy.

---

## §2 Sub-passos materializados

| Sub | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-----|-------------------|----------------|---------|-------------|
| **P199A** (diagnóstico) | S puro | S | 0 | 0 |
| **P199B** (materialização α por construção) | M | M | +5 | 1 (`introspect.md`) |
| **P199C** (encerramento) | S puro | S | 0 | 0 |
| **Total série** | **M** agregado | **M** | **+5** | **1 L0 distinto** |

**Totais**:
- 3 sub-passos.
- +5 testes E2E novos (`set_equation_numbering_*`,
  `walk_arm_equation_*`, `consumer_layouter_equation_*`).
- 0 testes existentes adaptados.
- 1 Content variant nova (`SetEquationNumbering`).
- 0 ElementPayload variants novas (reuso `StateUpdate` P171).
- 0 ElementKind variants novas (reuso `StateUpdate` P171).
- 0 sub-stores novos (reuso `StateRegistry` P171).
- 0 ADRs novas (ADR-0069 já ACEITE).
- 0 helpers privados novos na família ADR-0069 stylesheet
  (sem helper porque mutação trivial 1 linha — paralelo a
  P198B).
- 1 helper Layouter novo (`layout_set_equation_numbering`
  em `rules/layout/counters.rs`).
- 1 consumer Layouter novo (arm `Content::SetEquationNumbering`
  em `rules/layout/mod.rs`).

---

## §3 Decisões arquitecturais

7 cláusulas P199A fechadas:

1. **Cláusula 1 — Forma**: Opção α — `SetEquationNumbering { active: bool }` literal a SetHeadingNumbering.
2. **Cláusula 2 — Parser**: Opção α — apenas materialização interna; sem `#set equation` parser sintáctico.
3. **Cláusula 3 — `is_locatable` + `extract_payload`**: replica literal P182C com chave canónica `numbering_active:equation`.
4. **Cláusula 4 — Walk arm**: replica P182C com chave `equation` + comentário inline P199B.
5. **Cláusula 5 — `from_tags`**: sem modificação (arm StateUpdate P171 genérica).
6. **Cláusula 6 — Cadeia E1**: preservar mutação legacy.
7. **Cláusula 7 — Critério fecho**: E1 fecha estruturalmente; M5 universal NÃO fecha (E2-residuo persiste).

**Decisões adicionais durante execução**:

- **Helper Layouter `layout_set_equation_numbering`** adicionado fora escopo P199A (P199B §8). Necessário porque consumer arm em `layout/mod.rs` requer função correspondente. Trabalho trivial (1 linha + comentário) — replica `layout_set_heading_numbering`. Não previsto em P199A diagnóstico mas obrigatório para `cargo check` passar.

- **Cenário α por construção escalado a variante autónoma** (P199B §2). P199A §5 declarou cenário α por construção como sub-variante de cenário α padrão. P199B §2 escalou para 5ª variante operacional ADR-0069 distinguida formalmente — caracteres distintivos:
  - Pré-passo: caminho **activável** (não meramente activo nem inactivo).
  - Trabalho: materializar variant — caminho activa imediatamente porque infra downstream pré-planeada.

- **DEBT-10 introduzida no comentário** do variant. Alinhamento com vanilla StyleChain futuro. Auditor pode formalizar entry em `m1-lacunas-captura.md` se desejar — não obrigatório (DEBT-10 já existe documentada em SetHeadingNumbering).

---

## §4 Achados não-triviais durante execução

### Achado A1 — Layouter substitution-with-fallback antes adormecida (P199A §3)

`layout/equation.rs:32-33` já tinha `intr.is_numbering_active("numbering_active:equation") || self.counter.is_numbering_active("equation")`. First branch retornava `false` em produção pré-P199B porque `intr.state` nunca era populado para chave `numbering_active:equation`. **Activação por construção** ao materializar a variant.

### Achado A2 — 7 match arms exhaustivos induzidos (P199B §8)

Mais do que P198C (4). Cobertura:
- `content.rs:980` (`plain_text`).
- `content.rs:1200` (comparação `eq`).
- `content.rs:1483, 1694` (2 listas de "terminais sem effect em counters").
- `introspect.rs:101` (lista terminais em `materialize_time`).
- `introspect.rs:453` (walk match — coberto adicionando walk arm).
- `layout/mod.rs:257` (Layouter dispatch — coberto adicionando consumer arm).

Auditor cobriu todos via `cargo check` warnings iterativos.

### Achado A3 — Helper Layouter adicionado fora escopo P199A

P199A não previu adição de `layout_set_equation_numbering`. Necessário para consumer arm em `layout/mod.rs`. Trivial (1 linha) mas não-previsto.

### Achado A4 — Cenário α por construção escalado a variante autónoma

P199A §5 declarou como sub-variante; P199B §2 escalou para variante operacional autónoma. Justificação: caracteres distintivos materialmente diferentes do cenário α padrão (variant não existia antes; infra downstream pré-planeada).

### Achado A5 — DEBT-10 introduzida no comentário

Variant comenta DEBT-10 (StyleChain futuro per vanilla typst). Não obrigatória formalização — precedente em SetHeadingNumbering.

### Achado A6 — Reserva 1 materializada após >12 séries

P189B (Reserva 1) → P190..P198 → **P199B** = 12+ séries entre declaração e materialização. Recurso arquitectural notável: pendência mantida explicitamente declarada e endereçada quando a infraestrutura downstream estava pronta.

---

## §5 Estado activo vs preservado

### Activado em P199B (E1)

- **Caminho Introspector para SetEquationNumbering**: StateRegistry populated via Tag::StateUpdate (chave canónica `numbering_active:equation`).
- **Counter Equation activado em CounterRegistry** via gate em `from_tags::Equation` (P186E) antes dormente em produção.
- **Layouter `equation.rs:32-33` first branch activa** em produção real (substitution-with-fallback antes adormecida).
- **`compute_labelled` Equation arm produz `Some("Equação (n)")`** para Equation labels via legacy.
- **`intr.resolved_labels[label]` populated** via Tag::Labelled P195D para Equation labels.

### Mutação legacy preservada (write paralelo M5)

- **SetEquationNumbering**: 1 mutação preservada (`state.numbering_active.insert("equation", *active)`). Necessária porque:
  - Walk arm Equation lê `state.is_numbering_active("equation")` durante walk para gating do counter step.
  - `compute_labelled` Equation arm (P195D) lê `state.get_flat("equation")` durante walk (cadeia indirecta E1 — counter só avança se gate disparar).

### Cleanup orgânico em M6 (P190A reescrita do zero)

Quando walk arm Equation migrar para `is_numbering_active_at` (Introspector path location-aware) ou `compute_labelled` Equation arm migrar para CounterRegistry, mutação legacy pode ser removida.

---

## §6 Estado final M9 e M5

### Marco M9 (Introspector capabilities)

| Métrica | P198C | P199B | Δ |
|---------|-------|-------|---|
| Variants `ElementPayload` | 12 | 12 | 0 |
| Variants `ElementKind` | 10 | 10 | 0 |
| Métodos trait `Introspector` | 19 | 19 | 0 |
| Sub-stores `TagIntrospector` | 8 | 8 | 0 |
| Variants `Content` | (não tracked) | **+1** | +1 (`SetEquationNumbering`) |
| Tests workspace | 1.859 | **1.864** | +5 |

M9 estável — P199 não introduz capabilities novas (reusa toda
infra P171/P182C).

### Marco M5 (walk-puro progressão)

| Arm | Estado pré-P199 | Estado pós-P199 |
|-----|-----------------|-----------------|
| Outline | migrado (P189B) | migrado |
| Bibliography | migrado (P181H) | migrado |
| Labelled | migrado estruturalmente (P195D) | migrado estruturalmente |
| Heading | migrado parcialmente (E2-residuo P196B) | inalterado (E2-residuo persiste) |
| Figure | fechada estruturalmente (P197B) | fechada estruturalmente |
| SetHeadingNumbering | fechada estruturalmente (P198B) | fechada estruturalmente |
| CounterUpdate | fechada estruturalmente (P198C) | fechada estruturalmente |
| **Equation/SetEquationNumbering** | activa (E1, Reserva 1) | **fechada estruturalmente (P199B — cenário α por construção)** |

**Excepções M5 activas após P199**: **0 + 1 residuo** (E2-residuo).
**Marco — 0 excepções activas pela primeira vez desde P189B**.

---

## §7 Estado final lacunas

| # | Lacuna | Pré-P199 | Pós-P199 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa |
| #1b | from_tags arm Figure sem gate `is_counted` | activa | activa |
| #2 | reservada | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa, bloqueia E2-residuo | **única bloqueante restante** |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

Lacunas inalteradas em P199 — fecho de E1 ortogonal a lacunas
existentes. **Lacuna #3 ganha proeminência** como único
pré-requisito restante de M5 universal.

---

## §8 Pendências cumulativas + DEBT M5-residual

### Pendências série P199

- ✅ A — diagnóstico empírico SetEquationNumbering.
- ✅ B — materialização variant + 3 arms + walk arm + Layouter helper/consumer + 5 tests + L0.
- ✅ C — auditoria + relatório consolidado + nota DEBT.

### DEBT M5-residual — estado actualizado

> **Antes P199**: 1 excepção activa + 1 residuo (E1, E2-residuo); 2 pré-requisitos M5-residual restantes.
>
> **Após P199B**: **0 excepções activas + 1 residuo**:
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3 bloqueia fechamento total).
>
> **1 pré-requisito restante**:
> - Sub-store `intr.headings_for_toc` (lacuna #3). **Fecha E2-residuo**.
>
> **E1 fechada estruturalmente** (P199B — cenário α por construção 1ª aplicação). Variant `Content::SetEquationNumbering` materializada após >12 séries de espera (Reserva 1 desde P189B). Caminho Introspector activado imediatamente em produção real (Layouter `equation.rs:32-33` first branch substitution-with-fallback antes adormecida).
>
> **Marco arquitectural**: pela primeira vez desde P189B há **0 excepções M5 activas**. M5 universal a 1 passo paralelo do fecho.
>
> Mutação legacy preservada como write paralelo M5 (`state.numbering_active["equation"]` lido por walk arm Equation + `state.get_flat("equation")` lido por `compute_labelled` Equation arm); cleanup orgânico em M6 (P190A reescrita do zero).

**Cenário B continua** (sem DEBT formal aberto).

---

## §9 Próximos passos sugeridos

### Pré-requisito restante (M5 universal fecha após)

- **Sub-store `intr.headings_for_toc`** (passo paralelo
  independente, lacuna #3): fecha **E2-residuo**. Magnitude
  esperada: M (sub-store novo + arm `from_tags` + variant
  Tag possivelmente nova `ElementPayload::HeadingForToc` ou
  reuso de Labelled/StateUpdate; walk arm Heading emite Tag
  dedicada pós-recursão para a 4ª mutação E2; consumer
  `outline.rs:24` migra). **Único restante** — pode executar
  imediatamente.

### Após M5 universal fechar

- **P190A reescrita do zero — M6 eliminação `CounterStateLegacy`**
  (passo agregado): cleanup do write paralelo M5; remoção
  do struct + dependências; consumer migrações finais
  (`compute_*` helpers eliminados ou migrados para sub-stores
  Introspector location-aware). Magnitude esperada: **L**
  (refactor maior cross-modular). P190A original
  (`typst-passo-185a-relatorio.md` renomeado em série P185)
  declarado obsoleto — escrever do zero baseado no estado
  real consolidado.

---

## §10 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- **5 variantes operacionais consolidadas**:
  - **P195D variante** (não-locatable): snapshot+find_map.
  - **P196B variante** (locatable + body): `emitted_loc` directo.
  - **Cenário α** (P197B Figure, P198B SetHeadingNumbering): refactor estilístico ou declaração formal sem Tag pós-recursão.
  - **Cenário α por construção** (**P199B SetEquationNumbering** — 1ª aplicação autonomizada): materializar variant; caminho activa imediatamente.
  - **Cenário β-promote** (P198C CounterUpdate — 1ª aplicação): promote completo.
- **6 aplicações ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + P198C + **P199B**.
- **Helpers privados família ADR-0069**: 3 (P195D `compute_labelled`; P196B `compute_heading_auto_toc`; P197B `compute_figure`); P198B + P198C + **P199B** sem helper.
- **Template primário P199B**: P182C (`Content::SetHeadingNumbering`) replicado literalmente com chave `equation`.
- **Reuso `ElementPayload::StateUpdate`** (P171/P173) sob chave canónica `numbering_active:equation`.
- **Reuso `from_tags::StateUpdate` arm** (P171) — genérica.
- **Sub-store consumido**: `intr.state` (StateRegistry P171/P182).
- **Consumer Layouter activado**: `equation.rs:32-33` substitution-with-fallback antes adormecida — first branch retorna Some pós-P199B.
- **Cadeia E1**: walk arm Equation (gate counter step em `introspect.rs:517`) + `compute_labelled` Equation arm (P195D format em `introspect.rs:337`) — ambos preservados; lêem state legacy.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `603170c8`.
- **Código tocado**: 7 ficheiros `01_core/src/`:
  - `entities/content.rs` (variant + 4 match arms).
  - `rules/introspect/locatable.rs` (locatable arm).
  - `rules/introspect/extract_payload.rs` (extract arm).
  - `rules/introspect.rs` (walk arm + comentário inline + 1 lista terminais; hash `0092886d`).
  - `rules/layout/counters.rs` (helper novo).
  - `rules/layout/mod.rs` (consumer arm Layouter).

---

## §11 Métricas finais

- **Sub-passos**: 3 (A diagnóstico + B materialização + C encerramento).
- **LOC produção**: ~70 (variant + 3 arms + walk arm + helper Layouter + consumer Layouter + 4 match arms induzidos).
- **LOC teste**: ~150 (5 tests E2E).
- **LOC L0**: ~80 (secção nova + tabela Excepções + ordem inversa).
- **LOC relatórios**: ~870 (3 relatórios + consolidado).
- **Variants Content novas**: +1 (`SetEquationNumbering`).
- **Variants ElementPayload novas**: 0 (reuso StateUpdate).
- **Variants ElementKind novas**: 0 (reuso StateUpdate).
- **Sub-stores novos**: 0.
- **ADRs novas**: 0.
- **Excepções M5 fechadas**: 1 (E1 — Reserva 1 materializada).
- **Tests netos adicionados**: +5.
- **Hashes desactualizados**: 0 (corrigidos por `--fix-hashes` em P199B).

---

## §12 Marco arquitectural

**Sequência §9 P189 cumprida na totalidade + Reserva 1 fechada**:

| Série | Estado | Excepção fechada |
|-------|--------|------------------|
| P193 (sub-store ResolvedLabelStore) | ✅ | (infraestrutura) |
| P194 (consumer C4 Ref-arm) | ✅ | (infraestrutura) |
| P195 (walk arm Labelled) | ✅ | E4 |
| P196 (walk arm Heading auto-toc) | ✅ | E2 → E2-residuo |
| P197 (walk arm Figure) | ✅ | E3 |
| P198 (walks Set + Counter) | ✅ | E5 + E6 |
| **P199** (`SetEquationNumbering` materialização) | **✅** | **E1 (Reserva 1)** |

**7 séries materializadas, 6 excepções fechadas, 1 residuo declarado.**

**0 excepções M5 activas pela primeira vez desde P189B.**

**M5 universal a 1 passo paralelo do fecho** — sub-store
`intr.headings_for_toc` é único restante. Após esse passo,
**M5 universal completo desbloqueia M6** (P190A reescrita do
zero — eliminação `CounterStateLegacy`).

---

## §13 Notas operacionais

- **Tamanho série**: ~220 LOC produção/tests + ~870 LOC documentação.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal aberto**.
- **DEBT-10 referenciada** no comentário do variant — não obrigatória formalização (precedente em SetHeadingNumbering).
- **Padrão replicado**: encerramento série P186/P187/P188/P189/P193/P194/P195/P196/P197/P198 (relatório consolidado 9 secções padrão).
- **Cláusulas gate disparadas**: 0 substanciais (cadeia E1 ↔ walk arm Equation + compute_labelled Equation arm resolvida sem disparar via mutação legacy preservada).
- **Cláusulas gate triviais resolvidas**: 1 (helper Layouter `layout_set_equation_numbering` não previsto em P199A — adicionado em P199B).
- **5 variantes operacionais ADR-0069 consolidadas como catálogo arquitectural completo** — auditor escalou cenário α por construção a variante autónoma em P199B §2.

**Próximo passo**: **P200A** — diagnóstico sub-store
`headings_for_toc`. Trabalho concreto previsto:
- Adicionar sub-store `headings_for_toc: Vec<(Label, Content, u8)>` em `TagIntrospector`.
- Possível variant Tag nova (`ElementPayload::HeadingForToc` ou reuso de Labelled/StateUpdate).
- Walk arm Heading: emitir Tag dedicada pós-recursão (cláusula 4 mutação E2 — última mutação legacy do walk arm Heading).
- `from_tags` arm popula sub-store novo.
- Migrar consumer `outline.rs:24` para ler do Introspector.
- Eliminar mutação 4 legacy → E2-residuo fecha completamente.
- Magnitude esperada: M (similar a P198C β-promote em complexidade — sub-store novo + arm + variant possivelmente).

**Após P200 fechar**: M5 universal completo. Desbloqueia M6 (P190A reescrita do zero — eliminação `CounterStateLegacy`; magnitude L).
