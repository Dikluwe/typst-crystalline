# Passo 216A — Sub-fase (a) parte 1: `Region` type + refactor cursor/items/line/page_config

**Série**: 216 (primeiro sub-passo materialização Layout
Fase 3; parte A de sub-fase (a) decomposta per P215.div-1).
**Marco**: nenhum (quarto passo pós-M9c; primeira
materialização real de DEBT-56).
**Tipo**: refactor estrutural cross-modular **sem mudança
observable** — preserva todos os tests workspace como
regression suite.
**Magnitude**: M+ (~3-5h).
**Pré-condição**: P215 concluído (diagnóstico Fase 3 +
ADR-0078 PROPOSTO + roadmap P216A-P224); ADR-0078 §"Plano
materialização" sub-fase (a) parte 1 documentada;
inventário empírico 135 call-sites (102 cursor/items/line +
33 page_config) confirmado em P215 C1; tests 1939 verdes;
0 violations; humano fixou Caminho 1 (prosseguir P216A
imediatamente).
**Output**: 1 ficheiro relatório curto + código alterado +
1 ficheiro L0 novo + ADR-0078 anotada (sem transição de
status).

---

## §1 Trabalho

P215 fixou que DEBT-56 sub-fase (a) tem 135 call-sites no
Layouter; > 100 limiar do spec acionou P215.div-1
decompondo em P216A+P216B. P216A é parte 1 — introduzir
tipo `Region` em L1 (`entities/region.rs`) que **agrupa**
os 4 cursores escalares (`cursor_x`, `cursor_y`,
`current_items`, `current_line`) + parte da `page_config`
(width/height) num único struct semanticamente coerente.
Refactorar `Layouter` para usar `Region` em vez dos
escalares dispersos.

**Decisão central de P216A**: refactor **strutural** sem
mudança observable. Layouter ainda single-region pós-P216A
— `Region` é apenas reorganização. Sub-fase (a) parte 2
(P216B) adiciona `Regions` wrapper; sub-fase (b) (P219)
consumer multi-column real.

**Critério de aceitação rigoroso**: **todos os 1939 tests
workspace devem manter-se verdes** após P216A. Zero
mudança observable cristalino. Testes existentes funcionam
como regression suite de altíssima cobertura.

**Decisão arquitectural rejeitada**: introduzir `Region` +
`Regions` num único sub-passo (P216 monolítico hipotético).
Rejeitada per P215.div-1 — 135 call-sites > 100 limiar; risco
de erro humano em refactor mecânico amplo justifica
decomposição.

Reuso de dados (sem recolha nova):

- P215 C1 inventário Layouter (135 call-sites mapeados).
- ADR-0078 PROPOSTO §"Decisão" tipo `Region` + `Regions`
  simplificados vs vanilla.
- `LayouterRuntimeState` (P190C) como precedente
  arquitectural — struct dedicada para state Layouter-
  runtime (P216A introduz análogo `Region` para state
  geométrico).
- ADR-0029 (pureza física L1) — `Region` é L1 puro.
- ADR-0036 (atomização progressiva) — P216A é primeira
  atomização do refactor multi-region.

---

## §2 Cláusulas (10)

### C1 — Inventário pré-refactor: confirmar call-sites

Antes de tocar em código, confirmar empíricamente:

```
grep -c "self\.\(cursor_x\|cursor_y\|current_items\|current_line\)" \
  01_core/src/rules/layout/mod.rs
grep -c "self\.page_config\.\(width\|height\)" \
  01_core/src/rules/layout/mod.rs
grep -c "self\.\(cursor_x\|cursor_y\|current_items\|current_line\|page_config\)" \
  01_core/src/rules/layout/*.rs
```

Critério:
- Contagens dentro de ±10% de P215 inventário (102 + 33
  = 135 esperado em mod.rs; total `*.rs` maior).
- Se desvio > 10%: registar `P216A.div-1` e reajustar
  scope (e.g. se 200 call-sites detectados em todos os
  `*.rs`, sub-passo precisa de mais granularidade).

### C2 — Criar tipo `Region` em L1

Criar `01_core/src/entities/region.rs`:

