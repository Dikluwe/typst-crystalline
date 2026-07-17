# Diagnóstico — Fase A do Passo 285 (`P-line-color-rg-emit`)

**Data**: 2026-05-18
**Spec mãe**: `00_nucleo/materialization/typst-passo-285.md`
**Origem dupla**: P282 §1.5 (pendência `RG` em Line) + P284 §5.4
(`stroke` parseado mas inerte em decorações).

---

## A.1 — Inventário exaustivo de `FrameItem::Line`

`grep -rn "FrameItem::Line" 01_core/src/ 03_infra/src/` produz 18+ hits.
Separação em **3 buckets** (produtores / consumers de emit / match patterns):

### A.1.1 — Produtores (sítios que constroem `FrameItem::Line { ... }`)

| Sítio | Origem | Cor actual | Comportamento pós-P285 |
|---|---|---|---|
| `01_core/src/engine/math/layout/frac.rs:65` | Linha de fracção (P38) | hardcoded preto (sem campo) | `color: None` → preserva preto bit-exact |
| `01_core/src/engine/math/layout/root.rs:63` | Overline da raíz (sqrt) | idem | `color: None` |
| `01_core/src/engine/layout/mod.rs:2008` | **Consumer P284 decorações** (Underline/Strike/Overline) | `stroke: Option<Color>` parseado mas IGNORADO | `color: stroke.or(self.style.fill)` (A.3 herança) |
| `01_core/src/engine/math/layout/mod.rs:92` | Reflector de Line dentro de math layout (translação) | preserva | `color: *color` (pass-through) |
| `01_core/src/engine/layout/equation.rs:79` | Reflector dentro de equação | preserva | `color: *color` |
| `01_core/src/engine/layout/cursor.rs:251` | Reflector (cursor adjustment) | preserva | `color: *color` |
| `01_core/src/engine/layout/helpers.rs:37` | Reflector (helpers genéricos) | preserva | `color: *color` |
| `01_core/src/engine/layout/slicing.rs:83` | Reflector (page slicing) | preserva | `color: *color` |

### A.1.2 — Consumers de emit (sítios que **lêem** `FrameItem::Line`)

| Sítio | Função | Comportamento pós-P285 |
|---|---|---|
| `03_infra/src/export.rs:2256-2264` | `build_page_stream` top-level | Insere condicionalmente `{r} {g} {b} RG ` antes de `{w} w` quando `color = Some(c)`; quando `None` preserva string idêntica (backward-compat bit-exact) |

### A.1.3 — Match patterns destrutivos (não-produtores)

| Sítio | Uso | Acção |
|---|---|---|
| `01_core/src/entities/layout_types.rs:427` | `Frame::plain_text` filter | adicionar `_` no pattern para evitar warning |
| `01_core/src/engine/layout/cursor.rs:314` | extract `start.y.0` | `FrameItem::Line { start, .. }` continua válido (`..`) |
| `01_core/src/engine/layout/helpers.rs:21` | extract `(start.x, start.y)` | `..` continua válido |
| `01_core/src/engine/layout/slicing.rs:60` | extract `start.y.0` | idem |
| `03_infra/src/pipeline.rs:128, 183` | filtering com `matches!` `..` | idem |
| `01_core/src/engine/math/layout/tests.rs` (várias) | test patterns | `..` continua válido |
| `01_core/src/engine/layout/tests.rs:9818` | test helper destructuring `{ start, end, thickness }` | **ajustar** para `{ start, end, thickness, color: _ }` ou usar `..` |
| `01_core/src/engine/math/layout/tests.rs:471` | constroi Line directo | adicionar `color: None` |

**Conclusão A.1**: 8 produtores + 1 consumer emit + ~8 match patterns. Nenhum
gap empírico fora do esperado (limite 10 sítios per §7 risco secundário não
atingido — não é necessário sub-passo P285.1).

---

## A.2 — Tipo do campo: decisão

