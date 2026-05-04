# Passo P186F — Tests E2E paridade + fecho série P186

Quinto e último passo de implementação P186 (após P186A
diagnóstico, P186B variants, P186C `extract_payload` arm,
P186D `is_locatable` activado, P186E `from_tags` arm com
gate).
Magnitude **S**.

Adiciona tests E2E em submódulo dedicado e produz
relatório consolidado da série P186. Confirma que infra
está pronta para C2 migrar em P188; documenta gate dormente
em produção; encerra série.

Após P186F:
- 3-5 tests E2E confirmam que `flat_counter_at("equation",
  loc)` retorna valor correcto via pipeline real **quando
  state é injectado**.
- 1 test sentinela confirma gate dormente em produção
  (sem state injectado, counter permanece vazio).
- Relatório consolidado P186A–F produzido (9 secções).
- Série P186 fechada formalmente.
- M4-residual progresso: eixo 2 do bloqueio P183C resolvido
  estruturalmente; C2 pronto para migrar em P188.

**Pré-condição**: P186E concluído. Tests workspace 1.797
verdes; zero violations. Arm `from_tags::Equation` activo
com gate location-aware (`block && state.value_at(...) ==
Some(Bool(true))`); counter populável via injection de
state. Invariante `is_locatable ↔ extract_payload.is_some()`
íntegra.

**Restrições**:
- **Não** modificar código de produção em
  `01_core/src/rules/`, `01_core/src/entities/`,
  `02_shell/`, `03_infra/`, `04_wiring/`.
- **Não** modificar walk arm legacy.
- **Não** modificar variants, `is_locatable`,
  `extract_payload`, `from_tags`.
- **Não** migrar consumer C2 — P188.
- API pública preservada — P186F é apenas tests + docs.
- Output observable em produção inalterado.

---

## Sub-passos

### .A Auditoria de tests existentes

1. Inventariar tests existentes que cobrem o caminho:
   - `grep -rn "flat_counter_at\|equation" 01_core/src/`.
   - Tests P185B unitários para `flat_counter_at` em
     isolation (em `introspector.rs:mod tests`).
   - Tests P185D E2E sincronização Locator que tocam
     Equation (`gating_locator_apenas_em_locatables`
     ajustado em P186C).
   - Tests P186B/D/E unitários em ficheiros respectivos.

2. Identificar lacuna que P186F preenche:
   - Pipeline **completo** `walk → from_tags →
     flat_counter_at` para Equation com state injectado.
   - Sentinela do gate dormente em produção (sem state).
   - Validação de paridade legacy vs Introspector
     (similar a P184E para Figure).

3. Inventariar helpers disponíveis:
   - Construção `Content::Equation { body, block }` —
     estabelecida.
   - Pipeline `walk + from_tags + layout_with_introspector`
     — usado em P184E e P185D.
   - Injecção de state — usado em P185B test caso 4.

Output: tabela com tests existentes + cobertura + lacuna
identificada.

**Critério de saída**:
- Inventário completo. Helpers identificados.

### .B Test E2E pipeline com state injectado

1. Submódulo novo `p186f_equation_locatable` em
   `01_core/src/rules/layout/tests.rs` (irmão de
   `p184e_figure_per_kind` e `p185d_locator_sync`).

2. Test `pipeline_e2e_equation_block_com_state_activo`:
   - Construir documento com 3 equations block:
     `Content::Equation { body: Empty, block: true }`
     intercaladas com Text.
   - **Injectar state**: pré-popular
     `state.numbering_active:equation` antes do walk
     (ou usar mecanismo equivalente — verificar `.A`
     qual é a forma idiomática nos tests existentes).
   - Pipeline: `walk → from_tags → TagIntrospector`.
   - Asserções:
     - `intr.flat_counter_at("equation", loc(0))` =
       `Some(1)`.
     - `intr.flat_counter_at("equation", loc(1))` =
       `Some(2)`.
     - `intr.flat_counter_at("equation", loc(2))` =
       `Some(3)`.

**Critério de saída**:
- Test passa.
- Sequencialização do counter validada via pipeline
  real.

### .C Test sentinela gate dormente em produção

