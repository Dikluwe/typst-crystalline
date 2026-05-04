# Relatório consolidado — Série P195

**Período**: 2026-05-04 (P195A–E executados no mesmo dia)
**Magnitude agregada**: M (dominado por P195D; outros são S)
**Estado**: ✅ Série fechada (A ✅ B ✅ C ✅ D ✅ E ✅)
**ADR vinculada**: ADR-0069 — `ACEITE` em P195E.

---

## §1 Resumo executivo

A série P195 materializou o **pattern arquitectural
post-recursion tag emission for state-dependent payload**
(ADR-0069 ACEITE). Walk arm `Content::Labelled` modificado
para emitir Tag manualmente após recursão, populando
sub-store `intr.resolved_labels` (P193B) sem usar o
mecanismo `extract_payload` puro pre-recursion.

Decisão arquitectural central: `extract_payload` é função
pura sem acesso a state — não pode replicar lógica
state-dependent (counter formatting, lang). Em vez de
refactor major do contrato, P195 introduz pattern
alternativo aplicável a walk arms cujo payload depende de
state mutado durante walk recursivo.

Custo real:
- P195A: diagnóstico (8 cláusulas).
- P195B: +5 LOC variant + stub + ADR PROPOSTO + tests.
- P195C: +10 LOC arm funcional + tests.
- P195D: ~30 LOC walk arm + ~50 LOC helper +
  4 tests E2E.

Δ tests cumulativo: **+13** (1825 → 1838) com **zero
regressões**.

ADR-0069 transitou `PROPOSTO` → `ACEITE` em P195E.

**E4 fecha estruturalmente** — caminho Introspector activa
para explicit labels. **Funcionalmente fecha em M6** quando
mutação legacy for removida (write paralelo preservado
durante janela compat M5).

**Inversão observable parcial em produção**: explicit
labels (Heading/Figure/Equation com `Content::Labelled`
wrapper) via Introspector path; Heading auto-toc continua
legacy (E2 activa, fecha em P196).

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados | ADRs |
|-------|---------------------|-----------------|---------|-------------|------|
| **P195A** | S (diagnóstico) | S | 0 | nenhum (cria diagnóstico) | — |
| **P195B** | S | S | **+5** | `entities/element_payload.md` + `rules/introspect/from_tags.md` | ADR-0069 PROPOSTO |
| **P195C** | S | S | **+4** | `rules/introspect/from_tags.md` | — |
| **P195D** | M | M | **+4** | `rules/introspect.md` | — |
| **P195E** | S (documental) | S | 0 | nenhum (apenas ADR + relatório) | ADR-0069 ACEITE |
| **Total** | — | — | **+13** | 3 L0 produção | 1 ADR transitada |

Detalhe Δ tests: 1825 (baseline P194B) → 1830 (P195B) →
1834 (P195C) → 1838 (P195D) → 1838 (P195E sem código).

---

## §3 Decisões arquiteturais

### ADR-0069 transitou PROPOSTO → ACEITE

Justificação literal registada no `Histórico`:
- P195D 4 tests E2E `mod p195d_walk_labelled` passam.
- Paridade observable preservada via mutação legacy
  paralela.
- Sincronização ADR-0068 mantida via reuso de Location
  do target (snapshot+find_map).
- Critério §6 da ADR cumprido.

### 7 cláusulas P195A fechadas + decisão Locator P195D

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma do payload | **Opção 1-modificada** — variant `Labelled` + walk emite Tag pós-recursão | P195B |
| 2 | Ordem | Não aplicável (sem janela invariante quebrada) | — |
| 3 | Estrutura | **Opção α** estrutura única `{ label, resolved_text, figure_number }` | P195B |
| 4 | Auto-toc vs explicit | E4 (Labelled) migra; E2 (Heading auto-toc) fica para P196 | P195D |
| 5 | resolved_text computation | walk arm computa via helper privado `compute_labelled` | P195D |
| 6 | figure_label_numbers | `from_tags` arm popula ambos sub-stores (write paralelo redundante com P168) | P195C |
| 7 | Critério fecho | E4 fecha estruturalmente; funcionalmente em M6 | P195E |
| **+1** (P195D `.A.3`) | Locator handling | Snapshot `tags.len()` + find_map para reuso de Location do target — preserva ADR-0068 sincronização | P195D |

---

## §4 Achados não-triviais durante execução

### P195A §11.1 — Bloqueador arquitectural crítico

`extract_payload(content: &Content) -> Option<ElementPayload>`
é função pura sem parâmetro `state`. Walk arm Labelled
depende de `state.format_hierarchical`, `state.get_flat`,
`state.figure_numbers`, `state.lang` — replicar em
extract_payload exigiria refactor major do contrato +
8 implementações.

**Opção 1 padrão impossível**. ADR-0069 documenta solução
alternativa (post-recursion emit).

### P195A §11.2 — Pattern arquitectural novo documentado

