# Passo P187B — Migrar C1 heading prefix + tests E2E

Único passo de implementação P187 (após P187A diagnóstico).
Magnitude **S** agregada — passo único combinando migração,
tests E2E, edits L0, e actualização de nota DEBT.

Migra consumer C1 em `01_core/src/rules/layout/mod.rs:345`
(per P187A §11.1 — site real é 345, não 310 como specs
anteriores indicavam) de
`self.counter.format_hierarchical("heading")` para a forma
substitution-with-fallback location-aware fixada em P187A
§3:

```
self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"))
```

Após P187B:
- C1 consulta Introspector primeiro com `current_location`
  (P185C); fallback legacy quando Introspector retorna
  `None` ou quando `current_location` é `None`.
- C1 é o **primeiro consumer onde Introspector é caminho
  funcional real** para counter hierárquico (heading
  prefix), diferente de C2 em P186 (dormente).
- P183B aprendizado validado retroactivamente — estratégia
  estava certa; primitiva precisava de P185 location-aware.
- DEBT M4-residual reduz para cobrir apenas C2 (cenário B
  per P187A §8).

**Pré-condição**: P187A concluído. Tests workspace 1.801
verdes; zero violations. Forma da expressão fixada
(combinação Opção B + Opção A das cláusulas P187A).

**Restrições**:
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar Layouter struct (P185C fechou).
- **Não** modificar `formatted_counter_at` ou
  `current_location` API.
- **Não** modificar walk arm legacy.
- **Não** migrar C2 — P188.
- **Não** abrir DEBT M4-residual formal (cenário B per
  P187A §8 — apenas nota preventiva nos relatórios).
- API pública preservada.
- Output observable em produção pode mudar — Introspector
  path activa para casos de re-update onde antes fallback
  legacy era o caminho funcional. **Verificação central**:
  paridade observable confirmada via tests E2E.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C1 em `mod.rs:345`:
   - Per P187A §11.1, site real é 345 (não 310). Verificar
     empiricamente — se localização mudou novamente entre
     P187A e P187B, ajustar.
   - Localizar leitura `self.counter.format_hierarchical("heading")`.
   - Identificar contexto exacto (arm `Content::Heading`
     em `layout_content`).

2. Confirmar acesso a `self.introspector` e
   `self.current_location`:
   - Ambos confirmados em P187A §2.2 e §2.3. Re-verificar
     empiricamente.

3. Confirmar L0 `rules/layout.md`:
   - Localizar entrada existente sobre heading-arm
     (P182D introduziu, P186 estendeu).
   - Identificar onde adicionar nota sobre migração C1.

4. Confirmar tests existentes que cobrem heading prefix:
   - `p182d_heading_numbering`.
   - `p182e_e2e_heading_numbering`.
   - Outros que possam regredir.
   - Identificar quais devem manter-se inalterados após
     P187B (paridade observable preservada).

5. Confirmar blueprint P185D `.E`:
   - `pipeline_e2e_is_numbering_active_at_via_current_location`
     em `tests.rs` `p185d_locator_sync` submódulo.
   - Adaptar para `formatted_counter_at` symetricamente.

6. Confirmar nota DEBT M4-residual em relatórios:
   - `grep -rn "DEBT M4-residual\|C1.*C2\|M4.residual"
     00_nucleo/`.
   - Localizar notas em P184F, P185-consolidado,
     P186-consolidado.
   - Identificar quais precisam de actualização para
     reflectir "cobre apenas C2 após P187".

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Site C1 confirmado.
- Tests existentes inventariados (esperado: paridade
  preservada).
- Blueprint P185D `.E` localizado.
- Notas DEBT M4-residual localizadas para actualização.

### .B Migrar consumer C1

1. Em `01_core/src/rules/layout/mod.rs:345` (ou linha
   real per `.A.1`):
   - Substituir leitura legacy por forma fixada P187A
     §3:
     ```
     self.current_location
         .and_then(|loc| self.introspector
             .formatted_counter_at("heading", loc))
         .or_else(|| self.counter.format_hierarchical("heading"))
     ```
   - Forma exacta fica para Claude Code conforme
     convenção do projecto (variável intermédia vs
     inline).

2. Confirmar `@prompt-hash` actualiza após edit do L0
   (vai ser actualizado em `.C`).

**Critério de saída**:
- `cargo check --workspace` passa.
- Tests existentes não regridem (paridade observable
  preservada por construção — P185 location-aware retorna
  valores correctos por Location).
- Linter passa (após `--fix-hashes` em `.D`).

### .C Actualizar L0 `rules/layout.md`

1. Adicionar entrada para C1 migration:
   - Heading prefix consultado via Introspector
     `formatted_counter_at("heading", current_location)`
     primeiro.
   - Fallback legacy `format_hierarchical("heading")`
     activo durante janela compat M6.
   - Justificação: P185 forneceu primitiva location-aware
     que P183B aprendizado identificou como necessária.
   - Cross-reference: P184D Figure (padrão idêntico),
     P185D `.E` (blueprint).

