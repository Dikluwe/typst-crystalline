# Diagnóstico — Figure per-kind counter (Passo 184A)

**Data**: 2026-05-03
**Passo**: P184A — diagnóstico-primeiro / L0-puro
**Escopo**: desbloqueio de C3 (figure auto-number per kind) refinando
arm `Figure` em `from_tags` para usar chave `figure:{kind}` em vez
de chave global `"figure"`.
**Postura**: zero código tocado em L1–L4; zero testes modificados;
zero L0 modificado. Decisões + plano executável.

---

## §1 Validação do estado actual

Inspecção empírica em 2026-05-03. Comandos executados:

- `grep -rn "figure_numbers\|figure_label_numbers\|figure:" 01_core/src/`
- `grep -rn "kind_index\[ElementKind::Figure\]\|query_by_kind(ElementKind::Figure" 01_core/`
- `grep -rn "figure_numbers" 01_core/src/rules/layout/`

| Item | Estado confirmado | Linha actual / observação |
|------|-------------------|---------------------------|
| 1 | Arm `Figure` em `from_tags` usa chave global `"figure"` | `01_core/src/rules/introspect/from_tags.rs:77`: `intr.counters.apply_at("figure".to_string(), counter_update.clone(), *loc)`. Campo `kind: Option<String>` ignorado via `..` pattern em linha 71. |
| 2 | `element_payload.rs:52` documenta convenção `figure:{kind}` | Doc comment `/// Update implícito do contador `figure:{kind}`.` em `01_core/src/entities/element_payload.rs:52`. **Não está implementada** em `from_tags`. |
| 3 | `extract_payload` produz `ElementPayload::Figure { kind: kind.clone(), ... }` | `01_core/src/rules/introspect/extract_payload.rs:27–34`. `kind` data flui ao tag stream. |
| 4 | `kind_index[ElementKind::Figure]` consumers | Apenas tests (`introspector.rs:277-290`, `from_tags.rs:311,365,395`, `introspect.rs:1654-1751`). **Zero consumers de produção** fora de tests. Refinar arm `Figure` para popular per-kind counter não regride. |
| 5 | Consumer C3 em `mod.rs:435–439` | `kind_key = kind.as_deref().unwrap_or("image")` (linha 431); `idx = *figure_progress.entry(kind_key).or_insert(0)` (linha 432–433); `state.counter.figure_numbers.get(kind_key).and_then(\|v\| v.get(idx)).copied().unwrap_or(idx + 1)` (linha 435–439). |
| 6 | `state.figure_numbers` legacy é copiado para Layouter? | **NÃO**. `mod.rs:1414–1430` (no-TOC) e `mod.rs:1444–1460` (TOC fixpoint) não copiam o campo. `figure_numbers` aparece **apenas uma vez** em `mod.rs` — o read em linha 435. `Layouter::new()` (`mod.rs:150`) inicializa `CounterStateLegacy::new()` (default vazio). Logo `unwrap_or(idx + 1)` é o caminho real em produção. **Dead code confirmado**. |
| 7 | `figure.rs:16` doc comment | `/// `counter_state.figure_numbers` — a introspecção pré-computou os números.` — **doc comment factualmente desactualizado** (a introspecção popula o campo no `state` mas o `state` nunca é copiado ao Layouter). Cleanup ortogonal à migração. |

---

## §2 Decisões cláusula 1–6

### Cláusula 1 — Convenção de chave

**O1 (inputs)**: `element_payload.rs:52` documenta `figure:{kind}`.
Walk legacy em `introspect.rs:391` resolve `kind.as_deref().unwrap_or("image")`.
Consumer Layouter em `mod.rs:431` resolve `kind.as_deref().unwrap_or("image")`.
Default `"image"` é convenção pré-existente em ambos os lados.

**O2 (alternativas)**:
- **Opção A**: `figure:{kind}` quando `Some`, `figure:image` quando `None`
  (default kind = `"image"`).
- **Opção B**: `figure:{kind}` quando `Some`, `figure` sem sufixo quando
  `None`.
- **Opção C**: sempre `figure:{kind}`, default fixado em código.

**O3 (critério)**: Opção A alinha-se com o default `"image"` já presente
nos dois caminhos legacy (walk + Layouter). Opção B introduz divergência
de chave (`figure` vs `figure:image`) sem ganho. Opção C é equivalente
a A se o default for `"image"`.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: reversível — chave é detalhe interno do
sub-store; mudar de A para B/C é localizado a `from_tags` arm + método
trait.

