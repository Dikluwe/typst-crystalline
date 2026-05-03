# Relatório P184A — diagnóstico-primeiro / L0-puro

**Data**: 2026-05-03
**Passo**: P184A — diagnóstico de C3 (figure auto-number per kind)
**Resultado**: 6 cláusulas fechadas; plano de 5 sub-passos (B–F) sem
condicionais; magnitude S agregada confirmada; ADR não criada; DEBT
M4-residual com dependência preventiva registada.
**Postura**: zero código L1–L4 tocado; zero L0 prompts modificados;
zero testes alterados.

---

## §1 Contexto

P184 abre série M4-residual. P183A diagnosticou que dos 7 read-sites
não migrados de `CounterStateLegacy`, 3 (C1, C2, C3) são consumers de
contadores. P183B/C/D auditaram cada um aplicando a regra dos 2 eixos
e bloquearam todos.

C3 (figure auto-number per kind, `mod.rs:435–439`) bloqueou em **eixo
2 apenas** — eixo 1 (semântica temporal) é OK porque figures recebem
números fixados pós-walk. Eixo 2 falha porque `from_tags` arm Figure
usa chave global `"figure"` em vez da convenção `figure:{kind}`
documentada em `element_payload.rs:52` mas nunca implementada.

P183D §7 declarou C3 como "mais barato individualmente" para
desbloquear (não exige cross-cutting M6+). P184 materializa essa
observação.

---

## §2 Postura do auditor / executor

P184A é passo **L0-puro / diagnóstico-primeiro**, no mesmo registo de
P181A/P182A/P183A.

- Zero código L1–L4 tocado.
- Zero testes novos ou modificados.
- Zero L0 prompts modificados.
- Pode criar ADR `PROPOSTO` se decisão arquitectural exigir — não
  exigiu (cf. §10).
- Pode abrir DEBT — não abriu (cf. §11).
- Não modifica `from_tags` arm Figure (P184B).
- Não adiciona método trait (P184C).
- Não migra consumer C3 (P184D).

**Regra dos 2 eixos pré-aplicada**: P183D já confirmou eixo 1 OK +
eixo 2 falha. P184A não re-faz a auditoria — aceita o resultado e
desenha o caminho de desbloqueio do eixo 2.

---

## §3 Validação do estado actual (sub-passo .A)

Confirmações empíricas (detalhe em
`00_nucleo/diagnosticos/diagnostico-figure-per-kind-passo-184a.md` §1):

1. Arm `Figure` em `from_tags.rs:71–95` continua a usar chave global
   `"figure"` e a ignorar `kind` via `..` pattern.
2. `element_payload.rs:52` continua a documentar convenção
   `figure:{kind}` (não implementada).
3. `extract_payload.rs:27–34` produz `ElementPayload::Figure` com
   `kind: kind.clone()` — dado disponível na tag stream.
4. `kind_index[ElementKind::Figure]` tem **zero consumers de produção**
   fora de tests. Refinar arm não regride outros consumers.
5. Consumer C3 em `mod.rs:435–439` lê
   `state.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx)).copied().unwrap_or(idx + 1)`.
   `kind_key = kind.as_deref().unwrap_or("image")`.
6. `state.figure_numbers` legacy **nunca é copiado** ao Layouter
   (copy-sites `mod.rs:1414–1430` e `mod.rs:1444–1460` não copiam).
   `unwrap_or(idx + 1)` é o caminho real em produção. **Dead code
   confirmado** (achado P183D §1 ratificado).
7. `figure.rs:16` doc comment "introspecção pré-computou os números"
   é factualmente desactualizado. Cleanup ortogonal à migração.

---

## §4 Decisões cláusula 1–6

Detalhe O1–O5 em diagnóstico §2.

