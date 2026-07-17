---

# P476 — Sonda Trilha 5 (rustybuzz) + operadores de cor (`lighten`/`darken`/`mix`)

> **Passo:** 476
> **Data:** 2026-06-27
> **Foco:** (A) Sonda de viabilidade de Trilha 5 — `rustybuzz` já está no `Cargo.toml` (ADR-0019); confirmar estado de shaping e RTL; (B) Operadores de cor — `color.lighten`, `color.darken`, `color.mix`, `color.negate` — scope-out remanescente de ADR-0083.
> **Trilhas:** 5 (sub-item A — sonda) + 4 (sub-item B — extensão Color).
> **Tipo:** Sonda-first + Materialização.
> **Tamanho:** XS (sonda) + S–M (operadores cor, ~30 min).
> **ADR-0117 Cláusula 4:** Sub-item A: ADR-0019 confirma `rustybuzz` em `03_infra/Cargo.toml` desde Passo 8. Verificar se shaping está activo ou é stub. Sub-item B: `entities/color.rs` (P257); `rules/stdlib/shapes.rs` ou módulo de cor; ADR-0083 §"Operadores cor" scope-out.

---

## Contexto

**Sub-item A — Sonda Trilha 5:**
O roteiro marca Trilha 5 como "Bloqueada — Sonda de viabilidade rustybuzz". ADR-0019 revela que `rustybuzz` **já está em `03_infra/Cargo.toml` desde o Passo 8** — não é uma dependência nova a adicionar. A pergunta real é: está activamente usada para shaping, ou é uma dependência declarada mas com shaping como stub?

**Sub-item B — Operadores de cor:**
ADR-0083 declara como scope-out formal: `lighten`/`darken`/`mix`/`saturate`/`desaturate`/`negate`. Estes são os operadores mais usados em documentos Typst reais (ex: `color.lighten(red, 20%)`, `color.mix(red, blue, 50%)`). Os outros scope-outs de ADR-0083 estão fechados (DeviceCMYK PDF P270.2, ColorSpace runtime P270). Este é o último item de impacto prático da Trilha 4.

---

## ADR-0108 — Medir antes de decidir

### Sub-item A — Sonda Trilha 5

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `rustybuzz` em `03_infra/Cargo.toml`? | `03_infra/Cargo.toml` (confirmado ADR-0019) | ✅ |
| `rustybuzz` usado activamente em código de produção? | `grep -r "rustybuzz" 03_infra/src/` | 🟡 |
| `03_infra/src/fonts.rs` usa `rustybuzz::Face` para shaping? | `fonts.rs` ou `shaping.rs` | 🟡 |
| Stub de shaping (sem glifos posicionados)? | arm de glif no exporter PDF | 🟡 |
| `text.dir` / RTL parsing/layout activo? | `rules/eval/` ou `engine/layout/` | 🟡 |
| `bidi` crate no `Cargo.toml`? | `03_infra/Cargo.toml` | 🟡 |
| Testes de shaping reais existem? | `cargo test` filtro `shaping` | 🟡 |

**Conclusão esperada:** rustybuzz está no Cargo.toml mas shaping é provável stub (crate usada para validação de fonte em P8, não para shaping real). RTL/bidi: provável scope-out declarado. A sonda confirma e determina a magnitude do trabalho restante de Trilha 5.

### Sub-item B — Operadores de cor

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `Color::lighten` método existe? | `entities/color.rs` | 🟡 |
| `color.lighten` no stdlib? | `rules/stdlib/shapes.rs` ou `color.rs` | 🟡 |
| `color.mix` no stdlib? | idem | 🟡 |
| `native_color_*` funcs em módulo de cor? | `rules/stdlib/` | 🟡 |
| `Value::Color` existe? | `entities/value.rs` | ✅ (P257) |
| `Color::to_srgb()` disponível para implementar mix? | `entities/color.rs` | ✅ (P257) |
| Hue-wrap `interpolate_hue_shorter` existe? | `entities/gradient.rs` ou `color.rs` | ✅ (P270) |

---

## Sub-item A — Sonda Trilha 5 (resultado documentado; sem código obrigatório)

### A.1 — Se rustybuzz é stub (cenário mais provável)

