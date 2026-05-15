# Spec do passo P246 — Cell layout migration `cell_available_h` + `cell_origin_x/y/w` → consumo via `regions.current.*` (refactor consumer Layouter; activa A.4 breakable per-cell scope-out P235 graded)

**Data**: 2026-05-14.
**Tipo**: refactor consumer Layouter — migração 4 fields
transientes per-célula (`cell_available_h`, `cell_origin_x`,
`cell_origin_y`, `cell_origin_w`) para consumo via campo
estrutural `regions.current` introduzido em P216A/B e estendido
em P243. Decisão 7 do P243 diferida materializada agora.
**Magnitude planeada**: **M (~2-4h)** — paridade ADR-0081 §"Escopo"
referenciada como continuação natural pós-M9d. Audit C1 do P245
deixou empíricamente confirmado que os 4 fields são declarados
em `mod.rs:151-160` (e inicializados a `None` em `mod.rs:271-274`)
sem usos em `mod.rs` próprio — usos estão noutros sub-módulos
do `01_core/src/rules/layout/` (`grid.rs`, `placement.rs`,
`cursor.rs`, ou similar — mapeamento empírico §2).
**Marco**: **continuação materialização pós-M9d completo**;
**primeira aplicação Layouter refactor sem migração funcional**
(não activa feature nova; consolida arquitectura); **fechamento
estrutural da dívida implícita Decisão 7 P243**; activa
**A.4 breakable per-cell** scope-out histórico graded P235 +
preserva DEBT-34e aberto (algoritmo placement completo).

---

## §1 O que será feito

P246 migra 4 fields transientes do Layouter (`cell_available_h`,
`cell_origin_x`, `cell_origin_y`, `cell_origin_w`) para serem
**consultados via `self.regions.current.*`** durante consumer
das células Grid/Table (e qualquer outro arm que abra uma
sub-região por-célula). Trabalho é refactor consumer puro
— **zero alteração L1 entities; zero variant Content novo;
zero ADR nova**.

### Origem dos 4 fields (histórico)

- **`cell_available_h: Option<f64>`** — P83 (DEBT-34c
  ENCERRADO). Altura disponível na célula activa; consumido
  por `resolve_alignment` para items dentro de células
  (VAlign::Bottom ancora ao limite inferior; VAlign::Horizon
  centra verticalmente).
- **`cell_origin_x: Option<f64>`** + **`cell_origin_y:
  Option<f64>`** + **`cell_origin_w: Option<f64>`** — P84.6
  (DEBT-37 ENCERRADO). Coordenadas + largura da célula activa;
  consumido por `Content::Place` arm para anchoring scope
  Column dentro de Grid.

### Hipótese arquitectural pré-P246 (a confirmar empíricamente em §2)

- Fields declarados em `mod.rs:151-160` + inicializados a `None`
  em `mod.rs:271-274` (confirmado P245 audit).
- Usos estão noutros sub-módulos do `01_core/src/rules/layout/`
  (`grid.rs`, `placement.rs`, `cursor.rs`, ou similar — naming
  exacto a determinar).
- **Save/restore por célula** existe no arm Grid (`Content::Grid`)
  e provavelmente em arm Table (`Content::Table`, que delega a
  `layout_grid` per P157A).
- **Reads** estão em arms que consomem o contexto célula:
  `Content::Place` (P84.6), `resolve_alignment` (P83), e
  possivelmente outros (per arm Block + arm Boxed + arm Pad
  se a hipótese P156G/H/L for que esses arms também respeitam
  célula).

### Hipótese material pós-P246 (objectivo)

`Regions.current` (estendido P216A+P216B+P243) tem fields:

```rust
// Hipótese pré-P246 — confirmar empíricamente em §2:
pub struct Region {
    pub width:  f64,
    pub height: f64,
    // Possivelmente outros — confirmar
}

pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,  // P243
    pub last:    Option<Region>,  // P243
}
```

Para suportar consultas células, **uma de duas opções
arquitecturais será fixada na Decisão 1**:

**Opção A — Push-Pop stack de Region no Regions**

Adicionar `regions.push(cell_region)` + `regions.pop()` à
entrada/saída de cada célula. Cell region é uma `Region`
adicional na pilha; `regions.current` aponta sempre ao topo
da pilha. Após pop, volta à region da página.

