# Relatório P199A — Diagnóstico `Content::SetEquationNumbering`

**Data**: 2026-05-04
**Magnitude**: S puro (diagnóstico-primeiro)
**Estado**: Completo
**Pattern arquitectural relevante**: ADR-0069 — cenário α por construção.
**Template primário**: P182C (`Content::SetHeadingNumbering`).

---

## §1 Sumário executivo

P199A audita a Reserva 1 — `Content::SetEquationNumbering`
ausente desde P189B — para fechar **E1**, último passo paralelo
da série §9 P189 antes de M5 universal completo.

Auditoria empírica confirma:
- **Variant ausente** (apenas referenciada em comentários como Reserva 1).
- **Template P182C totalmente mapeado** (`SetHeadingNumbering`).
- **Caminho Introspector pronto a activar**: arm StateUpdate (P171) genérica processa `numbering_active:equation` transparentemente; consumer Layouter `equation.rs:32` já tem substitution-with-fallback implementada (caminho dorme até P199 materializar variant).
- **Sem parser sintáctico** — Opção α confirmada (apenas materialização interna).

P199 é replicação literal de P182C com chave `equation`. Cenário
α **por construção** — caminho Introspector activa imediatamente
após adição da variant porque toda a infraestrutura (StateRegistry,
arm StateUpdate, consumer com fallback) já existe.

P199 implementação granular em **2 sub-passos**:
- **P199B** (M): variant + 3 arms + walk arm + tests + L0.
- **P199C** (S): consolidado + DEBT.

Após P199 fechar: **0 excepções activas + 1 residuo** (E2-residuo);
**1 pré-requisito restante** (sub-store `headings_for_toc`).

---

## §2 Contexto

P199 é **passo paralelo fora série §9 P189**. Após série
P198 fechada, M5 universal está a 2 pré-requisitos paralelos
do fecho:
- **E1** ↔ `Content::SetEquationNumbering` materialização (P199).
- **E2-residuo** ↔ sub-store `intr.headings_for_toc` (passo paralelo).

P199A é diagnóstico para o primeiro pré-requisito.

**Estado pré-P199**:
- 1 excepção activa + 1 residuo (E1, E2-residuo).
- 2 pré-requisitos paralelos restantes.
- Pattern ADR-0069 com 4 variantes operacionais (P195D, P196B, cenário α, cenário β-promote).

**Esperado pós-P199B**:
- 0 excepções activas + 1 residuo (E2-residuo).
- 1 pré-requisito restante.
- Pattern ADR-0069 com 4 variantes operacionais (inalterado — cenário α por construção é variante operacional do cenário α P197B/P198B aplicada no momento da materialização da variant).

---

## §3 Estado actual confirmado empiricamente

| Componente | Estado |
|------------|--------|
| Variant `Content::SetEquationNumbering` | ❌ ausente |
| Template P182C (`SetHeadingNumbering`) | ✅ totalmente mapeado |
| `is_locatable(SetHeadingNumbering)` | ✅ true (locatable.rs:49) |
| `extract_payload(SetHeadingNumbering)` | ✅ retorna `Some(StateUpdate)` (extract_payload.rs:63) |
| Walk arm SetHeadingNumbering | ✅ existe (introspect.rs:611) |
| `from_tags` arm StateUpdate | ✅ genérica (P171) — sem hardcoded keys |
| Consumer Layouter Equation | ✅ substitution-with-fallback já implementada (equation.rs:32-33) — caminho Introspector dorme até P199 |
| Walk arm Equation (consumer cadeia E1) | ✅ lê `state.is_numbering_active("equation")` durante walk |
| `compute_labelled` Equation arm | ✅ lê `state.get_flat("equation")` (cadeia indirecta E1) |
| Parser sintáctico `#set equation` | ❌ ausente — Opção α |

---

## §4 Decisões cláusula 1–7

| # | Cláusula | Decisão | Magnitude |
|---|----------|---------|-----------|
| 1 | Forma materialização | Opção α — replica literal `SetEquationNumbering { active: bool }` | ~5 LOC |
| 2 | Scope parser | Opção α — apenas materialização interna; sem parser sintáctico | 0 LOC |
| 3 | `is_locatable` + `extract_payload` arms | Replica literal P182C com chave `numbering_active:equation` | ~10 LOC |
| 4 | Walk arm | Replica literal P182C com chave `equation` + comentário inline | ~15 LOC |
| 5 | Reuso `from_tags` arm | Sem modificação (genérica) | 0 LOC |
| 6 | Cadeia E1 ↔ helpers | Preservar mutação legacy (write paralelo M5) | 0 LOC |
| 7 | Critério fecho | E1 fecha estruturalmente; M5 universal NÃO fecha (E2-residuo persiste) | declaração L0 |

