# Passo P190B — Categoria Bibliography (M6 incremental por categoria)

Primeiro passo de implementação P190 (após P190A
diagnóstico). Magnitude **M** — categoria mais simples
da estratégia β (per P190A §12).

P190A confirmou empiricamente:
- **16 campos** em `CounterStateLegacy` (correcção -2 vs F1).
- **API interna** ao crate — eliminação livre.
- **8 sub-passos** B-I planeados (estratégia β
  incremental por categoria).
- **Categoria 1 (Bibliography)** identificada como
  mais simples — caminho Introspector activo desde
  P181H; consumer migration directa.
- **ADR-0070 PROPOSTO** criada em P190A.N.

P190B é trabalho **incremental**:
- Migrar 2 Layouter consumers
  (`mod.rs:665, 673`).
- Eliminar 2 fields (`bib_entries`, `bib_numbers`).
- Walk arm Bibliography já é puro desde P181H — sem
  trabalho de purificação necessário.

Trabalho concreto:
1. Confirmar walk arm Bibliography puro desde P181H.
2. Migrar Layouter consumer `mod.rs:665, 673`:
   - `self.counter.bib_entries.iter().find(...)` →
     `self.introspector.bib_store.lookup(...)`.
   - `self.counter.bib_numbers.get(...)` →
     `self.introspector.bib_store.assigned_number(...)`.
3. Eliminar fields `bib_entries` + `bib_numbers` de
   `CounterStateLegacy`.