Vantagem: paridade conceptual com vanilla `Regions::push/pop`
+ semantic clara "região activa".
Custo: refactor `Regions` struct (adiciona pilha; field
`current` substituído por método `current()` que retorna topo).

**Opção B — Snapshot save/restore com cell_region field separado**

Manter `regions.current` como page region (não muda); adicionar
field `regions.cell: Option<Region>` que armazena cell region
quando dentro de célula. Reader resolve: `let r =
self.regions.cell.as_ref().unwrap_or(&self.regions.current);`
para consumir region efectiva.

Vantagem: refactor minimal; preserva `regions.current` como
page-level absoluto.
Custo: bifurcação semantic ("current" significa página, "cell"
significa célula).

**Decisão preliminar (spec)**: **Opção B** parece mais
conservadora — minimiza refactor + preserva semantic de
`regions.current` como page-level introduzida P216A. Mas a
Decisão 1 final fica fixada em §3 após audit C1 §2 empírico
revelar a estrutura real de uso.

### Activação A.4 breakable per-cell

Pós-P246, `Content::Block.breakable` + `Content::Boxed.height`
+ `Content::TableCell` (sem breakable explícito mas com altura
herdada) podem consumir `regions.cell.height` (Opção B) ou
`regions.current.height` (Opção A) para decisão real de quebra
**dentro da célula**. **P246 não materializa esta activação**
— é refactor preparatório. Activação real fica para passo
futuro (escopo M S por feature; não-reservado per política
P158).

### Tests esperados

Tests P246 novos: **0-3** (range M magnitude refactor não-feature).

- Refactor preserva comportamento — tests baseline P83+P84.6+
  P156G+P156H+P157A+P157B preservados literal.
- Eventualmente 1-3 tests novos validando que `regions.cell`
  (Opção B) é populado/limpo correctamente em arm Grid;
  esses tests são internal-only não user-facing.

**Workspace pós-P246**: **2203 → ~2203-2206 verdes**
(range +0-3 paridade refactor não-feature).

### Adaptações pre-existentes

Estimativa **N=0-5** adaptações tests pré-existentes. Cenários:

- Tests P83 + P84.6 que verificam comportamento E2E via output
  PDF/Frame são preservados literal (output não muda).
- Tests que **introspectam fields internos do Layouter** via
  `pub(super)` exposure (improvável; o módulo é encapsulado)
  podem precisar de adaptação para usar nova API
  `regions.cell()` ou similar.

**Se audit C1 detectar tests baseline que dependem directamente
dos 4 fields cell_*** (`cell_available_h` etc) com acesso
public/pub(super), criar `P246.div-N` formal antes de adaptar.

---

## §2 Verificação empírica pré-P246 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=8 → 9 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=8 P245 ("grep fields/arms já implementados antes de assumir
trabalho original") expande para **N=9 cumulativo**: "mapear
empíricamente distribuição de usos por sub-módulo antes de
fixar arquitectura de migração".

### §2.1 Inventário declarações pré-P246 (já confirmado P245)

`grep -n "cell_available_h\|cell_origin_x\|cell_origin_y\|cell_origin_w" 01_core/src/rules/layout/mod.rs`:

```
151:    pub(super) cell_available_h: Option<f64>,
155:    /// Quando todos `Some` em conjunto com `cell_available_h`,
158:    pub(super) cell_origin_x: Option<f64>,
159:    pub(super) cell_origin_y: Option<f64>,
160:    pub(super) cell_origin_w: Option<f64>,
271:            cell_available_h:        None,
272:            cell_origin_x:           None,
273:            cell_origin_y:           None,
274:            cell_origin_w:           None,
```

**Conclusão parcial**: 4 fields declarados; inicializados a
`None`; sem usos no `mod.rs` (só declarações + init).

### §2.2 Mapeamento usos por sub-módulo (BLOQUEANTE)

Comando recomendado:

```bash
grep -rn "cell_available_h\|cell_origin_x\|cell_origin_y\|cell_origin_w" \
  01_core/src/rules/layout/
```

Resultado esperado: lista completa com ficheiro + linha + contexto.

**Categorias a identificar empíricamente**:

1. **Save/restore points** (provavelmente em arm Grid + arm
   TableCell): pares `let saved = self.cell_*` + `self.cell_*
   = saved`. Cada par marca entrada/saída duma célula.
