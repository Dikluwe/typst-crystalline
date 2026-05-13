# Relatório do passo P216B — Sub-fase (a) parte 2: `Regions` wrapper minimal

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-216B.md`.
**Tipo**: refactor estrutural sem mudança observable.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: S (~1h).
**Marco**: nenhum (quinto passo pós-M9c; **fecha sub-fase (a)
DEBT-56 estructuralmente**).

---

## §1 O que foi feito

P216B introduziu struct `Regions` minimal cohabitando com
`Region` em `entities/region.rs`. Forma `{ current: Region }`
fixada por **anti-inflação 11ª aplicação cumulativa pós-P205D**
— fields `backlog: Vec<Region>` + `last: Option<Region>`
diferidos a P219 (consumer multi-column real). `Layouter`
struct refactored: `region: Region` → `regions: Regions`;
158 call-sites refactored mecânicamente via `sed`. Sub-fase
(a) DEBT-56 fechada estruturalmente. Tests: 1946 verdes
(1943 P216A + 3 P216B sentinelas). 0 violations preservadas.
Sem `P216B.div-N`.

---

## §2 Confirmação inventário call-sites pós-P216A

| Ficheiro | `self.region.` count |
|----------|----------------------|
| `mod.rs` | 107 |
| `cursor.rs` | 22 |
| `equation.rs` | 8 |
| `grid.rs` | 12 |
| `placement.rs` | 9 |
| `tests.rs` | 0 (mas 2 refs `l.region.X` — corrigidos manualmente) |
| **Total** | **158** |

Próximo a P216A's 167 (-5.4%); dentro de ±10% tolerated. **Sem
`P216B.div-1`** registada.

---

## §3 `Regions` minimal adicionado

**Estrutura fixada** (anti-inflação 11ª aplicação):

```rust
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: Region,
    // backlog: Vec<Region> — DIFERIDO P219 (consumer multi-column)
    // last:    Option<Region> — DIFERIDO P219
}

impl Regions {
    pub fn single(width, height) -> Self { ... }
    pub fn reset_current(&mut self) { ... }
}
```

**Justificação literal anti-inflação**:
- `backlog`/`last` só fazem sentido com multi-column real.
- Em single-region (P216B), ambos sempre vazios/None.
- Zero consumers reais em P216B.
- Precedente literal P205D `SealedLabelPages` deferred.

**Critério de reabertura `backlog`/`last`**: materialização de
`Content::Columns` consumer no Layouter (P219). Documentado
em L0 region.md + ADR-0078 anotação.

**3 sentinelas P216B** em `region.rs::tests`:
- `p216b_regions_single_cria_current_com_dimensoes`
- `p216b_regions_reset_current_delega`
- `p216b_regions_clone_funciona`

Total tests `region.rs::tests`: 4 (P216A) + 3 (P216B) = **7**.

**Cohabitação L0 N=2**: `Region` + `Regions` no mesmo módulo
+ mesmo L0 prompt — precedente `Sides<T>` em sides.md
(struct + helpers cohabitam). Sem L0 separado para `Regions`.

---

## §4 Layouter refactor `region` → `regions.current`

**Field declaração**:

Antes (P216A):
```rust
pub(super) region: crate::entities::region::Region,
```

Depois (P216B):
```rust
/// P216B (DEBT-56 sub-fase a parte 2): agregação em `Regions`
/// wrapper. Single-region em P216B; multi-region em P219.
pub(super) regions: crate::entities::region::Regions,
```

**`Layouter::new`**:

Antes:
```rust
region: { let mut r = Region::new(cfg.width, cfg.height); ...; r },
```

Depois:
```rust
regions: { let mut rs = Regions::single(cfg.width, cfg.height); ...; rs },
```

**Substituição mecânica** via `sed
's/self\.region\./self.regions.current./g'`:

| Ficheiro | Substituições |
|----------|----------------|
| `mod.rs` | 107 |
| `cursor.rs` | 22 |
| `equation.rs` | 8 |
| `grid.rs` | 12 |
| `placement.rs` | 9 |
| **Total sed** | **158** |

**Ajustes manuais** (2 pontos):
1. `Layouter::new` field init (struct field rename `region` →
   `regions`; init wrapped em `Regions::single`).
2. `tests.rs` — 2 refs `l.region.cursor_y` →
   `l.regions.current.cursor_y` (tests.rs estava fora da sed
   list; substituição literal manual via Edit tool
   `replace_all`).

Pattern `Content::SetPage` arm em `mod.rs:761-763` (sincronização
P216A): herdou substituição uniforme — `self.region.width =
page_config.width` → `self.regions.current.width = page_config.width`.
Sem novo ajuste manual.

---

## §5 Decisões substantivas

- **Forma minimal vs rica vanilla** (Caminho fixado: minimal):
  precedente literal P205D `SealedLabelPages` deferred.
  10 aplicações cumulativas anti-inflação pós-P205D; P216B
  é **11ª aplicação** consecutiva. Pattern operacional
  consolidado.
- **Cohabitação L0 N=2**: `Region` + `Regions` no mesmo
  módulo + mesmo L0 prompt — sem L0 separado para `Regions`.
  Precedente `Sides<T>` em `sides.rs`/`sides.md`. Reduz
  inflação documental.
- **Pattern emergente "refactor stacking" N=1**: P216B
  refactora output P216A (`self.region.X` → `self.regions.current.X`).
  Sítios viraram `self.regions.current.cursor_x` (verboso vs
  P216A `self.region.cursor_x` original `self.cursor_x`).
  Possível pattern N=2 se P217+ stack sobre P216B; promoção
  a meta diferida (N=3-4 política consistente).
- **Helper `reset_current`** mantido em `Regions` (não
  helper `Layouter`): conveniência semântica simétrica com
  `Region::reset`. Não viola Opção α (anti-inflação aplica-se
  a helpers no `Layouter`, não em tipos L1).
- **Sub-fase (a) DEBT-56 fechada estruturalmente**: P216A
  + P216B agregaram state geométrico Layouter em `Region` e
  agora em `Regions` wrapper. Pre-condição cumprida para
  sub-fase (b) consumer multi-column (P219).

---

## §6 Resultados verificação

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 1946 verdes | **1946 verdes** (1657 + 242 + 24 + 2 + 21 + ignored) |
| `crystalline-lint .` | 0 violations | **0 violations** |
| `crystalline-lint --fix-hashes` | sync se necessário | 1 hash sincronizada (region.rs ↔ region.md) |
| Tests P216B novos | 3 sentinelas | ✓ 3 sentinelas verdes |
| Mudança observable | 0 | **0** (1943 → 1946 = +3 sentinelas; nenhum test pre-existente regrediu) |
| Substituições mecânicas | ~158 | **158** sed + 2 manuais (Layouter::new + tests.rs) |
| Borrow checker quebras | 0-3 | **0** (refactor estritamente uniforme) |

---

## §7 ADR-0078 anotação P216B + sub-fase (a) fechada

ADR-0078 §"Plano de materialização" anotada com bloco
`### P216B materializado 2026-05-12`:

