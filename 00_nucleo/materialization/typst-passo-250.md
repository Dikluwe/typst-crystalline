# Spec do passo P250 — A.4 Block 4 scope-outs restantes (spacing + above + below + sticky) com semantic real + refactor Sequence consumer peekable (agregado L; primeira aplicação cumulativa citante ADR-0082 PROPOSTO N=1 → 1; fecha 10/10 scope-outs originais Block P156G; A.4 Block completo)

**Data**: 2026-05-14.
**Tipo**: refino aditivo a variant Block + activação consumer
Layouter + **refactor Sequence consumer para peekable (cross-arm
arquitectural)**. Promove 4 scope-outs originais P156G
restantes com semantic real cumulativa. **+4 fields em Block**
(spacing + above + below + sticky); **0 fields em Boxed**
(estes 4 scope-outs são exclusivos Block — vanilla BlockElem
properties; BoxElem não os tem).
**Magnitude planeada**: **L (~5-7h)** — paridade conservadora.
Maior parcela: lógica spacing neighbour-aware (~30%) + refactor
Sequence consumer peekable (~30%) + lógica sticky lookahead
(~20%) + tests cross-multiplicados (~15%) + audit C1 +
anotações (~5%).
**Marco**: **primeira aplicação citante ADR-0082 PROPOSTO**
N=0 → 1 (paridade pattern ADR-0065 P156K validado pós-P156J
N=1 citante → P157A N=2 → P157B N=3 EM VIGOR); **fecha 10/10
scope-outs originais Block P156G** (outset+radius+clip+fill+
stroke+breakable+spacing+above+below+sticky); **Block A.4
completo arquitecturalmente**; **primeira aplicação cumulativa
do padrão "refactor Sequence consumer cross-arm"** N=1
inaugurado; décima terceira aplicação cumulativa pattern "spec
C1 audit obrigatório bloqueante pós-P236.div-1" N=12 → 13
cumulativo (lição refinada P250: "refactor cross-arm Sequence
consumer exige audit de todos os patterns de iteração
existentes antes de migrar a peekable").

---

## §1 O que será feito

### §1.1 Estado pré-P250 (factual; confirmado audit empírico 2026-05-14)

**Block (10 fields pós-P247)**:
```rust
Block {
    body, width, height, inset, breakable,    // P156G original
    outset, radius, clip,                     // P231 + P242
    fill, stroke,                             // P247
}
```

**Scope-outs originais P156G fechados pós-P249**: 6/9 (outset
P231→P247 + radius P242 + clip P242 + fill P247 + stroke P247
+ breakable P248). **Restam 4 não-fechados**: spacing + above
+ below + sticky.

**Sequence consumer actual** (confirmado audit empírico):
```rust
// mod.rs:1 (layout consumer)
Content::Sequence(parts) => {
    for part in parts.iter() {
        self.layout_content(part);
    }
}

// mod.rs:1850+ (measure consumer)
Content::Sequence(children) => {
    let mut total_h = 0.0_f64;
    for child in children.iter() { ... }
}
```

**Single-pass simples**, sem peekable, sem index, sem look-ahead.

**Layouter fields pré-P250** (confirmado audit empírico): zero
fields `prev_*`/`pending_*`/`sticky`/`last_block`. Apenas
`prev_style`/`prev_chain` save/restore locais por arm.

**Spacing entre Blocks pré-P250**: **inexistente**. Blocks
consecutivos fluem sem qualquer separação além do conteúdo
interno. **Primeira semantic neighbour-aware spacing**.

### §1.2 Trabalho a fazer P250

**Fields novos (4)**:
1. `spacing: Option<Length>` — espaço genérico antes E depois;
   `None` == zero (paridade vanilla default).
2. `above: Option<Length>` — override de `spacing` no lado
   superior; `None` == usar `spacing`.
3. `below: Option<Length>` — override de `spacing` no lado
   inferior; `None` == usar `spacing`.
4. `sticky: bool` — `true` impede page break entre este Block
   e o próximo; default `false`.

**Block fields**: 10 → **14**.

**Boxed fields**: **10 preservado** (estes 4 scope-outs são
exclusivos Block; BoxElem vanilla não os tem).

**Refactor Sequence consumer (cross-arm)**:

Mudança em ambos os arms `Content::Sequence`:

```rust
// Antes (mod.rs:1):
Content::Sequence(parts) => {
    for part in parts.iter() {
        self.layout_content(part);
    }
}

// Depois:
Content::Sequence(parts) => {
    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        let next = iter.peek().copied();  // Option<&Content>
        self.layout_content_with_next(part, next);
    }
}
```

Novo método `layout_content_with_next(part, next: Option<&Content>)`
delega a `layout_content` para arms não-Block; arm Block consome
`next` para lógica sticky.

**Activação A — `spacing` + `above` + `below` semantic real**:

Arm Block (após flush_line, antes do outset/inset):

```rust
let above_pt = above
    .or(*spacing)
    .map(|l| l.resolve_pt(font))
    .unwrap_or(0.0);
let below_pt = below
    .or(*spacing)
    .map(|l| l.resolve_pt(font))
    .unwrap_or(0.0);

// Avança cursor.y antes do block (não no primeiro do Sequence;
// gestão via field `prev_block_was_sticky_or_first`).
self.regions.current.cursor_y += Pt(above_pt);
// ... emit Shape + body + inset/outset ...
self.regions.current.cursor_y += Pt(below_pt);
```

**Activação B — `sticky` semantic real**:

```rust
if *sticky {
    // Look-ahead: medir next se for Block.
    if let Some(next) = next {
        let next_h = self.measure_content_constrained(
            next, self.available_width()
        ).1;
        let combined_h = block_total_h + below_pt + next_h;
        if combined_h > remaining_h && combined_h <= page_usable_h {
            self.new_page();  // break antes do block actual
        }
        // else: cabe ambos OU overlong → emit normal.
    }
    // else (sem next): sticky sem efeito; emit normal.
}
```

### §1.3 Tests esperados

Tests P250 novos estimados: **20-30** (range L magnitude;
4 activações × cenários combinatorial + refactor Sequence):

- 6-8 unit Block spacing (above=Some, below=Some, ambos None
  default zero, ambos Some override spacing, spacing sozinho,
  combinados com inset/outset P247).
- 4-6 unit Block sticky (sticky=true + next cabe, sticky=true
  + next não cabe → break antes, sticky=true sem next, sticky
  =false preserva P248).
- 4-6 unit stdlib `native_block` (4 args novos aceitos +
  default None/false, tipos errados rejeitados, spacing
  negativo rejeitado).
- 3-5 E2E layout (cross-attribute: spacing + breakable P248;
  sticky + heading; multi-block sequence com spacing
  cumulativo).
- 2-4 unit Sequence refactor (preserva comportamento
  pre-refactor para non-Block children; peekable invariants).
- 1-2 unit Block.A.4 completude (Block.A.4 sentinela 10/10).

**Workspace pós-P250**: **2255 → ~2275-2285 verdes** (range
+20-30 paridade L magnitude).

### §1.4 Adaptações pre-existentes

Estimativa **N=5-15** adaptações tests pré-existentes (range
maior que P247 N=12 devido a refactor Sequence cross-arm):

- Construtores `Content::block(...)` ganham +4 args (spacing,
  above, below, sticky).
- PartialEq tests com fields explícitos → adicionar 4 fields.
- Tests E2E Sequence-based que verificam comportamento
  exacto **devem preservar literal** quando spacing/above/
  below/sticky = defaults (`None`/`None`/`None`/`false`).
- Tests Layouter que constroem `Content::Sequence` directamente
  → preservados literal (refactor consumer não muda interface).

**Cenário `P250.div-N` se >15 adaptações** → reconciliação
prévia.

---

## §2 Verificação empírica pré-P250 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=12 → 13 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=12 P249 ("ADR meta administrativo XS exige audit empírico
das N≥4 aplicações concretas antes de formalizar pattern")
expande para **N=13 cumulativo**: "refactor cross-arm Sequence
consumer exige audit de todos os patterns de iteração
existentes antes de migrar a peekable".

### §2.1 Inventário Block fields actuais (confirmado 2026-05-14)

10 fields pós-P247. Confirmado via sed + spec P247 §1.1.

### §2.2 Sequence consumer patterns (confirmado 2026-05-14)

```bash
grep -rn "Content::Sequence" 01_core/src/rules/layout/
```

**2 sítios** identificados:
- `mod.rs:1` (layout consumer) — `for part in parts.iter()`.
- `mod.rs:1850+` (measure consumer) — `for child in children.iter()`.

**Hipótese pré-P250**: ambos refactorados simétricamente para
peekable. Audit empírico confirmar pré-P250.

### §2.3 Look-ahead pré-existente em Layouter

```bash
grep -rn "peekable\|peek()" 01_core/src/rules/layout/
```

Confirmado audit anterior: **zero peekable usage no Layouter
actual**. P250 inaugura pattern.

### §2.4 Algoritmo vanilla spacing/above/below/sticky — REFERÊNCIA EMPÍRICA OBRIGATÓRIA

```bash
cat lab/typst-original/crates/typst-library/src/layout/block.rs | head -100
grep -B2 -A 5 "above\|below\|sticky" lab/typst-original/crates/typst-library/src/layout/block.rs
```

Identificar:
- Default values vanilla (`spacing` default 1.2em? `above`/
  `below` default igual a `spacing`? `sticky` default false?).
- Resolução `above.or(spacing)` ou outro fallback?
- Sticky vanilla algoritmo: lookahead 1-block ou multi-block?
- Interacção sticky + breakable: sticky força breakable=false
  no block actual?

**Referência empírica obrigatória em C2** antes de fixar
implementação Decisões 1-4.

### §2.5 Layouter fields actuais (confirmado 2026-05-14)

`pub(super)` fields confirmados via grep (linhas 82-201
mod.rs). **P250 adiciona 0-2 fields**:
- **`prev_below_consumed: bool`** (opcional) — para evitar
  double-spacing entre blocks consecutivos (`prev.below` +
  `curr.above` colapsam ou somam? Decisão §3 fixa pós-audit
  §2.4 vanilla).
- **`is_first_in_sequence: bool`** (opcional) — para suprimir
  `above` no primeiro Block dum Sequence.

**Decisão final fixa pós-audit §2.7** se fields necessários.

### §2.6 Tests pré-P250 baseline

```bash
cargo test --workspace
```

Esperado: **2255 verdes** (estado pós-P249).

### §2.7 Decisão arquitectural pós-audit

Após §2.1-§2.6 completos, fixar empíricamente:
- **Decisão 1** Sequence refactor (peekable simétrico ambos
  arms).
- **Decisão 2** algoritmo spacing collapse (paridade vanilla
  §2.4: máx, soma, ou só `below` consumido).
- **Decisão 3** sticky lookahead (paridade vanilla §2.4: 1-block
  ou multi-block).
- **Decisão 4** Layouter fields opcionais (`prev_below_consumed`,
  `is_first_in_sequence`).

### `P250.div-N` antecipadas — possíveis

- **`P250.div-1`** se §2.2 revelar Sequence consumer com
  patterns adicionais não-listados (improvável).
- **`P250.div-2`** se §2.4 paridade vanilla revelar semantic
  significativamente diferente (ex: sticky vanilla usa multi-block
  ou interage com outros mecanismos não previstos) → re-escopo.
- **`P250.div-3`** se §2.5 revelar trabalho parcial pré-existente
  (improvável dado audit empírico extensivo precedente).
- **`P250.div-4`** se §2.6 baseline ≠ 2255 → reconciliação prévia.

---

## §3 Decisões fixadas P250 — 11 decisões

### Decisão 0 — Audit C1 lição N=12 → 13 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=12 → **13 cumulativo**. Refino procedural P250: "refactor
cross-arm Sequence consumer exige audit de todos os patterns
de iteração existentes antes de migrar a peekable". Anotação
em ADR-0080 §"Lição refinada P250".

### Decisão 1 — 4 fields novos em Block (paridade vanilla)

- `spacing: Option<Length>` (default `None` == zero).
- `above: Option<Length>` (default `None` == fallback `spacing`).
- `below: Option<Length>` (default `None` == fallback `spacing`).
- `sticky: bool` (default `false`).

**Boxed permanece 10 fields** — estes scope-outs são
exclusivos Block.

**Padrão "refino aditivo paralelo entre variants irmãos"**
**não se aplica** P250 (assimetria vanilla intencional).

### Decisão 2 — Refactor Sequence consumer para peekable

Ambos arms `Content::Sequence` migrados simétricamente:

```rust
// layout consumer (mod.rs:1):
Content::Sequence(parts) => {
    let mut iter = parts.iter().peekable();
    let mut first = true;
    while let Some(part) = iter.next() {
        let next = iter.peek().copied();
        self.layout_content_with_context(part, next, first);
        first = false;
    }
}

// measure consumer (mod.rs:1850+): preservado simples
// (medição não precisa de neighbour context — spacing colapsa
// para zero na medição estática per ADR-0054 graded).
```

Novo método `layout_content_with_context(part, next, is_first)`
delega a `layout_content` para non-Block arms; arm Block consume
`next` + `is_first`.

### Decisão 3 — Algoritmo spacing collapse (preliminar; final pós-audit §2.4)

**Preliminar (paridade vanilla provável)**:
- `above` no primeiro Block dum Sequence → **suprimido**
  (`is_first == true` ignora above).
- `prev.below + curr.above` entre Blocks consecutivos →
  **colapso max(prev.below, curr.above)** (paridade CSS margin
  collapse; semantic vanilla a confirmar §2.4).
- **Decisão final §2.7 confirma** paridade vanilla exacta.

### Decisão 4 — Algoritmo sticky lookahead (preliminar)

**Preliminar**:
```rust
if *sticky {
    if let Some(next) = next {
        if matches!(next, Content::Block { .. }) {
            // Medir next + actual; decidir page break antes.
            let next_h = self.measure_content_constrained(next, avail_w).1;
            let combined = block_total_h + below_pt + next_h;
            let remaining = self.available_height();
            if combined > remaining && combined <= page_usable_h {
                self.new_page();
            }
            // else: cabe OU overlong → emit normal.
        }
        // next não-Block: sticky sem efeito; emit normal.
    }
    // sem next: sticky sem efeito; emit normal.
}
```

**Lookahead 1-block apenas** (não multi-block). Paridade
vanilla a confirmar §2.4 — se vanilla suporta sticky
transitivo (block A sticky → B sticky → C), `P250.div-2`
formal + re-escopo.

### Decisão 5 — Layouter fields opcionais 0-2

Preliminar **0 fields novos** (algoritmo encapsulado no método
`layout_content_with_context` via parâmetros `next` + `is_first`).

Se §2.4-§2.7 revelar necessidade de `prev_below_consumed` para
evitar double-spacing → adicionar 1 field. **Decisão final
§2.7**.

### Decisão 6 — Sem novo Content variant; sem nova ADR; sem novo entity type

P250 é refino aditivo + activação consumer Layouter + refactor
Sequence consumer. **Anti-inflação 42ª aplicação cumulativa**
preservar.

### Decisão 7 — Cita ADR-0082 PROPOSTO N=0 → 1 (primeira citante)

P250 é **primeira aplicação concreta a citar ADR-0082
PROPOSTO** (criada P249). 4 critérios operacionais ADR-0082
verificados explicitamente:

1. **Storage prévio**: 4 fields já declarados scope-out P156G
   (não são variants novos).
2. **Consumer Layouter pre-promoção é graded**: 4 scope-outs
   actualmente "rejeitados em `native_block` com erro hard"
   (P156G "Limitações conscientes").
3. **Paridade vanilla referência empírica**: audit C1 §2.4
   obrigatório antes de cristalizar.
4. **Backward compat literal**: defaults `None`/`None`/`None`/
   `false` produzem output PDF bit-equivalente para Block sem
   estes args.

**Validação ADR-0082 N=1 citante** — primeiro passo de uma
sequência hipotética N=3 para promoção EM VIGOR (paridade
ADR-0065 P156K → P156J/P157A/P157B).

### Decisão 8 — Padrão emergente "Refactor Sequence consumer cross-arm" N=1 inaugurado

P250 inaugura sub-padrão **N=1**: "Refactor Sequence consumer
cross-arm via peekable + neighbour context". Pattern emergente
candidato a formalização N=3-4 futuro se outras features
exigirem look-ahead (ex: `pagebreak weak` collapse com weak
adjacent; `weak` collapse genérico para HSpace/VSpace P156D).

### Decisão 9 — Promoções reais scope-outs ADR-0054 graded cumulativas N=8 → N=12

P250 promove **4 sub-activações granulares** novas:
- spacing semantic real.
- above semantic real.
- below semantic real.
- sticky semantic real.

**Cumulativo granular**: N=8 (pós-P248) + 4 = **N=12 cumulativo
pós-P250**. ADR-0054 §"Promoções reais cumulativas" anotada
P250 (sem promoção ADR-0054 status).

### Decisão 10 — Anti-inflação 42ª aplicação cumulativa

- Opção β L0 minimal: refino aditivo `entities/content.md`
  documentando 4 fields + 4 semantic real activadas; hash
  propagado.
- Opção α extensão field-by-field (4 fields novos asymétricos
  Block-only).
- Opção α activação consumer real (4 sub-activações).
- Opção α refactor Sequence consumer (cross-arm; primeira
  aplicação peekable).
- Opção α reuso `measure_content_constrained` (helper existente
  puro P248).
- Opção α anotação cumulativa minimal ADRs (0061 + 0079 + 0080
  + 0054 + 0082 citação primeira).
- Opção α sub-padrão N=1 "Refactor Sequence consumer cross-arm"
  inaugurado.
- Opção α scope-outs Block 6/9 → 10/10 cumulativo (Block A.4
  completo).

### Decisão 11 — Marco: A.4 Block completo 10/10 scope-outs originais P156G fechados

Após P250, **todos os 9 scope-outs declarados P156G** + breakable
(que era field não-scope-out mas com semantic adiada P248) =
**10 elementos** ficam materializados em Block. Marco conceptual:

- outset (P231 storage + P247 semantic real).
- radius (P242).
- clip (P242).
- fill (P247).
- stroke (P247).
- breakable (P248 semantic real).
- spacing (P250).
- above (P250).
- below (P250).
- sticky (P250).

**Categoria A.4 Block: COMPLETO 10/10**. Boxed: 5/6 (resta
stroke-overhang). TableCell: overflow clip implícito P248
(row break diferido).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 entity | `01_core/src/entities/content.rs` | Block: +4 fields (spacing, above, below, sticky); cascata 9 arms (declaração, construtor, is_empty `..`, plain_text `..`, PartialEq +4 fields, map_content +4 fields, map_text +4 fields, materialize_time, walk) |
| L0 prompt | `00_nucleo/prompts/entities/content.md` | Secção Block: +4 fields documentados; §"Limitações conscientes P156G" actualizada a "10/10 scope-outs P156G fechados P250"; cita ADR-0082 PROPOSTO N=1 citante |
| L1 stdlib | `01_core/src/rules/stdlib/layout.rs` | `native_block` aceita 4 named args novos; helpers `extract_length` reusado para spacing/above/below; `extract_bool` para sticky (reuso N+1) |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Arm Block: spacing/above/below cursor.y advance; sticky lookahead via `next`; refactor 2 arms `Content::Sequence` para peekable + neighbour context; novo método `layout_content_with_context(part, next, is_first)` (ou similar) |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Possivelmente +0-2 fields conforme Decisão 5 final §2.7 |
| Tests content | `01_core/src/entities/content.rs` (test module) | 4-6 unit tests cascata + adaptações construtores existentes N=5-15 |
| Tests stdlib | `01_core/src/rules/stdlib/mod.rs` (test module) | 4-6 unit tests native_block 4 args novos |
| Tests Layouter | `01_core/src/rules/layout/tests.rs` (ou módulo) | 6-8 unit Block spacing + 4-6 unit sticky + 2-4 Sequence refactor + 1-2 sentinela A.4 completude |
| Tests E2E | local | 3-5 E2E cross-attribute (spacing+breakable; sticky+heading; multi-block cumulativo) |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `block(...)` reclassificada (footnote ⁶⁷ P250 — A.4 Block completo 10/10); cobertura Layout per metodologia recalculada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotação P250: A.4 Block completo |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | Categoria A.4 §"Sub-categorias materializadas": Block.spacing/above/below/sticky P250; **Block A.4 COMPLETO** |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P250" N=13 cumulativo; sub-categoria nova "Refactor Sequence consumer cross-arm" N=1 inaugurada |
| ADR-0054 | `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` | §"Promoções reais cumulativas" extensão: P250 ×4 = cumulativo N=12 (P242 ×2 + P247 ×3 + P248 ×3 + P250 ×4) |
| **ADR-0082** | `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md` | **§"Aplicações citantes" sub-secção nova**: P250 primeira aplicação citante explícita (paridade ADR-0065 P156K → P156J N=1 citante); status preservado PROPOSTO (promoção a EM VIGOR pendente N=3 citantes) |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-30/34c/34e sentinelas preservadas; sem reabertura; sem novo DEBT |
| Relatório P250 | `00_nucleo/materialization/typst-passo-250-relatorio.md` | Estrutura canónica passos materialização L magnitude |

---

## §5 Critério aceitação P250 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2255 → ~2275-2285 verdes** (+20-30 paridade L) |
| `crystalline-lint .` | **0 violations** |
| `crystalline-lint --fix-hashes` | 1 hash propagado (`content.md`) |
| Content variants | **62 preservado** |
| ShapeKind variants | **5 preservado** |
| Block fields | **10 → 14** (+spacing, +above, +below, +sticky) |
| Boxed fields | **10 preservado** (asymétrico intencional) |
| TableCell fields | **5 preservado** |
| Layouter fields | preservado ou **+0-2** (Decisão 5 final §2.7) |
| Layouter methods | **+1** (`layout_content_with_context` ou equivalente) |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** |
| §A.5 `block(...)` | reclassificação implementado⁺ + footnote ⁶⁷ P250 (A.4 Block completo) |
| Cobertura Layout per metodologia | **~95-96% → ~96-97%** (+1pp refino qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| Scope-outs Block originais P156G fechados | 6/9 → **10/10** (incluindo breakable contado como 10º) — **A.4 Block COMPLETO** |
| Scope-outs Boxed originais P156H fechados | **5/6 preservado** (P250 não toca Boxed) |
| Promoções reais scope-outs ADR-0054 cumulativas granular | **8 → 12** (P242 ×2 + P247 ×3 + P248 ×3 + **P250 ×4**) |
| ADR-0079 Categoria A.4 | anotação cumulativa P242+P246+P247+P248+P250; Block A.4 COMPLETO |
| ADR-0080 sub-categoria | "Refactor Sequence consumer cross-arm" N=1 inaugurada |
| ADR-0061 §"Refino futuro" | anotação P250 |
| ADR-0054 §"Promoções reais" | cumulativo granular N=12 (P250 ×4) |
| **ADR-0082** | **§"Aplicações citantes" N=0 → 1** (primeira citante explícita) |
| DEBT-30/34c/34e | sentinelas preservadas |
| L0 hashes propagados | 1 (`content.md`) |
| Adaptações pre-existentes | **N=5-15** estimadas (range maior que P247 N=12 devido a refactor Sequence cross-arm); `P250.div-N` se >15 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Refactor Sequence consumer cross-arm" N=1 inaugurado; "Aplicação citante ADR-0082" N=0 → 1; "Spec C1 audit obrigatório bloqueante" N=12 → 13 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2255 verdes pré-P250 →
   ~2275-2285 pós-P250 (+20-30 novos; N=5-15 adaptações
   construtores + refactor Sequence documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P250 toca Layouter consumer + entities + stdlib apenas;
   Introspector trait intocada.
3. **Backward compat literal**: Block com defaults (`spacing=
   None`, `above=None`, `below=None`, `sticky=false`) + refactor
   Sequence preserva output pre-P250 literal (test sentinela
   `p250_block_4_scope_outs_defaults_preserva_p249`).

**Promoções ADR esperadas**:

- ADR-0079 Categoria A.4 Block A.4 **COMPLETO** documentado.
- ADR-0080 sub-categoria nova "Refactor Sequence consumer
  cross-arm" N=1 inaugurada + lição refinada N=13 cumulativo.
- ADR-0061 §"Refino futuro" anotação P250.
- ADR-0054 §"Promoções reais" cumulativo N=12 (P250 ×4 — limiar
  ADR meta reforçado).
- **ADR-0082 §"Aplicações citantes" N=0 → 1** (primeira citante
  explícita; status PROPOSTO preservado).
- **Sem novas ADRs criadas**.
- Distribuição ADRs preservada literal: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**.

---

## §6 Próximo sub-passo pós-P250

P250 fecha **Block A.4 COMPLETO** (10/10 scope-outs originais
P156G + breakable). Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Boxed 1 scope-out restante** | stroke-overhang (cita ADR-0082 PROPOSTO N=1 → 2; passo XS isolado) | XS | **alta** (Boxed A.4 completo 6/6; valida ADR-0082 N=2 citante) |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito; cita ADR-0082 N=2 → 3 → promoção EM VIGOR possível) | M-L | média-alta |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |
| **ADR-0082 promoção EM VIGOR** | Decisão humana pós-P250 se N=3 atinge limiar prematuro | XS | baixa (P251 + P252 candidatos podem materializar N=2 + N=3) |

**Recomendação subjectiva pós-P250**: **A.4 Boxed stroke-overhang**
(XS isolado) — **segunda aplicação citante ADR-0082** N=1 → 2;
fecha Boxed A.4 completo 6/6; magnitude controlada. Pós-P251
recomendação seria TableCell row break (M-L) como **terceira
citante** N=2 → 3 → **promoção ADR-0082 EM VIGOR humana**
possível.

Alternativa: **A.4 TableCell row break real** directo (M-L) —
salta passo XS Boxed; segunda citante mas magnitude maior;
P251 sticky-overhang fica para depois.

**Decisão humana fica em aberto literal** pós-P250.

**Estado esperado pós-P250**:
- Tests workspace: **~2275-2285 verdes** (+20-30 P250).
- Content variants: **62 preservado**.
- Block fields: **10 → 14**.
- Boxed fields: **10 preservado** (asymétrico).
- TableCell fields: **5 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: preservado ou +0-2.
- Layouter methods: **+1** (`layout_content_with_context`).
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁷ A.4 Block
  completo).
- Cobertura Layout per metodologia: **~95-96% → ~96-97%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=1**.
- **Saldo DEBTs: 11 preservado** (DEBT-30/34c/34e sentinelas
  preservadas; sem reabertura; sem novo DEBT).
- **42 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P250** (3):
  - "Refactor Sequence consumer cross-arm" N=1 inaugurado.
  - "Aplicação citante ADR-0082 PROPOSTO" N=0 → **1**.
  - "Spec C1 audit obrigatório bloqueante" N=12 → **13
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" granular N=8 →
    **12 cumulativo** (P250 ×4).
- **Scope-outs originais Block fechados cumulativamente**:
  6/9 → **10/10 incluindo breakable** (Block A.4 COMPLETO).
- **Scope-outs originais Boxed fechados**: 5/6 preservado.
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa
  (Block COMPLETO 10/10; Boxed 5/6; TableCell overflow clip).
- **Marco interno**: Block A.4 COMPLETO 10/10; primeira
  aplicação citante ADR-0082 PROPOSTO; padrão "Refactor Sequence
  consumer cross-arm" N=1 inaugurado; lição C1 audit N=13
  cumulativa refinada procedimentalmente; primeiro passo onde
  Sequence consumer é refactorado para peekable; algoritmo
  spacing neighbour-aware + sticky lookahead validados em
  semantic real cristalina (paridade vanilla per audit §2.4).

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.7 completos. **Lição N=13 cumulativa**:
   primeira aplicação onde audit C1 é especificamente para
   "refactor cross-arm Sequence consumer". §2.4 (referência
   vanilla `lab/typst-original/crates/typst-library/src/layout/
   block.rs`) é **crítica** para fixar Decisões 3-4 (spacing
   collapse + sticky lookahead).

2. **Decisões 3, 4, 5 final fixas pós-audit §2.7**. Algoritmos
   preliminares em §3 são hipóteses; decisão final baseada em
   achado empírico §2.4.

3. **Refactor Sequence consumer é cross-arm**. Ambos arms
   (`layout_content` mod.rs:1 + `measure_content_constrained`
   mod.rs:1850+) devem ser tratados simétricamente per Decisão
   2. **Risco médio-alto**: Sequence é hot path; refactor
   exige preservação rigorosa de comportamento para todos os
   non-Block children (Text, HSpace, VSpace, Strong/Emph,
   etc.). Test sentinela específico:
   `p250_sequence_refactor_preserva_non_block_pre_p250`.

4. **Ordem de implementação recomendada**:
   1. Audit C1 §2 completo (~30-45 min — inclui leitura
      vanilla obrigatória §2.4).
   2. Decisões finais §3 (~15-20 min documentação).
   3. Block +4 fields cascata 9 arms (~60-90 min).
   4. stdlib native_block 4 args novos (~30-45 min).
   5. Layouter refactor Sequence + arm Block + método novo
      (~90-120 min — maior parcela).
   6. Tests cross-multiplicados (~60-90 min).
   7. Anotações ADRs + inventário 148 + relatório (~30-45 min).

   **Total ~5-7h** paridade L magnitude.

5. **Backward compat literal absoluto**: defaults (None×3 +
   false) + refactor Sequence preservam output PDF
   bit-equivalente para todos os casos pre-P250. Test sentinela
   `p250_block_4_scope_outs_defaults_preserva_p249` valida
   directamente.

6. **Asymetria Block ↔ Boxed intencional**. Sub-padrão "refino
   aditivo paralelo entre variants irmãos" N=5 (último em
   P247) **não aplica P250**. Documentar em relatório §"Decisões
   substantivas" explicitamente para evitar confusão futura.