2. **Write points** (atribuições `self.cell_* = Some(...)` ou
   `self.cell_* = None`): dentro de save/restore blocks.
3. **Read points** (consumo `if let Some(h) = self.cell_*`,
   `self.cell_*.unwrap_or(...)`, etc): em arms que respeitam
   contexto célula (`Place`, `resolve_alignment`, possivelmente
   `Block`, `Boxed`, `Pad`).

### §2.3 Inventário `regions.current` actual

```bash
grep -rn "regions\.current\|regions\.cell\|regions\.push\|regions\.pop" \
  01_core/src/rules/layout/
```

Identifica:
- Quantos consumers já usam `regions.current.*` (estado pós-P243).
- Se existe API `push`/`pop` (Opção A já materializada?) ou
  apenas `current` campo directo (Opção B viável).
- Se já existe `regions.cell` ou similar (refactor parcial
  pré-existente?).

### §2.4 Inventário Region struct actual

```bash
grep -n "pub struct Region\b\|pub struct Regions\b" \
  01_core/src/entities/region.rs
```

Confirma fields actuais da struct `Region`. P216A declarou minimal
(`width`, `height`); P243 estendeu `Regions` (`backlog`, `last`)
mas pode não ter tocado `Region` interno.

### §2.5 Save/restore pattern em arm Grid

```bash
grep -B2 -A5 "Content::Grid\|layout_grid" 01_core/src/rules/layout/*.rs | head -80
```

Localiza onde o save/restore por linha+coluna acontece.
**Hipótese**: arm `Content::Grid` em ficheiro dedicado
(provavelmente `grid.rs`) com loop por célula que:
1. Calcula `cell_origin_x`, `cell_origin_y`, `cell_origin_w`,
   `cell_available_h` para a célula actual.
2. Armazena em `self.cell_*` (write).
3. Chama `layout_content(cell.body)` que consome via reads.
4. Restaura `self.cell_* = None` (ou valor anterior se
   aninhado Grid-in-Grid).

### §2.6 Sub-store DTO P83+P84.6 — tests E2E baseline

```bash
grep -rn "cell_available_h\|cell_origin" 01_core/tests/ 03_infra/tests/ 2>/dev/null
```

Tests que dependem directamente dos fields (improvável; tests
normalmente verificam comportamento E2E via output PDF/Frame).

### §2.7 Tests pré-P246 baseline

```bash
cargo test --workspace
```

Esperado: **2203 verdes** (estado pós-P245).

### §2.8 Decisão arquitectural pós-audit

Após §2.2 + §2.3 + §2.4 + §2.5 completos, fixar empíricamente
a Decisão 1 (Opção A vs B). Critérios:

- Se §2.3 já tem `regions.push`/`pop` API → **Opção A já
  materializada parcial** → P246 estende esse caminho.
- Se §2.4 mostra `Region` sem fields adicionais para suporte
  cell (apenas `width`/`height`) → **Opção B** simplifica
  (adiciona `cell: Option<Region>` à `Regions`).
- Se §2.2 revela ≤10 usos cumulativos → migração trivial
  ambas as opções; Opção B preferível (menos refactor).
- Se §2.2 revela >20 usos cumulativos → migração massiva;
  considerar **Opção C** (preservar fields legacy mas adicionar
  API `regions.cell()` em paralelo; deprecação gradual em
  passos futuros).

### `P246.div-N` antecipadas — possíveis

- **`P246.div-1`** se §2.3 revelar que `regions.cell` ou
  similar **já existe** (refactor parcial pré-existente não
  documentado).
- **`P246.div-2`** se §2.4 revelar que `Region` struct já
  tem fields cell-specific (`cell_origin_*` ou similar
  embutidos).
- **`P246.div-3`** se §2.6 revelar dependência directa de
  tests aos fields legacy → reescopo para Opção C (preservar
  fields + adicionar API nova).
- **`P246.div-4`** se §2.7 baseline ≠ 2203 verdes → reconciliação
  prévia.

---

## §3 Decisões fixadas P246 — 8 decisões

### Decisão 1 — Arquitectura migração: PRELIMINAR Opção B (snapshot via `regions.cell: Option<Region>`); FINAL fixada pós-audit C1 §2.8

**Opção B preliminar**:

