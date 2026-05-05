# Relatório P200A — Diagnóstico sub-store `intr.headings_for_toc`

**Data**: 2026-05-04
**Magnitude**: S puro (diagnóstico-primeiro)
**Estado**: Completo
**Pattern arquitectural relevante**: ADR-0069 — trabalho híbrido (combinação de variantes existentes; sem nova variante operacional).

---

## §1 Sumário executivo

P200A audita o último pré-requisito paralelo restante para
**M5 universal completo**: sub-store `intr.headings_for_toc`
para fechar **E2-residuo** (lacuna #3 declarada desde P189B e
formalizada em P196B).

Auditoria empírica confirma:
- **Sub-store ausente** em `TagIntrospector`.
- **Walk arm Heading mutação 4** localizada em `introspect.rs:486`.
- **Type signature legacy**: `Vec<(Label, Content, usize)>` (não `u8` como mencionado na instrução).
- **Consumer único**: `outline.rs:24`.
- **Layouter assignments**: `mod.rs:1490, 1521` (state legacy → counter — funciona com write paralelo M5).
- **TagIntrospector** com 8 sub-stores; trait com 19 métodos.

P200B é trabalho **híbrido** combinando 3 padrões testados:
- **Sub-store novo** (replica P193B `ResolvedLabelStore`).
- **Variant Tag pós-recursão locatable** (variante P196B).
- **Consumer migration substitution-with-fallback** (replica P184D / P194B).

P200 implementação granular em **2 sub-passos**:
- **P200B** (M+): sub-store + trait method + variant + helper + walk arm + from_tags + consumer + L0 + tests.
- **P200C** (S): consolidado + DEBT + **marco M5 universal**.

Após P200 fechar: **0 excepções activas + 0 residuos + 0 pré-requisitos**. **M5 universal completo pela primeira vez desde P189B**. Desbloqueia M6 (P190A reescrita do zero).

---

## §2 Contexto

P200 é **passo paralelo final** fora série §9 P189. Após P199B
materializar Reserva 1, último pré-requisito M5-residual é
sub-store `headings_for_toc`.

