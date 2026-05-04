# Diagnóstico — Walk arm Heading auto-toc migration (Passo P196a)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico); P196B+ implementação **M agregado**.
**ADR vinculada**: nenhuma (reuso pattern ADR-0069 ACEITE
em P195E).
**Pré-condição**: P195E concluído ✅; ADR-0069 ACEITE;
sub-store `intr.resolved_labels` populated por P195D para
explicit labels.

---

## §1 Validação do estado actual

### §1.1 — Walk arm `Content::Heading` (introspect.rs:347-379)

```rust
Content::Heading { level, body } => {
    state.step_hierarchical("heading", *level as usize);

    state.auto_label_counter += 1;
    let auto_label = Label(format!("auto-toc-{}", state.auto_label_counter));

    let resolved_text = if state.is_numbering_active("heading") {
        state.format_hierarchical("heading")
            .map(|prefix| format!("Secção {}", prefix))
            .unwrap_or_default()
    } else {
        String::new()
    };
    state.resolved_labels.insert(auto_label.clone(), resolved_text);

    let frozen_body = materialize_time(body, state);
    state.headings_for_toc.push((auto_label, frozen_body, *level as usize));

    walk(body, state, locator, tags, None);
}
```

**4 mutações empíricas** (E2 P189B):
1. `state.step_hierarchical("heading", *level)`.
2. `state.auto_label_counter += 1`.
3. `state.resolved_labels.insert(auto_label, resolved_text)`.
4. `state.headings_for_toc.push((auto_label, frozen_body, level))`.

### §1.2 — `Content::Heading` variant

`{ level: u8, body: Box<Content> }`. Sem field `label`
directo. Labels associadas via wrapper `Content::Labelled`
(coberto por P195D).

### §1.3 — Auto-label format

`Label(format!("auto-toc-{}", state.auto_label_counter))`.
Counter incrementa sequencialmente; cada Heading recebe
chave única.

### §1.4 — Resolved text computation

```rust
if state.is_numbering_active("heading") {
    state.format_hierarchical("heading")
        .map(|prefix| format!("Secção {}", prefix))
        .unwrap_or_default()
} else {
    String::new()
}
```

- Numbering activa + counter populated → `"Secção 1.2.3"`.
- Numbering activa + counter vazio → `""` (unwrap_or_default).
- Numbering inactiva → `""`.

**Sempre String** (nunca None). Legacy insere mesmo quando
empty.

### §1.5 — `is_locatable(Content::Heading)`

`true` (P164 baseline). Walk top emite Tag::Start para
Heading com `locator.next()`. `emitted_loc: Some(loc)`
acessível em arm via match arm scope.

Diferente de Labelled (não-locatable): P195D usou
snapshot+find_map. P196 pode usar `emitted_loc` directo
— mais simples.

### §1.6 — `ElementPayload::Labelled` (P195B) cobre semanticamente

Campos:
- `label: Label` — pode ser `Label("auto-toc-N")`.
- `resolved_text: Option<String>` — texto auto-toc.
- `figure_number: Option<usize>` — `None` para Heading.

`from_tags` arm Labelled (P195C) popula
`intr.resolved_labels` quando `resolved_text.is_some()`.
Funciona directamente para auto-toc — sem variant nova
necessária.

### §1.7 — Walk arm Labelled (P195D) wrapping Heading

Cenário `Content::Labelled { target: Heading, label:
explicit }`:
- Walk arm Labelled chama `walk(target=Heading, ...)`.
- Walk arm Heading (P196 pós-implementação) emite Tag
  auto-toc com chave `auto-toc-N`.
- Walk arm Labelled (P195D) emite Tag explicit com chave
  `explicit_label`.
- `from_tags` processa **ambas as Tags** sequencialmente
  → ambas as keys populadas em `intr.resolved_labels`.
- Consumer C4 resolve qualquer das chaves.

**Sem conflito** — auto-toc-N e explicit_label são
chaves distintas.

### §1.8 — Sub-store `intr.resolved_labels` (P193B)

