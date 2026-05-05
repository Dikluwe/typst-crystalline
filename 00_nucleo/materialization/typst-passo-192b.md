# Passo P192B — Declaração formal M7 estruturalmente fechado + ADRs

Segundo e último passo da série P192 (após P192A
diagnóstico). Magnitude **S** — passo declarativo
puro:
- ADR-0072 nova (M7 estruturalmente fechado;
  hash-based convergence intermédio).
- ADR-0066 PROPOSTO → ACEITE com nota
  "intermediário até M8".
- Relatório consolidado P192 (9 secções + §10
  marco).
- Corrigir narrativa P192A §4.3 (divergência foi
  intermédia, não intencional permanente).
- Cross-references.

P192A confirmou empiricamente:
- **Estado A** — M7 estruturalmente fechado.
- 2 loops fixpoint distintos (TOC + run_fixpoint).
- 4 queries runtime location-aware activas.
- Tests E2E cobrem ambos loops (13+ tests).
- Hash-based convergence funciona como mecanismo
  intermédio.
- **Correcção interpretativa**: divergência sem
  comemo é **intermédia**, não permanente. Paridade
  comemo é objectivo final (M8).

P192B é trabalho **declarativo** com **interpretação
arquitectural correcta**:
- M7 fechado estruturalmente — hash-based
  convergence funciona; comemo virá em M8.
- M8 redefinido: introduzir `comemo::Track` em
  Introspector + queries para alcançar paridade
  vanilla typst.

Trabalho concreto:
1. Criar
   `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md`
   ACEITE imediato com clarificação "fechamento
   estrutural; paridade comemo em M8".
2. Transitar `ADR-0066` PROPOSTO → ACEITE com nota
   "intermediário até M8":
   - Hash-based convergence é decisão intermédia
     viável.
   - Paridade comemo é objectivo final.
   - M8 introduzirá `comemo::Track`.
3. Criar
   `00_nucleo/materialization/typst-passo-192-relatorio-consolidado.md`
   (9 secções + §10 marco) com narrativa correcta:
   - M5+M6+M7 **estruturalmente fechados**.
   - M8 (comemo) próximo passo natural para
     paridade vanilla.
4. Cross-references actualizadas.

Após P192B:
- ADR-0072 ACEITE registada (fechamento estrutural).
- ADR-0066 ACEITE com nota "intermediário até M8".
- 7 ADRs ACEITES no ciclo M5/M6/M7 (0066, 0067,
  0068, 0069, 0070, 0071, 0072).
- M5+M6+M7 estruturalmente fechados.
- Tests workspace 1.802 verdes (inalterados).
- Pattern "auditoria sobre estado existente" 1ª
  aplicação documentada.
- **M8 reconhecido como próximo passo natural**.

**Pré-condição**: P192A concluído com Estado A
confirmado. Tests workspace 1.802 verdes; zero
violations.

**Restrições**:
- **Não** modificar código produção — passo
  documental.
- **Não** modificar tests existentes.
- **Não** modificar trait `Introspector`,
  `TagIntrospector`, `LayouterRuntimeState`.
- **Não** modificar `fixpoint.rs`.
- **Não** modificar Layouter.
- **Não** materializar lacunas residuais.
- **Não** introduzir novos defers.
- **Corrigir interpretação P192A §4.3** — sem
  inflar a correcção; nota arquivística simples.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P192A:

1. Tests workspace 1.802 verdes.
2. Linter zero violations.
3. Estado A confirmado per P192A relatório §6.
4. 2 loops fixpoint funcionais.
5. 4 queries runtime location-aware activas em
   Layouter.
6. ADR-0066 estado actual: PROPOSTO desde
   2026-04-27.
7. ADRs ACEITES até agora: 5 (ADR-0067, 0068,
   0069, 0070, 0071) + ADR-0066 a transitar +
   ADR-0072 a criar = 7 após P192B.