---

## §5 Cenário α por construção

**Distinção formal** das 4 variantes operacionais ADR-0069:

| Variante | Pré-passo | Trabalho |
|----------|-----------|----------|
| P195D (não-locatable) | Caminho inactivo | Tag pós-recursão + snapshot+find_map |
| P196B (locatable + body) | Caminho inactivo | Tag pós-recursão + emitted_loc directo |
| Cenário α (P197B, P198B) | Caminho activo | Refactor estilístico ou declaração formal |
| **Cenário α por construção (P199)** | **Caminho activável** | **Materializar variant** — caminho activa imediatamente |
| Cenário β-promote (P198C) | Caminho inactivo | Promote completo (variant nova + locatable + 2 arms) |

**Cenário α por construção** é distinto de cenário α padrão:
- P197B/P198B: variant **já existia**; cenário α era declaração formal/refactor sobre código existente.
- P199: variant **não existia**; cenário α por construção é declaração formal **no momento da materialização** da variant — toda a infraestrutura downstream pronta.

Pode ser argumentada como sub-variante operacional do cenário α (mesma pattern stylesheet) ou como variante operacional nova. **Decisão preliminar**: sub-variante (não justifica nova entry no pattern stylesheet).

---

## §6 Cláusula gate substancial — cadeia E1 ↔ helpers

Walk arm Equation (`introspect.rs:517`) lê `state.is_numbering_active("equation")` durante walk para gate counter step. `compute_labelled` Equation arm lê `state.get_flat("equation")` (cadeia indirecta).

**Risco se mutação legacy for removida em P199B**: cadeia E1 quebra; counter "equation" não avança em legacy; `compute_labelled` Equation arm retorna `(None, None)`; quebra paridade observable.

**Mitigação P199B**: mutação legacy `state.numbering_active.insert("equation", *active)` preservada como write paralelo M5. Cleanup orgânico em M6 quando walk arm Equation migrar para `is_numbering_active_at` (Introspector path location-aware).

**Cláusula gate substancial resolvida sem disparar gate**.

---

## §7 Plano de sub-passos

| Sub | Escopo | Magnitude |
|-----|--------|-----------|
| **P199B** | Variant + 3 arms + walk arm + comentário + 5 tests E2E + L0 | **M** |
| **P199C** | Auditoria + relatório consolidado P199 + DEBT M5-residual | **S** |

**Total agregado**: M.

---

## §8 Magnitude consolidada

- **P199A**: S puro. ~250 LOC diagnóstico + relatório.
- **P199B**: M. ~50 LOC produção + ~120 LOC tests + ~60 LOC L0.
- **P199C**: S puro. ~250 LOC consolidado.

Total agregado: ~730 LOC documentação/relatórios + ~170 LOC código/tests cristalinos.

---

## §9 ADR avaliação

- Pattern ADR-0069 cenário α aplicável directamente.
- Sem decisão arquitectural nova.
- Analogia directa com P182C.

**Conclusão**: **não cria ADR**.

---

## §10 DEBT M5-residual avaliação

- **Antes P199**: 1 excepção activa + 1 residuo.
- **Após P199B**: **0 excepções activas + 1 residuo** (E2-residuo).
- **1 pré-requisito restante** — sub-store `headings_for_toc`. Fecha E2-residuo.

**Marco arquitectural**: M5 universal a **1 passo do fecho** após P199 fechar. Após esse último passo paralelo, M5 universal completo desbloqueia M6 (P190A reescrita do zero — eliminação `CounterStateLegacy`).

**Cenário B continua** (sem DEBT formal aberto).

---

## §11 Estado dormente vs activo (esperado pós-P199B)

### Activo

- Caminho Introspector para SetEquationNumbering: StateRegistry populated via Tag::StateUpdate (chave `numbering_active:equation`).
- Counter Equation activado em CounterRegistry — gate em `from_tags` arm Equation (P186E, linha 230) começa a disparar quando `numbering_active:equation = true` no StateRegistry.
- Consumer Layouter `equation.rs:32` first branch retorna Some via Introspector path; fallback legacy raramente disparado.
- `compute_labelled` Equation arm retorna `(Some("Equação (n)"), None)` para Equation labels.