1. Test `gate_dormente_sem_state_active`:
   - Documento idêntico ao `.B`: 3 equations block.
   - **Sem injectar state** (estado de produção real).
   - Pipeline: `walk → from_tags → TagIntrospector`.
   - Asserções:
     - `intr.flat_counter_at("equation", loc(*))` =
       `None` para todas as locations.
     - Confirma que gate dormente é o caminho activo em
       produção.

2. Caso simétrico (mas opcional): equations inline
   (`block: false`):
   - Mesmo setup que `.B` (state activo) mas equations
     inline.
   - Asserções: counter **não** populado (gate exige
     `block`).

**Critério de saída**:
- Test passa.
- Gate dormente confirmado empiricamente.

### .D Test E2E paridade legacy vs Introspector

1. Test `paridade_equation_counter_legacy_vs_introspector`:
   - Documento idêntico aos anteriores.
   - **Path A**: legacy
     (`state.counter.get_flat("equation")` per consumer
     C2 actual em `equation.rs:97`).
   - **Path B**: Introspector populado via state
     injectado.
   - Comparar valores retornados.

2. Asserções:
   - Quando legacy retorna valor: Path B retorna mesmo
     valor (ou `None` se gate dormente).
   - Documenta empiricamente que os dois paths podem
     divergir em produção (legacy pode retornar valor
     enquanto Introspector retorna `None` com gate
     dormente).

**Critério de saída**:
- Test passa.
- Divergência documentada honestamente — em produção,
  legacy é o caminho funcional; Introspector é dormente.

### .E Test E2E sincronização Locator (extensão de P185D)

Opcional — se `.A` identificar que test P185D `.C`
precisa de extensão pós-P186 (Equation agora locatable):

1. Test `equation_locatable_avanca_locator_no_layouter`:
   - Documento: `[Heading, Equation_block, Cite]`.
   - Layouter avança Locator 3 vezes (cada um locatable).
   - Walk emite 3 tags.
   - Sequências de Locations idênticas.

2. Confirma que P186 não regrediu sincronização P185D.

**Critério de saída** (opcional):
- Test passa se incluído.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P186E
   baseline: +3 a +4 (3-4 tests E2E).
3. `crystalline-lint .` zero violations.
4. Tests `p186f_equation_locatable::*` passam isoladamente
   (`cargo test --workspace --lib p186f`).
5. Tests existentes não regridem.
6. Output observable em produção inalterado (P186F não
   toca produção).
7. Snapshot tests ADR-0033 verdes.
8. Linter passa final.

### .G Escrever relatório consolidado P186

