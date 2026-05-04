# Passo P188B — Migrar C2 equation counter + tests E2E

Único passo de implementação P188 (após P188A diagnóstico).
Magnitude **S** agregada — passo único combinando migração,
tests E2E, edits L0, documentação inline obrigatória sobre
estado dormente, e actualização de nota DEBT M4-residual.

**Último passo funcional de M4-residual.**

Migra consumer C2 em
`01_core/src/rules/layout/equation.rs:97` de
`self.counter.get_flat("equation")` para a forma
substitution-with-fallback location-aware fixada em P188A
§3:

```rust
let n = self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"));
```

Após P188B:
- C2 consulta Introspector primeiro com `current_location`
  (P185C); fallback legacy quando Introspector retorna
  `None` ou quando `current_location` é `None`.
- C2 é o **primeiro consumer da série M4-residual onde
  migração estrutural não traduz em mudança funcional** —
  Introspector path está presente no código mas nunca
  dispara em runtime real até equation set rule
  materializar (per P186A §11.2).
- Honestidade documental obrigatória em 4 pontos
  (P188A §11.6).
- DEBT M4-residual reduz para vazio em prática.
- M4-residual fechado funcionalmente.

**Pré-condição**: P188A concluído. Tests workspace 1.805
verdes; zero violations. Forma da expressão fixada
(combinação Opção B + Opção A com `unwrap_or_else` por
diferença de tipo legacy).

**Restrições**:
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar Layouter struct (P185C fechou).
- **Não** modificar `flat_counter_at` ou
  `current_location` API.
- **Não** modificar walk arm legacy.
- **Não** modificar P186 (Equation locatable) — P186F
  fechou.
- **Não** materializar `Content::SetEquationNumbering` —
  passo dedicado fora da série.
- **Não** abrir DEBT M4-residual formal (cenário B per
  P188A §8 — apenas notas preventivas).
- API pública preservada.
- Output observable em produção **inalterado** —
  Introspector path dormente; fallback legacy fornece
  valores idênticos aos actuais. Diferente de P187B onde
  caminho funcional muda; em P188 caminho funcional
  permanece legacy.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C2 em `equation.rs:97`:
   - Per P188A §2.1, site real é 97. Verificar
     empiricamente — se localização mudou entre P188A e
     P188B, ajustar.
   - Localizar leitura `self.counter.get_flat("equation")`.
   - Identificar contexto exacto (método em
     `impl<M, S> Layouter<M, S>`).

2. Confirmar acesso a `self.introspector` e
   `self.current_location`:
   - Cenário 1 confirmado em P188A §2.2. Re-verificar
     empiricamente.

3. Confirmar L0 `rules/layout.md`:
   - Localizar entradas existentes sobre equation-arm
     (P186 estendeu).
   - Identificar onde adicionar nota sobre migração C2
     com **honestidade explícita sobre estado dormente**.

4. Confirmar tests existentes que cobrem equation
   counter:
   - `layout_equation_bloco_numerada` (per P188A §2.7).
   - Outros que possam regredir.
   - **Esperado**: tests existentes não regridem porque
     output observable é idêntico (Introspector dormente
     em produção; fallback fornece valores legacy).

5. Confirmar nota DEBT M4-residual em relatórios:
   - Per P188A §8: notas em P184F/P185-consolidado/
     P186-consolidado/P187-consolidado.
   - Localizar para actualização — P188 reduz cobertura
     para "vazio em prática".

6. Confirmar 4 pontos de documentação obrigatória (per
   P188A §11.6):
   - `equation.rs:97` (comentário inline).
   - L0 `rules/layout.md` (secção).
   - Test `gate_dormente_caso_producao` (P188B `.D`).
   - Relatório consolidado §"Estado dormente" (P188B
     `.G`).

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Site C2 confirmado.
- Tests existentes inventariados.
- 4 pontos de documentação localizados.

### .B Migrar consumer C2

1. Em `01_core/src/rules/layout/equation.rs:97` (ou
   linha real per `.A.1`):
   - Substituir leitura legacy pela expressão fixada
     P188A §3:
     ```
     let n = self.current_location
         .and_then(|loc| self.introspector
             .flat_counter_at("equation", loc))
         .unwrap_or_else(|| self.counter
             .get_flat("equation"));
     ```
   - Forma exacta fica para Claude Code conforme
     convenção do projecto.

