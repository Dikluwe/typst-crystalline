# ⚖️ ADR-0078: Column flow algorithm — `Region/Regions` abstraction + multi-column consumer

**Status**: **PROPOSTO** (P215 2026-05-12; transita IMPLEMENTADO
em P221 quando sub-fase (a)+(b) materializadas + DEBT-56 fechado).
**Data**: 2026-05-12 (PROPOSTO P215).
**Sub-passos planeados**: P215 (PROPOSTO; este P215 diagnóstico
prévio); **P216A-B** (sub-fase a — refactor estrutural Layouter
em 2 sub-passos per P215.div-1); **P217** (`Content::Columns`
variant); **P218** (`native_columns` stdlib); **P219** (sub-fase
b — consumer multi-column); **P220** (`Content::Colbreak` +
`native_colbreak`); **P221** (encerramento Fase 3 + transição
ACEITE/IMPLEMENTADO).
**Diagnóstico prévio**:
- `00_nucleo/materialization/typst-passo-215-relatorio.md` (P215).

---

## Contexto

ADR-0061 PROPOSTO (Layout Fase X roadmap) fixou em P156B
caminho 1 graded — Fase 1 (sub-passos C-F: pad/hide/h/v/skew),
Fase 2 (sub-passos G-I: block/box/stack), Fase 3 sub-passo 1
(P156J: repeat). **Pós-P156L (refino sides individualizadas
em pad)**, ADR-0061 está em **50% concluído** caminho 1:
- ✅ Fase 1 fechada (4 sub-passos).
- ✅ Fase 2 fechada (3 sub-passos; atinge target 72% Layout).
- ✅ Fase 3 sub-passo 1 fechado (repeat).
- ⏸ Fase 3 sub-passo 2 (refino pad) fechado P156L.
- ❌ **Fase 3 columns + colbreak pendentes** — DEBT-56
  EM ABERTO desde P156B (estimado L+ ~5-8h).

P214 confirmou Layout em 78% via sincronização §2.1 ↔ Tabela A
(P156B-L já reflectidos em footnotes). 4 entradas restantes
em §A.5 separam Layout de 100%:

1. `columns(n)` — ausente (Fase 3 DEBT-56).
2. `colbreak()` — ausente (depende columns; DEBT-56).
3. `place(...)` — parcial (refino column scope).
4. `measure(body)` — parcial (Bloco C ADR-0066).

Adicionalmente 2 entradas em §A.6 Model dependem de DEBT-56:
5. `grid` header/footer real (P157 diferiu).
6. `TableHeader.repeat` algoritmo (P157C diferiu).

DEBT-56 é o maior refactor estrutural pendente. Layouter
actual é **single-page write-target** com cursors escalares
globais (`cursor_x`/`cursor_y`/`current_items`/`current_line`/
`page_config`); 135 call-sites empíricos em
`01_core/src/rules/layout/mod.rs` (P215 C1 inventário).

ADR-0078 endereça especificamente a **abstracção arquitectural**
necessária — `Region/Regions` — que desbloqueia 4 das 6
entradas restantes (columns, colbreak, grid header/footer,
TableHeader.repeat) e deixa 2 isoladas (place refino + measure
stdlib).

---

## Decisão

**Refactor `Layouter` em 2 sub-fases distintas**, com sub-fase
(a) decomposta em 2 sub-passos per P215.div-1 (>100 call-sites
empíricos):

### Sub-fase (a) — Region/Regions abstraction

**Objectivo**: introduzir `Region` / `Regions` como tipos L1
preservando comportamento single-column. Comportamento
observable inalterado (todos os tests workspace continuam a
passar).

**Decomposição P215.div-1** (135 call-sites > 100 limiar):

- **P216A**: introduzir `Region` type + substituir
  `cursor_x`/`cursor_y`/`current_items`/`current_line`/
  `page_config` em `Layouter` por `Region` activa. ~80
  call-sites refactor; comportamento single-column preservado
  via `Regions::new_single(width, height)`.
