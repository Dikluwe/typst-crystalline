# Passo 257 — Color paridade vanilla (Leitura B — funcional)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada **M-L**
(diagnóstico + materialização + adaptação consumers + tests).
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0028 (revogada — contexto histórico).
- ADR-0029 (em vigor — **obriga diagnóstico vanilla + ADR explícita
  para scope-outs**).
- ADR-0033 (paridade observable).
- ADR-0034 (diagnóstico canónico).
- ADR-0054 (perfil graded — scope-outs aceites com justificação).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
  (Fase A executada — estrutura literal do vanilla; imutável
  após criação per ADR-0034).
- ADR nova (ADR-0067 ou próximo disponível) se Fase B
  materializar subset com scope-outs — exigido por ADR-0029
  §"Simplificações aceites apenas com ADR explícita".
- Prompt L0 novo `00_nucleo/prompts/entities/color.md`
  (estrutura nova; ADR-0029 obriga prompt antes do código).
- Código L1 novo `01_core/src/entities/color.rs` (struct ou
  enum conforme decisão Fase A).
- Código L1 actualizado em `01_core/src/entities/layout_types.rs`
  (Color removido) + todos os consumers (Stroke, Style::Fill,
  FrameItem::Text, exportador PDF).
- Stdlib actualizado: `native_rgb`, `native_luma`, e novas
  funções para cada espaço materializado (`native_oklab`,
  `native_cmyk`, etc. conforme Fase B decidir).
- DEBT-4 (se ainda aberto e referenciar cor) actualizado ou
  encerrado.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-257-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Order: diagnóstico → ADR → prompt L0 → fix-hashes
   → testes-primeiro → código.
2. **ADR-0029 §"Diagnosticar primeiro"** — vanilla deve ser
   lido literal antes de definir estrutura cristalina.
   `lab/typst-original/crates/typst-library/src/visualize/color.rs`
   é fonte canónica.
3. **ADR-0029 §"Simplificações aceites apenas com ADR
   explícita"** — qualquer espaço de cor que NÃO seja
   materializado obriga ADR nova com:
   - Diferença vanilla vs cristalino.
   - Custo semântico da simplificação.
   - Critério de revisão (passo específico, não vaga).
4. **ADR-0033 paridade observable** — `Color::rgb(r,g,b)` deve
   produzir os mesmos bytes PDF que `Color::rgb(r,g,b)` vanilla
   para o mesmo input. Conversões internas podem divergir.
5. **Ordem testes-primeiro** — para cada espaço materializado,
   testes antes de implementação.
6. **`crystalline-lint .`** zero violations no fim do passo.
7. **Tests workspace** sem regressão (contagem ≥ baseline).
8. **Materialization é leitura proibida por iniciativa própria**
   — Claude Code não deve ler `00_nucleo/materialization/`
   excepto com path explícito.

---

## §1 — Sub-passo P257.A: Fase A (diagnóstico vanilla obrigatório)

**Objectivo**: produzir inventário literal de Color vanilla.

**Materialização**: zero código novo. Apenas leitura e
diagnóstico imutável.

### Acções obrigatórias

#### A.1 — Leitura literal do vanilla

```bash
# Estrutura principal Color
view lab/typst-original/crates/typst-library/src/visualize/color.rs

# Espaços de cor — confirmar enumeração
grep -n "^\s*pub enum ColorSpace\|^\s*pub enum Color\|impl Color " \
  lab/typst-original/crates/typst-library/src/visualize/color.rs

# Funções stdlib registadas
grep -rn "fn rgb\|fn luma\|fn cmyk\|fn oklab\|fn oklch\|fn hsl\|fn hsv\|fn linear_rgb\|fn html\|fn hex" \
  lab/typst-original/crates/typst-library/src/visualize/

# Componentes auxiliares (per ADR-0029 §exclusões já listados)
grep -n "pub struct Cmyk\|pub struct WeightedColor\|pub struct RatioComponent" \
  lab/typst-original/crates/typst-library/src/visualize/color.rs

# Conversões entre espaços
grep -n "to_rgb\|to_srgb\|to_oklab\|to_linear" \
  lab/typst-original/crates/typst-library/src/visualize/color.rs
```

