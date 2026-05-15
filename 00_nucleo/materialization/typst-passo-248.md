# Spec do passo P248 — A.4 breakable + Boxed.height overflow + TableCell overflow activação real (agregado L; 3 promoções graded → real cumulativas: Block.breakable P156G + Boxed.height P156H + TableCell overflow P157B)

**Data**: 2026-05-14.
**Tipo**: refactor consumer Layouter — activação semantic real
de 3 fields/comportamentos graded armazenados em passos prévios
(P156G `breakable`; P156H `height` overflow; P157B
`TableCell.body` overflow). Trabalho material concentrado em
3 arms Layouter (Block + Boxed + GridCell/TableCell) +
mecanismo de medição antecipada + lógica de page break/cell
break decisions.
**Magnitude planeada**: **L (~5-8h)** — paridade ADR-0081-style
agregação L+. Maior parcela: lógica algorítmica (medir + decidir
+ emit) cross-3-arms; tests cross-multiplicados (≥25 cenários
combinatorial); audit C1 detalhado.
**Marco**: continuação materialização pós-P247 atributos
visuais; **inaugura sub-padrão "Agregar promoções graded →
real semantic activação multi-consumer"** N=1 (P247 inaugurou
"agregar promoções cosméticos visuais" N=1 — relacionado mas
distinto: P247 = cosméticos visuais ortogonais aditivos; P248 =
semantic real multi-consumer com interacção via mecanismo
comum medição antecipada); **promoção graded → real** N=2
(P245 Place float real N=1; **P248 N=2 cumulativo**); décima
primeira aplicação cumulativa pattern "spec C1 audit
obrigatório bloqueante pós-P236.div-1" N=10 → 11 cumulativo;
**activa DEBT-34c relação cumulativa** (DEBT-34c fechado P83
via `cell_available_h`; P246 migrou para `regions.cell.height`;
P248 usa para overflow detection).

---

## §1 O que será feito

### §1.1 Estado pré-P248 (factual; confirmado audit empírico 2026-05-14)

**Block.breakable**:
- Field `breakable: bool` (default `true`) em `Content::Block`
  desde P156G.
- **Layouter ignora literal**: `mod.rs:1330` + `mod.rs:1864` →
  `Content::Block { ..., breakable: _, ... }` (underscore).
- Tests pré-P248 não exercitam `breakable: false` semantic.

**Boxed.height**:
- Field `height: Option<Length>` em `Content::Boxed` desde
  P156H.
- Layouter "armazenado mas semantic real adiada" per L0 prompt
  `entities/content.md`.
- `clip: true` (P242) materializa clipping geométrico mas não
  combina com `height` overflow real.

**TableCell.body overflow**:
- Field `body: Box<Content>` em `Content::TableCell` desde
  P157B.
- Layouter renderiza body in-place na célula (paridade P157B
  `cell_available_h` P83 / `regions.cell.height` P246) sem
  detectar overflow vertical.
- DEBT-34e em aberto (colspan/rowspan placement) — **distinto
  de overflow Y** que este passo trata.

**Mecanismo page break actual** (`cursor.rs:126`):
```rust
if self.regions.current.cursor_y.0 > self.regions.current.height
                                     - self.page_config.margin {
    self.new_page();
}
```
Disparado em 9 sítios distintos (flush_line + Grid×3 + Place×1
+ mod.rs×5). **Mecanismo simples: overflow Y por flush_line +
overflow Y antecipado em arms específicos**.

**`regions.cell.height`**: P246 migrou; consumido por
`placement.rs:55` + `placement.rs:101` para Place anchoring.
**Não consumido para overflow detection**.

**Helper `measure_content_constrained`**: confirmado existir
em arm Block (Layouter; permite medição de altura sem emit
real). **Reutilizável para todas as 3 activações**.

### §1.2 Trabalho a fazer P248

**Activação A — Block.breakable:**
1. Layouter arm Block: substituir `breakable: _` por leitura
   real `breakable: *breakable`.
2. Se `breakable == false` E altura body excede espaço
   disponível na página actual MAS body cabe na página inteira:
   - `new_page()` antes de emitir.
   - Re-medir + emit normal na nova página.
