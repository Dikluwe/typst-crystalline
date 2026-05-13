# Relatório do passo P220 — Colbreak agregado (variant + stdlib + arm graded)

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-220.md`.
**Tipo**: aditivo agregado (variant + arm + stdlib) com mudança
observable graded (paridade P156E pagebreak literal).
**Magnitude planeada**: S+ (~1.5h). **Magnitude real**: S+ (~1h).
**Marco**: nenhum (nono passo pós-M9c; primeiro sub-passo
agregado pós-M9c — fecha sub-fase b DEBT-56 4/4
estructuralmente).

---

## §1 O que foi feito

P220 agrega num único sub-passo: variant `Content::Colbreak {
weak: bool }` + stdlib `native_colbreak(weak: ?)` + arm
Layouter Opção β graded (downgrade literal a pagebreak via
reuso `Layouter::new_page` P156E). Vanilla também downgrade
fora de columns context — paridade literal preservada.
**Sub-passo agregado** (anti-inflação 15ª aplicação cumulativa)
vs P217+P218+P219 atomizados — justificado por arm trivial
+ precedente P156E. 15 tests novos (5+6+4); 1972 → **1987
verdes**; 0 violations; 0 regressões. §A.5 `colbreak`
reclassificada `ausente` → `parcial`. Recontagem auditoria
empírica corrige off-by-one footnote ⁴⁰. **Sub-fase (b)
DEBT-56 FECHADA estructuralmente (4/4)**. Sem `P220.div-N`.

---

## §2 Confirmação contagem Content (C1)

```
grep -c "^    [A-Z][A-Za-z]\+\(\s\|{\|(\)" 01_core/src/entities/content.rs
```
- **Pré-P220**: 55 variants (paridade hipótese spec; P217
  adicionou Columns; P218+P219 sem alteração ao enum).
- **Pós-P220**: 56 variants (+1 Colbreak).

Contagem confirma hipótese; sem `P220.div-1`.

---

## §3 Variant Colbreak (C2)

```rust
// ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak manual ──
/// `weak: bool` armazenado mas semantic adiada (paridade P156E).
/// Sem `to: Option<Parity>` — vanilla `ColbreakElem` não tem
/// (paridade só faz sentido em páginas).
Colbreak {
    weak: bool,
},
```

Construtor Rust `Content::colbreak(weak: bool) -> Content`
adicionado após `Content::pagebreak`. Posição: imediatamente
após `Content::Pagebreak` (paridade ordem semântica).

**Decisões fixadas**:
- 1 field (`weak: bool`) — paridade Pagebreak sem `to`.
- Construtor sem default (stdlib `native_colbreak` tem
  default `weak: false`).

---

## §4 Arms exhaustivos (C3; 8 arms / 4 ficheiros)

Compiler-driven (paridade P217 estratégia):

**`entities/content.rs`** (5 arms):
- `is_empty` — `Self::Colbreak { .. } => false` (event
  observable; paridade Pagebreak/Divider P154B).
- `plain_text` — `Self::Colbreak { .. } => String::new()`
  (event sem texto).
- `PartialEq::eq` — `(Self::Colbreak { weak: wa },
  Self::Colbreak { weak: wb }) => wa == wb` (1 field).
- `map_content` (terminal) — `Content::Colbreak { .. }`
  adicionado à chain `=> self.clone()`.
- `map_text` (terminal) — idem `Content::Colbreak { .. }`.

**`rules/introspect.rs`** (2 arms):
- `materialize_time` — `Content::Colbreak { .. }` na
  chain terminal `=> content.clone()`.
- `walk` — `Content::Colbreak { .. }` na chain terminal
  `=> {}` (sem children; sem tag).

**`rules/layout/mod.rs::layout_content`** (1 arm — Opção β):
```rust
Content::Colbreak { weak: _ } => {
    if self.regions.current.cursor_x.0
        > self.regions.current.line_start_x.0 {
        self.flush_line();
    }
    self.new_page();
}
```

**`rules/layout/mod.rs::measure_content_constrained`** (1 arm):
```rust
Content::Colbreak { .. } => (0.0, 0.0),
```

**`rules/introspect/locatable.rs`** (entrada explícita):
- `Content::Colbreak { .. }` adicionado à chain `_ => false`
  (não-locatable; leaf event).

Total: **8 arms** em **5 sítios físicos** distribuídos por
**4 ficheiros L1**. Build verde sem missed-arm errors —
estratégia compiler-driven literal P217 funcionou.

---

## §5 `native_colbreak` + arm downgrade graded (C4; Opção β fixada)

**Stdlib** em `01_core/src/rules/stdlib/layout.rs` após
`native_columns` (paridade ordem ADR-0061 Fase 3 sub-passos
3→4):

```rust
pub fn native_colbreak(_ctx: &mut EvalContext, args: &Args, ...)
    -> SourceResult<Value>
{
    if !args.items.is_empty() {
        return Err(/* "não aceita argumentos posicionais" */);
    }
    let mut weak: bool = false;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => return Err(/* "espera bool, recebeu …" */),
            },
            other => return Err(/* "named inesperado '…'" */),
        }
    }
    Ok(Value::Content(Content::Colbreak { weak }))
}
```

**Re-export** em `stdlib/mod.rs` (`pub use … native_colbreak,
native_columns, …`). **Scope register** em `eval/mod.rs` após
`columns`:
```rust
scope.define("colbreak", Value::Func(Func::native(
    "colbreak", native_colbreak,
)));
```

**Decisão Opção β (downgrade colbreak = pagebreak)**:
- vs Opção α stub puro (no-op) — rejeitada (colbreak
  invisible viola paridade vanilla).
- vs Opção γ erro fora de columns context — rejeitada
  (complica arm; viola pattern P217+).
- **Opção β fixada** — paridade vanilla literal: vanilla
  também downgrade colbreak fora de columns context;
  pós-P219 cristalino (single-region) tudo é "fora";
  reusa `Layouter::new_page` P156E (zero refactor
  estrutural).

**Decisão sem `to: Option<Parity>`**: vanilla `ColbreakElem`
não tem campo de paridade (faz sentido só em páginas).
Test `p220_native_colbreak_to_rejeita` verifica que
`colbreak(to: "even")` falha como named desconhecido.

**Decisão `weak: bool` armazenado mas semantic adiada**:
paridade P156D HSpace/VSpace + P156E Pagebreak. Anti-inflação
cumulativo.

---

## §6 Resultados verificação (C7-C8)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 1987 verdes | **1987 verdes** (1698 + 242 + 24 + 2 + 21) |
| `crystalline-lint .` | 0 violations | **0 violations** |
| `crystalline-lint --fix-hashes` | sync se necessário | "Nothing to fix" (L0 não tocado) |
| Tests P220 novos | 15 | ✓ 5 unit content + 6 unit stdlib + 4 E2E = 15 verdes |
| Mudança observable | sim (downgrade β) | ✓ colbreak produz nova página |
| Regressões pre-existente | 0 | **0** |
| Borrow checker quebras | 0 | **0** |
| Ajustes manuais | 0-1 | **0** (refactor literal sem fricção) |
| Variant Content count | 55 → 56 | ✓ 56 |
| Stdlib funcs count | ~54 → 55 | ✓ 55 |

---

## §7 Inventário 148 reclassificação + ADR-0078 anotação (C9-C10)

**Inventário 148**:
- §A.5 Layout linha `colbreak()` **reclassificada
  `ausente` → `parcial`** ⁴¹ (variant + stdlib + arm real
  existem; multi-region salto entre colunas reais
  ausente — P-Layout-Fase4 candidato).
- **Auditoria empírica P220 detectou off-by-one da
  footnote ⁴⁰**: tabela A.5 declarou `13/1/4/0/0=18`
  pós-P219 mas auditoria mostrou real era `12/1/4/1/0=18`
  (colbreak permanecia em `ausente`). Footnote ⁴¹
  documenta correcção transparente.
- Tabela A.5 Layout: `12/1/4/1/0 = 18 → **12/1/5/0/0 = 18**`
  (1 ausente eliminado pelo P220; **zero ausentes em
  Layout pós-P220** — ganho estructural real preservado).
- **Total user-facing**: `69/24/26/20/2 = 141 →
  **68/24/27/20/2 = 141**` (corrige offset Layout +
  reclassifica colbreak).
- Cobertura Layout: `13/18 = **72% preservada**` (parcial
  fora numerador per metodologia §A.9 P213; ganho
  qualitativo: segunda reclassificação Layout pós-M9c).
- Cobertura user-facing total: `92/141 ≈ **65% preservada**`
  (declínio de 1pp reflecte correcção de auditoria, não
  regressão semântica).
- **Footnote ⁴¹ adicionada** documentando P220 + Opção β +
  scope-out + reclassificação + recontagem auditada.

**ADR-0078** §"Plano de materialização" anotada com bloco
`### P220 materializado 2026-05-12`:
- Variant Colbreak adicionado + 8 arms exhaustivos.
- `native_colbreak` stdlib registada.
- Arm Layouter Opção β downgrade graded + reuso new_page.
- `weak` armazenado adiado; sem `to`.
- Multi-region scope-out documentado.
- Sub-passo agregado (anti-inflação 15ª).
- 15 tests + 0 regressões.
- Inventário 148 reclassificação + recontagem registada.
- L0 extensão Opção γ deferida (convenção N=4).
- **Sub-fase (b) DEBT-56 FECHADA estructuralmente: 4/4**.