**Decisão**: **Opção A**. Chave `figure:{kind}` quando `Some`,
`figure:image` quando `None`. Replica o default `"image"` já estabelecido
em `introspect.rs:391` e `mod.rs:431`.

### Cláusula 2 — Método trait

**O1 (inputs)**: trait `Introspector` actual tem 15 métodos. Métodos
relacionados:
- `figure_number_for_label(&self, label: &Label) -> Option<usize>`
  (P168) — pattern `<noun>_for_<key>`.
- `bib_number_for_key(&self, key: &str) -> Option<u32>` (P181F) —
  mesmo pattern.
- `bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>` (P181F).
- `formatted_counter(&self, key: &str) -> Option<String>` (P170).
- `formatted_counter_at(&self, key: &str, location: Location) -> Option<String>`
  (P177) — pattern `<noun>_at`.

**O2 (alternativas)**:
- **Opção α**: `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>`.
- **Opção β**: `figure_number_for_kind_index(&self, kind: &str, idx: usize) -> Option<usize>`.
- **Opção γ**: `figure_number_at(&self, kind: &str, location: Location) -> Option<usize>`
  — location-aware.

**O3 (critério)**: Opção γ resolveria também C1/C2 (location-aware é o
desbloqueio cross-cutting M6+) mas pressupõe Layouter location-aware,
que é P185+. Misturar P184 com P185 viola escopo. Opção β segue pattern
`<noun>_for_<key>` mas o "key" é composto (`kind` + `idx`) — naming
desajeitado. Opção α é mais legível e o sufixo `_at_index` documenta
explicitamente que `idx` é posição (não Location). Já existe pattern
`<noun>_at` no trait (`formatted_counter_at`); `_at_index` é variação
clara.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: reversível — renomear método é refactor
mecânico.

**Decisão**: **Opção α**. `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>`.

### Cláusula 3 — Sub-store alvo

**O1 (inputs)**: `TagIntrospector` tem 5 sub-stores
(`LabelRegistry`, `CounterRegistry`, `kind_index` HashMap,
`figure_label_numbers` HashMap directo, `MetadataStore`,
`StateRegistry`, `BibStore`). `CounterRegistry` já tem
`apply_at(key, update, loc)` que regista snapshot por Location e
suporta `value_at(key, location) -> Option<&[usize]>`.

**O2 (alternativas)**:
- **Opção 1**: reusar `CounterRegistry`, populando com chave
  `figure:{kind}` (cláusula 1).
- **Opção 2**: novo sub-store dedicado `FigureNumbersRegistry`
  (análogo a `BibStore`).

**O3 (critério)**: 6 sub-stores para 1 caso de uso é overhead.
`CounterRegistry` foi desenhado exactamente para isto — counters por
kind. A primitiva `apply_at` regista a sequência de snapshots; o
método trait `figure_number_at_index` extrai o snapshot do idx-th
update ao counter, que coincide com o número da figure (idx-th figure
do kind tem número idx+1, capturado no snapshot).

**O4 (magnitude)**: trivial (decisão arquitectural).

**O5 (reversibilidade)**: reversível em P184B se forem detectados
limites do `CounterRegistry` (improvável — primitiva já existe).

**Decisão**: **Opção 1**. `CounterRegistry` populado pelo arm
`Figure` refinado.

**Nota P184B**: a impl de `figure_number_at_index` precisará de aceder
à `history` do `CounterRegistry` por idx (não por Location). Pode
exigir um helper interno `value_at_index(&self, key: &str, idx: usize) -> Option<&[usize]>`
no `CounterRegistry` que expõe a história por posição. Esta extensão
é parte de P184B e não é novidade arquitectural — apenas um getter
sobre o campo `history` já existente.

### Cláusula 4 — Forma de migração de consumer

**O1 (inputs)**: padrão P168/P181G/P182D — substitution-with-fallback.

**O2 (alternativas)**:
- **Opção i**: substitution-with-fallback (Introspector primeiro,
  fallback legacy via `||` ou `.or_else`).
- **Opção ii**: substituição directa (sem fallback) — só se legacy for
  garantidamente substituído.

**O3 (critério)**: padrão P168/P181G/P182D estabeleceu fallback como
forma de migração reversível. Achado §1 item 6: legacy é dead code,
logo o fallback é defensivo (reversibilidade) mas não preserva
paridade observable de path legacy real (não existe). Manter padrão é
defensável: sinal explícito de migração, fácil reverter.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: trivial — `||` é padrão repetido.

**Decisão**: **Opção i**. Substitution-with-fallback replicando
P168/P181G/P182D. Forma esperada (per `mod.rs:435–439`):

