# Passo 286 — Relatório consolidado

**Tema**: Materialização da frente `P-text-deco-multiline` —
consumer Layouter de `Content::Underline`/`Strike`/`Overline`
torna-se **wrap-aware**: body multi-line produz N `FrameItem::Line`
(1 por linha visual) em vez de 1 Line ignorando wraps. Fecha o
último defeito graded remanescente do cluster decorações
P284-P285-P286.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: S (modificação cirúrgica num consumer + 1 campo
opcional no Layouter + 5 LOC em `flush_line`; sem variants novos;
zero impacto em `export.rs`; +7 testes).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 732** testes (baseline P285: 2 725 → **+7**) |
| Delta esperado +6 a +12 | ✅ +7 (dentro do range) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `layout.md` muda — consumer alterado | ✅ propagado para 11 ficheiros do cluster (`12536b5c`) |
| Hash L0 `region.md` muda **se** A.2 → (b) | ⚠ A.2 → (b) variante minimalista — apenas Layouter struct mexida, **`region.md` preservado** (refinamento decisão A.2 documentado em §3.2) |
| Hash L0 `export.rs` **preserved** (`66cb8ac3` desde P285) | ✅ **Preservado bit-exact** (emit não muda; mais ou menos Lines passam pelo mesmo caminho) |
| **Regressão bit-exact validada** para body single-line | ✅ `p286_underline_single_line_emite_uma_linha_regression_p285` + 2 725 testes pré-P286 preserved |
| Tabela A.3 linha 103 actualizada com referência P286 | ✅ Cita "P284 + P285 + **P286**" + "cluster COMPLETO" |
| P284 §5.3 marcada RESOLVIDA | ✅ Footnote ⁷² marca explicitamente |
| Cluster decorações P284-P285-P286 marcado COMPLETO | ✅ Zero defeitos graded remanescentes |
| Diagnóstico A.1+A.2+A.3 produzido | ✅ `diagnostico-deco-multiline-passo-286.md` |

**Conformidade**: 10/10 critérios estritos; 1 com observação
pragmática (`region.md` preservado em vez de mudado; ver §5.1).

---

## §2 — Resumo factual

### §2.1 — Modificação cirúrgica em 3 sítios L1

**Antes P286** (consumer P285):
- 1 arm match `Underline | Strike | Overline` → captura
  `(start_x_before, end_x_after, baseline_y)` → emite 1
  `FrameItem::Line`.
- Sem coordenação com `flush_line` quando wrap ocorre.

**Pós-P286**:
- **+1 campo opcional no Layouter**:
  `decoration_lines_collector: Option<Vec<DecoSegment>>` (None
  por default).
- **+1 struct privada** `DecoSegment { start_x, end_x, baseline_y }`
  (3 fields `Pt`; `Copy`).
- **+5 LOC condicional em `flush_line`** (`cursor.rs:89+`): hook
  que regista segment **antes** do drain quando collector activo
  + had_items.
- **~22 LOC adicionais no consumer P284**: snapshot inicial +
  activa collector + recurse no body + drena vec + acrescenta
  segment final não-flushed + loop emite 1 Line por segment.

### §2.2 — Cobertura de mudança (per diagnóstico §A.1)

| Local | Tipo | Mudança |
|---|---|---|
| `01_core/src/engine/layout/mod.rs:243+` | extensão struct Layouter | +1 campo `decoration_lines_collector` |
| `01_core/src/engine/layout/mod.rs:250+` | nova struct privada | `DecoSegment { start_x, end_x, baseline_y }` |
| `01_core/src/engine/layout/mod.rs:391+` | constructor | +1 init line (None) |
| `01_core/src/engine/layout/mod.rs:1987+` | **consumer P284 reescrito** | algoritmo wrap-aware + fallback single-line bit-exact |
| `01_core/src/engine/layout/cursor.rs:89+` | hook em `flush_line` | +5 LOC condicional (collector → push segment) |
| `00_nucleo/prompts/engine/layout.md` | L0 cluster | +33 LOC seção "Decoração textual wrap-aware (P286)" |

Total: **5 sítios L1** tocados (3 produção + 1 cursor.rs + 1 L0).
Zero ficheiros tocados em L3/L4. **Hash `export.rs` preservado**
porque o emit PDF é estructuralmente igual.

### §2.3 — Algoritmo P286 (consumer)