Resultado da sonda: `rustybuzz` está no `Cargo.toml` mas é usado apenas para validação estrutural de fontes (verificar se um ficheiro `.ttf` é válido), não para shaping real. O shaping actual é um stub que coloca glifos sequencialmente sem kern, ligatures, ou posicionamento OpenType.

**Implicação para Trilha 5:** materializar shaping real com `rustybuzz` é um épico (XL, ~8-12h), não um passo único. Requer:
1. Pipeline `rustybuzz::UnicodeBuffer` → `rustybuzz::GlyphBuffer`.
2. Extração de `GlyphPosition` (advance + offset) para cada glif.
3. Substituição do stub sequencial no Layouter.
4. Testes de paridade: kern pairs, ligatures básicas.

**Decisão para este passo:** documentar o estado real em ADR anotação + declarar Trilha 5 como épico separado (não passo único de Trilha 8). Sem código de shaping neste passo.

### A.2 — Se rustybuzz já está activo (cenário improvável mas possível)

Se a sonda revelar shaping activo, documentar o estado e identificar lacunas (RTL, bidi, ligatures em scripts específicos). Trilha 5 pode estar parcialmente fechada sem o saber.

### A.3 — ADR anotação

Adicionar anotação ao DEBT.md ou ao roteiro: "Trilha 5 shaping rustybuzz — estado real confirmado em P476; épico dedicado (XL) se stub confirmado; ver sonda P476."

---

## Sub-item B — Operadores de cor

### B.1 — Contexto vanilla

O vanilla expõe (em `color.rs:1400+`):

```typst
color.lighten(col, amount)   // aumenta luminância por amount (%)
color.darken(col, amount)    // diminui luminância por amount (%)
color.mix(..cols, space:?)   // mistura de cores com pesos
color.negate(col, space:?)   // complementar no espaço
color.saturate(col, amount)  // aumenta saturação
color.desaturate(col, amount)// diminui saturação
```

### B.2 — Subset P476

Implementar o subset de maior impacto prático:

| Operador | Vanilla | Subset P476 | Notas |
|----------|---------|-------------|-------|
| `lighten` | ✓ | **✓** | Via Oklch: aumenta `l` |
| `darken` | ✓ | **✓** | Via Oklch: diminui `l` |
| `mix` | ✓ | **✓** | Interpolação linear Oklab (2 cores, peso 50% default) |
| `negate` | ✓ | **✓** | `(1-r, 1-g, 1-b)` em sRGB |
| `saturate` | ✓ | ⏸ scope-out | Requer Oklch chroma; pode ser P477 |
| `desaturate` | ✓ | ⏸ scope-out | Idem |

**Decisão:** 4 operadores neste passo (`lighten`, `darken`, `mix`, `negate`). `saturate`/`desaturate` em passo futuro — mesma infraestrutura, menor prioridade.

### B.3 — Implementação em `entities/color.rs`

```rust
impl Color {
    /// Aumenta luminância por `amount` [0.0, 1.0] via Oklch.
    /// Clamp: l ∈ [0.0, 1.0].
    pub fn lighten(self, amount: f32) -> Self {
        let (l, c, h, alpha) = self.to_oklch_components();
        Color::oklch((l + amount).clamp(0.0, 1.0), c, h, alpha)
    }

    /// Diminui luminância por `amount` [0.0, 1.0] via Oklch.
    pub fn darken(self, amount: f32) -> Self {
        let (l, c, h, alpha) = self.to_oklch_components();
        Color::oklch((l - amount).clamp(0.0, 1.0), c, h, alpha)
    }

    /// Interpolação linear entre `self` e `other` em Oklab.
    /// `weight` [0.0, 1.0]: 0.0 = self; 1.0 = other.
    pub fn mix(self, other: Self, weight: f32) -> Self {
        let (l0, a0, b0, alpha0) = self.to_oklab_components();
        let (l1, a1, b1, alpha1) = other.to_oklab_components();
        let t = weight.clamp(0.0, 1.0);
        Color::oklab(
            l0 + (l1 - l0) * t,
            a0 + (a1 - a0) * t,
            b0 + (b1 - b0) * t,
            alpha0 + (alpha1 - alpha0) * t,
        )
    }

    /// Negação: complementar em sRGB (1 - componente).
    /// Alpha preservado.
    pub fn negate(self) -> Self {
        let (r, g, b, a) = self.to_rgba_f32();
        Color::srgb_f32(1.0 - r, 1.0 - g, 1.0 - b, a)
    }
}
```