#### A.2 — Consumers actuais cristalino (impacto da mudança)

```bash
# Onde Color é usado hoje
grep -rn "use.*Color\|: Color\b\|Color::" 01_core/src/ 03_infra/src/ 02_shell/src/
grep -rn "Value::Color" 01_core/src/

# Stroke / Paint / Style::Fill / FrameItem::Text
grep -n "paint: Color\|fill: Color" 01_core/src/
```

#### A.3 — Exportador PDF — limitações actuais

```bash
# Como o exportador PDF lida com Color hoje
grep -rn "Color::Rgb\|Color::Rgba\|DeviceRGB\|DeviceCMYK\|DeviceGray" \
  03_infra/src/export*
```

**Output esperado**: confirmar que PDF actual só sabe sRGB +
alpha; CMYK requer `DeviceCMYK`; outros espaços (Oklab, HSL,
HSV) precisam de conversão para sRGB antes de export PDF.

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
com a seguinte estrutura (imutável após criação per ADR-0034):

```markdown
# Diagnóstico Color vanilla — Passo 257 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0034 +
ADR-0065 inventariar primeiro.
**Diagnóstico pai**: discussão pré-P257 + ADR-0029 §"Sobre o
código do Passo 25".

---

## §1 — Estrutura literal vanilla

(Colar output literal dos comandos A.1, especialmente:
- Definição completa do enum/struct Color vanilla.
- Lista exhaustiva dos espaços de cor.
- Componentes auxiliares (Cmyk, WeightedColor, etc.).
- Conversões entre espaços disponíveis.)

## §2 — Funções stdlib vanilla relacionadas

(Lista literal de funções: rgb, luma, cmyk, oklab, oklch, hsl,
hsv, linear-rgb, html (hex), etc. — confirmar nomes exactos e
assinaturas.)

## §3 — Estado cristalino actual

`01_core/src/entities/layout_types.rs` Color:
- enum com 2 variantes: Rgb{r,g,b:u8}, Rgba{r,g,b,a:u8}.

Consumers (listar com path:linha):
- (do A.2)

## §4 — Análise de paridade

Tabela: espaço vanilla → materializável cristalino agora?

| Espaço vanilla | Representação | Conversão para sRGB? | Materializar P257? |
|----------------|---------------|----------------------|---------------------|
| sRGB | r,g,b,a:u8 | identidade | ✓ (já existe) |
| Linear RGB | f32 r,g,b,a (linearizado) | gamma 2.2 | ☐ |
| Oklab | L,a,b,alpha:f32 | matriz LMS + linear RGB | ☐ |
| Oklch | L,c,h,alpha:f32 | Oklab + polar | ☐ |
| HSL | h:f32(deg),s,l,a:f32 | algoritmo HSL→RGB | ☐ |
| HSV | h,s,v,a:f32 | algoritmo HSV→RGB | ☐ |
| CMYK | c,m,y,k:f32 | (1-c)(1-k)... | ☐ |
| Luma | y:u8 | r=g=b=y | ✓ (já existe via rgb) |

## §5 — Componentes auxiliares vanilla

(Cmyk struct, WeightedColor, etc. — ADR-0029 §exclusões já
disse que são internos a Color; documentar literal aqui.)

## §6 — Conversões e operadores

(Operadores entre cores: lighten, darken, mix, opacify; per
vanilla — quais materializar?)

## §7 — Impacto exportador PDF

PDF tem 3 espaços nativos: DeviceRGB, DeviceCMYK, DeviceGray.
Outros espaços precisam de conversão para um destes 3 antes de
export.

(Documentar estado actual + plano: converter tudo para sRGB
antes de export OU manter CMYK nativo quando aplicável?)

## §8 — Plano materialização (Fase B)

**Decisão por espaço**:

| Espaço | Decisão | Justificação | ADR-required? |
|--------|---------|--------------|---------------|
| sRGB (Rgb/Rgba) | preservar (refactor para struct) | já em produção | — |
| Linear RGB | materializar | base para conversões | — |
| Oklab | materializar | espaço moderno; útil | — |
| Oklch | materializar | derivado Oklab | — |
| HSL | ☐ materializar / ☐ scope-out | uso comum web | ADR se scope-out |
| HSV | ☐ materializar / ☐ scope-out | uso comum design | ADR se scope-out |
| CMYK | ☐ materializar / ☐ scope-out | print-only | ADR se scope-out |
| Luma | preservar (alias para sRGB cinza) | já em produção | — |

## §9 — Localização do tipo

Ficheiro novo proposto: `01_core/src/entities/color.rs`
(saída de `layout_types.rs`).

Re-export em `01_core/src/entities/mod.rs`.

## §10 — Decisão arquitectural

(Resumir decisões §4-§9; identificar ADR(s) novas necessárias.)
```