```rust
let figure_number = self.introspector
    .figure_number_at_index(&format!("figure:{}", kind_key), idx)
    .or_else(|| self.counter.figure_numbers
        .get(kind_key).and_then(|v| v.get(idx)).copied())
    .unwrap_or(idx + 1);
```

(Forma final fixada em P184D.)

### Cláusula 5 — Legacy paralelo

**O1 (inputs)**: walk arm `Content::Figure` em `introspect.rs:391–399`
popula `state.figure_numbers[kind].push(N)`. Achado §1 item 6: este
campo nunca é copiado ao Layouter. É dead code do ponto de vista
observable de produção.

**O2 (alternativas)**:
- **Opção 1**: manter walk arm legacy populado em paralelo (M6
  elimina junto com o resto de `CounterStateLegacy`).
- **Opção 2**: eliminar população legacy já em P184 (P184B refina
  walk arm para parar de popular `state.figure_numbers`).

**O3 (critério)**: Opção 2 limpa dead code. Opção 1 mantém simetria
com P181/P182 (que preservaram legacy paralelo até M6 mesmo com
Opção 3). Honestidade obrigatória: o legacy não preserva paridade
observable real — só simetria de processo. Cleanup de dead code é
ortogonal à migração de C3. Misturar reduções de dead code com
migrações arquitecturais aumenta o tamanho de cada passo sem
benefício correlacionado.

**O4 (magnitude)**: trivial em ambas.

**O5 (reversibilidade)**: reversível em ambas.

**Decisão**: **Opção 1**. Manter walk arm legacy populado em paralelo
até M6. Cleanup de `state.figure_numbers` (e `local_figure_counters`)
fica como item separado de DEBT-cleanup quando M6 for tratado, junto
com a eliminação geral de `CounterStateLegacy`. Esta decisão preserva
a simetria com P181/P182 e mantém o passo P184 focado.

**Registo honesto**: o paralelo legacy é dead code factual em
produção (não "redundância defensiva"). Manter é decisão de simetria
de processo e custo, não de preservação observable.

### Cláusula 6 — Critério de fecho de C3

**O1 (inputs)**: padrão P181/P182 — Opção 3 (infra pronta + consumer
migrado + tests E2E confirmam paridade).

**O2 (alternativas)**:
- **Opção 3**: infra (arm refinado + método trait + impl) **e**
  consumer C3 migrado **e** tests E2E confirmam paridade.
- **Opção 3'**: variação com `+` adicional (ex. eliminação legacy
  já em P184).

**O3 (critério)**: Opção 3 simétrica com P181/P182. Opção 3' adiciona
escopo (cláusula 5 Opção 2) — rejeitado em cláusula 5.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: trivial.

**Decisão**: **Opção 3**. C3 fecha quando:
1. `from_tags` arm `Figure` popula `CounterRegistry` com chave
   `figure:{kind}` via `apply_at` (P184B).
2. Trait `Introspector::figure_number_at_index(kind, idx) -> Option<usize>`
   declarado e implementado em `TagIntrospector` (P184C).
3. Consumer C3 em `mod.rs:435–439` migrado para
   substitution-with-fallback (P184D).
4. Tests E2E em `tests.rs` submódulo `p184e_figure_per_kind`
   confirmam paridade (P184E).
5. Linter zero violations; `cargo test --workspace --lib` passa
   com Δ baseline esperado +5 (~3 unit em arm refinado/método trait
   + ~2 E2E).

---

## §3 Plano de sub-passos sem condicionais

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Refinar `from_tags.rs` arm `Figure`: `kind_key = kind.as_deref().unwrap_or("image")`; `apply_at(format!("figure:{}", kind_key), counter_update.clone(), *loc)`. Manter chamada existente `apply_at("figure", ...)` global em paralelo OU substituí-la — decidido em P184B per Q3. Hash L0 actualiza. | S | — |
| `.C` | Adicionar `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>` ao trait `Introspector`. Impl em `TagIntrospector` delega a `CounterRegistry` (helper `value_at_index` se necessário). 5 tests unitários (vazio, populate, kinds isolados, idx fora de range, default kind). Hash L0 `entities/introspector.md` actualiza. | S | `.B` |
| `.D` | Migrar consumer C3 em `mod.rs:435–439` com substitution-with-fallback. Trait import local. Hash L0 `rules/layout.md` actualiza. | S | `.B`, `.C` |
| `.E` | Tests E2E em submódulo `p184e_figure_per_kind` em `tests.rs`. ~3 tests: pipeline via Introspector, pipeline via fallback, paridade legacy vs migrated. | S | `.D` |
| `.F` | Relatório `00_nucleo/materialization/typst-passo-184f-relatorio.md`. Actualização preventiva DEBT M4-residual: se P183F já correu, P184F update remove C3; se P183F não correu, P184F precede e DEBT abre cobrindo apenas C1+C2. | S | `.E` |