8. Marcos arquitectónicos:
   - M5 universal completo (P200B).
   - M6 fechado completo (P190I).
   - M9 11/11 estável.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 8 verificações empíricas passam.
- Tests 1.802 inalterados.

### .B Criar ADR-0072

1. Criar
   `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md`:

```markdown
# ADR-0072 — M7 fixpoint runtime estruturalmente fechado

**Estado**: ACEITE
**Data**: 2026-05-05
**Sub-passo**: P192B

## Contexto

M7 (loop fixpoint runtime) era marco arquitectural
pendente para resolver dependências reverse em
introspection (page numbers TOC; queries runtime
stdlib).

P192A diagnóstico revelou que M7 estava
**estruturalmente fechado** por força de sequência
incremental P174 → P175-P179 → M9 → P190 série →
P191 série, sem ADR explícita.

## Decisão

M7 fechado **estruturalmente** via dois loops
fixpoint complementares:

1. **TOC fixpoint** (`layout/mod.rs:1515`):
   - Resolve forward refs em page numbers.
   - Activo em produção quando há Outline.
   - MAX_ITERATIONS = 5 (paridade vanilla).
   - Convergência via hash `extracted_label_pages`
     map.

2. **`run_fixpoint`** (`introspect/fixpoint.rs:65`):
   - Mecanismo opt-in para stdlib features.
   - Estruturalmente pronto; sem clientes runtime
     em produção.
   - MAX_ITERATIONS = 5.
   - Convergência via `compute_tags_hash`.

Os dois loops resolvem categorias distintas de
dependências reverse (não redundantes).

## Distinção crítica — fechamento estrutural

M7 fechado **estruturalmente**, **não
arquiteturalmente definitivo**. Hash-based
convergence é decisão **intermédia viável**:
- Funciona empíricamente.
- Tests E2E verdes.
- 2 loops complementares cobrem categorias
  distintas.

**Mas** paridade vanilla typst exige `comemo::Track`
para:
- Memoização cross-iteration (evitar re-walk full
  por iteração).
- Tracking granular de dependências.
- Performance comparável.

**M8 introduzirá comemo** — adopção planeada como
próximo passo natural após M7 estrutural.

## Consequências

- M7 declarado estruturalmente fechado em P192B.
- M5 universal completo + M6 fechado + M7
  estruturalmente fechado: cristalino atinge
  consolidação arquitectural intermédia.
- Pattern "auditoria sobre estado existente"
  documentado.
- M8 definido como adopção `comemo` para paridade
  vanilla.

## Alternativas avaliadas

- **comemo imediato em M7**: rejeitado em ADR-0066.
  Hash-based convergence escolhida como mecanismo
  intermédio viável. Comemo planeado para M8.
- **Sem fixpoint**: rejeitado — dependências reverse
  exigem convergência iterativa.

## Cross-references

- P174 (run_fixpoint mecanismo).
- P175-P179 (features stdlib via fixpoint).
- M9 11/11.
- P190I (M6 fechado).
- P191C (ADR-0071 ACEITE).
- ADR-0066 ACEITE em P192B com nota "intermediário
  até M8".
- M8 — adopção comemo (passo futuro).

## Pattern emergente

P192A é primeira instância de **"auditoria sobre
estado existente vs planeamento de trabalho
futuro"**. Distinção:
- Auditorias planeadoras (P190A, P191A, etc.)
  produzem ADR PROPOSTO + plano implementação.
- Auditoria sobre estado existente (P192A) produz
  declaração formal + ADR ACEITE retrospectiva.

Pattern reaproveitável quando trabalho cumulativo
incremental atinge fechamento estrutural sem ADR
explícita.
```

2. Confirmar ficheiro criado.

**Critério de saída**:
- ADR-0072 ACEITE existe com narrativa correcta
  (fechamento estrutural; comemo em M8).
- Cross-references presentes.

### .C Transitar ADR-0066 → ACEITE com nota intermediário