```rust
Content::Underline { body, stroke, offset, extent }
| Content::Strike   { body, stroke, offset, extent }
| Content::Overline { body, stroke, offset, extent } => {
    // [...] cálculo kind_em, offset_pt, extent_pt, thickness, color
    let color = stroke.or(self.style.fill);             // P285 §A.3

    // P286 — snapshot + activa collector.
    let start_x_initial    = self.regions.current.cursor_x;
    let baseline_y_initial = self.regions.current.cursor_y;
    let prev_collector     = self.decoration_lines_collector.take();
    self.decoration_lines_collector = Some(Vec::new());

    self.layout_content(body);

    let mut segments = self.decoration_lines_collector
        .take().unwrap_or_default();
    self.decoration_lines_collector = prev_collector;

    // Patch primeiro segment (body pode começar mid-linha).
    if let Some(first) = segments.first_mut() {
        first.start_x    = start_x_initial;
        first.baseline_y = baseline_y_initial;
    }
    // Segment final não-flushed.
    let final_end_x = self.regions.current.cursor_x;
    let final_start_x = if segments.is_empty() { start_x_initial }
                       else { self.regions.current.line_start_x };
    if final_end_x.val() > final_start_x.val() {
        segments.push(DecoSegment { /* ... */ });
    }

    // Emite 1 Line por segment + extent simétrico (§A.3 α).
    for seg in &segments {
        self.regions.current.current_line.push(FrameItem::Line {
            start: Point { x: Pt(seg.start_x.val() - extent_pt), y: line_y },
            end:   Point { x: Pt(seg.end_x.val()   + extent_pt), y: line_y },
            thickness, color,
        });
    }
}
```

### §2.4 — Hook em `flush_line` (cursor.rs)

```rust
pub(super) fn flush_line(&mut self) {
    let had_items = !self.regions.current.current_line.is_empty();

    // P286 — hook decorações wrap-aware.
    if had_items {
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            coll.push(super::DecoSegment {
                start_x:    self.regions.current.line_start_x,
                end_x:      self.regions.current.cursor_x,
                baseline_y: self.regions.current.cursor_y,
            });
        }
    }
    // [...] resto do flush_line original inalterado
}
```

**Zero overhead nos call-sites pré-existentes**: collector default
`None` → o `if let Some(coll)` é dead-code optimizado pelo branch
predictor; apenas decorações P284 activam o hook localmente.

---

## §3 — Fase A — decisões registadas (`diagnostico-deco-multiline-passo-286.md`)

### §3.1 — A.1 inventário `flush_line` empírico

`grep -rn "fn flush_line\|cursor_x\|current_line" 01_core/src/engine/layout/`
mapeou 30+ hits. Achados decisivos:

| Achado | Implicação |
|---|---|
| `flush_line` definido em `cursor.rs:89-129` | Single source of truth |
| `Region` em `region.rs:43` **não tem campo `history`** | A.2 (a) estructuralmente bloqueada |
| Items são `.drain()` para `current_items` sem marcar fronteiras | Não há reconstrução pós-hoc viável |
| `cursor_y` avança **depois** do drain | Snapshot `cursor_y` antes do advance preserva baseline correcto da linha que fechou |
| `flush_line` chamado por wrap natural (`layout_word:71,79`) **e** por barreiras estruturais (Heading/Block) | Só wrap natural conta — body inline-only não dispara barreiras |

### §3.2 — A.2 estratégia de captura

**Decidido**: opção **(b) variante minimalista** — vec inline em
vez de callback Fn boxed.

