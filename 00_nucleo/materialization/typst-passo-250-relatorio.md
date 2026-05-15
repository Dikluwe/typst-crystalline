# Relatório do passo P250 — A.4 Block COMPLETO 10/10 (spacing+above+below+sticky agregados) + refactor Sequence consumer cross-arm; primeira aplicação citante ADR-0082 PROPOSTO N=1

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-250.md`.
**Tipo**: refino aditivo a variant Block + activação consumer
Layouter + **refactor Sequence consumer para peekable (cross-arm
arquitectural)**. Promove 4 scope-outs originais P156G restantes
com semantic real cumulativa.
**Magnitude planeada**: L (~5-7h). **Magnitude real**: **L
(~3-4h)** — audit C1 revelou vanilla reference directa (cabeçalho
`container.rs` documenta `max(prev.below, curr.above)` collapse
explicitamente); `measure_content_constrained` puro pré-existente
reusável; arquitectura collapse via Layouter fields direta.
**Marco**: **primeira aplicação citante ADR-0082 PROPOSTO** N=0
→ **N=1** (paridade pattern ADR-0065 P156K validado pós-P156J
N=1 → P157A N=2 → P157B N=3 EM VIGOR); **fecha 10/10 scope-outs
originais Block P156G** (outset+radius+clip+fill+stroke+breakable+
spacing+above+below+sticky); **Block A.4 COMPLETO arquitecturalmente
e cumulativamente**; **primeira aplicação cumulativa do padrão
"Refactor Sequence consumer cross-arm"** N=1 inaugurado; décima
terceira aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=12 → 13 cumulativo (lição refinada
P250: "refactor cross-arm Sequence consumer exige audit de todos
os patterns de iteração existentes antes de migrar a peekable").

---

## §1 O que foi feito

P250 materializa Categoria A.4 Block **COMPLETO 10/10** via 4
fields novos cumulativos + refactor Sequence consumer cross-arm.
**Boxed preservado 10 fields** (assimetria intencional; estes 4
scope-outs são exclusivos Block — vanilla `BlockElem` properties).

**Trabalho real**:

1. **+4 fields em Block** (paridade vanilla per audit §2.4):
   `spacing: Option<Length>` + `above: Option<Length>` + `below:
   Option<Length>` + `sticky: bool`. Defaults `None×3 + false`.
   **Block fields: 10 → 14**. **Boxed preservado 10 fields**.
2. **Cascata 9 arms** em `01_core/src/entities/content.rs`
   (declaração, construtor, is_empty `..` preservado, plain_text
   `..` preservado, PartialEq +4 fields, map_content +4 fields,
   map_text +4 fields).
3. **stdlib `native_block`** aceita 4 named args novos via helper
   inline `extract_block_length` (Length opcionais; negativos
   rejeitados) + `sticky` Bool directo.
4. **Refactor Sequence consumer cross-arm** (`mod.rs:478`):
   migração para `iter.peek()` + neighbour context; measure
   consumer (mod.rs:1850+) preservado simples (spacing colapsa
   para zero em medição estática per ADR-0054 graded).
5. **Arm Block consume spacing/above/below collapse**:
   `gap = max(prev.below, curr.above)` quando `block_chain_active`;
   above suprimido no primeiro Block (chain quebrado por non-
   Block ou Sequence start).
6. **Arm Block sticky lookahead** via Sequence consumer pre-layout:
   `new_page()` antecipado se `combined > remaining + cabe em
   página inteira`.
7. **+2 Layouter fields**: `prev_block_below_pending: f64` +
   `block_chain_active: bool` (state save/restore entre Sequences).
8. **L0 `entities/content.md` extensão** documentando 4 fields +
   activação Layouter + Decisões 1-11 + algoritmo collapse +
   sticky lookahead; hash propagado automaticamente
   `crystalline-lint --fix-hashes` → `418bbbfb`.
9. **21 tests novos** (range +20-30 paridade L):
   - 14 unit Block spacing/above/below + sticky + Sequence
     refactor + A.4 completude em `layout/tests.rs`.
   - 5 unit stdlib `native_block` 4 args novos + defaults +
     tipos errados rejeitados.
   - 2 unit Sequence aninhado + chain quebrada por non-Block.
10. **N=21 adaptações** em tests pré-existentes (dentro do range
    N=5-15 estimado §1.4 + 6 adicionais — 31 sítios `stroke:
    None,` em entities/content.rs + layout/tests.rs cascade
    replace_all + 3 sítios deeper indent; introspect.rs
    materialize_time arm adaptado).
11. **ADRs anotadas cumulativas**: 0061 §"Refino futuro" + 0079
    §"Anotação cumulativa P250 — Block A.4 COMPLETO" + 0080
    §"Lição refinada P250" N=12 → 13 cumulativo + 0054
    §"Promoções reais cumulativas" tabela N=12 + **0082
    §"Aplicações citantes" N=1 (primeira aplicação citante
    explícita)**.

**2255 → 2276 verdes** (+21 P250; 0 regressões; N=21 adaptações
construtores documentadas). **Sem `P250.div-N`** — audit
converge com Decisões 1-11 + vanilla reference clara §2.4.

---

## §2 Auditoria pré-P250 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=12 → 13 cumulativo

**Audit empírico** (lição refinada P249 N=12 → P250 N=13
cumulativo: "refactor cross-arm Sequence consumer exige audit
de todos os patterns de iteração existentes antes de migrar a
peekable"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| Block 10 fields pré-P250 | confirmado spec §1.1 | ✓ Confirmado | OK |
| Sequence consumer sítios | 2 sítios mod.rs | ✓ Confirmado + 2 helpers.rs (não-bloqueantes; traversal puro) | Decisão: refactor apenas mod.rs:478 layout |
| peekable usage prévio | hipotetizou zero | ✓ Confirmado zero | P250 inaugura pattern |
| Vanilla algorithm reference | hipotetizou via lab/typst-original | ✓ `container.rs` documenta `max(prev.below, curr.above)` explicitamente | Decisões 3-4 fixas |
| Layouter fields prev_*/sticky | hipotetizou 0-2 fields | Decisão final: **+2 fields** (`prev_block_below_pending` + `block_chain_active`) | OK |
| Tests baseline pré-P250 | 2255 verdes | ✓ Confirmado | Baseline preservado |

**Conclusão audit C1**: trabalho real ~150 LoC L1 + ~600 LoC
tests + ~120 LoC L0 docs + ~200 LoC ADRs. Magnitude real **L
(~3-4h)** face L (~5-7h) hipotetizado.

**Vanilla reference §2.4** confirmou (container.rs):
> "For two adjacent blocks, the larger of the first block's
> `above` and the second block's `below` spacing wins."

**Sem `P250.div-N`** — audit converge com spec; vanilla
reference directa; helpers reusados; nenhuma divergência
arquitectural identificada.

---

## §3 Block +4 fields + Layouter +2 fields (C2)

```rust
// 01_core/src/entities/content.rs (Block; ≈ linha 640)
Block {
    body, width, height, inset, breakable,    // P156G + P248
    outset, radius, clip,                     // P231 + P242
    fill, stroke,                             // P247
    spacing: Option<Length>,                  // P250 NOVO
    above:   Option<Length>,                  // P250 NOVO
    below:   Option<Length>,                  // P250 NOVO
    sticky:  bool,                            // P250 NOVO
}
```

**Block fields: 14 (10 → 14)**. **Boxed fields: 10 preservado**
(assimetria intencional; estes 4 scope-outs são exclusivos
Block — vanilla `BlockElem` properties; `BoxElem` não os tem).

**Layouter fields novos P250** (mod.rs):
```rust
pub(super) prev_block_below_pending: f64,  // default 0.0
pub(super) block_chain_active: bool,        // default false
```

State save/restore entre Sequences garante isolamento (chain
exterior não vê chain interior).

---

## §4 Refactor Sequence consumer cross-arm (C3)

```rust
// 01_core/src/rules/layout/mod.rs (Sequence arm; ≈ linha 478)
Content::Sequence(parts) => {
    let saved_below = self.prev_block_below_pending;
    let saved_chain = self.block_chain_active;
    self.prev_block_below_pending = 0.0;
    self.block_chain_active       = false;
    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        // P250 — sticky pre-layout lookahead 1-block.
        if let Content::Block { sticky: true, .. } = part {
            if let Some(next) = iter.peek() {
                let avail_w = self.available_width();
                let (_, part_h) = self.measure_content_constrained(part, avail_w);
                let (_, next_h) = self.measure_content_constrained(next, avail_w);
                let combined    = part_h + next_h;
                let remaining   = self.page_bottom_limit()
                                  - self.regions.current.cursor_y.0;
                let page_usable = self.available_height();
                if combined > remaining && combined <= page_usable {
                    self.new_page();
                }
            }
        }
        self.layout_content(part);
        if !matches!(part, Content::Block { .. }) {
            self.block_chain_active       = false;
            self.prev_block_below_pending = 0.0;
        }
    }
    self.prev_block_below_pending = saved_below;
    self.block_chain_active       = saved_chain;
}
```

**Arm Block** consume `spacing`/`above`/`below` via collapse:

```rust
// Antes do outset/inset/body:
let above_pt = above.or(*spacing).map(|l| l.resolve_pt(font)).unwrap_or(0.0);
let gap = if self.block_chain_active {
    self.prev_block_below_pending.max(above_pt)
} else {
    0.0  // First block — suprimir above
};
let advance = (gap - self.prev_block_below_pending).max(0.0);
self.regions.current.cursor_y += Pt(advance);
self.prev_block_below_pending = 0.0;