```rust
// 01_core/src/entities/region.rs (extensão minimal):
pub struct Regions {
    pub current: Region,           // page region (P216A; P246 preservado literal)
    pub backlog: Vec<Region>,      // P243
    pub last:    Option<Region>,   // P243
    pub cell:    Option<Region>,   // P246 — cell region transient
}

impl Regions {
    /// Region efectiva: célula se activa, senão página.
    pub fn effective(&self) -> &Region {
        self.cell.as_ref().unwrap_or(&self.current)
    }

    /// Entra célula com region.
    pub fn enter_cell(&mut self, cell: Region) -> Option<Region> {
        std::mem::replace(&mut self.cell, Some(cell))
    }

    /// Sai célula restaurando saved.
    pub fn exit_cell(&mut self, saved: Option<Region>) {
        self.cell = saved;
    }
}
```

**Layouter API substitui 4 fields**:

- `self.cell_available_h` → `self.regions.cell.as_ref().map(|r| r.height)`.
- `self.cell_origin_w` → `self.regions.cell.as_ref().map(|r| r.width)`.
- `self.cell_origin_x` + `self.cell_origin_y` → **PRESERVADOS**
  no Layouter como `cell_origin_x/y` (paralelos a cell region;
  necessários porque `Region` actual não contém origem absoluta).

**Justificação `cell_origin_x/y` preservados**: `Region`
introduzido P216A é geometria abstracta (width + height) sem
coordenadas absolutas; cell origin necessita de coordenadas
em pt absolutas na página. **Decisão preliminar acomoda**
limitação estrutural; Decisão final §3.8 fica em aberto se
Region for estendido com `origin: Point` em passo futuro.

### Decisão 2 — `Content::Grid` arm + `Content::TableCell` arm refactor save/restore

Arm que entra célula:

```rust
// Antes:
let saved_h = self.cell_available_h;
let saved_y = self.cell_origin_y;
let saved_w = self.cell_origin_w;
self.cell_available_h = Some(cell_h);
self.cell_origin_y    = Some(row_y);
self.cell_origin_w    = Some(col_w);
// ... layout body ...
self.cell_available_h = saved_h;
self.cell_origin_y    = saved_y;
self.cell_origin_w    = saved_w;

// Depois:
let saved_cell = self.regions.enter_cell(Region {
    width: col_w, height: cell_h,
});
let saved_y = std::mem::replace(&mut self.cell_origin_y, Some(row_y));
// ... layout body ...
self.regions.exit_cell(saved_cell);
self.cell_origin_y = saved_y;
```

**Pattern**: 2 chamadas API substitui 6 atribuições directas;
reduz risco de bug "esquecer restaurar".

### Decisão 3 — Consumer reads: `Place` arm + `resolve_alignment`

`Content::Place` arm (P84.6):

```rust
// Antes:
let in_cell = self.cell_origin_x.is_some()
           && self.cell_origin_y.is_some()
           && self.cell_origin_w.is_some()
           && self.cell_available_h.is_some();

// Depois:
let in_cell = self.cell_origin_y.is_some()
           && self.regions.cell.is_some();
```

`resolve_alignment` (P83):

```rust
// Antes:
let available_h = self.cell_available_h.unwrap_or(page_remaining_h);

// Depois:
let available_h = self.regions.effective().height;
// (cell se activa; page caso contrário — equivalente semantic)
```

### Decisão 4 — Activação A.4 breakable per-cell — DIFERIDA

P246 é refactor consumer puro. **Não activa** semantic real de
`Block.breakable` ou `Boxed.height` ou `TableCell` overflow
dentro de células. Activação fica para passo futuro
**não-reservado** per política P158.

Documentar em ADR-0079 §"Categoria A.4" + ADR-0061 §"Refino
futuro": breakable per-cell agora arquiteturalmente desbloqueado;
materialização pendente.

### Decisão 5 — DEBT-34c + DEBT-37 sentinelas preservadas

DEBT-34c (P83 alinhamento vertical células) + DEBT-37 (P84.6
Place scope Parent + float: true) **preservados ENCERRADO**.
P246 refactor não reabre DEBTs encerradas — semantic preservada
literal via wrapper API.

### Decisão 6 — `Region` struct intocada (preservação P216A literal)