| Opção | Veredicto |
|---|---|
| (a) snapshot `Region.history` | ❌ Bloqueada (`Region` não tem history; adicionar seria mais intrusivo que b) |
| (b) callback Fn boxed | ⚠ Sugerida pela spec mas overhead desnecessário; refutado por evolução para vec inline drenado |
| (b') **vec inline drenado** | ✅ Escolhida — `Option<Vec<DecoSegment>>` no Layouter; hook trivial em flush_line; consumer drena no fim |
| (c) iteração manual | ❌ Replica lógica de wrap (P144 hyphenation); alto risco de divergir |

**Refinamento sobre a spec**: a spec descrevia (b) como "callback
registado". A inspecção mostrou que vec inline é **estruturalmente
equivalente** com menos overhead (sem boxing, sem closure capture).
Refutação pragmática de pressuposto da spec — padrão P285 §8.3
inaugural agora atinge **N=2 cumulativo** na vida do cluster
decorações (P285 refutou opção emergente em A.2 export.rs; P286
refutou pressuposto sobre estrutura de captura).

**Consequência crítica**: `region.md` **não precisa de mudar**
porque a alteração é em `Layouter`, não em `Region`. Critério §4
"Hash L0 `region.md` muda **se** A.2 → (b)" interpretado em sentido
estrito (ver §5.1).

### §3.3 — A.3 política de extent multi-line

**Decidido**: opção **(α)** — extent simétrico em **todas** as N
linhas.

| Sinal | Decisão |
|---|---|
| Vanilla `painter/render.rs::paint_decoration_chunk` aplica extent simétrico por chunk visual | (α) é paridade vanilla |
| Visualmente consistente (cada linha "respira" igual) | spec §A.3 default |
| Simétrico à regra single-line P284 (extent aplicado uma vez) | preserva semântica original |
| Implementação trivial (aplica no loop sem casos especiais) | minimiza touch points |

Opções (β) "apenas primeira e última" e (γ) "apenas última"
rejeitadas como divergentes de vanilla sem benefício prático.

### §3.4 — Riscos mitigados

| Risco §7 spec | Status | Mitigação |
|---|---|---|
| A.2 (a) bloqueada → forçar (b) intrusivo | ✅ Atenuado | Refinamento para (b') vec inline minimiza intrusão (+1 campo opcional + 5 LOC; sem boxing) |
| Granularidade flush ≠ linha visual | ✅ Refutado | A.1.4 confirmou que wrap natural cobre 100% do caso body inline-only |
| Regressão bit-exact single-line | ✅ Coberto | Fallback explícito + teste regression dedicado |

---

## §4 — Testes adicionados (+7)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `01_core/src/engine/layout/tests.rs` (`p284_decoration_tests`) | 6 | Single-line regression (1 Line); body longo → N≥2 Lines; Y distintos por linha; cor uniforme per-decoração; strike+overline paridade simétrica; extent aplicado a TODAS as linhas (+8pt por linha em ambos os lados) |
| `03_infra/src/export.rs` (`tests`) | 1 | PDF integration — N operadores `l S Q` separados + N matches `0.000 0.000 1.000 RG` (cor uniforme em todas as linhas) |
| **Total** | **7** | dentro do alvo spec (+6 a +12) |

**Resultado**: 7/7 verdes (`cargo test --lib p286` em ambos
`typst-core` e `typst-infra`).

---

## §5 — Observações pragmáticas

### §5.1 — `region.md` preservado (refinamento decisão A.2)

A spec §4 dizia: *"Hash L0 `region.md` muda **se** A.2 → (b)"*. A
decisão A.2 foi efectivamente (b), mas no refinamento "(b') vec
inline" o estado de captura vive no **Layouter struct**, não em
`Region`. Consequentemente, `region.md` **não precisa de mudar** —
e não mudou.

Esta divergência face à expectativa literal da spec é um **ganho
arquitectural não previsto**: menos ficheiros L0 tocados, menos
hashes propagados, menos drift cross-camada. Documentado em
diagnóstico §A.2 como "consequência crítica" da escolha (b').

### §5.2 — Hash `export.rs` preservado (Win arquitectural P281+P285 N=3)

`bc7b8b95` (P281) → `66cb8ac3` (P285) → **`66cb8ac3` (P286
preserved)**. A modificação P286 é **inteiramente no L1
Layouter** — o emit PDF é estructuralmente idêntico, apenas
processa N Lines em vez de 1.

Este é o **3º passo cumulativo** a confirmar o win arquitectural
P281 ("single source of truth como invariante anti-bug"):
- N=1: P282 §1.1 (auditoria empírica paridade local vs top-level).
- N=2: P285 §8.2 (alteração simétrica em emit via helper único).
- **N=3: P286 §5.2** (alteração em L1 sem necessidade de tocar L3).

Atinge limiar histórico N=3 — padrão "single source of truth como
invariante anti-bug" registado §8.2 abaixo para promoção a ADR
meta candidato.

### §5.3 — Fallback single-line bit-exact por construção

Se o collector ficar vazio (body cabe na linha actual sem disparar
`flush_line`), o algoritmo P286 colapsa para:
1. `segments = []` após drain.
2. `final_end_x > final_start_x` → push 1 segment final
   `{ start_x_initial, cursor_x_final, baseline_y_initial }`.
3. Loop emite 1 `FrameItem::Line` com X início = `start_x_initial`,
   X fim = `cursor_x_final` — **idêntico ao algoritmo P284 original**.

Validado por teste explícito `p286_underline_single_line_emite_uma_linha_regression_p285`
+ regressão dos 2 725 testes pré-P286 (zero breakages).

### §5.4 — Cluster decorações P284-P285-P286 COMPLETO

| Defeito graded original | Estado |
|---|---|
| P284 §5.3 multi-line wrap | ✅ **RESOLVIDO P286** |
| P284 §5.4 stroke parseado mas inerte | ✅ RESOLVIDO P285 |
| `evade` (descender skipping) | ⏸ Scope-out ADR-0054 graded (continua) |
| `background` (z-order) | ⏸ Scope-out ADR-0054 graded (continua) |
| Objecto `Stroke` rico | ⏸ Scope-out (Tabela A.7 linha 201; passo dedicado) |

Os 3 itens em scope-out continuam **conscientes e documentados**
per ADR-0054 graded — não são defeitos, são decisões de escopo.

### §5.5 — Padrão "activação posterior de feature graded" N=2 cumulativo

P285 §8.3 inaugurou o padrão "activação posterior de feature
parseada-mas-inerte" — `stroke` parseado em P284 ficou activo via
adição de `FrameItem::Line.color` em P285.

P286 aplica o mesmo padrão à outra dimensão (wrap em vez de
stroke): o consumer P284 era escrito como se single-line fosse a
única possibilidade; P286 activa a feature wrap **sem revisitar
P284** — apenas estende o Layouter (campo opcional + hook) e
substitui o algoritmo do consumer.

**N=2 cumulativo** na vida do cluster decorações:
- N=1: P285 (stroke → cor activa).
- N=2: P286 (single-line → wrap-aware).

Padrão demonstra robustez: pendências graded podem ser resolvidas
incrementalmente sem refactor de fundo. Aguardar N≥3 (próximo
passo aplicar padrão a área ortogonal) para formalizar como ADR
meta.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~45 (+1 campo struct +5 LOC init +5 LOC hook flush_line +~22 LOC reescrita consumer +~12 LOC struct DecoSegment) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado) |
| LOC L0 modificado | ~150 (`layout.md` +33; diagnóstico cobertura +60 footnote ⁷²; diagnóstico A.1-A.3 +~55) |
| Testes adicionados | 7 (6 L1 wrap-aware + 1 L3 PDF integration) |
| Testes baseline P285 | 2 725 preserved bit-exact |
| Testes pós-P286 | **2 732** |
| Hash L0 `layout.md` | propagado (`...` → `12536b5c`; 11 ficheiros do cluster layout) |
| Hash L0 `export.rs` | **`66cb8ac3` preserved** (sem mudança L3) |
| Hash L0 `region.md` | inalterado (refinamento A.2 → b' vs b literal) |
| Lint | zero violations |
| Variants Content | 63 (inalterado) |
| Variants FrameItem | 6 (inalterado) |
| Campos `Layouter` | N → **N+1** (+`decoration_lines_collector`) |
| Structs privadas novas | **+1** (`DecoSegment`) |
| Pendências resolvidas | **1** (P284 §5.3) |
| Cluster decorações P284-P285-P286 | **COMPLETO** (zero defeitos graded remanescentes) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `DecoSegment` é struct com 3
  campos `Pt` (`Copy`); `decoration_lines_collector` é
  `Option<Vec<DecoSegment>>` — tipos L1 puros, sem I/O, sem state
  global mutável (vec é state local do Layouter, não global).
- ✅ **ADR-0054 scope graded**: `evade`/`background`/multi-line
  wrap registados como scope-out **conscientes** em diagnósticos
  P284/P286; multi-line agora resolvido (P286), outros 2 continuam
  scope-out documentado.
- ✅ **ADR-0061 padrão Layout Fase 2**: P286 estende cluster
  decorações P284 sem promover ADR nova — alteração confinada ao
  consumer existente; backward-compat estricta.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  inventário literal de `flush_line` (grep + leitura linha-a-linha)
  + decisão A.2 emergente derivada de observação empírica (não
  pressuposto).
- ✅ **ADR-0085 diagnóstico imutável**: `diagnostico-deco-multiline-passo-286.md`
  produzido com 3 secções A.1-A.3 + métricas + risco residual
  mitigado.
- ✅ **Anti-padrão over-formalização P273.17 §0**: zero ADR nova;
  2 padrões emergentes registados §8 (1 atinge N=3 limiar candidato
  mas **não** promovido neste passo).
- ✅ **Honestidade epistémica**: divergência face à spec literal
  (`region.md` não muda) **documentada explicitamente** em §5.1
  como ganho arquitectural não previsto; refutação pragmática do
  pressuposto da spec sobre tipo de captura A.2 documentada em
  §3.2.
- ✅ **Win arquitectural P281 N=3 cumulativo**: `export.rs`
  preservado bit-exact; alteração inteiramente em L1 sem necessidade
  de tocar L3 — confirma que o investimento P281 (helpers únicos +
  single source of truth) continua a pagar dividendos no terceiro
  passo cumulativo.
- ✅ **Bit-exact regression validada**: teste dedicado
  `p286_underline_single_line_emite_uma_linha_regression_p285` +
  2 725 testes pré-P286 preserved.

---

## §8 — Padrões emergentes (sem formalização ADR)

### §8.1 — "Activação posterior de feature graded" — N=2 cumulativo

- N=1: **P285 §8.3** (`stroke` parseado em P284 → activo via
  adição de `FrameItem::Line.color`).
- **N=2**: **P286** (consumer P284 single-line → wrap-aware via
  adição de `decoration_lines_collector` + hook em `flush_line`).

Características partilhadas:
- Resolve pendência graded registada em passo anterior **sem
  revisitar** esse passo.
- Extensão de tipo/Layouter com campo opcional (`None`/`null`
  default) preserva backward-compat trivialmente.
- Bit-exact regression validada por teste dedicado.

Reaplicações candidatas: futuras pendências graded em P156
(`evade` em decorações; floating headers; rich `Stroke` object).
Aguardar N≥3 (próximo passo aplicar padrão a área ortogonal) para
formalizar como ADR meta.

### §8.2 — "Win arquitectural single source of truth como invariante anti-bug" — **N=3 cumulativo (limiar histórico atingido)**

- N=1: **P282 §1.1** (auditoria empírica paridade local vs
  top-level pós-P281; refutou 6/6 suspeitas).
- N=2: **P285 §8.2** (alteração em emit propaga simetricamente
  top-level + local via mesmo helper `line_rg_prefix`).
- **N=3**: **P286 §5.2** (alteração em L1 Layouter sem necessidade
  de tocar L3 export — hash `export.rs` preservado bit-exact pelo
  3º passo consecutivo).

Padrão: "single source of truth como invariante anti-bug" — o
investimento arquitectural P281 (helpers únicos + unificação
top-level/local) **automaticamente** elimina classes de bugs
latents e reduz blast radius de futuras alterações.

**Limiar histórico N=3 atingido** — candidato natural a formalização
em ADR meta no próximo passo onde o padrão for citado. Não é
objectivo de P286 promover; apenas registar.

### §8.3 — "Refutação pragmática de pressuposto da spec via inspecção empírica" — N=3 cumulativo

- N=1: **P285 §A.2** (3 opções a/b/c na spec partiam de pressuposto
  falso que `export.rs` precisava de helper novo; inspecção revelou
  que `FrameItem::Line` pré-existente bastava).
- N=2: **P286 §A.2** primeira refutação (opção (a) snapshot history
  estructuralmente bloqueada porque `Region` não tem campo
  `history`; reconhecida pré-implementação via inspecção literal).
- **N=3**: **P286 §A.2** segunda refutação (opção (b) descrita
  como "callback Fn boxed" refinada para "vec inline drenado" —
  estruturalmente equivalente com menos overhead).

Padrão emergente: spec ≠ verdade — diagnóstico empírico pode
redefinir o espaço de opções **e os pressupostos sobre cada opção**.
Aguardar N≥4 (passo distinto, fora do cluster decorações) para
considerar formalização.

---

## §9 — Próximos passos sugeridos (estado pós-P286)

Cobertura agregada estimada: inalterada (~64%). P286 fecha o último
defeito graded do cluster decorações sem alterar contagem
user-facing — Text features (linha 434) continuam em 10/5/1/5/2 =
23 (paridade pós-P284, agora **completa** vs vanilla nos 3
variants).

### Rank 1-3: continuar quick wins horizontais

1. **`P-smartquote`** (S; Text 43% → 48%) — typographic smart
   quotes; reaplica padrão "variant rico com cosméticos opcionais"
   (N=4 cumulativo pós-P284) e activa potencial promoção ADR meta
   se atingir N=5.
2. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%) — Accent +
   Cancel primitives.
3. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b) — Curve
   geometry primitive.

### Rank 4-6: features médias

4. **`P-footnote-cluster`** (M; Model 60% → 70%).
5. **`P-outline-cluster`** (M; Introspection 70% → 80%).
6. **`P-math-op-lr`** (M; Math 40% → 60%).

### Rank 7: refino opcional pós-P286

7. **`P-deco-evade`** (M-L; activa `evade: bool` em
   Underline/Overline — geometria glifo-a-glifo via shaping
   results). Não-objectivo P286 §5; aguarda materialização de
   shaping real (DEBT-53 candidato).

### Promoção candidata (potencial gatilho)

8. **ADR meta "Win arquitectural single source of truth como
   invariante anti-bug"** — **N=3 limiar atingido** (P282 + P285 +
   P286). Próximo passo que cite o padrão dispara formalização
   natural. Não é objectivo de P286 promover.