2. **Adicionar comentário inline obrigatório** (per
   P188A §11.6 ponto 1):
   - Texto sugerido (curto, factual):
     ```
     // Path Introspector dormente em produção até
     // `Content::SetEquationNumbering` materializar
     // (vide P186A §11.2). Fallback legacy é caminho
     // funcional permanente.
     ```
   - Localização: imediatamente antes da expressão.

3. Confirmar `@prompt-hash` actualiza após edit do L0
   (vai ser actualizado em `.C`).

**Critério de saída**:
- `cargo check --workspace` passa.
- Tests existentes não regridem (paridade observable —
  Introspector dormente; fallback fornece valor legacy
  idêntico).
- Linter passa (após `--fix-hashes` em `.D`).

### .C Actualizar L0 `rules/layout.md`

1. Adicionar entrada para C2 migration (per P188A §11.6
   ponto 2):
   - C2 equation counter consultado via Introspector
     `flat_counter_at("equation", current_location)`
     primeiro.
   - Fallback legacy `get_flat("equation")` activo
     **permanentemente em produção** (não apenas durante
     janela compat M6).
   - **Estado dormente explicitamente documentado**:
     `Content::SetEquationNumbering` ausente em
     cristalino → state `numbering_active:equation`
     nunca populado → gate em P186E nunca dispara →
     counter introspector sempre vazio.
   - Trabalho identificado fora série: materialização
     de `Content::SetEquationNumbering` activa
     Introspector path; após esse passo, janela compat
     M6 pode abrir para Equation.
   - Cross-references: P186A §11.2, P186E gate, P188A.

2. Hash em branco aguarda recálculo manual (`.E`).

**Critério de saída**:
- L0 contém entrada para C2 migration com estado
  dormente honestamente documentado.
- Coerente com entradas P186 + P187 existentes.

### .D Tests E2E em submódulo `p188b_c2_equation_counter`

Submódulo novo em `01_core/src/rules/layout/tests.rs`
(ou ficheiro de tests do `equation.rs` se separado).
Irmão de `p184e_figure_per_kind`, `p185d_locator_sync`,
`p186f_equation_locatable`, `p187b_c1_heading_prefix`.

3 tests obrigatórios.

#### Test 1 — `c2_equation_counter_via_introspector_path_quando_state_injectado`

1. Construir documento com 3 equations block + state
   `numbering_active:equation` injectado:
   ```
   // pre-popular state
   let mut intr = TagIntrospector::empty();
   intr.state.apply_at(
       "numbering_active:equation",
       StateUpdate::Set(Value::Bool(true)),
       loc(0));
   // equations
   let content = Content::Sequence(vec![
       equation_block("a + b"),
       equation_block("c + d"),
       equation_block("e + f"),
   ]);
   // walk + from_tags com state injectado
   ```

2. Pipeline:
   - Walk + from_tags → `TagIntrospector` populado
     (gate dispara porque state activo).
   - Layouter `layout_with_introspector(content, &intr)`.

3. Asserções:
   - `plain_text` contém numerações `(1)`, `(2)`, `(3)`
     (ou formato exacto conforme convenção empírica).
   - Validação intermédia: `intr.flat_counter_at(
     "equation", loc_eq1)` retorna `Some(1)`.

#### Test 2 — `c2_equation_counter_via_fallback_legacy_caso_producao`

**Caso central da produção** (per P188A §11.6 ponto 3).

1. Mesmo documento (3 equations block).
2. **Sem injectar state** — caso real de produção.
3. Pipeline:
   - Walk + from_tags → state nunca populado → gate em
     P186E nunca dispara → counter introspector vazio.
   - `layout_with_introspector(content, &intr_vazio)`.
4. Asserções:
   - `flat_counter_at("equation", loc_*)` retorna `None`
     para todas as locations.
   - `unwrap_or_else` cai em `get_flat` legacy.
   - `plain_text` contém numerações correctas via
     fallback legacy.
   - Validação que **cenário de produção real** é
     coberto pelo fallback.

#### Test 3 — `c2_equation_counter_paridade_legacy_vs_introspector`

1. Mesmo documento.
2. Path A: `layout()` legacy puro (sem Introspector).
3. Path B: `layout_with_introspector(content,
   intr_com_state_injectado)` (Introspector funcional).
