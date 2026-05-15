# Relatório do passo P257 — Color paridade vanilla (Leitura B funcional)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-257.md`.
**Tipo**: passo composto sequencial (P257.A diagnóstico + P257.B
ADR + P257.C materialização + P257.D promoção).
**Magnitude planeada**: M-L. **Magnitude real**: **M (~3-4h)**
— diagnóstico vanilla literal acelerou decisões; cascade
replace_all em ~5 consumers (vs ~15-20 estimados) reduziu
adaptações.

---

## §1 Sumário executivo

**Cenário**: ADR-0029 §"Diagnosticar primeiro" + §"Simplificações
aceites apenas com ADR explícita" cumprido literalmente:

- **P257.A** diagnóstico vanilla imutável criado (`diagnostico-color-vanilla-passo-257.md`).
- **P257.B** ADR-0083 PROPOSTO criada com 4 scope-outs documentados.
- **P257.C** 8 variantes materializadas + 7 stdlib funcs novas +
  5 consumers adaptados + PDF exporter preservado.
- **P257.D** ADR-0083 promovida PROPOSTO → IMPLEMENTADO no mesmo passo.

**Tests delta**: **2304 → 2334 verdes (+30 P257)**:
- 22 unit em `entities/color.rs` (8 variantes; ≥2 tests por
  espaço; PartialEq exacto; Copy/Clone).
- 8 unit em `stdlib/mod.rs` tests (`native_oklab`, `native_oklch`,
  `native_linear_rgb`, `native_cmyk`, `native_hsl`, `native_hsv`
  + 2 oklab variants + 1 refactor `stdlib_luma` adaptado).

**ADRs tocadas**: **1 nova (ADR-0083)** + 0 anotações cumulativas.

**Prompts L0 criados**: 1 (`entities/color.md`; hash `7188e8d9`
propagado).

**Hashes propagados**: `entities/color.rs` recebeu
`@prompt-hash 20a91590` via `crystalline-lint --fix-hashes`.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
  (imutável per ADR-0034; §1-§10 preenchidos).
