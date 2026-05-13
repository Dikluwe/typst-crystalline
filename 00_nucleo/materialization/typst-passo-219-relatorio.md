# Relatório do passo P219 — Consumer multi-column real graded no Layouter

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-219.md`.
**Tipo**: refactor substantivo do arm `Content::Columns` no
Layouter — deixa de ser stub transparente.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M (~1.5h).
**Marco**: nenhum (oitavo passo pós-M9c; **primeiro passo
pós-M9c com mudança observable** substantiva).

---

## §1 O que foi feito

P219 substitui arm stub transparente (P217+P218) por **consumer
real graded (Opção B fixada)** — width temporariamente reduzida
`(full_width - (count-1)*gutter) / count`; body single-render
em primeira coluna virtual; width restaurada após body.
Default gutter ~4% via constante `COLUMNS_DEFAULT_GUTTER_RATIO`
(Opção β; anti-inflação 14ª aplicação). **Multi-region flow
real é scope-out** (Opção A diferida a P-Layout-Fase4 candidato).
Decisão P216B preservada literal. 8 E2E tests novos. Tests:
1964 → 1972 verdes; 0 violations; 0 regressões pre-existente.
§A.5 `columns(n)` reclassificada `ausente` → `parcial`.
Sem `P219.div-N`.

---

## §2 Inventário pré-P219 arm stub (C1)

`grep -n "Content::Columns" 01_core/src/rules/layout/mod.rs`:
- `mod.rs:1129` — `layout_content` arm (stub transparente
  P217).
- `mod.rs:1373` — `measure_content_constrained` arm (stub
  transparente P217).

**2 arms** stub identificados (paridade spec C1 hipótese).

---

## §3 Consumer real graded `layout_content` arm (C2)

```rust
Content::Columns { count, gutter, body } => {
    // 1. Flush line pendente (columns são structural).
    if self.regions.current.cursor_x.0
        > self.regions.current.line_start_x.0 {
        self.flush_line();
    }
    let full_width = self.regions.current.width;
    let count_f = if *count == 0 { 1.0 } else { *count as f64 };

    // 2. Resolver gutter (Length → f64 Pt; default ~4% width).
    let gutter_pt = match gutter {
        Some(g) => g.resolve_pt(self.font_size_pt.0),
        None    => full_width * COLUMNS_DEFAULT_GUTTER_RATIO,
    };

    // 3. column_width = (full_width - (count-1)*gutter) / count.
    let column_width = if count_f >= 1.0 {
        (full_width - (count_f - 1.0) * gutter_pt) / count_f
    } else {
        full_width
    };

    // 4. Saved/restore pattern (paridade P156C Pad).
    let saved_width = full_width;
    self.regions.current.width = column_width;

    // 5-6. Layout body com width reduzida + flush pendente.
    self.layout_content(body);
    if self.regions.current.cursor_x.0
        > self.regions.current.line_start_x.0 {
        self.flush_line();
    }

    // 7. Restaurar width original.
    self.regions.current.width = saved_width;
}
```

**Decisões fixadas**:
- Default gutter ~4% via `COLUMNS_DEFAULT_GUTTER_RATIO`
  constante named (Opção β; top de mod.rs).
- `count=0` caso degenerate: passthrough (count_f=1;
  column_width=full_width).
- Saved/restore explícito; cursor_x **não-restaurado**
  (avança naturalmente; paridade Block/Pad/Repeat).

---

## §4 `measure_content_constrained` arm (C3)

Pattern paralelo (sem render):

```rust
Content::Columns { count, gutter, body } => {
    let count_f = if *count == 0 { 1.0 } else { *count as f64 };
    let gutter_pt = match gutter {
        Some(g) => g.resolve_pt(self.font_size_pt.0),
        None    => max_width * COLUMNS_DEFAULT_GUTTER_RATIO,
    };
    let column_width = if count_f >= 1.0 {
        (max_width - (count_f - 1.0) * gutter_pt) / count_f
    } else {
        max_width
    };
    let (_body_w, body_h) =
        self.measure_content_constrained(body, column_width);
    (max_width, body_h)
}
```

Retorna `(max_width, body_h)` — columns ocupa width inteira;
height do body single-render graded.

---

## §5 Decisões substantivas

- **Opção B fixada** (paridade ADR-0054 graded; vs Opção A
  multi-region L+ refactor; vs Opção C multi-render que
  duplica counters). Justificação literal: preserva todas
  decisões anteriores (P216B `Regions { current }` minimal);
  introduz mudança observable mínima; cumpre critério
  "consumer real graded" suficiente para reclassificar
  §A.5 `columns` para `parcial`.
- **Decisão P216B preservada literal** — `Regions { current:
  Region }` mantido. Critério "consumer multi-column real"
  redefinido em P219 como **flow real** entre colunas (não
  single-render graded). Trade-off honesto registado em
  ADR-0078.
- **Default gutter constante named (Opção β)** — vs
  Opção α inline (magic number) vs Opção γ helper privado.
  `COLUMNS_DEFAULT_GUTTER_RATIO = 0.04` named documentado.
  Anti-inflação 14ª aplicação cumulativa pós-P205D.
- **L0 `entities/content.md` extensão Opção α deferida**:
  spec C7 propôs secção dedicada `Variant Content::Columns`.
  P219 manteve convenção emergente P217 (inline-doc no
  L1). Decisão empírica — overhead documental não
  compensava (refactor focado em arm Layouter; variant
  inalterado desde P217). Documentação formal pode emergir
  em P221 encerramento série se útil.
- **Multi-region flow real scope-out**: explícito em
  comment do arm + ADR-0078 risco 2 + recomendação
  metodológica forte. Refino candidato a P-Layout-Fase4
  (não-reservado per política P158).
- **count=0 caso degenerate**: tratado como passthrough
  (count_f=1 equivalente). Defesa contra construtor Rust
  directo que não passa pelo stdlib P218 validation
  `>= 1`. Pragmatic vs panic.

---

## §6 Resultados verificação

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 1972 verdes | **1972 verdes** (1683 + 242 + 24 + 2 + 21) |
| `crystalline-lint .` | 0 violations | **0 violations** |
| `crystalline-lint --fix-hashes` | sync se necessário | "Nothing to fix" (L0 não tocado) |
| Tests P219 novos | 8 E2E | ✓ 8 verdes |
| Mudança observable | **sim** (primeiro pós-M9c) | ✓ width reduzida em columns block |
| Regressões pre-existente | 0 | **0** (P217 E2E `arm_transparente` preservado — count=2 ainda renderiza body via column_width reduzida) |
| Borrow checker quebras | 0 | **0** |
| Ajustes manuais | 0-1 | **0** (refactor literal sem fricção) |

---

## §7 Inventário 148 reclassificação + ADR-0078 anotação

**Inventário 148**:
- §A.5 Layout linha `columns(n)` **reclassificada
  `ausente` → `parcial`** ⁴⁰ — variant + stdlib + arm real
  existem; multi-region flow real ausente.
- Tabela A.5 Layout: `13/1/3/1/0 = 18 → 13/1/4/0/0 = 18`
  (1 ausente eliminado; **zero ausentes em Layout**).
- **Total user-facing**: `69/24/25/21/2 = 141 →
  69/24/26/20/2 = 141` (1 entrada movida ausente → parcial;
  total preservado).
- Cobertura Layout: `(13+1)/18 = **78% preservada**`
  (parcial fora numerador per metodologia §A.9 P213; ganho
  qualitativo).
- Cobertura user-facing total: `(69+24)/141 ≈ **66%
  preservada**` (idem metodologia).
- **Footnote ⁴⁰ adicionada** documentando P217+P218+P219
  cumulativos + Opção B + scope-out + reclassificação.

**ADR-0078** §"Plano de materialização" anotada com bloco
`### P219 materializado 2026-05-12`:
- Arm `Content::Columns` consumer real graded.
- Fórmula vanilla literal + Default gutter constante.
- Saved/restore pattern explícito.
- count=0 passthrough.
- Multi-region flow real scope-out documentado.
- Decisão P216B preservada.
- 8 E2E tests + 0 regressões.
- Inventário 148 reclassificação registada.
- L0 extensão Opção α deferida.
- Sub-fase (b) DEBT-56: **3/4 sub-passos materializados**.