3. Se body excede página inteira: emit normal (paridade
   vanilla "overlong atómico quebra mesmo com breakable false";
   confirmar empíricamente em `lab/typst-original/` durante C2).
4. Se `breakable == true`: comportamento P156G preservado
   literal (body emit + page break natural via flush_line).

**Activação B — Boxed.height:**
1. Layouter arm Boxed: se `height: Some(h)` E body real height
   > `h`:
   - Se `clip == true`: emit body via `FrameItem::Group` com
     `clip_mask: Rect` (paridade P242 mecanismo) limitando à
     altura `h`.
   - Se `clip == false`: emit body normal (overflow visível,
     paridade vanilla padrão).
2. Se `height: None` ou body cabe em `h`: comportamento P156H
   preservado literal.

**Activação C — TableCell overflow:**
1. Arm GridCell/TableCell em layout_grid: usar `regions.cell.height`
   (já populado P246) para detectar overflow body.
2. Se body overflow + cell tem `breakable` herdado (paridade
   vanilla TableCell.breakable): row break (algoritmo
   condicional simples).
3. Se body overflow + cell não breakable: clip ou aceitar
   overflow visual conforme decisão (decisão fixa §3 pós-audit
   §2.8 conferindo paridade vanilla).
4. Activação C interage com DEBT-34e — limitar a "overflow Y
   simples" sem expansão colspan/rowspan; DEBT-34e preservado
   aberto.

### §1.3 Tests esperados

Tests P248 novos estimados: **25-35** (range L magnitude;
3 activações × cenários combinatorial):

- 8-10 unit Block breakable (true cabe, true não cabe page
  break natural, false cabe na actual, false cabe em página
  nova, false overlong always emit; combinados com inset/
  height; combinados com fill/stroke/outset P247).
- 5-7 unit Boxed height (None preserva, Some cabe normal,
  Some excede + clip true → clipped, Some excede + clip false
  → overflow visível; combinados radius/outset P242/P247).
- 5-8 unit TableCell overflow (cell sem overflow normal,
  cell overflow + row break, cell overflow + clip; combinados
  com colspan=1/rowspan=1 paridade single-cell; multi-row
  table com 1 cell que overflow).
- 4-6 E2E layout cross-activação (Block breakable=false dentro
  de Table; Boxed height overflow dentro de Block breakable;
  TableCell overflow numa table com Block breakable=false
  no header).
- 3-4 unit Layouter (mecanismo medição antecipada;
  `measure_content_constrained` consume; page break
  trigger pre-emit).

**Workspace pós-P248**: **2229 → ~2254-2264 verdes** (range
+25-35 paridade L magnitude).

### §1.4 Adaptações pre-existentes

Estimativa **N=0-5** adaptações tests pré-existentes:

- Tests P156G + P156H + P157B preservam output literal quando
  `breakable: true` + `height: None` + cell body cabe na
  célula (defaults pre-P248).
- Tests Layouter que verificam comportamento E2E de Block /
  Boxed / Table em cenários **sem overflow** preservados.
- Tests que **propositalmente exercitam overflow** (improvável
  pre-P248 dado scope-out P156G/H/P157B) precisariam de
  adaptação — confirmar `P248.div-N` formal antes de adaptar.

---

## §2 Verificação empírica pré-P248 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=10 → 11 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=10 P247 ("mapear scope-outs declarados historicamente vs
estado real materializado antes de assumir ausência") expande
para **N=11 cumulativo**: "mapear pontos de check overflow
existentes antes de adicionar novos checks duplicados".

### §2.1 Mecanismo page break existente (confirmado 2026-05-14)

```bash
grep -rn "new_page\|cursor_y >" 01_core/src/rules/layout/ | head -20
```

Resultado audit anterior: 9 sítios distintos disparando
`new_page()`. `cursor.rs:126` é check principal por overflow Y.
**P248 acrescenta checks antecipados em arms específicos**,
não substitui mecanismo existente.

### §2.2 `breakable` lido em qualquer sítio (confirmado 2026-05-14)

