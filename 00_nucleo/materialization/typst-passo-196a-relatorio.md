# Relatório P196a — Diagnóstico walk arm Heading auto-toc migration

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico); P196B+ implementação **M agregado**.
**Pré-condição**: P195E concluído ✅; ADR-0069 ACEITE;
sub-store `intr.resolved_labels` populated por P195D para
explicit labels.

---

## §1 Escopo

P196A é o passo de diagnóstico-primeiro que precede a
migração walk arm Heading auto-toc. Replica registo de
P181A/.../P195A.

P196 é **passo 4 da sequência §9 P189 consolidado** —
**segunda aplicação concreta do pattern ADR-0069**
(P195D foi a primeira). Pattern já estabelecido reduz
incerteza arquitectural.

P196A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-walk-heading-passo-196a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-196a-relatorio.md` (este, 14 secções).

Sem ADR (pattern ADR-0069 reusado). Sem DEBT formal
(Cenário B continua).

---

## §2 Inputs verificados empiricamente (12 reads/greps)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Walk arm Heading actual | `introspect.rs:347-379` — 4 mutações empíricas |
| 2 | `Content::Heading` variant | `{ level: u8, body: Box<Content> }` (sem label directo) |
| 3 | Mutação 1 | `state.step_hierarchical("heading", *level)` |
| 4 | Mutação 2 | `state.auto_label_counter += 1` |
| 5 | Mutação 3 | `state.resolved_labels.insert(auto_label, text)` |
| 6 | Mutação 4 | `state.headings_for_toc.push((auto_label, frozen_body, level))` (E2-residuo — lacuna #3) |
| 7 | Auto-label format | `Label(format!("auto-toc-{}", state.auto_label_counter))` |
| 8 | Resolved text computation | `format_hierarchical + "Secção {}"` ou `""` (unwrap_or_default) |
| 9 | `is_locatable(Heading)` | `true` (P164) — walk Locator avança; `emitted_loc: Some(loc)` em scope |
| 10 | `ElementPayload::Labelled` (P195B) | cobre semanticamente auto-toc com `figure_number: None` |
| 11 | `from_tags` arm Labelled (P195C) | já popula `intr.resolved_labels` — reusa-se directamente |
| 12 | Lacuna #3 `headings_for_toc` | sub-store ausente; consumer outline.rs:24 lê directamente; mutação 4 fica residual |

Crítico: **`emitted_loc` em walk fn scope é acessível em
arms** (Location é Copy). P196 reusa directamente — sem
necessidade de snapshot+find_map (mais simples que P195D).

---

## §3 Decisões cláusulas 1–7 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma do payload | **Opção 1** reusar `ElementPayload::Labelled` (P195B) — sem variant nova, sem ADR nova |
| 2 | Helper `compute_heading_auto_toc` | função privada análoga a `compute_labelled` (P195D) |
| 3 | Chave auto_label | `Label(format!("auto-toc-{}", N))` — paridade legacy literal |
| 4 | `headings_for_toc` residual | manter como **E2-residuo** com 4 pontos de documentação |
| 5 | Locator handling | reuso directo de `emitted_loc` (Heading locatable; mais simples que P195D) |
| 6 | Mutação legacy | preservada paralela durante janela compat M5 |
| 7 | Critério fecho | E2 fecha **3 das 4 mutações** estruturalmente; E2-residuo continua para passo dedicado |

---

## §4 Plano de sub-passos B–C (sem condicionais)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Heading + helper `compute_heading_auto_toc` + Tag emit pós-recursão + comentário inline E2-residuo + tests E2E + L0 actualizado | M |
| `.C` | Relatório consolidado P196 + actualização nota DEBT M5-residual | S |

Total agregado: ~80 LOC walk arm + ~50 LOC helper + ~150
LOC tests + edits L0 + relatório consolidado ≈ **M**.

---

## §5 Magnitude agregada

**P196 série = M agregado** (1×M + 1×S em 2 sub-passos).

Maior que P194 (S agregado) porque envolve walk arm
modification + helper. Igual a P186/P195 em magnitude
per sub-passo principal.

Pattern ADR-0069 reduz incerteza face a P195A.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- Sub-store `ResolvedLabelStore` aberto (P193B).
- Trait method `resolved_label_for` disponível (P193B).
- Consumer C4 migrado (P194B).
- `ElementPayload::Labelled` variant (P195B).
- `from_tags` arm Labelled funcional (P195C).
- Walk arm Labelled emite Tag pós-recursão (P195D).
- ADR-0069 ACEITE (P195E).

### §6.2 — Dependentes

- **E2 fecha 3 das 4 mutações estruturalmente** após
  P196B.
- **E2-residuo** (`headings_for_toc.push`) continua até
  passo dedicado abrir sub-store.
- E3, E5, E6 destrancam para migração em P197/P198 com
  pattern ADR-0069 aplicado.

### §6.3 — Independente

- Sub-store `headings_for_toc` (lacuna #3) — passo
  dedicado paralelo. Quando aberto, fecha E2-residuo.
- `Content::SetEquationNumbering` materialização —
  paralelo. E1 fecha.

---

## §7 ADR avaliação

**Sem ADR criada.** Pattern ADR-0069 reusado:
- Pattern post-recursion tag emission for state-dependent
  payload: aplicável aqui (Heading auto-toc payload
  depende de `state.format_hierarchical("heading")`).
- Aplicação difere de P195D em detalhes (Heading
  locatable; Locator reuso via `emitted_loc`) mas decisão
  arquitectural é a mesma.

P196 é **segunda aplicação concreta** do pattern;
estabelece precedente para P197/P198.

---

## §8 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada para
relatório consolidado P196:

> **Antes P196**: 5 excepções activas (E1, E2, E3, E5,
> E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P196B**: **4 excepções activas + 1 resíduo**:
> - E1 — Reserva 1.
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3
>   bloqueia fechamento total).
> - E3 — Figure.
> - E5 — SetHeadingNumbering.
> - E6 — CounterUpdate.
>
> **2 pré-requisitos restantes**:
> - `headings_for_toc` (sub-store). **Fecha
>   E2-residuo**.
> - `SetEquationNumbering`. **Fecha E1**.
>
> Cadeia E2 estruturalmente desbloqueada — E3, E5, E6
> agora migráveis com pattern ADR-0069 aplicado em
> P197/P198.

DEBT M5-residual continua em Cenário B.

---

## §9 Restrições honradas

- **Zero código tocado**.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica walk** — P196B+.
- **Não toca `from_tags`** — P196B+.
- **Não modifica trait `Introspector`** — P185B fechou.
- **Não modifica `TagIntrospector`** — P193B fechou.
- **Não modifica consumer C4** — P194B fechou.
- **Não migra walk arm Figure** — P197.
- **Não abre sub-store `headings_for_toc`** — passo
  dedicado paralelo.
- **Sem inflação retórica**.
- **Aplicar regra dos 2 eixos** ✅ — §1.12 do diagnóstico.
- **Reaproveitar pattern ADR-0069** ✅ — sem decisão
  arquitectural nova.
- **Sem cláusulas condicionais** nos sub-passos.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.838** inalterado
  vs P195E.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR nova.
- ✅ Sem DEBT formal.

---

## §11 Achados não-triviais

### §11.1 — `emitted_loc` em scope simplifica P196 vs P195D

P195D (Labelled não-locatable) precisou de snapshot
`tags.len()` + find_map para extrair Location do target.
Complexidade adicional.

P196 (Heading locatable) tem `emitted_loc: Option<Location>`
acessível em scope do arm. `if let Some(loc) = emitted_loc`
é trivial. **Mais simples que P195D**.

Esta diferença reflecte: locatable = walk top já alocou
Location; arm pode reusar. Não-locatable = walk top não
alocou; arm precisa de extrair via inspecção de tags.

### §11.2 — `ElementPayload::Labelled` cobre auto-toc

Decisão de cláusula 1 — reuso do variant existente —
funciona porque:
- `label`: pode receber `auto-toc-N` (Label aceita
  qualquer string).
- `resolved_text`: texto auto-toc.
- `figure_number`: `None` para Heading auto-toc.

Sem variant nova evita refactor + ADR. P196 é mais
económico que se exigisse `ElementPayload::HeadingAutoToc`
distinto.

### §11.3 — Lacuna #3 mantida — E2 não fecha completamente

Mutação 4 (`state.headings_for_toc.push`) **continua
activa** porque sub-store ausente. Fechamento total de
E2 exige passo dedicado abrir sub-store
`intr.headings_for_toc`.

P196 documenta como **E2-residuo** com 4 pontos de
documentação obrigatória (replica P189B pattern para
excepções residuais).

### §11.4 — Counter mutations 1+2 continuam activas em produção

Walk arm faz:
```rust
state.step_hierarchical("heading", *level);
state.auto_label_counter += 1;
```

Estas mutações **continuam** após P196 (não são
migradas para Tag — counters são write-only durante
walk; consumers C1/C2 leem via Introspector mas pelo
Locator-aware path P185).

P196 trata mutações 1+2 como write paralelo necessário.
Não conflita com Tag emit (que cobre mutação 3).

### §11.5 — Tag auto-toc emitida com mesma Location que Heading tag

Walk top emite `Tag::Start(loc, heading_payload)` para
Heading. Arm Heading depois emite `Tag::Start(loc,
labelled_payload)` para auto-toc. Ambas com **mesma
Location**.

`from_tags` processa em ordem:
1. `Tag::Start(loc, heading_payload)` → arm Heading
   (P170): `kind_index[Heading].push(loc)`;
   `counters.apply_hierarchical_at("heading", depth, loc)`.
2. `Tag::Start(loc, labelled_payload)` → arm Labelled
   (P195C): `intr.resolved_labels.insert(auto-toc-N, text)`.

Sub-stores diferentes — sem conflito. Locação compartida
preserva ADR-0068 sincronização.

### §11.6 — Inversão observable parcial após P196B

Após P196B em produção:
- Consumer C4 recebe `Some(text)` do Introspector path
  para auto-toc labels (`auto-toc-N`).
- Consumer C4 recebe `Some(text)` do Introspector path
  para explicit labels (P195D já cobriu).
- Consumer C4 recebe `Some(text)` para figure-ref
  (P168 já cobriu).
- **Caminho Introspector universal** para resolved
  labels após P196.

`or_else` fallback legacy raramente disparado em
produção; continua funcional como backup.

---

## §12 Snapshot pós-P196A

- **Tests workspace**: 1.838 (inalterado).
- **Trait `Introspector`**: 19 métodos.
- **`TagIntrospector` sub-stores**: 8.
- **`ElementPayload`**: 11 variants.
- **`ElementKind`**: 9.
- **DEBT M5-residual**: 2 pré-requisitos pendentes;
  5 excepções activas (após P196B: 4 + 1 residuo).
- **70 passos executados** (P195E = 69 + P196A = 70).
- **Padrão diagnóstico-primeiro**: 18ª aplicação consecutiva.

---

## §13 Próximo passo

**P196B** — walk arm Heading auto-toc + helper +
documentação E2-residuo:

1. Editar `01_core/src/rules/introspect.rs`:
   - Adicionar helper `compute_heading_auto_toc`.
   - Modificar walk arm Heading:
     - Manter 4 mutações legacy.
     - Após `walk(body, ...)`: emitir Tag auto-toc
       reusando `emitted_loc` + `ElementPayload::Labelled`.
   - Comentário inline E2-residuo na mutação
     `headings_for_toc.push`.

2. Editar L0 `rules/introspect.md`:
   - Secção Excepções M5 actualizada (E2 →
     E2-residuo).
   - Secção "Walk arm Heading migrado (P196B, ADR-0069)".

3. Tests E2E em `mod p196b_walk_heading`:
   - `heading_auto_toc_walk_emite_tag_e_popula_introspector`.
   - `heading_auto_toc_paridade_legacy_vs_introspector`.
   - `heading_auto_toc_numbering_inactivo_emite_string_vazia`.
   - `walk_e2_residuo_headings_for_toc_via_legacy`.
   - `consumer_c4_recebe_some_para_auto_toc_label`.

4. P196C: relatório consolidado P196 + actualização DEBT.

Magnitude: **M genuíno**.

---

## §14 Conclusão

P196A fechou 7 cláusulas com decisão literal e plano em
2 sub-passos B–C. Magnitude S agregada para diagnóstico;
implementação P196B é **M**.

Achados centrais:
- **Reuso de `ElementPayload::Labelled`** evita variant
  nova — Opção 1 cobre semanticamente auto-toc.
- **`emitted_loc` directo** simplifica face a P195D
  (snapshot+find_map). Heading locatable é vantagem.
- **E2-residuo** declarado — `headings_for_toc.push`
  fica activa com 4 pontos de documentação até passo
  dedicado abrir sub-store.
- **Pattern ADR-0069 reusado** — segunda aplicação
  concreta. Estabelece precedente para P197/P198.
- **Inversão observable parcial completa** após P196B —
  caminho Introspector universal para resolved labels.

Padrão diagnóstico-primeiro mantido — 18/18 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A/194A/195A/196A).

Próximo passo: **P196B** — walk arm Heading materialização
+ tests E2E + L0.