ADR-0069 PROPOSTO em P195B; ACEITE em P195E. Aplicabilidade
futura registada (P196 Heading auto-toc, P197 Figure, P198
walks state-dependent).

### P195A §11.4 — E4 fecha estruturalmente, não funcionalmente

P195 introduz caminho Introspector mas mantém mutação
legacy paralela (write paralelo M5). E4 funcionalmente
fecha em M6 quando legacy for removido. Documentação
honesta em §5 (Estado dormente vs activo).

### P195A §11.6 — Helper `compute_labelled` proposto

Função privada (sem `pub`) em `introspect.rs:323-368`
materializada em P195D. Replica match legacy sem mutação;
reuso entre legacy mutation + populate Tag; reduz
duplicação literal.

### P195D `.A.3` — Decisão Locator: snapshot+find_map reuso

Análise revelou bloqueador potencial:
- Se walk arm Labelled chamasse `locator.next()`, walk
  Locator avançaria 1 a mais que Layouter Locator (que
  mantém `is_locatable=false` para Labelled).
- ADR-0068 sincronização-por-construção quebrada para
  todos os locatables APÓS uma Labelled.

**Solução**: snapshot `tags.len()` antes da recursão; após
recursão, `find_map` para primeira `Tag::Start` no range
novo extrai a Location do target; P195D reusa essa
Location para a Tag Labelled.

```rust
let tags_len_before = tags.len();
walk(target, ...);
let target_loc = tags[tags_len_before..]
    .iter()
    .find_map(|t| if let Tag::Start(l, _) = t { Some(*l) } else { None });
```

Resultado: walk Locator não avança para Labelled; Layouter
Locator não avança para Labelled; sequências sincronizadas.
Caso edge target não-locatable: find_map retorna None;
Tag não emitida; sub-store via Tag não populated; mutação
legacy preservada cobre.

### P195D — 5 cláusulas gate substancial declaradas; nenhuma activa

Spec previu 5 riscos potenciais:
- Locator dessincronização → resolvido via Opção (a).
- Helper exige state não-disponível → não aconteceu.
- Tests timing → não aconteceu.
- Tests sentinela E4 P189B regridem → não regrediram
  (mutação legacy preservada).
- compute_labelled diverge da legacy → replica literal.

Nenhum risco activou-se em prática. Spec defensiva foi
suficiente.

---

## §5 Estado dormente vs activo (secção dedicada)

### Activo após P195E (em produção real)

**Explicit Labelled → Introspector path**:
- `intr.resolved_labels` populated via P195D Tag para
  labels explícitas (`Content::Labelled { target, label }`).
- Consumer C4 (P194B) recebe `Some(text)` do
  Introspector; `or_else` fallback legacy não chamado
  mas continua funcional como backup.
- Output observable: idêntico ao actual (mutação legacy
  paralela fornece valores idênticos).

**Figure label numbers**:
- `intr.figure_label_numbers` populated paralelamente
  por:
  - P168 arm Figure (write desde 2026-05-01).
  - P195D arm Labelled (write desde 2026-05-04).
- Write paralelo redundante mas inofensivo (mesmo valor).

### Continua legacy (E2 ainda activa)

**Heading auto-toc → state.resolved_labels[auto-toc-N]**:
- Walk arm Heading muta directamente
  `state.resolved_labels` para chave `auto-toc-N`.
- Consumer C4 recebe `None` do Introspector path para
  auto-toc labels; `or_else` chama legacy → `Some(text)`.
- Output observable preservado por fallback.

**Fecha em P196** (Heading walk arm migration). Pattern
ADR-0069 aplicável. Após P196:
- E2 fecha residual.
- `intr.resolved_labels` populated universalmente (auto-toc + explicit).

### Mutação legacy preservada (write paralelo)

Per pattern P181 Bibliography (P181D-P181H):
- Walk arm Labelled muta legacy E também emite Tag
  (P195D).
- Walk arm Heading muta legacy só (P196 migra).
- Consumers M4-fallback (C4 P194B) lêem Introspector
  primeiro com `or_else` legacy.
- Cleanup orgânico em M6 quando todos os walks migrarem
  e legacy puder ser removido.

### Janela compat M6

Após:
1. P195D ✅ (E4 estruturalmente fechada).
2. P196 (E2 fecha residual).
3. P197 (E3 fecha — Figure write paralelo P195D + P168
   torna-se path único Tag).
4. P198 (E5+E6 fecham).
5. SetEquationNumbering (E1 fecha).

Todos os walks puros → `state.resolved_labels`,
`state.figure_label_numbers`, `state.figure_numbers`,
`state.flat`, `state.hierarchical`, `state.numbering_active`
removíveis em M6.

---

## §6 Estado final M9 e M5

### M9 (counter-feature) — inalterado: 11/11

P195 não introduz feature M9 nova. Pattern post-recursion
é trabalho M5.

### M5 — incremental

**Antes P195**: 1 arm migrado (Outline P189B); 6 excepções
activas.

**Após P195E**:
- 1 arm migrado completamente (Outline).
- 1 arm migrado parcialmente (Labelled — estruturalmente;
  funcionalmente em M6).