`HashMap<Label, String>` aceita qualquer Label.
`auto-toc-N` é válido. Sem necessidade de adaptar
sub-store.

### §1.9 — Consumer C4 (P194B)

`references.rs:53-67` consulta
`intr.resolved_label_for(target)` com fallback legacy.
Após P196 activa para auto-toc labels também — segunda
inversão observable real.

### §1.10 — Lacuna #3 confirmada — `headings_for_toc`

Field `pub headings_for_toc: Vec<(Label, Content, usize)>`
em `counter_state_legacy.rs:42`. Consumer:
`layout/outline.rs:24` em `layout_outline`. Sub-store
`intr.headings_for_toc` **NÃO existe**.

Mutação 4 da E2 **continua activa após P196** —
**E2-residuo**. Documentação 4 pontos planeada para
P196B.

### §1.11 — `emitted_loc` em walk fn scope

```rust
fn walk(content, state, locator, tags, label_from_parent) {
    let emitted_loc = if let Some(payload) = do_extract_payload(content) {
        let loc = locator.next();
        ...
        Some(loc)
    } else {
        None
    };

    match content {
        ...
        Content::Heading { level, body } => {
            // emitted_loc acessível aqui (Location é Copy).
            // ...
        }
        ...
    }
}
```

Para Heading (locatable), `emitted_loc` é sempre `Some(loc)`.
P196B reusa esta `loc` para Tag auto-toc — sem snapshot
nem find_map.

### §1.12 — Análise dos 2 eixos

| Eixo | Análise | Conclusão |
|------|---------|-----------|
| **Eixo 1** (semântica temporal) | Consumer C4 lê após walk completo. Snapshot final. | Sem variante `*_at`. |
| **Eixo 2** (existência dados) | Sub-store `intr.resolved_labels` populated por P195C arm Labelled; aceita auto-toc-N. | Possível em P196. |

---

## §2 Decisões cláusulas 1–7

### §2.1 — Cláusula 1: forma do payload

**Decisão fixada**: **Opção 1** — reusar
`ElementPayload::Labelled` (P195B).

Justificação:
- Variant existe e cobre semanticamente (per §1.6).
- `from_tags` arm Labelled (P195C) já popula
  `intr.resolved_labels` directamente — funciona para
  auto-toc-N.
- Sem ADR nova; sem variant nova.
- Magnitude S (apenas walk arm modification + helper).
- Replicação literal de pattern ADR-0069 (P195D).

Auto-toc-N é **caso particular** de resolved label sem
figure_number. Mesma estrutura, mesmo sub-store.

### §2.2 — Cláusula 2: helper `compute_heading_auto_toc`

**Decisão fixada**: helper privado análogo a
`compute_labelled` (P195D §11.6).

```rust
fn compute_heading_auto_toc(
    state:        &CounterStateLegacy,
    auto_label_n: u64,
) -> (Label, String) {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    let resolved_text = if state.is_numbering_active("heading") {
        state
            .format_hierarchical("heading")
            .map(|prefix| format!("Secção {}", prefix))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (auto_label, resolved_text)
}
```

Função pura sobre `(state, counter)`. Sem mutação. Replica
lógica legacy do walk arm Heading.

**Nota**: helper recebe `auto_label_n` (u64 actual do
counter) já incrementado pelo caller. Mantém helper puro
sem mutar state.

### §2.3 — Cláusula 3: chave `auto_label`

**Decisão fixada**: `Label(format!("auto-toc-{}", N))` —
paridade literal com legacy.

### §2.4 — Cláusula 4: `headings_for_toc` residual

**Decisão fixada**: manter mutação como **E2-residuo**
durante janela compat M5.

Documentação 4 pontos (replica P189B pattern):
1. **Comentário inline** no walk arm:
   ```
   // E2-residuo: state.headings_for_toc.push(...) continua
   // activo porque sub-store `intr.headings_for_toc` não
   // existe (lacuna #3). Fecha em passo dedicado quando
   // sub-store for aberto.
   ```
2. **L0 `rules/introspect.md`** secção "Excepções M5"
   actualizada — E2 → E2-residuo.
