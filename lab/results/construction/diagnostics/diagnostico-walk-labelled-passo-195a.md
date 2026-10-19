# Diagnóstico — Walk arm Labelled migration (Passo P195a)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico); P195B+ implementação **M**.
**ADR vinculada**: **ADR PROPOSTO** — "Post-recursion tag
emission for state-dependent payload" (decisão
arquitectural nova; sem precedente cristalino directo).
**Pré-condição**: P194B fechado; consumer C4 migrado;
sub-store `ResolvedLabelStore` aberto.

---

## §1 Validação do estado actual + análise dos 2 eixos

### §1.1 — Walk arm `Content::Labelled` (introspect.rs:432-486)

```rust
Content::Labelled { target, label } => {
    walk(target, state, locator, tags, Some(label));  // recursão primeiro

    let resolved_text = match &**target {
        Content::Heading { .. } => state
            .format_hierarchical("heading")
            .map(|n| format!("Secção {}", n)),
        Content::Equation { block, .. } if *block => {
            let n = state.get_flat("equation");
            if n > 0 { Some(format!("Equação ({})", n)) } else { None }
        }
        Content::Figure { kind, numbering, caption, .. } => {
            let kind_key = kind.as_deref().unwrap_or("image");
            let n = if numbering.is_some() && caption.is_some() {
                state.figure_numbers.get(kind_key).and_then(|v| v.last()).copied().unwrap_or(0)
            } else { 0 };
            if n > 0 {
                state.figure_label_numbers.insert(label.clone(), n);
                let supplement = figure_supplement_for_lang(kind_key, state.lang.as_ref());
                Some(format!("{} {}", supplement, n))
            } else {
                Some(String::new())
            }
        }
        _ => None,
    };
    if let Some(text) = resolved_text {
        state.resolved_labels.insert(label.clone(), text);
    }
}
```

**Mutações**:
- `state.figure_label_numbers.insert(label, n)` (linha
  468; condicional para Figures numbered+captioned).
- `state.resolved_labels.insert(label, text)` (linha 484).

**Dependências de state durante walk**:
- `state.format_hierarchical("heading")` — counter
  mutado em walk arm Heading (E2 chained).
- `state.get_flat("equation")` — counter mutado em walk
  arm Equation (E1 chained).
- `state.figure_numbers` — counter mutado em walk arm
  Figure (E3 chained).
- `state.lang: Option<Lang>` — state field externo
  populated por eval (não-walk).

### §1.2 — Bloqueador arquitectural crítico

`extract_payload(content: &Content) -> Option<ElementPayload>`
é **função pura** (sem parâmetro state). Não pode
replicar a lógica de walk que depende de:
- Counter mutado durante walk recursivo do target.
- `state.lang` field.

**Consequência**: **Opção 1 padrão (extract_payload arm
puro)** — **impossível sem refactor major**.

Soluções viáveis:
1. **Opção 1-modificada**: payload nova + walk emite
   Tag manualmente após recursão (bypass extract_payload).
   Pattern novo.
2. **Opção 2 (StateUpdate)**: walk emite Tag
   `ElementPayload::StateUpdate {key: "resolved_label:{label}",
   update: Set(Str(text))}`. Inconsistência — popula
   `StateRegistry` mas consumer C4 lê
   `intr.resolved_labels` (sub-store P193B).
3. **Opção 3 (variant não-locatable)**: payload nova mas
   `is_locatable=false`. Sem precedente.

### §1.3 — Análise dos 2 eixos

| Eixo | Análise | Conclusão |
|------|---------|-----------|
| **Eixo 1** (semântica temporal) | Consumer C4 lê durante layout (após walk completo). Snapshot final. | Sem variante `*_at`. |
| **Eixo 2** (existência dados) | Sub-store `intr.resolved_labels` aberto P193B; populate possível via Tag. | Possível em P195. |

Bloqueador é estrutural (extract_payload puro) não
temporal — sub-store está pronto; só precisa de mecanismo
diferente para popular.