### Dormente / continua legacy (write paralelo M5)

- `state.numbering_active.insert("equation", ...)` continua activo — walk arm Equation lê durante walk.
- Counter `state.flat["equation"]` continua activo — `compute_labelled` Equation arm lê durante walk.
- Cleanup orgânico em M6.

---

## §12 Próximo sub-passo concreto

**P199B — Materialização `Content::SetEquationNumbering`**:

1. Variant em `entities/content.rs:177` (após SetHeadingNumbering linha 176):
   ```rust
   /// Activa ou desactiva a numeração automática de equations.
   /// Análoga a SetHeadingNumbering (P57). Materializada em P199B.
   /// DEBT-10: substituir por StyleChain quando motor de introspecção
   /// completo for implementado.
   SetEquationNumbering { active: bool },
   ```

2. Cobrir match arms exaustivos via `cargo check` (analogia P198C):
   - `Content::to_string` ou método similar.
   - Comparação `eq`/`partial_cmp`.
   - `materialize_time` em `introspect.rs`.
   - Lista de "terminais sem effect em counters" no walk match (deve incluir).

3. `locatable.rs` — adicionar arm `Content::SetEquationNumbering { .. } => true`.

4. `extract_payload.rs` — arm replica P182C com chave `numbering_active:equation`.

5. `introspect.rs` walk arm — replica P198B com chave `equation` + comentário inline P199B.

6. L0 `rules/introspect.md`:
   - Tabela Excepções M5: E1 fechada estruturalmente.
   - Ordem inversa: passo 9 ✅.
   - Secção nova ou actualização menção paralelo SetEquationNumbering.

7. 5 tests E2E:
   - `set_equation_numbering_extract_payload_emite_state_update`.
   - `set_equation_numbering_from_tags_popula_state_registry`.
   - `set_equation_numbering_paridade_legacy_vs_introspector`.
   - `walk_arm_equation_le_numbering_active_legacy_apos_set`.
   - `consumer_layouter_equation_recebe_some_via_introspector`.

8. `crystalline-lint --fix-hashes`.

**Critério de fecho P199B**: tests workspace 1.864 verdes (1.859 + 5); lint zero violations; variant disponível programaticamente; Layouter equation.rs:32 first branch activa.

---

## §13 Restrições mantidas

- ✅ Zero código tocado em P199A (passo diagnóstico-primeiro).
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores criadas.
- ✅ Walk não modificado.
- ✅ `from_tags` não tocado.
- ✅ Trait `Introspector` não modificado.
- ✅ `TagIntrospector` não modificado.
- ✅ Consumer C3/C4 não modificados.
- ✅ Parser sintáctico não materializado (cláusula 2 Opção α).
- ✅ Sub-store `headings_for_toc` não tocado (passo paralelo independente).
- ✅ P190A não materializada.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Regra dos 2 eixos aplicada (§1.7 do diagnóstico).
- ✅ Template P182C reaproveitado literalmente.
- ✅ Plano P199B sem cláusulas condicionais.

---

## §14 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069.
- **Variante operacional**: cenário α por construção (sub-variante de cenário α P197B/P198B).
- **Template primário**: P182C (`Content::SetHeadingNumbering`).
- **Reuso `ElementPayload::StateUpdate`** (P171/P173) sob chave canónica `numbering_active:equation`.
- **Reuso `from_tags` arm StateUpdate** (P171) — genérica.
- **Sub-store consumido**: `intr.state` (StateRegistry P171/P182).
- **Consumer Layouter**: `equation.rs:32-33` substitution-with-fallback já implementada (caminho dorme até P199B activar).
- **Cadeia E1**: walk arm Equation (introspect.rs:517) + `compute_labelled` Equation arm (P195D introspect.rs:337) lêem state legacy — write paralelo preservado.
- **L0 alvo**: `00_nucleo/prompts/rules/introspect.md` (a actualizar em P199B).
- **Padrão diagnóstico-primeiro**: 21ª aplicação consecutiva.
- **Marco arquitectural projectado pós-P199**: M5 universal a 1 pré-requisito paralelo do fecho.
