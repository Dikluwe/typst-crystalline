# ⚖️ ADR-0083: Color paridade vanilla com subset materializado

**Status**: **`IMPLEMENTADO`** (PROPOSTO P257.B → **IMPLEMENTADO
P257.D** — materialização cumpre 8 variantes + scope-outs
documentados; paridade observable preservada).
**Data**: 2026-05-15 (PROPOSTO P257.B; **IMPLEMENTADO P257.D**)
**Autor**: Humano + IA
**Validado**: diagnóstico vanilla
`diagnostico-color-vanilla-passo-257.md` (P257.A) — leitura
literal de `lab/typst-original/crates/typst-library/src/visualize/color.rs`
(1996 linhas; 8 espaços enumerados em vanilla); **P257.C
materializou** 8 variantes em `01_core/src/entities/color.rs`
+ 7 stdlib funcs novas + paridade observable estricta
preservada (2304 → 2334 verdes; +30 tests; 0 regressões).
**Reservado para**: meta-arquitectural cumpre ADR-0029
§"Simplificações aceites apenas com ADR explícita" para
scope-outs específicos de Color em P257.

**Nota numeração**: spec P257 hipótese previa ADR-0067 mas
ADR-0067 já estava ocupado (`attribute-grammar-scoping`).
ADR-0083 escolhido como próximo slot disponível após ADR-0082
(paridade pattern P226.div-1 + P249.div-2 precedentes).

---

## Contexto

ADR-0028 (REVOGADA P84.8c) propôs Color simplificado a `enum
{ Rgb, Rgba }` com argumento "u8 cobre 24-bit; suficiente para
output PDF actual; precisão sub-byte adia-se".

ADR-0029 (EM VIGOR pós-P84.8c) revogou ADR-0028 e estabeleceu
**Regra de execução**:
1. Diagnosticar primeiro (estrutura vanilla literal).
2. Não simplificar sob pretexto de adiamento.
3. `Arc` permitido (gestão RAM).
4. **Simplificações aceites apenas com ADR explícita** que
   documente diferença + custo + critério de revisão.

ADR-0029 listou Color na enumeração de tipos tipográficos
(linha 144) mas **não materializou** a expansão — reservou
para passo dedicado.

**P257** executa: diagnóstico vanilla literal + materialização
subset com scope-outs documentados (este ADR).

---

## Decisão

### Regra vinculativa

**Color em cristalino é enum tagged com 8 variantes
correspondendo aos 8 espaços vanilla** (paridade estrutural
literal):

```rust
pub enum Color {
    Srgb     { r: f32, g: f32, b: f32, a: f32 },
    Luma     { l: f32, a: f32 },
    LinearRgb { r: f32, g: f32, b: f32, a: f32 },
    Oklab    { l: f32, a: f32, b: f32, alpha: f32 },
    Oklch    { l: f32, c: f32, h: f32, alpha: f32 },
    Hsl      { h: f32, s: f32, l: f32, a: f32 },
    Hsv      { h: f32, s: f32, v: f32, a: f32 },
    Cmyk     { c: f32, m: f32, y: f32, k: f32 },
}
```

### 8 espaços materializados

Todos os 8 espaços vanilla materializados estructuralmente.
Métodos públicos por variante:

- Construtores: `Color::rgb(r:u8,g:u8,b:u8)` (paridade
  cristalino existente; constrói `Srgb` com f32 normalizado),
  `Color::rgba(r,g,b,a:u8)`, `Color::oklab(l,a,b:f32)`,
  `Color::oklch(l,c,h:f32)`, `Color::linear_rgb(r,g,b:f32)`,
  `Color::cmyk(c,m,y,k:f32)`, `Color::hsl(h,s,l:f32)`,
  `Color::hsv(h,s,v:f32)`, `Color::luma(l:f32)`.
- `to_srgb() -> (u8,u8,u8,u8)` — conversão para sRGB byte
  (consumer PDF exporter via 4 caminhos cumulativos).
