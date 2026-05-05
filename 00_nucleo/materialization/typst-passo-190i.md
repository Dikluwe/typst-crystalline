# Passo P190I — PASSO FINAL M6 + ADR-0070 ACEITE

**Marco arquitectural**: passo final da série P190
e fim do M6. Magnitude **M+** — eliminação struct
completa + Layouter refactor + L0 final + ADR-0070
PROPOSTO → ACEITE + relatório consolidado P190.

P190H confirmou empiricamente:
- 7ª aplicação write paralelo M5 (P190B-H).
- 2 aplicações directas ADR-0071 em P190 (G, H).
- Helper `compute_figure` eliminado (orphan).
- Walk arm Figure puro.
- `CounterStateLegacy`: 3 fields restantes (`flat`,
  `hierarchical`, `lang`).
- 31ª aplicação diagnóstico-primeiro consecutiva.

P190I elimina **completamente** `CounterStateLegacy`:
- 3 fields restantes (`flat`, `hierarchical`,
  `lang`).
- Struct + impl + Default.
- Layouter `counter` field.
- Walk fn `state` parameter.
- Defer `lang` resolvido aqui.

Trabalho concreto:
1. **Eliminar walk fn `state` parameter**:
   - 9 parameters → 8 (drop `state: &mut
     CounterStateLegacy`).
   - Adaptar todos os call sites.
2. **Migrar 3 fields restantes**:
   - `flat`, `hierarchical`: eliminar (counters
     já em CounterRegistry via populate_intr).
   - `lang`: eliminar — walk arm Labelled passa
     `lang` de outra fonte (config, parameter walk
     fn novo, ou via `intr` se sub-store dedicado
     existir).
3. **Eliminar struct `CounterStateLegacy`**:
   - Ficheiro `entities/counter_state_legacy.rs`
     eliminado ou esvaziado.
   - Imports limpos.
4. **Layouter `counter` field eliminado**:
   - `Layouter<M, S>::counter: CounterStateLegacy`
     removido.
   - 19 → 18 fields (delta -1).
   - F3 parcialmente fecha.
5. **L0 final**:
   - `entities/counter_state_legacy.md` eliminado.
   - `entities/mod.md` actualizado.
   - `rules/introspect.md` actualizado.
   - Histórico M6 documentado.
6. **ADR-0070 PROPOSTO → ACEITE**:
   - Validação empírica registada.
   - Pattern stylesheet "eliminação write paralelo
     M5" 8 aplicações concretas (B-I).
7. **Relatório consolidado P190**:
   - 9 secções padrão + secção dedicada ao marco
     M6 fechado.

Após P190I:
- `CounterStateLegacy`: 3 → **0 fields** (struct
  eliminado).
- Layouter: 20 → 19 fields (-1; field `counter`
  eliminado).
- Walk fn signature: 9 → 8 parameters.
- Pattern "eliminação write paralelo M5": **8ª e
  final aplicação**.
- 3ª aplicação directa ADR-0071 em P190.
- **F1 fecha** (CounterStateLegacy eliminado).
- **F3 parcialmente fecha** (Layouter -1 field;
  outros 18 fields ortogonais ficam para refactor
  futuro).
- **M6 fechado**.
- ADR-0070 ACEITE.
- LayouterRuntimeState 3 fields (inalterado).
- Tests workspace verdes.

**Pré-condição**: P190H concluído. Tests workspace
1.812 verdes; zero violations. `CounterStateLegacy`
3 fields. ADR-0070 PROPOSTO ainda activa. Walk fn
9 parameters.

**Restrições**:
- **Não** modificar trait `Introspector` (estável
  20 métodos).
- **Não** modificar `TagIntrospector` struct
  fields (estável 9 sub-stores).
- **Não** modificar 1 helper walk-internal restante
  (`compute_heading_for_toc`) excepto signature
  adaptation se necessário.