**Nota:** `to_oklch_components()` e `to_oklab_components()` já existem em `gradient.rs` como helpers privados (P270). Verificar se podem ser movidos/re-exportados para `color.rs`, ou duplicar os helpers minimais.

### B.4 — Stdlib: módulo `color` no scope

O vanilla expõe os operadores como métodos de módulo: `color.lighten(red, 20%)`.

**Ficheiro:** `rules/stdlib/color.rs` (novo, se não existir) ou `rules/stdlib/shapes.rs`

```rust
pub fn native_color_lighten(_ctx, args) -> Result<Value, EvalError> {
    let col:    Color = args.expect_positional_color("col")?;
    let amount: f32   = args.expect_positional_ratio("amount")?;
    args.expect_no_more()?;
    Ok(Value::Color(col.lighten(amount)))
}

pub fn native_color_darken(_ctx, args) -> Result<Value, EvalError> { /* análogo */ }

pub fn native_color_mix(_ctx, args) -> Result<Value, EvalError> {
    // 2 cores posicionais + weight: opcional (default 0.5)
    let col1:   Color = args.expect_positional_color("col1")?;
    let col2:   Color = args.expect_positional_color("col2")?;
    let weight: f32   = args.named_or("weight", 0.5)?;
    args.expect_no_more()?;
    Ok(Value::Color(col1.mix(col2, weight)))
}

pub fn native_color_negate(_ctx, args) -> Result<Value, EvalError> {
    let col: Color = args.expect_positional_color("col")?;
    args.expect_no_more()?;
    Ok(Value::Color(col.negate()))
}
```

**Registo no scope** como módulo `color` com campos `lighten`, `darken`, `mix`, `negate`:

```rust
// Em eval/mod.rs — análogo ao módulo `sym` (P471):
let mut color_module = Dict::new();
color_module.insert("lighten", Value::Func(Func::native("lighten", native_color_lighten)));
// ...
scope.define("color", Value::Dict(color_module));
```

### B.5 — Helper `expect_positional_ratio`

`amount` em `lighten`/`darken` aceita `Value::Float` ou `Value::Relative(Rel { rel, abs: 0 })` como percentagem. Verificar se existe helper ou adicionar:

```rust
fn expect_positional_ratio(args, name) -> Result<f32, EvalError> {
    match args.expect_positional(name)? {
        Value::Float(f) => Ok(f as f32),
        Value::Int(i)   => Ok(i as f32 / 100.0),
        Value::Relative(r) if r.abs.is_zero() => Ok(r.rel as f32),
        _ => Err(EvalError::type_mismatch("float or percent", name))
    }
}
```

---

## Tests

### Sub-item A

- Não há testes de código (passo de sonda). Resultado documentado em relatório.

### Sub-item B

- **L1:** `Color::rgb(255,0,0).lighten(0.2)` → cor com `l` aumentado em Oklch; não é vermelho puro.
- **L1:** `Color::rgb(255,0,0).darken(0.2)` → cor com `l` diminuído.
- **L1:** `Color::mix(Color::rgb(255,0,0), Color::rgb(0,0,255), 0.5)` → cor intermediária em Oklab.
- **L1:** `Color::rgb(255,0,0).negate()` → `Color::srgb_f32(0.0, 1.0, 1.0, 1.0)` (ciano).
- **L1:** `lighten` com `amount = 0.0` → cor inalterada; `amount = 1.0` → branco Oklch.
- **L2:** `native_color_lighten(red, 20%)` → `Value::Color(...)`.
- **L2:** `native_color_mix(red, blue, weight: 0.25)` → cor mais próxima de red.
- **L2:** `native_color_mix(red, blue)` sem `weight:` → peso 0.5 por defeito.
- **L2:** `native_color_negate(red)` → ciano.
- **L2:** `color.lighten` acessível via module `color` no scope eval.

---

## Spec L0

### Sub-item A

- DEBT.md ou `roteiro-conclusao-typst-cristalino-atualizado.md` — nota sobre estado de Trilha 5 pós-sonda P476.

### Sub-item B

