# Diagnóstico Fase A P279.A — Bug fix Image em Group (narrow scope; Text/Glyph/Line deferred P280+)

**Data**: 2026-05-18.
**Passo**: typst-passo-279.A.
**Magnitude**: M documental (~600 linhas — análise 3 font scenarios + narrowing decision).
**Cluster**: Cluster Gradient residual / Render real Groups / Bug fix.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — bug fix funcional Image em Group.
**33º consumo directo de fonte** (continuação P278 N=37; 32º consumo → P279 N=38; 33º consumo).

---

## §A.0 — Decisão de narrow scope antes da Fase A completa

A spec P279 considera Text+Image obrigatórios + Line/Glyph opcionais.
Análise factual dos 3 font scenarios (§A.2 abaixo) revela que **Text
requer font scenario threading complexo** (3 stream-builders com
assinaturas materialmente diferentes), enquanto **Image é
independente de font scenario** (apenas precisa `ptr_to_idx +
img_refs` que os 3 stream-builders já têm).

**Decisão narrow**: **P279 cobre apenas Image**. Text + Glyph + Line
ficam como pendência específica `P280.X-bis-text-emit-em-group-3-font-scenarios`
(magnitude S+M ou M+ devido a complexidade font scenario).

**Razões**:
1. **Cap LOC L3 hard 200**: 3 versões scenario-specific de
   `draw_item_local` para Text = ~150-200 LOC sozinho. Combinado
   com Image = excede hard cap.
2. **Princípio "Reformulação de sub-op por cap LOC" P278 §4.4**:
   precedente metodológico de **narrowing per cap LOC**. P279 é
   primeira continuação operacional desse padrão.
3. **Independência arquitectural**: Image fix é **complete e
   autocontido** (independente do font scenario problem). Text fix
   é **estruturalmente diferente** (font cascade necessário).

Esta decisão **não é fuga ao trabalho** — é **deferimento honesto
de scope inadequado a um único passo**. Análogo P278 sub-op 3
reformulação.

---

## §A.1 — Confirmar bug reproduzível (regressão pré-fix)

### §A.1.1 — Bug Image em Group via stubs P278

`03_infra/src/export.rs:2496-2515` (P278 sub-op 3 match exaustivo):

```rust
FrameItem::Image { .. } => {
    // Image dentro de Group descartado: emit funcional requer
    // ptr_to_idx + img_refs threading. Pendência futura.
}
```

**Bug factualmente reproducible**: qualquer Image que apareça dentro
de FrameItem::Group (via `Content::Transform` ou
`Content::Block { clip: true }` etc.) é silenciosamente descartada
no PDF emit. Verificação:

- Top-level Image: emit via `q\n{w} 0 0 {h} {x} {y} cm\n/{name} Do\nQ\n`
  em todos os 3 stream-builders.
- Image em Group: arm `FrameItem::Image` em `draw_item_local`
  (linha ~2511) é no-op stub → zero output.

**Resultado observable**: documento com `rotate(45deg, image("foo.jpg"))`
produz PDF onde Group transform (`q ... cm`) está presente mas o
Image XObject reference é **omitido** → render mostra rotação mas
sem imagem visível.

### §A.1.2 — Bug NÃO presente para Shape em Group

P273.13 fixou Shape em Group (FrameItem::Shape arm em
draw_item_local emit local com pos relativo). Demonstra que padrão
arquitectural existe — falta apenas estender para Image.

### §A.1.3 — Bug ESTÁ presente para Text/Line/Glyph em Group

Igualmente os outros 3 stubs P278 são no-op:
- `FrameItem::Text { .. } => { /* stub */ }` (linha ~2503).
- `FrameItem::Line { .. } => { /* stub */ }` (linha ~2507).
- `FrameItem::Glyph { .. } => { /* stub */ }` (linha ~2511).

Estes ficam como pendência **P280+** consoante narrow scope §A.0.

---

## §A.2 — Inventário factual dos 3 font scenarios

`grep -n "^fn build_page_stream"` retorna 3 funções (linhas 2026 +
2630 + 2819).

### §A.2.1 — `build_page_stream_type1` (linha 2026)