**Status ADR-0078**: PROPOSTO mantido. Marco interno
atingido. Transição IMPLEMENTADO em P221 (encerramento
documental).

---

## §8 Próximo sub-passo

P220 fecha quarto e último sub-passo sub-fase (b) DEBT-56
(4/4). Decisão humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P221** imediatamente — encerramento Fase 3 + ADR-0078 PROPOSTO → IMPLEMENTADO + DEBT-56 fecha + ADR-0061 candidato; possível promoção Tabela B.2 Content variants | S documental (~30min) | alta (sub-fase b 4/4 completa; Layout em estado clean — zero ausentes; momentum natural para fechar marco) |
| **Caminho 2** | Pivot Bloco C P222 — `measure(body)` stdlib expose | S+ (~1-2h) | média (win rápido §A.9 estricto; isolado de DEBT-56) |
| **Caminho 3** | Adiar Layout; outro módulo (Visualize/Foundations/Markup) | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P221)** —
fechar marco DEBT-56 estructuralmente e documentalmente
agora que sub-fase (b) está completa preserva clareza
arquitectural. ADR-0078 transição PROPOSTO → IMPLEMENTADO
torna o marco oficial. Layout em "estado clean" (zero
ausentes) é momento ideal para encerrar Fase 3.

**Estado pós-P220**:
- **Sub-fase (a) DEBT-56**: 2/2 ✓ (P216A + P216B).
- **Sub-fase (b) DEBT-56**: **4/4 ✓** (P217 + P218 + P219
  + P220). DEBT-56 estructuralmente completo.