- `00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md`.
- `00_nucleo/prompts/entities/color.md`.
- `01_core/src/entities/color.rs` (~500 LoC incluindo 22 tests).
- `00_nucleo/materialization/typst-passo-257-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `01_core/src/entities/mod.rs` (+1 `pub mod color;`).
- `01_core/src/entities/layout_types.rs` (Color removido +
  re-export `pub use crate::entities::color::Color;`).
- `01_core/src/rules/stdlib/foundations.rs` (+6 funcs novas;
  refactor `native_luma`).
- `01_core/src/rules/stdlib/mod.rs` (+6 re-exports; 8 unit tests
  + refactor `stdlib_luma`).
- `01_core/src/rules/eval/mod.rs` (+6 imports; +6 scope.define).
- `01_core/src/rules/layout/tests.rs` (5 sítios pattern-match
  adaptados de `Color::Rgb { r, g, b }` para `*c == Color::rgb(r, g, b)`).
- `00_nucleo/adr/README.md` (distribuição IMPLEMENTADO 24 → 25;
  entrada P257 nos passos-chave).

**Sem código L3/L4 tocado** (PDF exporter preservado via
`to_rgba_f32` API estável; novos espaços convertem
transparentemente).

---

## §2 Sub-passo P257.A — Fase A diagnóstico vanilla

**Output resumido** (detalhes literais em
`diagnostico-color-vanilla-passo-257.md`):

### Vanilla Color (literal)

`lab/typst-original/crates/typst-library/src/visualize/color.rs`
(1996 linhas) define **enum `Color` com 8 variantes** (linha
194): `Luma`, `Oklab`, `Oklch`, `Rgb`, `LinearRgb`, `Cmyk`,
`Hsl`, `Hsv`. Componentes f32 internos.

### Cristalino actual

`entities/layout_types.rs:638` **enum 2 variantes**: `Rgb { r,g,b: u8 }`,
`Rgba { r,g,b,a: u8 }`. Cobertura 2/8 = 25%.

### Decisão Fase A §8

**Materializar 8 variantes** com paridade estrutural total
(representação interna f32 paridade vanilla); **scope-out
formal documentado em ADR-0083**:

1. PDF native `/DeviceCMYK` — converte para sRGB no exporter.
2. Operadores cor (`lighten`/`darken`/`mix`/etc.) — não
   materializados.
3. `ColorSpace` enum runtime — não materializado.
4. Constantes nomeadas extras — refino incremental ADR-0080.

---

## §3 Sub-passo P257.B — ADR-0083 PROPOSTO criada

**Nome**: "Color paridade vanilla com subset materializado".

**Numeração**: ADR-0083 (próximo livre pós-ADR-0082; spec previa
ADR-0067 mas ocupada por `attribute-grammar-scoping`; paridade
pattern P226.div-1 + P249.div-2 + P257 nota numeração).

**Decisão**: enum tagged 8 variantes paridade estrutural literal
vanilla:

```rust
pub enum Color {
    Srgb { r, g, b, a: f32 },
    Luma { l, a: f32 },
    LinearRgb { r, g, b, a: f32 },
    Oklab { l, a, b, alpha: f32 },
    Oklch { l, c, h, alpha: f32 },
    Hsl { h, s, l, a: f32 },
    Hsv { h, s, v, a: f32 },
    Cmyk { c, m, y, k: f32 },
}
```

**Scope-outs formais documentados per ADR-0029
§"Simplificações aceites apenas com ADR explícita"**:

1. PDF native `/DeviceCMYK` — diferença + custo + critério
   revisão (passo **P-Color-CMYK-PDF** específico).
2. Operadores cor — passo dedicado por operador
   (`P-Color-Op-{lighten/darken/mix/etc.}`).
3. `ColorSpace` enum runtime — passo **P-Color-Space-Runtime**.
4. Constantes nomeadas extras — refino incremental sem ADR
   (cobre ADR-0080 EM VIGOR).

---

## §4 Sub-passo P257.C — Materialização

### C.1 — Prompt L0 `entities/color.md`

Criado seguindo padrão entities/* L0:
- Módulo, camada, propósito (referência ADR-0083).
- Tipo exportado: enum 8 variantes com docstrings.
- Métodos públicos: 10 construtores + 2 conversões.
- Comportamento: pureza L1 + paridade observable + PartialEq
  exacto.
- Critérios verificação: ≥2 tests por espaço.
- Localização + re-export.
- Sobre paridade vanilla com referências ADR-0083 + linha
  literal vanilla.

Hash propagado `7188e8d9` (em prompt) + `20a91590` (em código
via `@prompt-hash` line).

### C.2 — `01_core/src/entities/color.rs`

~500 LoC incluindo:

- Enum `Color` 8 variantes (`Debug`, `Copy`, `Clone`,
  `PartialEq` manual via `f32::to_bits`).
- 10 construtores públicos: `rgb`, `rgba`, `srgb_f32`, `luma`,
  `linear_rgb`, `oklab`, `oklch`, `hsl`, `hsv`, `cmyk`.
- `to_srgb() -> (u8, u8, u8, u8)` consumer PDF.
- `to_rgba_f32() -> (f32, f32, f32, f32)` preservado para
  compatibilidade hot path PDF exporter.
- Helpers conversão: `linear_to_srgb` (gamma encoding),
  `oklab_to_linear_rgb` (matriz LMS Björn Ottosson),
  `hsl_to_rgb`, `hsv_to_rgb`.
- 22 unit tests cobrindo:
  - sRGB: construção u8 paridade observable + roundtrip
    `to_srgb` + PartialEq exacto via bits.
  - Luma: construção + to_srgb cinza.
  - LinearRgb: construtor f32 + gamma inversa (linear 0.5 →
    sRGB ~188).
  - Oklab: construtor + L=1 → branco + L=0 → preto.
  - Oklch: construtor + chroma=0 → cinza.
  - Hsl: construtor + s=0 → cinza + vermelho puro
    HSL(0,100%,50%) → (255,0,0).
  - Hsv: construtor + s=0 v=1 → branco.
  - Cmyk: construtor + zero → branco + K=1 → preto.
  - Cross-variant: variants diferentes nunca eq + Copy/Clone.

### C.3 — Remoção Color de `layout_types.rs`

Substituído por `pub use crate::entities::color::Color;`
(re-export para compatibilidade hot path). Zero adaptações em
consumers que importam via `layout_types::Color`.

### C.4 — Adaptações consumers (5 sítios)

Tests em `layout/tests.rs` que faziam pattern-match
`Color::Rgb { r, g, b }` (variant antiga) adaptados para
`*c == Color::rgb(r, g, b)` comparison via equality:

- `:3442` p230 grid cell fill green.
- `:6432` p234 grid colspan green.
- `:6469` p234 grid rowspan blue.
- `:6708` p234 grid colspan 100pt width.
- `:6747` p234 grid 2x2 cell fill.

Construtores `Color::rgb(255, 0, 0)`/`Color::rgba(...)` em
todos os outros consumers (stroke, fill, style, value, stdlib)
**preservados literal** — paridade observable estricta.

### C.5 — PDF exporter intocado

4 caminhos `to_rgba_f32` em `03_infra/src/export.rs`
(linhas 857, 863, 1121, 1125, 1367, 1371, 1549, 1553)
**preservados literal**. Novos espaços convertem para sRGB
transparentemente via implementação `to_rgba_f32` no enum
expandido.

### C.6 — Stdlib funcs novas (6) + refactor (1)

`foundations.rs`:
- **Refactor** `native_luma`: agora constrói `Color::Luma` (não
  `Color::rgb(l,l,l)` cinza); paridade vanilla D65Gray; PDF
  output bit-equivalente via `to_srgb()`.
- **Novas (6)**: `native_oklab`, `native_oklch`,
  `native_linear_rgb`, `native_cmyk`, `native_hsl`, `native_hsv`.

`eval/mod.rs`: 6 imports + 6 `scope.define` para nomes literais
`oklab`, `oklch`, `linear_rgb`, `cmyk`, `hsl`, `hsv` registados
no stdlib scope.

`stdlib/mod.rs`: 6 re-exports + refactor `stdlib_luma` test
(compara via `to_srgb()` em vez de `Color::rgb(l,l,l)`) + 8
unit tests novos P257.

### C.7 — Verificação final P257.C

- `cargo build --workspace` → verde.
- `RUST_MIN_STACK=33554432 cargo test --workspace` → **2334
  verdes** (+30 P257; 0 regressões).
- `crystalline-lint .` → **`✓ No violations found`**.
- `crystalline-lint --fix-hashes .` → 1 hash propagado
  (`entities/color.md` → `7188e8d9` em prompt; `20a91590` no
  código).

---

## §5 Sub-passo P257.D — Promoção ADR + README + relatório

### D.1 — ADR-0083 PROPOSTO → IMPLEMENTADO

- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Data: `2026-05-15 (PROPOSTO P257.B; IMPLEMENTADO P257.D)`.
- Validado: P257.C materialização + 2304 → 2334 verdes +
  paridade observable preservada + 0 regressões + 4
  scope-outs documentados.

### D.2 — README ADRs

- IMPLEMENTADO 24 → **25** (+ADR-0083).
- PROPOSTO 11 preservado (ADR-0083 entra e sai no mesmo passo).
- Total 69 → **70**.
- Entrada P257 nos passos-chave (~40 linhas).

### D.3 — DEBT-4

DEBT-4 não referencia Color directamente (verificação rápida);
sem actualização necessária. Color simplificado P25 era a
origem mas DEBT-4 cobria outras questões. ADR-0028 (REVOGADA)
era o instrumento histórico — preservada como contexto.

### D.4 — Relatório P257 (este ficheiro)

Estrutura canónica seguida.

---

## §6 Padrões metodológicos

**ADR-0029 cumprida literal**: §"Diagnosticar primeiro" via
P257.A imutável; §"Não simplificar sob pretexto de adiamento"
via materialização 8 variantes paridade estrutural total;
§"Simplificações aceites apenas com ADR explícita" via ADR-0083
documentando 4 scope-outs com diferença+custo+critério revisão.

**Subpadrão "auditoria condicional"** N=3 → **N=4 cumulativo**:
- N=1 P192A.
- N=2 P255 (DEBT-8 Math).
- N=3 P257 (Color — Fase A diagnóstico vanilla literal).
- (P253 ADR-0079 promoção não conta — foi Cenário A scope-out
  formal, não audit condicional).

**Subpadrão "Diagnóstico imutável precedente à acção" N=1 → N=2
cumulativo** (P255 + P257).

**Subpadrão "Refactor cross-cutting entity primitivo"** N=1 →
**N=2 cumulativo** (P252 Stroke + **P257 Color**). Pattern
emergente sólido (entities primitivas com múltiplos consumers
cross-camada).

**Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via
Cenário B1"** N=1 inaugurado P257 — ADR-0083 cria-se PROPOSTO
em P257.B e logo promove-se em P257.D quando materialização
cumpre critério integralmente. Distinto de P229/P254
(promoção em passo administrativo XS posterior). Candidato
formalização N=3-4 futuro.

---

## §7 Cobertura

**Visualize ganha pp via Color expansion**:

- Antes P257: Color = 2 variantes (Rgb, Rgba); cobertura
  Visualize cor ≈ 25% (2/8 espaços).
- Pós-P257: Color = 8 variantes; cobertura Visualize cor
  **100% estructural** (8/8 espaços).
- Total Visualize cobertura per metodologia: subir ~5-10pp
  (Color é primeira entrada user-facing com maior peso).
- **Cobertura user-facing total**: ~75-76% → preservado (Color
  expansão é refino qualitativo da entrada existente).

**Layout**: ~98-99% preservado (P253 IMPLEMENTADO; Color
refactor não afecta Layout cobertura).

---

## §8 Limitações e trabalho futuro

**Scope-outs P257 documentados** (ADR-0083):

1. **PDF native `/DeviceCMYK`** — refino futuro
   **P-Color-CMYK-PDF** quando export print for prioritário.
2. **Operadores cor** (`lighten`/`darken`/`mix`/`saturate`/
   `desaturate`/`negate`) — refino futuro por operador
   (`P-Color-Op-{nome}`).
3. **`ColorSpace` enum runtime** — refino futuro
   **P-Color-Space-Runtime** quando uso surgir.
4. **Constantes nomeadas extras** (NAVY/PURPLE/etc.) — refino
   incremental via ADR-0080 EM VIGOR (sem ADR dedicada
   necessária).

**Sem DEBT novo aberto** — política P158 preservada.

**Sem ADR nova além de ADR-0083** — magnitude controlada;
scope-outs agregados em ADR única conforme spec §B.2 decisão.

---

## §9 Critério de aceitação global P257 — Checklist final

- [x] `crystalline-lint .` retorna `✓ No violations found`.
- [x] `cargo test --workspace` retorna **2334 verdes** (+30
  P257; range +20-40 esperado).
- [x] `diagnostico-color-vanilla-passo-257.md` criado em
  `00_nucleo/diagnosticos/` com §1-§10 preenchidos.
- [x] ADR-0083 criada e **promovida a IMPLEMENTADO**.
- [x] `00_nucleo/prompts/entities/color.md` criado.
- [x] `01_core/src/entities/color.rs` materializado (~500 LoC).
- [x] `01_core/src/entities/layout_types.rs` sem Color (apenas
  re-export).
- [x] Todos os consumers adaptados (Stroke, Style, FrameItem,
  Value, stdlib).
- [x] Exportador PDF preservado (4 caminhos `to_rgba_f32`).
- [x] Hashes propagados (`crystalline-lint --fix-hashes`).
- [x] README ADRs actualizado.
- [x] Relatório do passo criado.
- [x] Cada espaço materializado tem ≥2 tests (22 tests cobrem
  8 espaços; média 2.75 tests/espaço).
- [x] Paridade observable preservada (`Color::rgb(255,0,0).to_srgb()
  == (255, 0, 0, 255)`; PDF bit-equivalente).

**Estado pós-P257**:
- Tests workspace: **2334 verdes** (+30 P257).
- Hash drift: zero.
- Lint: zero violations.
- ADR-0083: IMPLEMENTADO.
- Prompts L0 criados: 1 (`entities/color.md`).
- Diagnóstico imutável criado: 1 (`diagnostico-color-vanilla-passo-257.md`).
- ADRs distribuição: PROPOSTO 11; EM VIGOR 30; IMPLEMENTADO
  **25**; **total 70**.
- Saldo DEBTs: 10 preservado.
- **45 aplicações cumulativas anti-inflação** pós-P205D.

**Marco P257**: Color paridade vanilla materializada (8/8
espaços estructural; 4 scope-outs documentados); cumpre regra
ADR-0029 literal; primeira aplicação cumulativa "ADR PROPOSTO+
IMPLEMENTADO no mesmo passo via Cenário B1" N=1 inaugurado;
segunda aplicação cumulativa "Refactor cross-cutting entity
primitivo" (Stroke P252 + Color P257). Visualize ganha expansão
substantiva user-facing.

**Recomendação subjectiva pós-P257**:

- **Pivot para Visualize/Text/Model** restantes (50-54% cobertura
  pré-P257; Color foi primeira entrada).
- **OU passo dedicado P-Color-CMYK-PDF** (scope-out #1) — XS-S
  conforme refactor exporter.
- **OU P256 (Model)** — sequência natural pós-Visualize Color
  expansion.

**Decisão humana fica em aberto literal** pós-P257.

---

## §10 Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0028** (REVOGADA P84.8c) — Color simplificado original.
- **ADR-0029** (EM VIGOR) — Pureza física L1 + obriga
  diagnóstico vanilla + ADR explícita para scope-outs.
- **ADR-0033** — Paridade observable vanilla.
- **ADR-0034** — Diagnóstico canónico (P257.A imutável).
- **ADR-0054** — Perfil graded.
- **ADR-0065** — Inventariar primeiro.
- **ADR-0080** — L0 minimal (cobre expansão futura constantes
  nomeadas).
- **ADR-0083** — Color paridade vanilla com subset materializado
  (criada P257.B; promovida P257.D).
- DEBT-4 (`00_nucleo/DEBT.md`) — não tocado P257 (origem Color
  simplificado mas refinos restantes não-Color).
- `lab/typst-original/crates/typst-library/src/visualize/color.rs`
  — fonte canónica (1996 linhas; 8 espaços).
- `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
  — fonte literal Fase A.
- P25 — Color simplificado original.
- P84.8c — ADR-0029 promulgada (revogou ADR-0028).
- P156C-G, P157A-C — precedentes materialização granular com
  tests-first.
- P229 — ADR-0080 PROPOSTO → EM VIGOR (precedente promoção
  passo administrativo).
- P252 — Refactor cross-cutting Stroke primitivo (precedente
  pattern).
- P253 — ADR-0079 Layout Fase 5 PROPOSTO → IMPLEMENTADO via
  Cenário A.
- P255 — DEBT-8 Math ENCERRADO via audit condicional (paralelo
  P257.A diagnóstico imutável).