1. Criar
   `00_nucleo/materialization/typst-passo-186-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P182F / P184F / P185E):

   - §1 Resumo executivo + pipeline final desbloqueio
     C2 eixo 2.
   - §2 Sub-passos materializados (tabela métricas A–F).
   - §3 Decisões arquiteturais (6 cláusulas P186A
     fechadas).
   - §4 Achados não-triviais durante execução:
     - P186A §11.2 — `Content::SetEquationNumbering`
       ausente; gate dormente em produção.
     - P186A §11.3 — `ElementKind::Equation` exigido.
     - P186A §11.5 — gate `block && state-active`
       não-trivial face a Figure.
     - P186B — `from_tags` exaustivo forçou stub no-op
       (cláusula gate trivial).
     - **P186C — descoberta empírica: walk gateia em
       `extract_payload`, não em `is_locatable`**
       (`introspect.rs:329`). Inversão de ordem
       sugerida (`extract_payload` antes de
       `is_locatable`) **não eliminou janela quebrada**;
       apenas inverteu sentido. Auditor resolveu
       pragmaticamente removendo Equation do fixture
       P185D durante janela (depois restaurou em P186D).
     - P186D — cláusula gate trivial em `from_tags`:
       spec restringia "não modificar from_tags" mas
       verificação `.F.8` exigia 4 locatables visíveis
       em `kind_index`. Auditor estendeu stub no-op
       com populate de `kind_index` (sem counter logic
       — esse fica para P186E). Decisão correcta dado
       inconsistência interna da spec.
     - P186D — fechou lacuna pré-existente em
       `build_minimal_for_each_variant` (helper de
       teste invariante não cobria Equation; outros
       variants podem ter mesma lacuna — fora de
       escopo).
     - P186E — decisão Opção B (gate location-aware
       inlined via `state.value_at()` para evitar
       circularidade estilística no `from_tags`).
   - §5 Estado final M9 (inalterado 11/11) e M5/M4
     (6/12 read-sites; eixo 2 de C2 resolvido
     estruturalmente; C2 ainda não migrado).
   - §6 Estado final lacunas (inalterado).
   - §7 Pendências cumulativas + janela compat M6:
     - DEBT M4-residual mantém C1+C2.
     - Trabalho futuro: `Content::SetEquationNumbering`
       (fora série actual).
   - §8 Próximos passos sugeridos:
     - P187 (migrar C1) — pode prosseguir.
     - P188 (migrar C2) — agora desbloqueado;
       documentar que migração resulta em Introspector
       path dormente em produção.
   - §9 Conclusão: M4-residual infraestrutural completo;
     migrações finais P187+P188 fecham fase.

2. Sem L0 novo; sem alteração de tests; sem ADR; sem
   DEBT novo.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- Dados consistentes com relatórios individuais P186A–E.

### .H Encerramento

P186F é o passo de encerramento. Após `.G` concluído, a
série P186 está formalmente fechada.

Estado projectado pós-P186F:

- **P186 série**: A ✅ B ✅ C ✅ D ✅ E ✅ **F ✅**.
  Fechada.
- **Eixo 2 do bloqueio P183C resolvido estruturalmente**.
- **Gate dormente em produção** documentado honestamente
  como design intencional.
- **C2 pronto para migrar em P188** (Introspector path
  estruturalmente disponível; fallback legacy ainda
  funcional em produção).
- M9: 11/11 (inalterado).
- M5/M4 progresso: 6/12 read-sites migrados (inalterado
  — C2 migra em P188).
- 53 passos executados.
- Padrão diagnóstico-primeiro: 11ª aplicação consecutiva
  (P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Test E2E pipeline com state injectado passa (`.B`).
3. Test sentinela gate dormente passa (`.C`).
4. Test E2E paridade legacy vs Introspector passa
   (`.D`).
5. (Opcional) Test sincronização Locator P185D ajustado
   passa (`.E`).
6. Tests existentes não regridem.
7. Verificações `.F` passam (8/8).
8. Relatório consolidado P186 (9 secções) escrito (`.G`).
9. Output observable em produção inalterado.

---

## O que pode sair errado

- **Helpers de injecção de state ausentes**: cláusula
  gate substancial. Recuar e investigar mecanismo de
  inject — provável que tests P185B unit tenham padrão
  reutilizável.
- **`flat_counter_at` retorna valor diferente do
  esperado**: indica que gate ou apply_at tem semântica
  divergente do previsto. Investigar.
- **Sentinela `.C` falha** (counter populado sem state):
  indica que gate em P186E não dispara correctamente.
  Cláusula gate substancial — recuar para P186E.
- **Paridade `.D` diverge**: divergência observable.
  Investigar — pode ser que legacy e Introspector tenham
  semântica subtilmente diferente.
- **Snapshot tests divergem**: improvável dado que P186F
  não toca produção. Se acontecer, indica que tests de
  produção estão a usar estado novo indirectamente.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 LOC tests + ~150 LOC
  relatório consolidado.
- **Sem código de produção tocado** — tests + docs apenas.
- **Sem dependências externas novas**.
- **Padrão replicado**: P184E (tests E2E paridade) +
  P181J/P182F/P184F/P185E (relatório consolidado).
- **Cláusula gate trivial**: aplicável a forma de
  injection de state, formato de tests.
- **Cláusula gate substancial**: aplicável apenas se
  helpers ausentes ou se gate dormente não disparar
  como esperado.
- **Inversão observable diferente de P184**: P184D
  (Figure) fechou C3 com Introspector como caminho
  funcional. P186 (Equation) fecha eixo 2 mas
  Introspector fica **dormente em produção** —
  fallback legacy é o caminho funcional permanente até
  `SetEquationNumbering` ser materializado. Diferença
  importante registada honestamente.
- **Fim da série P186**: M4-residual infraestrutural
  completo. P187 (C1) e P188 (C2) são migrações finais
  que fecham M4-residual. Após eles, DEBT M4-residual
  fecha; segue M5 (P189).