- Falta só P221 documental (ADR-0078 transição + marco).
- ADRs: ADR-0078 PROPOSTO (transição P221); ADR-0061
  PROPOSTO ~80% concluído (Fase 1 ✓, Fase 2 ✓, Fase 3 5/5
  features sub-fase b — só measure/place refinos pendentes).
- Layout cobertura agregada **72% preservada** (parcial fora
  numerador). 2 reclassificações qualitativas cumulativas
  (`columns` P219 + `colbreak` P220 ausente → parcial;
  **zero ausentes Layout pós-P220**).
- Tests workspace: **1987 verdes**; `crystalline-lint`: **0
  violations**.
- Cumulativo P216A+B+P217+P218+P219+P220 = ~325 substituições
  + 2 variants + 18 arms + 2 stdlib funcs + 1 helper +
  1 constante + 1 arm refactored substantivo + 41 tests
  novos em 6 sessões.
- **15 aplicações cumulativas anti-inflação pós-P205D**
  (P220 sub-passo agregado).
- **Pattern emergente "L0 minimal para refactors" N=4**
  (P217+P218+P219+P220 todos Opção γ). Possível
  formalização como ADR meta documental se N≥5.
- **Pattern emergente "stub transparente → consumer real
  graded" N=1 estável** (P219). P220 NÃO segue —
  agregação directa.
- **Pattern emergente "sub-passo agregado paridade
  precedente" N=2** (P156E pagebreak agregado + P220
  colbreak agregado). Anti-inflação fundamentada.
- **Primeira reclassificação Layout pós-M9c por
  reclassificação qualitativa de **dupla** ordem N=2**
  (`columns` P219 + `colbreak` P220 ausente → parcial).
  Zero ausentes Layout — marco estructural real.
