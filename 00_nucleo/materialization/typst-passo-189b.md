# Passo P189B — Migrar Outline + documentar 5 excepções

Único passo de implementação P189 (após P189A diagnóstico).
Magnitude **S** agregada — passo único combinando migração
Outline, auditoria CounterUpdate, documentação 5 excepções
em 4 pontos cada, tests sentinela, e relatório consolidado.

**Início incremental de M5**, não fim. Per P189A §11.6,
M5 universal exige 3-4 passos pré-requisito (sub-store
`resolved_labels`, C4 migration, sub-store
`headings_for_toc`, `SetEquationNumbering`) antes de
fechar.

P189B fecha:
- 1 arm migrável: `Content::Outline` (eixo 1 ✅ snapshot
  final; eixo 2 ✅ `kind_index[Outline]` populado por
  P178).
- 1 arm a auditar empiricamente: `Content::CounterUpdate`
  (decisão α/β/δ em `.A`).
- 5 excepções declaradas (Equation, Heading.hierarchical,
  Figure+Labelled cadeia, headings_for_toc,
  SetHeadingNumbering).

**Pré-condição**: P189A concluído. Tests workspace 1.808
verdes; zero violations. Decisões 7 cláusulas P189A
fechadas. 5 excepções identificadas; sub-store
`resolved_labels` confirmado ausente; cadeia de
dependências Heading→Labelled→resolved_labels documentada.

**Restrições**:
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar Layouter struct (P185C fechou).
- **Não** abrir sub-store `resolved_labels` — passo
  dedicado fora série.
- **Não** materializar `SetEquationNumbering` — passo
  dedicado fora série.
- **Não** migrar C4 — passo dedicado fora série.
- **Não** abrir sub-store `headings_for_toc` — passo
  dedicado fora série.
- **Não** abrir DEBT M5-residual formal (Cenário B per
  P189A §8 — apenas notas preventivas).
- **Não** migrar arms excepcionados — apenas documentar.
- API pública preservada.
- Output observable em produção **inalterado** — Outline
  migração preserva paridade; excepções continuam a
  funcionar via legacy.

---

## Sub-passos

### .A Auditoria + decisão CounterUpdate

1. Confirmar arm `Content::Outline` em
   `01_core/src/rules/introspect.rs`:
   - Per P189A §11.3: linha 611 (a confirmar
     empiricamente — auditor M4-residual descobriu várias
     vezes que linhas mudam entre passos).
   - Localizar mutação `state.has_outline = true` (ou
     similar).
   - Identificar contexto exacto.

2. Confirmar consumer Outline em `mod.rs:1423`:
   - Per P189A §2.5: `layout_with_introspector` lê
     `state.has_outline`.
   - Localizar leitura legacy.
   - Confirmar que `intr.kind_index` está acessível no
     site.

3. Auditar `Content::CounterUpdate` arm:
   - `grep -rn "CounterUpdate" 01_core/src/rules/introspect.rs`.
   - Localizar arm + mutações.
   - Aplicar regra dos 2 eixos:
     - Eixo 1: consumer downstream precisa de valor
       "durante walk" (mutável) ou snapshot final?
     - Eixo 2: sub-store correspondente populado?
   - Se ambos eixos passam: migrável (Opção α ou β).
   - Se algum falha: excepção (Opção δ).