### §1.4 — Independência empírica de auto-toc Heading (E2)

Walk arm Labelled é **independente** do walk arm Heading
auto-toc. Heading auto-toc gera label `auto-toc-N` e
popula `state.resolved_labels[auto-toc-N]` directamente.
Labelled wraps target arbitrário (Heading/Figure/Equation/etc)
e popula `state.resolved_labels[user-label]`.

P195 migra apenas E4 (Labelled). E2 (Heading auto-toc)
fica para P196 — chave `auto-toc-N` continua a ser
mutada legacy directamente.

### §1.5 — Sub-stores prontos

- `intr.resolved_labels: ResolvedLabelStore` (P193B) —
  aberto, vazio em produção.
- `intr.figure_label_numbers: HashMap<Label, usize>`
  (P168) — populated por `from_tags` arm Figure quando
  `is_counted`.

P195 adiciona populate de `intr.resolved_labels` via Tag
Labelled. `figure_label_numbers` já é populated por arm
Figure existente — duplicação de populate em arm Labelled
seria redundante.

**Importante**: P168 `from_tags` arm Figure só popula
`figure_label_numbers` se `is_counted == true` E
`info.label.is_some()`. O label vem de `info.label`
(propagado via `label_from_parent` no walk recursivo).
Walk arm Labelled passa `Some(label)` para `walk(target,
state, locator, tags, Some(label))` — confirmado.

Logo `intr.figure_label_numbers` **já está populated**
por arm Figure existente quando target é Figure. P195 NÃO
precisa de duplicar este populate.

**Walk arm Labelled muta `state.figure_label_numbers`
(legacy)** — mas isto é write paralelo redundante com
P168 que popula `intr.figure_label_numbers`. **E4
mutação `figure_label_numbers` pode ser removida em P195
sem regressão** (consumer C4 não lê este field; consumer
P168 figure-ref já lê de `intr.figure_label_numbers`).

### §1.6 — Pattern arquitectural novo

P195 introduz **post-recursion tag emission** — walk arm
empurra Tag manualmente em `tags` após recursão do target.
Sem precedente:
- P181D Bibliography: extract_payload puro (entries clone).
- P186C Equation: extract_payload puro (block, counter_update).
- P182C SetHeadingNumbering: extract_payload puro
  (`numbering_active:heading` via StateUpdate).
- **P195 Labelled**: walk arm emite Tag manualmente
  pós-recursão (resolved_text depende de state).

Decisão arquitectural nova → **ADR PROPOSTO**.

---

## §2 Decisões cláusulas 1–7

### §2.1 — Cláusula 1: forma do payload

**Decisão fixada**: **Opção 1-modificada** — variant
`ElementPayload::Labelled { label, resolved_text,
figure_number: Option<usize> }` mas:
- `is_locatable(Content::Labelled) = false` (mantido).
- `extract_payload` arm retorna `None` (não-puro;
  payload depende de state).
- Walk arm Labelled emite Tag **manualmente** em `tags`
  após recursão.

**Justificação**: Opção 1 padrão impossível (extract_payload
puro sem state). Opção 2 (StateUpdate) inconsistente com
sub-store P193B. Opção 3 (variant não-locatable) é
exactamente esta — Opção 1-modificada formaliza.

### §2.2 — Cláusula 2: ordem de implementação

Sem janela invariante quebrada porque `is_locatable` NÃO
muda. Sem aprendizado P186C/D aplicável.

Plano linear sem riscos de ordem.

### §2.3 — Cláusula 3: estrutura do payload

**Decisão fixada**: **Opção α** — estrutura única:

```rust
ElementPayload::Labelled {
    label: Label,
    resolved_text: String,
    figure_number: Option<usize>,
}
```

Replica P186 ElementPayload::Equation (carrega múltiplos
fields). `figure_number` é `None` excepto quando target
é Figure numbering+captioned.

**Nota sobre `figure_number`**: redundante com P168
populate de `intr.figure_label_numbers` (per §1.5). Mas
incluímos no payload para auto-suficiência — `from_tags`
arm Labelled pode popular ambos sub-stores
consistentemente sem depender da ordem dos arms.

