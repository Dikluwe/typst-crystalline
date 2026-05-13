# Relatório do passo P222 — `native_measure` stdlib expose (Bloco C ADR-0066)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-222.md`.
**Tipo**: aditivo à stdlib expondo helper privado existente
como função pública (paridade P218 native_columns pattern).
**Magnitude planeada**: S+ (~1-2h). **Magnitude real**: S+
(~1h).
**Marco**: nenhum (décimo passo pós-M9c; **primeiro
sub-passo Fase 4 Layout candidata**; paridade estrutural
P217 para Fase 3 sub-fase b).

---

## §1 O que foi feito

P222 materializa `measure(body) -> dict(width: length,
height: length)` stdlib expose graded — paridade vanilla
observable via Dict indexing. Helper privado
`measure_content` em `layout/helpers.rs` promovido
`pub(super)` → `pub(crate)`; módulo `helpers` promovido a
`pub(crate)`. **Width override scope-out** per Opção β
graded ADR-0054; rejeitado explicitamente. **3 decisões
fixadas**: Opção α retorno `Value::Dict` (paridade vanilla
literal); Opção β sem width override (scope-out graded);
Opção γ L0 sem extensão (pattern N=4 → 5). **ADR-0066
PROPOSTO mantido** — 3 condições §"Plano promoção" não
satisfeitas (helper é puro single-pass; sem runtime queries
genuínas). 11 tests novos (9 unit + 2 integração unit-as-E2E
pragmática); workspace **1987 → 1998 verdes**; 0 violations;
0 regressões. §A.5 `measure(body)` reclassificada `parcial`
→ `implementado⁺`. Cobertura Layout: **72% → 78% real**
(coincide com paridade visual histórica Opção γ §2.1).
Sem `P222.div-N`.

---

## §2 Inventário pré-P222 helper (C1)

`grep -n "fn measure_content" layout/helpers.rs`:
- Linha 78: `pub(super) fn measure_content(content: &Content,
  available_w: f64) -> (f64, f64)`.

Signature confirmada (paridade hipótese spec C1):
- Função privada `pub(super)` — visível só ao módulo
  layout; **NÃO acessível** ao stdlib em mesmo crate.
- Params: `&Content` + `available_w: f64`.
- Return: `(f64, f64)` (width, height em pt).
- Suporte real: `Shape::Rect/Ellipse/Path/Line` +
  `Sequence` composição (max_w + total_h).
- Fallback: `(0.0, 0.0)` para texto multi-linha + equações
  + heading + etc. (aproximação conservadora documentada).

**Decisão crítica C1**: visibility promotion NECESSÁRIA
(helper privada absoluta `pub(super)` vs spec hipótese
"provavelmente já acessível").

---

## §3 Visibility promotion (C2)

2 mudanças mínimas:

1. **`layout/helpers.rs:78`**: `pub(super) fn
   measure_content(...)` → **`pub(crate) fn
   measure_content(...)`**. Documentação inline
   adicionada referenciando P222 + ADR-0066 Bloco C.

2. **`layout/mod.rs:39`**: `mod helpers;` → **`pub(crate)
   mod helpers;`** (descoberta em compile-error E0603 —
   module-level visibility também necessária para
   cross-module crate access).

Mudança de API minimalista — helper continua não-público
fora do crate; pattern consistente com outros helpers
privados acessíveis ao stdlib mesmo crate. Hash propagado
em ambos ficheiros L1.

---

## §4 `native_measure` + scope register (C3 + C4)

**Stdlib** em `01_core/src/rules/stdlib/layout.rs` após
`native_colbreak` (paridade ordem ADR-0061 Fase 3 → Fase 4
candidata):