Nem `origin: Point` adicionado em `Region`, nem outros fields
cell-specific. P246 estende apenas `Regions` (adiciona field
`cell: Option<Region>`). Justificação: minimizar acoplamento;
`Region` permanece geometria abstracta width+height.

### Decisão 7 — Anti-inflação 38ª aplicação cumulativa

- Opção β L0 minimal (paridade P243+P245 Layouter internal
  refactor). Hash L0 `entities/region.rs` actualizado apenas
  se Region struct receber field (não acontece em P246; só
  `Regions` recebe `cell`).
- Opção α extensão Regions mínima (1 field + 3 métodos
  helper).
- Opção α API substitui atribuições directas (redução de
  superfície).
- Opção α preservação `cell_origin_x/y` como Layouter fields
  legacy (transição não-completa; debt latente para refactor
  futuro de Region com origem).
- Opção α anotação cumulativa minimal em ADR-0079 +
  ADR-0080.

### Decisão 8 — Padrão emergente "Layouter consumer migration via API wrapper" N=1 inaugurado

P246 inaugura sub-padrão **N=1**: "Migração field-by-field
Layouter → API wrapper struct entity-side". Reduz acoplamento
entre Layouter privado e contexto activo (cell/page/region).
Candidato a formalização N=3-4 se outras migrações ocorrerem
(hipóteses futuras: `floats_pending` P245 → API `regions.floats()`
quando floats forem extendidos a per-region; `cursor_y_top_reserve`
+ `cursor_y_bottom_reserve` P245 → API `regions.reserves()`
quando reserves forem extendidos).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 entity | `01_core/src/entities/region.rs` | `Regions { cell: Option<Region> }` field adicionado; métodos `effective`, `enter_cell`, `exit_cell` |
| L0 prompt | `00_nucleo/prompts/entities/region.md` | Documentar field novo + 3 métodos; hash propagado |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Remover declarações `cell_available_h` + `cell_origin_w` (fields 1+4 migrados); preservar `cell_origin_x` + `cell_origin_y` (Decisão 1) |
| L1 Layouter | `01_core/src/rules/layout/grid.rs` (ou similar; confirmar §2) | Substituir save/restore patterns nos arms Grid + TableCell |
| L1 Layouter | `01_core/src/rules/layout/placement.rs` (ou similar) | Substituir reads em `Content::Place` arm |
| L1 Layouter | `01_core/src/rules/layout/cursor.rs` (ou similar) | Substituir reads em `resolve_alignment` |
| Tests Layouter | onde aplicável | 0-3 tests novos validando `regions.cell` populado/limpo |
| Tests existentes | conforme `P246.div-N` | Adaptações se necessárias (esperado N=0-5) |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | Sem alteração quantitativa (refactor não-feature); footnote ⁶⁴ P246 documentando refactor cell-region migration |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | §"Categoria A.4" anotada: A.4 breakable per-cell arquiteturalmente desbloqueado P246 |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | Sub-categoria "Layouter consumer migration via API wrapper" N=1 inaugurada; lição refinada N=8 → 9 anotada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotada: breakable per-cell desbloqueado P246 |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-34c + DEBT-37 anotação cumulativa "P246 refactor preserva semantic; sentinelas mantidas"; **sem reabertura**. **DEBT novo opcional**: "Region sem field `origin: Point`; `cell_origin_x/y` ainda preservados como Layouter fields legacy" — abertura **opcional**; decisão humana §3 |
| Relatório P246 | `00_nucleo/materialization/typst-passo-246-relatorio.md` | Estrutura canónica refactor M magnitude |

---