```rust
//! Region — abstracção para área de layout single-column.
//!
//! Introduzida em P216A (DEBT-56 sub-fase a parte 1) para
//! agrupar state geométrico previamente disperso no Layouter:
//! cursor (x/y), buffers (items/line), dimensões (width/height).
//!
//! Single-region por design em P216A — `Regions` (Vec<Region>)
//! introduzido em P216B; consumer multi-column em P219.
//!
//! Paridade vanilla simplificada per ADR-0078 PROPOSTO:
//! - Sem `expand` axes (cristalino não tem auto-expand explícito).
//! - Sem `full` flag (cristalino infere via cursor_y vs height).
//! - Owned (não borrowed) — vanilla `Regions<'a>` borrow inválido
//!   no contexto cristalino single-pass.

use crate::entities::frame_item::FrameItem;

/// Área de layout single-column.
///
/// **Estado geométrico** previamente disperso em `Layouter`:
/// - Posição corrente do cursor.
/// - Buffer de items pendentes (linha + acumulados).
/// - Dimensões fixas da region.
///
/// P216A apenas reorganiza; P216B introduz `Regions` wrapper;
/// P219 consumer multi-column.
#[derive(Debug, Clone)]
pub struct Region {
    /// Posição horizontal corrente do cursor (Pt).
    pub cursor_x: f64,
    /// Posição vertical corrente do cursor (Pt).
    pub cursor_y: f64,
    /// Início horizontal da linha actual (Pt). Após `flush_line`,
    /// `cursor_x` reseta para este valor.
    pub line_start_x: f64,

    /// Itens já flushed para a region (espera flush_page).
    pub current_items: Vec<FrameItem>,
    /// Itens pendentes na linha actual (esperam flush_line).
    pub current_line: Vec<FrameItem>,

    /// Largura disponível da region (Pt).
    pub width: f64,
    /// Altura disponível da region (Pt).
    pub height: f64,
}

