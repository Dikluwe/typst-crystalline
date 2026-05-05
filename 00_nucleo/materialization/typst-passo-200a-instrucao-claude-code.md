# Passo 200A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.864 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental:
  - P189B: Outline migrado + 6 excepções declaradas.
  - P193B: sub-store `ResolvedLabelStore`.
  - P194B: consumer C4 migrado.
  - P195B-E: walk arm Labelled (E4 fechada).
  - P196B-C: walk arm Heading auto-toc (E2 → E2-residuo).
  - P197B-C: walk arm Figure (E3 fechada — cenário α).
  - P198B-D: walks SetHeadingNumbering + CounterUpdate
    (E5 + E6 fechadas).
  - P199B-C: `Content::SetEquationNumbering` materializado
    (E1 fechada — cenário α por construção).
  - **0 excepções activas + 1 residuo (E2-residuo)**.
- DEBT M5-residual: **1 pré-requisito paralelo restante**.
- Trait `Introspector`: 19 métodos.
- `TagIntrospector`: 8 sub-stores.
- `ElementPayload`: 12 variants.
- `ElementKind`: 10.
- `Content`: + 1 variant em P199B (`SetEquationNumbering`).
- Pattern ADR-0069 com **5 variantes operacionais
  consolidadas**: P195D + P196B + cenário α + cenário α
  por construção + cenário β-promote.
- 6 aplicações ADR-0069 stylesheet: P195D + P196B +
  P197B + P198B + P198C + P199B.

