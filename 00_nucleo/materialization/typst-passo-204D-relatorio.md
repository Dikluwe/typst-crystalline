# Relatório do passo P204D

**Data de execução**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204D.md`.
**Natureza**: implementação focada Position concrete.
**Sub-passo `D` da série M8** — terceiro de 7 (B-H) per
ADR-0073.
**Magnitude planeada**: S–M.
**Magnitude real**: **S–M** (~70 min; 1 ficheiro novo +
3 modificados + 4 tests).

---

## §1 O que foi feito

P204D materializou Position concrete no cristalino —
o concern adiado por ADR-0066 (intermediário até M8) e
confirmado em P203 consolidado §13 como parte natural de
M8.

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204D-inventario.md`.

Conteúdo:
- §1 C1 inventário (6 sub-secções).
- §2 C2 API-decisão (migrar stub).
- §3 C6 trait API (C6a — TagIntrospector retorna None).
- §4 C5 ponto de emissão (advance_locator).
- §5 Alterações literais aplicadas (8 sub-secções).
- §6 Decisões.
- §7-§9 Verificações + métricas + critério.

Tamanho: ~13 KB.

### Output 2 — Alterações em código

7 ficheiros modificados/criados:

#### Production (01_core)

1. **`entities/position.rs`** (NOVO) — Tipo `Position`
   réplica vanilla `PagedPosition`; Hash manual via
   `to_bits()` (3 unit tests embebidos).
2. **`entities/mod.rs`** — `pub mod position`.
3. **`entities/layouter_runtime_state.rs`** — Field 4º
   `positions: HashMap<Location, Position>` adicionado.
4. **`entities/introspector.rs`** — Trait method
   `position_of` migra signature de `Option<()>` para
   `Option<Position>`; TagIntrospector impl retorna
   `None`.
5. **`rules/layout/mod.rs`** —
   `advance_locator_if_locatable` ganha emit Position
   single-pass (mesmo gating que set
   `current_location`).

#### Tests (01_core)