| Cláusula | Decisão | Justificação curta |
|----------|---------|--------------------|
| 1 — Convenção de chave | `figure:{kind}` quando `Some`; `figure:image` quando `None` (Opção A) | Replica default `"image"` já presente em `introspect.rs:391` e `mod.rs:431`. |
| 2 — Método trait | `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>` (Opção α) | `_at_index` documenta que `idx` é posição (não Location). Variação clara do pattern `<noun>_at` (cf. `formatted_counter_at`). Option γ (location-aware) misturaria escopo com P185. |
| 3 — Sub-store alvo | `CounterRegistry` (Opção 1) | Já desenhado para counters por kind. `apply_at` regista snapshot por Location; helper `value_at_index` em P184B expõe acesso por idx. 6º sub-store para 1 caso de uso seria overhead. |
| 4 — Forma migração | Substitution-with-fallback (Opção i) | Replica P168/P181G/P182D. Fallback é defensivo — legacy é dead code (cf. §3.6) mas o padrão preserva reversibilidade. |
| 5 — Legacy paralelo | Manter walk arm legacy populado em paralelo até M6 (Opção 1) | Simetria com P181/P182. Cleanup de dead code é ortogonal e fica para limpeza geral M6 junto com `CounterStateLegacy`. **Registo honesto**: o paralelo é dead code factual, não "redundância defensiva". |
| 6 — Critério de fecho | Opção 3 simétrica P181/P182 | Infra (arm + método trait + impl) + consumer migrado + tests E2E paridade. |

---

## §5 Plano de sub-passos sem condicionais

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Refinar `from_tags` arm Figure: chave `figure:{kind}` + L0 actualizado | S | — |
| `.C` | Adicionar `figure_number_at_index` ao trait + impl + helper `value_at_index` no `CounterRegistry` + 5 tests unit | S | `.B` |
| `.D` | Migrar consumer C3 (`mod.rs:435–439`) com substitution-with-fallback | S | `.B`, `.C` |
| `.E` | Tests E2E em submódulo `p184e_figure_per_kind` (~3 tests) | S | `.D` |
| `.F` | Relatório + actualização preventiva DEBT M4-residual | S | `.E` |

Sequência fixa B → C → D → E → F. Sem cláusulas condicionais.

---

## §6 Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

- Convenção chave `figure:{kind}` replica `numbering_active:heading`
  (P182A) com mesmo separador `:`.
- Método trait `figure_number_at_index` segue pattern de
  `formatted_counter_at` (P177) e `bib_number_for_key` (P181F):
  `<noun>_<position-marker>`.

### Q2 — Honestidade de magnitude

P183D §7 declarou C3 como "mais barato individualmente". P184A
confirma S agregado para B–F (~80–150 LOC):
- `.B`: ~5 LOC arm + 0–2 tests existentes adaptados.
- `.C`: ~10 LOC trait + impl + 5 tests unit (~30 LOC) + helper.
- `.D`: ~5 LOC consumer.
- `.E`: ~3 tests E2E (~30 LOC).
- `.F`: relatório.

### Q3 — Cobertura sem regressão

`kind_index[ElementKind::Figure]` tem zero consumers de produção fora
de tests (§3.4). Refinar arm Figure adiciona população `figure:{kind}`
sem remover `kind_index` ou `figure_label_numbers`. Consumers
existentes (figure-ref P168 via `figure_label_numbers`, query_by_kind
em tests) continuam a receber os mesmos dados.

### Q4 — Fechamento de C3

Critério literal: cláusula 6 (§4). DEBT M4-residual reduz de
"C1+C2+C3" para "C1+C2" via P184F (cf. §11).

### Q5 — Granularidade

5 sub-passos para passo S agregado. Cada peça é trivial:
- Refinar arm: alteração mecânica de uma chamada `apply_at`.
- Método trait: declarar + delegar.
- Migrar consumer: substitution-with-fallback.
- Tests: padrão P181F/P182B replicado.
- Relatório: padrão P181J/P182J.

---

## §7 Pré-condição confirmada

- Tests workspace baseline P183D mantido — 1.756 verdes; zero
  violations linter. Sem alterações de código em P184A.
- M9 ✅ 11/11 (cf. P182J).
- M4 parcialmente concluído via P183: C4 resolved label estado
  pendente em P183E (não corrido); C1, C2, C3 bloqueados em
  P183B/C/D.
- Trait `Introspector` 15 métodos (sem `figure_number_at_index`).

---

## §8 Magnitude consolidada

S agregado P184B–F. Ver §5 e diagnóstico §4.

---

## §9 Restrições aplicadas

Cumpridas:

- Zero código tocado em qualquer ficheiro fora de `00_nucleo/`.
- Zero testes modificados.
- Zero L0 prompts modificados.
- Não criadas reservas de identificadores.
- Não modificado `from_tags`.
- Não adicionado trait method.
- Não migrado consumer.
- Sem inflação de linguagem ("patamar", "limiar", "consolidação",
  "deriva", "subpadrão", "cumulativo", "cross-domínio", "paridade
  observable" como bandeira).
- Honestidade obrigatória: "dead code em produção" (P183D §1) é
  registado como tal — não rebaptizado. Cláusula 5 mantém legacy
  paralelo por simetria de processo, não por preservação observable.
- Sem cláusulas condicionais nos sub-passos `.B`+ do plano.

---

## §10 ADR avaliação

- **Convenção `figure:{kind}`**: replica P182A — não ADR.
- **Método trait `figure_number_at_index`**: replica P181F — não ADR.
- **Refinamento de arm existente**: Figure já é locatable (P165);
  refino, não novo locatable kind — não ADR.
- **Helper `CounterRegistry::value_at_index`**: extensão localizada de
  sub-store existente — não ADR.

**Conclusão**: P184A **não cria ADR**.

---

## §11 DEBT avaliação

P184A não abre DEBT. P184F (fecho da série P184) actualiza o DEBT
M4-residual conforme estado de P183F:

- **Se P183F já correu** (DEBT já aberto cobrindo C1+C2+C3): P184F
  edita o DEBT removendo C3 da lista (deixa C1+C2).
- **Se P183F ainda não correu**: P184F precede; quando P183F correr,
  abre DEBT cobrindo apenas C1+C2 (não C3).

Ambos os caminhos convergem para mesmo estado final: DEBT cobre
C1+C2 após P184F + P183F (em qualquer ordem).

---

## §12 Estado actual

- **P183 série**: A ✅ B ❌ C ❌ D ❌ E pendente F pendente.
- **P184 série**: A ✅ (este passo) | B–F pendentes.
- **Progresso M4**: 5 read-sites migrados; C3 desbloqueio agendado em
  P184B–F; C1+C2 esperam P185+.
- **Progresso M5/M9**: M9 ✅ 11/11.
- **38 passos executados** (P183A + P183B + P183C + P183D + P184A;
  contagem cumulativa per `typst-passo-183d-relatorio.md` §4 que
  declarava 37 → +1).

---

## §13 Actualização preventiva — DEBT M4-residual

Registo per .J output 3 da spec P184A:

P183F (passo de fecho da série P183) está pendente. Quando correr:

- **Cenário A** (P183F corre antes de P184F): DEBT M4-residual
  abre cobrendo C1+C2+C3. P184F (depois) actualiza removendo C3.
- **Cenário B** (P184F corre antes de P183F): C3 fechado em P184F;
  P183F (depois) abre DEBT cobrindo apenas C1+C2.

A spec de P183F deve ser actualizada antes de correr para reflectir
qual cenário aplica. Se ainda for redigida considerando C1+C2+C3
estática, requer aviso na própria P183F a relacionar com P184F.

---

## §14 Próximo passo — P184B

Refinar `from_tags.rs:71–95` arm Figure:

```rust
ElementPayload::Figure { kind, counter_update, is_counted, .. } => {
    intr.kind_index
        .entry(ElementKind::Figure)
        .or_default()
        .push(*loc);
    let kind_key = kind.as_deref().unwrap_or("image");
    intr.counters.apply_at(
        format!("figure:{}", kind_key),
        counter_update.clone(),
        *loc,
    );
    // Decisão P184B: manter `apply_at("figure", ...)` global em
    // paralelo (sem consumers actuais, mas simétrico com walk legacy).
    intr.counters.apply_at(
        "figure".to_string(),
        counter_update.clone(),
        *loc,
    );
    if *is_counted {
        if let Some(label) = &info.label {
            let next_num = intr.figure_label_numbers.len() + 1;
            intr.figure_label_numbers
                .entry(label.clone())
                .or_insert(next_num);
        }
    }
}
```

L0 `00_nucleo/prompts/rules/introspect/from_tags.md` documenta a
convenção `figure:{kind}` (promovida do doc comment de
`element_payload.rs:52`).

Tests existentes em `from_tags.rs:339–396` confirmados não regridem
(asserções sobre `figure_label_numbers` e `kind_index[Figure]` —
inalterados).

Critério de fecho P184B: `cargo test --workspace --lib` passa com
Δ baseline 1.756 (sem novos tests).

P184A é instrumento. Refinamento concreto da arm Figure começa em
P184B.