### §2.4 — Cláusula 4: auto-label vs explicit confirmado

**Confirmado empiricamente**: walk arm Labelled é
**independente** de walk arm Heading auto-toc (per §1.4).
P195 migra apenas E4 (Labelled explicit). E2 (Heading
auto-toc) fica para P196.

Após P195: `intr.resolved_labels` populated **apenas para
explicit labels**. `auto-toc-N` keys continuam a vir de
fallback legacy (até P196).

### §2.5 — Cláusula 5: resolved_text computation

**Decisão fixada**: walk arm computa `resolved_text`
(replica lógica actual em §1.1). Tag carrega texto
**pré-computed** no payload.

Implementação:
```rust
Content::Labelled { target, label } => {
    walk(target, state, locator, tags, Some(label));

    // P195B: computar resolved_text + figure_number
    // (lógica replicada de write legacy actual).
    let (resolved_text, figure_number) = compute_labelled(target, state, label);

    // Manter mutação legacy (write paralelo durante
    // janela compat — E4 estruturalmente fechada;
    // funcionalmente em M6).
    if let Some(n) = figure_number {
        state.figure_label_numbers.insert(label.clone(), n);
    }
    if let Some(text) = &resolved_text {
        state.resolved_labels.insert(label.clone(), text.clone());
    }

    // P195B: emit Tag manualmente após recursão.
    if let Some(text) = resolved_text {
        let loc = locator.next();
        tags.push(Tag::Start(
            loc,
            ElementInfo::new(ElementPayload::Labelled {
                label: label.clone(),
                resolved_text: text,
                figure_number,
            }),
        ));
        tags.push(Tag::End(loc, 0));  // hash não usado
    }
}
```

### §2.6 — Cláusula 6: figure_label_numbers populate

**Decisão fixada**: `from_tags` arm Labelled popula
**ambos** `intr.resolved_labels` e
`intr.figure_label_numbers` (se `figure_number.is_some()`).

Auto-suficiência do payload garante consistência sem
depender de ordem dos arms em `from_tags`. P168 arm
Figure continua a popular `figure_label_numbers` para
labels via wrapping `label_from_parent`. P195 arm
Labelled adiciona populate **paralelo** — write redundante
mas inofensivo.

### §2.7 — Cláusula 7: critério de fecho de P195

**Decisão fixada**:

P195 fecha quando:
1. Variant `ElementPayload::Labelled` adicionada.
2. Walk arm Labelled emite Tag manualmente após recursão
   (com payload pré-computed).
3. Walk arm Labelled **mantém** mutação legacy
   `state.resolved_labels` e `state.figure_label_numbers`
   (write paralelo durante janela compat).
4. `from_tags` arm Labelled popula `intr.resolved_labels`
   e `intr.figure_label_numbers`.
5. Tests E2E confirmam:
   - Paridade observable preservada.
   - Consumer C4 (P194B) começa a receber `Some(text)`
     do Introspector path para explicit labels (inversão
     observable parcial).
   - Heading auto-toc continua a usar fallback legacy
     (E2 ainda activa).
6. **E4 estruturalmente fechada** — Introspector path
   activa parcialmente.
7. **E4 funcionalmente fecha em M6** quando mutação
   legacy for removida.

Excepções E1, E2, E3, E5, E6 continuam activas.

---

## §3 Plano de sub-passos B–E (sem condicionais)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar variant `ElementPayload::Labelled` + L0s + tests unit. **Sem** `ElementKind::Labelled` (não-locatable). **Sem** `is_locatable` arm. **Sem** `extract_payload` arm. Adicionar stub no-op em `from_tags` (cláusula gate trivial — match exhaustivo força arm explícito). | S |
| `.C` | Estender stub `from_tags` arm Labelled com populate completo de `intr.resolved_labels` + `intr.figure_label_numbers` | S |
| `.D` | Walk arm Labelled emite Tag manualmente após recursão (mantém mutação legacy paralela). Helper `compute_labelled(target, state, label) -> (Option<String>, Option<usize>)` para isolar lógica. | M |
| `.E` | Tests E2E paridade + activação Introspector path + relatório consolidado P195 | S |

