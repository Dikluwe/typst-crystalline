# Relatório P188a — Diagnóstico C2 equation counter migration

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: P187 série fechada; tests workspace 1.805
verdes; zero violations.

---

## §1 Escopo

P188A é o passo de diagnóstico-primeiro que precede a
migração C2 (equation counter em `equation.rs:97`).
Replica registo de P181A/P182A/P183A/P184A/P185A/P186A/P187A.

P188A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-c2-equation-counter-passo-188a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-188a-relatorio.md` (este ficheiro, 14 secções).

Sem ADR nova. Sem DEBT novo. Sem código tocado.

---

## §2 Inputs verificados empiricamente (9 grep/read)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Site C2 actual | `equation.rs:97` `let n = self.counter.get_flat("equation")` (linha confirmada empiricamente) |
| 2 | Receptor (Cenário 1/2/3) | **Cenário 1**: `self` é `Layouter<M, S>` directamente (`equation.rs:19` impl block) |
| 3 | `flat_counter_at` API | `(&self, key: &str, location: Location) -> Option<usize>` (P185B) |
| 4 | `current_location` no site | OK; Equation locatable após P186D; gating precede arm |
| 5 | `get_flat` legacy | retorna **`usize`** (não `Option`); default 0 — exige `unwrap_or_else` |
| 6 | Uso downstream | `format!("({})", n)` — variável final `n: usize` mantém-se |
| 7 | Tests existentes | `layout_equation_bloco_numerada` (tests.rs:966-979) — preservar |
| 8 | `Content::SetEquationNumbering` | **AUSENTE** (zero hits em produção; só comentários) |
| 9 | DEBT M4-residual cenário | B (continuação de P187 — sem DEBT formal; apenas nota) |

Crítico descoberto: **diferença de tipo face a P187B**.
Legacy `get_flat -> usize` (não `Option`) força `unwrap_or_else`
em vez de `or_else`. Pequena adaptação na sintaxe; semântica
substitution-with-fallback preservada.

---

## §3 Decisões cláusulas 1–7 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma da expressão | **Opção A** inline com `and_then(...).unwrap_or_else(legacy)` |
| 2 | `None` do Introspector | **Opção A** `unwrap_or_else` (caminho **funcional permanente** em produção, não defensivo como C1) |
| 3 | `None` do `current_location` | **Opção B** `and_then` defensivo |
| 4 | Receptor | **Cenário 1** — `self` é Layouter directamente |
| 5 | Forma de retorno | **`usize`** preservado (sem conversão de tipo) |
| 6 | Documentação inline | **Opção A** comentário curto + cross-reference P186A §11.2 |
| 7 | Critério fecho | **Opção 3** — consumer + tests E2E + DEBT M4-residual vazio em prática |

Forma final:

```rust
let n = self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"));
```

---

## §4 Plano de sub-passos B (sem condicionais)

**Sub-passo único agregado** (idêntico a P187B):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar `equation.rs:97` + L0 + tests E2E + comentário inline + actualização DEBT + relatório consolidado P188 | S |

---

## §5 Magnitude agregada

**P188 série = S puro** (1×S agregado em sub-passo único).

Idêntico a P187 em magnitude. Diferença está em semântica
(estado dormente em produção), não em volume.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- `flat_counter_at(key, location)` no trait (P185B).
- `current_location: Option<Location>` no Layouter (P185C).
- Equation locatable + variants (P186 série completa).
- `from_tags` arm Equation com gate location-aware (P186E).
- ADR-0068 ACEITE (P185E).

### §6.2 — Dependentes

- **DEBT M4-residual fica vazio em prática** após P188B.
- **M4-residual fechado funcionalmente**.
- **Próximo: M5 (P189)** — segue novos read-sites ou M9
  slot 11.

### §6.3 — Trabalho identificado fora de escopo

- **`Content::SetEquationNumbering` materialização** — passo
  dedicado quando equation set rule for prioridade. Activa
  Introspector path em P188; permite janela compat M6
  abrir para Equation.

---

## §7 ADR avaliação

**Sem ADR criada.** Substitution-with-fallback é padrão
P187B (replicação de P184D). `unwrap_or_else` vs `or_else`
é detalhe de tipo. Estado dormente é honestidade documental.

---

## §8 DEBT avaliação

### Cenário B (continuação)

P187B reduziu DEBT M4-residual cobertura de C1+C2 → C2
(sem DEBT formal aberto; apenas notas preventivas em
relatórios consolidados).

P188B fecha C2 estruturalmente. DEBT M4-residual fica
**vazio em prática**:
- C1: Introspector funcional (P187B).
- C2: Introspector dormente; fallback legacy é caminho
  funcional permanente (P188B).

P183F formal pode ser **dispensado** (DEBT vazio antes de
abrir formalmente). Decisão fica para passo subsequente.

---

## §9 Restrições honradas

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica trait `Introspector`** (P185B fechou).
- **Não modifica Layouter struct** (P185C fechou).
- **Não modifica P186 (Equation locatable)** (P186F fechou).
- **Não materializa `Content::SetEquationNumbering`**.
- **Não migra consumer C2** — P188B.
- **Sem inflação retórica**.
- **Honestidade obrigatória sobre estado dormente** —
  documentado em §11.5 e §11.6 abaixo.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.805** inalterado vs
  P187B.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR nova.
- ✅ Sem DEBT novo.

---

## §11 Achados não-triviais

### §11.1 — Diferença de tipo legacy: `get_flat -> usize` (não Option)

`get_flat("equation")` retorna `usize` directamente, não
`Option<usize>`. Diferente de `format_hierarchical("heading")`
(P187B) que retorna `Option<String>`.

Consequência sintáctica: substitution-with-fallback em C2
usa `unwrap_or_else` em vez de `or_else`:

```rust
// P187B (heading): or_else
.or_else(|| self.counter.format_hierarchical("heading"))