6. **`rules/layout/tests.rs`** — 2 sentinels P204D
   (`p204d_position_struct_existe`,
   `p204d_runtime_positions_field_existe`) + 2 E2E
   tests (locatable produces Position; non-locatable
   doesn't).

#### L0 prompt

7. **`00_nucleo/prompts/entities/position.md`** — Prompt
   L0 formal criado per CLAUDE.md Protocolo de
   Nucleação.

### Output 3 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P204D: ~3 min.
- C1 inventário (Point + LayouterRuntimeState +
  position_of + Layouter current_page detection):
  ~15 min.
- C2 + C6 decisões fixadas: ~5 min.
- C3 ficheiro `position.rs` novo (com 3 unit tests):
  ~10 min.
- C4 LayouterRuntimeState campo: ~3 min.
- C5 advance_locator populate: ~5 min.
- C2 trait migration (signature + impl): ~5 min.
- Verificação cargo build + iteração: ~3 min.
- C7 tests E2E (2): ~5 min.
- C11 sentinels (2): ~3 min.
- L0 prompt criação + `--fix-hashes`: ~5 min.
- Verificação final + lint: ~3 min.
- Inventário interno (Output 1): ~10 min.
- Este relatório: ~5 min.

**Total**: ~70 min.

---

## §3 Métricas

| Métrica | Pré-P204D | Pós-P204D | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1829 | **1836** | +7 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | baseline | +~80 | +80 |
| LOC tests | baseline | +~120 | +120 |
| L0 prompts novos | — | 1 | +1 |
| Tipos novos em L1 | — | 1 (`Position`) | +1 |
| Fields novos em LayouterRuntimeState | 3 | 4 | +1 |
| Trait API breaking | — | sim (`position_of` signature) | breaking interno |
| Ficheiros modificados | — | 7 | (5 código + 1 tests + 1 L0) |

---

## §4 Decisões tomadas durante a leitura

### 4.1 C2 = migrar stub (em vez de manter provisório)

- 0 consumers em produção (per P204A A3) — migração
  trivial.
- Trait API matches vanilla literal (per ADR-0073
  Padrão A).
- Tests stub `assert_eq!(_, None)` infere tipo
  correctly — sem alteração.

### 4.2 C6 = C6a (TagIntrospector retorna None)

- Cristalino single-pass: Position vive em runtime
  (Layouter), não em Introspector estrutural.
- TagIntrospector é construído pre-layout — sem acesso
  a Layouter runtime.
- Tracked é `&` borrow imutável após `.track()` —
  populating intr.positions impossível.
- Future PagedIntrospector pode override.

### 4.3 Hash via `to_bits()` em vez de Debug formatting

P204B usou Debug-based Hash para Value/Content (60+
variantes / complex types). P204D opta por explicit
field hashing porque Position só tem 3 fields simples.

```rust
impl Hash for Position {
    fn hash<H>(&self, state: &mut H) {
        self.page.hash(state);
        self.point.x.val().to_bits().hash(state);
        self.point.y.val().to_bits().hash(state);
    }
}
```

Mais eficiente; Position é hot path em cache lookups
comemo.

### 4.4 Single canonical site no Layouter — `advance_locator_if_locatable`

Não distribuir populate por arms de `layout_content`.
Vantagens:
- 1 site em vez de N — mantenance simples.
- Garantia paridade com `current_location`.
- Mirror exacto do gating já existente.

### 4.5 L0 prompt criado em P204D

Per CLAUDE.md "Protocolo de Nucleação", L0 deve preceder
L1. Criado em P204D para satisfazer linter V1.
`--fix-hashes` syncronizou hash automaticamente.

Decisão honesta: spec assumiu preparação prévia. L0
criado aqui mantém workflow auditável.

### 4.6 0 alterações em consumers existentes

Tests stub continuam sem alteração (assert None infere
tipo). Production consumers 0. Trait migration é
breaking semanticamente mas no-op funcionalmente.

### 4.7 Sem `Eq` derive em Position

`f64` em Pt bloqueia. Position fica com PartialEq
apenas. Acceptable — comparisons usam `==` (PartialEq).

---

## §5 Sugestão para próximo sub-passo (não-vinculativa)

**P204E — `crystalline_evict()` wrapper** (per ADR-0073
plano de materialização).

Trabalho concreto P204E:
1. Wrapper L4 em `04_wiring/src/eviction.rs` (ou
   similar).
2. Re-export `comemo::evict` via API cristalina.
3. Sem CLI integration (futuro pós-M8).

Magnitude esperada: **S** (trivial wrapper).

Pré-condição cumprida por P204D:
- Position concrete materializada ✅.
- Trait API estabilizada (Option<Position>) ✅.
- Tests verdes (1836) ✅.
- Lint 0 violations ✅.

---

## §6 Critério de progressão respeitado

Per spec §3 C13, P204D está concluído quando:

- [x] C1 inventário completo (6 sub-secções).
- [x] C2 API-decisão fixada (migrar stub).
- [x] C3 tipo `Position` criado.
- [x] C4 sub-store `runtime.positions` adicionado.
- [x] C5 população single-pass aplicada.
- [x] C6 trait API resolvida (C6a).
- [x] C7 tests E2E (2 — locatable + non-locatable).
- [x] C8 compilação verde.
- [x] C9 tests workspace verdes (1836).
- [x] C10 linter 0 violations.
- [x] C11 sentinelas (2 adicionadas).
- [x] Inventário registado (Output 1).
- [x] Relatório escrito (este).

**Sem `P204D.div-N`** registadas — sem divergências
empíricas relevantes.

---

## §7 Não-objectivos respeitados

Per spec §7, P204D não:

- [x] Não adicionou `evict()` wrapper (P204E).
- [x] Não adicionou ficheiros ao corpus de paridade
  (P204F).
- [x] Não adicionou benchmarks (P204G).
- [x] Não transitou ADR-0073 para ACEITE (P204H).
- [x] Não transitou ADR-0066 para superseded (P204H).
- [x] Não criou ADR nova.
- [x] Não tocou em consumers fora dos sites
  necessários.
- [x] Não modificou loops fixpoint.
- [x] Não materializou sub-stores trackable
  separadamente.
- [x] Não materializou Position no walk-time.
- [x] Não materializou Position via fixpoint.

---

## §8 Achados resumo

| Achado | Implicação |
|--------|-----------|
| Point existe em layout_types.rs (não derive Hash; Pt(f64) bloqueia) | Position precisa Hash manual via `to_bits()` |
| Tipo `Position` réplica vanilla `PagedPosition { page: NonZeroUsize, point: Point }` | Padrão A (paridade vanilla literal) directamente |
| TagIntrospector sem acesso a Layouter runtime | C6a obrigatório — TagIntrospector retorna None |
| Single canonical site `advance_locator_if_locatable` | Populate em 1 site (não N arms) |
| 0 consumers production de `position_of` | Migration breaking semanticamente sem impacto |
| L0 prompt criado em P204D | Per CLAUDE.md Protocolo de Nucleação |

---

## §9 Notas operacionais

### 9.1 Magnitude real S-M alinhada com planeada

P204D foi S-M (~70 min). Sem inflação.

### 9.2 Diagnóstico-primeiro funcionou (de novo)

C1 inventário detectou pre-emptively:
- Point sem Hash (Pt(f64) bloqueia).
- Single canonical site `advance_locator_if_locatable`.
- L0 prompt em falta.

Resoluções aplicadas in-flight sem `P204D.div-N`.

### 9.3 Trabalho útil cumulativo

P204A + P204B + P204C + P204D juntos:
- Auditoria empírica (16 cláusulas).
- Diagnóstico (14 cláusulas).
- ADR-0073 PROPOSTO.
- Trait `Introspector` trackable + Send + Sync.
- 3 Hash impls (Value, BibEntry, Content) — P204B.
- Layouter migração para Tracked (P204C).
- Position concrete materializada (P204D).
- 7 sentinel tests activos (3 P204B + 2 P204C +
  2 P204D).

P204E (wrapper evict) pode iniciar imediatamente.

### 9.4 Paridade vanilla observable

Position concrete cristalino é forma equivalente a
`PagedPosition` vanilla (page + point). Pipeline difere
(single-pass vs post-layout) mas saída observable
mantém-se equivalente — ADR-0033 preservada.

### 9.5 Performance

- Hash via `to_bits()` — O(1).
- `runtime.positions.insert(loc, ...)` — O(1) amortizado.
- Zero overhead em non-locatable contents.
- Sem alocação adicional além da entry em HashMap.

---

**Fim do relatório P204D.**
