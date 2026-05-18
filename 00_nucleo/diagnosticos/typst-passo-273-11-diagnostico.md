# Diagnóstico Fase A P273.11.A — Extract Stack measurement helper (cleanup §9 P273.9)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.11.A.
**Magnitude**: XS-XXS documental (~10 min).
**Cluster**: Visualize / Gradient (segundo de 6 sub-passos para fechar cluster).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Vigésimo segundo consumo directo de fonte** (cluster Gradient cleanup
intra-cluster).

---

## §A.1 — Localização literal dos dois sítios

`01_core/src/rules/layout/mod.rs`:

### Sítio 1 — `measure_content_constrained` Stack arm (P156I)

- **Linhas 2153-2180** — arm `Content::Stack { children, dir, spacing }`.
- Signature: `&self`, recebe `max_width: f64` como parâmetro do método.
- Early-return `(0.0, 0.0)` se `n == 0` (linha 2157).
- Computação:
  - Vertical: `(max_w_children, sum_h + (n-1)*space_pt)`.
  - Horizontal: `(sum_w + (n-1)*space_pt, max_h_children)`.

### Sítio 2 — `layout_content` Stack arm save/restore (P273.9)

- **Linhas 1301-1380** — arm `Content::Stack { children, dir, spacing }`.
- **Linhas 1319-1337** — replicação inline da medição P156I para
  construir `parent_bbox`.
- Signature: `&mut self`, computa `stack_avail_w = self.available_width()`
  localmente.
- Early-return `if n == 0 { return; }` antes da medição (linha 1311);
  helper extraído NÃO precisa do early-return (n>0 garantido aqui)
  mas mantém-no para suportar Sítio 1.

---

## §A.2 — Verificação equivalência bit-exact

Análise empírica linha-a-linha confirma equivalência **bit-exact**:

| Aspecto | Sítio 1 (P156I) | Sítio 2 (P273.9) | Equivalência |
|---|---|---|---|
| `space_pt` resolução | `spacing.map_or(0.0, ...)` | `spacing.map_or(0.0, ...)` | ✓ idêntico |
| `n` early-return | `if n == 0 { return (0.0,0.0); }` | `if n == 0 { return; }` (do arm) | ✓ funcionalmente equivalente |
| `dir.is_vertical()` branch | Sim | Sim | ✓ |
| Loop sobre `children.iter()` | Sim | Sim | ✓ |
| `measure_content_constrained(child, max_w)` | `max_w = max_width` (param) | `max_w = stack_avail_w` (local) | ✓ source diferente mas tipo idêntico |
| `(n-1) as f64 * space_pt` | Sim | Sim | ✓ |
| Vertical retorno | `(max_w, sum_h + ...)` | `(max_w, sum_h + ...)` | ✓ |
| Horizontal retorno | `(sum_w + ..., max_h)` | `(sum_w + ..., max_h)` | ✓ |

**Resultado**: lógica literal-equivalente. Source de `max_w` é o
único delta — ambos passam o valor como parâmetro lógico ao helper.

---

## §A.3 — Decisão 1 fixada: forma do helper

**Fixada**: **1β — método em Layouter**.

Razões:
1. Ambos callers (`measure_content_constrained` + Stack arm save/restore)
   já são métodos de `Layouter` — sem custo signature.
2. `measure_content_constrained` é `&self` — método encaixa naturalmente.
3. Menos args (4 vs 5+) = menos signature noise = mais legível.

Signature proposto:

```rust
impl Layouter {
    fn measure_stack(
        &self,
        children: &[Content],
        dir: Dir,
        spacing: Option<Length>,
        max_w: f64,
    ) -> (f64, f64) {
        // (corpo idêntico ao Sítio 1)
    }
}
```

---

## §A.4 — Decisão 2 fixada: `max_w` source

**Fixada**: **parâmetro explícito** (não inferir do estado).

Razões:
1. Sítio 1 já recebe `max_width` como param — preservar.
2. Sítio 2 passa `self.available_width()` que é `f64` — equivalente
   no tipo.
3. Helper testável isolado (Decisão 1β preserved porque param é
   intrínseco do contrato, não state).

---

## §A.5 — Análise de risco

| Risco | Estado |
|---|---|
| Regressão tests P273.9 Stack | **Mitigado** — helper bit-exact por §A.2 |
| Regressão tests P156I Stack handler | **Mitigado** — substitutição directa do corpo |
| Cap LOC L1 estourado | **Mitigado** — cap hard 15 / soft 10; net esperado **negativo** (-15 a -20 LOC) |
| Lint zero alterado | **Mitigado** — refactor cosmético sem mudança L0 |
| `n` underflow ao calcular `n-1` | **Mitigado** — helper early-return preserves Sítio 1 behavior |

---

## §A.6 — Critério de aceitação Fase A

- ✓ §A.1 cita ambos sítios literal (mod.rs:1319-1337 + mod.rs:2153-2180).
- ✓ §A.2 confirma equivalência bit-exact tabela 8 linhas.
- ✓ §A.3 Decisão 1 fixada: **1β método em Layouter**.
- ✓ §A.4 Decisão 2 fixada: **`max_w` parâmetro explícito**.

**Fase A produzida — critério §A.6 cumprido absoluto.**

---

## §A.7 — Plano de implementação (Fase C)

### Cap LOC

- **L1 hard cap**: ≤ 15 LOC net adicionado.
- **L1 soft cap**: ≤ 10 LOC net adicionado.
- **L1 net esperado**: **-15 a -20 LOC** (remoção > adição).
- **Tests**: 0 novos (cleanup mecânico; tests P273.9 + P156I validam).

### Patch literal

1. Adicionar `Layouter::measure_stack(&self, children, dir, spacing, max_w) -> (f64, f64)` (~25 LOC novos).
2. Substituir Sítio 2 P273.9 inline (~25 LOC → ~1 LOC chamada).
3. Substituir Sítio 1 P156I (~28 LOC → ~3 LOC chamada).

Net: +25 - 25 - 25 = -25 LOC.

### Verificação

- `cargo test --workspace` — 2632 verdes preserved bit-exact.
- `cargo build` — zero novos warnings.
- `crystalline-lint .` — zero violations.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo segundo
consumo. Decisões 1β + 2 fixadas; pronto para Fase C (~25 LOC L1
removidos líquidos; 0 testes novos).*
