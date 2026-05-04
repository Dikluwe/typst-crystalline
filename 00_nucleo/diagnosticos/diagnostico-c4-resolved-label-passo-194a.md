# Diagnóstico — C4 resolved label migration (Passo P194a)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação P184D/P187B/P188B).
**Pré-condição**: P193B fechado; sub-store `ResolvedLabelStore`
aberto; trait `resolved_label_for` disponível.

---

## §1 Validação do estado actual

### §1.1 — Site C4 (`references.rs:53`)

```rust
pub(super) fn layout_ref<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>, target: &Label) {
    use crate::entities::introspector::Introspector;

    // P168: tentar introspector primeiro (caminho M5+).
    if let Some(fig_num) = layouter.introspector.figure_number_for_label(target) {
        layouter.layout_content(&Content::text(format!("Figura {}", fig_num)));
        return;
    }
    // Fallback legacy: caller via `layout()` legacy não populou introspector.
    if let Some(&fig_num) = layouter.counter.figure_label_numbers.get(target) {
        layouter.layout_content(&Content::text(format!("Figura {}", fig_num)));
        return;
    }
    let display_text = match layouter.counter.resolved_labels.get(target) {
        Some(text) => text.clone(),
        None       => format!("@{}", target.0),
    };
    layouter.layout_content(&Content::text(display_text));
}
```

C4 site = match na linha 53 sobre `layouter.counter.resolved_labels.get(target)`:
- Tipo: `Option<&String>` (legacy HashMap).
- `Some(text)` arm: `text.clone()` → `String`.
- `None` arm: `format!("@{}", target.0)` → `String`.
- Tipo final: `String`.

### §1.2 — Acesso a `layouter.introspector` confirmado

`layouter: &mut Layouter<M, S>` (parâmetro). Linha 44 já
acede `layouter.introspector.figure_number_for_label(target)`
sem qualificação adicional. **Cenário α** (acesso directo).
Sem necessidade de getter ou cláusula gate trivial.

### §1.3 — Copy-sites Layouter (`mod.rs:1481, 1512`)

Análise empírica:

```rust
// mod.rs:1481 (short-circuit path)
l.introspector             = introspector;
l.counter.resolved_labels  = initial_state.resolved_labels;

// mod.rs:1512 (fixpoint loop)
l.introspector             = introspector.clone();
l.counter.resolved_labels  = initial_state.resolved_labels.clone();
```

**Layouters secundários têm `introspector` próprio**
(set independentemente; clonado em fixpoint loop). Cópia
de `state.resolved_labels` para `l.counter.resolved_labels`
**continua necessária** porque:

1. Em P194, sub-store `intr.resolved_labels` está vazio
   (P193B abre infra; populate em P195+).
2. Fallback legacy `layouter.counter.resolved_labels.get(target)`
   é caminho funcional.
3. Copy-sites populam o `counter.resolved_labels` que o
   fallback consulta.

**Decisão (cláusula 3 abaixo)**: **manter copy-sites**
durante janela compat M5. Remoção em M6 quando
`CounterStateLegacy.resolved_labels` for eliminado.

### §1.4 — Tipo retorno actual: `String`

Caller `layouter.layout_content(&Content::text(display_text))`
recebe `String`. Migração preserva tipo.

### §1.5 — API `resolved_label_for` (P193B)

`fn resolved_label_for(&self, label: &Label) -> Option<&str>`
— retorna referência. Para conversão a `String`:
- `.map(String::from)` ou `.to_string()` no resultado.

### §1.6 — Tests existentes

- `layout_resolved_labels_nao_interfere_entre_documentos`
  (`tests.rs:908-948`).
- Tests P189B sentinela E2/E4 (`walk_excepcao_e2_*`,
  `walk_excepcao_e4_*`).

Todos devem manter-se inalterados após P194:
- Sub-store vazio em produção (até P195).
- Fallback legacy chamado.
- Output idêntico ao actual.