4. Inventariar 5 excepções confirmadas (per P189A §3):
   - **E1**: Equation walk arm (Reserva 1 — sem
     `SetEquationNumbering`).
   - **E2**: Heading.hierarchical mutation (cadeia
     Heading→Labelled→resolved_labels).
   - **E3**: Figure+Labelled (idem cadeia).
   - **E4**: `headings_for_toc` (lacuna #3 não fechada).
   - **E5**: SetHeadingNumbering (a confirmar — pode ser
     migrável via P171 StateUpdate; auditor decide
     empiricamente).

5. Confirmar L0s actuais:
   - `00_nucleo/prompts/rules/introspect.md`.
   - `00_nucleo/prompts/rules/layout.md` (consumer
     Outline).
   - Identificar onde adicionar:
     - Secção "Walk puro M5 incremental" + nota sobre
       Outline migrado.
     - Secção "Excepções M5" com 5 entradas.

6. Confirmar tests existentes que cobrem caminhos
   afectados:
   - `grep -rn "has_outline\|Outline" 01_core/src/rules/`.
   - Identificar quais devem manter-se inalterados após
     P189B (paridade observable preservada).

Output: tabela com:
- Site Outline confirmado.
- Decisão CounterUpdate (migrável α/β ou excepção δ).
- 5 excepções documentadas.
- L0s localizados.
- Tests existentes inventariados.

**Critério de saída**:
- Outline arm + consumer localizados.
- CounterUpdate decidido empiricamente.
- 5 excepções (potencialmente 6 se CounterUpdate for δ)
  confirmadas.

### .B Migrar Outline arm

1. Em `introspect.rs:611` (ou linha real per `.A.1`):
   - Remover mutação `state.has_outline = true`.
   - Adicionar comentário inline: walk puro para Outline
     — flag obtida via `intr.kind_index` em vez de state.
   - Confirmar que arm continua a fazer recursão correcta
     (walk do body se houver).

2. Em `mod.rs:1423` (consumer):
   - Substituir leitura legacy por:
     ```
     intr.kind_index.contains_key(&ElementKind::Outline)
     ```
   - Forma exacta fica para Claude Code.

3. Confirmar `@prompt-hash` actualiza após edits.

**Critério de saída**:
- `cargo check --workspace` passa.
- Outline arm puro (sem mutação directa).
- Consumer lê via Introspector.
- Tests existentes não regridem.

### .C Migrar (ou excepcionar) CounterUpdate per `.A.3`

**Cenário 1** — `.A.3` decide migrável:
1. Aplicar Opção α ou β conforme decisão.
2. Adicionar arm em `from_tags` se Opção α.
3. Confirmar paridade.

**Cenário 2** — `.A.3` decide excepção:
1. Adicionar comentário inline justificando excepção.
2. Adicionar à lista de 5 excepções (passa a 6).
3. Cross-reference ao pré-requisito que desbloqueia.

Output: decisão materializada conforme `.A.3`.

**Critério de saída**:
- CounterUpdate tratado conforme decisão `.A.3`.
- `cargo check --workspace` passa.
- Tests existentes não regridem.

### .D Documentar 5 excepções (4 pontos cada)

Per P189A Q6 + replicação padrão P188B `.B`/`.C`/`.D`/`.G`:

#### Ponto 1 — Comentário inline

Para cada excepção (E1–E5; E6 se CounterUpdate δ):
- Adicionar comentário inline no arm walk
  correspondente em `introspect.rs`.
- Texto sugerido (curto, factual):
  ```
  // Excepção M5: walk muta state diretamente porque
  // <razão>. Migração depende de <pré-requisito>.
  // Vide P189 consolidado §"Excepções M5".
  ```
- Cross-reference específico para cada excepção:
  - E1: Reserva 1 — `SetEquationNumbering`.
  - E2/E3: Reserva 2 alargada — sub-store
    `resolved_labels` + C4 migration.
  - E4: Lacuna #3 — sub-store `headings_for_toc`.
  - E5: TBD per `.A` (pode ser migrável via P171).
  - E6 (se CounterUpdate excepcionado): per `.A.3`.

#### Ponto 2 — L0 `rules/introspect.md`

Adicionar secção "Excepções M5" listando todas as
excepções com:
- Razão.
- Pré-requisito que desbloqueia.
- Cross-reference ao P189 consolidado.

#### Ponto 3 — Tests sentinela

Para cada excepção, 1 test que valida que comportamento
em produção é preservado:
- `walk_excepcao_E1_equation_counter_via_legacy`.
- `walk_excepcao_E2_heading_hierarchical_via_legacy`.
- `walk_excepcao_E3_figure_labelled_via_legacy`.
- `walk_excepcao_E4_headings_for_toc_via_legacy`.
- `walk_excepcao_E5_set_heading_numbering` (ou per
  decisão `.A`).
- `walk_excepcao_E6_counter_update_via_legacy` (se
  aplicável).

Tests confirmam que cada excepção não regride
funcionalmente — walk legacy continua a popular state
correctamente.

#### Ponto 4 — Secção em P189 consolidado

Em `.E`.

**Critério de saída**:
- 4 pontos de documentação materializados para cada
  excepção.
- Tests sentinela passam.
- Tests existentes não regridem.

### .E Escrever relatório consolidado P189

1. Criar
   `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
   com 9 secções (padrão P188 / P187 / etc.):

   - §1 Resumo executivo + Outline migrado +
     M5 incremental.
   - §2 Sub-passos materializados (tabela métricas A–E
     dentro de P189B único).
   - §3 Decisões arquitecturais (7 cláusulas P189A
     fechadas + decisão CounterUpdate `.A.3`).
   - §4 Achados não-triviais durante execução:
     - P189A §11.1 — Reserva 2 alargada (sub-store
       resolved_labels ausente, não apenas C4).
     - P189A §11.2 — cadeia de dependências bloqueia
       migração granular.
     - P189A §11.3 — Outline é único migrável trivial.
     - P189A §11.6 — P189 é início incremental de M5.
     - Achados P189B `.A.3` (CounterUpdate decisão).
     - Achados P189B `.D` (excepções materializadas).
   - §5 **Excepções M5** (secção dedicada — Ponto 4 da
     documentação obrigatória):
     - Lista completa (E1–E5 ou E1–E6).
     - Cada uma com: razão + pré-requisito + plano de
       fechamento.
     - Cross-reference a passos futuros que cada uma
       desbloqueia.
   - §6 Estado final M9 (inalterado 11/11) e M5
     (1 arm migrado; 5-6 excepções).
   - §7 Estado final lacunas (#3 ainda activa per E4).
   - §8 Pendências cumulativas + DEBT M5-residual
     vazio em prática (Cenário B per P189A §8).
   - §9 Próximos passos sugeridos:
     - Pré-requisitos M5 universal (4 trabalhos
       identificados).
     - P190 (M6) — só pode fechar parcialmente até M5
       fechar universalmente.
     - Ordem inversa à mutação documentada.

2. **Nota DEBT M5-residual** (per P189B `.D` Ponto 4):
   > Após P189, DEBT M5-residual cobre 5-6 excepções
   > declaradas. Quando pré-requisitos fecharem
   > (sub-store `resolved_labels` + C4 migration; sub-store
   > `headings_for_toc`; `SetEquationNumbering`),
   > excepções fecham incrementalmente. Walk torna-se
   > universalmente puro. Segue M6 (eliminação
   > `CounterStateLegacy`).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes (com §5 dedicada a excepções).
- Nota DEBT M5-residual documentada.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P189A
   baseline (1.808): +5 a +7 (1 test paridade Outline +
   5-6 tests sentinela excepções).
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. `state.has_outline` mutation **REMOVIDA** de
   `introspect.rs` arm Outline.
5. Consumer `mod.rs:1423` lê via
   `intr.kind_index.contains_key(&ElementKind::Outline)`.
6. CounterUpdate tratado conforme `.A.3` (migrado ou
   excepcionado).
7. **5-6 comentários inline de excepção** presentes em
   `introspect.rs`.
8. L0 `rules/introspect.md` com secção "Excepções M5".
9. 5-6 tests sentinela de excepção passam.
10. 1 test paridade Outline passa.
11. Tests existentes não regridem.
12. Snapshot tests ADR-0033 verdes.
13. Linter passa final.

### .G Encerramento

P189B é o passo único de implementação. Após `.E`
concluído, série P189 está fechada (incrementalmente).

Estado projectado pós-P189B:

- **P189 série**: A ✅ B ✅. Fechada (incrementalmente).
- **M5 progresso**: 1 arm migrado (Outline) +
  5-6 excepções declaradas.
- **M5 universal**: ainda **não fecha** — bloqueado por
  4 pré-requisitos.
- **DEBT M5-residual**: cobre 5-6 excepções (Cenário B —
  notas preventivas).
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso (read-sites)**: 8/12 (inalterado —
  P189 não migra read-sites).
- **60 passos executados** (per P189A §12: P189A = 59 +
  P189B = 60).
- **Padrão diagnóstico-primeiro**: 14ª aplicação
  consecutiva (P189A na lista — 14/14 acertaram a
  magnitude planeada ±1 nível).
- **Próximo: passos pré-requisito M5 ou M6 parcial
  (P190)**.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial;
   CounterUpdate decidido empiricamente.
2. Outline arm migrado (`introspect.rs:611`);
   consumer migrado (`mod.rs:1423`).
3. CounterUpdate tratado conforme `.A.3`.
4. 5-6 excepções declaradas com 4 pontos de documentação
   cada.
5. 1 test paridade Outline passa.
6. 5-6 tests sentinela de excepção passam.
7. Tests existentes não regridem.
8. Verificações `.F` passam (13/13).
9. Relatório consolidado P189 (9 secções com §5
   dedicada a excepções) escrito.
10. Output observable em produção **inalterado** — Outline
    paridade preservada; excepções continuam funcionais
    via legacy.

---

## O que pode sair errado

- **Site Outline mudou** entre P189A e P189B (improvável
  mas possível): cláusula gate trivial — ajustar
  referência em `.B`.
- **Consumer Outline acede a `state.has_outline` em mais
  sítios além de `mod.rs:1423`**: cláusula gate trivial —
  migrar todos os sítios.
- **CounterUpdate revela cadeia de dependências similar a
  Heading→Labelled**: cláusula gate substancial —
  excepcionar (Opção δ); registar como E6.
- **Test paridade Outline falha**: indica que migração
  altera output observable. Investigar — pode ser que
  `kind_index[Outline]` é populado em momento diferente
  do `state.has_outline = true`. Cláusula gate
  substancial. **Risco moderado**.
- **Test sentinela de excepção falha**: indica que walk
  excepcionado não está a popular state como esperado.
  Investigar.
- **Tests existentes regridem por mudança em
  `state.has_outline`**: indica que ainda há consumers
  legacy não inventariados em `.A.2`. Investigar.
- **Snapshot tests divergem**: improvável (output
  preservado por construção). Se acontecer, investigar.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S agregado. ~10 LOC produção (Outline
  migração + 5-6 comentários inline) + ~150 LOC tests
  + edits L0 + relatório consolidado.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**:
  - Migração Outline: padrão P184D (substitution sem
    fallback porque legacy desaparece de uma vez).
  - Excepções: padrão P188B (4 pontos de documentação
    obrigatória).
- **Cláusula gate trivial**: aplicável a divergência de
  linhas, locais de consumer, formato de tests.
- **Cláusula gate substancial**: aplicável apenas se
  CounterUpdate revelar bloqueador inesperado, se test
  paridade falhar, ou se consumers Outline forem mais
  numerosos do que inventariado em `.A.2`.
- **Test paridade Outline é gate de qualidade do passo**
  — empiricamente valida que migração preserva output
  observable. Se falhar, P189B regista bloqueio e
  re-arquitecta.
- **5-6 tests sentinela são contracto de não-regressão**
  para arms excepcionados — empiricamente validam que
  walk legacy continua funcional para esses arms.
- **Honestidade obrigatória sobre M5 incremental**:
  documentação em §5 do consolidado + 5-6 comentários
  inline + secção em L0 + 5-6 tests sentinela.
- **M5 não fecha após P189B**: chave de honestidade
  institucional. Próximos passos identificados para
  fechamento universal.
- **Ordem inversa à mutação documentada em §9 do
  consolidado**: para fechar M5 universalmente, migração
  tem que acontecer da camada mais baixa (sub-stores) para
  a mais alta (Layouter consumers). P190 (M6) só pode
  fechar parcialmente até M5 universalmente fechar.
