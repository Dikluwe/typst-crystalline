# Relatório P184D — migração consumer C3 (figure auto-number per kind)

**Data**: 2026-05-03
**Passo**: P184D — substitution-with-fallback do consumer C3
**Resultado**: consumer migrado; Introspector path activo em produção
(P184B+C populou dados); fallback legacy + heurística `idx + 1` como
rede de segurança; Δ tests workspace 0 (1.764 inalterado); zero
violations linter.

---

## §1 Resumo

`mod.rs:435–439` agora consulta `Introspector::figure_number_at_index`
primeiro, com fallback `or_else` ao legacy `state.figure_numbers` e
`unwrap_or(idx + 1)` final defensivo. Padrão substitution-with-fallback
(P168/P181G/P182D) replicado com camada heurística adicional preservada
do código pré-existente.

L0 `rules/layout.md` actualizado com nova secção "Figure-arm consome
Introspector (P184D)" simétrica às secções P181G e P182D.

**Achado P184A §3.6 ratificado**: o legacy `state.figure_numbers` é
dead code factual em produção (copy-sites não copiam o campo;
Layouter recebe `CounterStateLegacy::new()` vazio em
`Layouter::new()`). Em produção real, fallback legacy retorna sempre
`None`. Antes de P184D: `unwrap_or(idx + 1)` era o caminho activo.
Após P184D: `figure_number_at_index` retorna `Some(idx + 1)` em
produção (P184B+C populou via `apply_at("figure:{kind}", Step, loc)`
para cada figure no walk). Output coincide; estrutura mudou.

C3 é o **primeiro consumer onde o Introspector populado é o caminho
activo**, não redundância paralela ao legacy. Resolve eixo 2 do
bloqueio P183D.

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria L0 | ✅ | C3 confirmado em `mod.rs:435-439`; `kind_key = .unwrap_or("image")` (linha 431); `idx` 0-indexed em ambos paths (`Vec::get` e `value_at_index` via `history.get(key)?.get(idx)`); `self.introspector` acessível com trait import local (P181G/P182D); L0 `rules/layout.md` tem secções dedicadas por migração — adicionar nova. |
| `.B` Actualizar L0 | ✅ | Nova secção "Figure-arm consome Introspector (P184D)" após P182D em `layout.md`. Documenta convenção, idx 0-indexed, comportamento por path, paridade output. |
| `.C` Migrar consumer | ✅ | `mod.rs:435–439` substituído por `self.introspector.figure_number_at_index(kind_key, idx).or_else(|| ...legacy...).unwrap_or(idx + 1)`. Trait import local `use crate::entities::introspector::Introspector;` adicionado dentro do arm. |
| `.D` Regressão | ✅ | `cargo test --workspace` 1.504 + 215 + 24 + 21 = **1.764 verdes** (Δ vs P184C baseline 1.764: **0**). Sem regressão. |
| `.E` Verificação `.E` | ✅ | 11/11 verificações (cf. §3). |
| `.F` Encerramento | ✅ | Este relatório. |

---

## §3 Confirmação `.E` — 11 verificações

1. ✅ `cargo check --workspace` passa (warnings pré-existentes não relacionados).
2. ✅ `cargo test --workspace` passa: **1.764 verdes** (Δ vs P184C baseline 1.764: **0**).
3. ✅ `crystalline-lint .` zero violations (após `--fix-hashes` para 9 ficheiros do módulo `rules/layout/`).
4. ✅ Consumer C3 (`mod.rs:435–439`) consulta `self.introspector.figure_number_at_index(kind_key, idx)` primeiro; fallback legacy `or_else` + `unwrap_or(idx + 1)`.
5. ✅ Walk arm canonical legacy (`introspect.rs:391–399`) **NÃO modificado**.
6. ✅ Write paralelo legacy (`from_tags.rs` global `"figure"`) **NÃO modificado** (P184B preservou).
7. ✅ Copy-sites legacy (`mod.rs:1414–1430`, `mod.rs:1444–1460`) **NÃO modificados**.
8. ✅ Trait `Introspector` **NÃO modificado** (P184C fechou).
9. ✅ `CounterRegistry` **NÃO modificado** (P184C fechou).
10. ✅ Snapshot tests ADR-0033 verdes (parte do conjunto 1.764). Output observable preservado.
11. ✅ Linter passa final (`✓ No violations found`).

---

## §4 Hashes finais L0 modificado

- `00_nucleo/prompts/rules/layout.md`: Hash do Código `647047a9` (anterior `59811524`).
- 9 ficheiros em `01_core/src/rules/layout/` actualizados via `--fix-hashes` para `@prompt-hash: 4c94a7c0`:
  - `mod.rs`, `cursor.rs`, `equation.rs`, `grid.rs`, `helpers.rs`, `hyphenation.rs`, `metrics.rs`, `placement.rs`, `tests.rs`.