```rust
pub fn native_measure(_ctx: &mut EvalContext, args: &Args, ...)
    -> SourceResult<Value>
{
    use crate::rules::layout::helpers::measure_content;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    // 1. Extract body (Content ou Str shortcut).
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(/* tipo errado */),
        None        => return Err(/* body ausente */),
    };

    // 2. Reject >1 posicional.
    if args.items.len() > 1 {
        return Err(/* "aceita 1 posicional" */);
    }

    // 3. Reject all named args (Opção β scope-out).
    if let Some(key) = args.named.keys().next() {
        return Err(/* "named arg `{}` não suportado (paridade
                    graded; refino futuro candidato
                    NÃO-reservado per ADR-0054)" */);
    }

    // 4. Call helper privado (pub(crate) pós-C2).
    //    available_w = f64::INFINITY (sem constraint).
    let (width_pt, height_pt) = measure_content(&body, f64::INFINITY);

    // 5. Build Dict { width, height } com Length.
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
        IndexMap::default();
    dict.insert("width".into(),  Value::Length(Length::pt(width_pt)));
    dict.insert("height".into(), Value::Length(Length::pt(height_pt)));

    Ok(Value::Dict(dict))
}
```

**Re-export** em `stdlib/mod.rs`: `native_measure` adicionado
à lista alfabética em `pub use crate::rules::stdlib::layout::{...}`.

**Scope register** em `eval/mod.rs` (paridade P218 pattern):
```rust
scope.define("measure", Value::Func(Func::native(
    "measure", native_measure,
)));
```

Posição: imediatamente após `colbreak` (paridade ordem
materialização Fase 3 → Fase 4 cumulativa).

**Stdlib funcs count**: 55 → **56**.

---

## §5 Decisões substantivas

**Decisão 1 — Tipo de retorno Opção α Dict**:
- vs Opção β Tuple ou `Value::Size` novo — rejeitada
  (paridade observable vanilla preferida; sem novo tipo
  L1 a introduzir).
- vs Opção γ stub vazio — rejeitada (viola paridade).
- **Fixada**: `Value::Dict { "width": Length, "height":
  Length }`. `IndexMap` ordena por inserção; vanilla
  `measure(body).width` literal funcional.

**Decisão 2 — Promoção ADR-0066 vs paridade graded
Opção β**:
- vs Opção α promover PROPOSTO → IMPLEMENTADO — rejeitada
  (viola 3 condições §"Plano promoção": state(), 2-pass,
  E2E feature observable).
- vs Opção γ sem mencionar ADR-0066 — rejeitada (viola
  pattern documentação).
- **Fixada**: Opção β anotação ADR-0066 sem promoção.
  Bloco `### P222 materializado 2026-05-13 — Bloco C
  primeira materialização parcial` adicionado ao §"Plano
  de promoção futuro" com 3 condições explicitamente
  marcadas ✗ pendentes. **Pattern emergente "ADR
  PROPOSTO com materialização parcial graded" N=1
  inaugurado**.

**Decisão 3 — Width override Opção β graded**:
- vs Opção α aceita `width: Length` named — rejeitada
  (paridade vanilla completa mas exige refactor
  multi-region scope; paridade decisão columns Opção B
  P219).
- vs Opção γ aceita mas ignora silenciosamente — rejeitada
  (viola pattern explicit error P217+).
- **Fixada**: rejeitar com mensagem clara documentando
  refino futuro candidato NÃO-reservado per ADR-0054.

**Decisão 4 — L0 stdlib.md Opção γ** (pattern N=4 → 5):
- Opção γ pattern "L0 minimal para refactors" consolida
  P217+P218+P219+P220+**P222** = N=5. Promoção formal a
  ADR meta documental fica diferida (Caminho 4 P221 §8;
  política consistente N=3-4 mínima ultrapassada).
- Hash `stdlib.md` não tocado; "Nothing to fix" hashes.

**Decisão pragmática 5 — Spec C5 "2 E2E layout tests"**:
spec propôs 2 tests parse+eval em `layout/tests.rs` mas
infrastructure de parse+eval não existe em
`layout/tests.rs` (tests usam `layout(&content)` directo
construindo Content manualmente). **Decisão pragmática**:
substituir os 2 E2E pela par de tests integração
unit-as-E2E em `stdlib/mod.rs` testando Sequence
composição + round-trip Dict indexing. Cumpre intent E2E
(verifica round-trip semântico Dict.get("width") retorna
Length) sem overhead harness de parse+eval. Documentada
em footnote ⁴³ e §6 risco 11 (não-listado em spec — emergente).

