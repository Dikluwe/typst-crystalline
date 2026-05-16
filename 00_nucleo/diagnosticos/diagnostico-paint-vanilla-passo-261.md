# Diagnóstico Paint vanilla — Passo 261 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-261.md` (spec).
**Análogo estrutural**: `diagnostico-color-vanilla-passo-257.md`
(P257 Color paridade vanilla).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085 §"Propriedades obrigatórias".

---

## §1 — Estrutura literal vanilla Paint

```rust
// lab/typst-original/crates/typst-library/src/visualize/paint.rs:10
pub enum Paint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Tiling),
}

impl Paint {
    pub fn unwrap_solid(&self) -> Color { ... }
    pub fn relative(&self) -> Smart<RelativeTo> { ... }
    pub fn as_decoration(&self) -> Self { ... }
}

impl Debug for Paint { ... }
impl From<Tiling> for Paint { ... }
impl Repr for Paint { ... }
impl<T: Into<Color>> From<T> for Paint { ... }  // 80
impl From<Gradient> for Paint { ... }            // 86
```

**3 variants vanilla**: `Solid` + `Gradient` + `Tiling`.
**Derives**: `Clone, Eq, PartialEq, Hash`. **Não Copy** (porque
`Gradient` e `Tiling` têm Arc); `Solid(Color)` poderia ser Copy
mas o enum como um todo não é.

**Conversões**:
- `impl<T: Into<Color>> From<T> for Paint` — implementação
  blanket que abrange `From<Color> for Paint`.
- `impl From<Gradient> for Paint`.
- `impl From<Tiling> for Paint`.

**Métodos**:
- `unwrap_solid()` — panic se não Solid (usado para texto).
- `relative()` — coordinate system.
- `as_decoration()` — converte para text decoration.

---

## §2 — Consumers cristalino actual (impacto)

### §2.1 — Stroke construções (literais)

```bash
$ grep -rn "Stroke {" 01_core/src/ 03_infra/src/ | head -30
```

Sítios identificados (~22 construções `Stroke { paint: ... }`):

| Ficheiro | Linhas (~hits) |
|----------|----------------|
| `01_core/src/entities/geometry.rs` | 86 (1 teste interno) |
| `01_core/src/entities/content.rs` | 4047, 4068, 4097, 4195, 4228, 4280, 4518, 4548, 4591 (9 tests) |
| `01_core/src/rules/layout/mod.rs` | 1142 (1 sítio real) |
| `01_core/src/rules/stdlib/layout.rs` | 361, 363, 1460 (3 sítios; 2 builders + 1 destrutivo) |
| `01_core/src/rules/stdlib/shapes.rs` | 63, 67, 100, 103, 151, 154, 203, 260 (8 sítios) |

**Total estimativa primeira ordem**: **~22 construções
literais**. Pode haver +5-10 em testes não detectados pelo grep
(e.g. construção via tuple/dict args). Magnitude real ~30.

### §2.2 — PDF exporter consumer

```bash
$ grep -n "s.paint.to_rgba_f32\|\.paint\b" 03_infra/src/export.rs | head -10
```

Sítios identificados (4 sítios em `03_infra/src/export.rs`):

| Linha | Padrão |
|-------|--------|
| 863 | `let (r, g, b, _) = s.paint.to_rgba_f32();` |
| 1125 | `let (r, g, b, _) = s.paint.to_rgba_f32();` |
| 1371 | `let (r, g, b, _) = s.paint.to_rgba_f32();` |
| 1553 | `let (r, g, b, _) = s.paint.to_rgba_f32();` |

**Adaptação**: `s.paint.to_rgba_f32()` → `s.paint.to_color().to_rgba_f32()`
(4 sítios; substituição literal).

### §2.3 — Style::Fill consumers (NÃO TOCA P261)

```bash
$ grep -rn "Style::Fill\|TextStyle.*fill" 01_core/src/ | head -10
```

`Style::Fill(Color)` e `TextStyle.fill: Option<Color>` permanecem
**literal preservado** per ADR-0039 SR-Struct Resolvido. P261
**não migra** estes para Paint — apenas Stroke.paint.

### §2.4 — Stdlib native_rgb (NÃO TOCA P261)

```bash
$ grep -n "Value::Color\|native_rgb\|native_luma" 01_core/src/rules/stdlib/foundations.rs | head
```

Stdlib funcs cor (`native_rgb`/`native_luma`/etc.) continuam
retornar `Value::Color`. P261 **não introduz** `Value::Paint`.
Paint::Solid é wrapper interno cristalino, transparente para
user-facing.

---

## §3 — Decisão Solid only vs full enum

| Variant Paint | Status P261 | Razão |
|---------------|-------------|-------|
| Solid(Color) | **Materializar** | wrapper para Stroke.paint |
| Gradient(Gradient) | **Comentário reserva** | Sem Gradient L1 (P259 confirmou ausência); activa em P262 |
| Tiling(Tiling) | **Comentário reserva** | Sem Tiling L1; baixa prioridade |