---

## §10 — Referências cross-passos

- **P38** — Math fraction line introduziu `FrameItem::Line`.
- **P78** — Line shape primitive; segunda aplicação de
  `FrameItem::Line`.
- **P102** — `text.fill` introduziu `rg`/`RG` em `FrameItem::Text`.
- **P144** — Hyphenation em `layout_word` (estendeu mecanismo de
  flush; P286 lê este código pós-P144 para garantir compatibilidade).
- **P216A/B/C** — Field-aggregation Region/Regions (precedente
  arquitectural para "campo opcional ao Layouter struct" — P286
  segue o mesmo padrão).
- **P281** — Unificação β-completa de stream-builders;
  single-source-of-truth em emit.
- **P282 §1.1, §1.5** — auditoria empírica paridade local vs
  top-level (N=1 do padrão §8.2).
- **P284** — Cluster decorações P-text-deco-emit (3 variants ricos);
  §5.3 multi-line wrap registado como pendência graded (resolvido
  por **P286**); §5.4 stroke inerte (resolvido por P285).
- **P285** — `FrameItem::Line.color` + emit `RG` + herança
  `style.fill`; activou stroke inerte de P284 (N=1 do padrão §8.1;
  N=2 do padrão §8.2).
- **ADR-0029** — pureza física L1 (`DecoSegment` tipo L1 puro).
- **ADR-0054** — scope graded (`evade`/`background`/`Stroke` rico
  continuam scope-out consciente).