- **P216B**: introduzir `Regions` (Vec<Region>) wrapper +
  helper `Layouter::with_regions`; iteração sequencial de
  regions (ainda só 1 region por padrão). ~30-40 call-sites
  refactor; preparação para sub-fase (b).

**Tipo proposto** (paridade vanilla `typst-layout/src/regions.rs`):

```rust
pub struct Region {
    pub size:  Size,             // largura × altura disponível
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub items: Vec<FrameItem>,   // acumulador da region
}

pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,    // regions futuras (multi-column)
    pub last:    Option<Region>, // region final (overflow)
}
```

### Sub-fase (b) — Consumer multi-column

**Objectivo**: materializar `Content::Columns { count, gutter,
body }` arm + iteração sequencial de N regions com
`width / count - gutter`.

**Decomposição** (per P215 C4 roadmap):

- **P217**: variant enum `Content::Columns { count, gutter,
  body }` + arms exhaustivos.
- **P218**: stdlib `native_columns(count, gutter: ?)` +
  scope register.
- **P219**: consumer Layouter — converter Content::Columns
  arm em Regions multi-column; iterar; flush per region.
- **P220**: `Content::Colbreak { weak: bool }` + stdlib
  `native_colbreak(weak: ?)` + tests mixing pagebreak.

### Decisão arquitectural — sub-fases reduzem risco

Sub-fase (a) é refactor sem mudança observable; tests
existentes funcionam como regression suite. Sub-fase (b) é
aditivo (novo arm `Content::Columns`); não modifica
single-column behavior.

Total decomposição: **6 sub-passos núcleo (P216A+B+217+218+
219+220) + P221 encerramento**. Magnitude cumulativa **L
(~6-9h)** vs DEBT-56 estimado L+ original (~5-8h L+
incluindo todas as 6 entradas dependentes).

---

## Não-objectivos

- **Balanceamento altura entre colunas** — vanilla typst
  também não faz balanceamento; sequência sequential
  preenche região 1, transborda para região 2, etc.
- **Column span** (rowspan-like) — não existe no vanilla;
  fora escopo.
- **Cross-page column flow rules** — específicas (e.g.
  "última coluna deve ter altura ≥ 50%") — fora escopo
  Fase 3 inicial.
- **Refino de `place`** float/clearance — sub-passo opcional
  P223 paralelo; não bloqueia sub-fases (a)+(b).
- **`measure(body)` stdlib expose** — sub-passo opcional
  P222 paralelo; isolado de DEBT-56 per ADR-0066 Bloco C.
- **`grid` header/footer real + `TableHeader.repeat`
  algoritmo** — sub-passo opcional P224 que requer sub-fases
  (a)+(b) completas; fora escopo desta ADR (cobre apenas
  abstracção Region + columns/colbreak; grid extension é
  consumer adicional).

---

## Plano de materialização

ADR-0078 transita PROPOSTO → IMPLEMENTADO quando todas as 6
condições forem verdadeiras (verificadas em P221):

1. **Sub-fase (a) materializada** (P216A+B): `Region`/`Regions`
   tipos L1 introduzidos; 135 call-sites refactor sem
   regressão; tests workspace verdes pré e pós refactor.
2. **`Content::Columns { count, gutter, body }` variant**
   (P217): enum cresce 56→57 variants; arms exhaustivos
   passam crystalline-lint.
3. **`native_columns` stdlib registada** (P218): stdlib
   funcs cresce 53→54; scope register em `eval/mod.rs`;
   `extract_count` helper validando count≥1.
4. **Consumer multi-column funcional** (P219): tests
   `columns(2)`, `columns(3)`, `columns(4)` produzem
   regions correctas; `width / count - gutter` aplicado;
   tests mixing com pagebreak válidos.