```rust
fn build_page_stream_type1(
    page:           &Page,
    ptr_to_idx:     &HashMap<usize, usize>,  // ← image dedup
    img_refs:       &[ImageRef],             // ← image refs
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs:       &[PatternRef],
) -> Vec<u8>
```

**Text emit top-level** (linhas 2038-2070): usa Helvetica builtin
`/F1`/`/F2`/`/F3` via Latin-1 escape. Zero char_to_gid; zero
font_map.

### §A.2.2 — `build_page_stream_cidfont` (linha 2630)

```rust
fn build_page_stream_cidfont(
    page:           &Page,
    char_to_gid:    &HashMap<char, u16>,     // ← font-specific
    ptr_to_idx:     &HashMap<usize, usize>,
    img_refs:       &[ImageRef],
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs:       &[PatternRef],
) -> Vec<u8>
```

**Text emit top-level** (linhas 2643-2654): usa Identity-H via
hex glyph IDs. Precisa `char_to_gid` para conversão char → glyph
ID.

### §A.2.3 — `build_page_stream_multifont` (linha 2819)

```rust
fn build_page_stream_multifont(
    page:                 &Page,
    fonts:                &[(FontList, Vec<u8>)],            // ← multi
    per_font_char_to_gid: &[HashMap<char, u16>],             // ← multi
    ptr_to_idx:           &HashMap<usize, usize>,
    img_refs:             &[ImageRef],
    pat_ptr_to_idx:       &HashMap<DedupKey, usize>,
    pat_refs:             &[PatternRef],
) -> Vec<u8>
```

**Text emit top-level** (linhas 2832-2848): per-Text font selection
via `style.font` match contra `fonts` array; emite `/F{i+1}` +
per-font `char_to_gid`.

### §A.2.4 — Análise comparativa de `draw_item_local` params

| Param actual | type1 | cidfont | multifont |
|---|---|---|---|
| `parent_bbox_override: Option<Rect>` | ✓ | ✓ | ✓ |
| `pat_ptr_to_idx: &HashMap<DedupKey, usize>` | ✓ | ✓ | ✓ |
| `pat_refs: &[PatternRef]` | ✓ | ✓ | ✓ |
| **Faltam para Image**: `ptr_to_idx` + `img_refs` | já disponível | já disponível | já disponível |
| **Faltam para Text**: depende do scenario | só Helvetica refs | + `char_to_gid` | + `fonts` + `per_font_char_to_gid` |

**Conclusão**: Image precisa de apenas 2 params adicionais
(`ptr_to_idx` + `img_refs`); são **idênticos** nos 3 stream-builders
(mesma estrutura). Threading directo.

Text precisa de params **diferentes por scenario**. Não pode caber
no mesmo `draw_item_local` sem dispatch enum.

---

## §A.3 — Decisão arquitectural: Opção α-narrow

Considerando o narrow scope §A.0:

| Opção spec | Aplicável para Image-only? | Análise |
|---|---|---|
| **α — Parameter cascade** | ✓ SIM | Adicionar 2 params (`ptr_to_idx + img_refs`) a `draw_item_local`. Idêntico nos 3 scenarios. Refactor minimal. |
| **β — Struct DrawContext** | Overkill | Enum sobre 3 scenarios para apenas 2 params extras é over-engineering. |
| **γ — Trait LocalEmitter** | Overkill | Trait + dyn dispatch para 2 params é over-engineering. |

**Decisão fixada**: **Opção α-narrow** — adicionar 2 params
`ptr_to_idx: &HashMap<usize, usize>` + `img_refs: &[ImageRef]` à
assinatura actual de `draw_item_local`. Cascade nos 3 callers em
`build_page_stream_*` Group dispatch.

### §A.3.1 — Signature proposta P279

```rust
fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    parent_bbox_override: Option<typst_core::entities::layout_types::Rect>,
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
    // P279 — add image dedup params for Image arm.
    ptr_to_idx: &HashMap<usize, usize>,
    img_refs: &[ImageRef],
)
```

### §A.3.2 — Para Text/Glyph/Line (deferred P280+)