---

## §6 Resultados verificação (C7-C8)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde (após 2 visibility fixes em C2 + ShapeKind import) |
| `cargo test --workspace` | 1998 verdes | **1998 verdes** (1709 + 242 + 24 + 2 + 21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 stdlib.md não tocado; hashes L1 propagados em helpers.rs + mod.rs) |
| Tests P222 novos | 11 (9 unit + 2 E2E) | ✓ 11 verdes (9 unit + 2 integração unit-as-E2E) |
| Visibility promotion | 1-2 sítios | ✓ 2 sítios (`helpers.rs:78` + `mod.rs:39`) |
| Regressões pre-existente | 0 | **0** (1987 preservados) |
| Stdlib funcs count | 55 → 56 | ✓ 56 |
| Content variants | 56 (sem alteração) | ✓ 56 preservado |

**3 ajustes durante execução** (não-listados em spec):
1. Compile error E0603 helpers module privacy — fix
   `pub(crate) mod helpers;` em layout/mod.rs (visibility
   expansion mínima cross-module crate; foi necessário além
   da promoção do helper signature).
2. Compile error tests `ShapeKind` import — fix
   `use crate::entities::geometry::ShapeKind` (não
   re-exportado via layout_types).
3. Spec field types Shape — `width`/`height` são
   `Option<Box<Value>>` (não `Option<Length>` directo);
   tests construídos `Some(Box::new(Value::Length(...)))`.

---

## §7 Inventário 148 reclassificação + ADR-0066 anotação + ADR-0061 anotação (C9-C10)

**Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`):

§A.5 Layout linha `measure(body)` (linha 151):
- Classificação: `parcial` → **`implementado⁺` ⁴³**.
- Coluna "Cristalino": "helper privado" → "Passo 222".
- Nota: stdlib `#measure(body)` exposta; helper
  promovido `pub(crate)`; Opção β width override
  scope-out; ADR-0066 §"Plano promoção" Bloco C primeira
  materialização parcial.

Tabela A (linha 431 + 436):
- Layout footnotes: `⁴⁰ ⁴¹ ⁴²` → `⁴⁰ ⁴¹ ⁴² ⁴³`.
- Distribuição §A.5: `12/1/5/0/0 = 18 → **12/2/4/0/0 =
  18**` (1 parcial → impl⁺; zero ausentes preservado).
- Total user-facing: `68/24/27/20/2 = 141 →
  **68/25/26/20/2 = 141**` (1 parcial → impl⁺).