**Sem cláusulas condicionais**. Sequência fixa B → C → D → E → F.

---

## §4 Magnitude consolidada

S agregado para P184B–F (~80–150 LOC):
- `.B`: ~5 LOC alteradas + 0–2 tests existentes adaptados em
  `from_tags.rs` (test `figura_numerada_com_label_popula_figure_label_numbers`
  pode precisar update se o counter "figure" global mudar).
- `.C`: ~10 LOC trait + impl + 5 tests unit (~30 LOC tests) + helper
  `value_at_index` no `CounterRegistry` (~5 LOC + ~5 LOC tests).
- `.D`: ~5 LOC alteradas em `equation.rs:97` consumer (na verdade
  `mod.rs:435–439`, typo na minha contagem inicial; corrigido).
- `.E`: ~3 tests E2E (~30 LOC).
- `.F`: relatório.

Total: ~80–120 LOC produção + tests, alinhado com S.

---

## §5 ADR avaliação

- **Convenção de chave `figure:{kind}`**: replica padrão P182A
  (`numbering_active:<feature>`) — **não ADR**.
- **Método trait `figure_number_at_index`**: replica P181F
  (`bib_number_for_key`) — **não ADR**.
- **Refinamento de arm existente em `from_tags`**: refino, não novo
  locatable kind (Figure já é locatable desde P165) — **não ADR**.
- **Helper `CounterRegistry::value_at_index`**: extensão localizada
  do sub-store já existente — **não ADR**.

**Conclusão**: P184A não cria ADR.

---

## §6 DEBT avaliação

P184A não abre DEBT. P184F (fecho da série) actualiza o DEBT
M4-residual:

- **Se P183F já correu** (DEBT aberto cobre C1+C2+C3): P184F update
  remove C3 da lista, deixando C1+C2.
- **Se P183F ainda não correu**: P184F precede; quando P183F correr,
  abre DEBT cobrindo apenas C1+C2 (não C3).

P184A regista esta dependência preventiva (per .J output 3) mas
não toma acção sobre o DEBT.

---

## §7 Relação com P183D bloqueio

P183D §3 categorizou os bloqueios por eixo:

| Consumer | Eixo 1 | Eixo 2 |
|----------|--------|--------|
| C1 | ❌ snapshot-during-walk | ✅ |
| C2 | ❌ snapshot-during-walk | ❌ sem arm Equation |
| C3 | ✅ snapshot-final adequado | ❌ chave global, não per-kind |

P184 ataca **eixo 2 de C3** isoladamente:
- Não toca em eixo 1 (C3 já é OK em eixo 1).
- Não toca em C1 (espera Layouter location-aware — P185).
- Não toca em C2 (espera location-aware + emissão Tag Equation —
  P185 + P186).

P183D §3 §4.3 confirmou: C3 é o consumer **mais barato** para
desbloquear individualmente (não exige cross-cutting M6+ change).
P184 materializa esta observação.

---

## §8 Próximo sub-passo — P184B

Escopo concreto:

1. Em `01_core/src/rules/introspect/from_tags.rs:71–95`, alterar arm
   `ElementPayload::Figure { counter_update, is_counted, kind, .. }`
   (destructure `kind`):
   ```rust
   let kind_key = kind.as_deref().unwrap_or("image");
   intr.counters.apply_at(
       format!("figure:{}", kind_key),
       counter_update.clone(),
       *loc,
   );
   ```
   Decisão sobre manter `apply_at("figure", ...)` global em paralelo
   é parte de P184B (provável: manter por simetria com walk legacy,
   mas pode ser eliminado se nenhum consumer existir).

2. L0 `00_nucleo/prompts/rules/introspect/from_tags.md` actualizado
   com a convenção `figure:{kind}` documentada (era doc comment em
   `element_payload.rs:52`; promover a L0).

3. Hash L0 actualiza após edit.

4. Tests existentes em `from_tags.rs` (linhas 339–396) podem precisar
   update se asseram sobre chave global. Confirmar empiricamente em
   P184B.

5. Critério de fecho P184B: `cargo test --workspace --lib` passa com
   Δ baseline +0 (apenas alteração de arm, sem novos tests; existentes
   actualizados se necessário).
