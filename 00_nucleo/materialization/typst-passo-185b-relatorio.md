# Relatório P185B — Trait methods location-aware

**Data**: 2026-05-03
**Magnitude**: S (executada como S puro — sem cláusula gate substancial)
**Pré-condição**: P185A concluído + ADR-0068 PROPOSTO ✅

---

## Resumo

Trait `Introspector` ganha 2 métodos location-aware:

- `is_numbering_active_at(key, location) -> bool` — variante
  location-aware de `is_numbering_active` (P182B). Delega a
  `state.value_at(key, location)` + match `Some(Value::Bool(true))`.
- `flat_counter_at(key, location) -> Option<usize>` — variante
  location-aware para counters flat. Delega a
  `counters.value_at(key, location)?.last().copied()`.

Impls em `TagIntrospector` seguem padrão P177
(`formatted_counter_at`) + P184C (`figure_number_at_index`):
delegação directa aos sub-stores `StateRegistry` e
`CounterRegistry` (já com `value_at` populado per P171/P184B).

10 tests unitários novos cobrem populate + lookup por Location +
re-update + isolamento por chave + Location anterior à primeira
update. **Caso central** (re-update reflectindo Location
consultada) inclui assert explícito de divergência face a
`is_numbering_active` snapshot-final, validando o eixo 1 da
regra dos 2 eixos.

Layouter, walk arm, `extract_payload`, `from_tags`, `Locator`,
`StateRegistry` e `CounterRegistry` **não** foram modificados.
Output observable inalterado em produção (métodos novos sem
consumer ainda; Layouter migra em P187+P188 após P185C).

---

## Confirmação `.F` (11/11)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa | ✅ 1779 verdes (1769 +10) |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | `is_numbering_active_at` accessible via trait | ✅ |
| 5 | `flat_counter_at` accessible via trait | ✅ |
| 6 | `TagIntrospector` impls delegam correctamente | ✅ |
| 7 | Re-update casos retornam valor por Location | ✅ test `is_numbering_active_at_re_update_reflecte_location_consultada` + `flat_counter_at_re_update_reflecte_location_consultada` |
| 8 | Walk **NÃO** modificado | ✅ `git diff --stat` confirma |
| 9 | Layouter **NÃO** modificado | ✅ idem |
| 10 | Snapshot tests ADR-0033 verdes | ✅ incluídos no total `cargo test --workspace` |
| 11 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P185A (P184F): **1769** verdes
- Após P185B: **1779** verdes
- Δ: **+10** (5 para `is_numbering_active_at`, 5 para
  `flat_counter_at`) — limite superior do range esperado [+8, +10].

---

## Hashes finais

L0 modificado: `00_nucleo/prompts/entities/introspector.md`

- Hash do código (registado no L0): `0938d161`
- Hash do prompt (registado em `@prompt-hash` do `.rs`): `070a390f`

`crystalline-lint --fix-hashes .` aplicado uma vez após edit.
Análise final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

1. **Posição dos métodos no trait**: inseridos após
   `figure_number_at_index` (P184C, último método antes da
   adição), preservando ordem cronológica de adição —
   convenção observada em P181F→P182B→P184C. Alternativa
   considerada (agrupar por categoria location-aware: `state_value`,
   `formatted_counter_at`, `is_numbering_active_at`,
   `flat_counter_at`) descartada por exigir reorganização de
   métodos pré-existentes — fora do escopo P185B.

2. **Defaults idênticos a P182B/P177**: `false` para flag
   `Bool` ausente; `None` para counter ausente. Nenhuma
   divergência semântica face às variantes snapshot-final —
   apenas o eixo 1 (Location) muda.

3. **Test re-update inclui assert de divergência**: o test
   `is_numbering_active_at_re_update_reflecte_location_consultada`
   inclui `assert!(!i.is_numbering_active(...))` para tornar
   explícito que `is_numbering_active` (snapshot final) e
   `is_numbering_active_at(loc(15))` divergem para o mesmo
   estado — esta é a precondição funcional para C1 desbloqueio
   em P187. Sem o assert de contraste, o test seria
   semânticamente equivalente a um teste de P171 e o valor
   diferencial perderia-se.

4. **Sem cláusula gate disparada**: `value_at` em ambos
   sub-stores tem assinatura esperada conforme P185A
   diagnóstico; `Location` já importado; matching `Value::Bool`
   replica P182B literalmente.

5. **Documentação inline aviso heading**: `flat_counter_at`
   doc string explicita que `.last()` em counter hierárquico
   (heading) retorna apenas o nível mais profundo — caller
   deve usar `formatted_counter_at` (P177) nesse caso. Nota
   inline em vez de `debug_assert!` (counter hierárquico não
   é erro funcional, é semântica documentada).

---

## Estado actual

- **P185 série**: A ✅ B ✅ | C-E pendentes.
- **M9** (counter): 11/11 — métodos novos são extensão
  location-aware, não slot novo de feature.
- **Trait `Introspector`**: 16 → **18 métodos**.
- **Tests workspace**: 1769 → 1779 (+10).
- **45 passos executados** (após P184F).
- **M5/M4 progresso**: 6/12 read-sites migrados (inalterado —
  C1+C2 ainda bloqueados; desbloqueio em P187+P188).
- **DEBT M4-residual**: cobre apenas C1 + C2 (inalterado
  per P184F cenário B).

---

## Pendências cumulativas

Inalteradas em P185B:

- P183B (C1 heading prefix migration) — depende de P185C
  (Layouter `current_location`) + P187 (consumer migra para
  `is_numbering_active_at`).
- P183C (C2 equation counter migration) — depende de P185C +
  P188 (consumer migra para `flat_counter_at`).
- 4 sites M4-fora-de-escopo (TOC, fixpoint side-channels,
  resolved labels) — fora de escopo P185.

---

## Próximo passo

**P185C** — Layouter `Locator` field + `current_location`
field; gating em `layout_content` para actualizar
`current_location` durante walk. **Magnitude M genuíno** —
primeira introdução de `Locator` no Layouter; corresponde ao
mecanismo M3 da ADR-0068 PROPOSTO. Pré-condição P187/P188.

Após P185C, ADR-0068 transita de PROPOSTO → ACEITE.