4. Adaptar tests Layouter dependentes (padrão
   pragmático auditor #1).
5. Hash actualizado via `crystalline-lint
   --fix-hashes`.
6. L0 actualizado.

Após P190B:
- `CounterStateLegacy`: 16 → **14 fields**.
- Layouter Bibliography consumers via Introspector
  path completo (sem fallback legacy).
- F1 progresso: 14/16 fields ortogonais ainda
  presentes.
- Pattern "eliminação write paralelo M5" — 1ª
  aplicação concreta.

**Pré-condição**: P190A concluído. Tests workspace
1.869 verdes; zero violations. ADR-0070 PROPOSTO
criada. Inventário 16 campos × consumer × cobertura
disponível.

**Restrições**:
- **Não** modificar walk arm Bibliography — já é
  puro desde P181H.
- **Não** modificar `from_tags` arm Bibliography
  (P181H).
- **Não** modificar trait `Introspector` (P185B
  fechou; bib_store funcional desde P181H).
- **Não** modificar `TagIntrospector` (P181H abriu
  bib_store).
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar outros campos do struct (apenas
  bib_entries + bib_numbers).
- **Não** modificar 4 helpers `compute_*` — outros
  passos.
- **Não** materializar lacunas residuais (#1, #1b,
  #2).
- API pública preservada (eliminação interna —
  confirmado P190A §1).

---

## Sub-passos

### .A Auditoria L0

#### Estado walk arm Bibliography

1. Confirmar walk arm Bibliography em
   `01_core/src/rules/introspect.rs`:
   - **Esperado**: walk puro desde P181H — sem
     mutações `state.bib_entries.push` ou
     `state.bib_numbers.insert`.
   - Empiricamente: `grep -n "bib_entries\|bib_numbers"
     01_core/src/rules/introspect.rs` deve mostrar
     **zero ocorrências** de mutação no walk arm.

2. Confirmar `from_tags` arm Bibliography (P181H)
   funcional:
   - Popula `intr.bib_store` via Tag::Bibliography.
   - Trait method `bib_store_lookup`,
     `bib_store_assigned_number`, etc. expostos.

#### Inventário Layouter consumers

3. Confirmar consumers Layouter em
   `01_core/src/rules/layout/mod.rs:665, 673` (per
   P190A §6):
   - Forma exacta: `self.counter.bib_entries.iter().find(...)`
     e `self.counter.bib_numbers.get(...)` ou
     similar.
   - Identificar contexto (qual função; em que arm
     do `layout_content` match).

4. Identificar API equivalente Introspector:
   - `intr.bib_store.lookup(key) -> Option<&BibEntry>`
     ou similar (per P181H).
   - `intr.bib_store.assigned_number(key) ->
     Option<u32>` ou similar.
   - Confirmar empiricamente nomes exactos dos
     métodos.

#### Inventário fields a eliminar

5. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub bib_entries: Vec<BibEntry>`.
   - `pub bib_numbers: HashMap<String, u32>`.
   - Eliminar **ambos os fields**.
   - Adaptar `Default` impl, `new()` constructor,
     etc.

#### Tests dependentes

6. Identificar tests que usam `state.bib_entries` ou
   `state.bib_numbers`:
   - `grep -rn "bib_entries\|bib_numbers"
     01_core/src/`.
   - Tests sentinela P181H que verificam paridade
     legacy + Introspector.
   - **Decisão**: tests sentinela legacy (paridade)
     ficam **redundantes** após eliminação dos
     fields — adaptar ou remover.

#### L0 alvos

7. Identificar L0s a tocar:
   - `entities/counter_state_legacy.md` (fields
     eliminados).
   - `rules/layout/mod.md` (consumers migrados).
   - Possivelmente outros.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Walk arm Bibliography confirmado puro.
- Layouter consumers localizados.
- API Introspector equivalente confirmada.
- Tests dependentes identificados.

### .B Migrar Layouter consumer 1 (`mod.rs:665`)

1. Em `01_core/src/rules/layout/mod.rs:665` (per
   `.A.3`):
   - Substituir `self.counter.bib_entries.iter().find(...)`
     por `self.introspector.bib_store_lookup(...)`
     (forma exacta depende de assinatura confirmada
     em `.A.4`).
   - **Sem fallback legacy** — caminho Introspector
     completo. Razão: P181H já garante write paralelo
     há 19 séries; caminho Introspector estável.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumer 1 migrado.
- `cargo check --workspace` passa.

### .C Migrar Layouter consumer 2 (`mod.rs:673`)

1. Em `01_core/src/rules/layout/mod.rs:673`:
   - Substituir `self.counter.bib_numbers.get(...)`
     por `self.introspector.bib_store_assigned_number(...)`.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumer 2 migrado.

### .D Eliminar fields `bib_entries` + `bib_numbers`

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar field `pub bib_entries: Vec<BibEntry>`.
   - Eliminar field `pub bib_numbers: HashMap<String,
     u32>`.

2. Adaptar `Default` impl ou `new()` constructor:
   - Remover inicialização dos 2 fields.

3. Confirmar `cargo check --workspace` passa.

4. Importações de `BibEntry` em
   `counter_state_legacy.rs` podem ficar
   redundantes — limpar via `cargo check` warnings.

**Critério de saída**:
- 2 fields eliminados.
- `cargo check --workspace` passa.
- `CounterStateLegacy`: 16 → 14 fields.

### .E Adaptar tests

1. Identificar tests que regridem (per `.A.6`):
   - Tests sentinela P181H que verificam paridade
     legacy + Introspector.
   - Tests que iteram sobre `state.bib_entries` ou
     usam `state.bib_numbers`.

2. Adaptação por padrão pragmático auditor #1:
   - Tests redundantes (paridade) — **remover** ou
     **converter** para verificar apenas Introspector
     path.
   - Tests que ainda fazem sentido — adaptar usando
     `intr.bib_store_*` em vez de `state.bib_*`.

3. Confirmar tests existentes não regridem (após
   adaptação).

**Critério de saída**:
- Tests adaptados.
- Tests workspace verdes.

### .F Actualizar L0

1. `entities/counter_state_legacy.md`:
   - Remover entradas para fields `bib_entries` +
     `bib_numbers`.
   - Cross-reference: P190B (eliminação categoria
     Bibliography); P181H (origem do sub-store
     Introspector).
   - Indicar progresso: 16 → 14 fields.

2. `rules/layout/mod.md` (se existir):
   - Actualizar consumers Bibliography.
   - Cross-reference P190B.

3. Hash em branco aguarda recálculo manual em `.G`.

**Critério de saída**:
- L0s actualizados.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P190A
   baseline (1.869): **depende da adaptação tests**
   (esperado: 0 ou Δ marginal negativo se sentinelas
   legacy removidas).
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Layouter consumers Bibliography migrados (sem
   fallback legacy).
5. `CounterStateLegacy.bib_entries` **NÃO existe**.
6. `CounterStateLegacy.bib_numbers` **NÃO existe**.
7. `CounterStateLegacy`: 14 fields (era 16).
8. Walk arm Bibliography **NÃO modificado** (já era
   puro desde P181H).
9. `from_tags` arm Bibliography **NÃO modificado**
   (P181H).
10. Trait `Introspector` **NÃO modificado**.
11. `TagIntrospector` **NÃO modificado**.
12. ADR-0070 PROPOSTO **NÃO transitada** ainda
    (ACEITE em P190I).
13. Snapshot tests verdes.
14. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190b-relatorio.md`
com:

- Resumo: categoria 1 (Bibliography) eliminada;
  Layouter migrado para Introspector path completo;
  struct reduzido a 14 fields.
- Confirmação `.G` (14 verificações).
- Δ tests vs baseline P190A.
- Hashes finais L0 (`counter_state_legacy.md` +
  `rules/layout/mod.md` se existir).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P190 série: A ✅ B ✅ | C-I pendentes.
  - **Categoria 1 (Bibliography) fechada**.
  - 86 passos executados (P190A=85 + P190B=86).
  - 1ª aplicação concreta do pattern stylesheet
    "eliminação write paralelo M5".
- Pendências cumulativas: 7 categorias restantes.
- Próximo passo: P190C — categoria 2 (Page tracking).
  Magnitude M.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Layouter consumer 1 migrado (`.B`).
3. Layouter consumer 2 migrado (`.C`).
4. Fields `bib_entries` + `bib_numbers` eliminados
   (`.D`).
5. Tests adaptados (`.E`).
6. L0s actualizados (`.F`).
7. Verificações `.G` passam (14/14).
8. `CounterStateLegacy`: 14 fields.
9. Walk arm Bibliography NÃO modificado (já puro).
10. Output observable em produção inalterado
    (caminho Introspector activo desde P181H; mudança
    é apenas onde Layouter lê).
11. Relatório `.H` escrito.

---

## O que pode sair errado

- **Walk arm Bibliography ainda tem mutações
  legacy** (improvável — P181H deveria ter purificado):
  cláusula gate substancial. Investigar.
- **`from_tags` arm Bibliography não popula
  bib_store correctamente** (improvável — P181H
  funcional há 19 séries): cláusula gate
  substancial.
- **API equivalente Introspector tem assinatura
  diferente do esperado**: cláusula gate trivial —
  ajustar consumer migration.
- **Tests sentinela P181H regridem por causa de
  eliminação de fields**: cláusula gate trivial —
  remover ou adaptar.
- **Tests Layouter regridem por mudança de
  consumer**: cláusula gate substancial — investigar
  divergência observable.
- **Snapshot tests divergem**: improvável (caminho
  Introspector e legacy fornecem mesmos dados);
  investigar se acontecer.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M. ~30 LOC produção (consumer
  migrations + field elimination) + ~20 LOC tests
  adaptados + ~30 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR nova** (ADR-0070 PROPOSTO criada em
  P190A; ACEITE em P190I).
- **Categoria mais simples da estratégia β** — caminho
  Introspector activo desde P181H; consumer
  migration directa.
- **Pattern stylesheet "eliminação write paralelo
  M5"**: 1ª aplicação concreta. Padrão para passos
  P190C-P190I:
  1. Auditar consumers Layouter para a categoria.
  2. Migrar consumers para Introspector path.
  3. Eliminar fields legacy correspondentes.
  4. Adaptar tests dependentes.
  5. Actualizar L0.
- **Cláusula gate trivial**: aplicável a forma exacta
  de assinatura, recálculo de hashes.
- **Cláusula gate substancial**: aplicável a:
  - Walk arm não puro (deveria estar).
  - API Introspector divergente.
  - Tests Layouter regridem por divergência
    observable.
- **Próximo passo P190C**: categoria 2 (Page
  tracking). Magnitude M. Trabalho concreto: campos
  `label_pages` + `known_page_numbers` — **categoria
  Layouter-runtime** (P190A §3 achado crítico) —
  movem para nova struct `LayouterRuntimeState` (ou
  similar).
- **Diferença vs P190B**: P190C é primeira aplicação
  da decisão "4 campos Layouter-runtime → struct
  dedicada". Magnitude marginalmente maior.