2. Hash em branco aguarda recálculo manual (`.D`).

**Critério de saída**:
- L0 contém entrada para C1 migration.
- Coerente com entradas P182D e P186 existentes.

### .D Tests E2E em submódulo `p187b_c1_heading_prefix`

Submódulo novo em `01_core/src/rules/layout/tests.rs`,
irmão de `p184e_figure_per_kind`, `p185d_locator_sync`,
`p186f_equation_locatable`. 4 tests obrigatórios.

#### Test 1 — `c1_heading_prefix_via_introspector_path`

1. Construir documento típico com 3 headings:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       heading(level=1, body="Intro"),
       heading(level=2, body="Motivação"),
       heading(level=1, body="Conclusão"),
   ])
   ```

2. Pipeline:
   - Walk + from_tags → `TagIntrospector` populado.
   - Layouter `layout_with_introspector(content, &intr)`.

3. Asserções:
   - `plain_text` contém prefixos "1." (Intro), "1.1"
     (Motivação), "2." (Conclusão).
   - Validação intermédia: `intr.formatted_counter_at(
     "heading", loc_intro)` retorna `Some("1.")`.

#### Test 2 — `c1_heading_prefix_via_fallback_legacy`

1. Mesmo documento.
2. Pipeline com `TagIntrospector::empty()` em vez de
   populado (força fallback).
3. Asserções:
   - `plain_text` contém prefixos correctos via fallback
     `format_hierarchical("heading")` legacy.
   - Output observable idêntico ao test 1.

#### Test 3 — `c1_heading_prefix_paridade_legacy_vs_migrated`

1. Mesmo documento.
2. Path A: `layout()` legacy (sem Introspector).
3. Path B: `layout_with_introspector(content, intr_populado)`.
4. Asserção: `plain_text(A) == plain_text(B)`.

#### Test 4 — `c1_heading_prefix_re_update_correctness`

**Caso central** que P183B falhou. Replica empiricamente
o cenário onde location-aware é decisivo:

1. Construir documento com sequência H1, H2, H1:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       heading(level=1, body="A"),  // esperado "1."
       heading(level=2, body="B"),  // esperado "1.1"
       heading(level=1, body="C"),  // esperado "2."
   ])
   ```

2. Pipeline `layout_with_introspector`.

3. Asserções:
   - `plain_text` contém "1.", "1.1", "2." em ordem.
   - **Diferente de P183B onde output era "2.", "2.",
     "2."** porque `formatted_counter` snapshot-final
     pré-emptava fallback.
   - Empiricamente valida que P185 desbloqueio resolve
     P183B.

**Critério de saída**:
- 4 tests novos passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P187A
   baseline (1.801): +4.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p187b_c1_heading_prefix::*` passam isoladamente
   (`cargo test --workspace --lib p187b`).
5. Tests existentes (`p182d_*`, `p182e_*`) **não
   regridem** — paridade observable preservada.
6. Consumer C1 (`mod.rs:345`) consulta Introspector
   primeiro com `current_location`; fallback legacy.
7. Walk arm legacy **NÃO modificado**.
8. Trait `Introspector` **NÃO modificado**.
9. Layouter struct **NÃO modificado**.
10. P186 (Equation) **NÃO afectado**.
11. Snapshot tests ADR-0033 verdes.
12. Linter passa final.

### .F Actualizar notas DEBT M4-residual

P187 reduz cobertura de DEBT M4-residual (cenário B per
P187A §8 — apenas notas preventivas).

1. Localizar notas relevantes (per `.A.6`):
   - `00_nucleo/materialization/typst-passo-184-relatorio-consolidado.md`
     §"Pendências cumulativas" (mencionou DEBT cobrindo
     C1+C2).
   - `00_nucleo/materialization/typst-passo-185-relatorio-consolidado.md`
     §7.
   - `00_nucleo/materialization/typst-passo-186-relatorio-consolidado.md`
     §7.

2. **Não editar** os relatórios anteriores
   (preservação histórica). Em vez disso, adicionar
   nota nova no relatório consolidado P187 (`.G`) que
   actualiza estado.

3. Texto sugerido para P187 consolidado:
   > Após P187, DEBT M4-residual cobre apenas C2 (era
   > C1+C2 nos relatórios anteriores). C1 fechado em
   > P187B. Quando P188 fechar C2, DEBT M4-residual
   > torna-se vazio; P183F formal pode arquivar
   > sem cobrir nada (ou ser dispensado).

**Critério de saída**:
- Nota actualizada no relatório consolidado P187 (`.G`).
- Sem edits em relatórios anteriores.

### .G Escrever relatório consolidado P187

1. Criar
   `00_nucleo/materialization/typst-passo-187-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P182F / P184F / P185 /
   P186):

   - §1 Resumo executivo + C1 fechado.
   - §2 Sub-passos materializados (tabela métricas A–F
     dentro de P187B único).
   - §3 Decisões arquitecturais (6 cláusulas P187A
     fechadas).
   - §4 Achados não-triviais durante execução:
     - P187A §11.1 — site real `mod.rs:345` (não 310).
     - P187A §11.3 — P183B retroactivamente validado.
     - P187A §11.4 — heading-arm consolidado (numbering
       active + heading prefix ambos via Introspector).
     - P187B §`.D.4` — test re-update empiricamente
       valida resolução P183B.
   - §5 Estado final M9 (inalterado 11/11) e M5/M4
     (7/12 read-sites; C1 migrado; +1 vs P186).
   - §6 Estado final lacunas (inalterado).
   - §7 Pendências cumulativas + DEBT M4-residual
     reduzido para C2 apenas + nota actualizada (vide
     `.F`).
   - §8 Próximos passos sugeridos:
     - P188 (migrar C2) — pode prosseguir; blueprint
       similar a P187B mas com Introspector path
       dormente em produção.
     - Após P188: M4-residual fechado; segue M5 (P189).
   - §9 Conclusão.