Sincronização automática via `crystalline-lint --fix-hashes .`.

---

## §5 Decisões de execução notáveis

1. **Camada heurística `unwrap_or(idx + 1)` preservada**: o passo
   poderia ter eliminado o fallback final pós-Introspector (uma vez
   que P184B+C garante dados em produção), mas a preservação
   replica conservadorismo do padrão P181G/P182D — fallback é
   defensivo + reversível, não redundância acidental. Cleanup do
   fallback fica para M6 junto com eliminação geral do legacy.

2. **Trait import local em vez de top-level**: padrão P181G/P182D
   usa `use crate::entities::introspector::Introspector;` dentro do
   arm específico, não no topo do file. P184D replica esta
   localização — minimiza ruído em escopo geral; trait fica visível
   apenas no bloco que o usa.

3. **Sem alteração ao `figure_progress`**: campo pré-existente do
   Layouter que conta figures `numbering.is_some()` por kind; não
   fica obsoleto pela migração. Continua a ser a fonte de `idx`
   passado ao Introspector. M6 pode reavaliar se este campo é
   necessário ou se Layouter pode tornar-se totalmente
   location-aware (cf. pendência paralela P182E §5.2).

4. **Achado "dead code em produção" registado honestamente**: o
   relatório e o L0 `layout.md` declaram explicitamente que
   `state.figure_numbers` é dead code factual (copy-sites não
   copiam). Não rebaptizado como "redundância defensiva". Cláusula
   de honestidade obrigatória do P184A respeitada.

5. **Paridade output verificada por construção**: counter flat com
   `apply_at(Step)` produz snapshots `[1], [2], [3], ...` — `.last()`
   no impl trait extrai o número 1-based. Mesma sequência que o
   legacy gerava (e que `idx + 1` heurística replica). Tests
   workspace passam sem regressão (1.764 → 1.764), ratificando
   paridade empiricamente.

---

## §6 Estado actual

- **P184 série**: A ✅ B ✅ C ✅ **D ✅** | E–F pendentes.
- **C3 desbloqueado**: eixos 1 e 2 atendidos; consumer migrado.
- **M5/M4 progresso**: 5+1 = **6 read-sites migrados** (P168 figure-ref +
  P181G cite-arm bib_entry + P181G cite-arm bib_number + P182D heading
  numbering + P182D equation numbering + **P184D figure auto-number
  per kind**). 6 de 12 read-sites originais. C1 e C2 continuam
  bloqueados (esperam P185+ location-aware Layouter).
- **Trait `Introspector`**: 16 métodos.
- **`CounterRegistry`**: 6 métodos públicos.
- **M9**: 11/11 (inalterado).
- **41 passos executados** (P184C = 40 + P184D = 41).

---

## §7 Pendências cumulativas

Inalteradas em relação ao estado pós-P184C:

- Lacuna #3 (TOC entries via Introspector) — bloqueada, separada da
  série P184.
- DEBT M4-residual a abrir em P183F (ou actualizar em P184F):
  cobertura final será **C1 + C2** (não C3 — fechado em P184D).
  Cláusula 6 P184A "critério de fecho" satisfeita.
- Pendência paralela P182E §5.2 (location-aware Layouter para
  desbloquear C1+C2) — espera M6+.
- Cleanup dead code legacy (`state.figure_numbers`,
  `local_figure_counters`, chave global `"figure"` paralela em
  `from_tags`, `figure.rs:16` doc comment factualmente
  desactualizado) — orgânico em M6 com eliminação geral de
  `CounterStateLegacy`.

---

## §8 Próximo passo — P184E

Tests E2E em submódulo `p184e_figure_per_kind` em `tests.rs` (~3
tests):

1. **Pipeline completo via Introspector**: documento com 3 figures
   numeradas+captioned; layout via `layout_with_introspector`
   produz prefixos `Figura 1:`, `Figura 2:`, `Figura 3:` em ordem.
2. **Pipeline via fallback (Introspector vazio)**: documento idêntico;
   layout com `TagIntrospector::empty()` cai no fallback legacy
   (que é dead code e cai no `unwrap_or(idx + 1)`); produz mesmos
   prefixos por construção heurística.
3. **Paridade legacy vs migrated**: comparação directa entre
   `layout()` legacy e `layout_with_introspector` populado —
   PagedDocument idêntico (snapshot ADR-0033 implícito).

Tests adicionais possíveis: kinds distintos (`image` + `table`)
isolados; figure sem caption (idx incrementado mas sem
prefixo computado).

Pré-condição P184E: este passo concluído. C3 estruturalmente
migrado; tests E2E confirmam paridade observable.
