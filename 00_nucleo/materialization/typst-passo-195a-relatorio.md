# Relatório P195a — Diagnóstico walk arm Labelled migration

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico); P195B+ implementação **M**.
**Pré-condição**: P194B fechado; tests workspace 1.825
verdes; zero violations.

---

## §1 Escopo

P195A é o passo de diagnóstico-primeiro que precede a
migração walk arm Labelled. Replica registo de
P181A/.../P194A.

P195 é **passo 3 da sequência §9 P189 consolidado** —
**primeira migração walk arm M5**, mais arquitectural que
P193+P194 (infra + consumer).

P195A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-walk-labelled-passo-195a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-195a-relatorio.md` (este, 14 secções).

**Achado central crítico**: walk arm Labelled depende de
state mutado durante walk recursivo do target (counters
+ lang). `extract_payload` puro não pode replicar lógica.
**Opção 1 padrão (locatable + extract_payload)
impossível** sem refactor major.

Decisão arquitectural nova → **ADR PROPOSTO em P195B**
("Post-recursion tag emission for state-dependent
payload").

---

## §2 Inputs verificados empiricamente

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Walk arm Labelled actual | `introspect.rs:432-486` — recursão + match target type para texto resolvido |
| 2 | `Content::Labelled` variant | `{ target: Box<Content>, label: Label }` |
| 3 | `resolved_text` Heading | `state.format_hierarchical("heading")` durante walk |
| 4 | `resolved_text` Equation block | `state.get_flat("equation")` durante walk |
| 5 | `resolved_text` Figure | `state.figure_numbers + state.lang + figure_supplement_for_lang` |
| 6 | `state.lang` field | `Option<Lang>` em `CounterStateLegacy` |
| 7 | `intr.figure_label_numbers` | já populated por P168 arm Figure (write paralelo) |
| 8 | `intr.resolved_labels` | aberto P193B; vazio em produção |
| 9 | Independência walk Labelled vs Heading auto-toc | confirmada empiricamente |
| 10 | Eixo 1 (consumer C4) | snapshot final ✅ |
| 11 | Eixo 2 (sub-store) | sub-store P193B pronto; populate via Tag em P195 |

Bloqueador arquitectural identificado em §1.2 do
diagnóstico:
> `extract_payload(content: &Content) -> Option<ElementPayload>`
> é função pura (sem state). Não pode replicar lógica que
> depende de `state.format_hierarchical`,
> `state.figure_numbers`, `state.lang`.

---

## §3 Decisões cláusulas 1–7 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma do payload | **Opção 1-modificada** — variant `ElementPayload::Labelled` mas walk emite Tag manualmente pós-recursão (sem extract_payload arm; sem `is_locatable=true`) |
| 2 | Ordem | Não aplicável (sem janela invariante quebrada) |
| 3 | Estrutura | **Opção α** estrutura única `{ label, resolved_text, figure_number: Option<usize> }` |
| 4 | Auto-toc vs explicit | Independência confirmada; P195 fecha apenas E4 (Labelled explicit); E2 (Heading auto-toc) fica para P196 |
| 5 | resolved_text computation | walk arm computa (replica lógica actual); Tag carrega texto pré-computed |
| 6 | figure_label_numbers | `from_tags` arm Labelled popula ambos sub-stores (write paralelo redundante com P168) |
| 7 | Critério fecho | E4 fecha **estruturalmente** (Introspector activa); funcionalmente em M6 |

---

## §4 Plano de sub-passos B–E (sem condicionais)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Variant `ElementPayload::Labelled` + L0s + tests unit + stub no-op `from_tags` + **ADR PROPOSTO** | S |
| `.C` | Estender `from_tags` arm Labelled com populate completo | S |
| `.D` | Walk arm Labelled emite Tag pós-recursão (mantém mutação legacy paralela) + helper `compute_labelled` | M |
| `.E` | Tests E2E paridade + activação Introspector path + relatório consolidado P195 | S |

Total agregado: ~150-250 LOC produção + ~150 LOC tests +
ADR + relatório consolidado ≈ **M**.

---

## §5 Magnitude agregada

**P195 série = M** (4 sub-passos S+S+M+S).

Maior que P186 (M em 6 sub-passos) porque P195 introduz
**pattern arquitectural novo** (post-recursion emit) por
sub-passo.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- Sub-store `ResolvedLabelStore` aberto (P193B).
- Trait method `resolved_label_for` disponível (P193B).
- Consumer C4 migrado com substitution-with-fallback
  (P194B).
- `intr.figure_label_numbers` populated por P168 arm
  Figure (write paralelo).

### §6.2 — Dependentes

- **E4 fecha estruturalmente após P195B**: Introspector
  path activa para explicit labels.
- **Heading auto-toc (E2) ainda activa** — só fecha em
  P196 quando walk arm Heading migrar para popular
  sub-store via Tag para chave `auto-toc-N`.
- M5 universal fecha após P195+P196+P197+P198 +
  passo independente SetEquationNumbering.

### §6.3 — Independente

- Sub-store `headings_for_toc` (lacuna #3).
- `Content::SetEquationNumbering`.

---

## §7 ADR avaliação — **PROPOSTO**

**ADR PROPOSTO** será criada em P195B `.B` — primeira ADR
desde P185 (ADR-0068).

**Pattern**: "Post-recursion tag emission for
state-dependent payload".

**Justificação**: walk arm Labelled é o primeiro caso
identificado onde payload depende de state mutado durante
walk recursivo. `extract_payload` (pre-recursion + pure
function) não suporta. P195 introduz pattern alternativo:
walk arm emite Tag manualmente em `tags` após recursão.

**Numeração ADR**: a determinar em P195B `.B` per
inventário de `00_nucleo/adr/`.

---

## §8 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. P195 fecha **E4 estruturalmente**
(introduz caminho Introspector); funcionalmente em M6.

Excepções E1, E2, E3, E5, E6 continuam activas após P195.

DEBT M5-residual continua em Cenário B com 2 pré-requisitos
restantes (`headings_for_toc`, `SetEquationNumbering`).

---

## §9 Restrições honradas

- **Zero código tocado**.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica walk** — P195B+.
- **Não toca `from_tags`** — P195B+.
- **Não modifica trait `Introspector`** — P185B fechou.
- **Não modifica `TagIntrospector`** — P193B fechou.
- **Não modifica consumer C4** — P194B fechou.
- **Não migra walk arm Heading** — P196.
- **Não migra walk arm Figure** — P197.
- **Sem inflação retórica**.
- **Aplicar regra dos 2 eixos** ✅ — §1.3 do diagnóstico.
- **Aprendizado P186C/D**: aplicado mas não-relevante
  porque `is_locatable` não muda — sem janela
  invariante quebrada.
- **Sem cláusulas condicionais** nos sub-passos.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.825** inalterado
  vs P194B.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ⚠️ ADR PROPOSTO **será criada em P195B** (não em
  P195A — diagnóstico não materializa ADR).

---

## §11 Achados não-triviais

### §11.1 — Bloqueador arquitectural crítico (extract_payload puro)

`extract_payload(&Content) -> Option<ElementPayload>` é
**função pura** sem acesso a state. Walk arm Labelled
depende de:
- `state.format_hierarchical("heading")` — mutado em walk
  recursivo do target Heading.
- `state.get_flat("equation")` — mutado em walk recursivo
  do target Equation.
- `state.figure_numbers` — mutado em walk recursivo do
  target Figure.
- `state.lang` — state field.

**Opção 1 padrão impossível**. Decisão fixada: Opção 1-modificada
com walk arm emite Tag manualmente após recursão.

### §11.2 — Pattern arquitectural novo

Pattern existente em cristalino:
- **Pre-recursion + pure**: extract_payload chamado em
  walk top antes do match arm; payload puro.
- **Pre-recursion + payload simples**: Bibliography (clone
  entries), Equation (block + counter_update),
  SetHeadingNumbering (StateUpdate).

Pattern novo P195:
- **Post-recursion + state-dependent**: walk arm emite
  Tag manualmente em `tags` após recursão; payload
  pré-computed usando state.

ADR PROPOSTO documenta. Aplicabilidade futura: outros
walk arms cujo payload depende de state.

### §11.3 — `figure_label_numbers` é populated paralelamente por P168 + P195

P168 arm Figure popula `intr.figure_label_numbers` quando
`is_counted == true` E `info.label.is_some()`. Walk arm
Labelled chama `walk(target, ..., Some(label))` propagando
label via `label_from_parent`.

P195 arm Labelled também popula `intr.figure_label_numbers`
quando `figure_number.is_some()`. **Write paralelo
redundante** — ambos arms produzem mesmo resultado.
Aceitável; cleanup em M6.

### §11.4 — E4 fecha estruturalmente, não funcionalmente

P195 introduz caminho Introspector mas **mantém mutação
legacy paralela** durante janela compat:

```rust
// Walk arm Labelled (após P195):