// P188B (equation): unwrap_or_else
.unwrap_or_else(|| self.counter.get_flat("equation"))
```

Diferença semântica: `or_else` continua a propagar `Option`;
`unwrap_or_else` resolve para tipo final. C2 produz `usize`
final; C1 produz `Option<String>` final. Diferença
mecânica, não conceptual.

### §11.2 — Receptor é `self` directo (Cenário 1)

`equation.rs:19` `impl<M: FontMetrics, S: ImageSizer>
super::Layouter<M, S>` — `self` é `Layouter<M, S>`. Acesso
inline a `self.introspector` e `self.current_location` sem
reorganização.

Diferente de Cenário 2 (helper struct) que exigiria
recombinação ou Cenário 3 (função livre) que exigiria
parâmetro adicional.

### §11.3 — `current_location` populated no site C2

Equation é locatable após P186D. Layouter:
- `layout_content(Content::Equation { .. })` → gating
  `advance_locator_if_locatable` (P185C `mod.rs:236-240`)
  precede match arm.
- Match arm chama `self.layout_equation(body, *block)`.
- `layout_equation` é onde C2 está (linha 97).

Logo no site C2, `self.current_location` é
`Some(loc_da_equation)` — confirmado por arquitectura.

### §11.4 — Estado dormente confirmado empiricamente

`grep -rn "SetEquationNumbering" 01_core/src/` retorna **zero
hits** em produção. Apenas:
- `equation.rs:25-29` — comentário inline já documenta.
- `element_payload.rs:113` — referência em comentário L0.
- `locatable.rs:55` — referência em comentário L0.

Nenhum produtor de `Content::StateUpdate { key:
"numbering_active:equation", ... }` em produção. Walk
real nunca emite tag que populate state. Gate em P186E
nunca dispara em runtime real → counter introspector
sempre vazio.

P186A §11.2 documentação confirmada empiricamente.

### §11.5 — P188 é primeira migração com Introspector dormente

Comparação cumulativa:

| Caso | Introspector em produção | Caminho funcional |
|------|---------------------------|-------------------|
| C3 Figure (P184D) | activo | Introspector |
| C1 Heading prefix (P187B) | activo | Introspector |
| **C2 Equation counter (P188B)** | **dormente** | **fallback legacy permanente** |

P188 é o **primeiro caso da série M4-residual** onde
migração estrutural não traduz em mudança funcional. O
Introspector path está presente no código mas nunca dispara
em runtime real até equation set rule materializar.

### §11.6 — Documentação obrigatória em 4 pontos

Honestidade sobre estado dormente exige documentação em:
1. Comentário inline em `equation.rs:97` (P188B `.B`).
2. Secção em L0 `rules/layout.md` (P188B `.C`).
3. Tests E2E `gate_dormente_caso_producao` que valida
   empiricamente fallback (P188B `.D`).
4. Relatório consolidado P188 §"Estado dormente" (P188B
   `.G`).

Sem estes 4 pontos, leitores futuros podem assumir
incorrectamente que P188 fecha completamente C2 (não fecha
funcionalmente — apenas estruturalmente).

---

## §12 Snapshot pós-P188A

- **Tests workspace**: 1.805 (inalterado).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **Layouter**: `current_location` + `locator` (inalterado).
- **ADR-0068**: ACEITE.
- **DEBT M4-residual**: cobre C2 (reduzirá para vazio em
  prática após P188B).
- **M5/M4 progresso**: 7/12 read-sites migrados (subirá
  para 8/12 após P188B).
- **57 passos executados** (P187B = 56 + P188A = 57).
- **Padrão diagnóstico-primeiro**: 13ª aplicação consecutiva
  (P188A na lista).

---

## §13 Próximo passo

**P188B** — migração C2 + tests E2E + nota DEBT
M4-residual:

- Editar `01_core/src/rules/layout/equation.rs:97`:
  - Substituir `self.counter.get_flat("equation")` pela
    expressão substitution-with-fallback location-aware.
  - Adicionar comentário inline cross-referenciando
    P186A §11.2 e P186E gate dormente.
- Editar L0 `00_nucleo/prompts/rules/layout.md`:
  - Secção "C2 equation counter migrado (P188B)" com nota
    explícita sobre estado dormente.
- Tests E2E em `mod p188b_c2_equation_counter`:
  - `c2_equation_counter_via_introspector_path_quando_state_injectado`.
  - `c2_equation_counter_via_fallback_legacy_caso_producao`
    (caso central).
  - `c2_equation_counter_paridade_legacy_vs_introspector`.
- Actualizar nota DEBT M4-residual no relatório consolidado
  P188.

Magnitude: S puro. Sem cláusulas condicionais.

---

## §14 Conclusão

P188A fechou 7 cláusulas com decisão literal e plano em
sub-passo único. Magnitude S agregada confirmada para P188.
ADR avaliada e dispensada (replicação P187B). DEBT
avaliado: cenário B continuação (vazio em prática após
P188B).

Achado central: **C2 é primeiro caso da série M4-residual
onde Introspector path migra mas não é caminho funcional
em produção**. Estado dormente é design intencional
herdado de P186A §11.2 (`Content::SetEquationNumbering`
ausente). P188 honra esta condição com documentação
explícita em 4 pontos.

P188 é o **último passo funcional de M4-residual**. Após
P188B:
- C1 fechado (Introspector funcional, P187B).
- C2 fechado estruturalmente (Introspector dormente;
  fallback legacy permanente, P188B).
- DEBT M4-residual vazio em prática.
- M5/M4 progresso 8/12.
- M4-residual completo. Segue M5 (P189).

Padrão diagnóstico-primeiro mantido — 13/13 acertaram a
magnitude planeada ±1 nível.