- **Não** modificar `LayouterRuntimeState`.
- **Não** modificar Layouter outros 18 fields
  (apenas eliminar `counter`).
- **Não** materializar lacunas residuais.
- **Não** modificar pipeline walk arquitectural
  (preservar mecanismo ADR-0071).
- API pública preservada — `introspect()` retorna
  apenas `TagIntrospector` agora (em vez de
  `(CounterStateLegacy, TagIntrospector)`).
  - **Cláusula gate substancial**: API pública
    breaking change. Verificar empiricamente se
    callers existem fora do crate.

---

## Sub-passos

### .A Auditoria L0 final

#### Inventário 3 fields restantes

1. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `flat: HashMap<String, usize>` (privado).
   - `hierarchical: HashMap<String, Vec<usize>>`
     (privado).
   - `pub lang: ...` (público).

#### Inventário walk fn `state` parameter

2. Confirmar walk fn signature actual em
   `01_core/src/rules/introspect.rs`:
   - 9 parameters esperados (per histórico):
     - `content: &Content`.
     - `state: &mut CounterStateLegacy`.
     - `locator: &mut Locator`.
     - `tags: &mut Vec<Tag>`.
     - `intr: &mut TagIntrospector`.
     - `auto_label_counter: &mut usize`.
     - `label_from_parent: Option<&Label>`.
     - (eventualmente outros).

3. Identificar todas as recursive walk call sites:
   - `grep -n "walk(" 01_core/src/rules/introspect.rs`.
   - Esperado: ~25 chamadas.

#### Inventário walk arm leitores `state`

4. Identificar walk arms que ainda lêem `state`:
   - Walk arm Labelled lê `state.lang` (per P190H
     Caso 2).
   - Outros walk arms — esperado: nenhum.
   - Verificar empíricamente.

5. Identificar mutações de `state` ainda existentes:
   - Walk arm que set `state.lang` (se existir).
   - Outras — esperado: nenhuma.

#### Inventário Layouter `counter` field

6. Confirmar Layouter struct:
   - `pub counter: CounterStateLegacy` ou similar.
   - Localização exacta.
   - Inicialização.

7. Identificar **todos os usos** de
   `self.counter` em Layouter:
   - `grep -rn "self.counter" 01_core/src/rules/layout/`.
   - Esperado: zero ou apenas usos para `lang`.

8. Identificar Layouter assignments para `counter`:
   - `grep -rn "counter\s*=" 01_core/src/rules/layout/mod.rs`.

#### Inventário API pública (cláusula gate substancial)

9. Confirmar API pública:
   - `pub fn introspect(...) -> ...` em qual lib.rs?
   - Re-exports de `CounterStateLegacy`?
   - `grep -rn "CounterStateLegacy" 01_core/src/lib.rs`.
   - Callers externos via `02_shell/`, `03_infra/`,
     `04_wiring/`?

10. **Decisão obrigatória cláusula gate**:
    - **Opção α**: API pública preservada — manter
      retorno `(CounterStateLegacy, TagIntrospector)`
      mas com `CounterStateLegacy` reduzida a struct
      vazia/marker.
    - **Opção β**: API breaking change — retorno
      apenas `TagIntrospector`. Adaptar callers
      externos.
    - **Opção γ**: criar type alias temporário.

    Sugestão: depende do inventário em `.A.9`.

#### Resolução defer `lang`

11. Decisão sobre `lang`:
    - **Opção α**: walk fn ganha 9º parameter
      `lang: Option<&Lang>`.
    - **Opção β**: walk arm Labelled lê `lang` via
      sub-store dedicado em TagIntrospector
      (over-engineering — improvável).
    - **Opção γ**: passar `lang` via context
      Engine durante walk (se Engine acessível).
    - **Opção δ**: walk fn ganha context struct
      pequeno em vez de parameter avulso.

    Sugestão preliminar: **Opção α** — incremental,
    consistente com `auto_label_counter` parameter
    P190G.