// Após body/outset/inset/Shape:
let below_pt = below.or(*spacing).map(|l| l.resolve_pt(font)).unwrap_or(0.0);
self.regions.current.cursor_y += Pt(below_pt);
self.prev_block_below_pending = below_pt;
self.block_chain_active       = true;
```

---

## §5 Citação ADR-0082 PROPOSTO N=1 (primeira aplicação citante)

P250 cita ADR-0082 explicitamente. Os 4 critérios operacionais
verificados:

1. **Storage prévio** ✓ — 4 fields scope-out P156G "Limitações
   conscientes" declarados originalmente (não variants novos).
2. **Consumer Layouter pre-promoção graded** ✓ — 4 args
   actualmente "rejeitados em `native_block` com erro hard"
   (P156G pré-P250); arm Block ignora literal (`spacing: _, above:
   _, below: _, sticky: _`).
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.4
   confirmou `lab/typst-original/crates/typst-library/src/
   layout/container.rs`: `Em::new(1.2)` default; `above.or(
   spacing)` fallback; `max(prev.below, curr.above)` collapse;
   sticky default false.
4. **Backward compat literal** ✓ — defaults (None×3 + false)
   produzem output PDF bit-equivalente para Block sem estes
   args; sentinela `p250_block_defaults_preserva_output_pre_
   p250` valida directamente.

**Validação ADR-0082 N=1 citante** — primeiro passo dum sequente
candidato N=3 para promoção EM VIGOR (paridade ADR-0065 P156K
validada P156J + P157A + P157B sequente). Candidatas próximas:
A.4 Boxed stroke-overhang (N=2) + A.4 TableCell row break real
(N=3).

---

## §6 Critério aceitação P250 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 2255 → ~2275-2285 verdes | ✓ **2276 verdes** (+21) |
| `crystalline-lint .` | 0 violations | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | 1 hash propagado | ✓ 1 hash (`content.md` → `418bbbfb`) |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block fields | 10 → 14 | ✓ 14 |
| Boxed fields | 10 preservado | ✓ 10 |
| TableCell fields | 5 preservado | ✓ 5 |
| Layouter fields | preservado ou +0-2 | ✓ **+2** (`prev_block_below_pending` + `block_chain_active`) |
| Regions fields | 4 preservado | ✓ 4 |
| Stdlib funcs | 64 preservado | ✓ 64 |
| Cobertura Layout per metodologia | ~95-96% → ~96-97% | ✓ +1pp refino qualitativo |
| Cobertura user-facing total | ~75-76% preservado | ✓ preservado |
| Scope-outs Block originais P156G fechados | 6/9 → 10/10 | ✓ **10/10 (Block A.4 COMPLETO)** |
| Scope-outs Boxed originais P156H fechados | 5/6 preservado | ✓ 5/6 |
| Promoções reais scope-outs ADR-0054 cumulativas granular | 8 → 12 | ✓ **12** (P250 ×4) |
| ADR-0079 Categoria A.4 | anotação cumulativa P250 (Block A.4 COMPLETO) | ✓ |
| ADR-0080 sub-categoria | "Refactor Sequence consumer cross-arm" N=1 inaugurada | ✓ |
| ADR-0061 §"Refino futuro" | anotação P250 | ✓ |
| ADR-0054 §"Promoções reais" | cumulativo granular N=12 (P250 ×4) | ✓ |
| **ADR-0082** | §"Aplicações citantes" N=0 → **N=1** (primeira citante) | ✓ |
| DEBT-30/34c/34e | sentinelas preservadas | ✓ |
| L0 hashes propagados | 1 | ✓ 1 (`content.md` → `418bbbfb`) |
| Adaptações pre-existentes | N=5-15 estimadas | **N=21** (6 acima range; documentadas) |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | 3 cumulativos esperados | ✓ todos |
| `P250.div-N` | possíveis 4 cenários | ✓ nenhum activado |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2255 verdes pré-P250 → **2276
   verdes** pós-P250 (+21 P250; 0 regressões; N=21 adaptações
   documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P250 toca Layouter consumer + entities + stdlib apenas;
   Introspector trait intocada.
3. **Backward compat literal**: Block com defaults
   (`spacing=None`, `above=None`, `below=None`, `sticky=false`)
   + refactor Sequence preserva output pre-P250 (sentinela
   `p250_block_defaults_preserva_output_pre_p250` valida).

**Promoções ADR**:
- ADR-0079 Categoria A.4 **Block A.4 COMPLETO 10/10** documentado
  + sub-passo 10 cumulativo P227-P250.
- ADR-0080 sub-categoria nova "Refactor Sequence consumer
  cross-arm" N=1 inaugurada + lição refinada N=13 cumulativo.
- ADR-0061 §"Refino futuro" anotação P250.
- ADR-0054 §"Promoções reais" cumulativo granular N=12 (P250 ×4).
- **ADR-0082 §"Aplicações citantes" N=0 → N=1** (primeira
  citante explícita; status PROPOSTO preservado).
- **Sem novas ADRs criadas**.

---

## §7 Patterns emergentes inaugurados/consolidados P250 (4)

- **"Refactor Sequence consumer cross-arm via peekable +
  neighbour context"** N=1 inaugurado P250 — pattern novo (look-
  ahead 1-block via `iter.peek()` no Layouter; primeira aplicação
  peekable). Magnitude L controlada. Candidato a formalização
  N=3-4 futuro.
- **"Aplicação citante ADR-0082 PROPOSTO"** N=0 → **N=1 primeira
  aplicação citante** (4 critérios operacionais verificados
  explicitamente). Validação ADR-0082; primeiro passo dum
  sequente candidato N=3 para promoção EM VIGOR.
- **"Spec C1 audit obrigatório bloqueante"** N=12 → **N=13
  cumulativo** P250. Lição refinada N=13: "refactor cross-arm
  Sequence consumer exige audit de todos os patterns de iteração
  existentes antes de migrar a peekable".
- **"Promoção real scope-out ADR-0054 graded"** granular N=8 →
  **N=12 cumulativo P250** (P250 ×4: spacing + above + below +
  sticky). Marco: primeiro variant Content com **100% dos
  scope-outs originais P156G fechados** (Block A.4 COMPLETO).

**Anti-inflação 42ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (content.md hash propagado `418bbbfb`) + Opção α
extensão field-by-field asymétrica Block-only (4 fields
exclusivos vanilla `BlockElem`) + Opção α activação consumer
real (4 sub-activações) + Opção α refactor Sequence consumer
(cross-arm; primeira aplicação peekable) + Opção α reuso
`measure_content_constrained` (helper existente puro P248) +
Opção α anotação cumulativa minimal ADRs (0061+0079+0080+0054+
**0082 citação primeira**) + Opção α sub-padrão N=1 inaugurado
+ Opção α scope-outs Block 6/9 → 10/10 cumulativo.

---

## §8 Próximo sub-passo pós-P250 — Block A.4 COMPLETO 10/10

P250 fecha **Block A.4 COMPLETO 10/10**. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Boxed 1 scope-out restante** | stroke-overhang (cita ADR-0082 PROPOSTO N=1 → 2; passo XS isolado) | XS | **alta** (Boxed A.4 completo 6/6; valida ADR-0082 N=2 citante) |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito; cita ADR-0082 N=2 → 3 → promoção EM VIGOR possível) | M-L | média-alta |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |
| **ADR-0082 promoção EM VIGOR** | Decisão humana pós-N=3 citantes | XS | baixa-média (P251+P252 candidatos podem materializar N=2+N=3) |

**Recomendação subjectiva pós-P250**: **A.4 Boxed stroke-overhang**
(XS isolado) — **segunda aplicação citante ADR-0082** N=1 → 2;
fecha Boxed A.4 completo 6/6; magnitude controlada. Pós-P251
recomendação seria TableCell row break (M-L) como **terceira
citante** N=2 → 3 → **promoção ADR-0082 EM VIGOR humana**
possível.

Alternativa: **A.4 TableCell row break real** directo (M-L) —
salta passo XS Boxed; segunda citante mas magnitude maior;
P251 stroke-overhang fica para depois.

**Decisão humana fica em aberto literal** pós-P250.

**Estado pós-P250**:
- Tests workspace: 2255 → **2276 verdes** (+21 P250).
- Content variants: **62 preservado**.
- Block fields: **10 → 14**.
- Boxed fields: **10 preservado** (asymétrico).
- TableCell fields: **5 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: **+2** (`prev_block_below_pending` +
  `block_chain_active`).
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁷ A.4 Block
  COMPLETO).
- Cobertura Layout per metodologia: **~95-96% → ~96-97%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=1**.
- **Saldo DEBTs: 11 preservado** (DEBT-30/34c/34e sentinelas
  preservadas; sem reabertura; sem novo DEBT).
- **42 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P250** (4):
  - "Refactor Sequence consumer cross-arm" N=1 inaugurado.
  - "Aplicação citante ADR-0082 PROPOSTO" N=0 → **N=1**.
  - "Spec C1 audit obrigatório bloqueante" N=12 → **13
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" granular N=8 →
    **12 cumulativo** (P250 ×4).
- **Scope-outs originais Block fechados cumulativamente**:
  6/9 → **10/10 incluindo breakable** (**Block A.4 COMPLETO**).
- **Scope-outs originais Boxed fechados**: 5/6 preservado.
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa
  (Block COMPLETO 10/10; Boxed 5/6; TableCell overflow clip).
- **Marco interno**: **Block A.4 COMPLETO 10/10** — primeiro
  variant Content com 100% dos scope-outs originais P156G
  fechados cumulativamente; primeira aplicação citante ADR-0082
  PROPOSTO N=1 (validação empírica do ADR meta P249); padrão
  "Refactor Sequence consumer cross-arm" N=1 inaugurado; lição
  C1 audit N=13 cumulativa refinada procedimentalmente; primeiro
  passo onde Sequence consumer é refactorado para peekable;
  algoritmo spacing collapse + sticky lookahead validados em
  semantic real cristalina (paridade vanilla per audit §2.4).