- **Cobertura Layout per metodologia §A.9**: `(12+2)/18 =
  **78%**` real ✓ (paridade visual histórica Opção γ
  §2.1 — coincidência aritmética agradável; "divergência
  metodológica" fechou-se via materialização real).
- Cobertura user-facing: `(68+25)/141 ≈ **66%**` (+1pp
  real).

**Footnote ⁴³ adicionada** documentando P222 + visibility
promotion + Opção α Dict + Opção β scope-out + 11 tests +
decisão pragmática "2 E2E → 2 integração" + pattern
emergente "ADR PROPOSTO com materialização parcial graded"
N=1 + pattern "Fase 4 candidata reclassifica parcial →
impl⁺" N=1.

**ADR-0066** (`typst-adr-0066-introspection-runtime-adiada.md`):

Bloco `### P222 materializado 2026-05-13 — Bloco C
primeira materialização parcial` adicionado ao §"Plano
de promoção futuro" após bloco "Bloco C cross-módulo":
- Documentação completa P222 (helper expose + width
  scope-out + tests + reclassificação).
- 3 condições §"Plano promoção" explicitamente marcadas
  ✗ pendentes (state(), 2-pass, E2E feature observable
  dependente de runtime queries genuínas).
- **Status ADR-0066 PROPOSTO mantido** — pattern
  emergente "ADR PROPOSTO com materialização parcial
  graded" N=1 inaugurado.

**ADR-0061** (`typst-adr-0061-layout-fase-x-roadmap.md`):

Bloco `### P222 anotação — Fase 4 Layout candidata
sub-passo 1` adicionado após §"P221 encerramento Fase 3":
- Status ADR-0061 **IMPLEMENTADO mantido** (Fase 3
  fechada P221; Fase 4 candidata em curso sem nova
  reserva formal per política P158).
- Fase 4 candidata 1/3 sub-passos (P222 ✓; P223 place
  pendente; P224 grid refino pendente).

---

## §8 Próximo sub-passo

P222 fecha primeiro sub-passo Fase 4 Layout candidata
(série α "terminar Layout" Opção α P221 §8). Decisão
humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P223 place refino float + clearance** — segundo sub-passo Fase 4 candidata (Opção α P221 §8 continuação directa) | S+ (~1-2h) | alta (sub-passo isolado; Layout cobertura 78% → ~83% se materializado; momentum natural P222 → P223 cumulativo) |
| **Caminho 2** | P224 grid refino (gutter/header/footer/colspan) — segundo sub-passo Fase 4 candidata isolado de P223 | M (~2-3h) | média (grid é DEBT-34d/e separado; trabalho maior; Layout cobertura ganho ~5-10pp se múltiplos refinos materializados) |
| **Caminho 3** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%) | varia | baixa-média |
| **Caminho 4** | ADR meta administrativa — formalizar pattern "L0 minimal para refactors" N=4 → 5 + pattern "ADR PROPOSTO com materialização parcial graded" N=1 | XS (~30min) | baixa (passo administrativo paridade P213/P214/P160A; N=5 atinge limiar formalização sólida) |
| **Caminho 5** | Adiar Layout completo; outro objectivo arquitectural | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P223)** —
continuação directa série α "terminar Layout"; sub-passo
isolado; momentum cumulativo P222 → P223. Layout cobertura
78% → ~83% se P223 reclassifica `place` parcial → impl⁺
(paridade reclassificação P222 measure).

**Estado pós-P222**:
- Tests workspace: 1987 → **1998 verdes** (+11 P222).
- Stdlib funcs: 55 → **56** (+native_measure).
- Content variants: 56 (sem alteração).
- §A.5 distribuição: `12/2/4/0/0 = 18` (1 parcial →
  impl⁺; zero ausentes preservado).
- **Cobertura Layout per metodologia**: 72% → **78%**
  real (+6pp; coincide com paridade visual histórica
  Opção γ §2.1).
- Cobertura user-facing total: 65% → **66%** (+1pp).
- ADR-0066 PROPOSTO mantido; ADR-0061 IMPLEMENTADO
  mantido; ADR-0078 IMPLEMENTADO mantido.
- Saldo DEBTs: 13 abertos (preservado).
- **16 aplicações cumulativas anti-inflação** pós-P205D
  (P222: Opção β width override scope-out + Opção γ L0
  sem extensão).
- **Pattern emergente "L0 minimal para refactors" N=4 →
  5** (P217+P218+P219+P220+**P222** todos Opção γ).
  Promoção formal Caminho 4 candidato sólido N=5.
- **Pattern emergente "ADR PROPOSTO com materialização
  parcial graded" N=1 inaugurado** — ADR-0066 mantém
  PROPOSTO apesar de Bloco C primeira materialização
  parcial.
- **Pattern emergente "Fase 4 candidata reclassifica
  parcial → impl⁺" N=1 inaugurado** — paridade Fase 3
  que reclassificou ausente → parcial em P219+P220.
- **Coincidência aritmética agradável fechada**: 78%
  per metodologia rígida agora coincide com 78% per
  paridade visual histórica — Layout fechou "divergência
  metodológica" qualitativa via materialização real P222.
- **Fase 4 Layout candidata 1/3 sub-passos** (P222 ✓;
  P223 place pendente; P224 grid refino pendente — Opção
  α P221 §8 série α "terminar Layout").