impl Region {
    /// Cria region nova com cursor zerado, sem items pendentes.
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            cursor_x: 0.0,
            cursor_y: 0.0,
            line_start_x: 0.0,
            current_items: Vec::new(),
            current_line: Vec::new(),
            width,
            height,
        }
    }

    /// Reseta cursor + buffers para nova page (mantém width/height).
    pub fn reset(&mut self) {
        self.cursor_x = self.line_start_x;
        self.cursor_y = 0.0;
        self.current_items.clear();
        self.current_line.clear();
    }

    /// True se há items pendentes em qualquer buffer.
    pub fn has_pending(&self) -> bool {
        !self.current_items.is_empty() || !self.current_line.is_empty()
    }
}
```

Re-export em `01_core/src/entities/mod.rs`:

```rust
pub mod region;
pub use region::Region;
```

Sentinelas mínimas (4 tests unitários em `region.rs::tests`):
- `region_new_inicia_cursor_zero`.
- `region_reset_preserva_dimensoes`.
- `region_has_pending_false_apos_new`.
- `region_clone_funciona`.

Magnitude: XS isolada (~15min).

### C3 — Criar L0 prompt `entities/region.md`

Criar `00_nucleo/prompts/entities/region.md` (paridade
estrutural com `layouter_runtime_state.md` + `sealed-positions.md`):

- Módulo: `01_core/src/entities/region.rs`.
- Histórico: 2026-05-12 (P216A criação — DEBT-56 sub-fase
  (a) parte 1).
- Propósito: abstracção single-region; pre-condição
  `Regions` (P216B) + consumer multi-column (P219).
- Campos: 7 fields documentados (cursor_x/y/line_start_x +
  current_items/line + width/height).
- Pattern arquitectural: "Layouter-state agregado em struct
  dedicada" — N=2 (precedente `LayouterRuntimeState` P190C).
- Consumers planeados: P216B (Regions wrapper); P219
  (consumer multi-column).
- Pureza L1: zero I/O; só owns `Vec<FrameItem>` + escalares.
- Sentinelas: `p216a_region_struct_existe`.

Hash propagado via `crystalline-lint --fix-hashes`.

### C4 — Refactor `Layouter` struct

Editar `01_core/src/rules/layout/mod.rs`:

**Antes (fields dispersos)**:

```rust
pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer> {
    cursor_x: f64,
    cursor_y: f64,
    line_start_x: f64,
    current_items: Vec<FrameItem>,
    current_line: Vec<FrameItem>,
    page_config: PageConfig,
    // ... outros fields (introspector, locator, runtime, etc.) ...
}
```

**Depois (agregação em Region)**:

```rust
pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer> {
    /// Region única (P216A). `Regions` wrapper introduzido em P216B.
    region: Region,

    /// Config da página (margens, etc.). `width`/`height` movidos
    /// para `region`; restantes fields preservados.
    page_config: PageConfig,

    // ... outros fields preservados inalterados ...
}
```

**`PageConfig` ajuste**: dois caminhos possíveis em C2.B:
- **Caminho B1** (literal — preferido): manter `PageConfig`
  inalterado; `region.width/height` é cópia derivada de
  `page_config.width/height` em `Layouter::new`. Redundância
  controlada; reduz blast radius.
- **Caminho B2** (radical): mover `width`/`height` para
  `region` exclusivo; remover de `PageConfig`. Mais limpo
  estructuralmente mas força refactor em `page_config`
  consumers.

**Decisão fixada em C4.B**: **Caminho B1**. Justificação:
P216A é primeira atomização; minimizar blast radius. P216B
pode migrar para B2 se útil (não obrigatório).

`PageConfig` em P216A: inalterado em assinatura. Só leitura
em `Layouter::new` para inicializar `region`.

### C5 — Substituição mecânica de call-sites

Refactor mecânico literal:

| Antes | Depois |
|-------|--------|
| `self.cursor_x` | `self.region.cursor_x` |
| `self.cursor_y` | `self.region.cursor_y` |
| `self.line_start_x` | `self.region.line_start_x` |
| `self.current_items` | `self.region.current_items` |
| `self.current_line` | `self.region.current_line` |
| `self.page_config.width` | `self.region.width` |
| `self.page_config.height` | `self.region.height` |

Substituição em todos os `01_core/src/rules/layout/*.rs`:

- `mod.rs` (~135 esperados per P215).
- `counters.rs` (TBD em C1).
- `references.rs` (TBD em C1).
- `outline.rs` (TBD em C1).
- `hyphenation.rs` (TBD em C1).
- `cursor.rs` (TBD em C1).
- `equation.rs` (TBD em C1).

Ferramenta: `sed` ou refactor IDE. **Verificação manual obrigatória**
após cada ficheiro — refactor mecânico pode quebrar:
- Pattern matching em expressões compostas
  (e.g. `let (a, b) = (self.cursor_x, self.cursor_y)`).
- Borrows simultâneos
  (e.g. `self.cursor_x = self.cursor_x + self.current_items.len()`
  pode quebrar se `region` for borrowed múltiplas vezes).
- Closures que capturam fields individuais.

Hipótese provável: ~5-15 sítios precisam de ajuste manual
adicional além da substituição literal.

### C6 — Helpers `Layouter` para acesso conveniente

Adicionar métodos privados em `impl Layouter` para acesso
limpo (evita verbosidade `self.region.X` em ~135 sítios):

```rust
impl<M: FontMetrics, S: ImageSizer> Layouter<M, S> {
    // Já existe `self.region` directo; helpers para uso comum:

    #[inline]
    fn cursor_x(&self) -> f64 { self.region.cursor_x }
    #[inline]
    fn cursor_y(&self) -> f64 { self.region.cursor_y }

    #[inline]
    fn set_cursor_x(&mut self, x: f64) { self.region.cursor_x = x; }
    #[inline]
    fn set_cursor_y(&mut self, y: f64) { self.region.cursor_y = y; }
}
```

**Decisão sobre helpers em C6**:
- **Opção α** — sem helpers; tudo via `self.region.X`.
  Verboso mas explícito.
- **Opção β** — helpers para reads frequentes
  (`cursor_x()`, `cursor_y()`).
- **Opção γ** — helpers full set (reads + writes).

Hipótese provável: **Opção α** — sem helpers em P216A.
Verbosidade aceitável; refactor mecânico é mais simples;
helpers podem ser introduzidos em P216B+ se necessário.
Anti-inflação aplicada (per série M9c precedente — 9
aplicações cumulativas).

### C7 — Verificação tests workspace

Critério **rígido**: 1939 tests verdes pós-P216A.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: zero. Qualquer test red indica regressão
observable — incompatível com critério de aceitação P216A
(refactor sem mudança observable).

Se algum test falhar:
- Identificar quebra empírica.
- Decidir: rollback parcial OU ajuste cirúrgico se causa for
  óbvia (e.g. borrow simultâneo).
- Documentar em `P216A.div-N` se ajuste cirúrgico ocorrer.

Hipótese provável: 1939 tests verdes. Refactor mecânico bem
executado preserva comportamento por construção.

### C8 — Verificação lint

```
crystalline-lint .
```

Critério: 0 violations. ADR-0029 (pureza L1) preservada —
`Region` é struct puro L1, sem I/O. `entities/region.md`
hash propagado.

### C9 — ADR-0078 anotação

Editar `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
adicionando anotação cumulativa em §"Plano de materialização":

```markdown
### P216A materializado 2026-05-12

Sub-fase (a) parte 1 fechada:
- Tipo `Region` introduzido em `entities/region.rs` (7 fields).
- Layouter struct refactored para usar `region: Region` único.
- ~135 call-sites refactored mecânicamente
  (`self.cursor_x` → `self.region.cursor_x`, etc.).
- 0 mudança observable (1939 tests preservados).
- L0 `entities/region.md` criado; hash propagado.

ADR-0078 mantém-se PROPOSTO. Próximo sub-passo: P216B
(`Regions` wrapper + `Layouter::with_regions` helper).
```

**Status**: PROPOSTO mantido. Não promove a IMPLEMENTADO
— condição é encerramento Fase 3 (P221).

### C10 — Verificação final e relatório

```
cargo test --workspace
crystalline-lint .
crystalline-lint --fix-hashes .
git diff --stat 01_core/src/
wc -l 01_core/src/entities/region.rs
```

Critério:
- Tests 1939 verdes.
- 0 violations.
- Hashes propagados.
- `region.rs` ~50-100 LOC (estimativa).
- Diff stats mostram principal alteração em
  `01_core/src/rules/layout/mod.rs`.

Relatório P216A documenta tudo em §1-§8.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-216A-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Confirmação inventário call-sites (C1 resultados).
- §3 `Region` tipo criado (campos + sentinelas).
- §4 Layouter refactor (struct antes/depois;
  call-sites contagem real).
- §5 Decisões substantivas (Caminho B1 vs B2; Opção α
  vs β/γ helpers; anti-inflação aplicado).
- §6 Resultados verificação (tests + lint).
- §7 ADR-0078 anotação (cross-reference).
- §8 Próximo passo (P216B sub-fase (a) parte 2 —
  `Regions` wrapper).

Código alterado:
- **Novo**: `01_core/src/entities/region.rs` (~50-100 LOC).
- **Novo**: `00_nucleo/prompts/entities/region.md` (~80
  linhas).
- **Editado**: `01_core/src/entities/mod.rs` (+ 2 linhas
  re-export).
- **Editado**: `01_core/src/rules/layout/mod.rs` (~135
  substituições).
- **Editado**: `01_core/src/rules/layout/*.rs` (substituições
  per C5).
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P216A em §"Plano de materialização").

---

## §4 Não-objectivos

- Materializar `Regions` (Vec<Region>) wrapper — diferido
  a P216B (sub-fase (a) parte 2).
- Adicionar `Content::Columns` variant — diferido a P217.
- Adicionar `native_columns` stdlib — diferido a P218.
- Consumer multi-column no Layouter — diferido a P219.
- `Content::Colbreak` — diferido a P220.
- Mover `width`/`height` de `PageConfig` para `Region`
  exclusivamente — Caminho B2 rejeitado; ficar com B1.
- Adicionar helpers `cursor_x()/set_cursor_x()` etc. —
  Opção β/γ rejeitada; ficar com α (acesso directo via
  `self.region.X`).
- Tocar em código fora de `01_core/src/rules/layout/*.rs`
  e `01_core/src/entities/`.
- Promover ADR-0078 PROPOSTO → IMPLEMENTADO — só após
  P221.
- Fechar DEBT-56 — só após P221.
- Mudar comportamento observable (qualquer mudança em
  output PDF, contagens, etc.).

---

## §5 Riscos a evitar

1. **Borrow checker quebras**: refactor de fields
   individuais para field agregado pode quebrar padrões
   como `self.cursor_x = self.current_items.len() as f64`
   onde Rust agora vê dois borrows mutáveis a `self.region`
   simultaneamente. Mitigação: copy-to-local antes de
   mutar (`let n = self.region.current_items.len(); self.region.cursor_x = n as f64;`).
2. **Pattern matching quebrado**: expressões `(self.cursor_x,
   self.cursor_y) = (a, b)` ou `let Layouter { cursor_x,
   .. } = self` quebram. Mitigação: refactor manual desses
   sítios; estimar ~5-15 sítios.
3. **Closures que capturam fields**: `|y| self.cursor_y +=
   y` continua funcionando; `|y| { let x = self.cursor_x;
   self.cursor_y = x + y; }` pode quebrar. Mitigação:
   identificar caso-a-caso.
4. **Mudança observable acidental**: introduzir `Region`
   com semantic ligeiramente diferente (e.g. `reset()` que
   não preserva `line_start_x`) causa regressão. Mitigação:
   `Region::new` + `reset` espelham comportamento actual
   literal; tests existentes detectam qualquer divergência.
5. **Tests workspace red pós-P216A**: critério de aceitação
   rígido — se algum test red, refactor está incorrecto.
   Mitigação: rollback parcial possível; substituição
   ficheiro-a-ficheiro com `cargo test` entre cada para
   localizar regressão.
6. **Helper inflation**: tentação de adicionar `cursor_x()`,
   `set_cursor_x()`, etc. helpers em P216A. Rejeitada per
   Opção α (anti-inflação). Refactor mecânico é mais simples
   sem helpers; helpers podem vir em P216B se necessário.
7. **Move width/height de PageConfig**: tentação de fazer
   refactor mais limpo (Caminho B2) em P216A. Rejeitada per
   "minimizar blast radius primeiro sub-passo".
8. **Mudar comportamento de `flush_line`/`flush_page`**:
   estes helpers acedem cursor/items/line. Refactor deve ser
   mecânico (`self.cursor_x` → `self.region.cursor_x`); semantic
   preservada. Não tentar "melhorar" lógica.
9. **`PageConfig::new` mudança**: P216A NÃO toca a assinatura
   de `PageConfig::new`. `Layouter::new` lê
   `page_config.width/height` e copia para `region.width/height`.
   Redundância controlada.
10. **Inventário desviado de P215**: se contagem empírica
    em C1 diverge > 10% de P215 inventário (135 call-sites),
    registar `P216A.div-1`. Não prosseguir sem entender o
    desvio.

---

## §6 Hipótese provável

C1 confirmará 135 ± 10% call-sites (provável 130-145).

C2 criará `region.rs` ~80 LOC com 7 fields e 4 sentinelas;
isolado em ~15min.

C3 criará L0 prompt ~80 linhas; hash propagado.

C4 fixará Caminho B1 (PageConfig.width/height preservados;
region duplica); minimiza blast radius.

C5 refactor mecânico ~135 sítios em mod.rs + ~20-50 em
outros ficheiros layout/*.rs.

C6 fixará Opção α (sem helpers; acesso directo
`self.region.X`).

C7 reportará 1939 tests verdes. Possível 1-2 ajustes
manuais em sítios com borrow simultâneo (estimativa).

C8 reportará 0 violations.

C9 anotará ADR-0078 cumulativo.

C10 relatório documental.

Custo real: M+ (~3-5h). Maior parcela em C5 (refactor
mecânico + verificação) + C7 (debug de eventuais quebras
borrow checker).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P216A

P216A é estruturalmente distinto na trajectória pós-M9c e
em toda a série Layout:

- **Primeira materialização real pós-M9c** — pós-P213/P214
  (administrativos) + P215 (diagnóstico). P216A toca
  código.
- **Primeiro refactor estrutural sem mudança observable**
  do projecto — séries P156C-L foram aditivas (novas
  features); P216A é reorganização interna pura.
- **Tests existentes como regression suite** — pattern
  novo. 1939 tests funcionam como bateria de aceitação
  rigorosa; qualquer red indica refactor incorrecto.
- **Pattern emergente "decomposição empírica de magnitude"
  N=1 → 2** — P215.div-1 introduziu pattern; P216A
  confirma utilidade ao executar sub-fase (a) parte 1
  isolada de parte 2 (P216B).
- **Pattern arquitectural "Layouter-state agregado em
  struct dedicada" N=1 → 2** — precedente
  `LayouterRuntimeState` P190C; `Region` é segundo struct
  dedicado.
- **Anti-inflação aplicada N=10** — Opção α (sem helpers)
  + Caminho B1 (PageConfig preservado) = 10ª aplicação
  cumulativa do anti-inflação pós-P205D.

Por isso §5 risco 1 (borrow checker) é o mais provável.
Refactor mecânico de fields individuais para field
agregado tem precedente conhecido em Rust de causar
quebras borrow checker em sítios onde dois fields antigos
eram mutados simultaneamente. Mitigação documentada em
§5 risco 1.

**Critério de aceitação rígido — 1939 tests verdes**:
distintivo de P216A face a outros sub-passos
materialização. Refactor sem mudança observable não tem
"tests novos" como evidência — apenas "tests existentes
preservados". Cobertura é máxima porque o universo de
teste foi acumulado ao longo de 215 passos.