3. **Test sentinela** que valida `state.headings_for_toc`
   continua populated (paridade observable preservada).
4. **Secção em P196 consolidado** §"E2-residuo".

Cross-reference a passo dedicado para abrir sub-store
`headings_for_toc`. Magnitude esperada para esse passo:
S (replica P193B).

### §2.5 — Cláusula 5: Locator handling

**Decisão fixada**: **reuso directo de `emitted_loc`**
(Opção a — mais simples que P195D).

Justificação:
- Heading é locatable; walk top já emitiu
  `Tag::Start(emitted_loc, heading_payload)` antes do
  arm correr.
- `emitted_loc: Option<Location>` está em scope no arm.
- Para Heading, `emitted_loc` é sempre `Some(loc)`.
- Reuso preserva sincronização ADR-0068:
  - Walk Locator: avança 1 para Heading (já feito).
  - Layouter Locator: avança 1 para Heading (locatable).
  - Sequências sincronizadas.

```rust
if let Some(loc) = emitted_loc {
    tags.push(Tag::Start(loc, ElementInfo::new(payload)));
    tags.push(Tag::End(loc, 0));
}
```

Diferente de P195D que precisava de snapshot+find_map
(Labelled é não-locatable; sem `emitted_loc`).

### §2.6 — Cláusula 6: mutação legacy preservada

**Decisão fixada**: write paralelo durante janela compat
M5. Replica P195D pattern.

3 mutações ganham write paralelo Tag:
1. `state.step_hierarchical("heading", *level)` — counter
   mutation; preserved.
2. `state.auto_label_counter += 1` — counter mutation;
   preserved.
3. `state.resolved_labels.insert(auto_label, text)` —
   write paralelo; preserved + Tag emit P196B.