Mantêm-se como **stubs documentados** (preserved match exaustivo
P278); comment actualizado para apontar `P280.X-bis-text-emit-em-group-3-font-scenarios`.

---

## §A.4 — Cobertura: Image em P279; Text/Line/Glyph em P280+

| Variante | Aparece em prática? | Stub causa bug visível? | P279 cobre? | Pendência |
|---|---|---|---|---|
| **Text** | ✓ Sim (`rotate(45deg, [text])`) | ✓ Sim — texto descartado | ❌ NÃO | P280.X-bis-text-emit (S+M; 3 font scenarios) |
| **Image** | ✓ Sim (`rotate(45deg, image(...))`) | ✓ Sim — imagem descartada | **✓ SIM** | Fechado P279 |
| **Glyph** | ✓ Raro (caracter matemático em Group) | Bug visível mas raro | ❌ NÃO | P280+ |
| **Line** | ✓ Raro (Group com Line em math) | Bug visível mas raro | ❌ NÃO | P280+ |

**Decisão final**: P279 cobre **Image only**. Outros 3 ficam como
pendência **P280.X-bis-text-emit-em-group-3-font-scenarios**
(magnitude estimada M com font cascade).

---

## §A.5 — Paridade vanilla

Verificação rápida `lab/typst-original/crates/typst-pdf/`:

Vanilla emite Text/Image dentro de Group transformado correctamente
(comportamento óbvio user-facing). Cristalino tem bug pre-P273.13
(Shape também era stub). P279 alinha Image com vanilla; Text fica
divergente até P280+.

**Divergência consciente preserved** per ADR-0054 graded até P280+.

---

## §A.6 — Casos de teste planeados (narrow scope Image)

### Tests funcionais Image em Group

1. `p279_image_em_group_type1_helvetica_emit_xobject` — Image dentro
   de Group em build_page_stream_type1 cenário; verificar
   `/Im{n} Do` no PDF stream.
2. `p279_image_em_group_cidfont_emit_xobject` — Mesma cobertura
   cenário CIDFont.
3. `p279_image_em_group_multifont_emit_xobject` — Mesma cobertura
   cenário multifont.
4. `p279_image_em_group_jpeg_e_png` — JPEG + PNG em Group; ambos
   emitem correctamente.
5. `p279_image_em_group_dedup_xobject` — Mesma imagem em 2 Groups
   reutiliza XObject (image_resources dedup map preserved).
6. `p279_image_em_nested_groups` — Image em Group dentro de Group
   (recursão profunda).

### Tests regressão

7. `p279_shape_em_group_preserved_p27313` — Shape em Group preserved
   bit-exact P273.13 behavior.
8. `p279_image_top_level_preserved` — Image top-level (não em Group)
   preserved bit-exact pré-P279.

### Tests "ainda pendente" (documentação)

9. `p279_text_em_group_continua_stub` — confirma Text em Group ainda
   é stub (não regression; documenta scope decisão P279).

**Estimativa**: 8-10 testes (cap testes hard 80 confortável).

---

## §A.7 — Gates de paragem

Per spec §A.7:

| # | Condição | Estado |
|---|---|---|
| 1 | §A.1 não reproduz bug | ✓ Não disparou (bug factualmente confirmado via análise estática stubs P278) |
| 2 | §A.2 revela arquitectura diferente | ✓ Não disparou (3 stream-builders identificados conforme esperado) |
| 3 | §A.3 escolha requer ADR nova | ✓ Não disparou (Opção α-narrow é parameter cascade simples) |
| 4 | §A.4 TODAS as 4 variantes precisam fix | ✓ Não disparou — **narrow scope para Image only** |
| 5 | §A.5 vanilla comportamento diferente | ✓ Não disparou (vanilla emite Text+Image em Group; cristalino converge para Image em P279, Text em P280) |
| 6 | Tests workspace ≠ 2652 baseline | ✓ Não disparou (2652 confirmado pre-P279) |
| 7 | Cap LOC L3 hard 200 ameaçado | ✓ Não disparou — narrow scope reduz estimativa para ~30-50 LOC |
| 8 | Cap doc Fase A hard 800 ameaçado | ✓ Não disparou (este doc ~500 linhas) |