#### Tests dependentes

12. Identificar tests:
    - Tests `CounterStateLegacy` directos —
      todos redundantes; remover.
    - Tests `Layouter::new()` ou similar — adaptar.
    - Tests `introspect()` API — adaptar per
      decisão `.A.10`.

#### L0 alvos

13. Identificar L0s:
    - `entities/counter_state_legacy.md` —
      eliminar ou marcar como deprecated.
    - `entities/mod.md` — actualizar.
    - `rules/introspect.md` — actualizar walk fn
      signature.
    - `rules/layout/mod.md` — actualizar Layouter
      struct.
    - **Histórico M6**: secção dedicada em L0
      master sobre eliminação completa.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 3 fields localizados.
- Walk fn signature confirmada.
- Layouter `counter` field localizado.
- API pública analisada (cláusula gate substancial
  resolvida em `.A.10`).
- Defer `lang` resolução decidida em `.A.11`.
- Tests dependentes catalogados.

### .B Resolver `lang` per `.A.11`

**Per Opção α** (walk fn ganha parameter):

1. Em walk fn:
   - Adicionar parameter `lang: Option<&Lang>` (9º
     parameter; substitui `state` que é eliminado em
     `.D`).
   - Walk arms passam `lang` recursivamente.

2. Walk arm Labelled lê `lang` via parameter em vez
   de `state.lang`.

3. Caller externo (entry point) passa `lang` da
   config.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `lang` resolvido per `.A.11`.

### .C Eliminar walk fn `state` parameter

Per `.A.2` e `.A.3`:

1. Eliminar parameter `state: &mut
   CounterStateLegacy` da walk fn signature.

2. Actualizar **todas** as ~25 recursive call
   sites:
   - Remover `state` argument.
   - Sed mecânico (precaução com side effects).

3. Eliminar passagens de `state` em helpers que
   ainda recebem (se algum):
   - `compute_heading_for_toc` (per P190G adaptado;
     já não recebe state — parameter
     `auto_label_counter`).
   - Confirmar empíricamente.

4. **Cláusula gate substancial**: walk fn ainda
   chama métodos em `state` (per `.A.4`)?
   - Se sim: estes leitores precisam de migração
     antes (per `.B`).

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Walk fn 8 parameters.
- Recursive call sites actualizados.

### .D Eliminar struct `CounterStateLegacy`

Per `.A.1`:

1. **Decisão crítica per `.A.10`**:
   - **Opção α** (manter struct vazia): preservar
     `pub struct CounterStateLegacy;` como marker
     (struct vazia) para compatibilidade API.
   - **Opção β** (eliminar completamente): remover
     ficheiro
     `01_core/src/entities/counter_state_legacy.rs`
     + imports + re-exports.

2. Per Opção β:
   - Eliminar ficheiro.
   - Remover `pub mod counter_state_legacy;` em
     `entities/mod.rs`.
   - Remover `pub use ...` correspondentes.
   - Adaptar callers externos (se existirem per
     `.A.9`).

3. Per Opção α:
   - Esvaziar struct (3 fields removidos; struct
     fica `pub struct CounterStateLegacy;`).
   - Adaptar `Default` impl.
   - Adicionar comentário `#[deprecated]` ou TODO
     para eliminação total futura.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Struct eliminado per decisão.

### .E Eliminar Layouter `counter` field

Per `.A.6`-`.A.8`:

1. Em Layouter struct:
   - Eliminar `pub counter: CounterStateLegacy`.

2. Eliminar inicialização `counter` em
   `Layouter::new()` ou construtor.

3. Confirmar `self.counter` não usado em qualquer
   sítio (per `.A.7` esperado zero).

4. **Layouter: 20 → 19 fields**.

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Field eliminado.
- Layouter -1 field.

### .F API pública per `.A.10`

**Per Opção α** (preservar API com struct vazia):

