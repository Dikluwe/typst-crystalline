# Relatório do passo P217 — `Content::Columns` variant + arms exhaustivos

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-217.md`.
**Tipo**: aditivo puro ao `Content` enum + arms exhaustivos
em sítios L1 (paridade P156C-J).
**Magnitude planeada**: S+ (~1.5-2h). **Magnitude real**: S (~1h).
**Marco**: nenhum (sexto passo pós-M9c; primeiro sub-passo
sub-fase (b) DEBT-56 — primeira adição de variant pós-M9c).

---

## §1 O que foi feito

P216A+P216B fecharam sub-fase (a) DEBT-56 (Region/Regions
abstraction). P217 começa sub-fase (b) com **primeiro consumer
estrutural** da multi-region: variant `Content::Columns
{ count, gutter, body }`. **Aditivo puro** — sem semantic
real ainda; arm Layouter é stub transparente que delega a
body ignorando count/gutter (consumer multi-region real em
P219). 10 arms exhaustivos em 4 ficheiros L1 (vs 8 estimados
— 2 adicionais via compiler-driven). 6 tests novos. Tests:
1946 → 1952 verdes. 0 violations. Sem `P217.div-N`.

---

## §2 Confirmação contagem `Content` enum

Grep `^    [A-Z][A-Za-z]+(\s|\{|\()` em `01_core/src/entities/content.rs`:
**54 variants** (próximo a 56 hipótese — discrepância pequena
por regex variation). Pós-P217: **55 variants** (+1 Columns).

---

## §3 Variant `Content::Columns` adicionado

```rust
/// **P217 (DEBT-56 sub-fase b — Layout Fase 3)** — Multi-column
/// container per ADR-0078 PROPOSTO.
Columns {
    count:  usize,                                  // count >= 1 valida em P218
    gutter: Option<Length>,                         // ADR-0064 Caso C: None ↔ default
    body:   Box<Content>,                           // paridade Pad/Block/Boxed
},
```

**Decisões atributos**:
- `count: usize` — paridade Rust convencional. Validação `>= 1`
  diferida a `native_columns` (P218); construtor Rust aceita
  `count = 0` como caso degenerate.
- `gutter: Option<Length>` per **ADR-0064 Caso C** — `None` ↔
  default vanilla. Cumulativo N=cresce (precedentes P156I
  `Stack.spacing`, P156L `Sides<Option<Length>>`).
- `body: Box<Content>` — paridade Pad/Hide/Block/Boxed.

Construtor Rust `Content::columns(body, count, gutter)` adicionado.

---

## §4 Arms exhaustivos cobertura

**10 arms em 4 ficheiros L1** (vs 8 estimados — 2 adicionais
descobertos via compiler errors):

| Ficheiro | Arms |
|----------|------|
| `entities/content.rs` | `is_empty` + `plain_text` + `PartialEq::eq` + `map_content` + `map_text` = **5** |
| `rules/introspect.rs` | `materialize_time` + `walk` = **2** |
| `rules/layout/mod.rs` | `layout_content` (stub transparente) + `measure_content_constrained` (transparente) = **2** |
| `rules/introspect/locatable.rs` | `is_locatable` catch-all `_ => false` = **1** |
| **Total** | **10** |

**Stub transparente em Layouter** (`layout_content`):
```rust
Content::Columns { count: _, gutter: _, body } => {
    self.layout_content(body);
}
```

Mantém pattern P156J `Repeat` — variant disponível em todo o
pipeline; counters/labels dentro do body resolvem via walk;
consumer multi-region real diferido a P219.

`Content::Columns` **não-locatable** (sem Tag::Start/End próprio);
walk recurse no body normalmente.

---

## §5 Decisões substantivas

- **Stub transparente vs consumer real**: arm Layouter delega
  a body ignorando count/gutter. Permite parsing E2E
  `#columns(2)[texto]` + introspect sem semantic real.
  Pattern emergente "stub transparente para variant aditivo"
  (precedente P156J `Repeat` single-render diferido).
- **Separação P217 (variant) + P218 (stdlib)** por
  atomização ADR-0036. Anti-inflação **12ª aplicação
  cumulativa** pós-P205D — features distintas em sub-passos
  distintos. Precedente literal P156J `Repeat` (variant +
  stdlib materializados separados de refino lazy).
- **ADR-0064 Caso C cumulativo N=cresce** — `gutter:
  Option<Length>`. Patamar consolidado.
- **Content variants 54 → 55** (P217 +1). Tabela B.2
  inventário 148 a actualizar em P221 encerramento série
  (consistente com pattern: cobertura categoria sobe quando
  feature user-facing materializada — P217 não muda §A.5
  Layout porque consumer real diferido a P219).
- **Pattern "refactor stacking" preservado N=1** — P217
  acede `self.regions.current.X` no arm Layout (3 níveis
  stacking pós-P216A+B); sem nova camada (P217 não refactora
  P216B output). Pattern estável.
- **L0 `content.md` não tocado em P217** — variant
  inline-documentado per convenção emergente (precedente
  stdlib funcs P169+ inline). Reduz inflação documental.
  Extensão L0 fica para passo dedicado se humano julgar útil.

---

## §6 Resultados verificação

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 1952 verdes | **1952 verdes** (1663 + 242 + 24 + 2 + 21) |
| `crystalline-lint .` | 0 violations | **0 violations** |
| `crystalline-lint --fix-hashes` | sync se necessário | "Nothing to fix" (L0s não tocados) |
| Tests P217 novos | 5 unit + 1 E2E = 6 | ✓ 6 verdes |
| Mudança observable | 0 | **0** (1946 → 1952 = +6 sentinelas; nenhum test pre-existente regrediu) |
| Variant count Content | 54 → 55 | ✓ |
| Borrow checker quebras | 0 | **0** |
| Ajustes manuais | 0-2 | **1** (closure `Ok(Some(x.clone()))` em test map_content per signature) |

---

## §7 Inventário 148 + ADR-0078 anotação

**Inventário 148**:
- §A.5 Layout linha `columns(n)` **mantém-se `ausente`** —
  reclassificação a `implementado` ocorre só pós-P219+P220
  (consumer real + colbreak). Documentado em ADR-0078 P217
  bloco.
- Tabela B.2 actualização diferida a P221 encerramento série
  (consistente com pattern: variant aditivo sem consumer real
  ainda).

**ADR-0078** §"Plano de materialização" anotada com bloco
`### P217 materializado 2026-05-12`:
- Variant + 10 arms documentados.
- Stub transparente Layouter explícito.
- Stdlib `native_columns` diferido P218.
- ADR-0064 Caso C cumulativo registado.
- 6 tests adicionados.
- 1952 verdes / 0 violations.
- Sub-fase (b) DEBT-56: 1/4 sub-passos materializados.

**Status ADR-0078**: PROPOSTO mantido. Transição IMPLEMENTADO
só em P221 (6 condições satisfeitas).

---

## §8 Próximo sub-passo

P217 fecha primeiro sub-passo sub-fase (b) DEBT-56 (1/4).
Decisão humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P218** imediatamente — `native_columns(count, gutter: ?)` stdlib + scope register + validação `count >= 1` | S (~1h) | alta (continuação direta sub-fase b; aditivo trivial; humano fixou "focar no Layout") |
| **Caminho 2** | Pivot Bloco C P222 — `measure(body)` stdlib expose | S+ (~1-2h) | média (win rápido §A.9 estricto 83% → 100%; isolado de DEBT-56) |
| **Caminho 3** | Pivot P220 — `Content::Colbreak` variant + native_colbreak (skip P218 P219 — colbreak pode ser materializado independentemente como aditivo puro) | S+ (~1.5h) | baixa-média (rompe ordem natural P218→P219; menos coerente) |
| **Caminho 4** | Adiar Layout; outro módulo | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P218)** — continuação
direta P217; padrão mecânico stdlib registada; momentum
preservado. Pattern operacional consolidado (10 stdlib funcs
registadas em série Layout pós-P156C).

**Estado pós-P217**:
- Sub-fase (b) DEBT-56: 1/4 sub-passos.
- ADR-0078 PROPOSTO; ADR-0061 PROPOSTO ~50% concluído.
- Layout 78% preservado (`columns` ausente até P219+P220
  consumer real).
- Tests workspace: **1952 verdes**; `crystalline-lint`: **0
  violations**.
- Cumulativo P216A+B+P217 = ~325 substituições mecânicas + 1
  variant + 10 arms exhaustivos + 6 tests em 3 sessões sem
  mudança observable.
- 12 aplicações cumulativas anti-inflação pós-P205D
  (P217 separação variant/stdlib).
- Pattern emergente "stub transparente" N=1 (P217
  Layouter arm; possível N=2 em P220 Colbreak).
- Pattern "refactor stacking" preservado N=1 (estável; P217
  não cresce nova camada).