4. Asserção: `plain_text(A) == plain_text(B)`.
5. Confirma paridade observable entre os dois caminhos
   funcionais — em produção real, apenas Path A é
   activo; Path B só é activo em testes.

**Critério de saída**:
- 3 tests novos passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P188A
   baseline (1.805): +3.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p188b_c2_equation_counter::*` passam
   isoladamente
   (`cargo test --workspace --lib p188b`).
5. Tests existentes (incluindo
   `layout_equation_bloco_numerada`) **não regridem** —
   paridade observable preservada por construção.
6. Consumer C2 (`equation.rs:97`) consulta Introspector
   primeiro com `current_location`; fallback legacy.
7. **Comentário inline obrigatório presente** em
   `equation.rs:97`.
8. Walk arm legacy **NÃO modificado**.
9. Trait `Introspector` **NÃO modificado**.
10. Layouter struct **NÃO modificado**.
11. P186 (Equation locatable) **NÃO afectado**.
12. Snapshot tests ADR-0033 verdes.
13. Linter passa final.

### .F Actualizar notas DEBT M4-residual

P188 reduz cobertura de DEBT M4-residual para vazio em
prática (cenário B per P188A §8 — apenas notas
preventivas).

1. **Não editar** os relatórios anteriores
   (preservação histórica per P187B `.F`).

2. Adicionar nota nova no relatório consolidado P188
   (`.G`) que actualiza estado.

3. Texto sugerido para P188 consolidado:
   > Após P188, DEBT M4-residual cobre **vazio em
   > prática**. C1 fechado em P187B (Introspector
   > funcional). C2 fechado em P188B (Introspector
   > dormente; fallback legacy é caminho funcional
   > permanente).
   >
   > P183F formal pode ser **dispensado** (DEBT vazio
   > antes de abrir formalmente). Decisão fica para
   > passo subsequente.
   >
   > Quando `Content::SetEquationNumbering` materializar
   > (passo dedicado fora da série actual), Introspector
   > path em P188 activa-se em produção; janela compat
   > M6 pode abrir para Equation.

**Critério de saída**:
- Nota actualizada no relatório consolidado P188 (`.G`).
- Sem edits em relatórios anteriores.

### .G Escrever relatório consolidado P188

1. Criar
   `00_nucleo/materialization/typst-passo-188-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P182F / P184F / P185 /
   P186 / P187):

   - §1 Resumo executivo + C2 fechado estruturalmente +
     M4-residual fechado funcionalmente.
   - §2 Sub-passos materializados (tabela métricas A–G
     dentro de P188B único).
   - §3 Decisões arquitecturais (7 cláusulas P188A
     fechadas).
   - §4 Achados não-triviais durante execução:
     - P188A §11.1 — diferença de tipo legacy
       (`get_flat -> usize`).
     - P188A §11.4 — `SetEquationNumbering` ausente
       confirmado empiricamente.
     - P188A §11.5 — primeira migração com Introspector
       dormente.
     - P188A §11.6 — documentação obrigatória em 4
       pontos.
   - §5 **Estado dormente** (secção dedicada — per
     P188A §11.6 ponto 4):
     - Diferença observable: Introspector path
       presente; fallback legacy é caminho funcional
       permanente.
     - Razão: `Content::SetEquationNumbering` ausente.
     - Trabalho identificado fora série.
     - Quando passo de materialização correr,
       Introspector activa automaticamente; janela
       compat M6 pode abrir.
   - §6 Estado final M9 (inalterado 11/11) e M5/M4
     (8/12 read-sites; +1 vs P187).
   - §7 Estado final lacunas (inalterado).
   - §8 Pendências cumulativas + DEBT M4-residual vazio
     em prática + nota actualizada (vide `.F`).
   - §9 Próximos passos sugeridos:
     - **M4-residual fechado funcionalmente**.
     - P189 (M5 walk puro) — pode prosseguir.
     - Trabalho identificado: `Content::SetEquationNumbering`,
       outros M5/M9 slot 11.

2. Sem L0 novo; sem alteração de tests; sem ADR; sem
   DEBT formal.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes (com §5 dedicada a estado
  dormente).
- Nota DEBT M4-residual actualizada.

### .H Encerramento

P188B é o passo único de implementação. Após `.G`
concluído, série P188 está fechada e **M4-residual está
fechado funcionalmente**.

Estado projectado pós-P188B:

- **P188 série**: A ✅ B ✅. Fechada.
- **C2 fechado estruturalmente** com Introspector
  dormente.
- **M4-residual fechado funcionalmente** — todos os
  consumers C1+C3 fechados; C2 estruturalmente migrado.
- **DEBT M4-residual vazio em prática**.
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso**: 8/12 read-sites migrados (era
  7/12).
- **57 passos executados** (per P188A §12: P187B = 56 +
  P188A = 57 + P188B = 58). Recontagem: **58 passos**.
- **Padrão diagnóstico-primeiro**: 13ª aplicação
  consecutiva (P188A na lista — 13/13 acertaram a
  magnitude planeada ±1 nível).
- **Próximo: M5 (P189)**.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Consumer C2 migrado (`equation.rs:97`).
3. **Comentário inline obrigatório presente** em
   `equation.rs:97`.
4. L0 `rules/layout.md` actualizado com **estado
   dormente explicitamente documentado**.
5. 3 tests E2E novos passam (incluindo
   `c2_equation_counter_via_fallback_legacy_caso_producao`
   que valida cenário de produção real).
6. Tests existentes não regridem (paridade observable).
7. Verificações `.E` passam (13/13).
8. Nota DEBT M4-residual actualizada (vazio em prática).
9. Relatório consolidado P188 (9 secções com §5
   dedicada a estado dormente) escrito.
10. Output observable em produção **inalterado** —
    Introspector dormente; fallback funcional.
11. Honestidade documental confirmada em 4 pontos.

---

## O que pode sair errado

- **Site C2 mudou** entre P188A e P188B (improvável):
  cláusula gate trivial — ajustar referência em `.B`.
- **`current_location` é `None` no site** (improvável
  per P188A §11.3 — Equation locatable + gating precede
  arm): cláusula gate trivial — Opção B `and_then`
  cobre via fallback.
- **Test 2 (caso central produção) falha**: indica que
  Introspector está a popular counter em produção
  inesperadamente. Investigar — pode haver caminho de
  populate não-inventariado. Cláusula gate substancial.
  **Risco moderado** — auditor P186A confirmou
  empiricamente que `SetEquationNumbering` está ausente,
  mas pode haver outro produtor de
  `numbering_active:equation`.
- **Tests existentes regridem**: indica que `flat_counter_at`
  retorna valor diferente do legacy quando state é
  injectado. Investigar — pode ser que counter parte
  de índice diferente (0-based vs 1-based) ou que
  `apply_at` semântica difere de `get_flat`. Cláusula
  gate substancial.
- **Conversão de tipo necessária** (improvável per
  P188A §11.1 — ambos `usize`): cláusula gate trivial.
- **Snapshot tests divergem**: improvável (output
  observable preservado por construção). Se acontecer,
  investigar.
- **Linter divergência V13/V14**: cláusula gate trivial
  — `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S agregado. ~5 LOC consumer + comentário
  inline + ~80 LOC tests + edits L0 + relatório
  consolidado.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**: P187B literal + diferença
  sintáctica (`unwrap_or_else` em vez de `or_else`) +
  honestidade documental.
- **Cláusula gate trivial**: aplicável a forma exacta
  da expressão, localização do site, recálculo de
  hashes, estrutura de tests.
- **Cláusula gate substancial**: aplicável apenas se
  test 2 (caso central produção) falhar
  (indica produtor não-inventariado de
  `numbering_active:equation`) ou se tests existentes
  regridirem.
- **Test 2 é gate de qualidade do passo** — empiricamente
  valida que cenário de produção real é coberto pelo
  fallback legacy. Se falhar, confirmação de
  `SetEquationNumbering` ausente está em causa.
- **Inversão observable diferente face a P184D e P187B**:
  P188 é o **primeiro caso** onde migração estrutural
  não muda caminho funcional em produção. Auditor P186A
  identificou isto; P188 honra.
- **M4-residual fechado após P188B**: 8/12 read-sites
  migrados (C1, C3, e 6 outros via P181G/P182D/P184D/P186).
  C2 estruturalmente fechado. Restantes 4/12 são
  fora-de-escopo M4-residual (TOC, side-channels,
  resolved labels).
- **Próximo passo após P188B**: **P189 (M5 walk puro)**.
  Já produzido como instrução base; revisão pequena
  pode ser necessária para reflectir que C4 (resolved
  label) também ficou pendente em P183E não corrido.