Total agregado P195B–E: ~150-250 LOC produção + ~150
LOC tests + edits L0 + relatório consolidado ≈ **M**.

---

## §4 Magnitude consolidada

P195 série: **M agregado** (4 sub-passos S+S+M+S).

Maior que P186 (também M agregado mas em 6 sub-passos).
P195 é mais arquitectural por sub-passo devido a pattern
novo (post-recursion emit).

---

## §5 ADR avaliação

**ADR PROPOSTO criada em P195B `.B`** — "Post-recursion
tag emission for state-dependent payload".

Conteúdo proposto:
- **Contexto**: walk arms cujo payload depende de state
  mutado durante walk recursivo.
- **Decisão**: pattern alternativo a extract_payload
  (pre-recursion + pure function). Walk arm emite Tag
  manualmente em `tags` após recursão. ElementPayload
  carrega texto pré-computed.
- **Aplicabilidade**: Labelled (P195). Outros candidatos
  futuros: ?
- **Trade-offs**:
  - + permite payload state-dependent.
  - − walk arm não passa pelo gating uniforme; bypass
    `extract_payload`.
  - − padrão menos canônico; pode confundir leitores.
- **Estado**: PROPOSTO em P195B; ACEITE quando P195F
  consolidar.

---

## §6 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada per P194 §8:

> Antes P195: 2 pré-requisitos pendentes para fechar
> excepções restantes (E1, E2, E3, E5, E6).
>
> **Após P195B**: 2 pré-requisitos restantes
> (`headings_for_toc`, `SetEquationNumbering` —
> inalterados; P195 não avança esses).
>
> **E4 fecha estruturalmente** (Introspector path activa
> para explicit labels).
> **E4 fecha funcionalmente em M6** (mutação legacy
> removida).
> **E1, E2, E3, E5, E6 continuam activas**:
> - E1: Reserva 1 (SetEquationNumbering).
> - E2: depende sub-store `headings_for_toc`.
> - E3: chained com E2.
> - E5: chained com E2.
> - E6: chained com E2.

DEBT M5-residual continua em Cenário B.

---

## §7 Janela invariante quebrada — não aplicável

P195 **não promove** Labelled a locatable
(`is_locatable` não muda). Walk top-level continua a tratar
Labelled como não-locatable. Sub-store `intr.resolved_labels`
populated via Tag emitida manualmente pelo walk arm
após recursão.

Ao contrário de P186 (Equation promovido a locatable
com janela invariante quebrada entre P186C/D), P195
preserva invariante `is_locatable ↔ extract_payload.is_some()`
porque ambos retornam `false`/`None` para Content::Labelled.

**Sem ordem corrigida necessária**. Plano linear
B→C→D→E.

---

## §8 Próximo sub-passo

**P195B** — adicionar variant `ElementPayload::Labelled`:

1. Editar `01_core/src/entities/element_payload.rs`:
   - Adicionar variant `Labelled { label: Label,
     resolved_text: String, figure_number: Option<usize> }`.
   - Tests unit (variant construível, equality, hash
     distinto).

2. Editar L0 `00_nucleo/prompts/entities/element_payload.md`:
   - Entrada nova + Histórico.

3. **Sem** `ElementKind::Labelled` (não-locatable).

4. **Sem** alteração em `is_locatable` (continua false).

5. **Sem** alteração em `extract_payload` (continua
   catch-all → None).

6. Adicionar stub no-op em `from_tags` para
   `ElementPayload::Labelled` (cláusula gate trivial —
   match exhaustivo força arm explícito; replica P186B
   pattern). P195C estende.

7. ADR PROPOSTO criada em
   `00_nucleo/adr/typst-adr-XXXX-post-recursion-tag-emission.md`
   (numero a determinar).

Magnitude: S puro. Sem cláusulas condicionais.