2. Sem L0 novo; sem alteração de tests; sem ADR; sem
   DEBT formal.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- Nota DEBT M4-residual actualizada.

### .H Encerramento

P187B é o passo único de implementação. Após `.G`
concluído, série P187 está fechada.

Estado projectado pós-P187B:

- **P187 série**: A ✅ B ✅. Fechada.
- **C1 fechado** com Introspector como caminho funcional.
- **Inversão observable** confirmada — P187 é o segundo
  caso (após P184D Figure) onde Introspector é
  funcional, não redundante. Diferente de P186 (Equation
  dormente).
- **DEBT M4-residual**: cobre apenas **C2** (cenário B
  actualizado).
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso**: 7/12 read-sites migrados (era
  6/12).
- **54 passos executados** (per P187A §12 — recontagem
  cumulativa).
- **Padrão diagnóstico-primeiro**: 12ª aplicação
  consecutiva (P187A na lista).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Consumer C1 migrado (`mod.rs:345`).
3. L0 `rules/layout.md` actualizado.
4. 4 tests E2E novos passam.
5. Tests existentes não regridem (paridade observable).
6. Verificações `.E` passam (12/12).
7. Nota DEBT M4-residual actualizada.
8. Relatório consolidado P187 (9 secções) escrito.
9. Output observable em produção preservado para casos
   típicos; **caminho funcional muda** — Introspector
   passa a ser fonte primária para C1.

---

## O que pode sair errado

- **Site C1 mudou novamente** entre P187A e P187B
  (improvável mas possível): cláusula gate trivial —
  ajustar referência em `.B`.
- **`current_location` é `None` no site** (improvável per
  P187A §2.5 que confirmou gating precede arm Heading):
  cláusula gate trivial — Opção B `and_then` cobre via
  fallback legacy.
- **Test 4 (re-update) falha**: indica que P185 não
  desbloqueou completamente o caso. Cláusula gate
  substancial — investigar antes de prosseguir. **Risco
  central** — se este test falha, P187 não fecha.
- **Tests existentes regridem**: indica que Introspector
  retorna valor diferente do legacy para casos típicos.
  Investigar — pode ser que `formatted_counter_at`
  retorna formato diferente do `format_hierarchical`
  legacy. Cláusula gate substancial.
- **Snapshot tests divergem**: similar ao acima.
  Investigar.
- **Linter divergência V13/V14**: cláusula gate trivial
  — `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S agregado. ~5 LOC consumer + ~120 LOC
  tests + edits L0 + relatório consolidado.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**: P184D Figure literal +
  primitiva location-aware.
- **Cláusula gate trivial**: aplicável a forma exacta da
  expressão, localização do site, recálculo de hashes.
- **Cláusula gate substancial**: aplicável apenas se test
  re-update falhar (indica P185 desbloqueio incompleto)
  ou se snapshot tests divergirem inesperadamente.
- **Test 4 (re-update) é gate de qualidade do passo** —
  empiricamente valida que P185 desbloqueio resolve
  P183B aprendizado. Se falhar, ADR-0068 ACEITE estaria
  em causa retroactivamente.
- **Inversão observable confirmada após P187B**: P187 é
  o segundo caso da série M4-residual onde Introspector
  é caminho funcional (depois de P184D Figure). Counter
  contagem M5/M4 sobe para 7/12.
- **Próximo: P188 (C2 migration)** com blueprint similar
  mas Introspector dormente em produção até `Content::SetEquationNumbering`
  materializar. Diferença observable importante —
  P188 documenta honestamente.
