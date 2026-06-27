---

# P477 — `saturate`/`desaturate` + constantes de cor nomeadas

> **Passo:** 477
> **Data:** 2026-06-27
> **Foco:** (A) `Color::saturate` e `Color::desaturate` via Oklch chroma — fecha ADR-0083 §"Operadores cor" totalmente; (B) constantes de cor nomeadas alargadas (`parse_color` + ~13 cores adicionais) — fecha ADR-0083 §"Constantes nomeadas" parcialmente.
> **Trilha:** 4 — Extensão Color (refino final).
> **Tipo:** Materialização XS + XS.
> **Tamanho:** XS + XS (~20 min total).
> **ADR-0117 Cláusula 4:** `Color` em `entities/color.rs` (P257/P476); `to_oklch_p476` helper privado já existe (P476); `parse_color` em `rules/stdlib/shapes.rs`; módulo `color` em `rules/stdlib/color.rs` (P476). Verificar antes de propor.

---

## Contexto

P476 fechou 4/6 operadores de cor. Restam dois:

- **`saturate(amount)`** — aumenta a chroma `c` de Oklch; clamp `[0.0, ∞)`.
- **`desaturate(amount)`** — diminui a chroma `c`; clamp `[0.0, ∞)` (nunca negativa).

ADR-0083 §"Constantes nomeadas extras" é scope-out informal. O cristalino tem `parse_color` com 5 cores (`red`, `green`, `blue`, `black`, `white`). O vanilla tem 18+. Este passo adiciona as mais usadas em documentos reais sem exigir ADR nova (ADR-0080 EM VIGOR cobre refactors aditivos XS).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `Color::saturate` existe? | `entities/color.rs` | 🟡 |
| `Color::desaturate` existe? | idem | 🟡 |
| `to_oklch_p476` helper acessível no bloco P476? | `entities/color.rs:337–383` (P476) | ✅ |
| `native_color_saturate` existe em `stdlib/color.rs`? | `rules/stdlib/color.rs` (P476) | 🟡 |
| `parse_color` localização? | `rules/stdlib/shapes.rs` ou equivalente | 🟡 |
| Cores actualmente em `parse_color`? | idem | 🟡 |
| `make_color_module()` extensível com `saturate`/`desaturate`? | `rules/stdlib/color.rs:make_color_module` (P476) | ✅ |

**Sondas 🟡 com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — `saturate` e `desaturate`

### A.1 — Semântica

`saturate` e `desaturate` operam sobre a componente `c` (chroma) de Oklch:

- **`saturate(amount)`:** `c_new = c + amount`. Clamp mínimo `0.0` (chroma não negativa). Sem clamp superior — Oklch permite chroma > 1.0 para cores fora do gamut sRGB; o exporter trata via clamp na conversão.
- **`desaturate(amount)`:** `c_new = (c - amount).max(0.0)`. Equivale a `saturate(-amount)` com clamp.
- Cinzento puro (Oklch `c = 0.0`) desaturado permanece cinzento. Saturação de cinzento com hue preservado — `h` é preservado do valor original (mesmo que `c = 0.0`, `h` pode não ser zero dependendo da cor de origem).

### A.2 — Implementação em `entities/color.rs`

```rust
// P477 — saturate e desaturate via Oklch chroma.
impl Color {
    /// Aumenta saturação por `amount` (componente chroma Oklch).
    /// Chroma clampada ao mínimo 0.0; sem máximo (cores fora de gamut permitidas).
    /// Alpha preservado.
    pub fn saturate(self, amount: f32) -> Self {
        let (l, c, h, alpha) = to_oklch_p476(self);
        Color::oklch(l, (c + amount).max(0.0), h, alpha)
    }

    /// Diminui saturação por `amount` (componente chroma Oklch).
    /// Equivalente a `saturate(-amount)` com clamp mínimo 0.0.
    pub fn desaturate(self, amount: f32) -> Self {
        let (l, c, h, alpha) = to_oklch_p476(self);
        Color::oklch(l, (c - amount).max(0.0), h, alpha)
    }
}
```