1. Editar
   `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`:
   - Estado: PROPOSTO → **ACEITE** com nota.
   - Adicionar secção nova:

```markdown
## Validação empírica P192A + estado intermediário

**Data**: 2026-05-05 (P192B).

ADR-0066 transita PROPOSTO → ACEITE com qualificação
**intermediário até M8**.

### Validação empírica

P192A diagnóstico confirmou que decisão de adiar
introspection runtime (e adoptar hash-based
convergence como mecanismo intermédio) é viável:
- M7 estruturalmente fechado (P192B; ADR-0072).
- 2 loops fixpoint funcionais (TOC + run_fixpoint).
- Tests E2E verdes (1.802 workspace).
- 13+ tests fixpoint.rs.

### Estado intermediário

Hash-based convergence é decisão **intermédia
viável**, **não solução arquitectural definitiva**.

Diferenças vs vanilla typst:
- **Vanilla**: comemo invalida cache granularmente;
  re-walks parciais possíveis.
- **Cristalino actual**: hash-based; re-walk full
  por iteração (custo MAX_ITERATIONS = 5).

### M8 — adopção comemo planeada

M8 introduzirá `comemo::Track` em:
- Trait `Introspector` (`#[comemo::track]`).
- Queries location-aware
  (`is_numbering_active_at`, `flat_counter_at`,
  etc.).
- Sub-stores `TagIntrospector`.

Objectivos:
- Paridade vanilla typst.
- Saída igual ao vanilla.
- Performance comparável.

ADR-0066 cobre **decisão de adiar** comemo até M5+M6+M7
estarem estruturalmente fechados. Cumprido em P192B.
**Próximo passo natural**: M8 — ADR dedicada à
adopção comemo.

### Cross-references

