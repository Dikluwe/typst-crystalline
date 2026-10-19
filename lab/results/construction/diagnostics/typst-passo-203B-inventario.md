# Inventário interno P203B — desalinhamento Figure (C1+C2)

**Data**: 2026-05-05.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-203B.md`.
**Natureza**: diagnóstico interno (factos empíricos +
decisão de caminho). C1 produz factos; C2 fixa caminho.

---

## §1 C1 — Inventário empírico do desalinhamento

### Tabela 1 — 3 arms canónicos

| Arm | Ficheiro:linha | Condição/gate | Tratamento de `kind` |
|-----|----------------|---------------|----------------------|
| **walk** | `01_core/src/engine/introspect.rs:866` | (ausente) | (ausente — arm puro) |
| **extract_payload** | `01_core/src/engine/introspect/extract_payload.rs:27` | `is_counted: numbering.is_some() && caption.is_some()` | `kind: kind.clone()` (preserva `Option<String>` literal) |
| **populate_intr_from_tag_start** | `01_core/src/engine/introspect.rs:527-559` | `if *is_counted { ... }` | `let kind_key = kind.as_deref().unwrap_or("image");` |

### Detalhe — walk arm Figure (introspect.rs:866-890)

```rust
Content::Figure { body, caption, kind: _, numbering: _ } => {
    // P190H (M6 categoria Figures eliminada): mutações walk
    // arm Figure ELIMINADAS (Opção α `.D`):
    // - state.local_figure_counters.entry(...) += 1.
    // - state.figure_numbers.entry(...).push(...).
    // - Helper compute_figure removido (orphan).
    walk(body, locator, tags, intr, auto_label_counter, lang, None);
    if let Some(cap) = caption {
        walk(cap, locator, tags, intr, auto_label_counter, lang, None);
    }
}
```

**Walk arm Figure está PURO** desde P190H (M6 fechado).
Apenas desce em body + caption. **Não usa `kind` nem
`numbering`**. **Não aplica `unwrap_or("image")`**. **Não
incrementa contador**.

### Detalhe — extract_payload.rs:27-34

```rust
Content::Figure { kind, numbering, caption, .. } => Some(ElementPayload::Figure {
    kind:           kind.clone(),
    counter_update: CounterUpdate::Step,
    is_counted:     numbering.is_some() && caption.is_some(),
}),
```

`extract_payload`:
- Preserva `kind: Option<String>` literal (não aplica
  default).
- Deriva `is_counted` correctamente
  (`numbering.is_some() && caption.is_some()`).

### Detalhe — populate_intr_from_tag_start (introspect.rs:527-559)

```rust
ElementPayload::Figure { kind, counter_update, is_counted, .. } => {
    intr.kind_index.entry(ElementKind::Figure).or_default().push(loc);
    if *is_counted {
        let kind_key = kind.as_deref().unwrap_or("image");
        intr.counters.apply_at(format!("figure:{}", kind_key), counter_update.clone(), loc);
        intr.counters.apply_at("figure".to_string(), counter_update.clone(), loc);
        if let Some(label) = &info.label {
            let next_num = intr.figure_label_numbers.len() + 1;
            intr.figure_label_numbers.entry(label.clone()).or_insert(next_num);
        }
    }
}
```

`populate_intr_from_tag_start`:
- Aplica gate `is_counted` (linha 541).
- Aplica default `kind.as_deref().unwrap_or("image")`
  (linha 542).
- Aplica `counter_update` em ambas chaves: `figure:{kind}`
  e `figure` global.
- Popula `figure_label_numbers` quando há label.

### Detalhe — `from_tags` arm Figure: NÃO EXISTE

Pesquisa `grep -n "Content::Figure\|ElementPayload::Figure"
01_core/src/engine/introspect/from_tags.rs` retorna 0
matches.

**Não há arm `Figure` em `from_tags::from_tags`**.
População da informação Figure ocorre durante walk via
`populate_intr_from_tag_start` (helper chamado por
`walk()` quando emite `Tag::Start`). Esta é a forma
canónica desde P191B/C (ADR-0071).

---

## §2 Achado central — divergência face à premissa do spec

A spec P203B §1 declarou:

> - Walk arm usa `kind.as_deref().unwrap_or("image")` para
>   contador (default fallback).
> - from_tags arm para `Figure` ou usa o mesmo default ou
>   omite o gate `is_counted`, criando divergência.

**Ambas as afirmações são empíricamente falsas**:

1. Walk arm Figure **não usa** `unwrap_or("image")`. Foi
   eliminado em P190H. Walk arm está puro.
2. **Não há `from_tags` arm para Figure**. População
   acontece durante walk via `populate_intr_from_tag_start`
   (P191B/C ADR-0071).

A premissa do spec reflecte a arquitectura **pré-P190H**
(antes de M6 fechar). Pós-P190H/P191C a arquitectura é:
- Walk arm Figure puro.
- `populate_intr_from_tag_start` aplica gate +
  default consistentemente.
- Nenhum desalinhamento empírico existe entre walk e
  populate.

**Registo**: `P203B.div-1`.

---

## §3 C2 — Caminho fixado

Per spec P203B §6:

> Em caso de divergência empírica relevante face a P203A
> (ex: walk arm já não usar `unwrap_or("image")`), registar
> em `P203B.div-N` e:
> - Re-executar C1 com os valores actuais.
> - Re-fixar C2 com os novos dados.

C1 re-executado nas secções §1-§2. **Re-fixar C2 com os
novos dados**:

### Caminhos originais do spec

- **Caminho A — Alinhar `from_tags` ao walk**: aplicar o
  mesmo default + gate em `from_tags`.
  - **REJEITADO**: não há `from_tags` arm Figure; a
    população é em `populate_intr_from_tag_start` que já
    aplica default + gate consistentemente.
- **Caminho B — Alinhar walk ao extract_payload**: walk
  delega a `extract_payload`.
  - **REJEITADO**: walk arm Figure já é puro
    (P190H); nada para "alinhar".

### Caminho fixado — Caminho C (revisado)

**Caminho C — Confirmação (sem alteração de código)**:
lacunas #1 e #1b já estruturalmente fechadas por
P190H + P191C. Trabalho concreto P203B:

1. Adicionar **um** test E2E consolidado que cobre os 4
   casos canónicos do spec C3 (formalização do fecho).
2. Correcção administrativa: snapshot §7 + §13;
   auditoria delta P201 §2 anotação retroactiva;
   historiograma cirúrgico (caso necessário — verificado
   sem matches relevantes).
3. **Zero alterações em código produção**.

### Justificação

**A. Pipeline empírica actual já resolve as lacunas**:

- Tag preserva `kind: Option<String>` literal (não
  default) — atende ao critério "Tag distingue None de
  Some('image')" da lacuna #1.
- Counter `figure:{kind_key}` usa default `"image"`
  consistentemente em populate + reads
  (`compute_labelled` Figure arm linha 400 também aplica
  `unwrap_or("image")`) — atende ao critério "consumers
  resolvem para chave canónica".
- Gate `is_counted` aplicado em populate (linha 541) —
  atende à lacuna #1b.

**B. Trabalho de "alinhamento" de código seria
redundante** porque o alinhamento já existe.

**C. Spec P203B previu este caso** em §6 ("registar em
`P203B.div-N`"); per §10 ("cada passo começa com
inventário empírico"), C1 deve ditar C2 — mesmo quando
empírico contradiga premissa do spec.

---

## §4 Plano de implementação literal (Caminho C)

### Passo 1 — Test E2E consolidado

Adicionar em `01_core/src/engine/introspect.rs` (módulo
`tests`) um único test:

```
#[test]
fn p203b_lacuna_1_e_1b_fecho_formal_4_casos() {
    // 4 casos: kind=None±caption; kind=Some±caption.
    // Asserções:
    // (a) extract_payload preserva kind literal + is_counted derivado.
    // (b) populate_intr aplica gate is_counted + default unwrap_or("image").
    // (c) Walk emite Tags consistentes (kind preservado).
}
```

### Passo 2 — Verificação

```
cargo test -p typst-core --lib p203b_lacuna_1_e_1b_fecho_formal_4_casos
```

Critério: passa verde.

```
cargo test --workspace
```

Critério: tests baseline + 1 (1823 → 1824).

```
crystalline-lint .
```

Critério: 0 violations.

### Passo 3 — Correcção administrativa

- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §7:
  reescrever tabela lacunas com nomenclatura empírica
  correcta; marcar #1 e #1b como fechadas P203B.
- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §13:
  reescrever bloco "Lacunas residuais" no resumo nova
  sessão.
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  §2: bloco de correcção retroactiva no início da
  secção.
- `00_nucleo/diagnosticos/historiograma-passos.md`:
  edições cirúrgicas (verificado — sem matches
  relevantes; nenhuma edição necessária).

### Passo 4 — Outputs P203B

- Este ficheiro (inventário interno).
- `00_nucleo/materialization/typst-passo-203B-relatorio.md`
  (relatório padrão).

---

## §5 Cláusulas C3-C9 instanciadas

### C3 — Tests

**Decisão**: 1 test consolidado cobrindo os 4 casos
(kind=None±caption × kind=Some±caption). Não duplicar
tests existentes — formalização explícita do fecho.

Tests pré-existentes que já cobrem casos parciais:
- `introspect_figure_kind_none_resolve_para_image_no_counter`
  (linha 1750): caso 2 (kind=None + caption).
- `figure_walk_caminho_introspector_ja_activo` (2871):
  caso 3 (kind=Some + caption).
- `figure_paridade_introspector_pos_p190h` (2927): caso 3
  variante (kind="table").
- `figure_numbering_inactivo_nao_popula_intr` (2945):
  caso 4 (kind=Some + sem caption).

**Caso 1 (kind=None + sem caption)**: novo neste test.

### C4 — Correcção snapshot §7

Tabela reescrita. Position deslocada para "concerns
ortogonais não-catalogados".

### C5 — Correcção snapshot §13

Bloco "Lacunas residuais" reescrito; bloco "Concerns
ortogonais" adicionado.

### C6 — Anotação auditoria delta P201 §2

Bloco de correcção retroactiva adicionado no início da
secção. Conteúdo histórico preservado abaixo.

### C7 — ADR para P203B

**Decisão final**: **não criar ADR**. P203B resolve
divergência de nomenclatura administrativa + formaliza
fecho estrutural já existente. Nenhum mecanismo novo.
Caminho C escolhido — não justifica micro-ADR.

### C8 — Correcção historiograma

Verificação cirúrgica em
`historiograma-passos.md`: nenhum match para "Position"
ou "Counter at locations" como nomenclatura de lacuna.
**Sem edições necessárias**.

### C9 — Critério de fecho

- ✅ Tests workspace: 1823 → 1824 (+1 P203B test).
- ✅ Crystalline-lint: 0 violations.
- ✅ Walk e populate_intr consistência confirmada
  empíricamente em test (per C3).
- ✅ Snapshot §7 e §13 reescritos.
- ✅ Auditoria delta P201 §2 anotada.
- ✅ Historiograma cirurgicamente verificado (sem
  edições necessárias).
- ✅ Lacunas #1/#1b marcadas fechadas estruturalmente
  P190H/P191C; formalização P203B.

---

## §6 Referências

- `00_nucleo/materialization/typst-passo-203B.md` (spec).
- `00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`.
- `00_nucleo/diagnosticos/typst-passo-203A-diagnostico.md`.
- `01_core/src/engine/introspect.rs:714, 866-890, 527-559,
  377-422` (walk fn + Figure walk arm + populate_intr +
  compute_labelled).
- `01_core/src/engine/introspect/extract_payload.rs:27-34`
  (extract_payload Figure arm).
- `01_core/src/engine/introspect/from_tags.rs` (sem arm
  Figure — confirmado).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` (lacunas
  canónicas).
- `00_nucleo/materialization/typst-passo-200-relatorio-consolidado.md` §7.
- `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md`
  (P190H eliminação write paralelo).
- `00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md`
  (P191B/C populate_intr_from_tag_start).