**Decidido**: opção **(a)** `color: Option<Color>`.

### Tabela comparativa (resolvida)

| Opção | Veredicto |
|---|---|
| (a) `Option<Color>` | ✅ Escolhida — `None` mapeia trivialmente em backward-compat (todos os 5 reflectors + 2 math producers passam `color: None`) |
| (b) `Color` directo | ❌ Forçaria refactor de 7+ call-sites para passar `Color::rgb(0,0,0)`; ruído sem benefício |
| (c) `paint: Paint` (gradient/tiling) | ❌ Over-engineering; vanilla não suporta gradient/tiling em linha |

### Sintaxe final

```rust
FrameItem::Line {
    start:     Point,
    end:       Point,
    thickness: f64,
    color:     Option<Color>,  // P285 — None → emit preto default (bit-exact pré-P285)
}
```

---

## A.3 — Política de herança para decorações de P284

**Decidido**: opção **(β) com cascata graceful (γ)**.

### Inspecção empírica

O `Layouter` em `layout/mod.rs:92` expõe `pub(super) style: TextStyle`;
`TextStyle.fill: Option<Color>` (`layout_types.rs:128`). No ponto de emit
da decoração (`layout/mod.rs:2008`), `self.style.fill` está disponível
sem custo.

### Regra resolvida

```rust
let color = stroke                        // 1. utilizador especificou explicitamente?
    .or_else(|| self.style.fill)          // 2. herda do texto corrente (β)
    /* 3. fallback implícito: None → emit preto default (γ) */;
```

- `stroke: Some(c)` → `color: Some(c)` (utilizador wins).
- `stroke: None` + `style.fill: Some(text_c)` → `color: Some(text_c)` —
  paridade vanilla (decoração herda cor do texto).
- `stroke: None` + `style.fill: None` → `color: None` — emit preto default
  bit-exact. Aceite como divergência aproximada per ADR-0054 graded.

### Justificações

| Sinal | Decisão |
|---|---|
| Vanilla `UnderlineElem` doc: `If set to {auto}, takes on the text's color` | (β) é paridade real |
| `self.style.fill` disponível sem refactor | (β) trivial de implementar |
| Default `None` para text sem fill explícito | (γ) preserva bit-exact em testes pré-P285 |

---

## §Métricas do impacto

| Métrica | Antes | Pós-P285 |
|---|---:|---:|
| `FrameItem::Line` campos | 3 (`start`, `end`, `thickness`) | **4** (+`color`) |
| Sítios produtores | 8 (todos sem cor) | 8 (1 herda da style; 7 com `None`/pass-through) |
| Hash L0 `export.rs` | `bc7b8b95` (preservado desde P281) | **muda** (esperado per spec §1) |
| Hash L0 `layout_types.md` | actual | **muda** |
| Pendências resolvidas | — | **2**: P282 §1.5 + P284 §5.4 |
| Bit-exact backward-compat | n/a | **Validado** — `None` preserva string exacta |

---

## §Risco residual mitigado

- **Risco principal** (§7 spec): bit-exactness em call-sites sem color
  explícito. **Mitigado** por inserção condicional `if let Some(c) = color`
  que preserva a string `"q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n"`
  literalmente quando `color = None`. Validável por regression test
  comparando bytes pré/pós em `frac/sqrt/line`.
- **Risco secundário**: >10 sítios. Inventário A.1 conta 8 produtores
  → dentro do limite. Sub-passo P285.1 não necessário.
- **Risco terciário**: `text_color` indisponível no Layouter. Refutado
  empiricamente — `self.style.fill` está disponível no ponto exacto do
  consumer P284.

---

## §Fecho da Fase A

Inventário completo (8 produtores + 1 consumer + ~8 match patterns) +
decisão tipo (`Option<Color>`) + decisão herança (`stroke.or(style.fill)`)
registadas. Material suficiente para materialização cirúrgica sem
ambiguidades. Procede-se a §3 do passo.
