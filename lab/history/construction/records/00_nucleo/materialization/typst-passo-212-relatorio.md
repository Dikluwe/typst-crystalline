# Relatório do passo P212 — Encerramento marco M9c

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-212.md`.
**Tipo**: encerramento marco minimal documental.
**Magnitude planeada**: S (~30min-1h). **Magnitude real**: S (~45min).
**Marco**: **M9c (encerramento)**.

---

## §1 O que foi feito

Encerramento do marco arquitectónico M9c (M9-completion). 4
cláusulas executadas: C1 auditoria empírica das 7 condições
§Plano de validação ADR-0076 (5 cumpridas plenamente + 1 com
excepção documentada + 1 reinterpretada per `P207A.div-1`
reshape); C2 transição ADR-0076 PROPOSTO → ACEITE 2026-05-12;
C3 blueprint marca §3.0nonies adicionada (encerramento marco
qualitativamente distinto das marcas-por-série); C4
verificação preservada. ADR-0073 anotada com §"Fecho
retroactivo M9c — P212 2026-05-12" (cond #7 satisfeita).
Zero código tocado.

---

## §2 Auditoria 7 condições ADR-0076

| # | Condição | Estado | Evidência |
|---|----------|--------|-----------|
| 1 | P207B-E materializados | ✅ CUMPRIDA | P207B query_labelled + P207C LabelRegistry multi-label + label_count + P207D 4 page-aware + PageStore + P207E encerramento. Cargo build verde pós-cada sub-passo. |
| 2 | P208A-D + tests E2E real consumer + ≥5 tests | ⚠️ CUMPRIDA com excepção | P208A-D ✓ (here() + locate() + EvalContext); 8 tests novos (cumpre ≥5). **Excepção**: tests E2E "real consumer using here()" inexistente — Caminho 3 P210 deferiu counter.display/state.get; mock tests autorizados per P208B §5 risco 3. |
| 3 | P209A-D Selector::Label/And/Or + ≥3 tests/variant | ✅ CUMPRIDA | P209B Label+Location, P209C And+Or, P209D Regex (bónus), P209E encerramento. 28 tests novos cumulativos. |
| 4 | P210A-D page-aware infra + inject_page_data | ✅ CUMPRIDA (reinterpretada) | Reshape `P207A.div-1`: page-aware moveu para P207D (`PageStore` + `inject_pages`); P210 redirecionou Counter/State Q1=β. Funcionalmente equivalente. |
| 5 | Tests workspace 1873 → 1900-1950 | ✅ CUMPRIDA | 1873 → **1939** (+66) within range. |
| 6 | `crystalline-lint .` 0 violations | ✅ CUMPRIDA | preservadas toda a trajectória. |
| 7 | ADR-0073 retroactivo §"Fecho retroactivo M9c" | ✅ CUMPRIDA | Bloco §"Fecho retroactivo M9c — P212 2026-05-12" adicionado a ADR-0073 com tabela 6 métodos novos + sub-stores L1 novos + regra empírica P207B §5 confirmação. |

**Resultado**: 7/7 cumpridas. Fórmula adoptada: **"ACEITE
(completo retroactivo, com excepções documentadas)"** —
paralela ao pattern ADR-0073 cond 9 P206E.

---

## §3 ADR-0076 transição

**Pré-P212**: Status PROPOSTO; Data 2026-05-12 (PROPOSTO P207A).

**Pós-P212**: Status **ACEITE (completo retroactivo, P212
2026-05-12 — com excepções documentadas em cond #2 e cond #4
reinterpretadas per `P207A.div-1` reshape)**; Data 2026-05-12
(PROPOSTO P207A; ACEITE P212).

**Bloco "Transição ACEITE M9c — P212 2026-05-12"** adicionado
à ADR-0076 antes de §Contexto. Conteúdo:
- Auditoria 7 condições com tabela de evidência.
- Justificação fórmula "completo retroactivo com excepções".
- Caminhos rejeitados (A "ACEITE puro" + B "manter PROPOSTO").
- 11 deferreds documentados com critério de reabertura.
- 8 patterns emergentes formalizados M9c.
- Cross-references P212.

**ADR-0073 anotada** com §"Fecho retroactivo M9c — P212
2026-05-12":
- Tabela 6 métodos trait novos M9c (P207B/C/D).
- Sub-stores L1 novos: `PageStore` + `Regex` wrapper.
- Regra empírica P207B §5 confirmação.
- Status atualizado: "ACEITE (completo retroactivo + paridade
  trait estendida M9c, P212 2026-05-12)".

---

## §4 Blueprint marca §3.0nonies

**Opção γ fixada** (Marca §3.0nonies com nota
"encerramento marco M9c") — paralela em forma ao pattern
marca-por-fecho § 3.0 a § 3.0octies mas qualitativamente
distinta:

- **Marcas §3.0quater a §3.0octies (5 marcas)**: cobrem
  encerramentos de série (P207E + P208D + P209E + P210C +
  P211A).
- **Marca §3.0nonies (esta)**: cobre encerramento de marco
  (M9c) — escopo cumulativo qualitativamente distinto.

Conteúdo §3.0nonies:
- M9c ACEITE 2026-05-12 (ADR-0076).
- Mudanças factuais consolidadas: trait 20→26, Selector 1→6,
  sub-stores +2, stdlib funcs +3, deps allowlist 11→12.
- Custo agregado real ~20h vs orçamento ~30-50h (redução
  >40%).
- 8 patterns emergentes formalizados.
- 11 deferreds documentados com critério de reabertura.
- ADRs anotadas (0076 + 0073 + 0077).
- Estado pós-M9c (próximo marco fora de P212).

---

## §5 Decisões substantivas

- **Fórmula "ACEITE retroactivo com excepções"** (vs ACEITE
  puro vs manter PROPOSTO): caminho honesto face às 2
  excepções documentadas (cond #2 mock tests; cond #4
  reinterpretação `P207A.div-1` reshape). Pattern P206E
  para ADR-0073 cond 9 replicado.
- **§3.0nonies em vez de §3.0octies**: § 3.0octies já estava
  ocupada em P211A. P212 escolhe próxima letra cumulativa
  (8ª letra latina: nonies). Pattern marca-por-fecho
  preservado.
- **ADR-0073 §"Fecho retroactivo M9c"**: paralelo a §"Fecho
  retroactivo cond 9 — P206E" (preserva pattern;
  documenta progresso retroactivo sem revogar cláusulas
  prévias).
- **11 deferreds documentados explicitamente em ADR-0076**:
  pattern anti-inflação não deixa "buracos" — cada deferred
  tem gatilho claro de reabertura. Sem critério, deferreds
  ficariam permanentes (anti-inflação honest).
- **Zero código tocado em P212**: encerramento marco
  100% documental, paralelo a encerramentos série Caminho
  1 (P207E/P208D/P209E/P210C/P211A).

---

## §6 Métricas cumulativas M9c

| Métrica | Pré-M9c (P206E) | Pós-M9c (P212) | Δ marco |
|---------|-----------------|----------------|---------|
| Trait `Introspector` métodos | 20 | 26 | +6 |
| `Selector` enum variants | 1 | 6 | +5 |
| Sub-stores L1 (entities/) | 23 | 25 | +2 (`PageStore`, `Regex`) |
| Sub-store enriquecido | — | 1 (`LabelRegistry` multi-label) | — |
| Stdlib funcs novas | — | 3 (`here`, `locate`, `counter_step`) | +3 |
| `EvalContext` fields | 4 | 5 | +1 (`current_location`) |
| `EvalContext` methods | 2 | 3 | +1 (`with_current_location`) |
| Allowlist L1 deps externas | 11 | 12 | +1 (`regex`) |
| Tests workspace | 1873 | 1939 | +66 |
| `CALL_COUNTERS` (P204G slots) | 20 | 26 | +6 |
| ADRs PROPOSTO ou ACEITE em M9c | — | 2 (0076, 0077) | +2 |
| L0 prompts novos | — | 3 (`page_store.md`, `regex.md`, ADR-0077) | +3 |
| L0 prompts modificados | — | 6 (4 entities + measurements + selector) | +6 |
| L1 ficheiros novos | — | 2 (`page_store.rs`, `regex.rs`) | +2 |
| L1 ficheiros modificados | — | 6+ ao longo M9c | — |
| Blueprint marcas cirúrgicas | 3 (§3.0/bis/ter) | 9 (§3.0/bis/ter/quater/quinquies/sexies/septies/octies/nonies) | +6 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| `cargo build --workspace` | verde | verde | preservado |

**Custo agregado real M9c**: ~20h cumulativo.

| Série | Sub-passos | Magnitude |
|-------|------------|-----------|
| P207A-E | 5 | M-L (~10h) |
| P208A-D | 4 | M (~3h) |
| P209A-E | 5 | M (~4h) |
| P210A-C | 3 | S-M (~1.5h) |
| P211A | 1 | S (~30min) |
| P212 | 1 | S (~45min) |
| **Total** | **19** | **~20h** |

vs orçamento original `P207A.div-1` aprovado: **~30-50h
escopo amplo**. **Redução empírica >40%** via diagnóstico-primeiro
+ Caminho 1/3 honest + anti-inflação 9 aplicações
consecutivas.

---

## §7 Encerramento M9c — sumário literal 5 séries

| Série | Bloco | Sub-passos | Patterns aplicados |
|-------|-------|------------|---------------------|
| **P207** | I/II/III/VIII — Trait extensions + sub-store refactor + page-aware infra | A diagnóstico + B + C + D + E (5) | Diagnóstico-primeiro reduzido; regra P207B §5 (3 aplicações em B/C/D); Caminho 1 anti-inflação (P207E + P205D deferred) |
| **P208** | IV — `here()` + `locate()` | A diagnóstico + B + C + D (4) | Diagnóstico-primeiro reduzido; sub-mecanismo i minimal (`current_location` field); convenção L0 inline (P208B/C); Caminho 1 anti-inflação (P208D) |
| **P209** | VI — Selector minimal | A diagnóstico + B + C + D + E (5) | Diagnóstico-primeiro reduzido; convenção L0 inline (P209D C6); ADR-0077 PROPOSTO→ACEITE; Caminho 1 anti-inflação (P209C-vazios + P209D C6 + P209E C1.2) |
| **P210** | V — Counter/State extras Q1=β | A diagnóstico + B counter_step + C encerramento (3) | Diagnóstico-primeiro reduzido; **Caminho 3 honest subset** (pattern novo); convenção L0 inline (P210B); deferreds counter.display/state.get com critério |
| **P211** | VII — Outline configurável | A diagnóstico + encerramento (1) | Diagnóstico-primeiro reduzido; **1-sub-passo único** (pattern novo); Caminho 1 puro 9ª aplicação anti-inflação |

**Padrões formalizados em M9c**:

1. **Diagnóstico-primeiro reduzido** (5 aplicações).
2. **Caminho 1 anti-inflação** (9 aplicações cumulativas:
   P205D, P207E, P208B C1, P208D, P209C-vazios, P209D C6,
   P209E C1.2, P210 Caminho 3, P211A).
3. **Caminho 3 honest subset** (1 aplicação P210; pattern
   novo).
4. **1-sub-passo único** (1 aplicação P211A; pattern novo).
5. **Convenção L0 inline-documentada stdlib funcs P169+**
   (4 aplicações: P208B, P208C, P209D C6, P210B).
6. **Regra empírica P207B §5** (3 aplicações em P207B/C/D;
   não acionada em P208/P209/P210/P211).
7. **Stdlib funcs + Selector variants ≠ trait extensions**
   (formalizado em P209+P210).
8. **Marca-por-fecho cirúrgica blueprint** (8 marcas
   cumulativas pré-P212 + §3.0nonies P212 = 9 total).

**11 deferreds documentados com critério de reabertura**:

1. `SealedLabelPages` (P205D).
2. Page-meta capture (P207E).
3. Walk advance automático (P208B).
4. `Content::Context` block (P208D).
5. `native_regex` stdlib (P209D C6).
6. `counter.display` here-aware (P210A C3).
7. `state.get` here-aware (P210A C3).
8. Outline configurável params (P211A C3).
9. `Selector::Where` (P207A Q2=γ).
10. `Selector::Before/After/Within` (P207A roadmap).
11. `query_count_before` (P207A Q4=β).

Cada deferred com gatilho explícito; reabertura futura tem
entrada óbvia.

---

## §8 Próximo passo (fora de P212)

**M9c fechado**; trajectória aberta para próximo marco
arquitectónico.

**Recomendação para próxima sessão** (fora escopo P212):
- Auditar candidatos próximos marcos (M10? F4? outro?).
- Considerar **reabertura selectiva de deferreds** se
  consumer real emergir empíricamente (e.g. test fixture
  com `outline(depth: 2)` ou `#context { here() }` block).
- Manter pattern emergente: diagnóstico-primeiro → Caminho
  honest (1/2/3) → encerramento série → eventual encerramento
  marco.
- Considerar consolidação de patterns emergentes em ADR
  metodológica (e.g., ADR-0078 "Pattern Caminho 1/3 honest
  + 1-sub-passo único") se humano julgar útil para sessões
  futuras.

**Estado actual**:
- Marco M9c: ✅ ACEITE 2026-05-12 (ADR-0076).
- ADRs M9c: 2 (ADR-0076 + ADR-0077) ACEITES.
- ADR-0073 anotada com fecho retroactivo M9c.
- 5 séries M9c materializadas (P207-P211).
- 11 deferreds documentados com critério.
- Tests workspace: **1939 verdes**; `crystalline-lint`: **0
  violations**.

**M9c encerra como referência arquitectural estável** —
fundamento para marcos futuros + base de auditoria + planeamento.
