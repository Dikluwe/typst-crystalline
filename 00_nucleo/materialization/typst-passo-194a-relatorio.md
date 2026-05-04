# Relatório P194a — Diagnóstico C4 resolved label migration

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: P193B fechado; tests workspace 1.821
verdes; zero violations.

---

## §1 Escopo

P194A é o passo de diagnóstico-primeiro que precede a
migração C4 (consumer Ref-arm em `references.rs:53`).
Replica registo de P181A/P182A/.../P193A.

P194 é **passo 2 da sequência de 7 passos** identificada
em P189 §9 para fechar M5 universalmente.

P194A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-c4-resolved-label-passo-194a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-194a-relatorio.md` (este, 14 secções).

Sem ADR. Sem DEBT formal.

---

## §2 Inputs verificados empiricamente (7 reads/greps)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Site C4 actual | `references.rs:53` match sobre `layouter.counter.resolved_labels.get(target)` |
| 2 | Acesso `layouter.introspector` | **Cenário α** directo (já usado em linha 44 P168) |
| 3 | Tipo `target` | `&Label` (parâmetro) |
| 4 | Tipo final | `String` (passado a `Content::text`) |
| 5 | Legacy `resolved_labels.get` | `Option<&String>` |
| 6 | Copy-sites Layouter | `mod.rs:1481, 1512` independentes do introspector; **manter** |
| 7 | API `resolved_label_for` | `Option<&str>` (P193B) — exige `to_string()` no resultado |

Crítico: P194 é **estado temporário** (não permanente como
P188B). Sub-store fica vazio até P195+ activar populate.

---

## §3 Decisões cláusulas 1–6 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma da expressão | **Opção C** — match com `or_else` propagando `Option<&str>`; `to_string()` no Some arm |
| 2 | `None` do Introspector | **Opção A** aceitar; comentário leve |
| 3 | Copy-sites Layouter | **Manter** durante janela compat M5 (fallback depende) |
| 4 | Acesso `layouter.introspector` | **Cenário α** directo |
| 5 | Documentação inline | **Opção B** comentário curto referenciando sequência §9 P189 |
| 6 | Critério fecho | Consumer migrado + tests E2E + DEBT actualizado |

Forma final:

```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

---

## §4 Plano de sub-passos B (sem condicionais)

**Sub-passo único agregado**:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar `references.rs:53` + comentário inline + 4 tests E2E + actualização nota DEBT + relatório consolidado P194 | S |

---

## §5 Magnitude agregada

**P194 série = S puro** (1×S agregado em sub-passo único).

Idêntico em magnitude a P187/P188/P193. Replicação literal
de padrão substitution-with-fallback com primitiva
`resolved_label_for` (P193B).

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- Sub-store `ResolvedLabelStore` aberto (P193B).
- Trait method `resolved_label_for` disponível (P193B).
- Layouter tem `introspector` field (P168).

### §6.2 — Dependentes

- Excepções E2-E6 (P189B) continuam activas. **P195+
  destranca** quando walk arms Labelled/Heading migrarem
  e popularem sub-store via Tag.
- M5 universal fecha após sequência P195/P196/P197/P198 +
  passo independente SetEquationNumbering.

### §6.3 — Independente