- **5 excepções activas**: E1, E2, E3, E5, E6.
- E4 estruturalmente fechada.

### `ElementPayload` — 10 → **11 variants** (Labelled)

### `ElementKind` — 9 (inalterado — ADR-0069 bypass locatable)

### Trait `Introspector` — 19 métodos (inalterado vs P193B)

### `TagIntrospector` sub-stores — 8 (inalterado)

### M5/M4 (read-sites) — 8/12 (inalterado)

P194B C4 migration foi consumer migration; P195 é walk arm
migration (paralelo).

---

## §7 Estado final lacunas

- **Lacuna #3** (`headings_for_toc` sub-store): activa
  ainda. Independente de P195. Necessária para fechar
  parte residual de E2 em P196.
- Outras lacunas: inalteradas.

---

## §8 Pendências cumulativas + DEBT M5-residual

### DEBT M5-residual (Cenário B)

**Sem DEBT formal aberto**. Nota actualizada per P194 §7:

> **Antes P195**: 6 excepções activas (E1, E2, E3, E4,
> E5, E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P195E**: **5 excepções activas** (E1, E2, E3,
> E5, E6); **2 pré-requisitos restantes**
> (`headings_for_toc`, `SetEquationNumbering`).
>
> **E4 estruturalmente fechada** (Introspector path
> activa para explicit labels; mutação legacy preservada
> como fallback). Funcionalmente E4 fecha em M6 quando
> mutação legacy for removida.
>
> Cadeia E2 (Heading auto-toc) destranca em P196 com
> pattern ADR-0069 aplicado. E3 (Figure) e E5/E6
> (SetHeadingNumbering+CounterUpdate) destrancam em
> P197/P198.

DEBT M5-residual continua em Cenário B.

### ADR-0069 ACEITE

Primeira ADR ACEITE da fase M5 incremental.
ADR-0068 (M3 Locator/sync) já estava ACEITE desde P185E.

ADR-0069 documenta pattern reutilizável para 3 passos
futuros (P196/P197/P198). Reduz incerteza arquitectural.

---

## §9 Próximos passos sugeridos

### Sequência continua (per P189 §9 + ADR-0069)

1. **P196 — walk arm Heading auto-toc**: emite Tag
   pós-recursão para popular
   `intr.resolved_labels[auto-toc-N]`. Pattern ADR-0069
   aplicado. **E2 fecha residual**. Magnitude S–M
   (depende de decisão sobre payload format —
   provavelmente reusa `ElementPayload::Labelled` ou
   variant similar).

2. **P197** — walk arm Figure: write paralelo P195D já
   popula `figure_label_numbers`; resta migrar
   `state.figure_numbers` mutation. **E3 fecha**.

3. **P198** — walks `SetHeadingNumbering` +
   `CounterUpdate`: write paralelo via P182C arm
   StateUpdate já popula StateRegistry; resta remover
   mutação legacy. **E5+E6 fecham**.

4. (Independente) — `Content::SetEquationNumbering`
   materialização. **E1 fecha**.

Após sequência: M5 universalmente fechado; segue M6
(eliminação `CounterStateLegacy`).

### Independente

- Sub-store `headings_for_toc` (lacuna #3) — passo
  dedicado paralelo a P196.

---

## §10 Conclusão

P195 fechou em 5 sub-passos com magnitude correctamente
estimada (4×S + 1×M agregado). **ADR-0069 ACEITE** —
pattern post-recursion tag emission disponível para passos
futuros.

Achados centrais:
- **Bloqueador `extract_payload` puro** para state-dependent
  payload identificado em P195A; pattern alternativo
  formalizado em ADR-0069.
- **Locator sync preservado** via reuso de Location do
  target (P195D `.A.3` snapshot+find_map). ADR-0068
  intacto.
- **Helper `compute_labelled` materializado** — reduz
  duplicação entre mutação legacy e populate Tag.
- **E4 estruturalmente fechada** — Introspector path
  activa para explicit labels. Funcionalmente em M6.
- **Mutação legacy preservada** como write paralelo
  durante janela compat M5. Cleanup orgânico em M6.

Diferença vs P186 (Equation locatable): P186 promoveu
locatable kind canonical; P195 estabeleceu pattern
alternativo (não-locatable, post-recursion). Ambos são
aplicáveis conforme natureza do payload (state-independent
vs state-dependent).

A série P195 termina como **primeira aplicação do pattern
ADR-0069**. Precedente para P196/P197/P198 — cada um
aplica conforme necessário sem decisão arquitectural nova.

**69 passos executados** após P195E. Padrão
diagnóstico-primeiro mantido — 17/17 acertaram a magnitude
planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A/194A/195A).

Próximo passo sugerido: **P196A** — diagnóstico walk arm
Heading auto-toc. Magnitude S esperada (replica P195A
pattern); pattern ADR-0069 já estabelecido reduz
incerteza. **E2 fecha residual** após P196B+.