1 mutação **sem Tag equivalent**:
4. `state.headings_for_toc.push(...)` — E2-residuo
   (lacuna #3).

Cleanup orgânico em M6 (P190) para 1+2+3; mutação 4
fecha quando sub-store `headings_for_toc` for aberto.

### §2.7 — Cláusula 7: critério de fecho de P196

**Decisão fixada**:

P196 fecha quando:
1. Walk arm Heading emite Tag auto-toc pós-recursão.
2. Helper `compute_heading_auto_toc` materializado.
3. Mutação legacy preservada paralela (3 mutações).
4. Mutação `headings_for_toc.push` continua activa
   (E2-residuo) com 4 pontos de documentação.
5. `from_tags` arm Labelled (P195C) processa Tag →
   popula `intr.resolved_labels[auto-toc-N]`.
6. Tests E2E confirmam:
   - Paridade observable preservada.
   - Consumer C4 (P194B) recebe `Some(text)` para
     auto-toc labels.
7. **E2 fecha 3 das 4 mutações estruturalmente**.
   E2-residuo continua para passo dedicado.

---

## §3 Plano de sub-passos

**Sub-passos B–C** (sem condicionais):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Heading + helper `compute_heading_auto_toc` + Tag emit pós-recursão + mutação legacy preservada + tests E2E + L0 `rules/introspect.md` actualizado + comentário inline + 4 pontos documentação E2-residuo | M |
| `.C` | Relatório consolidado P196 + actualização nota DEBT M5-residual | S |

Total agregado P196B–C: ~80 LOC walk arm + ~50 LOC helper
+ ~150 LOC tests + edits L0 + relatório consolidado ≈
**M agregado**.

---

## §4 Magnitude consolidada

P196 série: **M agregado** (1×M + 1×S = 2 sub-passos).

Maior que P194 (S agregado em sub-passo único) porque
P196 muda walk arm + introduz helper. Igual a P186/P195
em magnitude per sub-passo principal.

Pattern ADR-0069 já estabelecido reduz incerteza face a
P195A — P196 é **segunda aplicação concreta**.

---

## §5 ADR avaliação

**Sem ADR criada.** Pattern ADR-0069 PROPOSTO em P195B,
ACEITE em P195E, **reusado em P196 sem decisão
arquitectural nova**.

Diferenças P196 vs P195D:
- Heading locatable (vs Labelled não-locatable).
- Locator handling: `emitted_loc` directo (vs snapshot+find_map).
- 3 das 4 mutações migradas (vs 2 mutações em P195D).

Estas diferenças são **detalhes de aplicação**, não
decisão arquitectural nova.

---

## §6 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada per P195 §8:

> **Antes P196**: 5 excepções activas (E1, E2, E3, E5,
> E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P196B**: **4 excepções activas + 1 resíduo**:
> - E1 — Reserva 1 (`SetEquationNumbering` ausente).
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3).
> - E3 — Figure (chained com E2 cadeia agora aberta).
> - E5 — SetHeadingNumbering (chained — pode fechar
>   isoladamente após P196).
> - E6 — CounterUpdate (chained com E2 cadeia agora
>   aberta).
>
> **2 pré-requisitos restantes** (`headings_for_toc`,
> `SetEquationNumbering`).
>
> Cadeia E2 estruturalmente desbloqueada para E3, E5, E6.
> P197/P198 podem prosseguir com pattern ADR-0069
> aplicado.

DEBT M5-residual continua em Cenário B.

---

## §7 E2-residuo: documentação 4 pontos para `headings_for_toc.push`

P196 fecha **3 das 4 mutações** de E2 estruturalmente.
Mutação 4 (`state.headings_for_toc.push(...)`) **continua
activa** porque sub-store `intr.headings_for_toc` ausente
(lacuna #3).

### Ponto 1 — Comentário inline em walk arm Heading

P196B adiciona após mutações 1-3 e antes de mutação 4:

```rust
// E2-residuo (lacuna #3): state.headings_for_toc.push
// continua activo porque sub-store `intr.headings_for_toc`
// não existe. Fecha em passo dedicado quando sub-store
// for aberto. Vide P196 consolidado §"E2-residuo".
state.headings_for_toc.push(...);
```

### Ponto 2 — L0 `rules/introspect.md` Excepções M5

Secção "Excepções M5" actualizada:
- E2 → **E2-residuo**: 1 mutação activa
  (`headings_for_toc.push`); 3 mutações fechadas
  estruturalmente em P196.
- Pré-requisito: sub-store `intr.headings_for_toc`
  (passo dedicado paralelo).

### Ponto 3 — Test sentinela

`walk_e2_residuo_headings_for_toc_via_legacy`:
- Documento com 3 Headings.
- Pipeline normal.
- Assert `state.headings_for_toc.len() == 3` (mutação
  legacy preservada).
- Assert que cada entry tem `(auto_label, frozen_body, level)`.

### Ponto 4 — Secção §7 do P196 consolidado

Esta secção do diagnóstico replica para o relatório
consolidado P196 com lista de pontos cumpridos.

---

## §8 Próximo sub-passo

**P196B** — walk arm Heading auto-toc + helper + Tag emit:

1. Editar `01_core/src/rules/introspect.rs`:
   - Adicionar helper privado `compute_heading_auto_toc`.
   - Modificar walk arm Heading:
     - Manter 4 mutações legacy (paridade preservada).
     - Após `walk(body, ...)` recursão: emitir Tag
       auto-toc usando `emitted_loc` reusado +
       `ElementPayload::Labelled` com `auto-toc-N` key.
   - Comentário inline E2-residuo per ponto 1.

2. Editar L0 `00_nucleo/prompts/rules/introspect.md`:
   - Secção Excepções M5 actualizada (E2 → E2-residuo).
   - Histórico relevante.

3. Tests E2E em submódulo `p196b_walk_heading`:
   - `heading_auto_toc_walk_emite_tag_e_popula_introspector`.
   - `heading_auto_toc_paridade_legacy_vs_introspector`.
   - `heading_auto_toc_numbering_inactivo_emite_string_vazia`.
   - `walk_e2_residuo_headings_for_toc_via_legacy`.
   - `consumer_c4_recebe_some_para_auto_toc_label`.

4. Actualizar nota DEBT M5-residual no relatório
   consolidado P196 (passo `.C`).

Magnitude: **M genuíno**. Sem cláusulas condicionais.
