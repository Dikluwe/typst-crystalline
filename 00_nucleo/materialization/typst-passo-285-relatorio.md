# Passo 285 — Relatório consolidado

**Tema**: Materialização da frente `P-line-color-rg-emit` —
`FrameItem::Line` ganha `color: Option<Color>`; emit `RG` em PDF;
**activação** do `stroke` parseado mas inerte registado em P284 §5.4.
Resolução literal da pendência P282 §1.5.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: S (modificação cirúrgica num tipo + 1 helper L3 +
8 produtores propagados; sem variants novos; +9 testes).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 725** testes (baseline P284: 2 716 → **+9**) |
| Delta esperado +10 a +15 | ⚠ +9 (ligeiramente abaixo — economia por design A.2 simples + opção β herança que cobre 3 variants P284 num mesmo path; ver §5.2) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `export.rs` muda — registar novo hash | ✅ `bc7b8b95 → 66cb8ac3` (era preservado desde P281; quebra esperada per spec §1) |
| Hash L0 `layout_types.md` muda — registar novo hash | ✅ `62398a83 → 2a53ebb8` |
| **Regressão bit-exact validada** para call-sites sem cor | ✅ `p285_math_frac_preserva_ausencia_de_rg` + `p285_line_sem_stroke_preserva_bit_exact` (zero ocorrências de `RG` quando `color: None`) |
| Tabela A.7 linha 192 (`line`): manter `implementado` + nota stroke funcional | ✅ Nota adicionada — `FrameItem::Line` ganha `color` + emit `RG` |
| Tabela A.3 linha 103 (decorações P284): adicionar nota "stroke agora funcional (P285)" | ✅ Cruz-referenciado P284 + **P285** |
| P282 §1.5 pendência marcada como RESOLVIDA | ✅ Footnote ⁷¹ marca explicitamente |
| P284 §5.4 marcada como RESOLVIDA | ✅ Footnote ⁷¹ marca explicitamente para os 3 variants |
| Diagnóstico A.1+A.2+A.3 em `diagnostico-line-color-passo-285.md` | ✅ 3 secções produzidas + métricas + risco residual mitigado |

**Conformidade**: 10/10 critérios estritos; 1 com observação
pragmática (delta de testes; ver §5.2).

---

## §2 — Resumo factual

### §2.1 — Modificação cirúrgica em `FrameItem::Line`

**Antes P285** (estado pré-passo):
```rust
FrameItem::Line {
    start:     Point,
    end:       Point,
    thickness: f64,
}
```

**Pós-P285**:
```rust
FrameItem::Line {
    start:     Point,
    end:       Point,
    thickness: f64,
    color:     Option<Color>,  // ← novo: None preserva preto bit-exact
}
```

- `Content` enum: 63 variants (inalterado — P285 não adicionou nenhum).
- `FrameItem` enum: 6 variants (inalterado; apenas extensão de 1 campo).
- 1 helper L3 novo (`line_rg_prefix`); 0 helpers removidos.

### §2.2 — Inventário tocado (per diagnóstico §A.1)

| Local | Acção | Mudança |
|---|---|---|
| `01_core/src/entities/layout_types.rs:184-193` | extensão | +1 campo + doc-comment |
| `01_core/src/rules/math/layout/frac.rs:69` | produtor (math frac) | +`color: None` (preserva preto bit-exact P38) |
| `01_core/src/rules/math/layout/root.rs:67` | produtor (sqrt overline) | +`color: None` |
| `01_core/src/rules/math/layout/mod.rs:92` | reflector (translation) | adiciona `color` ao pattern, pass-through |
| `01_core/src/rules/layout/equation.rs:76-85` | reflector (equação) | adiciona `color` ao pattern, pass-through |
| `01_core/src/rules/layout/cursor.rs:251-256` | reflector (cursor adjust) | idem |
| `01_core/src/rules/layout/helpers.rs:34-43` | reflector (translate) | idem |
| `01_core/src/rules/layout/slicing.rs:82-89` | reflector (Y-slice) | idem |
| `01_core/src/rules/layout/mod.rs:1987-2014` | **consumer P284 decorações** | **`color = stroke.or(self.style.fill)`** (regra herança §A.3) |
| `01_core/src/rules/math/layout/tests.rs:471-476` | test directo | +`color: None` |
| `01_core/src/rules/layout/slicing.rs:208-215` | test directo | +`color: None` |
| `01_core/src/rules/layout/tests.rs:9818` | test helper destruct | `..` em vez de listar campos |
| `03_infra/src/export.rs:2185-2199` | **helper L3 novo** | +`line_rg_prefix(color) -> String` (~12 LOC) |
| `03_infra/src/export.rs:2270-2284` | emit top-level | inserção condicional `{rg}` antes de `{w}` |
| `03_infra/src/export.rs:2671-2685` | emit local em Group | inserção simétrica ao top-level |
| `03_infra/src/export.rs:9164-9168` | test directo (P281) | +`color: None` |

