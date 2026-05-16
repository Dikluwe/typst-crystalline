# Passo 261 — Paint enum (sequência arquitectural Visualize pós-P259 Cenário B2 Opção 1)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada **S+**
(M cap; ~1-2h se decisão minimalista; mais se cascade alargada).
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0029 (EM VIGOR — obriga diagnóstico vanilla + ADR explícita
  para scope-outs).
- ADR-0033 (paridade observable).
- ADR-0034 (diagnóstico canónico — agora paralelo com ADR-0085
  para audits Fase A).
- **ADR-0083** (Color paridade vanilla — P257; modelo análogo
  para este passo).
- **ADR-0084** (auditoria condicional EM VIGOR P260 — não
  obrigatória aqui pois não é audit).
- **ADR-0085** (diagnóstico imutável EM VIGOR P260 — diagnóstico
  vanilla cumpre forma análoga).
- ADR-0039 (TextStyle SR — Struct Resolvido; **preservar
  intacto** per decisão minimalista P261).
- ADR-0054 (perfil graded).
- ADR-0065 (inventariar primeiro).
- DEBT-1 (fechado P142; **não reabrir** per decisão minimalista).
- Relatórios precedentes: P252 (Stroke cross-cutting refactor;
  precedente arquitectural N=1), P257 (Color paridade; N=2 do
  mesmo pattern), P259 §3 P259.C (Opção 1 spec preliminar de
  Paint enum + Gradient Linear).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  (Fase A diagnóstico vanilla per ADR-0029 §"Diagnosticar
  primeiro"; imutável per ADR-0085).
- ADR nova (provável ADR-0086) — "Paint wrapper com subset
  materializado: Solid(Color) only" per ADR-0029 §"Simplificações
  aceites apenas com ADR explícita".
  - Alternativa: **anotação cumulativa ADR-0083** se decisão de
    granularidade fundir Paint+Color no mesmo ADR (validação Fase
    A decide).
- Prompt L0 novo `00_nucleo/prompts/entities/paint.md`
  (estrutura análoga a `entities/color.md`).
- Código L1 novo `01_core/src/entities/paint.rs` (enum + tests).
- Código L1 actualizado em `01_core/src/entities/geometry.rs`
  (`Stroke.paint: Color → Paint`) + L0 prompt actualizado.
- Stdlib: nenhuma nova func — `native_rgb` etc. continuam a
  retornar `Value::Color`; Paint::Solid é wrapper interno.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-261-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Ordem: diagnóstico vanilla → ADR → prompt L0 →
   fix-hashes → testes-primeiro → código.
2. **Decisão minimalista declarada (paridade P25 → P257
   pattern)** — Paint::Solid(Color) only inicialmente;
   variants Gradient/Tiling ficam **comentados como reservas
   explícitas** no enum, não unit placeholders. Expansão
   consumer-driven em P262+ quando Gradient real materializar.
3. **ADR-0039 preservada literal** — `TextStyle.fill:
   Option<Color>` **não** vira `Option<Paint>` neste passo.
   DEBT-1 fechado preservado. Apenas `Stroke.paint` adapta.
4. **ADR-0029 §"Simplificações aceites apenas com ADR
   explícita"** — Solid only é simplificação face ao vanilla
   (que tem 3 variants); ADR nova ou anotação ADR-0083 documenta:
   - Diferença vanilla vs cristalino.
   - Custo semântico (zero — wrapper transparente para
     consumers).
   - Critério revisão (passo específico P262 Gradient Linear).
5. **Ordem testes-primeiro** — para cada código novo: testes
   antes de implementação.
6. **`crystalline-lint .`** zero violations no fim do passo.
7. **Tests workspace** sem regressão (contagem ≥ baseline
   2334 pós-P260).
8. **Materialization é leitura proibida por iniciativa
   própria**.
9. **Política "sem novas reservas"** preservada — variants
   Gradient/Tiling como comentários no enum são **roadmap
   visual**, não DEBT/ADR novo.

---

## §1 — Sub-passo P261.A: Fase A diagnóstico vanilla obrigatório

**Objectivo**: produzir inventário literal de Paint vanilla
+ analisar impacto cross-cutting nos consumers cristalino.

**Materialização**: zero código novo. Apenas leitura e
diagnóstico imutável per ADR-0085.

### Acções obrigatórias

#### A.1 — Leitura literal do vanilla

```bash
# Estrutura Paint vanilla
view lab/typst-original/crates/typst-library/src/visualize/paint.rs

# Variants Paint enum
grep -n "^\s*pub enum Paint\|impl Paint " \
  lab/typst-original/crates/typst-library/src/visualize/paint.rs

# Conversões From<Color> / From<Gradient> / From<Tiling>
grep -n "From<\|impl Paint " \
  lab/typst-original/crates/typst-library/src/visualize/paint.rs

# Stroke vanilla — confirmar se usa Paint ou Color directo
grep -n "pub paint\|fill\b" \
  lab/typst-original/crates/typst-library/src/visualize/stroke.rs

# Fill vanilla
grep -rn "FillElem\|fill:.*Paint" \
  lab/typst-original/crates/typst-library/src/visualize/ | head -20
```

#### A.2 — Consumers actuais cristalino (impacto da mudança)

```bash
# Onde Color é usado hoje (lista exhaustiva)
grep -rn "use.*Color\|: Color\b\|Color::\|paint: Color" \
  01_core/src/ 03_infra/src/ 02_shell/src/

# Stroke.paint consumers
grep -rn "\.paint\b\|Stroke\s*{\|stroke.paint" \
  01_core/src/ 03_infra/src/

# Style::Fill consumers
grep -rn "Style::Fill\|TextStyle.*fill\|FrameItem::Text.*fill" \
  01_core/src/ 03_infra/src/

# Native funcs cor stdlib
grep -n "Value::Color\|native_rgb\|native_luma\|native_oklab" \
  01_core/src/rules/stdlib/

# PDF exporter cor emit
grep -n "rg\b\|RG\b\|Color::" 03_infra/src/export.rs | head -20
```

**Output esperado**:
- **Stroke.paint** — único consumer estrutural que precisa de
  adaptação P261 (P252 estendeu com `overhang: bool`).
- **Style::Fill(Color)** — variant enum StyleChain; **NÃO toca
  P261** per decisão minimalista (preserva ADR-0039).
- **TextStyle.fill: Option<Color>** — cache resolvido ADR-0039;
  **NÃO toca P261**.
- **FrameItem::Text.style.fill** — consumer cache; **NÃO toca
  P261**.
- **PDF exporter** — emite cor de `TextStyle.fill` (Color) e de
  `Stroke.paint` (Paint pós-P261; converte para Color via
  `Paint::to_color()`).
- **Stdlib native_rgb etc.** — continuam retornando `Value::Color`
  (não Paint).

#### A.3 — Decisão arquitectural Paint::Solid only vs full enum

**Análise vanilla**:
- vanilla `enum Paint { Solid(Color), Gradient(Gradient),
  Tiling(Tiling) }` 3 variants.
- vanilla Stroke.paint: Paint (não Color directo).
- vanilla Fill.paint: Paint.

**Análise cristalino actual**:
- Stroke.paint: Color directo (P252).
- Style::Fill(Color), TextStyle.fill: Color directo
  (ADR-0038/0039).
- Sem Gradient/Tiling materializado (P259 confirmou ausência).

**Decisão proposta P261** (a confirmar Fase A):

| Variant Paint | Status P261 | Razão |
|---------------|-------------|-------|
| Solid(Color) | Materializar | wrapper para Stroke.paint |
| Gradient(Gradient) | **Comentário reserva** | Sem Gradient L1; activa em P262 |
| Tiling(Tiling) | **Comentário reserva** | Sem Tiling L1; baixa prioridade |

**Conversão `From<Color> for Paint`** — implementação trivial
(`Paint::Solid(color)`); permite call sites legacy
`Stroke { paint: color }` continuarem a compilar literalmente
durante migração (via auto-deref/auto-from).

#### A.4 — ADR explícita decisão (granularidade)

Per ADR-0029 §"Simplificações aceites apenas com ADR
explícita", Solid only obriga ADR. **Duas opções de
granularidade**:

**Opção α — ADR-0086 nova**:
- Análoga a ADR-0083 (Color paridade subset).
- Foco preservado em Paint específico.
- +1 ADR (total 72 → 73).

**Opção β — Anotação cumulativa ADR-0083**:
- ADR-0083 ganha secção "Anotação P261 — Paint wrapper enum
  materializado com Solid only" cobrindo Paint como extensão
  natural do trabalho Color P257.
- Total ADRs preservado em 72.
- Mistura âmbitos (Color paridade + Paint wrapper).

**Recomendação preliminar**: **Opção α (ADR-0086 nova)**
per precedente ADR-0083 (cada tipo vanilla tem ADR próprio
para scope-outs). Decisão final fica em Fase A §A.4 do
diagnóstico imutável.

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
com a seguinte estrutura (imutável após criação per
ADR-0085):

```markdown
# Diagnóstico Paint vanilla — Passo 261 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-261.md` (spec).
**Análogo estrutural**: `diagnostico-color-vanilla-passo-257.md`
(P257 Color paridade vanilla).

---

## §1 — Estrutura literal vanilla Paint

(Colar output A.1 — definição enum vanilla + conversões.)

## §2 — Consumers cristalino actual (impacto)

(Colar output A.2 — listar paths exactos com linhas.)

## §3 — Decisão Solid only vs full enum

(Tabela A.3 com decisão por variant.)

## §4 — Decisão granularidade ADR

☐ Opção α — ADR-0086 nova.
☐ Opção β — Anotação cumulativa ADR-0083.

Decisão escolhida: _.
Justificação: _.

## §5 — Plano materialização P261.C

(Cross-references entre sub-passos B/C/D.)

## §6 — Limitações conscientes

- Paint::Solid only (Gradient/Tiling comentários reserva).
- TextStyle.fill preservado Color literal (ADR-0039 intacto).
- Stdlib native_rgb continua retornar Value::Color (não Paint).

## §7 — Referências

- ADR-0029, ADR-0033, ADR-0083, ADR-0085.
- ADR-0039 (TextStyle SR — preservado).
- P252 (Stroke cross-cutting precedente N=1).
- P257 (Color paridade precedente N=2).
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/paint.rs`.
```

### Critério de aceitação P261.A

- Ficheiro
  `diagnostico-paint-vanilla-passo-261.md` criado em
  `00_nucleo/diagnosticos/`.
- §1-§7 preenchidos com conteúdo literal.
- Decisão Opção α/β em §4 explicitada.
- Lista exhaustiva consumers em §2.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md (ainda — vem
  em P261.B+).

---

## §2 — Sub-passo P261.B: ADR explícita (conforme decisão §A.4)

**Objectivo**: cumprir ADR-0029 §"Simplificações aceites
apenas com ADR explícita".

### Acções (conforme Opção α ou β escolhida em Fase A §4)

#### B.1 (Opção α) — Criar ADR-0086

Ficheiro novo
`00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md`:

- **Status**: `PROPOSTO` (transita a `IMPLEMENTADO` em P261.D
  pós-materialização).
- **Contexto**: Visualize Paint vanilla tem 3 variants;
  cristalino actual usa Color directo em Stroke; P259 §3 Opção
  1 spec preliminar de Paint enum; ADR-0083 P257 precedente
  Color subset materializado.
- **Decisão**: enum Paint com Solid(Color) only inicialmente;
  Gradient/Tiling comentários reserva.
- **Análise paridade**:
  - Paridade vanilla face a Solid: idêntica.
  - Paridade face a Gradient/Tiling: scope-out documentado.
- **Scope-outs**:
  - Gradient → P262 (Gradient Linear materialização).
  - Tiling → passo futuro sem prioridade.
- **Consequências**:
  - **Positivas**: enum shape pronta para Gradient consumer;
    `From<Color>` permite call sites legacy.
  - **Negativas**: +1 indirecção em Stroke; insignificante.
  - **Neutras**: ADR-0039 preserved (`TextStyle.fill: Color`
    inalterado).
- **Alternativas**:
  - α1 — Color directo (status pré-P261; rejeitada — bloqueia
    Gradient consumer real).
  - α2 — Paint::Solid only (escolhida).
  - α3 — Paint full enum desde já com Gradient/Tiling unit
    placeholders (rejeitada — variants vazios sem consumer
    poluem enum).
- **Critério revisão**: P262 Gradient Linear materializa
  Gradient real; Paint::Gradient activa.
- **Referências**: ADR-0083 (Color), ADR-0029 (regra),
  ADR-0085 (diagnóstico imutável).

#### B.1 (Opção β) — Anotação cumulativa ADR-0083

Editar
`00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md`
acrescentando secção:

```markdown
## Anotação cumulativa P261 — Paint wrapper enum Solid only

(Data 2026-05-15)

Paint wrapper materializado como extensão do trabalho Color
P257. enum Paint { Solid(Color) } com Gradient/Tiling
comentários reserva. Stroke.paint: Color → Paint. ADR-0039
preservado (TextStyle.fill Color inalterado).

Cross-references:
- Diagnóstico vanilla: diagnostico-paint-vanilla-passo-261.md.
- Materialização: entities/paint.rs.
```

### B.2 — README ADRs

Se Opção α:
- Distribuição: PROPOSTO 11 → **12** (ADR-0086 PROPOSTO).
- Total 72 → **73**.
- Em P261.D transita PROPOSTO → IMPLEMENTADO; distribuição
  ajusta a IMPLEMENTADO 25 → **26**; PROPOSTO 12 → **11**.

Se Opção β:
- Sem alteração de contagens.
- Linha ADR-0083 ganha referência cumulativa P261.

### Critério de aceitação P261.B

- Decisão Opção α/β concretizada.
- Se α: ADR-0086 criada PROPOSTO; README actualizado.
- Se β: ADR-0083 anotada; sem novas ADRs.
- Zero violations lint.

---

## §3 — Sub-passo P261.C: Materialização

### C.1 — Criar prompt L0 `entities/paint.md`

**Pré-requisito** Regra de Ouro: prompt antes de código.

Estrutura per padrão `entities/color.md` analógico:

```markdown
# Prompt L0 — `entities/paint`
Hash do Código: (gerado por --fix-hashes)

## Módulo
`01_core/src/entities/paint.rs`

## Camada
L1

## Propósito
Wrapper enum sobre fontes de cor para preenchimentos e
contornos. Substitui Color directo em Stroke (P261) para
abrir caminho a Gradient/Tiling (P262+; per ADR-0086).

## Estrutura

```rust
pub enum Paint {
    Solid(Color),
    // Gradient(Gradient),  // P262 — comentário reserva
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self;
    pub fn to_color(&self) -> Color;
}

impl From<Color> for Paint;
```

## Critérios de verificação

- `Paint::solid(c).to_color() == c` para qualquer Color.
- `Paint::from(c) == Paint::Solid(c)`.
- `PartialEq` derivado.
- `Copy` se Color é Copy.

## Sobre paridade vanilla

Vanilla tem 3 variants. P261 materializa Solid only;
Gradient/Tiling per ADR-0086 §scope-outs.

## Sobre ADR-0039

`TextStyle.fill: Option<Color>` preservado literal —
P261 não migra para Option<Paint>. Apenas Stroke.paint
adapta. Refino futuro pode migrar TextStyle.fill se
Gradient para texto for prioritário.
```

Calcular hash via `--fix-hashes`.

### C.2 — Materializar `01_core/src/entities/paint.rs`

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

#### C.2.1 — Testes

```rust
// 01_core/src/entities/paint.rs (tests submodule)

#[test]
fn paint_solid_construcao() {
    let c = Color::rgb(255, 0, 0);
    let p = Paint::solid(c);
    assert_eq!(p, Paint::Solid(c));
}

#[test]
fn paint_to_color_solid() {
    let c = Color::rgb(0, 128, 255);
    assert_eq!(Paint::Solid(c).to_color(), c);
}

#[test]
fn paint_from_color() {
    let c = Color::rgb(64, 64, 64);
    let p: Paint = c.into();
    assert_eq!(p, Paint::Solid(c));
}

#[test]
fn paint_partial_eq() {
    let p1 = Paint::Solid(Color::rgb(1, 2, 3));
    let p2 = Paint::Solid(Color::rgb(1, 2, 3));
    let p3 = Paint::Solid(Color::rgb(4, 5, 6));
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

// Tests adicionais: clone, debug, copy se aplicável.
```

Executar `cargo test paint::` — verificar falham.

#### C.2.2 — Implementação

```rust
// 01_core/src/entities/paint.rs

use crate::entities::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Paint {
    Solid(Color),
    // Gradient(Gradient),  // P262 — reserva
    // Tiling(Tiling),      // futuro — reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self {
        Paint::Solid(c)
    }

    pub fn to_color(&self) -> Color {
        match self {
            Paint::Solid(c) => *c,
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self {
        Paint::Solid(c)
    }
}
```

#### C.2.3 — Expor em `entities/mod.rs`

```rust
pub mod paint;
pub use paint::Paint;
```

Executar `cargo test paint::` — verificar passam.

### C.3 — Actualizar `entities/geometry.rs`

#### C.3.1 — L0 prompt actualizado

`00_nucleo/prompts/entities/geometry.md` ganha secção:

```markdown
## Anotação P261 — Stroke.paint: Color → Paint

(Data 2026-05-15)

`Stroke { paint: Paint, thickness: f64, overhang: bool }`
substituindo `paint: Color`. Migração consumer-side via
`impl From<Color> for Paint` — call sites legacy
`Stroke { paint: color, ... }` continuam compilar.

Cross-references:
- entities/paint.md (Paint definido P261).
- ADR-0086 (PROPOSTO; promovida IMPLEMENTADO P261.D).
```

Propagar hash via `--fix-hashes`.

#### C.3.2 — Código

```rust
// 01_core/src/entities/geometry.rs

use crate::entities::paint::Paint;

pub struct Stroke {
    pub paint:     Paint,      // antes: Color
    pub thickness: f64,
    pub overhang:  bool,       // P252
}
```

#### C.3.3 — Consumers de `Stroke.paint`

**Análise impacto** (Fase A §A.2 lista exhaustiva):

| Sítio | Acção P261 |
|-------|------------|
| Construção `Stroke { paint: color, ... }` | **Compila via From<Color>** se idiomático |
| Construção `Stroke::new(color, thickness)` | Helper preservado; assina Color; internal `into()` |
| Pattern-match `Stroke { paint, .. }` | `paint` agora é `Paint` (refactor pattern) |
| `stroke.paint` leitura cor | `stroke.paint.to_color()` |
| PDF exporter `stroke.paint` | `stroke.paint.to_color()` (consume Color) |

Cobertura exaustiva esperada **~5-10 sítios** (vs 27 P257 Color).
Granularidade preservada.

### C.4 — PDF exporter intocado

`03_infra/src/export.rs` continua a emitir Color directamente
de:
- `FrameItem::Text.style.fill: Option<Color>` — inalterado.
- `Stroke.paint.to_color()` — wrapper transparente.

Zero refactor exporter.

### C.5 — Stdlib intocado

`native_rgb`/`native_luma`/etc. continuam retornar
`Value::Color`. Paint::Solid é wrapper interno; user-facing
não vê Paint.

Refino futuro P262 Gradient adicionará `native_gradient_linear`
que retorna `Value::Gradient` (ou `Value::Paint` se decisão
unificada).

### C.6 — Verificação final P261.C

```bash
cargo build --workspace
# Esperado: verde
RUST_MIN_STACK=33554432 cargo test --workspace
# Esperado: 2334 → 2342-2346 (+8-12 P261)
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo run -p crystalline-lint -- --fix-hashes .
# Esperado: 2 hashes propagados (paint.md + geometry.md)
```

### Critério de aceitação P261.C

- `entities/paint.rs` materializado com 5+ tests verdes.
- `entities/geometry.rs` `Stroke.paint: Paint`.
- Consumers adaptados via From<Color> ou refactor pattern
  pontual.
- Tests workspace **2334 → 2342-2346** (+8-12 esperado).
- Zero violations linter.
- Hashes propagados.

---

## §4 — Sub-passo P261.D: Fecho ADR + relatório

### D.1 — Promover ADR-0086 PROPOSTO → IMPLEMENTADO
(se Opção α)

`00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md`:
- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Adicionar linha **Validado**: P261.
- Adicionar secção **Aplicação**: referência a
  `00_nucleo/materialization/typst-passo-261-relatorio.md`.

**Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo"**:
P257 N=1; **P261 cresce N=1 → N=2** (limiar formalização
N=3 começa a ser próximo).

### D.2 — Actualizar README ADRs

Se Opção α:
- Distribuição: PROPOSTO 12 → **11** (ADR-0086 promovida);
  IMPLEMENTADO 25 → **26**.
- Total preservado **73**.

Se Opção β:
- Sem alteração de contagens; total preservado **72**.

### D.3 — Subpadrão "Refactor cross-cutting entity primitivo"

Cumulativo:
- N=1 P252 (Stroke `overhang` cross-cutting).
- N=2 P257 (Color expansão cross-cutting).
- **N=3 P261** (Paint cross-cutting Stroke.paint).

**Patamar N=3 atinge limiar formalização sólida**. Mencionar
em §6 padrões metodológicos do relatório como candidato a
ADR meta futuro (improvável — auto-documentado per ADR
individual).

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-261-relatorio.md`
estrutura canónica:

- **§1 Sumário executivo** — Fase A confirmada; ADR criada/
  anotada; tests delta (+8-12); ADRs distribuição.
- **§2 Sub-passo P261.A** — diagnóstico Paint vanilla
  resumido.
- **§3 Sub-passo P261.B** — ADR-0086 ou anotação ADR-0083.
- **§4 Sub-passo P261.C** — código materializado.
- **§5 Sub-passo P261.D** — ADR promovida; READMR actualizado.
- **§6 Padrões metodológicos** — ADR-0084/0085 consumidos
  (primeiro consumo real pós-P260); subpadrão "Refactor
  cross-cutting entity primitivo" cresce N=2 → 3; subpadrão
  "ADR PROPOSTO+IMPLEMENTADO mesmo passo" cresce N=1 → 2 (se
  Opção α).
- **§7 Cobertura** — Visualize ~52% → ~55% (+3pp via Paint
  wrapper structural).
- **§8 Limitações e trabalho futuro** — Gradient/Tiling
  scope-outs P262+; ADR-0039 preservado; expansão consumer-
  driven.
- **§9 Critério de aceitação global P261 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P261.D

- ADR-0086 IMPLEMENTADO (se Opção α) ou ADR-0083 anotada
  (se Opção β).
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P261

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace` retorna contagem ≥ baseline
  2334 + 8-12 (sem regressão; +8-12 P261 esperado).
- [ ] `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  existe com §1-§7 preenchidos.
- [ ] ADR-0086 criada e promovida a IMPLEMENTADO (Opção α)
  OU ADR-0083 anotada (Opção β).
- [ ] `00_nucleo/prompts/entities/paint.md` criado.
- [ ] `01_core/src/entities/paint.rs` materializado.
- [ ] `01_core/src/entities/geometry.rs` `Stroke.paint:
  Paint`.
- [ ] `entities/mod.rs` re-export Paint adicionado.
- [ ] Consumers Stroke.paint adaptados.
- [ ] **TextStyle.fill: Option<Color> preservado literal**
  (ADR-0039 intacto).
- [ ] **Stdlib native_rgb continua retornar Value::Color**
  (sem alteração user-facing).
- [ ] Exportador PDF intocado.
- [ ] Hashes propagados (`crystalline-lint --fix-hashes`).
- [ ] README ADRs actualizado.
- [ ] Relatório do passo criado.
- [ ] Paridade observable preservada (PDFs idênticos pré/pós
  para inputs Stroke com cor literal).

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0029, ADR-0033, ADR-0039
   (preservado), ADR-0083 (precedente), ADR-0084/0085
   (consumir), relatórios P252 + P257 + P259.
2. **Reportar** estado inicial: tests count (esperado 2334
   pós-P260) + lint baseline + ADRs 72.
3. **P261.A** — Executar comandos Fase A (A.1 + A.2 + A.3 +
   A.4); criar diagnóstico Paint vanilla imutável; decisão
   Opção α/β explícita.
4. **P261.B** — Criar ADR-0086 PROPOSTO OU anotar ADR-0083
   per decisão Fase A §4.
5. **P261.C** — Criar L0 prompt `entities/paint.md`;
   `--fix-hashes`; testes primeiro; implementação;
   actualizar `entities/geometry.rs` (Stroke.paint);
   adaptar consumers; verificar tests + lint.
6. **P261.D** — Promover ADR-0086 IMPLEMENTADO (se Opção
   α); actualizar README ADRs; criar relatório.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADR criada/anotada, tests
   delta, ficheiros criados/editados, recomendação P262
   (Gradient Linear) pós-P261.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P261.A revela que vanilla `Paint` enum tem mais variants
  do que esperado (e.g. 4+ variants — Solid/Gradient/Tiling
  + outro).
- P261.A revela que `Stroke.paint` em vanilla **não** é
  Paint (usa Color directo) — invalidaria justificação
  arquitectural deste passo.
- P261.A revela que **Style::Fill(Color)** tem dependência
  estrutural que exigiria refactor maior (e.g. show rules
  usam Paint::Gradient cross-Style — cascade alargada).
- P261.C revela que cascade `Stroke.paint` consumer
  ultrapassa 15 sítios (vs ~5-10 estimado) — magnitude real
  M+ em vez de S+; considerar adiar para passo dedicado.
- P261.C revela que `From<Color> for Paint` impl causa
  ambiguidade em algum call site (e.g. inferência tipo
  falha por overload).
- Decisão Opção α vs β é ambígua (e.g. ADR-0083 já tem
  3+ anotações cumulativas — Opção β polui ADR).
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.

Em qualquer paragem, registar contexto no relatório parcial
e aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P259 (Visualize audit)

P259 Cenário B2 §3 Opção 1 spec preliminar de Paint enum.
P261 executa Opção 1 sub-passo 1 (Paint). P262 candidato
sub-passo 2 (Gradient Linear).

### Subpadrão "Refactor cross-cutting entity primitivo" cresce N=2 → N=3

Cumulativo:
- N=1 P252 (Stroke `overhang` cross-cutting).
- N=2 P257 (Color expansão cross-cutting; toca exporter +
  stdlib + variants).
- **N=3 P261** (Paint cross-cutting Stroke.paint via
  From<Color>).

**Patamar N=3 atinge limiar formalização sólida**. Análogo
ADR-0064/0065 podem citar este pattern futuramente.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo via Cenário B1/B2"

Cumulativo:
- N=1 P257 (ADR-0083).
- **N=2 P261** (ADR-0086; se Opção α).

**Patamar N=2 reforça pattern**. N=3 começará a ser
candidato a formalização.

### Subpadrão "Consumo P260 ADRs" (primeiro real)

P260 formalizou ADR-0084 (auditoria condicional) + ADR-0085
(diagnóstico imutável). P261 **não** é audit (é materialização
arquitectural), mas consome ADR-0085 indirectamente (diagnóstico
Paint vanilla per ADR-0029 §"Diagnosticar primeiro" cumpre forma
análoga). **Não é consumo directo** — não é Fase A audit.

Próximo passo P262 (Gradient Linear) consumirá ADR-0085 ao
produzir diagnóstico Gradient vanilla imutável.

### Política "sem novas reservas"

Preservada. Gradient/Tiling como comentários no enum são
**roadmap visual**, não DEBT/ADR novo. Roadmap real fica em
ADR-0086 §"Critério revisão".

### Pós-P261 — sequência lógica recomendada

1. **P262 Gradient Linear** (M; +15-20 tests; +8pp Visualize).
   Activa `Paint::Gradient` variant.
2. **OU outras Opções P259 Cenário B2** (DEBT-33; Stroke<T>;
   Polygon dedicado; Ellipse refino).
3. **OU Text audit** (consumo directo ADR-0084 + 0085).
4. **OU Footnote refino** (Model pendência P258).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029, ADR-0033, ADR-0034, ADR-0054, ADR-0065 —
  metodologia básica.
- **ADR-0039** (TextStyle SR; preservado literal).
- **ADR-0083** (Color paridade vanilla; precedente N=2 do
  pattern).
- **ADR-0084, ADR-0085** (P260 — auditoria condicional +
  diagnóstico imutável; consumo natural).
- **ADR-0086** (criada por este passo; se Opção α).
- DEBT-1 (fechado P142; preservado).
- `lab/typst-original/crates/typst-library/src/visualize/paint.rs`
  — fonte canónica vanilla.
- P25 — Color simplificado original (REVOGADO via P257).
- P252 — Stroke `overhang` cross-cutting (precedente N=1).
- P257 — Color paridade vanilla 8/8 (precedente N=2;
  template "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=1).
- P259 — Visualize Fase A audit (Cenário B2 Opção 1 spec
  preliminar).
- P260 — ADRs meta (formaliza padrões consumidos
  indirectamente).
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — diagnóstico imutável precedente.
