# Relatório P185A — diagnóstico-primeiro / L0-puro

**Data**: 2026-05-03
**Passo**: P185A — diagnóstico location-aware Layouter
**Resultado**: 6 cláusulas fechadas; mecanismo M3 (Locator dedicado
do Layouter com `current_location` field); plano 4 sub-passos
B–E sem condicionais; magnitude S-M agregada; **ADR-0068 PROPOSTO**
criada; zero código tocado.
**Postura**: zero código L1–L4 tocado; zero L0 prompts de produção
modificados; zero testes alterados.

---

## §1 Contexto

P185 ataca a pendência paralela P182E §5.2 (location-aware Layouter),
adiada várias vezes. É pré-condição para desbloquear C1 (heading
prefix, P183B) e C2 (equation counter, P183C) — os 2 consumers
restantes do DEBT M4-residual após P184 fechar C3.

A descoberta original em P182E §5.2: `Introspector::is_numbering_active`
usa `state_final_value` (snapshot final pós-walk) — insuficiente
para casos de re-update onde o Layouter precisa saber o valor "na
altura". Fallback `||` legacy mutável durante walk é o caminho
funcional para esses casos.

P184 ratificou empiricamente em §4.2: C3 fechado com Introspector
como caminho funcional (legacy é dead code). C1 e C2 esperam
location-aware para a mesma inversão.

P185A é diagnóstico arquitectural — escolhe entre 3 mecanismos
genuinamente diferentes (M1/M2/M3) sem "caminho óbvio" como em
diagnósticos anteriores que replicavam padrões.

---

## §2 Postura do auditor / executor

P185A é passo **L0-puro / diagnóstico-primeiro**, no mesmo registo
de P181A/P182A/P183A/P184A.

- Zero código L1–L4 tocado.
- Zero testes novos ou modificados.
- Zero L0 prompts de produção modificados.
- **ADR-0068 PROPOSTO criada** — diferente de P181A–P184A que não
  criaram ADR (decisão arquitectural substancial sem caminho
  óbvio).
- DEBT M4-residual cobertura inalterada (continua C1+C2 conforme
  P184F).
- Não modifica Layouter, walk, trait, consumers — P185B+.

---

## §3 Validação do estado actual (sub-passo .A)

Confirmações empíricas (detalhe em
`00_nucleo/diagnosticos/diagnostico-location-aware-layouter-passo-185a.md`
§1):

1. Trait `Introspector` tem `formatted_counter_at(key, location)`
   (P177) e `state_value(key, location)` (P171). Outros métodos
   `*_at` ausentes.
2. Layouter actual **não conhece** Location no ponto da consulta
   (zero hits de `Location` em ficheiros de produção `rules/layout/`).
3. **`Locator` é determinístico** — provado por test
   `duas_instancias_paralelas_produzem_sequencias_iguais`
   (`locator.rs:67-72`). Garantia documentada como design-intent.
4. Walk de introspect avança Locator **exactamente** quando
   `is_locatable(content) == true` (invariante explícito em
   `locatable.rs:11`).
5. `is_locatable` cobre Heading, Figure, Cite, Metadata, State,
   StateUpdate, Outline, Bibliography, SetHeadingNumbering.
   **`Equation` NÃO é locatable** — pré-requisito P186 para C2.
6. C1 site: `mod.rs:310` dentro de `Content::Heading` arm —
   Location pode ser conhecida.
7. C2 site: `equation.rs:97` dentro de `layout_equation` —
   precisa de P186 promover Equation a locatable antes.
8. Vanilla typst usa M2-like (Locator owned passing through layout
   functions) com `Locator::split` + `LocatorLink`. Cristalino tem
   `Locator` mais simples — não precisa de M2.

**Conclusão**: cristalino tem 4 ingredientes que tornam M3
trivialmente correcto (sincronização por construção via determinismo
+ `is_locatable` invariant).

---

## §4 Decisões cláusula 1–6

Detalhe O1–O5 em diagnóstico §3.