- Forma minimal `{ current: Region }` documentada.
- Anti-inflação 11ª aplicação registada.
- Cohabitação L0 N=2 documentada.
- Pattern emergente "refactor stacking" N=1 registado.
- 158 substituições mecânicas + 2 ajustes manuais.
- 1946 verdes / 0 violations.
- **Sub-fase (a) DEBT-56 fechada estruturalmente**.

Status ADR-0078: **PROPOSTO mantido**. Transição IMPLEMENTADO
só em P221 (encerramento Fase 3 + 6 condições satisfeitas).

---

## §8 Próximo sub-passo

P216B fecha sub-fase (a) inteira de DEBT-56. Decisão humana
sobre próxima sessão entre 3 opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | P217 imediatamente — `Content::Columns { count, gutter, body }` variant + arms exhaustivos; aditivo puro sem refactor | S+ (~1.5h) | alta (humano fixou "focar no Layout até onde der"; momentum P216A→B preservado) |
| **Caminho 2** | Pivot Bloco C opcional P222 — `measure(body)` stdlib expose; isolado, não bloqueia DEBT-56 | S+ (~1-2h) | média (win rápido §A.9 estricto 83% → 100%; alternativa "fechar gap pequeno antes de mergulhar em P217+") |
| **Caminho 3** | Adiar Layout; voltar a outro módulo (Model hayagriva DEBT-55; recálculo categoria; etc.) | varia | baixa-média (sub-fase a fechada; momentum perde) |

**Recomendação subjectiva**: **Caminho 1** consistente com
orientação humana ("focar no Layout até onde der") +
momentum cumulativo P215→P216A→P216B + pattern operacional
"refactor stacking" se P217 stack sobre P216B (promove N=1
→ 2; aproxima limiar formalização N=3-4).

**Estado pós-P216B**:
- Sub-fase (a) DEBT-56 fechada estruturalmente (P216A+B).
- Sub-fase (b) DEBT-56 começa em P217 (aditivo `Content::Columns`)
  ou P219 (consumer real).
- ADR-0078 PROPOSTO; ADR-0061 PROPOSTO 50%+ concluído.
- Layout 78% preservado (refactor sem mudança observable
  user-facing).
- Tests workspace: **1946 verdes**; `crystalline-lint`: **0
  violations**.
- Cumulativo P216A+P216B = **~325 substituições mecânicas em
  2 sessões** sem mudança observable. Demonstra viabilidade
  do pattern "decomposição empírica de magnitude" P215.div-1.
- 11 aplicações cumulativas anti-inflação (P205D + P207E +
  P208B C1 + P208D + P209C-vazios + P209D C6 + P209E C1.2 +
  P210 Caminho 3 + P211A + P216A Opção α + **P216B Regions
  minimal**).