**Estado pré-P200**:
- 0 excepções activas + 1 residuo (E2-residuo).
- 1 pré-requisito paralelo restante (lacuna #3).
- Pattern ADR-0069 com 5 variantes operacionais consolidadas.
- 6 aplicações ADR-0069 stylesheet.

**Esperado pós-P200B**:
- **0 excepções + 0 residuos + 0 pré-requisitos**.
- **M5 universal completo** — desbloqueia M6.
- TagIntrospector: 8 → 9 sub-stores.
- Trait Introspector: 19 → 20 métodos.
- ElementPayload: 12 → 13 variants.

---

## §3 Estado actual confirmado empiricamente

| Componente | Estado |
|------------|--------|
| Sub-store `intr.headings_for_toc` | ❌ ausente |
| Type legacy `state.headings_for_toc` | ✅ `Vec<(Label, Content, usize)>` |
| Walk arm mutação 4 | ✅ `introspect.rs:486` (E2-residuo) |
| Comentário inline P196B sobre E2-residuo | ✅ `introspect.rs:461-484` |
| Consumer outline.rs:24 | ✅ `let entries: Vec<_> = layouter.counter.headings_for_toc.clone();` |
| Layouter assignments (mod.rs:1490, 1521) | ✅ via state legacy assignment |
| `compute_heading_auto_toc` P196B | ✅ produz `(Label, String)` — distinta de helper P200B |
| ElementPayload variants | 12 (after P198C) |
| ElementKind variants | 10 |
| Trait Introspector métodos | 19 (after P185B) |
| TagIntrospector sub-stores | 8 (after P198D) |

---

## §4 Decisões cláusula 1–9

| # | Cláusula | Decisão | Magnitude |
|---|----------|---------|-----------|
| 1 | Forma sub-store | Opção α — `Vec<(Label, Content, usize)>` literal a state legacy | ~5 LOC |
| 2 | Variant Tag | Opção α — `ElementPayload::HeadingForToc { label, body, level }` (12→13) | ~10 LOC |
| 3 | `is_locatable` | Sem arm — Tag pós-recursão usando `emitted_loc` Heading | 0 LOC |
| 4 | Walk arm | 3ª Tag pós-recursão após Tag Labelled auto-toc P196B; mut 4 legacy preservada | ~10 LOC |
| 5 | Helper | `compute_heading_for_toc(state, level, body) -> Option<(Label, Content, usize)>` (4º na família ADR-0069) | ~15 LOC |
| 6 | `from_tags` arm | Push directo em `intr.headings_for_toc` | ~5 LOC |
| 7 | Trait method | `headings_for_toc(&self) -> &[(Label, Content, usize)]` (19→20) | ~8 LOC |
| 8 | Consumer outline | Substitution-with-fallback (padrão P184D/P194B) | ~10 LOC |
| 9 | Critério fecho | E2-residuo fecha; lacuna #3 fecha; **M5 universal completo** | declaração L0 |

---

## §5 Trabalho híbrido (3 categorias combinadas)

P200B combina **3 padrões testados** sem inventar nova variante operacional ADR-0069:

### Categoria A — Sub-store novo (replica P193B)

- Adicionar field em `TagIntrospector`.
- Adicionar trait method.
- Documentar em L0 `entities/introspector.md`.

P193B abriu `ResolvedLabelStore`. P200B abre 9º sub-store
`headings_for_toc`.

### Categoria B — Variant Tag pós-recursão (variante P196B)

- Adicionar variant `ElementPayload::HeadingForToc`.
- Helper privado `compute_heading_for_toc` (4º na família ADR-0069 stylesheet).
- Walk arm Heading emite 3ª Tag pós-recursão usando `emitted_loc` (replica P196B).
- `from_tags` arm popula sub-store.

P196B emitiu Tag::Labelled auto-toc pós-recursão. P200B emite
Tag::HeadingForToc pós-recursão (na mesma Location).

### Categoria C — Consumer migration substitution-with-fallback (replica P184D/P194B)

- Migrar `outline.rs:24` para tentar Introspector path primeiro;
  fallback legacy.

P184D migrou consumer C3 (figure ref-arm). P194B migrou consumer
C4 (text ref-arm). P200B migra consumer outline.

**Sem nova variante operacional ADR-0069** — combinação directa de variantes existentes.

---

## §6 Cláusula gate substancial — cadeia E2-residuo + helpers

`compute_heading_auto_toc` (P196B) lê `state.is_numbering_active("heading")` durante walk para resolver auto-toc text. **Helper distinto do helper P200B** — sub-stores diferentes (`resolved_labels` vs `headings_for_toc`).

Walk arm Heading mutação 4 (`state.headings_for_toc.push`) **preservada** como write paralelo M5 porque:
- Layouter `mod.rs:1490, 1521` faz assignment directo `l.counter.headings_for_toc = initial_state.headings_for_toc`.
- Substitution-with-fallback no consumer outline garante backup mesmo se Tag emit falhar.

**Cláusula gate substancial resolvida sem disparar gate**.

---

## §7 Plano de sub-passos

| Sub | Escopo | Magnitude |
|-----|--------|-----------|
| **P200B** | Sub-store + trait method + variant + helper + walk arm + from_tags + consumer + L0 + 5-6 tests E2E + adaptação 5 tests P196B (4→6 tags por Heading) | **M+** |
| **P200C** | Auditoria + relatório consolidado P200 + DEBT M5-residual + **marco M5 universal** | **S** |

**Total agregado**: M+ (~150 LOC produção + ~180 LOC tests + ~100 LOC L0 + relatórios).

---

## §8 Magnitude consolidada

- **P200A**: S puro. ~250 LOC diagnóstico + relatório.
- **P200B**: M+. ~150 LOC produção + ~180 LOC tests + ~100 LOC L0.
- **P200C**: S puro. ~250 LOC consolidado.

Total agregado: ~830 LOC documentação/relatórios + ~330 LOC código/tests cristalinos.

---

## §9 ADR avaliação

- 5 variantes ADR-0069 cobrem.
- Trabalho híbrido = combinação directa de variantes existentes.
- Sem decisão arquitectural nova.

**Conclusão**: **não cria ADR**.

---

## §10 DEBT M5-residual avaliação

- **Antes P200**: 0 excepções activas + 1 residuo (E2-residuo); 1 pré-requisito restante.
- **Após P200B**: **0 excepções + 0 residuos + 0 pré-requisitos**.

**M5 universal estado**: **completo**. Todos walk arms fechados estruturalmente.

**Marco arquitectural**: primeira vez M5 universal completo desde declaração em P189B.

**Cenário B continua** (sem DEBT formal aberto).

**DEBT M6**: write paralelo M5 ainda activo — `compute_*` helpers leem legacy. Cleanup orgânico em M6 (P190A reescrita do zero — magnitude L cross-modular).

---

## §11 Estado projectado pós-P200

### Activado em P200B

- Sub-store `intr.headings_for_toc` populated via Tag::HeadingForToc.
- `kind_index[ElementKind::Heading]` continua a indexar Heading locations (P196B inalterado).
- Consumer outline.rs:24 recebe entries via Introspector path; fallback legacy raramente disparado.

### Mutação legacy preservada (write paralelo M5)

- Mutação 4 walk arm Heading (`state.headings_for_toc.push`) continua activa.
- Layouter assignments (mod.rs:1490, 1521) inalterados.
- Cleanup orgânico em M6.

### Sequência de tags por Heading (após P200B)

```
Tag::Start(loc, Heading)               // walk top
[recursive body tags]
Tag::Start(loc, Labelled auto-toc-N)   // P196B
Tag::End(loc, 0)
Tag::Start(loc, HeadingForToc)         // P200B (NOVO)
Tag::End(loc, 0)
Tag::End(loc, hash_content(heading))   // walk bottom
```

**6 tags por Heading folha** (era 4 pós-P196B).

---

## §12 Próximo sub-passo concreto

**P200B — Sub-store + Tag + walk arm + consumer (cenário híbrido)**:

1. `entities/introspector.rs`:
   - Field `pub headings_for_toc: Vec<(Label, Content, usize)>` em `TagIntrospector`.
   - Trait method `headings_for_toc(&self) -> &[(Label, Content, usize)]`.
   - Impl em `impl Introspector for TagIntrospector`.

2. `entities/element_payload.rs`:
   - Variant `HeadingForToc { label: Label, body: Content, level: usize }` após `CounterUpdate`.

3. `rules/introspect.rs`:
   - Helper privado `compute_heading_for_toc(state, level, body)`.
   - Walk arm Heading: emitir 3ª Tag pós-recursão (após Tag Labelled auto-toc P196B; mesma `emitted_loc`).
   - Comentário inline P200B substitui parte da nota E2-residuo.

4. `rules/introspect/from_tags.rs`:
   - Arm `HeadingForToc` push directo.

5. `rules/layout/outline.rs:24`:
   - Substitution-with-fallback.

6. L0 (3 ficheiros):
   - `entities/introspector.md`.
   - `entities/element_payload.md`.
   - `rules/introspect.md` (tabela Excepções E2-residuo fechada; lacuna #3 fechada; secção nova "Walk arm Heading mutação 4 fechada P200B").

7. 5-6 tests E2E novos + adaptação 5 tests P196B (4→6 tags por Heading).

8. `crystalline-lint --fix-hashes`.

**Critério de fecho P200B**: tests workspace 1864 + 5 = 1869 verdes (ou +6); lint zero violations; E2-residuo fechada; lacuna #3 fechada; **M5 universal completo**.

---

## §13 Restrições mantidas

- ✅ Zero código tocado em P200A.
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores criadas.
- ✅ Walk não modificado.
- ✅ `from_tags` não tocado.
- ✅ Trait `Introspector` não modificado.
- ✅ `TagIntrospector` não modificado.
- ✅ Consumer outline não modificado.
- ✅ `compute_heading_auto_toc` (P196B) não modificado.
- ✅ Parser sintáctico não materializado.
- ✅ P190A não materializada.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Regra dos 2 eixos aplicada (§1.7 do diagnóstico).
- ✅ Pattern ADR-0069 + 5 variantes reutilizados (sem nova variante).
- ✅ Plano P200B sem cláusulas condicionais.
- ✅ ADR não criada.
- ✅ DEBT formal não aberto.

---

## §14 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069.
- **Trabalho híbrido P200B**: combinação directa de 3 variantes existentes:
  - Sub-store novo (P193B `ResolvedLabelStore`).
  - Variant Tag pós-recursão locatable (variante P196B).
  - Consumer migration substitution-with-fallback (P184D / P194B).
- **5 variantes operacionais ADR-0069 consolidadas** (inalterado em P200).
- **Helper análogo**: `compute_heading_for_toc` (P200B) — 4º na família ADR-0069 stylesheet (após `compute_labelled` P195D, `compute_heading_auto_toc` P196B, `compute_figure` P197B).
- **Sub-store novo**: `intr.headings_for_toc` (9º sub-store em TagIntrospector).
- **Trait method novo**: `headings_for_toc()` (20º método).
- **Variant nova**: `ElementPayload::HeadingForToc` (13ª variant).
- **Consumer migrado**: `outline.rs:24` (3ª migration substitution-with-fallback após C3 P184D + C4 P194B).
- **Cadeia E2-residuo**: walk arm Heading mutação 4 → mut 4 preservada; Tag pós-recursão emite payload com body materializado para outline.
- **L0 alvos**: 3 ficheiros (`entities/introspector.md`, `entities/element_payload.md`, `rules/introspect.md`).
- **Padrão diagnóstico-primeiro**: 22ª aplicação consecutiva.
- **Marco arquitectural projectado pós-P200**: M5 universal completo pela primeira vez desde declaração em P189B; desbloqueia M6 (P190A reescrita do zero).