P200 é **passo paralelo final fora série §9 P189** —
abre sub-store `intr.headings_for_toc` para fechar
**E2-residuo** (lacuna #3 declarada desde P189B/P196B).

**Após P200 fechar**: M5 universal completo
(0 excepções activas + 0 residuos + 0 pré-requisitos
restantes). Desbloqueia M6 (P190A reescrita do zero —
eliminação `CounterStateLegacy`; magnitude L).

**Material de partida** verificado:

- `00_nucleo/materialization/typst-passo-199-relatorio-consolidado.md`
  §13 — sugestão preliminar de trabalho concreto P200
  (sub-store + variant + walk arm + from_tags + consumer
  outline).
- `00_nucleo/materialization/typst-passo-196-relatorio-consolidado.md`
  — declaração da E2-residuo + lacuna #3.
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E2 — descrição original da excepção (4 mutações).
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
  — ACEITE.
- `00_nucleo/m1-lacunas-captura.md` lacuna #3
  (`headings_for_toc`).

P200A é o passo de diagnóstico. Magnitude esperada **S**
(diagnóstico). Implementação P200B+ depende criticamente
de cláusula 1 (forma do sub-store) e cláusula 2 (forma da
variant Tag).

---

## Postura do auditor / executor

P200A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 21 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (5
  variantes ADR-0069 cobrem).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P200B+.

**Magnitude diagnóstico**: S. Decisões expandidas porque
há 3 dimensões a auditar (sub-store, variant Tag, walk
arm + consumer outline).

**Regra dos 2 eixos aplicável**:
- Eixo 1: snapshot final (consumer outline.rs:24 lê após
  walk completo).
- Eixo 2: sub-store `intr.headings_for_toc` **a abrir** —
  não existe ainda.

**Particularidade de P200**: trabalho híbrido entre
variantes ADR-0069 — abre sub-store (similar a P193B)
**+** emite Tag pós-recursão (similar a P196B variante
locatable + body) **+** migra consumer (similar a
P194B C4). 3 categorias de trabalho numa só série.

---

## Escopo

**Primário**: abrir sub-store `intr.headings_for_toc:
Vec<(Label, Content, u8)>` (ou similar) em
`TagIntrospector` para fechar **E2-residuo** — última
mutação legacy (4ª) do walk arm Heading.

**Confirmação**: validar inventário factual — sub-store
ausente, mutação 4 walk arm Heading
(`state.headings_for_toc.push((auto_label, frozen_body,
level))`) preservada desde P196B, consumer outline.rs
lê directamente do legacy.

**Decisões a tomar** — 9 cláusulas:

1. **Forma do sub-store**:
   - **Opção α**: `Vec<(Label, Content, u8)>` literal
     ao state legacy.
   - **Opção β**: struct dedicada
     `HeadingTocEntry { label, body, level }` para
     legibilidade.
   - **Opção γ**: tipo mais elaborado com sub-stores
     auxiliares (improvável — over-engineering).

2. **Variant Tag**:
   - **Opção α**: nova variant
     `ElementPayload::HeadingForToc { label, body, level }`.
     `ElementPayload`: 12 → 13.
   - **Opção β**: reusar variant existente (improvável —
     `Labelled` cobre auto-toc resolved_text mas **não
     o body** que outline precisa para renderizar entry).
   - **Opção γ**: aproveitar variant `Heading` (P170)
     adicionando campos opcionais. Improvável — quebra
     atomização.

   Sugestão preliminar: **Opção α** (variant nova).
   Justificação: outline precisa de `body` materializado
   + `level` numérico que `Labelled` não cobre.

3. **`is_locatable` vs Tag pós-recursão**:
   - HeadingForToc Tag é **derivado de Heading** —
     Heading já é locatable.
   - **Decisão**: Tag pós-recursão usando `emitted_loc`
     do walk arm Heading (variante P196B).
   - HeadingForToc **não** precisa de ser locatable
     standalone (não existe como `Content` a parsear).

4. **Walk arm Heading — onde emitir Tag HeadingForToc**:
   - Walk arm Heading em `introspect.rs` já tem
     comentário inline P196B (cenário α + ADR-0069).
   - P200B emite Tag HeadingForToc após walk recursivo
     do body (replica P196B mas com payload diferente).
   - **Decisão**: emitir Tag apenas quando `numbering`
     activo (consistente com push legacy actual).

5. **Helper `compute_heading_for_toc`**:
   - **Opção α**: helper privado análogo a
     `compute_labelled` / `compute_heading_auto_toc`.
   - **Opção β**: lógica inline no walk arm.
   
   Sugestão: **Opção α** — consistência com pattern
   ADR-0069 stylesheet. Função pura sobre
   `(state, level, body)` retornando
   `Option<(Label, Content, u8)>`.

6. **`from_tags` arm HeadingForToc**:
   - Push directo no sub-store
     `intr.headings_for_toc.push(entry)`.
   - Replica padrão P195D (Labelled) /
     P198C (CounterUpdate) — `from_tags` arm dedicada.

7. **Trait `Introspector` — método novo**:
   - `fn headings_for_toc(&self) -> &[(Label, Content,
     u8)]` ou similar.
   - **Trait passa de 19 → 20 métodos**.
   - Documentar em L0 `entities/introspector.md`.

8. **Migração consumer `outline.rs:24`**:
   - Substitution-with-fallback (padrão P184D / P194B):
     ```
     let entries = match intr.headings_for_toc() {
         entries if !entries.is_empty() => entries,
         _ => &state.headings_for_toc,
     };
     ```
   - Ou directamente sem fallback se confiança alta.
   
   Sugestão: substitution-with-fallback durante janela
   compat M5; cleanup em M6.

9. **Critério de fecho de P200**:
   - Sub-store aberto, trait method exposto, variant
     Tag, walk arm emite Tag, `from_tags` arm popula,
     consumer outline migrado, mutação legacy preservada
     como write paralelo M5.
   - **E2-residuo fecha estruturalmente**.
   - **Lacuna #3 fecha**.
   - **M5 universal fecha completamente** — 0
     excepções, 0 residuos, 0 pré-requisitos.

**Fora de escopo**:

- `Content::SetEquationNumbering` materialização —
  fechado em P199B.
- Eliminação `CounterStateLegacy` (P190A — M6).
- Eliminação `compute_*` helpers (M6).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "headings_for_toc" 01_core/src/` para
mapear estado actual (state legacy + consumer
outline.rs).

### O2 — Alternativas

Cláusula 1 tem 3 opções (forma sub-store); cláusula 2
tem 3 opções (variant Tag); cláusula 5 tem 2 opções
(helper); cláusula 8 tem 2 opções (consumer migration
forma). Demais cláusulas determinadas por regra dos 2
eixos.

### O3 — Critério de escolha

Padrão estabelecido (P193B sub-store + P196B variante
locatable Tag pós-recursão + P194B consumer
substitution-with-fallback + helper análogo `compute_*`).
Replicação directa de 3 padrões já testados.

### O4 — Magnitude

P200 implementação **M agregada**:
- **P200B** — sub-store + variant + helper + walk arm
  + `from_tags` arm + trait method + consumer +
  L0 + tests E2E. Magnitude M genuína (mais escopo
  que passos anteriores porque combina 3 categorias
  de trabalho).
- **P200C** — relatório consolidado + DEBT.
  Magnitude S puro.

Total agregado: ~150 LOC produção + ~180 LOC tests +
~100 LOC L0 + relatório ≈ **M agregado**.

### O5 — Reversibilidade

Reversível por construção (write paralelo legacy
preservado).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P200 combina 3 padrões testados:
- Sub-store novo (P193B `ResolvedLabelStore`).
- Variant Tag pós-recursão locatable (P196B).
- Consumer migration substitution-with-fallback
  (P194B).

Sem invenção arquitectural. **6ª variante operacional
ADR-0069**? Improvável — combinação de variantes
existentes.

### Q2 — Honestidade de magnitude

P200A diagnóstico é S. Implementação:
- P200B: M genuíno (mais escopo que P198C ou P199B).
- P200C: S puro.

Total agregado: M+ (limite superior do M).

### Q3 — Cobertura sem regressão

P200 mantém output observable:
- Sub-store novo activa caminho Introspector.
- Mutação 4 legacy preservada como write paralelo M5.
- Consumer outline lê via Introspector path com fallback.
- `compute_heading_auto_toc` (P196B) inalterado —
  continua a popular `intr.resolved_labels` para
  auto-toc labels.

### Q4 — E2-residuo + lacuna #3 fecham completamente

Após P200B:
- E2-residuo fecha **estruturalmente completa** — 4ª
  mutação migra para Tag pós-recursão.
- Lacuna #3 fecha.
- E2 inteira fecha (3 mutações em P196B + 1 mutação
  em P200B).
- **M5 universal completo**.

### Q5 — Granularidade

Conforme padrão P195/P196/P197/P198/P199:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Sub-store + trait method + variant `ElementPayload::HeadingForToc` + helper + walk arm + `from_tags` arm + consumer outline + L0 + tests E2E | M |
| `.C` | Relatório consolidado P200 + actualização DEBT | S |

---

## Sub-passos de P200A

### Sub-passo 200A.A — Validação do estado actual

Auditor confirma empiricamente:

#### Sub-store + trait

1. Confirmar `TagIntrospector` em
   `01_core/src/entities/tag_introspector.rs`:
   - 8 sub-stores actuais (per P198D).
   - Onde adicionar `headings_for_toc:
     Vec<(Label, Content, u8)>`.

2. Confirmar trait `Introspector` em
   `01_core/src/entities/introspector.rs`:
   - 19 métodos.
   - Onde adicionar `headings_for_toc(&self) ->
     &[(Label, Content, u8)]`.

#### Walk arm Heading

3. Confirmar walk arm Heading em
   `01_core/src/rules/introspect.rs`:
   - Per P196B: 4 mutações; comentário inline P196B
     presente; mutação 4 (`state.headings_for_toc.push`)
     activa como E2-residuo.
   - Localizar linha exacta da mutação 4.
   - Confirmar variável `emitted_loc` em scope
     (Heading locatable per P164).

4. Confirmar `compute_heading_auto_toc` (P196B):
   - Helper actual produz `(label, resolved_text)`.
   - **NÃO modificar** — P200 adiciona helper distinto.

#### Variant Tag + extract_payload + from_tags

5. Confirmar `ElementPayload` em
   `01_core/src/entities/element_payload.rs`:
   - 12 variants (após P198C).
   - Onde adicionar `HeadingForToc { label, body,
     level }`.

6. Confirmar `ElementKind` em
   `01_core/src/entities/element_kind.rs`:
   - 10 variants.
   - **Decisão obrigatória**: HeadingForToc tem
     ElementKind correspondente?
   - Per convenção cristalino (per P198C): todo
     ElementPayload locatable tem ElementKind.
   - HeadingForToc é **derivado de Heading** —
     diferente de CounterUpdate (que é Content
     standalone). Auditor decide empiricamente.

7. Confirmar `extract_payload`:
   - HeadingForToc é Tag emitida pós-recursão
     (variante P196B), **não derivada de** `Content`.
   - Sem arm em `extract_payload` necessário.

8. Confirmar `from_tags` em
   `01_core/src/rules/introspect/from_tags.rs`:
   - Onde adicionar arm `HeadingForToc`.
   - Push directo: `intr.headings_for_toc.push((label,
     body, level))`.

#### Consumer outline

9. Confirmar consumer outline em
   `01_core/src/rules/layout/outline.rs`:
   - Linha 24 (per P196B referência).
   - Lê directamente `state.headings_for_toc` legacy.
   - Onde adicionar substitution-with-fallback.

10. Confirmar tests existentes:
    - Sentinela E2-residuo P196B
      (`walk_e2_residuo_headings_for_toc_via_legacy`).
    - **Decisão importante**: este test preserva-se
      ou adapta-se? Após P200B, mutação legacy
      continua activa (write paralelo M5) — sentinela
      continua válida; pode até reforçar-se.
    - Tests outline existentes — verificar não
      regridem.

#### L0 alvos

11. Confirmar L0 alvos:
    - `entities/tag_introspector.md` — sub-store novo.
    - `entities/introspector.md` — método novo (trait
      passa de 19 → 20).
    - `entities/element_payload.md` — variant nova.
    - (eventualmente) `entities/element_kind.md` —
      variant nova.
    - `rules/introspect.md` — walk arm Heading
      actualização (E2-residuo → fecha).
    - `rules/layout/outline.md` (se existir) —
      consumer migration.

12. Aplicar regra dos 2 eixos:
    - Eixo 1: snapshot final (consumer outline.rs:24
      lê após walk completo).
    - Eixo 2: sub-store `intr.headings_for_toc`
      populated em produção via Tag::HeadingForToc
      após P200B.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Sub-store `headings_for_toc` confirmado ausente.
- Walk arm Heading mutação 4 (E2-residuo) localizada.
- Consumer outline.rs:24 localizado.
- API trait + ElementPayload prontos para extensão.
- Convenção `ElementKind::HeadingForToc` decidida.

### Sub-passo 200A.B — Decisão cláusula 1 (forma sub-store)

Conforme `.A.1`:

**Opção α** (preferida) — `Vec<(Label, Content, u8)>`
literal a state legacy. Tipo simples; sem necessidade
de struct dedicada.

**Opção β** — `Vec<HeadingTocEntry>` com struct dedicada
`{ label: Label, body: Content, level: u8 }`. Mais
legível; magnitude marginalmente maior (struct nova).

**Opção γ** — improvável. Over-engineering.

Sugestão preliminar: **Opção α** (literal).

Output: forma fixada per decisão empírica em `.A`.

### Sub-passo 200A.C — Decisão cláusula 2 (variant Tag)

Conforme `.A.5`:

**Opção α** (preferida) — variant nova
`ElementPayload::HeadingForToc { label, body, level }`.
`ElementPayload`: 12 → 13. Justificação: outline
precisa de `body` + `level` que `Labelled` não cobre.

**Opção β** — reusar `Labelled`. Improvável — falta
campo `body` materializado.

**Opção γ** — reusar `Heading`. Improvável — quebra
atomização (Heading já cobre snapshot original).

Sugestão preliminar: **Opção α**.

Output: variant fixada.

### Sub-passo 200A.D — Decisão cláusula 3 (`is_locatable`)

HeadingForToc é Tag derivada de Heading — não é
`Content` a parsear. Sem `is_locatable` arm
necessário.

Tag pós-recursão usa `emitted_loc` do walk arm
Heading (variante P196B). Mesma Location que Heading
+ Tag::Labelled auto-toc P196B.

Output: decisão fixada — sem arm `is_locatable`
novo.

### Sub-passo 200A.E — Decisão cláusula 4 (walk arm Heading)

Per `.A.3`:

P200B modifica walk arm Heading (após P196B):
- Mutações 1-3 preservadas (write paralelo M5).
- Mutação 4 (`state.headings_for_toc.push`) **continua
  preservada** (write paralelo M5).
- Adicionar **3ª Tag emit pós-recursão** após Tag
  Labelled auto-toc P196B:
  ```
  if let Some(loc) = emitted_loc {
      let entry = compute_heading_for_toc(state, *level, ...);
      if let Some((label, body, level)) = entry {
          tags.push(Tag::Start(loc, ElementInfo::new(
              ElementPayload::HeadingForToc { label, body, level },
          )));
          tags.push(Tag::End(loc, 0));
      }
  }
  ```

Mesma Location que Heading + Labelled auto-toc.
Sub-stores diferentes — sem conflito (per P196A
§11.5).

Output: estrutura fixada.

### Sub-passo 200A.F — Decisão cláusula 5 (helper)

**Opção α** (preferida) — helper privado:

```
fn compute_heading_for_toc(
    state: &CounterStateLegacy,
    level: u8,
    body:  &Content,
) -> Option<(Label, Content, u8)> {
    if !state.is_numbering_active("heading") {
        return None;
    }
    let auto_label = Label(
        format!("auto-toc-{}", state.auto_label_counter),
    );
    let frozen_body = body.clone(); // ou materialize_time
    Some((auto_label, frozen_body, level))
}
```

Forma exacta replica P196B `compute_heading_auto_toc`
mas com retorno diferente (3-tupla em vez de 2-tupla).

**Opção β** — lógica inline. Adiciona inconsistência
com pattern ADR-0069 stylesheet.

Sugestão: **Opção α** — consistência.

**Cláusula gate substancial**: forma exacta de
`frozen_body` deve replicar literal a actual mutação
4 walk arm Heading (provável `materialize_time(body,
state)` ou similar — confirmar empiricamente).

Output: helper fixado.

### Sub-passo 200A.G — Decisão cláusula 6 (`from_tags` arm)

Per `.A.8`:

Adicionar arm em `from_tags.rs`:

```
ElementPayload::HeadingForToc { label, body, level } => {
    intr.headings_for_toc.push((
        label.clone(),
        body.clone(),
        *level,
    ));
    // Possivelmente:
    // intr.kind_index.entry(ElementKind::HeadingForToc)
    //     .or_default().push(*loc);
}
```

Decisão sobre `kind_index` depende de `.A.6`.

Output: arm fixada.

### Sub-passo 200A.H — Decisão cláusula 7 (trait method)

Adicionar método em trait `Introspector`:

```
fn headings_for_toc(&self) -> &[(Label, Content, u8)];
```

Implementação em `TagIntrospector`:

```
fn headings_for_toc(&self) -> &[(Label, Content, u8)] {
    &self.headings_for_toc
}
```

**Trait passa de 19 → 20 métodos**. Documentar em L0.

Output: assinatura fixada.

### Sub-passo 200A.I — Decisão cláusula 8 (consumer outline)

Per `.A.9`:

**Opção α** (preferida) — substitution-with-fallback:

```
// outline.rs:24 (aproximadamente)
let entries: Vec<&(Label, Content, u8)> =
    if !intr.headings_for_toc().is_empty() {
        intr.headings_for_toc().iter().collect()
    } else {
        state.headings_for_toc.iter().collect()
    };
```

Forma exacta depende da estrutura do código actual.
Janela compat M5; fallback raramente disparado em
produção; cleanup em M6.

**Opção β** — directamente sem fallback. Risco
elevado se Tag emit falha — sem backup.

Sugestão: **Opção α**.

Output: forma fixada.

### Sub-passo 200A.J — Decisão cláusula 9 (critério de fecho)

P200 fecha quando:
- Sub-store `headings_for_toc` aberto em
  `TagIntrospector`.
- Trait method `headings_for_toc(&self)` exposto.
- Variant `ElementPayload::HeadingForToc` adicionada.
- Helper `compute_heading_for_toc` criado.
- Walk arm Heading emite Tag pós-recursão (3ª Tag
  além de Heading + Labelled auto-toc P196B).
- `from_tags` arm popula sub-store.
- Consumer outline migrado (substitution-with-fallback).
- Tests E2E confirmam paridade observable + activação
  Introspector path.
- **E2-residuo fecha estruturalmente completa**.
- **Lacuna #3 fecha**.
- **M5 universal completo** — 0 excepções activas + 0
  residuos + 0 pré-requisitos.

**Marco arquitectural** — primeira vez M5 universal
completo desde declaração em P189B.

Output: critério literal verificável.

### Sub-passo 200A.K — Validação do plano de sub-passos

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Sub-store + trait method + variant + helper + walk arm + `from_tags` arm + consumer outline + L0 + tests E2E | M genuíno |
| `.C` | Relatório consolidado P200 + actualização DEBT M5-residual + marco M5 universal | S |

Total agregado: ~150 LOC produção + ~180 LOC tests +
~100 LOC L0 + relatório consolidado ≈ **M+
agregado** (limite superior do M devido a 3 categorias
de trabalho combinadas).

Output: tabela final.

### Sub-passo 200A.L — ADR

Avaliar:

- 5 variantes ADR-0069 cobrem.
- Trabalho híbrido (sub-store + Tag + consumer) é
  combinação de variantes existentes, não nova
  variante operacional.

Conclusão: **não cria ADR**.

### Sub-passo 200A.M — DEBT

P200 fecha **E2-residuo + lacuna #3** completamente.

DEBT M5-residual após P200B+:
- Antes: 0 excepções activas + 1 residuo (E2-residuo);
  1 pré-requisito restante.
- Após: **0 excepções activas + 0 residuos + 0
  pré-requisitos**.

**M5 universal estado**: completo. Todos arms walk
puro fechados estruturalmente (P189B Outline + P181H
Bibliography + P195D Labelled + P196B+P200B Heading +
P197B Figure + P198B SetHeadingNumbering + P198C
CounterUpdate + P199B SetEquationNumbering).

**Cenário B continua** (sem DEBT formal aberto).

**DEBT M6**: write paralelo M5 ainda activo —
mutações legacy em todos walk arms preservadas;
`compute_*` helpers leem legacy. Cleanup orgânico em
M6 (P190A reescrita do zero).

Output: estado actualizado.

### Sub-passo 200A.N — Outputs

Produzir 3 ficheiros (padrão P181A–P199A):

1. **`00_nucleo/diagnosticos/diagnostico-headings-for-toc-passo-200a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–9 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Cadeia E2-residuo + interacção com
     `compute_heading_auto_toc` P196B — análise
     empírica.
   - §8 Próximo sub-passo (P200B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-200a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P200B+.
- **Não tocar `from_tags`** — P200B+.
- **Não modificar trait `Introspector`** — P200B+.
- **Não modificar `TagIntrospector`** — P200B+.
- **Não modificar consumer outline** — P200B+.
- **Não modificar `compute_heading_auto_toc`** (P196B)
  — continua independente.
- **Não materializar parser sintáctico**.
- **Não materializar P190A** — aguarda M5 universal
  fechar empiricamente após P200 série.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos**.
- **Reaproveitar pattern ADR-0069 + 5 variantes
  operacionais**.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-headings-for-toc-passo-200a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-200a-relatorio.md`
  com 14 secções produzido.
- 9 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (2 sub-passos B
  + C).
- Magnitude consolidada confirmada empiricamente
  (M+ agregado).
- Critério de fecho P200 fixado (E2-residuo + lacuna
  #3 fecham; M5 universal completo).
- ADR avaliada (esperado: não criada).
- DEBT M5-residual estado registado (0 excepções + 0
  residuos + 0 pré-requisitos após P200B+).
- Cadeia E2-residuo analisada empiricamente.
- Variant `ElementPayload::HeadingForToc` decidida.
- Trait method `headings_for_toc` decidido (trait 19
  → 20).
- Consumer outline migration decidido.
- Regra dos 2 eixos aplicada empiricamente.
- Pattern ADR-0069 reaproveitado.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.864 inalterados.
- `crystalline-lint .` zero violations.

P200A é instrumento. Implementação concreta de P200B
é trabalho híbrido combinando 3 padrões testados.

**Após P200 série fechar**: **M5 universal completo
pela primeira vez desde declaração em P189B**. 7
séries materializadas + 6 excepções fechadas + 1
residuo fechado + 2 pré-requisitos paralelos
materializados.

**Marco arquitectural significativo** — desbloqueia
M6 (P190A reescrita do zero — eliminação
`CounterStateLegacy`; magnitude L cross-modular).