- `to_rgba_f32() -> (f32,f32,f32,f32)` — preservado para
  compatibilidade hot path PDF exporter.
- `PartialEq` exacto preservado per ADR-0028 regra herdada
  (sem tolerância em produção; f32 bitwise via `f32::to_bits`
  para comparação determinística).

### Scope-outs formais (ADR-0029 §"Simplificações aceites")

1. **PDF native `/DeviceCMYK`** — scope-out formal.
   - **Diferença vanilla vs cristalino**: vanilla emite CMYK
     nativo PDF; cristalino converte CMYK para sRGB no
     exporter via `Color::to_srgb()` antes de emitir bytes
     PDF.
   - **Custo semântico**: fidelity print degradada (CMYK
     converte-se para sRGB; impressoras profissionais podem
     querer CMYK nativo para colour management ICC).
   - **Critério de revisão**: passo dedicado **P-Color-CMYK-PDF**
     quando export print for prioritário (não vaga — passo
     específico futuro). Pré-requisitos: necessidade real
     identificada em consumer + ADR de autorização de embed
     ICC profiles se aplicável.
2. **Operadores cor (`lighten`/`darken`/`mix`/`saturate`/
   `desaturate`/`negate`)** — scope-out formal.
   - **Diferença**: vanilla tem `impl Color` com manipulação
     luminance/saturation/interpolação; cristalino expõe apenas
     `to_srgb()` + construtores.
   - **Custo semântico**: features vanilla `color.lighten(20%)`,
     `color.mix(red, blue, 50%)` não disponíveis cristalino
     pós-P257.
   - **Critério de revisão**: passo dedicado por operador
     **P-Color-Op-{lighten/darken/mix/etc.}** quando uso real
     surgir em consumers (e.g. show rules, gradient interpolation).
3. **`ColorSpace` enum runtime** — scope-out formal.
   - **Diferença**: vanilla expõe `Color::space() -> ColorSpace`
     + cast functions runtime para introspecção do espaço;
     cristalino expõe apenas match exhaustive em consumers
     (PDF exporter).
   - **Custo semântico**: features vanilla `color.space() ==
     oklab` não disponíveis cristalino.
   - **Critério de revisão**: passo dedicado **P-Color-Space-
     Runtime** quando uso surgir em consumers que precisam
     introspecção sem match exhaustive.
4. **Constantes nomeadas extras (vanilla 18+: NAVY/PURPLE/etc.)**
   — scope-out informal.
   - **Diferença**: vanilla expõe `Color::RED`, `Color::BLUE`,
     etc.; cristalino actual usa `parse_color` em
     `stdlib/shapes.rs` com lookup de string (5 cores: red,
     green, blue, black, white).
   - **Custo semântico**: stdlib cristalino tem cobertura
     limitada vs vanilla.
   - **Critério de revisão**: refino incremental via ADR-0080
     EM VIGOR (refactors aditivos) — sem ADR dedicada
     necessária. Expansão pode ocorrer sem novo passo.

### `PartialEq` exacto preservado

Per ADR-0028 regra herdada (sem tolerância em produção):
`Color` deriva `PartialEq` via `f32` exacto. Para conversões
entre espaços, equality compara representação interna **da
mesma variante**; conversão silenciosa via `to_srgb()` para
comparação cross-espaço NÃO é automática.

---

## Análise paridade

| Aspecto | Vanilla | Cristalino P257 | Paridade |
|---------|---------|------------------|----------|
| Número de espaços | 8 | 8 | ✓ estructural total |
| Representação interna | f32 normalizado | f32 normalizado | ✓ |
| Conversão para sRGB | matrix LMS+gamma | matrix LMS+gamma | ✓ semantic |
| PDF DeviceCMYK | native | converte para sRGB | ☐ scope-out |
| Operadores cor | lighten/mix/etc. | apenas to_srgb | ☐ scope-out |
| ColorSpace runtime | enum + introspection | match exhaustive | ☐ scope-out |
| Constantes nomeadas | 18+ | 5 (via stdlib) | △ parcial |
| Construtores u8 | `rgb(255,0,0)` | `rgb(255,0,0)` | ✓ paridade observable |