- P192A diagnóstico.
- P192B declaração M7 estruturalmente fechado.
- ADR-0072 ACEITE.
- M8 (próximo).
```

2. Confirmar ficheiro actualizado.

**Critério de saída**:
- ADR-0066 ACEITE com nota intermediário.
- Validação empírica registada.
- M8 reconhecido como próximo passo natural.

### .D Relatório consolidado P192

Criar
`00_nucleo/materialization/typst-passo-192-relatorio-consolidado.md`
com 9 secções padrão + secção 10 dedicada ao marco:

- **§1 Resumo executivo**: M7 **estruturalmente
  fechado** confirmado em P192A; 2 loops fixpoint
  complementares; ADR-0072 ACEITE; ADR-0066 ACEITE
  com nota "intermediário até M8"; pattern
  "auditoria sobre estado existente" documentado;
  cristalino atinge consolidação arquitectural
  intermédia (M5+M6+M7+M9 estruturalmente fechados;
  paridade vanilla via comemo virá em M8).

- **§2 Sub-passos materializados**:

  | Passo | Magnitude planeada | Magnitude real | Δ tests | Outputs |
  |---|---|---|---|---|
  | P192A | S-M | S-M | 0 | Diagnóstico + relatório |
  | P192B | S | S | 0 | ADR-0072 + ADR-0066 ACEITE com nota + relatório consolidado |
  | **Total** | S-M agregado | S-M | **0** | 2 ADRs + 2 relatórios |

- **§3 Decisões arquitecturais**:
  - 7 cláusulas P192A fechadas.
  - Estado A confirmado.
  - 2 loops fixpoint distintos identificados.
  - **Correcção interpretativa empírica** (vide
    §4.3).

- **§4 Achados não-triviais**:
  - 4.1: 2 loops fixpoint complementares.
  - 4.2: `run_fixpoint` mecanismo opt-in
    estruturalmente pronto sem tracção runtime.
  - **4.3 (CORRIGIDO)**: divergência sem comemo é
    **intermédia, não permanente**. Hash-based
    convergence é mecanismo intermédio viável;
    paridade vanilla via comemo é objectivo final
    (M8). Narrativa P192A §4.3 inicial — "Divergência
    arquitectural intencional sem comemo" — foi
    interpretação prematura. Correcção: divergência é
    **intencional para a fase intermédia**, não
    permanente.
  - 4.4: Layouter location-aware queries são
    pré-condição satisfeita por ADR-0068.

- **§5 Estado activo vs preservado**:
  - Activado (pré-existente): 2 loops fixpoint,
    queries location-aware, etc.
  - Preservado: trait, sub-stores, ADRs
    anteriores.
  - **Pendente para M8**: adopção comemo.

- **§6 Estado final M5, M6, M7, M9**:
  - M5 universal completo (P200B).
  - M6 fechado completo (P190I).
  - **M7 estruturalmente fechado (P192B)**.
  - M9 11/11.
  - Marco: consolidação arquitectural intermédia.

- **§7 Estado final lacunas**:
  - Lacunas residuais inalteradas.

- **§8 Pendências cumulativas**:
  - M7 fechado por declaração formal estrutural.
  - **M8 reconhecido como próximo passo natural** —
    adopção `comemo::Track` para paridade vanilla.
  - F3 completo (Layouter restantes 19 fields)
    pendente.
  - Lacunas residuais (#1, #1b, #2) pendentes.

- **§9 Próximos passos sugeridos**:
  - **M8 (comemo) é próximo passo arquitectural
    natural**.
  - Magnitude esperada: L cross-modular (similar
    M6).
  - Pré-condição arquitectural: M5+M6+M7
    estruturalmente fechados (cumprido).
  - Objectivos: paridade vanilla typst (saída +
    performance).
  - Outras opções ortogonais: F3 completo, lacunas
    residuais, pausa estratégica.

- **§10 Marco arquitectural — Consolidação intermédia M5+M6+M7+M9**:
  - Pattern "auditoria sobre estado existente" 1ª
    aplicação completa.
  - 7 ADRs ACEITES no ciclo M5/M6/M7 (ADR-0066
    com nota intermediário, 0067, 0068, 0069,
    0070, 0071, 0072).
  - 33ª aplicação diagnóstico-primeiro consecutiva
    (P181 → P200 + P190A-I + P191A-C + P192A-B).
  - Histórico fechamento M7:
    ```
    P174 (mecanismo) → P175-P179 (features stdlib)
    → M9 11/11 → P190 série → P191 série → P192A
    diagnóstico → P192B declaração formal
    ```
  - **Significado**: M5+M6+M7+M9 fechados
    **estruturalmente**; cristalino tem mecanismo
    funcional para todas as principais features
    typst. **M8 (comemo) elevará para paridade
    arquitectural com vanilla** — saída igual ao
    vanilla + performance comparável.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções + §10 marco.
- Narrativa correcta sobre interpretação
  intermédia.

### .E Cross-references

1. Actualizar L0 master (se existir) ou tracker
   sobre estado de marcos:
   - M5 ✅ universal completo.
   - M6 ✅ fechado completo.
   - **M7 ✅ estruturalmente fechado (P192B)**.
   - M9 ✅ 11/11.
   - **M8 — próximo passo natural (comemo
     adopção)**.

2. Actualizar `00_nucleo/m1-lacunas-captura.md`
   com status M7 estrutural + M8 reconhecido.

3. Hash recalcular se aplicável.

**Critério de saída**:
- Cross-references actualizadas.
- M8 sinalizado como próximo passo.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs
   P192A baseline (1.802): 0**.
3. `crystalline-lint .` zero violations.
4. ADR-0072 ACEITE existe com narrativa correcta
   (fechamento estrutural).
5. ADR-0066 ACEITE com nota "intermediário até
   M8".
6. Relatório consolidado P192 existe (9 + §10
   secções).
7. Cross-references actualizadas.
8. Trait `Introspector` **NÃO modificado**.
9. `TagIntrospector` **NÃO modificado**.
10. `LayouterRuntimeState` **NÃO modificado**.
11. `fixpoint.rs` **NÃO modificado**.
12. Layouter **NÃO modificado**.
13. Tests existentes **NÃO modificados**.
14. **Narrativa P192A §4.3 corrigida** em
    consolidado §4.3 (interpretação intermédia,
    não permanente).
15. Snapshot tests verdes.
16. Linter passa final.

### .G Encerramento

P192B é passo final da série P192. Após `.F`
concluído, série está fechada.

Estado projectado pós-P192B:

- **P192 série**: A ✅ B ✅ — fechada.
- **M7 estruturalmente fechado registado
  formalmente**.
- **7 ADRs ACEITES no ciclo M5/M6/M7**.
- **99 passos executados** (P192A=98 + P192B=99).
- **Tests workspace**: 1.802 (inalterado em P192B
  — passo documental).
- **Padrão diagnóstico-primeiro**: 33ª aplicação
  consecutiva.
- **Pattern "auditoria sobre estado existente"** 1ª
  aplicação completa.
- **Cristalino atinge consolidação arquitectural
  intermédia**: M5+M6+M7+M9 estruturalmente
  fechados.
- **M8 reconhecido como próximo passo natural** —
  adopção `comemo::Track` para paridade vanilla
  typst (saída igual + performance comparável).

**Próximas decisões estratégicas**:
- **M8 (comemo)** — próximo passo arquitectural
  natural; magnitude L cross-modular esperada.
- F3 completo (Layouter restantes 19 fields).
- Lacunas residuais (#1, #1b, #2).
- Pausa estratégica.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P192A empíricamente
   (8/8).
2. ADR-0072 ACEITE criada com narrativa correcta
   "fechamento estrutural" (`.B`).
3. ADR-0066 PROPOSTO → ACEITE com nota
   "intermediário até M8" (`.C`).
4. Relatório consolidado P192 escrito (9 + §10
   secções) com narrativa intermediária correcta
   (`.D`).
5. Cross-references actualizadas (`.E`).
6. Verificações `.F` passam (16/16).
7. Tests workspace 1.802 inalterados.
8. Linter zero violations.
9. Sem código produção tocado.
10. **M7 declarado estruturalmente fechado
    formalmente**.
11. **M8 reconhecido como próximo passo natural**.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**:
  cláusula gate substancial.
- **ADR-0066 tem cross-references obsoletas**:
  cláusula gate trivial.
- **Linter divergência** após edits L0: cláusula
  gate trivial.
- **Cross-references em L0 master incompletas**:
  cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~180 LOC ADRs (0072 nova +
  0066 update com secção nova) + ~280 LOC relatório
  consolidado (com narrativa intermediária).
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **2 ADRs transitadas**:
  - ADR-0072: NOVA (ACEITE imediato; fechamento
    estrutural; comemo em M8).
  - ADR-0066: PROPOSTO → ACEITE com nota
    "intermediário até M8".
- **Correcção interpretativa**: P192A §4.3
  interpretou divergência sem comemo como
  permanente. P192B corrige: divergência é
  intermédia; comemo virá em M8.
- **Marco**: M7 **estruturalmente** fechado.
  Cristalino com M5+M6+M7+M9 fechados — consolidação
  intermédia. **Paridade vanilla via comemo** é
  objectivo final (M8).
- **Cláusula gate trivial**: aplicável a hashes,
  cross-references, formato L0.
- **Sem cláusula gate substancial esperada**.
- **Próximas opções estratégicas após P192B**:
  - **M8 (comemo)**: próximo passo arquitectural
    natural. Magnitude L cross-modular esperada.
    Objectivos: paridade vanilla typst (saída
    igual + performance comparável). Pré-condição
    arquitectural cumprida (M5+M6+M7
    estruturalmente fechados).
  - F3 completo: Layouter 19 fields ortogonais.
  - Lacunas residuais: #1, #1b, #2.
  - Pausa estratégica.