### Critério de aceitação P257.A

- Ficheiro
  `diagnostico-color-vanilla-passo-257.md` criado em
  `00_nucleo/diagnosticos/`.
- §1-§10 preenchidos com **conteúdo literal**, não interpretativo.
- Decisão de materializar/scope-out documentada por espaço em §8.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações em prompts L0 ou ADRs (ainda — vem em P257.B).

---

## §2 — Sub-passo P257.B: ADR(s) explícita(s)

**Objectivo**: cumprir ADR-0029 §"Simplificações aceites apenas
com ADR explícita" para qualquer espaço NÃO materializado.

### Acções

#### B.1 — Criar ADR principal de paridade

**ADR-0067** (ou próximo disponível) — "Color paridade vanilla
com subset materializado".

Estrutura per cabeçalho canónico:
- **Status**: `PROPOSTO`.
- **Contexto**: Color em P25 com 2 variantes; ADR-0029
  reservou expansão; P257 executa.
- **Decisão**: enumeração exacta dos espaços materializados em
  P257 + lista de espaços scope-out com justificação por
  espaço (per ADR-0029 §"Simplificações aceites").
- **Análise paridade**: tabela vanilla vs cristalino.
- **Consequências**: positivas/negativas/neutras.
- **Critério de revisão**: para cada scope-out, passo
  específico em que será revisitado (e.g. "P-X CMYK
  materialização quando export print for prioritário").
- **Alternativas consideradas**: materializar tudo /
  materializar nada / subset proposto.

#### B.2 — ADR adicionais por espaço scope-out (se aplicável)

**Apenas se Fase A §8 decidir scope-out de espaços
específicos**. Cada scope-out grande pode merecer ADR dedicada
ou pode ser agregado em ADR-0067 conforme magnitude.

**Decisão de granularidade**: ADRs separadas se cada espaço
exigir justificação substancial diferente; ADR única agregada
se justificações são homogéneas.

#### B.3 — Verificação

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found (ADRs novas não tocam código)
```

### Critério de aceitação P257.B

- ADR-0067 (ou nome final) criada em `00_nucleo/adr/` com
  cabeçalho canónico per README ADRs.
- Status `PROPOSTO`; transição a `IMPLEMENTADO` após P257.D
  completar.
- Cada espaço scope-out justificado.
- README ADRs actualizado (linha nova listando ADR-0067).

---

## §3 — Sub-passo P257.C: Prompt L0 + materialização

**Objectivo**: criar tipo Color novo + adaptar consumers.

### C.1 — Criar prompt L0 `entities/color.md`

**Pré-requisito** Regra de Ouro: prompt antes de código.

Estrutura per padrão entities/* L0 prompts:

- **Módulo**: `01_core/src/entities/color.rs`.
- **Camada**: L1.
- **Contexto**: refactor de `layout_types.rs::Color`; expansão
  de paridade vanilla per P257 + ADR-0067.
- **Tipo principal**: enum/struct conforme Fase A decidiu.
  Recomendação geral: **enum tagged** com uma variante por
  espaço (paridade estrutural directa com vanilla).
- **Métodos públicos**: `to_srgb()`, `to_linear_rgb()`,
  `to_oklab()`, etc. conforme espaços materializados.
- **Operadores**: `PartialEq` exacto per ADR-0028 regra
  preservada (sem tolerância em produção).
- **Critérios de verificação**: por espaço materializado, pelo
  menos 2 testes (criação + conversão para sRGB).
- **Sobre paridade**: referência ADR-0067.

Calcular hash; `--fix-hashes`.

### C.2 — Materializar `01_core/src/entities/color.rs`

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

#### C.2.1 — Testes

Por cada espaço materializado (decidido em Fase A §8):

```rust
// Exemplos esquemáticos — adaptar à decisão Fase A
#[test]
fn srgb_construcao() { ... }
#[test]
fn srgb_partial_eq() { ... }
#[test]
fn oklab_construcao() { ... }
#[test]
fn oklab_to_srgb_branco() { ... } // L=1 → (255,255,255)
#[test]
fn oklab_to_srgb_preto() { ... }  // L=0 → (0,0,0)
#[test]
fn linear_rgb_gamma_22() { ... }
// ... por espaço
```

Executar `cargo test color::` — verificar que falham.

#### C.2.2 — Implementação

`01_core/src/entities/color.rs`:

```rust
// Schema indicativo — Fase A decide forma final
pub enum Color {
    Srgb { r: u8, g: u8, b: u8, a: u8 },
    LinearRgb { r: f32, g: f32, b: f32, a: f32 },
    Oklab { l: f32, a: f32, b: f32, alpha: f32 },
    // ... outros espaços materializados
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self { ... }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { ... }
    pub fn oklab(l: f32, a: f32, b: f32) -> Self { ... }
    // ...

    pub fn to_srgb(&self) -> (u8, u8, u8, u8) { ... }
    // ...
}
```

Executar `cargo test color::` — verificar que passam.

### C.3 — Remover Color de `layout_types.rs`

`01_core/src/entities/layout_types.rs`:
- Eliminar `pub enum Color { Rgb {...}, Rgba {...} }`.
- Manter outros tipos (Page, Frame, FrameItem, etc.).

Re-export em `entities/mod.rs`:
- Remover qualquer re-export de Color de `layout_types`.
- Adicionar `pub mod color;` e re-export apropriado.

### C.4 — Adaptar consumers L1/L3

Per inventário Fase A §3:

```rust
// entities/geometry.rs — Stroke.paint
// entities/style.rs — Style::Fill
// entities/layout_types.rs — FrameItem::Text.fill
// entities/value.rs — Value::Color (se existe)
// rules/stdlib*.rs — native_rgb, native_luma + novas funcs
```

Substituir `Color::Rgb { ... }` ↔ `Color::Srgb { ... }`
(conforme nome final decidido em Fase A).

Adaptar pattern-matches exaustivos em todos os sítios.

### C.5 — Adaptar exportador PDF

`03_infra/src/export*.rs`:

Para espaços materializados que não sejam sRGB nativo:
- Converter para sRGB via `Color::to_srgb()` antes de emitir
  bytes PDF.
- CMYK (se materializado): emitir `/DeviceCMYK` nativo PDF.
- Linear RGB / Oklab / Oklch / HSL / HSV: converter para sRGB.

### C.6 — Stdlib funcs novas

`01_core/src/rules/stdlib*.rs` (ficheiro a confirmar):

- `native_rgb` — preservar comportamento; agora retorna
  `Color::Srgb`.
- `native_luma` — preservar comportamento.
- `native_oklab(l, a, b[, alpha])` — nova.
- `native_oklch(l, c, h[, alpha])` — nova.
- `native_cmyk(...)` — nova se materializado.
- `native_linear_rgb(...)` — nova.
- `native_hsl(...)` — nova se materializado.
- `native_hsv(...)` — nova se materializado.

**Para cada nova func**: testes primeiro (range validation,
output esperado), implementação depois.

### C.7 — Verificação final P257.C

```bash
cargo test --workspace
# Esperado: contagem ≥ baseline + tests novos (~20-40)

cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
```

### Critério de aceitação P257.C

- `01_core/src/entities/color.rs` existe com tipo novo.
- `01_core/src/entities/layout_types.rs` sem Color.
- Todos os consumers adaptados.
- Stdlib funcs novas registadas e testadas.
- Exportador PDF converte cores correctamente.
- Tests workspace +N (esperado +20-40).
- Zero violations linter.
- Paridade observable preservada (PDFs idênticos para inputs
  sRGB; PDFs visualmente equivalentes para inputs em outros
  espaços materializados).

---

## §4 — Sub-passo P257.D: Fecho ADR + relatório

### D.1 — Promover ADR-0067 PROPOSTO → IMPLEMENTADO

`00_nucleo/adr/typst-adr-0067-*.md`:
- Status: `PROPOSTO` → `IMPLEMENTADO`.
- Adicionar linha **Validado**: P257.
- Adicionar secção **Aplicação**: referência a
  `00_nucleo/materialization/typst-passo-257-relatorio.md`.

### D.2 — Actualizar README ADRs

`00_nucleo/adr/README.md`:
- Linha de P257 adicionada com sumário (1 parágrafo).
- Contagem ADRs actualizada (+1 ou +N conforme B.2 decidiu).
- Distribuição actualizada.

### D.3 — DEBT-4 (se aplicável)

Se DEBT-4 ainda referencia cor pendente:
- Actualizar para reflectir o que P257 fechou.
- Manter aberto apenas para itens fora do scope P257.

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-257-relatorio.md`:

- **§1 Sumário executivo** — Fase A confirmada; ADR-0067
  criada e promovida; espaços materializados; espaços
  scope-out; tests delta; hashes propagados.
- **§2 Sub-passo P257.A** — output Fase A resumido.
- **§3 Sub-passo P257.B** — ADR(s) criada(s).
- **§4 Sub-passo P257.C** — código materializado;
  ficheiros tocados.
- **§5 Sub-passo P257.D** — ADR promovida; DEBT actualizado.
- **§6 Padrões metodológicos** — ADR-0029 aplicada
  literalmente (diagnóstico-first + ADR-explícita); subpadrão
  "auditoria condicional" N=3 → 4.
- **§7 Cobertura** — Visualize ganha N pp via Color
  expansion.
- **§8 Limitações e trabalho futuro** — espaços scope-out
  enumerados; passos futuros previstos por scope-out.
- **§9 Referências**.

### Critério de aceitação P257.D

- ADR-0067 IMPLEMENTADO.
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P257

Ao fim do passo, todos os seguintes têm de ser verdadeiros:

- [ ] `cargo run -p crystalline-lint -- .` retorna
  `✓ No violations found`.
- [ ] `cargo test --workspace` retorna contagem ≥ baseline +
  20-40 (sem regressão).
- [ ] `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md`
  existe com §1-§10 preenchidos.
- [ ] ADR-0067 (ou nome final) criada e promovida a
  IMPLEMENTADO.
- [ ] `00_nucleo/prompts/entities/color.md` criado.
- [ ] `01_core/src/entities/color.rs` materializado.
- [ ] `01_core/src/entities/layout_types.rs` sem Color.
- [ ] Todos os consumers adaptados (Stroke, Style, FrameItem,
  Value, stdlib).
- [ ] Exportador PDF converte cores correctamente per
  espaço.
- [ ] Hashes propagados (`crystalline-lint --fix-hashes`).
- [ ] README ADRs actualizado.
- [ ] Relatório do passo criado.
- [ ] Cada espaço materializado tem ≥2 tests.
- [ ] Paridade observable preservada (`native_rgb(255,0,0)`
  produz mesmos bytes PDF antes e depois).

---

## §6 — Sequência operacional condensada

Para Claude Code seguir linearmente:

1. **Ler** `CLAUDE.md`, ADR-0028/0029/0033/0034/0054.
2. **Reportar** estado inicial: tests count + lint baseline.
3. **P257.A** — Fase A diagnóstico vanilla (leitura literal de
   `lab/typst-original/.../visualize/color.rs` + consumers
   cristalino); criar diagnóstico imutável; decidir espaços
   materializar vs scope-out em §8.
4. **P257.B** — Criar ADR-0067 (e adicionais se necessário);
   PROPOSTO. README ADRs actualizado.
5. **P257.C** — Criar L0 prompt `entities/color.md`;
   `--fix-hashes`; testes primeiro; implementação; adaptar
   consumers; adaptar exportador PDF; adaptar stdlib; verificar
   tests verdes + lint limpo.
6. **P257.D** — Promover ADR-0067 a IMPLEMENTADO; actualizar
   README ADRs; actualizar DEBT-4 se aplicável; criar relatório.
7. **Verificação final** — todo o checklist §5 satisfeito.
8. **Reportar** ao utilizador: espaços materializados, espaços
   scope-out, tests delta, ficheiros criados/editados.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- Fase A revela que vanilla actual diverge significativamente
  da descrição "8 espaços" em ADR-0028 (e.g. vanilla expandiu
  para 10+ espaços ou consolidou para menos).
- Fase A revela que `Cmyk`/`WeightedColor`/`GradientStop`/etc.
  têm uso que excede "sub-componentes internos" da ADR-0029
  §exclusões.
- Materialização exige nova crate externa (e.g. `palette` ou
  `color-space`) — exigiria ADR de autorização adicional
  análoga a ADR-0019/0023/0024.
- Exportador PDF precisa de refactor estrutural além de
  "converter cor para sRGB antes de emitir" (e.g. embedding
  de ICC profiles).
- Conversões entre espaços têm erros numéricos significativos
  que afectam `PartialEq` exacto (ADR-0028 manteve regra "sem
  tolerância em produção"; conflict).
- Decisão de granularidade ADR (B.2): uma ADR agregada ou
  múltiplas dedicadas?
- `crystalline-lint` reporta violations não-triviais.

Em qualquer paragem, registar contexto no relatório parcial e
aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com Visualize

Color é tipo **Visualize**, não Model. Este passo executa
**antes** de P256 (diagnóstico Model) porque tu pediste para
não esquecer. Pós-P257, Visualize ganha cobertura inicial
significativa (Color é a entrada com maior peso em Visualize).

P256 (Model) pode prosseguir em paralelo ou após.

### Relação com `Gradient`, `Paint`, `Stroke<T>`, `Tiling`

ADR-0029 §enumeração lista estes tipos como vanilla mas
**fora do scope P257**:
- **Gradient** (Linear/Radial/Conic) — depende de Color
  materializado; passo dedicado pós-P257.
- **Paint** (enum Color | Gradient | Tiling) — wrapper que só
  faz sentido pós-Gradient + Tiling.
- **Stroke<T>** — actual usa `Stroke { paint: Color }`; após
  P257 pode usar `Stroke<Paint>` (refino futuro).
- **Tiling** — passo dedicado autónomo.

Estes ficam **registados** como roadmap pós-P257 (não DEBTs
novos per política "sem novas reservas").

### Relação com cor em diagnósticos (ADR-0048)

ADR-0048 cobre **cores ANSI em terminal** para diagnósticos
(`error:` vermelho, etc.). Domínio completamente disjunto de
P257 (que é cor tipográfica do conteúdo). Ambos coexistem.

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0028 — REVOGADA; contexto histórico.
- ADR-0029 — EM VIGOR; obriga diagnóstico vanilla +
  ADR explícita.
- ADR-0033 — paridade observable.
- ADR-0034 — diagnóstico canónico.
- ADR-0054 — perfil graded.
- ADR-0065 — inventariar primeiro.
- DEBT-4 (se aplicável) — original origem de Color simplificado.
- `lab/typst-original/crates/typst-library/src/visualize/color.rs`
  — fonte canónica.
- `01_core/src/entities/layout_types.rs` — Color actual.
- P192A, P255 — precedentes "auditoria condicional".
- P156C-G, P157A-C — precedentes "materialização granular
  com testes primeiro + L0 antes de código".