1. `pub fn introspect(...) -> (CounterStateLegacy,
   TagIntrospector)` continua a retornar
   tuple, mas `CounterStateLegacy` é struct vazia.

**Per Opção β** (breaking change):

1. `pub fn introspect(...) -> TagIntrospector`
   retorno simplificado.

2. Adaptar callers externos (per `.A.9`).

3. **Cláusula gate substancial**: confirmar
   empíricamente que callers externos foram
   adaptados ou não existem.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- API per decisão.

### .G Adaptar tests

1. Identificar tests afectados (per `.A.12`):
   - Tests `CounterStateLegacy` directos — remover.
   - Tests Layouter `new()` — adaptar.
   - Tests `introspect()` API — adaptar.

2. Adaptação:
   - Tests redundantes — remover.
   - Tests adaptáveis — substituir.

3. **Adicionar 1-2 tests sentinela final**:
   - Test "M6 — CounterStateLegacy eliminada":
     compile-time check via `not(impls)` ou
     similar.
   - Test "Layouter sem counter field": verificar
     struct membership.

4. Tests workspace verdes (Δ esperado: marginal
   negativo — sentinelas legacy redundantes
   removidas).

**Critério de saída**:
- Tests adaptados.
- Tests sentinela M6 fechado adicionados.

### .H L0 final

1. Eliminar
   `00_nucleo/prompts/entities/counter_state_legacy.md`
   ou marcar como histórico arquivístico.

2. Actualizar
   `00_nucleo/prompts/entities/mod.md`:
   - Remover entrada `counter_state_legacy`.

3. Actualizar
   `00_nucleo/prompts/rules/introspect.md`:
   - Walk fn signature 8 parameters.
   - **Marco arquitectural** "M6 — CounterStateLegacy
     eliminada" registado formalmente.
   - Cross-reference: P190 série + ADR-0070
     ACEITE + ADR-0071 ACEITE.

4. Actualizar
   `00_nucleo/prompts/rules/layout/mod.md` (se
   existir):
   - Layouter 19 fields.
   - Field `counter` eliminado.

5. Hash em branco aguarda recálculo manual em
   `.J`.

**Critério de saída**:
- L0s actualizados.
- Marco M6 fechado registado.

### .I ADR-0070 PROPOSTO → ACEITE

1. Editar
   `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md`:
   - Estado: PROPOSTO → **ACEITE**.
   - Validação empírica registada:
     - 8 sub-passos B-I executados.
     - Pattern "eliminação write paralelo M5" 8
       aplicações concretas.
     - 2 padrões complementares estabelecidos
       ("Layouter-runtime → struct dedicada";
       "eliminação directa via Introspector").
     - Pattern ADR-0071 (P191) usado em P190G/H/I
       como pré-condição.
     - F1 fechado.
     - F3 parcialmente fechado.
     - LOC líquido cumulativo: -X (calcular
       empíricamente).
     - Tests workspace verdes.

2. Cross-references actualizadas:
   - P190A (PROPOSTO).
   - P190B-H (aplicações incrementais).
   - P191A-C (ramo paralelo ADR-0071 ACEITE).
   - P190I (ACEITE).

**Critério de saída**:
- ADR-0070 ACEITE.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P190H baseline (1.812): **marginal**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. **`CounterStateLegacy` eliminado**.
5. **Layouter sem field `counter`** (-1 field; 20
   → 19 fields).
6. **Walk fn 8 parameters** (era 9).
7. ~25 recursive call sites actualizados.
8. Defer `lang` resolvido per `.B`.
9. Layouter `self.counter` zero usos.
10. **F1 fecha**.
11. **F3 parcialmente fecha** (Layouter -1 field).
12. **ADR-0070 ACEITE** (transitada per `.I`).
13. **L0s actualizados** com marco M6 fechado.
14. Trait `Introspector` **NÃO modificado** (20
    métodos).
15. `TagIntrospector` **NÃO modificado** (9
    sub-stores).