### §1.7 — Aplicação regra dos 2 eixos

- **Eixo 1**: consumer C4 lê durante layout (após walk
  completo). Snapshot final é suficiente — confirma decisão
  P193A §1.8 (sem variante `*_at`).
- **Eixo 2**: dados estarão presentes em produção após
  P195+. Em P194, sub-store fica vazio → fallback legacy
  carrega.

### §1.8 — Estado temporário (não permanente)

Diferente de C2 (P188B onde Introspector é dormente
permanente até `SetEquationNumbering`), C4 dorme **apenas
até P195+P196**:

- **P194**: Introspector vazio em produção.
- **P195** (walk Labelled migrado): Tag emitida; arm
  `from_tags` popula sub-store.
- **P196** (walk Heading migrado): auto-toc populado.
- Após P195+P196: Introspector é caminho funcional;
  fallback redundante mas mantido durante janela compat
  até M6.

---

## §2 Decisões cláusulas 1–6

### §2.1 — Cláusula 1: forma da expressão

**Decisão fixada**: **Opção C** — match com `or_else`
propagando `Option<&str>`:

```rust
// P194B: substitution-with-fallback location-aware.
// Introspector path activa após P195 (walk Labelled
// migrated); até lá, sub-store vazio → fallback legacy
// é caminho funcional. Vide sequência §9 P189 consolidado.
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

**O1**: §1.1-1.5 (tipo retorno String; API retorna `&str`;
legacy retorna `&String`).

**O2**:
- Opção A inline com `.map(|s| s.to_string()).or_else(...)`:
  rejeitada — clone duplicado nos dois braços.
- Opção B `unwrap_or_else` chain: rejeitada — match exterior
  é mais explícito.
- **Opção C**: aceite — `Option<&str>` propagado, único
  `to_string()` no `Some` arm.

**O3**: P184D padrão `or_else` chain. Opção C é variante
mais elegante quando ambos paths produzem `&str`.

**O4 — Magnitude**: trivial. ~5 LOC mudança.

**O5 — Reversibilidade**: ALTA.

### §2.2 — Cláusula 2: tratamento `None` do Introspector

**Decisão fixada**: **Opção A** — aceitar comportamento
sem alteração de lógica. Comentário leve em §2.5.

Em produção até P195: Introspector retorna sempre `None`
→ fallback legacy chamado consistentemente. Output
preservado por construção.

### §2.3 — Cláusula 3: copy-sites Layouter

**Decisão fixada**: **manter** copy-sites em
`mod.rs:1481, 1512` durante janela compat M5.

Per §1.3 análise: fallback legacy depende destes copy-sites.
Remoção dispararia regressão funcional. Cleanup em M6
quando `CounterStateLegacy.resolved_labels` for eliminado.

P194B **não toca** `mod.rs:1481, 1512`.

### §2.4 — Cláusula 4: acesso a `layouter.introspector`

**Cenário α confirmado** — acesso directo. Linha 44 já
usa o pattern.

### §2.5 — Cláusula 5: documentação inline

**Decisão fixada**: **Opção B** — comentário curto:

```rust
// P194B: substitution-with-fallback location-aware.
// Introspector path activa após P195 (walk Labelled
// migrated); até lá, sub-store vazio → fallback legacy
// é caminho funcional. Vide sequência §9 P189 consolidado.
```

Mais leve que P188B (que era permanente). Suficiente para
leitores entenderem porque há fallback.

### §2.6 — Cláusula 6: critério de fecho de P194

**Decisão fixada**: **Opção 3**.

P194 fecha quando:
1. Consumer C4 migrado em `references.rs:53` com
   substitution-with-fallback.
2. Comentário inline cross-referenciando sequência §9 P189.
3. Tests E2E confirmam paridade observable preservada
   (output idêntico — fallback path activo).
4. Tests existentes (`layout_resolved_labels_*`,
   sentinelas P189B E2/E4) não regridem.
5. Copy-sites `mod.rs:1481, 1512` **NÃO** modificados.
6. DEBT M5-residual nota actualizada: 3 → 2 pré-requisitos
   (C4 migration ✅ feita).
7. **Excepções E2-E6 continuam activas** — só fecham com
   P195+.

---

## §3 Plano de sub-passos

**Sub-passo único agregado P194B**:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar `references.rs:53` + comentário inline + tests E2E paridade + L0 (se aplicável) + actualização nota DEBT M5-residual + relatório consolidado P194 | S |

Total agregado: ~5-15 LOC produção (consumer migration) +
~50 LOC tests + relatório consolidado ≈ S puro.

---

## §4 Magnitude consolidada

P194 série: **S puro** (1×S agregado).

Idêntico em magnitude a P187/P188/P193. Replicação literal
de padrão substitution-with-fallback com primitiva
`resolved_label_for` (P193B).

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Substitution-with-fallback é padrão estabelecido
  P184D/P187B/P188B.
- `resolved_label_for` já existe (P193B).
- Sem decisão arquitectural nova.

---

## §6 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada per P193 §7:

> Antes P194: 3 pré-requisitos pendentes para fechar
> cadeia E2-E6.
>
> **Após P194B: 2 pré-requisitos restantes** (1 dos 3
> avançado).
> 1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
> 2. ~~C4 migration~~ ✅ P194B.
> 3. Sub-store `headings_for_toc` — passo dedicado.
> 4. `Content::SetEquationNumbering` — passo independente.
>
> Nota: P194 migra consumer mas excepções E2-E6 só
> fecham com P195+ (walk arms migrados). Cadeia avança
> incrementalmente.

---

## §7 Estado temporário (não permanente) face a P188B

P194 é **distinto de P188B**:

| | P188B (C2 Equation) | P194 (C4 resolved label) |
|---|---|---|
| Estado dormente | **Permanente** | **Temporário** |
| Razão dormente | `SetEquationNumbering` ausente; populate impossível | Sub-store P193B aberto, mas walks E2/E4 não migrados ainda |
| Quando activa | Quando passo dedicado materializar SetEquationNumbering | Quando P195 (walk Labelled) + P196 (walk Heading) executarem |
| Documentação | 4 pontos obrigatórios (comentário inline + L0 + test sentinela + relatório §5) | Mais leve: comentário inline curto + nota no relatório consolidado |
| Caminho funcional permanente | Fallback legacy | Fallback legacy **temporariamente**; Introspector activa após P195+ |

Honestidade obrigatória: documentar diferença entre P188B
(Equation permanente) e P194 (resolved label temporário) no
relatório consolidado P194.

---

## §8 Próximo sub-passo

**P194B** — migração consumer C4:

1. Editar `01_core/src/rules/layout/references.rs:53-57`:
   - Substituir match legacy pela expressão Opção C.
   - Adicionar comentário inline (4-5 linhas).

2. **Não tocar** `mod.rs:1481, 1512` (copy-sites).

3. Tests E2E em submódulo `p194b_c4_resolved_label`:
   - `c4_resolved_label_via_introspector_path_quando_populated`
     (popula manualmente intr.resolved_labels para
     simular pós-P195).
   - `c4_resolved_label_via_fallback_legacy_caso_atual`
     (sub-store vazio per P193B; fallback legacy retorna
     valor).
   - `c4_resolved_label_paridade_legacy_vs_introspector`
     (com state injectado em ambos paths).
   - `c4_resolved_label_fallback_at_arrobado_quando_ausente`
     (label inexistente; fallback `@nome`).

4. Actualizar nota DEBT M5-residual no relatório
   consolidado P194 (3 → 2 pré-requisitos).

5. Relatório consolidado P194 (9 secções padrão).

Magnitude: S puro. Sem cláusulas condicionais.
