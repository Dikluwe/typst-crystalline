# Passo P190H — Categoria Figures (2ª aplicação ADR-0071 em P190)

Sétimo passo de implementação P190 (após P190A-G +
ramo paralelo P191A-C). Magnitude **M** — 2ª
aplicação directa do mecanismo ADR-0071 em série
P190.

P190G confirmou empiricamente:
- 1ª aplicação directa ADR-0071 funcional.
- Padrão "eliminação dos fallbacks
  substitution-with-fallback" emergente.
- Threading via parameter `&mut usize` correcto
  para state recursion.
- Cleanup defer simultâneo durante categoria
  funciona (defer `numbering_active` resolvido em
  P190G via Caso 1).
- LOC líquido cumulativo desde P191A: -614.
- 30ª aplicação diagnóstico-primeiro consecutiva.

P190H trata categoria Figures — 3 fields:
- **`figure_numbers`** — `HashMap<String, usize>`.
  Caminho Introspector activo desde P185B
  (`figure_number_at_index` location-aware).
- **`figure_label_numbers`** — figure_label_numbers
  store; caminho Introspector activo (per histórico
  M5).
- **`local_figure_counters`** — walk-internal;
  possivelmente eliminável como local var ou
  parameter.

Trabalho derivado:
- **Resolver defer `lang`** se walk arm Labelled
  deixar de ler `state.lang` após Figure migration.
  Per P191C Opção β: walk arm Labelled passa
  `state.lang` ao helper. Após eliminação categoria
  Figures, walk arm Labelled pode deixar de ter
  fonte para `lang` — investigar.

Trabalho concreto (categoria 7 Figures):
1. **Eliminar mutações walk arm**:
   - Walk arm Figure: `state.figure_numbers.entry(...)
     += 1` (write paralelo M5).
   - Walk arm Figure:
     `state.figure_label_numbers.insert(...)`.
   - Walk arm Figure: `state.local_figure_counters`
     mutação (walk-internal).
2. **Eliminar 2-3 fields**:
   - `figure_numbers` (sempre).
   - `figure_label_numbers` (sempre).
   - `local_figure_counters` per `.D` (Opção α
     local var/parameter; Opção β preservar até
     P190I).
3. **Eliminar Layouter consumers/assignments**
   correspondentes.
4. **Eliminar fallbacks substitution-with-fallback**
   da categoria (per padrão P190G §5.4).
5. **Resolver defer `lang` se aplicável** (per
   `.A.10`).

Após P190H:
- `CounterStateLegacy`: 6 → **3-4 fields** (per
  Opção α/β + resolução defer `lang`).
- 2-3 fields eliminados.
- Possível resolução defer `lang`.
- Pattern "eliminação write paralelo M5": 7ª
  aplicação concreta.
- 2ª aplicação directa ADR-0071 mecanismo em P190.

**Pré-condição**: P190G concluído. Tests workspace
1.812 verdes; zero violations. `CounterStateLegacy`
6 fields (resolved_labels, headings_for_toc,
auto_label_counter, numbering_active eliminados).

**Restrições**:
- **Não** modificar walk fn signature (já tem 7
  parameters).
- **Não** modificar trait `Introspector`.
- **Não** modificar `TagIntrospector` struct
  fields.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar 2 helpers walk-internal
  (`compute_figure`, `compute_heading_for_toc`)
  excepto se necessário para parameter passing.
- **Modificar Layouter** se consumers desta
  categoria existirem (per P190G §5.4 padrão).
- **Não** materializar lacunas residuais.
- API pública preservada.
- **Lembrete crítico**: 1 sub-passo restante
  (P190I) após P190H. M6 fecha em P190I.

---

## Sub-passos

### .A Auditoria L0

#### Inventário 3 fields (categoria 7)

1. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub figure_numbers: HashMap<String, usize>`.
   - `pub figure_label_numbers: HashMap<...>` —
     type confirmar.
   - `pub local_figure_counters: HashMap<...>` —
     type confirmar.

#### Inventário walk arm mutações

2. Identificar mutações em walk arm Figure:
   - `grep -n "figure_numbers\|figure_label_numbers\|local_figure_counters"
     01_core/src/rules/introspect.rs`.

3. Confirmar walk arm Figure (per histórico M5):
   - Mutação 1: `state.figure_numbers.entry(...)`.
   - Mutação 2: `state.figure_label_numbers.insert(...)`.
   - Mutação 3: `state.local_figure_counters` (walk-internal).

#### Inventário walk readers (DURANTE walk)

4. Identificar walk readers de `state.figure_numbers`:
   - **Esperado**: nenhum após P191C (helper
     `compute_labelled` migrado para
     `figure_number_at_index` location-aware).
   - Verificar empíricamente.

5. Identificar walk readers de `state.figure_label_numbers`:
   - **Esperado**: nenhum.
   - Verificar empíricamente.

6. Identificar walk readers de `state.local_figure_counters`:
   - **Esperado**: walk arm Figure interno;
     possivelmente helpers walk-internal
     (`compute_figure` P197B).
   - **Cláusula gate substancial**: signature de
     `compute_figure` precisa adaptação para
     receber `local_figure_counters` como parameter
     se Opção α escolhida.

#### Inventário Layouter consumers + fallbacks

7. Identificar consumers Layouter:
   - `grep -rn "self.counter.figure_numbers\|self.counter.figure_label_numbers\|self.counter.local_figure_counters"
     01_core/src/`.
   - Possíveis fallbacks substitution-with-fallback
     em consumers.

8. Identificar Layouter assignments duais:
   - `grep -rn "figure_numbers\s*=\|figure_label_numbers\s*=\|local_figure_counters\s*="
     01_core/src/rules/layout/mod.rs`.

#### Inventário cleanup defer `lang`

9. Confirmar walk arm Labelled signature após
   P191C:
   - Walk arm Labelled passa `state.lang` ao
     helper `compute_labelled` (per P191C Opção β).
   - **Cláusula gate substancial**: walk arm
     Labelled é único leitor de `state.lang`?
     - Se **sim**: defer `lang` resolvível em P190H
       — eliminar mutação Labelled (passar `lang`
       de outra fonte) + eliminar field.
     - Se **não**: defer continua para P190I.
   - `grep -n "state.lang\|state\.lang" 01_core/src/rules/introspect.rs`.

10. **Decisão obrigatória empírica em `.H`** (mais
    abaixo): defer `lang` resolvível agora ou não.

#### Decisão `local_figure_counters`

11. **Decisão obrigatória `.D`**: 3 opções:
    - **Opção α** (local var / parameter): walk arm
      Figure usa local var ou walk fn ganha 8º
      parameter.
    - **Opção β** (preservar até P190I): defer.
    - **Opção γ** (mover para LayouterRuntimeState):
      improvável.

#### Tests dependentes

12. Identificar tests:
    - Tests sentinela mutação legacy
      `walk_arm_figure_*_via_legacy` ou similar.
    - Tests Layouter Figure (preservar).
    - Tests `local_figure_counters` directos.

#### L0 alvos

13. Identificar L0s:
    - `entities/counter_state_legacy.md` (defer
      P190I).
    - `rules/introspect.md` (walk arm Figure
      purificada).
    - Possivelmente outros.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 3 fields localizados.
- Mutações walk arm catalogadas.
- Walk readers confirmados.
- Layouter consumers + fallbacks identificados.
- Defer `lang` estado avaliado.
- Decisão `local_figure_counters` (`.D`)
  preparada.
- Tests dependentes listados.

### .B Eliminar mutação `state.figure_numbers`

Per `.A.2` e `.A.4`:

1. Localizar mutação em walk arm Figure.

2. **Cláusula gate trivial**: confirmar que walk
   arm Figure emite Tag::Figure que popula
   `intr.figure_numbers` ou equivalente via
   populate_intr (per P191B).

3. Eliminar `state.figure_numbers.entry(...)`
   mutação.

4. Comentário inline P190H substitui ou actualiza.

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Mutação eliminada.

### .C Eliminar mutação `state.figure_label_numbers`

Per `.A.2` e `.A.5`:

1. Localizar mutação em walk arm Figure.

2. **Cláusula gate trivial**: confirmar populate_intr
   arm Figure popula `intr.figure_label_numbers`
   (ou equivalente sub-store) per histórico M5.

3. Eliminar mutação.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Mutação eliminada.

### .D Decisão `local_figure_counters` (Opção α/β/γ)

Per `.A.6` e `.A.11`:

**Decisão obrigatória empírica**.

Sugestão preliminar: **Opção α** se walk arm
Figure + helpers walk-internal aceitam parameter;
**Opção β** se signature rígida.

Output: opção materializada.

### .E Eliminar/migrar `local_figure_counters` per `.D`

**Per Opção α**:

1. Walk fn ganha 8º parameter
   `local_figure_counters: &mut HashMap<...>` ou
   walk arm Figure usa local var.

2. Helpers walk-internal (`compute_figure`)
   adaptam signature para receber parameter.

3. Mutação `state.local_figure_counters` eliminada
   ou substituída por mutação no parameter/local
   var.

4. Confirmar `cargo check --workspace` passa.

**Per Opção β**: nenhum trabalho neste sub-passo;
defer P190I.

**Critério de saída**:
- Per `.D` decisão materializada.

### .F Eliminar 2-3 fields (per `.D`)

Per `.A.1`:

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar `pub figure_numbers`.
   - Eliminar `pub figure_label_numbers`.
   - **Per Opção α `.D`**: eliminar
     `pub local_figure_counters`.
   - **Per Opção β `.D`**: preservar
     `local_figure_counters`.

2. Adaptar `Default` impl ou `new()`.

3. Confirmar `cargo check --workspace` passa.

4. **`CounterStateLegacy`: 6 → 3 fields** (Opção
   α) ou **4 fields** (Opção β).

**Critério de saída**:
- Fields eliminados per decisão.

### .G Eliminar Layouter consumers + assignments + fallbacks

Per `.A.7` e `.A.8`:

1. Eliminar consumers Layouter (per `.A.7`):
   - Substituir `self.counter.figure_*` por
     queries Introspector path.
   - Eliminar fallbacks
     substitution-with-fallback (padrão P190G
     §5.4).

2. Eliminar assignments duais (per `.A.8`).

3. Comentário inline P190H actualiza.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Layouter migrações completas.
- Fallbacks eliminados.

### .H Resolver defer `lang` (se aplicável)

Per `.A.9` e `.A.10`:

**Caso 1**: walk arm Labelled é único leitor de
`state.lang`:
- Eliminar mutação que set `state.lang`.
- Eliminar field `lang` de `CounterStateLegacy`.
- Walk arm Labelled passa `lang` de outra fonte
  (parameter walk fn, config externa, etc.).
- **`CounterStateLegacy`: 3 → 2 fields** (Opção α
  + Caso 1) ou **3** (outras combinações).

**Caso 2**: outros readers existem:
- Defer `lang` continua para P190I.

**Decisão obrigatória empírica**.

Output: caso materializado.

### .I Adaptar tests

1. Identificar tests afectados (per `.A.12`):
   - Tests sentinela mutação legacy redundantes.
   - Tests `local_figure_counters` directos.
   - Tests Layouter Figure (preservar).

2. Adaptação:
   - Tests redundantes — remover.
   - Tests adaptáveis — substituir.

3. Tests workspace verdes (Δ esperado: marginal
   negativo).

**Critério de saída**:
- Tests adaptados.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P190G baseline (1.812): **marginal**.
3. `crystalline-lint .` zero violations.
4. `CounterStateLegacy.figure_numbers` **NÃO
   existe**.
5. `CounterStateLegacy.figure_label_numbers` **NÃO
   existe**.
6. **Per Opção α `.D`**:
   - `CounterStateLegacy.local_figure_counters`
     **NÃO existe**.
   - Walk fn 8 parameters ou local var.
   - Helper `compute_figure` adaptado.
7. **Per Opção β `.D`**: field preservado.
8. **Per Caso 1 `.H`**:
   - `CounterStateLegacy.lang` **NÃO existe**.
   - Walk arm Labelled `lang` source migrado.
9. **Per Caso 2 `.H`**: defer `lang` continua.
10. `CounterStateLegacy`: **2-4 fields** (depende
    decisões).
11. Walk arm Figure mutações eliminadas.
12. Layouter consumers Figure migrados.
13. Layouter fallbacks Figure eliminados.
14. Layouter assignments duais eliminados.
15. Comentários inline P190H presentes.
16. Trait `Introspector` **NÃO modificado**.
17. `TagIntrospector` fields **NÃO modificados**.
18. ADR-0070 PROPOSTO **NÃO transitada** (ACEITE
    em P190I).
19. Snapshot tests verdes.
20. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190h-relatorio.md`
com:

- Resumo: categoria 7 (Figures) eliminada; 2-3
  fields eliminados; possível resolução defer
  `lang`; 2ª aplicação directa ADR-0071 mecanismo
  em P190.
- Confirmação `.J` (20 verificações).
- Δ tests vs baseline P190G.
- Hashes finais.
- Decisões de execução notáveis:
  - Opção α/β em `.D` (`local_figure_counters`).
  - Caso 1/Caso 2 em `.H` (defer `lang`).
  - Padrão "2ª aplicação directa ADR-0071" no P190.
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️
    G ✅ H ✅ | I pendente.
  - **Categoria 7 (Figures) fechada**.
  - 96 passos executados.
- Pendências cumulativas: 1 sub-passo restante
  (P190I) + (eventualmente) defers remanescentes
  (`flat`, `hierarchical`, possivelmente `lang`,
  possivelmente `local_figure_counters`).
- Próximo passo: **P190I** — Walk arms purification
  + Layouter final + struct elim + ADR-0070
  ACEITE. Magnitude M+. **Fecha M6**.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperado.
2. Mutação `figure_numbers` eliminada (`.B`).
3. Mutação `figure_label_numbers` eliminada (`.C`).
4. Decisão `.D` materializada
   (`local_figure_counters`).
5. `local_figure_counters` eliminado/migrado per
   `.D` (`.E`).