- `entities/color.md` — métodos `lighten`, `darken`, `mix`, `negate` adicionados; `scope-outs` de ADR-0083 actualizados (4 operadores fechados; `saturate`/`desaturate` preservados).
- `rules/stdlib/color.md` — **NOVO** — módulo `color` no scope; 4 funcs nativas.
- ADR-0083 — anotação cumulativa P476: `lighten`/`darken`/`mix`/`negate` revogados do scope-out §"Operadores cor". Restam: `saturate`/`desaturate`.

---

## Scope-out explícito

- **Shaping rustybuzz real** — Trilha 5 épico; sem código neste passo.
- **RTL / bidi** — dependente de shaping; Trilha 5 épico.
- **Smallcaps via OpenType** — Trilha 5.
- **`saturate`/`desaturate`** — Oklch chroma; futuro P477.
- **`color.mix` com N cores** (`color.mix(red, green, blue)`) — vanilla aceita array variádico; cristalino aceita apenas 2 cores + `weight:` neste passo.
- **`color.mix` com `space:` arg** — pode ser P477.
- **`lighten`/`darken` preservando espaço de cor da entrada** — cristalino converte sempre via Oklch (paridade vanilla default); preservar espaço original é scope-out.

---

## Critério de fecho

- [ ] Sondas sub-item A executadas: `rustybuzz` activo ou stub; RTL activo ou ausente — todos com `file:line` e conclusão sobre Trilha 5.
- [ ] Estado de Trilha 5 documentado (épico separado se stub confirmado).
- [ ] Sondas sub-item B: `Color::lighten` (ausente), `native_color_lighten` (ausente) — todos com `file:line`.
- [ ] `Color::lighten`, `darken`, `mix`, `negate` implementados em `entities/color.rs`.
- [ ] `to_oklch_components` e `to_oklab_components` acessíveis em `color.rs` (movidos ou duplicados de `gradient.rs`).
- [ ] `native_color_lighten/darken/mix/negate` implementados.
- [ ] Módulo `color` registado no scope com 4 entradas.
- [ ] 10+ testes verdes (5 L1 + 5 L2).
- [ ] `entities/color.md` actualizado (4 métodos novos + scope-outs actualizados).
- [ ] `rules/stdlib/color.md` criado.
- [ ] ADR-0083 anotação cumulativa P476.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **ADR-0083 §"Operadores cor" parcialmente revogado** (4/6 operadores; restam saturate/desaturate).

---

## Próximo passo

Com P476:

| Trilha | Estado pós-P476 |
|--------|-----------------|
| 3 | COMPLETA |
| 4 | ~completa (restam saturate/desaturate como scope-out XS) |
| 5 | Épico declarado (XL) ou parcialmente activo (a confirmar) |
| 6 | 4/5 (LoF/LoT page numbers = scope-out longo prazo) |
| 7 | COMPLETA |
| 8 | COMPLETA |

Opções para P477:
- **`saturate`/`desaturate`** (Trilha 4 refino, S, ~15 min) — fecha ADR-0083 §"Operadores cor" totalmente.
- **Constantes de cor nomeadas** (`Color::RED`, `Color::NAVY`, etc.) — refino incremental ADR-0083 §"Constantes nomeadas", S.
- **Footnotes Fase 2** — se P295 deixou body não-renderizado no rodapé.
- **Sonda Tiling** — `Value::Tiling` activo ou stub.

---

## Estado pós-P475 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P468–P470 | Trilha 6 + 8 parcial | ✅ FECHADO | 6+8 |
| P471–P473 | Symbol, ibid., op.cit., regex | ✅ FECHADO | 8+6+3 |
| P474 | Fecho Trilhas 3+8 (sonda) | ✅ FECHADO | 3+8 |
| P475 | Inset/outset dict + Rel em extract_sides | ✅ FECHADO | 8 |
| **P476** | Sonda Trilha 5 + operadores cor | 🔄 EM PREPARAÇÃO | 5+4 |

**Trilha 1: COMPLETA. Trilha 2: COMPLETA. Trilha 3: COMPLETA.**
**Trilha 4: ~completa (saturate/desaturate pendentes).**
**Trilha 5: Bloqueada pendente sonda P476.**
**Trilha 6: 4/5. Trilha 7: COMPLETA. Trilha 8: COMPLETA.**
**Inventário de débitos: LIMPO.**