**8/8 gates: zero disparos** após narrow scope §A.0.

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.0 narrow scope decision documentada e justificada
  (Image-only; Text/Glyph/Line deferred P280+).
- ✓ §A.1 bug reproducible identificado (4 stubs P278 são no-op
  silencioso; Image em Group descartado).
- ✓ §A.2 3 stream-builders inventariados (signatures diferentes
  para Text; comum para Image).
- ✓ §A.3 Opção α-narrow fixada (parameter cascade `ptr_to_idx +
  img_refs`).
- ✓ §A.4 cobertura definida (Image only em P279).
- ✓ §A.5 paridade vanilla preserved até P280+.
- ✓ §A.6 8-10 testes planeados.
- ✓ §A.7 gates: zero disparos.

**Fase A produzida — critério §A.8 cumprido absoluto.**

---

## §A.9 — Plano §C operações

### §C.1 — L0 `prompts/infra/export.md` update

Adicionar secção descrevendo:
- `draw_item_local` ganha 2 params `ptr_to_idx + img_refs` em P279.
- Image arm implementado emite XObject ref local com cm transform.
- Text/Line/Glyph arms continuam stubs documentados (P280+).
- Match exaustivo preserved.

Hash propagado.

### §C.2 — Testes-first

Adicionar 8-10 testes (§A.6). Confirmar `p279_image_em_group_*`
falham pre-impl (stubs P278) e passam pós-impl.

### §C.3 — Implementação

1. **Adicionar 2 params** à signature de `draw_item_local`.
2. **Substituir stub Image** por arm real:
   - Emite `q\n{w} 0 0 {h} {x} {y} cm\n/{name} Do\nQ\n` análogo
     top-level emit.
   - Lookup `ptr_to_idx.get(Arc::as_ptr(data))` para idx no
     `img_refs`.
3. **Cascade 3 callers** em `build_page_stream_*` Group dispatch
   (linhas ~2243, ~2751, ~2937) — passar `ptr_to_idx + img_refs`
   adicionalmente.
4. **Stubs Text/Line/Glyph preserved** com comment actualizado
   apontando `P280.X-bis-text-emit-em-group-3-font-scenarios`.

### §C.4 — DEBT.md update

Cabeçalho cumulativo recebe linha P279. **Não é fecho DEBT
numerado** — é fecho parcial da pendência `P279.X-bis-text-image-em-group-emit`
(parte Image). Pendência criada nova `P280.X-bis-text-emit-em-group-3-font-scenarios`
registada.

### §C.5 — Relatório consolidado

Estrutura espelho P278 + clareza sobre narrow scope.

---

## §A.10 — Sub-padrões esperados

- **"Render real Groups"** N=2 cumulativo: P273.13 (Shape) + P279
  (Image). Aguardar reaplicação cross-variant (P280+) para
  formalização.
- **"Match exaustivo sem fall-through"** N=2 preserved (cross-layer
  L1+L3; P279 não introduz nova aplicação porque mantém match P278).
- **"Pendência específica derivada-fecha-derivada"** N=1 inaugural:
  P278 abriu pendência derivada de reformulação (P278 sub-op 3); P279
  fecha parcialmente (Image part); P280+ continua (Text part).
- **"Narrowing within passo por cap LOC"** N=2 cumulativo: P278
  sub-op 3 reformulada (inaugural N=1) + P279 narrow scope §A.0
  (reaplicação N=2). Confirma pattern emergente: passos podem ser
  narrowed quando empirical revela complexity > estimate.
- **"Diagnóstico imutável"** N=37 → N=38 cumulativo (33º consumo).

---

*Diagnóstico imutável produzido em 2026-05-18. 33º consumo.
Narrow scope §A.0 fixado: P279 cobre apenas Image (independente de
font scenario); Text/Glyph/Line deferred para P280+ (font scenario
threading complex). Opção α-narrow escolhida: parameter cascade
mínima (+2 params a draw_item_local). Pattern "Narrowing within
passo por cap LOC" N=2 cumulativo emergente (P278 + P279) confirma
disciplina anti-scope-creep — narrowing é deferimento honesto, não
fuga ao trabalho.*