```bash
grep -rn "\.breakable\b\|breakable:" 01_core/src/rules/layout/ \
  | grep -v "test\|/\*"
```

Resultado audit anterior: zero leituras reais (todos `breakable: _`).
**Confirma trabalho material 100% pendente para Activação A**.

### §2.3 `regions.cell.height` consumers (confirmado 2026-05-14)

`placement.rs:55` + `placement.rs:101`. **Não consumido por
arm GridCell para overflow detection**. Activação C precisa
adicionar consumer.

### §2.4 Helper `measure_content_constrained` capacidades

```bash
grep -B2 -A 8 "fn measure_content_constrained" 01_core/src/rules/layout/
```

Identificar:
- Aceita largura máxima? Altura máxima?
- Retorna `(width, height)` ou `(height, items)` para
  caching?
- Side-effects (avança cursor? emite items?) — preservar pureza.

**Hipótese pré-P248**: pura, returns dimensões sem side-effects.
Reutilizável directamente.

### §2.5 Algoritmo float vanilla para breakable + height — referência

```bash
ls lab/typst-original/crates/typst-layout/src/flow/
ls lab/typst-original/crates/typst-library/src/layout/block.rs
```

Identificar:
- Como vanilla decide page break para Block.breakable false?
- Existe lookahead/backtrack ou só forward heurística?
- Boxed.height overflow: clip default ou erro?

**Referência empírica obrigatória em C2** antes de fixar
implementação Decisões 1-3.

### §2.6 TableCell body actual layout

```bash
grep -B2 -A 12 "Content::TableCell\|Content::GridCell" \
  01_core/src/rules/layout/grid.rs | head -50
```

Como é actualmente layouted o body de uma cell? Reusa
`layout_content` recursivo? Tem buffer separado? **Crítico para
Activação C**.

### §2.7 Tests pré-P248 baseline

```bash
cargo test --workspace
```

Esperado: **2229 verdes** (estado pós-P247).

### §2.8 Decisão arquitectural pós-audit

Após §2.1-§2.7 completos, fixar empíricamente:
- **Decisão 1** algoritmo Block.breakable (look-ahead único
  vs medição completa).
- **Decisão 2** Boxed.height overflow (paridade vanilla:
  clip default ou overflow visível default).
- **Decisão 3** TableCell overflow (row break vs cell clip
  conforme audit §2.6).

### `P248.div-N` antecipadas — possíveis

- **`P248.div-1`** se §2.4 revelar que `measure_content_constrained`
  tem side-effects (avança cursor) → escopo expandir para incluir
  refactor desta helper a pura.
- **`P248.div-2`** se §2.5 revelar paridade vanilla
  significativamente diferente da hipótese (ex: Boxed.height
  vanilla expande page em vez de clip) → re-escopo.
- **`P248.div-3`** se §2.6 revelar que cell body layouting já
  tem mecanismo embrionário de overflow → cenário tipo
  P243→P244 (trabalho parcial pré-existente).
- **`P248.div-4`** se §2.7 baseline ≠ 2229 → reconciliação prévia.

---

## §3 Decisões fixadas P248 — 10 decisões

### Decisão 0 — Audit C1 lição N=10 → 11 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=10 → **11 cumulativo**. Refino procedural P248: "mapear
pontos de check overflow existentes antes de adicionar novos
checks duplicados". Anotação em ADR-0080 §"Lição refinada P248".

### Decisão 1 — Block.breakable activação algoritmo (preliminar; final pós-audit §2.8)

**Pseudo-código preliminar**:

```rust
Content::Block { body, width, height, inset, breakable,
                  outset, radius, clip, fill, stroke } => {
    self.flush_line();
    let inset_top    = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);
    let outset_top    = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);

    // P248: se breakable false, medir antecipadamente
    if !*breakable {
        let avail_w = self.available_width();
        let (_, body_h) = self.measure_content_constrained(body, avail_w);
        let block_total_h = outset_top + inset_top + body_h
                          + inset_bottom + outset_bottom
                          + height.map(|h| h.resolve_pt(font).max(0.0))
                                  .unwrap_or(0.0).max(body_h);
        let page_usable_h = self.regions.current.height
                          - 2.0 * self.page_config.margin
                          - self.cursor_y_top_reserve
                          - self.cursor_y_bottom_reserve;
        let remaining_h = page_usable_h
                        - (self.regions.current.cursor_y.0
                           - self.page_config.margin
                           - self.cursor_y_top_reserve);

        // 3 cenários:
        if block_total_h > page_usable_h {
            // Overlong: emit normal (paridade vanilla)
        } else if block_total_h > remaining_h {
            // Cabe em página nova: new_page() antes
            self.new_page();
        }
        // else: cabe na actual: emit normal
    }

    // Layout body normalmente (P156G semantic preservada)
    // ... outset + inset + body + height min ...
}
```

**Justificação**: medição antecipada via helper existente;
3 cenários distintos paridade vanilla; decisão final §2.8
confirma overlong handling.

### Decisão 2 — Boxed.height overflow (preliminar)

**Paridade vanilla a confirmar §2.5**:

- `height: None` → comportamento P156H preservado literal.
- `height: Some(h)` + body cabe → preservado literal.
- `height: Some(h)` + body excede:
  - `clip: true` → emit body via `FrameItem::Group` com
    `clip_mask: Rect { ..., height: h }` (reuso P242).
  - `clip: false` → emit body normal (overflow visível).

**Decisão final §2.8 confirma paridade vanilla**.

### Decisão 3 — TableCell overflow (preliminar)

**Algoritmo conservador**:

- Cell body actual cabe em `regions.cell.height` → preservado
  literal P157B.
- Cell body overflow → **clip implícito** ao limite cell
  (paridade vanilla default; row break é refinamento
  posterior).
- Row break é **scope-out P248** (refino futuro per ADR-0054
  graded; promoção a DEBT-34g novo se priorizado humanamente).

**Decisão**: P248 implementa **clip implícito de cell overflow**
(scope minimal); row break diferido.

### Decisão 4 — Mecanismo comum: medição antecipada via helper existente

`measure_content_constrained` reusado em Block + Boxed +
TableCell. Helper preservado puro (sem side-effects).

Se §2.4 revelar side-effects → `P248.div-1` formal + refactor
helper a pura.

### Decisão 5 — DEBT-34c + DEBT-34e + DEBT-30 sentinelas preservadas

- **DEBT-34c** (ENCERRADO P83): preservado. P246 + P248
  cumulativo via `regions.cell.height`.
- **DEBT-34e** (EM ABERTO): preservado aberto; **P248 não
  fecha** (row break / colspan placement diferidos).
- **DEBT-30** (ENCERRADO P79 clipping): preservado. P248
  reusa `clip_mask: Rect` em Boxed + TableCell.

### Decisão 6 — Sem novo Content variant; sem nova ADR; sem novo entity type

P248 é activação consumer Layouter. **Anti-inflação 40ª
aplicação cumulativa** preservar.

### Decisão 7 — Padrão emergente "Agregar promoções graded → real multi-consumer" N=1 inaugurado P248

Sub-padrão novo:
- 3 promoções agregadas com **mecanismo comum** (medição
  antecipada via helper); distinto P247 "agregar promoções
  cosméticos visuais" onde cada cosmetic é ortogonal aditivo.
- Magnitude L (não L+) porque mecanismo comum reduz custo.
- Tests cross-multiplicados naturalmente.

Candidato a formalização N=3-4 futuro.

### Decisão 8 — Padrão "Promoção graded → real semantic activação consumer" N=1 → 2 cumulativo

P245 inaugurou N=1 (Place float real). **P248 N=2 cumulativo**
agregado: Block.breakable + Boxed.height + TableCell (3
sub-activações granulares em passo único). Pattern emergente
sólido reforçado.

### Decisão 9 — Anti-inflação 40ª aplicação cumulativa

- Opção β L0 minimal (apenas leitura de `breakable` activada;
  comentários scope-outs P156G/H/P157B anotados removidos ou
  refinados em L0 content.md; hash propagado).