## §5 Critério aceitação P246 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2203 → ~2203-2206 verdes** (+0-3 paridade M refactor) |
| `crystalline-lint .` | **0 violations preservado** |
| `crystalline-lint --fix-hashes` | depende — se `entities/region.rs` modificado → hash propagado L0 `region.md`; **paridade Opção α** (hash limited L0 update) |
| Content variants | **62 preservado** (zero alterações entities Content) |
| ShapeKind variants | **5 preservado** |
| Layouter fields | **-2** (`cell_available_h` + `cell_origin_w` removidos); **+0** (preservados `cell_origin_x` + `cell_origin_y`) → net **-2 fields** |
| Regions fields | **3 → 4** (`current` + `backlog` + `last` + **`cell`**) |
| Regions methods | **+3** (`effective`, `enter_cell`, `exit_cell`) |
| Stdlib funcs | **64 preservado** |
| §A.5 distribuição | preservada literal (refactor não-feature; sem reclassificação) |
| Cobertura Layout per metodologia | **~93-94% preservado** (refactor qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| ADR-0079 Categoria A.4 | anotação "arquiteturalmente desbloqueado P246" |
| ADR-0080 sub-categorias | "Layouter consumer migration via API wrapper" N=1 inaugurada |
| ADR-0061 §"Refino futuro" | anotação "breakable per-cell desbloqueado P246" |
| DEBT-34c | **ENCERRADO preservado** (sentinela P83 + anotação P246) |
| DEBT-37 | **ENCERRADO preservado** (sentinela P84.6 + P223 + anotação P246) |
| DEBT novo `Region origin` | **opcional** — decisão humana §"Notas operacionais §7.5" |
| L0 hashes propagados | **0 ou 1** (`entities/region.md` se Region struct receber field — não acontece P246; só Regions estendido) |
| Adaptações pre-existentes | **N=0-5** estimadas; `P246.div-N` se >5 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Layouter consumer migration via API wrapper" N=1 inaugurado; "Spec C1 audit obrigatório bloqueante" N=8 → 9 cumulativo |

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" preservadas pós-IMPLEMENTADO
total):