16. `LayouterRuntimeState` **NÃO modificado** (3
    fields).
17. ADR-0071 **NÃO modificada** (ACEITE em P191C).
18. Comentários inline P190I presentes.
19. Tests sentinela M6 fechado adicionados (per
    `.G`).
20. Snapshot tests verdes.
21. Linter passa final.
22. **M6 fechado completamente**.

### .K Relatório consolidado P190

Criar
`00_nucleo/materialization/typst-passo-190-relatorio-consolidado.md`
com 9 secções padrão + secção 10 dedicada ao marco
M6 fechado:

- **§1 Resumo executivo**: M6 fechado pela primeira
  vez desde declaração em P185A; CounterStateLegacy
  eliminado completamente; F1 fecha; F3
  parcialmente fecha; pattern "eliminação write
  paralelo M5" 8 aplicações concretas; ADR-0070
  ACEITE.

- **§2 Sub-passos materializados**: tabela métricas
  A-I (magnitudes planeadas vs reais, Δ tests, L0s
  tocados, fields eliminados).

- **§3 Decisões arquitecturais**: 9 cláusulas
  P190A fechadas + decisões empíricas em cada
  sub-passo B-I + decisão ramo paralelo P191.

- **§4 Achados não-triviais**:
  - Achado P190F (barreira walk pipeline).
  - Achado P191B (Locations monotónicas garantem
    ordering).
  - Achado P191C (divergência latente Figure
    is_counted).
  - Achado P190G (eliminação dos fallbacks
    substitution-with-fallback).
  - Achado P190H (helpers podem ficar orphan).
  - Achado P190I (`lang` resolução; API pública
    decisão; possível struct vazia preservada).

- **§5 Estado activo vs preservado**:
  - Activado em P190 (categoria por categoria).
  - Preservado: ADR-0069, ADR-0071, helpers
    walk-internal, etc.

- **§6 Estado final M9, M5, M6**:
  - M9: 11/11 (inalterado).
  - M5: COMPLETO (P200B).
  - **M6: COMPLETO** (P190I — pela primeira vez
    desde declaração).
  - Fields eliminados: tabela 16 → 0 fields.
  - Helpers eliminados: 1 (`compute_figure`).
  - Helpers walk-internal preservados: 1
    (`compute_heading_for_toc`).
  - 2 helpers ADR-0069 migrados para Introspector
    path (`compute_labelled`, `compute_heading_auto_toc`).