- Sub-store `headings_for_toc` (lacuna #3) — passo
  dedicado.
- `Content::SetEquationNumbering` materialização —
  paralelo.

---

## §7 ADR avaliação

**Sem ADR criada.** Substitution-with-fallback é padrão
estabelecido P184D/P187B/P188B. Sem decisão arquitectural
nova.

---

## §8 DEBT avaliação

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada:

> Antes P194: 3 pré-requisitos pendentes.
>
> **Após P194B: 2 pré-requisitos restantes** (1 dos 3
> avançado — C4 migration ✅).
> 1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
> 2. ~~C4 migration~~ ✅ P194B.
> 3. Sub-store `headings_for_toc` — passo dedicado.
> 4. `Content::SetEquationNumbering` — passo independente.
>
> **Excepções E2-E6 continuam activas** — só fecham com
> P195+ (walk arms migrados). P194 desbloqueia consumer;
> activação completa do Introspector path requer P195+.

DEBT M5-residual continua em Cenário B.

---

## §9 Restrições honradas

- **Zero código tocado**.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não migra consumer C4** — P194B.
- **Não modifica trait `Introspector`** — P185B fechou.
- **Não modifica `TagIntrospector`** — P193B fechou.
- **Não toca copy-sites Layouter** — decisão fixada para
  P194B (manter).
- **Não modifica walk arms** — P195+.
- **Não modifica `from_tags`** — P195+.
- **Sem inflação retórica**.
- **Honestidade obrigatória sobre estado temporário**:
  documentado em §7 do diagnóstico (tabela P188B vs P194).
- **Sem cláusulas condicionais** nos sub-passos.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.821** inalterado
  vs P193B.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR; sem DEBT formal.

---

## §11 Achados não-triviais

### §11.1 — Site C4 está **após** dois early-returns figure-ref

`layout_ref` (referências.rs:40-58) tem 3 caminhos em
ordem:
1. Linha 44: figure-ref via Introspector (P168 já migrado).
2. Linha 49: figure-ref via legacy fallback (P168 mantido).
3. **Linha 53: resolved-labels** (este — C4) — atinge-se
   apenas se label não corresponder a figure.

Isto significa que C4 cobre Heading auto-toc + Labelled
explicit, **não figures**. Migration não interage com
P168 path.

### §11.2 — Cenário α directo confirmado empiricamente

Linha 44 já usa
`layouter.introspector.figure_number_for_label(target)` sem
qualificação adicional. Mesmo pattern aplicável a
`layouter.introspector.resolved_label_for(target)` em P194B.

Sem cláusula gate trivial sobre acesso.

### §11.3 — Layouters secundários têm `introspector` próprio

`mod.rs:1472 (l.introspector = introspector)` e
`mod.rs:1511 (l.introspector = introspector.clone())` mostram
que cada Layouter (short-circuit + cada iteração do
fixpoint) recebe a sua cópia do Introspector.

Em P194B:
- Layouter principal: introspector populado por walk +
  `from_tags` (mas sub-store `resolved_labels` está vazio
  até P195+).
- Layouters secundários: introspector clonado (também
  vazio).
- Fallback legacy via `l.counter.resolved_labels` (copiado
  separadamente em copy-sites).

**Fallback legacy é caminho funcional** durante janela
compat. Copy-sites necessários.

### §11.4 — Diferença chave face a P188B

P194 e P188B **ambos** usam substitution-with-fallback,
mas semântica diferente:

| | P188B (C2 Equation) | P194 (C4 resolved label) |
|---|---|---|
| Estado | **Permanente** dormente | **Temporário** dormente |
| Razão | `SetEquationNumbering` ausente | Walks E2/E4 não migrados |
| Activação | Passo dedicado para SetEquationNumbering | P195 + P196 (walk arms migration) |
| Doc obrigatória | 4 pontos | Mais leve (comentário curto + nota relatório) |

§7 do diagnóstico documenta esta diferença com tabela.

### §11.5 — Forma Opção C é variante mais elegante

Forma final:
```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

`Option<&str>` propagado através da chain (sem clone
intermediário). Único `to_string()` no `Some` arm.
Idiomático em Rust.

### §11.6 — Tests E2E preservam paridade trivialmente

Sub-store P193B vazio em produção → Introspector path
retorna `None` → fallback legacy chamado consistentemente
→ output idêntico ao actual.

Tests E2E em P194B replicam padrão P187B/P188B mas
**sem caso central de divergência observable** (não há —
produção real preserva). Apenas confirmar:
- Caso 1: state injectado em sub-store (simula
  pós-P195) → Introspector path retorna valor.
- Caso 2: produção real (sub-store vazio) → fallback
  legacy.
- Caso 3: paridade entre os dois caminhos.
- Caso 4: label ausente em ambos → fallback `@nome`.

---

## §12 Snapshot pós-P194A

- **Tests workspace**: 1.821 (inalterado).
- **Trait `Introspector`**: 19 métodos.
- **`TagIntrospector` sub-stores**: 8.
- **M5 progresso**: 1 arm migrado + 6 excepções (P189B);
  P193B + P194 cumprem 2 dos 4 pré-requisitos.
- **DEBT M5-residual**: 4 → 2 pré-requisitos pendentes
  (após P194B).
- **63 passos executados** (P193B = 62 + P194A = 63).
- **Padrão diagnóstico-primeiro**: 16ª aplicação
  consecutiva.

---

## §13 Próximo passo

**P194B** — migração C4 + tests E2E + nota DEBT
M5-residual:

- Editar `01_core/src/rules/layout/references.rs:53-57`:
  - Substituir match legacy pela expressão Opção C.
  - Adicionar comentário inline curto referenciando
    sequência §9 P189.
- **Não tocar** `mod.rs:1481, 1512`.
- Tests E2E em `mod p194b_c4_resolved_label`:
  - `c4_resolved_label_via_introspector_path_quando_populated`.
  - `c4_resolved_label_via_fallback_legacy_caso_atual`.
  - `c4_resolved_label_paridade_legacy_vs_introspector`.
  - `c4_resolved_label_fallback_at_arrobado_quando_ausente`.
- Actualizar nota DEBT M5-residual no relatório
  consolidado P194 (3 → 2 pré-requisitos).

Magnitude: S puro. Sem cláusulas condicionais.

---

## §14 Conclusão

P194A fechou 6 cláusulas com decisão literal e plano em
sub-passo único. Magnitude S agregada confirmada. ADR
avaliada e dispensada. DEBT M5-residual avança 1
pré-requisito (3 → 2 após P194B).

Achados centrais:
- **Forma Opção C** — `Option<&str>` propagado, único
  `to_string()` no `Some` arm — variante idiomática.
- **Cenário α directo** confirmado para
  `layouter.introspector.resolved_label_for(...)`.
- **Copy-sites Layouter manter** durante janela compat M5;
  fallback legacy depende.
- **Estado temporário** (não permanente como P188B) —
  Introspector activa após P195+. Documentação mais leve
  que P188B.

P194 é **passo 2 dos 7** da sequência §9 P189 consolidado.
Excepções E2-E6 continuam activas — só fecham com P195+.

Padrão diagnóstico-primeiro mantido — 16/16 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A/194A).

Próximo passo: **P194B** — migração concreta de C4.