5. **`Content::Colbreak { weak: bool }` + `native_colbreak`**
   (P220): variant 57→58; stdlib 54→55; tests
   `colbreak()` + `colbreak(weak: true)` válidos; mixing
   pagebreak/colbreak honest.
6. **`crystalline-lint .` 0 violations** preservadas em
   todos os sub-passos.

ADR transita REJEITADO se:
- Sub-fase (a) introduz regressão observable em tests
  existentes (improvável — refactor sem mudança comportamental).
- Sub-fase (b) `Region/Regions` impossível com Frame/Page
  pattern actual (improvável — paridade vanilla
  `typst-layout/src/regions.rs`).
- Custo agregado L+ → XL+ por complexidade não-prevista
  (improvável — decomposição P215.div-1 mitiga).

Se ADR rejeitada, DEBT-56 mantém-se aberto em formato
diferente; sub-passo dedicado pós-rejeição reavalia
abordagem.

### P216A materializado 2026-05-12

Sub-fase (a) parte 1 fechada:

- Tipo `Region` introduzido em `01_core/src/entities/region.rs`
  com 7 fields (`cursor_x: Pt`, `cursor_y: Pt`, `line_start_x: Pt`,
  `current_items: Vec<FrameItem>`, `current_line: Vec<FrameItem>`,
  `width: f64`, `height: f64`) + 3 métodos (`new`, `reset`,
  `has_pending`) + 4 sentinelas P216A.
- L0 `00_nucleo/prompts/entities/region.md` criado;
  hash propagado via `crystalline-lint --fix-hashes`.
- `Layouter` struct refactored em
  `01_core/src/rules/layout/mod.rs`: 5 fields escalares
  + 2 dimensões (via `page_config.width/height`)
  agregados em field único `region: Region`.
- Inventário empírico real: ~167 call-sites refactored
  mecânicamente em 6 ficheiros: `mod.rs` (~136 sub),
  `cursor.rs` (22), `equation.rs` (8), `grid.rs` (13),
  `placement.rs` (12), `tests.rs` (2).
- **Caminho B1 fixado**: `PageConfig.width/height` preservados;
  `region.width/height` é cópia derivada em `Layouter::new`.
  Sincronização adicionada em `Content::SetPage` arm
  (`mod.rs:761-763`) — region.width/height actualizados
  quando page_config muda. Único ajuste manual além da
  substituição mecânica `sed`.
- **Opção α fixada** (sem helpers): acesso directo
  `self.region.X` em todos os call-sites; sem
  `cursor_x()`/`set_cursor_x()` etc. Anti-inflação aplicada
  10ª vez cumulativa pós-P205D.
- 0 mudança observable: 1939 → **1943 verdes** (+4 P216A
  region sentinelas; nenhum test pre-existente regrediu).
- 0 violations preservadas.
- Sem `P216A.div-N` registadas (substituição mecânica
  + 1 ajuste sincronização SetPage = único ponto).

ADR-0078 mantém-se PROPOSTO. Próximo sub-passo: **P216B**
(`Regions` wrapper + `Layouter::with_regions` helper —
sub-fase (a) parte 2 per P215.div-1).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Region/Regions com sub-fase decomposta (escolhida)** | Risco mitigado por sub-fases; tests existentes como regression suite; paridade vanilla `regions.rs` | Custo M+ na sub-fase (a); 2 sub-passos vs 1 |
| Big-bang refactor (sub-fase a+b num passo único) | 1 sub-passo; menos overhead | Risco alto — 135 call-sites refactor + nova feature simultaneamente; difícil debug se regressão |
| `Content::Columns` sem refactor Region (Layouter mantém scalar cursors) | Sub-fase (a) evitada | `Content::Columns` consumer impossível arquitecturalmente — single-page cursor não suporta iteração regions |
| Adiar DEBT-56 pós-Layout 100% por outras vias | Baixa entrada de risco curto-prazo | Layout fica em 78% indefinidamente — 4 entradas restantes em cascata; gap não fechável sem column flow |