**Nota:** `to_oklch_p476` é o helper privado introduzido em P476 (`entities/color.rs:337–383`). Confirmar via sonda que está acessível no mesmo arquivo — se sim, não requer imports adicionais.

### A.3 — Stdlib em `rules/stdlib/color.rs`

Adicionar a `make_color_module()`:

```rust
color_module.insert("saturate",   Value::Func(Func::native("saturate",   native_color_saturate)));
color_module.insert("desaturate", Value::Func(Func::native("desaturate", native_color_desaturate)));
```

Novas funcs:

```rust
pub(crate) fn native_color_saturate(_ctx, args) -> SourceResult<Value> {
    let col:    Color = extract_color_arg(&mut args, "saturate", "col")?;
    let amount: f32   = extract_ratio_arg(&mut args, "saturate", "amount")?;
    args.expect_no_more()?;
    Ok(Value::Color(col.saturate(amount)))
}

pub(crate) fn native_color_desaturate(_ctx, args) -> SourceResult<Value> {
    let col:    Color = extract_color_arg(&mut args, "desaturate", "col")?;
    let amount: f32   = extract_ratio_arg(&mut args, "desaturate", "amount")?;
    args.expect_no_more()?;
    Ok(Value::Color(col.desaturate(amount)))
}
```

`extract_color_arg` e `extract_ratio_arg` já existem desde P476 — reutilizar sem duplicação.

---

## Sub-item B — Constantes de cor nomeadas

### B.1 — Estado atual

`parse_color` (localização a confirmar via sonda) aceita `"red"`, `"green"`, `"blue"`, `"black"`, `"white"` — 5 cores. O vanilla tem 18+ constantes. Este sub-item alarga para as cores mais usadas em documentos técnicos e académicos.

### B.2 — Cores a adicionar (13 novas)

Subset de maior cobertura prática, evitando duplicação ou raridade:

| Nome | RGB (u8) | Justificação |
|------|----------|--------------|
| `"yellow"` | 255, 255, 0 | Extremamente comum em destaque/highlight |
| `"cyan"` | 0, 255, 255 | Par complementar de red |
| `"magenta"` | 255, 0, 255 | Triade CMY |
| `"orange"` | 255, 165, 0 | Aviso/atenção em documentos |
| `"purple"` | 128, 0, 128 | Académico comum |
| `"gray"` / `"grey"` | 128, 128, 128 | Alias duplo (inglês BrE/AmE) |
| `"silver"` | 192, 192, 192 | Cinzento claro |
| `"maroon"` | 128, 0, 0 | Escuro académico |
| `"navy"` | 0, 0, 128 | Azul escuro comum |
| `"olive"` | 128, 128, 0 | CSS Web color standard |
| `"teal"` | 0, 128, 128 | CSS Web color standard |
| `"lime"` | 0, 255, 0 | Alias de green (CSS) — NÃO duplicar se `"green"` já existe como `0,128,0`; verificar sonda |
| `"aqua"` | 0, 255, 255 | Alias de `"cyan"` (CSS) |

**Nota sobre `"lime"` vs `"green"`:** no vanilla Typst, `"green"` = `Color::GREEN` = `#008000` (CSS green, não `(0,255,0)`). Se o cristalino usa `(0,255,0)` para `"green"`, verificar paridade antes de adicionar `"lime"`. A sonda resolve. Prioridade: paridade com vanilla CSS colors.

### B.3 — Implementação em `parse_color`