- **ADR-0061** — Layout roadmap (cluster decorações P284-P285-P286
  agora completo).
- **ADR-0065** — inventariar-primeiro (Fase A obrigatória cumprida
  com inspecção literal de `flush_line`).
- **ADR-0085** — diagnóstico imutável (40º consumo: P286 + P285 +
  P284 + P282 + 36 anteriores).

---

*P286 fecha a frente `P-text-deco-multiline` e marca o **cluster
decorações P284-P285-P286 COMPLETO** — zero defeitos graded
remanescentes. Modificação cirúrgica em L1 (+1 campo opcional no
Layouter + 5 LOC em `flush_line` + ~22 LOC no consumer P284); zero
impacto em L3 export (hash `export.rs 66cb8ac3` preservado pelo 3º
passo consecutivo, confirmando padrão "single source of truth como
invariante anti-bug" — limiar histórico N=3 atingido). Refinamento
pragmático sobre a spec: opção A.2 (b) "callback Fn boxed" foi
refinada para (b') "vec inline drenado" — estruturalmente
equivalente sem overhead de boxing; ganho não previsto: `region.md`
não precisa de mudar. Bit-exact regression validada (single-line
emite 1 Line idêntica a P284); 7 testes P286 verdes; baseline
2 725 → 2 732 (+7). Padrão "activação posterior de feature graded"
inaugurado P285 atinge N=2 cumulativo na vida do cluster
decorações.*