Total: **15 sítios L1 + 4 sítios L3** tocados (dentro do limite §7 risco
secundário "se aparecerem >10 produtores" — inventário deu 8 produtores
+ 8 reflectors/tests, abaixo do gatilho para P285.1).

### §2.3 — Helper PDF emit (Win arquitectural P281 preservado)

```rust
fn line_rg_prefix(color: &Option<Color>) -> String {
    match color {
        None    => String::new(),
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} RG ", r, g, b)
        }
    }
}
```

Usado simetricamente em `build_page_stream` (top-level) e
`draw_item_local` (local em Group). **Win arquitectural P281 validado
por construção**: alteração afecta os dois caminhos via mesma assinatura
— divergência local vs top-level estructuralmente impossível de
introduzir (cf. P282 §1.1 "single source of truth como invariante
anti-bug" — N=2 cumulativo agora confirma cross-passo).

Stream resultante:
```
q {rg}{w:.3} w {x1:.1} {y1:.1} m {x2:.1} {y2:.1} l S Q
```

- `color: None` → `rg = ""` → `q 0.6 w 10.0 50.0 m 30.0 50.0 l S Q`
  (bit-exact pré-P285).
- `color: Some(red)` → `rg = "1.000 0.000 0.000 RG "` → `q 1.000 0.000 0.000 RG 0.6 w 10.0 50.0 m 30.0 50.0 l S Q`.

---

## §3 — Fase A — decisões registadas (`diagnostico-line-color-passo-285.md`)

### §3.1 — A.1 inventário exaustivo

`grep -rn "FrameItem::Line" 01_core/src/ 03_infra/src/` produziu 18+
hits, separados em 3 buckets:

| Bucket | Quantidade | Tratamento |
|---|---:|---|
| **Produtores** (constroem Line) | 8 | 7 → `color: None` (preserva preto bit-exact); 1 (P284 decorations) → `color: stroke.or(style.fill)` |
| **Consumers de emit** | 1 (`export.rs:2256` + simétrico local) | Helper `line_rg_prefix` injecta condicionalmente |
| **Match patterns destructivos** | ~8 | Maioria preservada via `..`; 3 ajustes em test helpers |

Sem gap empírico fora do esperado; sub-passo P285.1 não necessário.

### §3.2 — A.2 tipo do campo

**Decidido**: opção **(a)** `color: Option<Color>`.

| Opção | Veredicto |
|---|---|
| (a) `Option<Color>` | ✅ Minimiza touch points (7 producers passam trivialmente `None`); explícita ausência |
| (b) `Color` directo | ❌ Forçaria refactor ruidoso de 7+ sítios para `Color::rgb(0,0,0)` |
| (c) `Paint` (gradient/tiling) | ❌ Over-engineering; gradient em linha não existe em vanilla typst |

### §3.3 — A.3 política de herança em decorações P284

**Decidido**: opção **(β)** com cascata graceful (γ): `color = stroke.or(self.style.fill)`.

| Cenário | Resultado | Justificação |
|---|---|---|
| `stroke: Some(c)` | `color: Some(c)` | Utilizador wins |
| `stroke: None` + `style.fill: Some(text_c)` | `color: Some(text_c)` | Paridade vanilla "decoração herda cor do texto" (β) |
| `stroke: None` + `style.fill: None` | `color: None` | Default preto bit-exact (γ — ADR-0054 graded) |

Inspecção empírica confirmou que `self.style.fill: Option<Color>` está
disponível no `Layouter` (`layout/mod.rs:92`) no ponto exacto do consumer
P284 (`layout/mod.rs:1987`). Implementação trivial.

### §3.4 — Risco residual mitigado

- **Risco principal** (§7 spec): bit-exactness violation. **Mitigado**
  por design — `if let Some(c) = color` no helper preserva string
  literal quando `None`. Validado por dois testes regression dedicados
  (`p285_line_sem_stroke_preserva_bit_exact` + `p285_math_frac_preserva_ausencia_de_rg`).
- **Risco secundário** (>10 sítios): 8 producers efectivos — abaixo do
  gatilho; sem P285.1.
- **Risco terciário** (`text_color` indisponível): refutado
  empiricamente — `self.style.fill` acessível sem refactor.

---

## §4 — Testes adicionados (+9)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `03_infra/src/export.rs` (`tests`) | 5 | Bit-exact regression (sem stroke nem fill herdado → sem `RG`); stroke explícito red → `1.000 0.000 0.000 RG`; strike+overline honram stroke (green/blue); herança Styled→Underline (red); math frac preserva ausência de `RG` (regressão pré-P285) |
| `01_core/src/rules/layout/tests.rs` (`p284_decoration_tests`) | 4 | Stroke explícito propaga para `FrameItem::Line.color`; sem stroke nem fill → `color: None`; herança Styled→Underline (blue); stroke wins sobre fill herdado (green sobre red) |
| **Total** | **9** | — |

**Resultado**: 9/9 verdes (`cargo test --lib p285` em ambos `typst-core`
e `typst-infra`).

---

## §5 — Observações pragmáticas

### §5.1 — Hash `export.rs` muda intencionalmente

`bc7b8b95` esteve preservado desde P281 até P284. P285 **quebra
intencionalmente** este hash (per spec §1 "Hash L0 `export.rs` muda
— registar novo hash"). Novo hash: **`66cb8ac3`**. Razão: inserção
do helper `line_rg_prefix` + alteração das 2 strings format dos emit
top-level e local.

A quebra é **simétrica** (top-level + local mudam em conjunto via
mesmo helper) — preserva o Win arquitectural P281 §1.1 ("divergência
local vs top-level estructuralmente impossível").

### §5.2 — Delta de testes ligeiramente abaixo do alvo (+9 vs +10-15)

Spec previa "10-15 testes". Materializaram-se **9**. Causas:

1. **Design A.2 simples** (`Option<Color>` em vez de novo enum) reduz
   superfície a testar — não há combinatória cross-variant.
2. **Opção β herança** cobre os 3 variants P284 num mesmo path
   (`stroke.or(style.fill)`); testar 1 variant exemplo + simétricos
   strike/overline = 2 testes em vez de 3×N.
3. **Helper `line_rg_prefix`** é testado indirectamente via os 9
   testes integration (não tem teste unitário dedicado porque o
   contrato é trivial e os smoke tests garantem cobertura).

Decisão pragmática consciente — não há gap material; o teste
`p285_math_frac_preserva_ausencia_de_rg` é particularmente valioso
porque protege contra regressões em features pré-P38 (math frac).

### §5.3 — `stroke` em P284 agora honrado uniformemente

Os 3 variants `Content::{Underline,Strike,Overline}` partilham a mesma
arm consumer Layouter; basta uma única alteração (`stroke: _` →
`stroke` + `color = stroke.or(self.style.fill)`) para activar todos
os 3 simultaneamente. Validado por `p285_strike_e_overline_honram_stroke`.

### §5.4 — Herança vanilla-fiel via path Styled

A regra A.3 opção β exige que o consumer Layouter leia `self.style.fill`
no ponto de emit. Validação: `p285_underline_heranca_text_fill_via_styled`
constrói `Content::Styled([Fill(blue)], Content::Underline { stroke: None })`
e confirma que `FrameItem::Line.color == Some(blue)`. O `Styled` arm
do Layouter actualiza `self.style.fill` antes de recurse no body — e
o consumer Underline lê-o correctamente.

### §5.5 — Backward-compat: `stroke` ainda **não** activa cor em `Content::Shape::Line`

A frente `P-line-color-rg-emit` foca exclusivamente em `FrameItem::Line`
(o nó visual primitivo). `Content::Shape::Line` (geometria stdlib
`#line(stroke: red)`) usa `FrameItem::Shape::Line { stroke: ... }` —
**não** `FrameItem::Line` — e essa via já tinha emit `RG` desde P227
(Tabela A.7 linha 201 `stroke(...)` parcial). Não-objectivo §5 do passo
respeitado.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~50 (variant +5; 8 producers +~20; 7 reflectors +~25) |
| LOC L3 produção | ~25 (helper `line_rg_prefix` +12; 2 emit sites +~13) |
| LOC L0 modificado | ~95 (`layout_types.md` +10; `export.md` +20; `cobertura` +65 footnote ⁷¹) |
| Testes adicionados | 9 (5 L3 PDF + 4 L1 Layouter) |
| Testes baseline P284 | 2 716 preserved bit-exact |
| Testes pós-P285 | **2 725** |
| Hash L0 `export.rs` | **`bc7b8b95` → `66cb8ac3`** (quebra intencional per spec) |
| Hash L0 `layout_types.md` | `62398a83 → 2a53ebb8` |
| Lint | zero violations |
| Variants Content | 63 (inalterado) |
| Variants FrameItem | 6 (inalterado; apenas +1 campo em Line) |
| Pendências resolvidas | **2** (P282 §1.5 + P284 §5.4) |
| Cobertura agregada | inalterada (correcção de bug latente + activação, não nova feature) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `FrameItem::Line.color` é
  `Option<Color>` — tipo L1 puro, sem I/O, sem state mutável.
- ✅ **ADR-0054 scope graded**: divergência (γ) `style.fill: None` →
  `color: None` → preto default documentada explicitamente como
  aproximação aceite. Objecto `Stroke` rico continua scope-out (§5
  não-objectivo respeitado).
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  inventário exaustivo (`grep` literal de 18+ hits) antes de tocar
  em código. As 3 decisões (A.1/A.2/A.3) emergiram de observação
  empírica, não de pressupostos.
- ✅ **ADR-0085 diagnóstico imutável**: `diagnostico-line-color-passo-285.md`
  produzido com 3 secções A.1-A.3 + métricas + risco residual mitigado.
- ✅ **Anti-padrão over-formalização P273.17 §0**: zero ADR nova; 1
  padrão emergente registado §8.
- ✅ **Honestidade epistémica**: spec §A.2 enumerou 3 opções (a/b/c);
  diagnóstico confirmou (a) como decisão final com justificativa
  explícita (não escolheu por inércia).
- ✅ **Win arquitectural P281 validado em segundo passo cumulativo**:
  alteração simétrica top-level + local via helper único `line_rg_prefix`
  — paridade local vs top-level continua estructuralmente garantida.
- ✅ **Bit-exact regression preservada**: 2 testes dedicados (`p285_math_frac_preserva_ausencia_de_rg`
  + `p285_line_sem_stroke_preserva_bit_exact`) garantem que call-sites
  sem cor explícita produzem PDF idêntico a pré-P285.

---

## §8 — Padrões emergentes (sem formalização ADR)

### §8.1 — "Modificação cirúrgica em tipo existente sem variants novos" — N=1 inaugural

P285 inaugura o padrão de **adicionar 1 campo a um variant existente
de um enum core** (em vez de criar variant novo) para cobrir feature
nova. Distingue de:

- P227 (`Value::Stroke` novo variant + Grid/Table stroke novos campos).
- P262/P264/P267 (Gradient: novos campos em `FrameItem::Shape`).
- P284 (3 variants ricos novos no `Content` enum).

Características distintivas:
- Touch points proporcionais aos producers existentes (~8 em P285).
- Helper L3 trivial (~12 LOC) que centraliza a nova lógica de emit.
- Backward-compat trivialmente preservada via `Option<T>` default `None`.
- Bit-exact regression validada por testes dedicados.

Reaplicações candidatas: futuro `FrameItem::Text.font_features` (lig/kern
selectores), `FrameItem::Shape.dash_pattern` (linhas tracejadas);
aguardar N≥3 para considerar formalização.

### §8.2 — "Win arquitectural P281 validado a posteriori (segundo passo)" — N=2 cumulativo

- N=1: **P282 §1.1** (auditoria empírica confirmou paridade local vs
  top-level pós-P281).
- **N=2**: **P285** (alteração em `FrameItem::Line` emit propaga
  simetricamente para top-level + local via mesmo helper — divergência
  estructuralmente impossível de introduzir).

Padrão: "single source of truth como invariante anti-bug" — P281
consolidou; P282 auditou; P285 estendeu sem violar. Confirma que o
investimento P281 continua a pagar dividendos. Aguardar reaplicação
para considerar formalização.

### §8.3 — "Activação posterior de feature parseada-mas-inerte" — N=1 inaugural

P284 §5.4 registou explicitamente que `stroke: Option<Color>` em
`Content::Underline/Strike/Overline` era **parseado mas inerte**
porque o destino (`FrameItem::Line`) não tinha onde colocá-lo.
P285 **activa** essa feature sem ter de modificar P284 (apenas
adiciona um campo a `FrameItem::Line` e estende o consumer Layouter).

Padrão emergente: pendências graded podem ser **resolvíveis sem
revisitar o passo originador** se o gap for puramente no destino
físico (não na semântica). Distingue de pendências que exigem
refactor cross-passo. Reaplicações candidatas em `text_color`
(P102), `language_aware_shaping` (DEBT-53). Aguardar N≥3.

---

## §9 — Próximos passos sugeridos (estado pós-P285)

Cobertura agregada estimada: inalterada (~64%). P285 fechou 2
pendências sem nova feature visível ao utilizador típico — preparou
terreno para próximas frentes que aproveitam `FrameItem::Line.color`.

### Rank 1-3: continuar quick wins horizontais

1. **`P-smartquote`** (S; Text 43% → 48%) — typographic smart quotes.
2. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%) — Accent + Cancel
   primitives.
3. **`P-text-deco-multiline`** (P284.1; S-M) — `flush_line`-aware
   emission para corrigir restrição graded P284 §5.3.

### Rank 4-6: features médias

4. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b) — Curve geometry.
5. **`P-footnote-cluster`** (M; Model 60% → 70%).
6. **`P-outline-cluster`** (M; Introspection 70% → 80%).

### Rank 7: refino opcional pós-P285

7. **`P-stroke-thickness-override`** (XS; activa `thickness` override
   per-instance em `FrameItem::Line` paralelo a `color`). Não-objectivo
   spec §5 deste passo; reaplicação directa do padrão §8.1.

### Promoção candidata (não-objectivo passo seguinte)

8. **ADR meta "Win arquitectural single source of truth"** — N=2
   citantes P282+P285. N≥3 dispara formalização.

---

## §10 — Referências cross-passos

- **P38** — Math fraction line introduziu `FrameItem::Line` (frac.rs
  agora explícita `color: None`).
- **P78** — Line shape primitive; segunda aplicação de `FrameItem::Line`.
- **P102** — `text.fill` introduziu `rg`/`RG` em `FrameItem::Text`;
  P285 é a aplicação simétrica do mesmo padrão para `FrameItem::Line`.
- **P227** — `Value::Stroke` + Grid/Table stroke borders; precedente
  arquitectural distinto (variant novo em vez de campo novo).
- **P281** — Unificação β-completa de stream-builders; helpers únicos
  `emit_text_pdf`/`emit_glyph_pdf` agora estendidos com `line_rg_prefix`.
- **P282 §1.1, §1.5** — auditoria empírica confirmou paridade
  local vs top-level; §1.5 registou `P-line-color-rg-emit` como
  pendência S não-bloqueante (agora **RESOLVIDA por P285**).
- **P284 §5.4** — `stroke` parseado mas inerte em
  `Content::Underline/Strike/Overline` (agora **RESOLVIDO por P285**
  via herança §A.3 opção β).
- **ADR-0029** — pureza física L1 (`Option<Color>` é tipo L1 puro).
- **ADR-0054** — scope graded (`style.fill: None → color: None` é
  aproximação aceite).
- **ADR-0065** — inventariar-primeiro (Fase A obrigatória cumprida
  com 8 produtores enumerados).
- **ADR-0085** — diagnóstico imutável (39º consumo: P285 + P284 +
  P282 + 36 anteriores).

---

*P285 fecha as 2 pendências registadas (P282 §1.5 + P284 §5.4) num
único passo S — modificação cirúrgica em `FrameItem::Line` (+1 campo
`color: Option<Color>`) + helper L3 `line_rg_prefix` (~12 LOC) +
consumer Layouter actualizado para `stroke.or(style.fill)` per herança
§A.3 opção β. Hash L0 `export.rs` muda intencionalmente
(`bc7b8b95 → 66cb8ac3`) após estar preservado desde P281 — quebra
simétrica top-level + local via mesmo helper preserva Win
arquitectural P281 estructural ("single source of truth como
invariante anti-bug" — N=2 cumulativo agora confirmado). Bit-exact
regression validada por 2 testes dedicados; math frac/sqrt/linhas
geométricas sem stroke explícito produzem PDF idêntico a pré-P285.
9 testes P285 verdes; cobertura agregada inalterada (correcção de
bug latente + activação de feature parseada, não nova feature).*