// Mutação legacy preservada (write paralelo).
state.figure_label_numbers.insert(label.clone(), n);
state.resolved_labels.insert(label.clone(), text.clone());

// Tag emit manual (P195B).
tags.push(Tag::Start(loc, ElementInfo::new(payload)));
tags.push(Tag::End(loc, 0));
```

**E4 fecha estruturalmente** — Introspector path activa.
**E4 fecha funcionalmente em M6** quando mutação legacy
for removida.

Padrão idêntico a P181 Bibliography (manteve mutação
legacy até P181H corrected).

### §11.5 — Heading auto-toc continua a usar fallback legacy

P195 fecha apenas E4 (Labelled explicit). E2 (Heading
auto-toc) continua a popular `state.resolved_labels[auto-toc-N]`
directamente.

Após P195:
- Consumer C4 (P194B) recebe `Some(text)` do Introspector
  path para **explicit labels** (cobertos por P195).
- Consumer C4 recebe `None` do Introspector + `Some(text)`
  do fallback legacy para **auto-toc-N labels**
  (cobertos só em P196).

**Inversão observable parcial** — explicit labels
migram para path Introspector; auto-toc continua legacy.

### §11.6 — Walk arm helper `compute_labelled`

P195D introduz helper privado:

```rust
fn compute_labelled(
    target: &Content,
    state:  &CounterStateLegacy,
    _label: &Label,
) -> (Option<String>, Option<usize>) {
    match target {
        Content::Heading { .. } => (
            state.format_hierarchical("heading").map(|n| format!("Secção {}", n)),
            None,
        ),
        Content::Equation { block, .. } if *block => {
            let n = state.get_flat("equation");
            if n > 0 { (Some(format!("Equação ({})", n)), None) }
            else     { (None, None) }
        }
        Content::Figure { kind, numbering, caption, .. } => {
            // ... (lógica existente)
        }
        _ => (None, None),
    }
}
```

Isolamento da lógica facilita reuso entre mutação legacy
e populate Tag. Reduz duplicação.

---

## §12 Snapshot pós-P195A

- **Tests workspace**: 1.825 (inalterado).
- **Trait `Introspector`**: 19 métodos.
- **`TagIntrospector` sub-stores**: 8.
- **DEBT M5-residual**: 2 pré-requisitos (inalterado).
- **65 passos executados** (P194B = 64 + P195A = 65).
- **Padrão diagnóstico-primeiro**: 17ª aplicação
  consecutiva.

---

## §13 Próximo passo

**P195B** — adicionar variant `ElementPayload::Labelled` +
ADR PROPOSTO:

1. Editar `01_core/src/entities/element_payload.rs`:
   - Variant nova `Labelled { label, resolved_text,
     figure_number: Option<usize> }`.
   - Tests unit.

2. Editar L0 `00_nucleo/prompts/entities/element_payload.md`.

3. Adicionar stub no-op em `from_tags` arm Labelled
   (cláusula gate trivial).

4. **Sem** `ElementKind::Labelled` (não-locatable).

5. **Sem** `is_locatable` arm.

6. **Sem** `extract_payload` arm.

7. Criar ADR PROPOSTO
   `00_nucleo/adr/typst-adr-XXXX-post-recursion-tag-emission.md`
   (número a determinar).

Magnitude: S puro.

---

## §14 Conclusão

P195A fechou 7 cláusulas com decisão literal e plano em
4 sub-passos B–E. Magnitude S agregada para diagnóstico;
implementação P195B+ é **M** com pattern arquitectural
novo.

Achados centrais:
- **Bloqueador arquitectural crítico**: `extract_payload`
  puro impossibilita Opção 1 padrão. Decisão pragmática:
  walk emite Tag manualmente pós-recursão.
- **Pattern arquitectural novo**: ADR PROPOSTO documenta.
- **E4 fecha estruturalmente em P195** (não
  funcionalmente — janela compat preservada).
- **Cadeia E1-E6 continua activa** — P195 fecha apenas
  E4. P196+P197+P198 + SetEquationNumbering ainda
  necessários.

P195 é **passo 3 dos 7** da sequência §9 P189 consolidado.
Diferente dos P193 (infra) e P194 (consumer) — primeira
migração walk arm com decisão arquitectural nova
documentada por ADR.

Padrão diagnóstico-primeiro mantido — 17/17 acertaram a
magnitude planeada ±1 nível (P195A acerta S; implementação
M é declarada explicitamente).

Próximo passo: **P195B** — variant nova + ADR PROPOSTO.