6. 2-3 fields eliminados (`.F`).
7. Layouter consumers + fallbacks + assignments
   eliminados (`.G`).
8. Decisão `.H` materializada (defer `lang`).
9. Tests adaptados (`.I`).
10. Verificações `.J` passam (20/20).
11. Output observable em produção inalterado.
12. **2ª aplicação directa ADR-0071 mecanismo em
    P190 estabelecida**.
13. Relatório `.K` escrito.

---

## O que pode sair errado

- **`compute_figure` signature rígida — não aceita
  parameter `local_figure_counters`**: cláusula
  gate substancial — Opção β em `.D` (defer).
- **Walk arm Figure `is_counted` gate ainda
  depende de legacy state**: cláusula gate
  substancial — investigar pattern P191C
  populate_intr Figure.
- **Layouter consumers Figure em sítios não
  previstos**: cláusula gate trivial.
- **Tests sentinela em quantidade significativa
  regridem**: padrão pragmático auditor #1.
- **Figure_label_numbers tem semântica
  incompatível com Introspector path**: cláusula
  gate substancial — investigar.
- **`state.lang` lido em sítio não previsto**
  (e.g., outro arm): cláusula gate substancial —
  Caso 2 obrigatório em `.H`.
- **Snapshot tests divergem**: improvável.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M. ~30-50 LOC produção (mutações
  eliminadas + local_figure_counters refactor +
  fields eliminados + Layouter migrations) + ~20
  LOC tests adaptados + ~20 LOC L0 (defer P190I).
- **Sem dependências externas novas**.
- **Sem ADR nova** (ADR-0070 PROPOSTO; ACEITE em
  P190I).
- **2ª aplicação directa ADR-0071 mecanismo em
  P190**.
- **Pattern "eliminação write paralelo M5"**: 7ª
  aplicação concreta.
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**:
  signature rígida helpers; gate Figure `is_counted`;
  state.lang em sítios não previstos.
- **Próximo passo P190I**: **PASSO FINAL M6** —
  Walk arms purification + Layouter final + struct
  elim + ADR-0070 ACEITE. Magnitude M+. Trabalho
  concreto:
  - Eliminar `CounterStateLegacy` struct
    completamente.
  - Layouter ganha `counter` field eliminado (F3
    parcialmente fecha).
  - L0 update final.
  - ADR-0070 PROPOSTO → ACEITE.
  - F1 fecha.
  - **M6 fechado**.
- **F1 progresso**: 6 → 2-4 fields (depende
  decisões). Faltam 2-4 fields para eliminação
  total em P190I.
- **F3 progresso**: Layouter ainda 20 fields;
  inalterado em P190H (struct elim em P190I).
- **Lembrete crítico**: P190I é passo final série.
  Após P190H, série fecha em 1 passo final.