```rust
// Extensão de parse_color — P477
match s {
    // Existentes (preservados):
    "red"   => Some(Color::rgb(255, 0, 0)),
    "green" => Some(Color::rgb(0, 128, 0)),   // CSS green (verificar com sonda)
    "blue"  => Some(Color::rgb(0, 0, 255)),
    "black" => Some(Color::rgb(0, 0, 0)),
    "white" => Some(Color::rgb(255, 255, 255)),
    // Novas P477:
    "yellow"  => Some(Color::rgb(255, 255, 0)),
    "cyan"    => Some(Color::rgb(0, 255, 255)),
    "magenta" => Some(Color::rgb(255, 0, 255)),
    "orange"  => Some(Color::rgb(255, 165, 0)),
    "purple"  => Some(Color::rgb(128, 0, 128)),
    "gray" | "grey" => Some(Color::rgb(128, 128, 128)),
    "silver"  => Some(Color::rgb(192, 192, 192)),
    "maroon"  => Some(Color::rgb(128, 0, 0)),
    "navy"    => Some(Color::rgb(0, 0, 128)),
    "olive"   => Some(Color::rgb(128, 128, 0)),
    "teal"    => Some(Color::rgb(0, 128, 128)),
    "lime"    => Some(Color::rgb(0, 255, 0)),
    "aqua"    => Some(Color::rgb(0, 255, 255)),
    _ => None,
}
```

**Nota:** `"gray"` e `"grey"` → mesmo valor; `"aqua"` e `"cyan"` → mesmo valor. Aliases explícitos, sem nova lógica.

---

## Tests

### Sub-item A

- **L1:** `Color::rgb(255, 0, 0).saturate(0.1)` — chroma Oklch aumenta; l e h preservados.
- **L1:** `Color::rgb(128, 128, 128).desaturate(0.1)` — cinzento (c=0) com `desaturate` permanece cinzento (c clampado a 0.0).
- **L1:** `saturate(0.0)` → cor inalterada em Oklch.
- **L1:** `desaturate(1.0)` → c = 0.0 (cinzento completo com l e h preservados).
- **L1:** `saturate(-0.1)` — amount negativo: clamp `(c - 0.1).max(0.0)` (mesma lógica que desaturate).
- **L2:** `native_color_saturate(red, 0.2)` → `Value::Color(...)`.
- **L2:** `native_color_desaturate(blue, 0.5)` → c diminuída.
- **L2:** `color.saturate` e `color.desaturate` acessíveis no módulo `color` do scope.

### Sub-item B

- **L1:** `parse_color("yellow")` → `Some(Color::rgb(255, 255, 0))`.
- **L1:** `parse_color("gray")` == `parse_color("grey")` → mesmo `Color`.
- **L1:** `parse_color("aqua")` == `parse_color("cyan")` → mesmo `Color`.
- **L1:** `parse_color("navy")` → `Some(Color::rgb(0, 0, 128))`.
- **L1:** `parse_color("unknown_color")` → `None` (sem regressão).
- **L1:** Todas as 5 cores originais continuam funcionais (regressão).

---

## Spec L0

### Actualizados

- `entities/color.md` — §"Operadores de cor" alargada com `saturate`/`desaturate`; ADR-0083 §"Operadores cor" marcado como **totalmente fechado** (6/6). §"Constantes nomeadas" alargada com 13 novas cores; ADR-0083 §"Constantes nomeadas" actualizado (18/18 paridade CSS Web colors básicas).
- `rules/stdlib/color.md` — `saturate` e `desaturate` adicionados ao módulo `color`.
- ADR-0083 — anotação cumulativa P477: §"Operadores cor" **totalmente revogado** (6/6). §"Constantes nomeadas" revogado parcialmente (cobertura CSS basic colors).

---

## Scope-out explícito