---

## Consequências

### Positivas

- **Paridade estrutural total** com vanilla (8/8 espaços).
- **Paridade observable preservada**: `rgb(255,0,0)` produz
  mesmos bytes PDF antes e depois (sentinela P257.C).
- **Cobertura Visualize aumenta** (Color é primeira entrada
  user-facing com maior peso na categoria Visualize).
- **Cumpre ADR-0029** §"Simplificações aceites apenas com ADR
  explícita" via scope-outs documentados.
- **Conversões cor (Oklab→sRGB, etc.)** disponíveis para
  consumers futuros (gradient interpolation, themes).

### Negativas (mitigadas)

- **Custo refactor ~15-20 consumers** — mitigado via
  preservação construtores `Color::rgb(u8,u8,u8)` paridade
  observable.
- **Cmyk PDF perde fidelity print** — mitigado via scope-out
  documentado + revisão passo futuro **P-Color-CMYK-PDF**.
- **Operadores cor não disponíveis** — mitigado via critério
  revisão por operador (passos dedicados quando uso surgir).

### Neutras

- `Color` migra de `entities/layout_types.rs` para ficheiro
  próprio `entities/color.rs` (paridade subpadrão "tipo em
  ficheiro próprio").
- Stdlib ganha 7 funcs novas (`native_oklab`, `native_oklch`,
  `native_cmyk`, `native_hsl`, `native_hsv`,
  `native_linear_rgb`, `native_luma`). `native_rgb`/`native_rgba`
  preservadas literal.

---

## Alternativas consideradas

### Alternativa 1 — Materializar tudo (8 espaços + operadores + DeviceCMYK)

Custo: L+ (~10-15h cumulativo) vs M (~3-5h) recomendado.
**Rejeitada**: magnitude excede tolerância P257; operadores
sem consumer real cristalino actual (over-engineering); CMYK
PDF native exige refactor exporter substancial.

### Alternativa 2 — Materializar nada (preservar `enum { Rgb, Rgba }`)

Custo: zero, mas viola ADR-0029 §"Diagnosticar primeiro" +
§"Não simplificar sob pretexto de adiamento".
**Rejeitada**: ADR-0029 já recusou esta alternativa
explicitamente.

### Alternativa 3 — Subset proposto (8 estructural + scope-outs operadores/PDF native/runtime)

**Adoptada**. Cumpre ADR-0029 (estructural total + ADR
explícita para scope-outs) + magnitude controlada M + paridade
observable preservada.

---

## Plano de aplicação

P257.C executa imediatamente após este ADR PROPOSTO:

1. Criar prompt L0 `entities/color.md`.
2. `crystalline-lint --fix-hashes` propaga hash.
3. Testes primeiro (~20-40 tests).
4. Materializar `entities/color.rs` (8 variantes + conversões).
5. Remover `Color` de `layout_types.rs`.
6. Adaptar ~15-20 consumers via cascade.
7. Adaptar PDF exporter (4 caminhos `to_rgba_f32`).
8. Stdlib funcs novas (7) + testes.
9. P257.D promove ADR-0083 PROPOSTO → IMPLEMENTADO.

**Critério aceitação P257**:
- 2304 verdes pré-P257 → 2324-2344 pós-P257 (+20-40 tests).
- Zero violations linter.
- Hash propagado (`entities/color.md`).
- Paridade observable estricta: `rgb(255,0,0)` produz mesmos
  bytes PDF antes e depois.

---

## Critério de promoção

ADR-0083 transita PROPOSTO → IMPLEMENTADO quando:

1. P257.C completar materialização (8 variantes + consumers
   adaptados + PDF + stdlib).
2. P257.D verificar critério aceitação satisfeito (tests +
   lint + hashes + paridade observable).
3. Relatório P257 documenta scope-outs aplicados literais.

**Decisão de promoção é humana** — automática se P257.D
atinge critérios sem regressão.

---

## Referências

- **ADR-0028** (REVOGADA P84.8c) — Color simplificado.
- **ADR-0029** (EM VIGOR) — Pureza física L1 + obriga
  diagnóstico vanilla + ADR explícita para scope-outs.
- **ADR-0033** — Paridade observable vanilla.
- **ADR-0034** — Diagnóstico canónico (P257.A imutável).
- **ADR-0054** — Perfil graded (scope-outs aceites com
  justificação).
- **ADR-0065** — Inventariar primeiro (P257.A diagnóstico).
- **ADR-0080** — L0 minimal para refactors aditivos (cobre
  expansão futura constantes nomeadas).
- **Passos**: P25 (Color simplificado original), P84.8c
  (ADR-0029 promulgada), **P257.A** (diagnóstico imutável),
  **P257** (este passo + materialização).
- `lab/typst-original/crates/typst-library/src/visualize/color.rs`
  (1996 linhas; 8 espaços enumerados).
- `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
  — fonte literal Fase A.

---

## Próximos passos

1. P257.C executa materialização imediata.
2. P257.D promove ADR-0083 → IMPLEMENTADO.
3. Passos futuros previstos por scope-out:
   - **P-Color-CMYK-PDF** quando print for prioritário.
   - **P-Color-Op-{lighten/darken/mix/etc.}** quando consumer
     surgir.
   - **P-Color-Space-Runtime** quando introspecção runtime
     for necessária.

Cada scope-out é candidato a passo dedicado pequeno (XS-M);
sem DEBT novo per política P158.

---

## Anotação cumulativa P259 (2026-05-15) — Cobertura Visualize agregada

Audit Fase A Visualize (P259.A) auditou 27 entradas dos
subsistemas Color + Shapes + Stroke + Image + Transform +
Gradient + Paint + Tiling + Clip; output literal em
`00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`.

**Color preservado**: 8/8 espaços + 7 stdlib funcs + zero
operadores cor + scope-out CMYK PDF preservado. ADR-0083 status
**IMPLEMENTADO preservado literal**.

**Cobertura Visualize empírica P259** (Tabela B):
- implementado: 10/27 (37%).
- implementado⁺: 4/27 (15%) — Color, Ellipse, Path, Stroke base.
- ausente: 13/27 (48%).
- Ponderada linear: 51.9%; ponderada com bonus implementado⁺
  ~54.8%.

**Promoções não-documentadas detectadas em P259.A**:
- Ellipse parcial → implementado⁺ (P242 Bézier kappa real, não
  placeholder rect DEBT-31).
- Polygon ausente → implementado (stdlib `native_polygon` via
  Path conversion).
- Stroke base implementado → implementado⁺ (P252 overhang
  cross-cutting refactor).

**Cenário Fase B confirmado**: B2 (55-70%). Opções para P260+
dedicados:
- Opção 1 (Paint enum + Gradient Linear) — sequência preferida.
- Opção 2 (Polygon variant separada + Curve) — parcial pré-cumprida.
- Opção 3 (DEBT-33 + Stroke<Length>) — refinos qualitativos.
- Opção 4 (Transform origin pivot) — scope-out ADR-0061
  preservado; materializar reverte decisão arquitectural
  existente em `transforms.rs:104-105`.
- Opção 5 (SVG image format) — L+ adiar.

**P259.C saltado** per decisão local (preservar política
administrativa documental + scope-out ADR-0061 sobre Opção 4).

**Subpadrão "auditoria condicional" N=5 cumulativo** (P192A +
P255 + P257 + P258 + **P259**). **Patamar N=5 atinge limiar
formalização clara** — candidato a formalizar em ADR meta admin
XS futuro.

**Subpadrão "Diagnóstico imutável precedente à acção" N=4
cumulativo** (P255 + P257 + P258 + **P259**).

Status ADR-0083 preservado literal (`IMPLEMENTADO`). Color
subsistema cobertura 100% estrutural preservada.