- **§7 Estado final lacunas**:
  - Lacunas residuais (#1, #1b, #2): inalteradas.
  - Não impactam M6 fechado.

- **§8 Pendências cumulativas + marco M6 fechado**:
  - 4 defers acumulados resolvidos.
  - DEBT M6 documentação fechado por execução.
  - F1 fechado.
  - F3 parcialmente fechado.

- **§9 Próximos passos sugeridos**:
  - **M7** (loop fixpoint).
  - **M8** (memoização comemo).
  - **F3 completo** — refactor Layouter restantes
    18 fields.
  - **Lacunas residuais** (#1, #1b, #2) — passos
    dedicados.

- **§10 Marco arquitectural — M6 fechado completo**:
  - Histórico: P190A (PROPOSTO) → P190B-H
    (aplicações incrementais) → P191A-C (ramo
    paralelo ADR-0071) → P190I (ACEITE).
  - 8 sub-passos materializados em P190 + 3 em
    P191 = 11 sub-passos para fechar M6.
  - 8 aplicações concretas pattern "eliminação
    write paralelo M5".
  - 2 padrões complementares estabelecidos.
  - 4 defers acumulados resolvidos.
  - LOC líquido cumulativo: ~-1500 ou similar
    (calcular empíricamente).
  - 31+ aplicações diagnóstico-primeiro
    consecutivas.
  - **Marco arquitectural significativo**: F1
    fechado.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções + §10 marco.

### .L Encerramento

P190I é o passo final da série P190 e fim do M6.
Após `.K` concluído, M6 está fechado.

Estado projectado pós-P190I:

- **P190 série**: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️
  G ✅ H ✅ I ✅ — fechada.
- **M6 fechado completo**.
- **F1 fechado**.
- **F3 parcialmente fechado**.
- **CounterStateLegacy eliminado**.
- **ADR-0070 ACEITE**.
- **ADR-0071 ACEITE** (P191C).
- **97 passos executados** (P190H=96 + P190I=97).
- **Padrão diagnóstico-primeiro**: 32ª aplicação
  consecutiva.
- **8 aplicações pattern "eliminação write paralelo
  M5"**.
- **Tests workspace**: verdes (Δ marginal vs
  baseline).

**Próximas decisões estratégicas**:
- M7 (loop fixpoint).
- M8 (memoização comemo).
- F3 completo (Layouter restantes 18 fields).
- Lacunas residuais (#1, #1b, #2).
- Pausa estratégica.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria final sem disparar gate
   substancial inesperado.
2. `lang` resolvido (`.B`).
3. Walk fn 8 parameters (`.C`).
4. Struct `CounterStateLegacy` eliminado (`.D`).
5. Layouter `counter` field eliminado (`.E`).
6. API pública per decisão (`.F`).
7. Tests adaptados + sentinelas M6 fechado (`.G`).
8. L0s actualizados com marco (`.H`).
9. **ADR-0070 ACEITE** (`.I`).
10. Verificações `.J` passam (22/22).
11. **M6 fechado completo**.
12. **F1 fechado**.
13. **F3 parcialmente fechado**.
14. Output observable em produção inalterado.
15. Relatório consolidado P190 escrito (`.K`).

---

## O que pode sair errado

- **Callers externos de `CounterStateLegacy` em
  `02_shell/03_infra/04_wiring/`**: cláusula gate
  substancial — Opção α em `.A.10` (struct vazia
  preservada) ou adaptação cross-cutting.
- **Walk arm Labelled `lang` source não acessível
  via parameter walk fn**: cláusula gate substancial
  — Opção γ ou δ em `.A.11`.
- **`compute_heading_for_toc` ainda recebe `state`
  parameter**: cláusula gate trivial — adaptar
  signature.
- **Layouter outros usos de `self.counter`
  esquecidos**: cláusula gate trivial — completar
  migration.
- **Tests sentinela em quantidade significativa
  regridem**: padrão pragmático auditor #1.
- **Snapshot tests divergem**: improvável após M6
  refactor; investigar.
- **Hash recálculo falha**: cláusula gate trivial.
- **L0 marco M6 não captura todos achados**:
  cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: M+. ~80-150 LOC produção (walk fn
  parameter elimination + struct elim + Layouter
  field + API + comentários) + ~30 LOC tests
  adaptados + ~50 LOC L0 + ~400 LOC relatório
  consolidado.
- **Sem dependências externas novas**.
- **ADR-0070 ACEITE**.
- **3ª aplicação directa ADR-0071 mecanismo em
  P190**.
- **Pattern "eliminação write paralelo M5"**: 8ª e
  final aplicação concreta.
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**:
  - Callers externos breaking change.
  - `lang` source não acessível via parameter.
- **Marco arquitectural**:
  - **M6 fechado completo pela primeira vez desde
    P185A**.
  - **F1 fechado**.
  - **F3 parcialmente fechado**.
  - 5 ADRs completos no ciclo (ADR-0067, 0068,
    0069, 0070, 0071).
  - 32ª aplicação diagnóstico-primeiro consecutiva.
- **Após P190I**: aguarda decisão estratégica do
  utilizador. Próximos passos potenciais:
  - M7 (loop fixpoint).
  - M8 (memoização comemo).
  - F3 completo.
  - Lacunas residuais.
  - Pausa estratégica.
