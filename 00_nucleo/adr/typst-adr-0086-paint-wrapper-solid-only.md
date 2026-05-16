# ⚖️ ADR-0086: Paint wrapper enum com subset materializado (Solid only)

**Status**: `IMPLEMENTADO`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 261.B (criação PROPOSTO) → Passo 261.D
(promoção `IMPLEMENTADO` pós-materialização).
**Aplicação**:
`00_nucleo/materialization/typst-passo-261-relatorio.md`.
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
(imutável per ADR-0085).
**Análogo estrutural**: ADR-0083 (Color paridade vanilla com
subset materializado P257).

---

## Contexto

Visualize vanilla define:

```rust
// lab/typst-original/.../visualize/paint.rs
pub enum Paint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Tiling),
}
```

Cristalino actual (pré-P261) usa `Stroke.paint: Color` directo
sem wrapper. P259 audit Fase A (Visualize Cenário B2 §3 Opção 1)
identificou Paint enum como **pré-requisito arquitectural** para
Gradient real consumer:

- Sem Paint wrapper: `Stroke { paint: Color }` apenas; Gradient
  futuro exigiria refactor cross-cutting.
- Com Paint wrapper Solid only: `Stroke { paint: Paint::Solid(c) }`;
  Gradient adicionado em P262+ como nova variant sem refactor
  Stroke.

P259 Cenário B2 §3 Opção 1 spec preliminar de Paint enum (M+S+;
+11pp Visualize). P261 executa sub-passo 1 (Paint Solid only);
P262 candidato sub-passo 2 (Gradient Linear).

ADR-0029 §"Simplificações aceites apenas com ADR explícita"
obriga ADR para subset materializado vs vanilla full. Paridade
pattern N=2 cumulativo com ADR-0083 (Color subset materializado
P257).

---

## Decisão

### Subset materializado P261 — Solid only

```rust
// 01_core/src/entities/paint.rs
pub enum Paint {
    Solid(Color),
    // Gradient(Gradient),  // P262 — comentário reserva
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self { Paint::Solid(c) }
    pub fn to_color(&self) -> Color { match self { Paint::Solid(c) => *c } }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self { Paint::Solid(c) }
}
```

**Derives**: `Debug, Clone, Copy, PartialEq` (`Color` é Copy
trivialmente em cristalino; Paint::Solid também).

**Variants `Gradient`/`Tiling`**: **comentários reserva no
enum**. Não unit placeholders. Política P158 "sem novas
reservas" — comentários são roadmap visual, não DEBT/ADR novo.

### Adaptação consumers — Stroke.paint Color → Paint

```rust
// 01_core/src/entities/geometry.rs
pub struct Stroke {
    pub paint:     Paint,       // antes: Color (P261)
    pub thickness: f64,
    pub overhang:  bool,        // P252
}
```

**Cascade ~30 sítios construção** `Stroke { paint: Color, ... }`
→ `Stroke { paint: Paint::Solid(Color), ... }`. Magnitude
controlada (refactor mecânico).

**4 sítios PDF exporter** `s.paint.to_rgba_f32()` →
`s.paint.to_color().to_rgba_f32()`.

### Preservações arquitecturais

- **ADR-0039 SR-Struct Resolvido**: `TextStyle.fill: Option<Color>`
  **inalterado**. P261 **não migra** TextStyle.fill para
  Option<Paint>. Apenas Stroke.paint adapta.
- **Style::Fill(Color)** (StyleChain variant): **inalterado**.
- **Stdlib `native_rgb`/`native_luma`/etc.**: continuam retornar
  `Value::Color`. P261 **não introduz** `Value::Paint`.
- **DEBT-1** (fechado P142): preservado.

### Scope-outs documentados

| Scope-out | Razão | Resolução prevista |
|-----------|-------|---------------------|
| `Paint::Gradient(Gradient)` | Sem Gradient L1 materializado (P259 confirmou) | **P262** — Gradient Linear materializa `Gradient` + activa variant |
| `Paint::Tiling(Tiling)` | Sem Tiling L1; baixa prioridade | Passo futuro sem prioridade designada |
| `From<T: Into<Color>>` blanket | Simplificação face vanilla | Não previsto (uso cristalino directo) |
| `Paint::unwrap_solid()` (panic) | Solid only garantido | Não previsto |
| `Paint::relative()` | Específico Gradient/Tiling | Activa quando Gradient materializar |
| `Paint::as_decoration()` | Específico Gradient text | Activa quando Gradient texto materializar |
| `impl Hash for Paint` | Não requerido por consumers actuais | Refino futuro se cache exigir |

### Compatibilidade com pré-existente

- `Stroke { paint: c, ... }` legacy: **falha compile** sem
  edição (struct literal não aceita coerção automática).