- Opção α activação consumer real (3 sub-activações).
- Opção α reuso `measure_content_constrained` (helper existente).
- Opção α reuso `FrameItem::Group + clip_mask` (P242 mecanismo).
- Opção α anotação cumulativa minimal ADRs (0061+0079+0080
  + nova entrada cumulativa ADR-0054 §"Promoções reais" N=5
  → 6+ granular cumulativo).
- Opção α sub-padrão N=1 inaugurado anotado.
- Opção α DEBT-34e preservado aberto (sem reabertura formal;
  documentado relação cumulativa P248 não fecha).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Arm Block: `breakable: _` → `breakable: *breakable` + medição antecipada + page break decision; Arm Boxed: height overflow handling |
| L1 Layouter | `01_core/src/rules/layout/grid.rs` | Arm GridCell/TableCell: overflow detection + clip implícito via `regions.cell.height` |
| L1 helpers | (eventual) `01_core/src/rules/layout/mod.rs` ou módulo | Verificar `measure_content_constrained` puridade; refactor se §2.4 revelar side-effects |
| L0 prompt | `00_nucleo/prompts/entities/content.md` | Secção Block: "breakable real activado P248"; Secção Boxed: "height overflow + clip activado P248"; secção TableCell: "overflow clip implícito P248"; "Limitações conscientes" anotadas P156G/H/P157B fechadas |
| Tests Layouter | `01_core/src/rules/layout/tests.rs` | 3-4 unit tests Layouter (mecanismo medição + page break trigger) |
| Tests content | `01_core/src/entities/content.rs` (test module) | Tests breakable + height + cell overflow integração |
| Tests stdlib | `01_core/src/rules/stdlib/mod.rs` (test module) | Tests cross-attribute combinatorial (breakable + height + fill/stroke P247 + radius/clip P242) |
| Tests E2E | `03_infra/tests/` ou local | 4-6 E2E layout cross-activação |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `block(...)` + `box(...)` reclassificadas (footnote ⁶⁶ P248); cobertura Layout per metodologia recalculada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotação P248: 3 semanticas reais activadas |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | Categoria A.4 §"Sub-categorias materializadas": Block.breakable + Boxed.height + TableCell overflow real P248 |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P248" N=11 cumulativo; sub-categoria "Layouter consumer migration via API wrapper" preservada; **nova sub-categoria** "Activação semantic real multi-consumer via mecanismo comum" N=1 inaugurada |
| ADR-0054 | `00_nucleo/adr/typst-adr-0054-graded-paridade.md` | §"Promoções reais" cumulativo: P242 ×2 + P245 ×1 + P247 ×3 + P248 ×3 = **9 promoções reais cumulativas granular**; limiar ADR meta candidata reforçado N=9 |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-34c + DEBT-34e + DEBT-30 sentinelas preservadas; nova entrada cumulativa P248 (sem reabertura; anotação relação) |
| Relatório P248 | `00_nucleo/materialization/typst-passo-248-relatorio.md` | Estrutura canónica passos materialização L magnitude |

---