- **Constantes P477 além das 18 CSS basic** — vanilla tem mais nomes (`DARK_GRAY`, `LIGHT_BLUE`, etc.); cobertura CSS basic é suficiente para este passo.
- **`Color::saturate` com chroma máxima definida** — sem clamp superior; cores fora de gamut são responsabilidade do exporter.
- **`saturate`/`desaturate` preservando espaço original** — cristalino converte via Oklch; paridade vanilla.
- **Operadores cor com `space:` arg** (`color.saturate(red, 20%, space: "hsl")`) — scope-out; subset sem espaço explícito.
- **`color.space()` runtime** — ADR-0083 §"ColorSpace runtime" preservado scope-out; não aberto neste passo.
- **`Color::RED`, `Color::BLUE` como constantes estáticas Rust** — vanilla expõe assim; cristalino expõe via `parse_color` string. Divergência declarada ADR-0083; não alterada.

---

## Critério de fecho

- [ ] Sondas: `Color::saturate` (ausente), `parse_color` localização, cores actuais — todos com `file:line`.
- [ ] `Color::saturate(self, amount: f32) -> Self` implementado.
- [ ] `Color::desaturate(self, amount: f32) -> Self` implementado.
- [ ] `native_color_saturate` e `native_color_desaturate` implementados em `stdlib/color.rs`.
- [ ] `make_color_module()` inclui `saturate` e `desaturate`.
- [ ] `parse_color` alargado com 13+ novas cores (incluindo aliases `gray`/`grey`, `aqua`/`cyan`).
- [ ] 8+ testes verdes sub-item A (5 L1 + 3 L2).
- [ ] 6+ testes verdes sub-item B (6 L1).
- [ ] `entities/color.md` e `rules/stdlib/color.md` actualizados.
- [ ] ADR-0083 anotação cumulativa P477 redigida.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **ADR-0083 §"Operadores cor" TOTALMENTE FECHADO** (6/6: lighten/darken/mix/negate P476 + saturate/desaturate P477).
- [ ] **ADR-0083 §"Constantes nomeadas" revogado parcialmente** (CSS basic colors cobertas).

---

## Próximo passo

Com P477, o cluster Color está praticamente completo:

| ADR-0083 scope-out | Estado pós-P477 |
|--------------------|-----------------|
| PDF native CMYK | FECHADO (P270.2) |
| Operadores cor | **TOTALMENTE FECHADO** (P476+P477) |
| ColorSpace runtime | Preservado scope-out |
| Constantes nomeadas | Parcialmente fechado (CSS basic P477) |

Trilhas abertas:

| Trilha | Estado | Próximo |
|--------|--------|---------|
| 4 | ~completa (ColorSpace runtime = scope-out permanente) | — |
| 5 | Épico XL declarado (shaping rustybuzz) | Passo dedicado futuro |
| 6 | 4/5 (LoF/LoT page numbers = scope-out longo prazo) | — |

Opções para P478:
- **Footnotes Fase 2** — se `Content::Footnote` body não é renderizado no rodapé (P295 materializou apenas o marker `[N]`; body diferido). S–M.
- **Tiling sonda** — verificar se `Value::Tiling` está activo ou stub. XS.
- **`color.space()` runtime** — se surgir necessidade concreta. S.
- **Sonda estado geral** — varrer DEBT.md e roteiro para identificar itens pendentes não cobertos pelas trilhas listadas.

---

## Estado pós-P476 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P257/P270 | Color 8 espaços + ColorSpace + CMYK PDF | ✅ FECHADO | 4 |
| P476 | lighten/darken/mix/negate + sonda Trilha 5 | ✅ FECHADO | 4+5 |
| **P477** | saturate/desaturate + constantes nomeadas | 🔄 EM PREPARAÇÃO | 4 |

**Trilha 1: COMPLETA. Trilha 2: COMPLETA. Trilha 3: COMPLETA.**
**Trilha 4: ~completa pós-P477 (ColorSpace runtime = scope-out).**
**Trilha 5: Épico XL declarado.**
**Trilha 6: 4/5. Trilha 7: COMPLETA. Trilha 8: COMPLETA.**
**ADR-0083: 3/4 scope-outs fechados pós-P477.**
**Inventário de débitos: LIMPO.**