- **Solução**: refactor mecânico ~30 sítios via `Paint::Solid(c)`
  literal.
- **Alternativa rejeitada**: macro construção `stroke!(paint=c, ...)`
  — over-engineering para refactor pontual.

---

## Consequências

### Positivas

- **Desbloqueia Gradient real consumer** em P262 sem refactor
  Stroke novo.
- **Paridade vanilla observable** preservada (Paint::Solid(c)
  comporta-se identicamente a Color directo para PDF output).
- **From<Color>** simplifica idioms em código futuro
  (`paint: c.into()` ou `Paint::Solid(c)`).
- **Granularidade arquitectural** — Stroke.paint independente
  de TextStyle.fill (ADR-0039 preservado).

### Negativas

- **+1 indirecção em Stroke**: `s.paint.to_color()` vs `s.paint`
  directo. Custo runtime zero (match + deref).
- **Cascade ~30 sítios refactor mecânico**. Magnitude S+
  controlada.

### Neutras

- **Variants reserva (comentários)** no enum: roadmap visual;
  não DEBT/ADR novo per política P158.
- **Cobertura Visualize**: +1pp estructural (1 entrada da
  Tabela A P259 §G "Paint wrapper" promovida ausente →
  implementado).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| α1 — Color directo (status quo) | Zero refactor | Bloqueia Gradient consumer real; cascade refactor futuro maior |
| **α2 — Paint::Solid only (escolhida)** | **Minimalismo + roadmap visível + magnitude controlada** | **Cascade ~30 sítios refactor mecânico (one-shot)** |
| α3 — Paint full enum com Gradient/Tiling unit placeholders | Enum completo desde já | Variants vazios sem consumer poluem enum; tests artificiais |
| β — Anotação cumulativa ADR-0083 | -1 ADR | Mistura âmbitos (Color paridade + Paint wrapper); paridade pattern ADR-0083 cada tipo com ADR próprio quebrada |

**Decisão**: **α2 (Paint::Solid only) + Opção α (ADR-0086 nova)**
per paridade ADR-0083.

---

## Critério revisão

ADR-0086 transita `IMPLEMENTADO` → expansão real quando:

1. **P262 Gradient Linear** materializa `entities/gradient.rs`
   → activa `Paint::Gradient(Gradient)` variant (descomentar
   linha + adaptar consumers Stroke se exigir).
2. **Tiling pattern** materializa (baixa prioridade) → activa
   `Paint::Tiling(Tiling)`.

Cada activação é **passo dedicado pequeno** (XS-S) per pattern
P262+; sem DEBT novo per política P158.

---

## Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2"

Cumulativo:
- N=1 P257 (ADR-0083 Color paridade vanilla).
- **N=2 P261** (ADR-0086 Paint wrapper Solid only; este passo).

**Patamar N=2 reforça pattern**. N=3 começará a ser candidato
a formalização meta (improvável; padrão auto-documentado).

---

## Subpadrão "Refactor cross-cutting entity primitivo" cresce N=2 → N=3

Cumulativo:
- N=1 P252 (Stroke `overhang` cross-cutting).
- N=2 P257 (Color expansão cross-cutting; toca exporter +
  stdlib + variants).
- **N=3 P261** (Paint cross-cutting Stroke.paint).

**Patamar N=3 atinge limiar formalização sólida**. Análogo
ADR-0064/0065 podem citar este pattern futuramente; **candidato
meta-ADR** se aplicações continuarem.

---

## Referências

- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- **ADR-0083** — Color paridade vanilla (precedente N=2 do
  pattern; análogo estrutural).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (consumo indirecto P261; diagnóstico Paint vanilla
  cumpre forma análoga).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (scope-outs aceites).
- ADR-0065 — Inventariar primeiro (cumprido em P261.A).
- DEBT-1 — fechado P142 (preservado).
- P252 — Stroke `overhang` cross-cutting (precedente N=1).
- P257 — Color paridade vanilla 8/8 (precedente N=2; template
  PROPOSTO+IMPLEMENTADO mesmo passo).
- P259 §3 Opção 1 — spec preliminar Paint enum + Gradient
  Linear.
- P260 — ADRs meta (formaliza padrões).
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  — diagnóstico imutável P261.A.
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — audit precedente Visualize.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/paint.rs`
  — fonte canónica (3 variants).

---

## Próximos passos

1. P261.C executa materialização imediata (paint.rs + Stroke
   refactor + consumers).
2. P261.D promove ADR-0086 → `IMPLEMENTADO`.
3. P262 (futuro) — Gradient Linear materializa
   `entities/gradient.rs` + activa `Paint::Gradient` variant.