## §5 Critério aceitação P248 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2229 → ~2254-2264 verdes** (+25-35 paridade L) |
| `crystalline-lint .` | **0 violations** |
| `crystalline-lint --fix-hashes` | 0 ou 1 hash propagado (`content.md` se refino L0 documentar 3 activações) |
| Content variants | **62 preservado** (zero novos) |
| ShapeKind variants | **5 preservado** |
| Block fields | **10 preservado** (P247 final) |
| Boxed fields | **10 preservado** |
| TableCell fields | **5 preservado** (P157B final) |
| Layouter fields | preservado |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** |
| §A.5 `block(...)` | reclassificação implementado⁺ + footnote ⁶⁶ P248 |
| §A.5 `box(...)` | reclassificação implementado⁺ + footnote ⁶⁶ P248 |
| Cobertura Layout per metodologia | **~94-95% → ~95-96%** (+1pp refino qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| Promoções graded → real cumulativas (P245+P248) | **N=1 → N=2** ("Promoção graded → real semantic activação consumer") |
| Promoções reais scope-outs ADR-0054 cumulativas granular | **5 (P242 ×2 + P247 ×3) → ~8-9** (P248 ×3 sub-activações; P245 não conta este pattern) |
| ADR-0079 Categoria A.4 | anotação cumulativa P242+P246+P247+P248 |
| ADR-0080 sub-categorias | "Activação semantic real multi-consumer via mecanismo comum" N=1 inaugurada |
| ADR-0061 §"Refino futuro" | anotação P248 |
| ADR-0054 §"Promoções reais" | cumulativo granular ~8-9 — limiar ADR meta reforçado N≥6 |
| DEBT-34c | ENCERRADO preservado P83 |
| DEBT-34e | EM ABERTO preservado (não fecha P248; relação cumulativa anotada) |
| DEBT-30 | ENCERRADO preservado P79 |
| L0 hashes propagados | 0-1 (`content.md` se refino documentar) |
| Adaptações pre-existentes | **N=0-5** estimadas; `P248.div-N` se >5 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Agregar promoções graded → real multi-consumer" N=1 inaugurado; "Promoção graded → real semantic" N=1 → 2 cumulativo; "Spec C1 audit obrigatório bloqueante" N=10 → 11 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2229 verdes pré-P248 →
   ~2254-2264 pós-P248 (+25-35 novos; N=0-5 adaptações
   documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P248 toca Layouter consumer apenas; trait Introspector
   intocada.
3. **Backward compat**: Block com `breakable: true` (default) +
   Boxed com `height: None` + TableCell sem overflow renderizam
   idênticos a P247 (output PDF bit-equivalente).

**Promoções ADR esperadas**:

- ADR-0079 Categoria A.4 cumulativa P242+P246+P247+P248.
- ADR-0080 sub-categoria nova "Activação semantic real
  multi-consumer via mecanismo comum" N=1.
- ADR-0061 §"Refino futuro" anotação P248.
- ADR-0054 §"Promoções reais" granular cumulativo ~8-9
  (limiar ADR meta reforçado).
- **Sem novas ADRs criadas**.

---

## §6 Próximo sub-passo pós-P248

P248 fecha 3 promoções graded → real. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Block 4 scope-outs restantes** | spacing + above + below + sticky (paridade P247 agregada) | S-M | média |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang | XS | baixa |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito) | M-L | baixa-média |
| **ADR meta admin XS** — "promoções reais scope-outs" N≥9 | Formalizar pattern cumulativo (limiar sólido pós-P248) | XS | **alta** (patamar conceptual sólido atingido) |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva pós-P248**: **ADR meta admin XS**
"promoções reais scope-outs" — N≥9 cumulativo granular
(2 P242 + 1 P245 + 3 P247 + 3 P248) atinge limiar sólido para
formalização. Patamar conceptual claro; paridade "passo
administrativo XS" N=6 (limiar P244). Magnitude XS controlada.

Alternativa: **A.4 Block 4 scope-outs restantes** (spacing +
above + below + sticky, S-M agregado) — paridade P247 agregação
fecharia Block.A.4 completo (9/9 scope-outs originais P156G).

**Decisão humana fica em aberto literal** pós-P248.

**Estado esperado pós-P248**:
- Tests workspace: **~2254-2264 verdes** (+25-35 P248).
- Content variants: **62 preservado**.
- Block fields: **10 preservado**.
- Boxed fields: **10 preservado**.
- TableCell fields: **5 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: preservado.
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnotes ⁶⁶ acrescentada).
- Cobertura Layout per metodologia: **~94-95% → ~95-96%**
  (+1pp refino qualitativo).
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**. Anotações
  cumulativas 0061+0079+0080+0054.
- **Saldo DEBTs: 11 preservado** (DEBT-34c+DEBT-34e+DEBT-30
  sentinelas preservadas; sem reabertura; sem novo DEBT).
- **40 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P248**:
  - "Agregar promoções graded → real multi-consumer via
    mecanismo comum" N=1 inaugurado P248.
  - "Promoção graded → real semantic activação consumer" N=1
    → **2 cumulativo** (P245 Place float + P248 agregado).
  - "Spec C1 audit obrigatório bloqueante" N=10 → **11
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" granular **~8-9
    cumulativo** (P242 ×2 + P247 ×3 + P248 ×3).