7. **Custo real esperado**: ~5-7h (paridade L magnitude). Maior
   parcela: refactor Sequence consumer (~30%); algoritmo
   spacing + sticky (~30%); tests cross-multiplicados (~25%);
   audit C1 + decisões + anotações ADR (~15%).

8. **`P250.div-N` cenários antecipados em §2.7**. Activar se:
   - Sequence consumer com patterns adicionais (`P250.div-1`).
   - Vanilla paridade muito diferente (`P250.div-2`).
   - Trabalho parcial pré-existente (`P250.div-3`).
   - Baseline ≠ 2255 (`P250.div-4`).

9. **Cita ADR-0082 PROPOSTO explícitamente**. Relatório P250
   §"Citação ADR-0082" lista 4 critérios verificados:
   1. Storage prévio ✓ (scope-out P156G).
   2. Consumer Layouter pre-promoção graded ✓ ("rejeitado em
      `native_block` com erro hard").
   3. Paridade vanilla referência empírica ✓ (audit §2.4).
   4. Backward compat literal ✓ (test sentinela).

   **Validação ADR-0082 N=1 citante**.

10. **Anti-inflação 42ª aplicação cumulativa** pós-P205D
    preservar: Opção β L0 minimal (content.md hash propagado)
    + Opção α extensão field-by-field asymétrica Block-only
    + Opção α activação consumer real (4 sub-activações) +
    Opção α refactor Sequence consumer (cross-arm; primeira
    aplicação peekable) + Opção α reuso
    `measure_content_constrained` (helper existente puro
    P248) + Opção α anotação cumulativa minimal ADRs (0061+
    0079+0080+0054+**0082 citação primeira**) + Opção α
    sub-padrão N=1 inaugurado + Opção α scope-outs Block
    6/9 → 10/10 cumulativo.

11. **Marco "A.4 Block COMPLETO"**. P250 fecha categoria
    inteira de scope-outs Block per P156G original. Documentar
    em relatório §"Marco P250" como milestone conceptual:
    primeiro variant Content que tem **100% dos scope-outs
    originais fechados** (10/10).