**Status ADR-0078**: PROPOSTO mantido. Transição IMPLEMENTADO
só em P221 (6 condições satisfeitas).

---

## §8 Próximo sub-passo

P219 fecha terceiro sub-passo sub-fase (b) DEBT-56 (3/4).
Decisão humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P220** imediatamente — `Content::Colbreak { weak: bool }` variant + `native_colbreak` stdlib + tests mixing pagebreak | S+ (~1.5h) | alta (continuação directa sub-fase b; aditivo análogo P220 a P217+P218 mas sem refactor; **fecha sub-fase b 4/4**) |
| **Caminho 2** | **P221** imediatamente — encerramento Fase 3 + ADR-0078 PROPOSTO → IMPLEMENTADO + DEBT-56 fecha (skip P220 colbreak) | S documental (~30min) | média (Layout 78% preservado; DEBT-56 fecharia mas sem colbreak vanilla — incompletude consciente; possível anti-inflação 15ª aplicação) |
| **Caminho 3** | Pivot Bloco C P222 — `measure(body)` stdlib expose | S+ (~1-2h) | média (win rápido §A.9 estricto 83% → 100%; isolado de DEBT-56) |
| **Caminho 4** | Adiar Layout; outro módulo | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P220)** — fecha
sub-fase (b) inteira completa (4/4); preserva paridade
estrutural vanilla (colbreak existe vanilla); momentum
P217→P218→P219→P220 cumulativo. Após P220, P221 fecha
DEBT-56 estructuralmente.

**Estado pós-P219**:
- Sub-fase (b) DEBT-56: **3/4 sub-passos** (P217 ✓, P218 ✓,
  P219 ✓; P220 pendente).
- ADR-0078 PROPOSTO; ADR-0061 PROPOSTO ~50% concluído.
- Layout 78% preservado (cobertura categoria); ganho
  qualitativo via 1 ausente eliminado (zero ausentes em
  Layout).
- Tests workspace: **1972 verdes**; `crystalline-lint`: **0
  violations**.
- Cumulativo P216A+B+P217+P218+P219 = ~325 substituições
  + 1 variant + 10 arms + 1 stdlib + 1 helper + 1 constante
  + 1 arm refactored substantivo + 26 tests novos em 5
  sessões.
- 14 aplicações cumulativas anti-inflação pós-P205D
  (P219 constante named).
- **Pattern emergente "stub transparente → consumer real
  graded"** N=1 (P219 inaugura — variant aditivo P217 +
  stdlib aditivo P218 + arm graded P219; sequência típica
  para refactor estrutural multi-componente).
- Pattern "refactor stacking" preservado N=1 (P219 não muda
  P216A/B; refactora P217 stub em arm semantic real).
- **Primeira reclassificação Layout pós-M9c**: `columns
  ausente → parcial`. Pattern emergente "Layout ganha por
  reclassificação qualitativa" N=1 (vs ganhos quantitativos
  via cobertura).