- **Scope-outs originais Block fechados cumulativamente**:
  5/9 → **6/9** (outset+radius+clip+fill+stroke+**breakable
  real**); restam 3 (spacing+above+below+sticky = 4
  scope-outs vanilla mas spacing tipicamente conta como 1).
- **Scope-outs originais Boxed fechados**: 5/6 → **5/6**
  (preservado — Boxed.height não era listado como scope-out
  P156H; era field implementado parcialmente; **P248 promove
  semantic real**).
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa.
- **Categoria C.1 Fase 5 Layout**: cumprida P245.
- **Categoria C.2 Fase 5 Layout**: parcial (cell overflow
  clip implícito P248; row break real pendente).
- **Marco interno**: 3 semantic real activações cumulativas
  num passo único; sub-padrão N=1 inaugurado; promoção graded
  → real atinge N=2 cumulativo; lição C1 audit N=11
  cumulativa refinada; mecanismo comum medição antecipada
  validado primeira vez em escala multi-consumer.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.7 completos. **Lição N=11 cumulativa**:
   refino procedural "mapear pontos de check overflow existentes
   antes de adicionar novos checks duplicados". Se §2.4 revelar
   `measure_content_constrained` com side-effects → `P248.div-1`
   formal + refactor helper. Se §2.6 revelar mecanismo
   embrionário cell overflow → `P248.div-3` formal (cenário
   tipo P243→P244).

2. **Decisões 1-3 final fixas pós-audit §2.8**. Algoritmos
   preliminares em §3 são hipóteses; decisão final baseada em
   achados §2.5 (paridade vanilla) + §2.6 (mecanismo cell
   actual). **Referência empírica `lab/typst-original/`
   obrigatória em C2** antes de cristalizar implementação.

3. **Ordem de implementação recomendada**:
   1. Audit C1 §2 completo (~30-45 min — inclui leitura
      vanilla).
   2. Decisões finais §3 (~10-15 min documentação).
   3. Activação A (Block.breakable) com tests (~90-120 min).
   4. Activação B (Boxed.height) com tests (~45-60 min).
   5. Activação C (TableCell overflow) com tests (~60-90 min).
   6. Tests E2E cross-activação (~45-60 min).
   7. Anotações ADRs + inventário 148 (~20-30 min).
   8. Relatório P248 (~30-45 min).

   **Total ~5-8h** paridade L magnitude.

4. **Backward compat literal**: defaults P156G/P156H/P157B
   preservados (breakable=true, height=None, cell sem overflow).
   Output PDF bit-equivalente para casos default.

5. **Não criar variant novo**. P248 é activação consumer puro.
   Anti-inflação 40ª preservar.

6. **DEBT-34e preservado aberto**. P248 trata "cell body overflow Y"
   (clip implícito) — distinto de "colspan/rowspan placement"
   (DEBT-34e). Anotação cumulativa em DEBT.md documenta relação
   sem reabertura.

7. **Custo real esperado**: ~5-8h (paridade L magnitude).
   Maior parcela: lógica algorítmica cross-3-arms (~50%);
   tests cross-multiplicados (~30%); audit C1 + decisões +
   anotações ADR (~20%).

8. **`P248.div-N` cenários antecipados em §2.8**. Activar se:
   - `measure_content_constrained` side-effects (`P248.div-1`).
   - Vanilla paridade muito diferente (`P248.div-2`).
   - Cell overflow mecanismo embrionário existente (`P248.div-3`).
   - Baseline ≠ 2229 (`P248.div-4`).

9. **Anti-inflação 40ª aplicação cumulativa** pós-P205D
   preservar: Opção β L0 minimal (content.md hash propagado
   se refino documenta 3 activações) + Opção α activação
   consumer real (3 sub-activações cumulativas) + Opção α
   reuso `measure_content_constrained` (helper existente) +
   Opção α reuso `FrameItem::Group + clip_mask` (P242
   mecanismo) + Opção α anotação cumulativa minimal ADRs
   (0061+0079+0080+0054) + Opção α sub-padrão N=1 inaugurado
   anotado + Opção α DEBT-34e preservado aberto sem reabertura.