**Escolha**: Region/Regions com decomposição P215.div-1 —
balança risco e progresso incremental. Pattern paridade
ADR-0061 sub-fases graded.

---

## Reservas

P216-P224 (sub-passos núcleo + opcionais) **NÃO são reservas**
per política P158 ("sem novas reservas"). São **opções
identificadas** documentadas em P215 §5 + roadmap C4. Decisão
humana sobre prosseguir fica em aberto pós-P215.

ADR-0078 PROPOSTO autoriza **abordagem arquitectural**
(Region/Regions); não compromete cronograma. Análoga a
ADR-0061 PROPOSTO em P156B que autorizou caminho 1 graded
sem comprometer quando materializar cada Fase.

---

## Análise paridade vanilla

Vanilla typst tem `typst-layout/src/regions.rs` com tipos
`Region` + `Regions` análogos. Diferenças cristalino vs
vanilla:

| Aspecto | Vanilla | Cristalino (P215 proposto) |
|---------|---------|----------------------------|
| `Region` | `pub struct Region { size: Size, expand: Axes<bool>, full: Abs }` | `pub struct Region { size: Size, cursor_x: f64, cursor_y: f64, items: Vec<FrameItem> }` |
| `Regions` | `pub struct Regions<'a> { size: Size, full: Abs, expand: Axes<bool>, backlog: &'a [Abs], last: Option<Abs>, root: bool }` | `pub struct Regions { current: Region, backlog: Vec<Region>, last: Option<Region> }` |
| Lifetime | `'a` (borrow `&'a [Abs]` backlog) | sem lifetime (owned `Vec<Region>`) |
| `expand` axes | sim (controla auto-expand) | adiar (cristalino single-pass; auto-expand controlado por flush_*) |
| `root` flag | sim (page-level vs nested) | adiar (cristalino infere via Region::new_single vs new_columns) |
| Balanceamento | não | não (paridade) |

**Divergência justificada** (per ADR-0054 graded perfil):
cristalino simplifica `Region`/`Regions` via owned Vec sem
lifetime; auto-expand controlado por flush helpers; sem
`root` flag explícito. Cobertura essencial preservada;
refinos vanilla (lifetime + axes + root) adiados se
consumer real necessitar.

---

## Referências

- **ADR-0061** PROPOSTO (Layout Fase X roadmap) — fixou
  caminho 1 graded em P156B; este ADR-0078 endereça
  especificamente Fase 3 sub-passos columns+colbreak.
- **ADR-0066** PROPOSTO (Introspection runtime) — Bloco C
  `measure(body)` stdlib expose isolado de DEBT-56.
- **ADR-0054** graded — perfil scope-out autorizado;
  refinos vanilla (lifetime, axes, root) adiados sob
  política graded.
- **DEBT-56** EM ABERTO (column flow L+) — endereçado
  por sub-fases (a)+(b) deste ADR.
- **DEBT-37** FECHADO P84.6 (`PlaceScope::Parent`) —
  refino column scope é trabalho novo (P223 opcional;
  fora ADR-0078).
- **P156B** diagnóstico Layout amplo — precedente
  metodológico para diagnóstico-primeiro arquitectural.
- **P215 relatório** — diagnóstico prévio + 135
  call-sites empíricos + P215.div-1 decomposição.
- **Vanilla `typst-layout/src/regions.rs`** — paridade
  arquitectural com simplificações cristalino documentadas.

---

## Histórico

| Data | Status | Notas |
|------|--------|-------|
| 2026-05-12 | PROPOSTO | Criada em P215 para autorizar abordagem Region/Regions abstraction. Diagnóstico prévio confirmou 135 call-sites empíricos > 100 limiar; P215.div-1 decompõe sub-fase (a) em P216A+B. Transição IMPLEMENTADO em P221 quando 6 condições satisfeitas. |