| Cláusula | Decisão | Justificação curta |
|----------|---------|--------------------|
| 1 — Mecanismo de propagação | **M3** — Locator dedicado do Layouter com `current_location` field, sincronizado por construção via determinismo + `is_locatable` gating | Capitaliza determinismo do Locator cristalino (provado em test); custo S vs M-L de M2; alinha ADR-0036 atomização + ADR-0067 attribute-grammar futuro; preserva P163 walk puro (mutação confinada ao Layouter, que já é stateful). |
| 2 — Trait methods location-aware | **Opção α**: adicionar ambos `is_numbering_active_at` + `flat_counter_at` em P185B | Trabalho preparatório barato; remove dependência cross-passo entre P185/P186/P187/P188. |
| 3 — Compatibilidade `Locator` | **Opção B** — Locator separado por walk com sincronização por construção | M3 implica B. A e C requerem comunicação cross-componente desnecessária. |
| 4 — Forma migração C1+C2 | **Opção A** — substitution-with-fallback (P184D pattern) | Padrão estabelecido; fallback defensivo durante janela compat M6. |
| 5 — Compat walk puro | **Confirmado compatível** | P163 cobre walk de introspect; M3 confina mutação ao Layouter (que já é stateful); cláusula trivial. |
| 6 — Critério de fecho P185 | **Opção 1** — diagnóstico (A) + infra (B+C) + tests (D) + relatório (E); C1+C2 ficam para P187+P188 | Mistura de escopo P185 com P187/P188 violaria atomização. |

---

## §5 Plano de sub-passos sem condicionais

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | `is_numbering_active_at` + `flat_counter_at` no trait + impl + 8-10 tests unit + L0 update | S | — |
| `.C` | Layouter `locator` + `current_location` fields; gating em `layout_content`; L0 `layout.md` update | M | `.B` |
| `.D` | Tests E2E mecanismo location-aware funciona (sem migrar C1/C2) | S | `.C` |
| `.E` | Relatório consolidado P185 (9 secções) + transição ADR-0068 PROPOSTO → ACEITE se validação passa | S | `.D` |

Sequência fixa B → C → D → E. Sem cláusulas condicionais.

---

## §6 Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

M3 alinha com sub-store pattern: `Locator` é "sub-store" do
Layouter; `current_location` é cursor field similar a
`figure_progress` (P184). Sem invenção estrutural nova.

### Q2 — Honestidade de magnitude

P185A diagnóstico é S. Implementação subsequente:
- P185B: S puro (replica P177 padrão para 2 métodos novos).
- P185C: **M genuíno** — primeira introdução de Locator no
  Layouter. Risco de edge case que pode escalar a M-L se
  sincronização-por-construção falhar em algum caso bordo.
  ADR-0068 critério de validação detecta divergência.
- P185D: S tests.
- P185E: S documental.

Total agregado P185B–E: ~150 LOC produção + ~120 LOC tests ≈ M.
Magnitude S-M registada honestamente.

### Q3 — Compatibilidade walk puro

P163 cobre walk de introspect; M3 não toca esse walk. Layouter já
tem state mutável (cursor_x, cursor_y, current_line); adicionar
`locator` + `current_location` é extensão natural. Compatibilidade
confirmada empiricamente.

### Q4 — Coerência com ADR-0067 (attribute-grammar)

ADR-0067 PROPOSTO sugere attribute-grammar para scoping. Location é
attribute herdado top-down via `current_location` field;
`prev_loc` save/restore implementa scoping léxico por construção.
M3 alinha sem replicar trabalho que ADR-0067 faria. Quando
ADR-0067 materializar, `current_location` pode generalizar para
attributes adicionais (cor, dir, lang).

### Q5 — Granularidade dos sub-passos P185B+

4 sub-passos (B–E). Cada um é S excepto C que é M. Sem
sub-passos condicionais. Cobertura completa: trait (B) +
integração (C) + validação (D) + fecho (E).

---

## §7 Pré-condição confirmada

- Tests workspace baseline P184F mantido — 1.769 verdes; zero
  violations linter. Sem alterações de código em P185A.
- M9: 11/11 (slot 11 livre).
- M5/M4 progresso: 6/12 read-sites migrados.
- DEBT M4-residual: cobre apenas C1 + C2 (P184F cenário B).
- Trait `Introspector`: 16 métodos.
- `Locator` provado determinístico.
- `is_locatable` cobertura conhecida (Equation excluído —
  pré-requisito P186 para C2).

---

## §8 Magnitude consolidada

S-M agregada (P185A S documental; P185B-E S+M+S+S).

---

## §9 Restrições aplicadas

Cumpridas:

- Zero código tocado em qualquer ficheiro fora de `00_nucleo/`.
- Zero testes modificados.
- Zero L0 prompts de produção modificados (apenas ADR + diagnóstico
  + relatório novos em `00_nucleo/`).
- Não criadas reservas de identificadores.
- Não modificado Layouter, walk, trait, Locator.
- Sem inflação de linguagem ("patamar", "limiar", "consolidação",
  "deriva", "subpadrão", "cumulativo", "cross-domínio", "paridade
  observable" como bandeira).
- Honestidade obrigatória: magnitude P185C registada como **M
  genuíno** com risco identificado de escalada M-L; não
  disfarçada.
- Sem cláusulas condicionais nos sub-passos B–E do plano.

---

## §10 ADR avaliação

**ADR-0068 PROPOSTO criada**:
`00_nucleo/adr/typst-adr-0068-location-aware-layouter.md`.

Diferente de P181A/P182A/P183A/P184A que não criaram ADR. P185A
criou porque:

- Decisão arquitectural substancial (escolha entre 3 mecanismos
  com perfis diferentes).
- Sem "caminho óbvio" que replicasse padrão P165/P171 etc.
- Mecanismo escolhido (M3) é cristalino-original — vanilla usa
  M2 mas forçado pela sua estrutura de Locator com link.

Status `PROPOSTO` até validação P185C+D. Critério ACEITE/REJEITADA
documentado na ADR §"Critério de validação".

---

## §11 DEBT avaliação

P185A não abre DEBT. P185 série é trabalho pendente planeado (P182E
§5.2 + P184F §8) — não DEBT-cleanup.

DEBT M4-residual mantém cobertura C1+C2 (registada em P184F
`m1-lacunas-captura.md` anexo). P187 e P188 fechá-los após P185
fornecer infra.

---

## §12 Estado actual

- **P185 série**: A ✅ (este passo) | B–E pendentes.
- **P184 série**: A–F ✅ (fechada).
- **P183 série**: A ✅ B ❌ C ❌ D ❌ E pendente F pendente.
- **Progresso M4**: 6 read-sites migrados; C1+C2 esperam P187/P188
  pós-P185.
- **M9**: 11/11 (inalterado).
- **44 passos executados** (P184F = 43 + P185A = 44).
- **Padrão diagnóstico-primeiro**: 10ª aplicação consecutiva
  (P131A/132A/140A/148/154A/181A/182A/183A/184A/**185A**).

---

## §13 Pendências cumulativas

Inalteradas:

- C1 + C2 (DEBT M4-residual): aguardam P185+ infra; migração em
  P187/P188.
- C5 TOC entries (`outline.rs:24`): bloqueado lacuna #3 (separado).
- Lacunas #1, #2, #3: abertas; nenhuma bloqueia milestones.
- Cleanup dead code legacy em M6 (incluindo
  `state.figure_numbers`/`local_figure_counters` confirmados em
  P184).
- Pré-requisito P186 para C2: promover `Content::Equation` a
  locatable (variant `ElementPayload::Equation`, arm em
  `from_tags`, `is_locatable` arm).

---

## §14 Próximo passo — P185B

Adicionar trait methods location-aware em falta:

1. Em `01_core/src/entities/introspector.rs`:
   - `fn is_numbering_active_at(&self, key: &str, location: Location) -> bool;`
   - `fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize>;`

2. Impl em `TagIntrospector`:
   - `is_numbering_active_at`: delega a `state.value_at(key, location)` + match `Value::Bool(true)` (replica P182B pattern com `value_at` em vez de `final_value`).
   - `flat_counter_at`: delega a `counters.value_at(key, location)?.last().copied()` (replica P184C pattern com Location em vez de idx).

3. L0 `entities/introspector.md` ganha entradas + histórico.

4. 8-10 tests unitários (4-5 cada): vazio→default; populate→Some/true;
   re-update→reflecte Location consultada; key isolation.

5. Critério de fecho P185B: `cargo test --workspace --lib` Δ +8-10;
   `crystalline-lint .` zero violations (após `--fix-hashes`).

Pré-condição P185B: este passo concluído (P185A); ADR-0068
PROPOSTO documenta mecanismo M3 escolhido em cláusula 1.

P185A é instrumento. Implementação concreta de location-aware
Layouter começa em P185B (trait methods) e P185C (Layouter integration).