**Justificação minimalista**:
- Paridade pattern P25 → P257 (Color subset gradual; expansão
  consumer-driven).
- Política P158 "sem novas reservas": variants
  Gradient/Tiling como **comentários no enum** são roadmap
  visual, não DEBT/ADR nova.
- Magnitude controlada: enum com 1 variant + From<Color> é
  minimal; tests + adaptação consumers ~30 sítios é factível
  em S+ session.

**Conversão `From<Color> for Paint`** implementada (não
blanket `From<T: Into<Color>>` — simplificação face vanilla;
suficiente para call sites cristalino).

**Helper `Paint::solid(c)`** + `Paint::to_color()` para
consumer access.

---

## §4 — Decisão granularidade ADR

☑ **Opção α — ADR-0086 nova**.
☐ Opção β — Anotação cumulativa ADR-0083.

**Decisão escolhida**: **Opção α**.

**Justificação**:
- Paridade precedente ADR-0083 — cada tipo vanilla com
  scope-outs tem ADR próprio.
- ADR-0083 (Color) e ADR-0086 (Paint) tratam de tipos
  distintos; misturar âmbitos polui ADR-0083.
- Granularidade preservada per ADR-0084 critério "cobertura
  ambígua" — cada decisão arquitectural com ADR explícita
  facilita citação.
- +1 ADR aceitável (total 72 → 73 PROPOSTO + 73 IMPLEMENTADO
  pós-P261.D); paridade pattern P257 "ADR PROPOSTO+IMPLEMENTADO
  mesmo passo via Cenário B1".

---

## §5 — Plano materialização P261.C

### Sequência sub-passos

1. **C.1** — L0 prompt `entities/paint.md` novo (estrutura
   análoga `entities/color.md`).
2. **C.2** — `entities/paint.rs` — tests primeiro + impl
   (Solid only enum + impl + From<Color> + to_color +
   tests ~5-8 unidades).
3. **C.3** — `entities/mod.rs` — re-export Paint.
4. **C.4** — `entities/geometry.rs` — Stroke.paint Color →
   Paint; L0 prompt actualizado.
5. **C.5** — Adaptar ~30 sítios construção `Stroke { paint: ...
   }`:
   - **Pattern 1**: `paint: <expr>` literal Color → `paint:
     Paint::Solid(<expr>)`.
   - **Decisão**: usar `Paint::Solid(expr)` explícito (não
     `expr.into()`) — mais legível em testes; consistência;
     evita ambiguity de inference.
6. **C.6** — Adaptar 4 sítios PDF exporter (`s.paint.to_rgba_f32()`
   → `s.paint.to_color().to_rgba_f32()`).
7. **C.7** — Verificação: lint zero violations; tests
   2334 → 2339-2342 (+5-8 esperado per tests P261).

### Magnitude esperada

- **Cascade ~30 sítios literais** + 4 exporter + 5-8 tests
  novos = ~40 sítios edits.
- **Tempo real** estimado: 30-45 min com edits paralelos.

---

## §6 — Limitações conscientes

- **Paint::Solid only** — Gradient/Tiling comentários reserva
  no enum. Expansão P262+ consumer-driven.
- **TextStyle.fill: Option<Color> preservado literal** —
  ADR-0039 SR intacto.
- **Stdlib native_rgb continua retornar Value::Color** — sem
  alteração user-facing.
- **From<Color> for Paint apenas** (não blanket
  `From<T: Into<Color>>`) — simplificação face vanilla.
- **Sem `Paint::unwrap_solid()` panicking** — método omitido;
  cristalino usa `to_color()` que sempre funciona (Solid only
  garantido).
- **Sem `Paint::relative()`**, **sem `as_decoration()`** —
  métodos vanilla específicos de Gradient/Tiling; não relevantes
  para Solid only.
- **Sem `Debug` derivado custom** — `#[derive(Debug)]`
  suficiente para Solid only.
- **Não implementa Hash/Eq vanilla style** — `#[derive(...)]`
  cobre PartialEq; Hash/Eq adicionais não necessários para
  Stroke consumers actuais.

---

## §7 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório.
- ADR-0033 — Paridade observable vanilla.
- **ADR-0083** — Color paridade vanilla (precedente N=2 do
  mesmo pattern).
- **ADR-0085** — Diagnóstico imutável (este ficheiro cumpre).
- **ADR-0086** — Paint wrapper Solid only (a criar P261.B).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0065 — Inventariar primeiro (cumprido aqui).
- DEBT-1 — Fechado P142 (preservado).
- P252 — Stroke cross-cutting precedente N=1.
- P257 — Color paridade precedente N=2 (template "ADR
  PROPOSTO+IMPLEMENTADO mesmo passo" N=1).
- P259 §3 P259.C Opção 1 — spec preliminar Paint enum +
  Gradient Linear.
- P260 — ADRs meta (ADR-0084/0085 consumidos indirectamente).
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/paint.rs`
  (97 linhas; 3 variants + 5 métodos + 4 conversões).