1. **Tests baseline preservados**: 2203 verdes pré-P246 →
   ~2203-2206 pós-P246 (+0-3 internal; 0 regressões; N=0-5
   adaptações documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P246 toca Layouter consumer apenas; `Introspector` trait
   intocada; sub-stores trackable F3 intocados.
3. **Backward compat**: API pública `Region`/`Regions`
   preserva-se (P216A semantic); novos métodos `effective`/
   `enter_cell`/`exit_cell` são additive; consumer Layouter
   preserva comportamento E2E literal (tests baseline E2E
   pré-P83/P84.6 inalterados).

**Promoções ADR esperadas**:

- ADR-0079 §"Categoria A.4" anotada "breakable per-cell
  arquiteturalmente desbloqueado P246".
- ADR-0080 sub-categoria "Layouter consumer migration via
  API wrapper" N=1 inaugurada.
- ADR-0061 §"Refino futuro" anotada.
- **Sem nova ADR criada**.

---

## §6 Próximo sub-passo pós-P246

P246 fecha refactor cell-region migration. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 breakable per-cell activação real** | Materializar semantic `Block.breakable` + `Boxed.height` + `TableCell` overflow dentro célula | M (~2-4h) | **alta** — primeira oportunidade pós-P246 desbloqueio |
| Refino A.4 — outset/fill/stroke Block+Boxed | 3 de 4 scope-outs restantes pós-P242 | S-M por attr | média |
| ADR-0079 → IMPLEMENTADO | Promoção Fase 5 Layout completa (decisão humana scope-out C.2) | XS-S | alta se humano decide fechamento |
| ADR meta admin XS | Formalizar "passo administrativo XS" N=6 (limiar sólido pós-P244) | XS | média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| DEBT novo `Region origin` | Refactor Region com `origin: Point` permitindo eliminar `cell_origin_x/y` Layouter | M | baixa — débito latente |

**Recomendação subjectiva pós-P246**: **A.4 breakable per-cell
activação real**. Materializa o desbloqueio arquitectural que
P246 instala; magnitude M paridade P246. Sequente natural.

**Decisão humana fica em aberto literal** pós-P246.

**Estado esperado pós-P246**:
- Tests workspace: **~2203-2206 verdes** (+0-3 internal).
- Content variants: **62 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: **-2** (cell_available_h + cell_origin_w
  removidos); **+0** (cell_origin_x + cell_origin_y preservados).
- Regions fields: **3 → 4** (+cell).
- Regions methods: **+3** (effective, enter_cell, exit_cell).
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: preservada literal.
- Cobertura Layout per metodologia: **~93-94% preservado**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição**: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  23 preservado; total **68 preservado**.
- **Saldo DEBTs: 11 preservado** (DEBT novo opcional não-aberto
  per política P158).
- **38 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P246**:
  - "Layouter consumer migration via API wrapper" N=1 inaugurado.
  - "Spec C1 audit obrigatório bloqueante" N=8 → **9 cumulativo**.
  - "Layouter internal refactor (semantic activation)" N=2
    preservado (P246 não-semantic; é estrutural).
- **Categoria A Fase 5 Layout**: 5/5 + parcial A.4 P242 +
  **arquiteturalmente desbloqueada P246**.
- **Categoria B Fase 5 Layout**: 3/3 preservado.
- **Categoria C.1 Fase 5 Layout**: cumprida P245.
- **Categoria C.2 Fase 5 Layout**: pendente; cell layout migration
  P246 é pré-requisito desbloqueador (não cumpre C.2 mas reduz
  risco arquitectural).
- **Categoria D Fase 5 Layout**: 3/? preservado.
- **Fase 5 Layout candidata**: 15/13-15 → **15/13-15** (P246
  refactor não-feature).
- **Marco interno**: cell layout migration completa; 4 fields
  Layouter reduzidos a 2 + API Regions; A.4 breakable per-cell
  arquiteturalmente desbloqueado; padrão "Layouter consumer
  migration via API wrapper" inaugurado N=1; lição C1 audit
  N=9 cumulativo refinada.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.7 completos. **Lição N=9 cumulativa**:
   primeira aplicação onde audit C1 expande de "grep fields/arms
   já implementados" (P245) para "mapear empíricamente
   distribuição de usos por sub-módulo antes de fixar
   arquitectura de migração". Se §2.3 revelar API
   `regions.push`/`pop` pré-existente, criar `P246.div-1`.
   Se §2.4 revelar Region com fields cell-specific
   pré-existentes, criar `P246.div-2`.

2. **Decisão 1 final fixada pós-audit §2.8**. Opção B
   preliminar pode reverter para Opção A se §2.3 revelar API
   push/pop pré-existente, ou para Opção C se §2.2 revelar
   >20 usos (preservação dual). Documentar transparentemente
   em relatório P246 §"Decisão 1 final".

3. **Refactor preserva semantic literal**. Tests E2E baseline
   P83 (DEBT-34c) + P84.6 (DEBT-37) + P156G/H (Block/Boxed)
   + P157A/B (Table/TableCell) **devem passar inalterados**.
   Se algum falhar, é sinal de refactor incompleto → não
   adaptar test → corrigir refactor.

4. **`cell_origin_x` + `cell_origin_y` preservados como
   Layouter fields legacy**. Decisão consciente Decisão 6:
   `Region` actual sem `origin: Point`; cell origin absoluto
   em pt na página exige fields paralelos. Quando `Region`
   for estendido (DEBT novo opcional §7.5), `cell_origin_x/y`
   podem ser eliminados em passo futuro.

5. **DEBT novo opcional**: abrir `DEBT-XX — Region sem field
   origin: Point; cell_origin_x/y preservados como Layouter
   fields legacy P246` em DEBT.md? **Decisão humana**:
   - Opção α (abrir): formaliza débito latente; magnitude M
     para refactor futuro.
   - Opção β (não-abrir): preserva política P158 "sem novas
     reservas"; débito implícito documentado em ADR-0080
     anotação P246.
   - Opção γ (ADR meta XS): formalizar pattern "campos legacy
     paralelos a abstracções modernas" em ADR meta dedicada.

   **Recomendação subjectiva**: Opção β (não-abrir; preserva
   política P158).

6. **Custo real esperado**: ~2-4h (paridade M magnitude). Maior
   parcela: refactor pattern save/restore em arm Grid (~50%);
   audit C1 + decisões + anotações ADR (~30%); tests +
   relatório (~20%).

7. **Sem `P246.div-N` antecipado para Decisão 1**. Cenários
   `P246.div-1`/`P246.div-2`/`P246.div-3`/`P246.div-4`
   detalhados em §2.8 são contingências reais; se algum
   activar, parar audit empírico antes de prosseguir.

8. **Anti-inflação 38ª aplicação cumulativa** pós-P205D
   preservar: Opção α extensão Regions minimal (1 field + 3
   métodos) + Opção α API substitui atribuições directas +
   Opção α preservação fields legacy (transição não-completa
   consciente) + Opção β L0 minimal (Region intocada; só
   Regions estendido — hash `region.md` propagado se
   formalmente documentado) + Opção α anotação cumulativa
   minimal ADRs + Opção α sub-padrão novo inaugurado anotado
   (sem ADR meta prematura).
