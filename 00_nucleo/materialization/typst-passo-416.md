# P416 — Nota de Rodapé Real: `Footnote` body renderizado no rodapé (M)

**Título**: Footnote nota rodapé real — de marker-only para body renderizado no rodapé  
**Tipo**: Materialização (M) — consumer Layouter + infraestrutura de rodapé  
**Bloqueadores**: Nenhum externo; pré-condições internas verificáveis  
**Referências**: P295 (Fase 1 marker-only), ADR-0113 (honestidade epistêmica), P156E (`Pagebreak`), P243 (`Regions`)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 6 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: todos os 6 produzem output conforme esperado. Se qualquer um falhar → **parar imediatamente** e reclassificar o passo.

```bash
# 1. Content::Footnote existe no enum?
grep -n "Footnote" 01_core/src/entities/content.rs

# 2. Layouter tem footnote_counter (marker-only P295)?
grep -n "footnote_counter" 01_core/src/rules/layout/mod.rs

# 3. new_page e finish existem no Layouter?
grep -n "fn new_page\|fn finish" 01_core/src/rules/layout/mod.rs

# 4. Body do footnote é armazenado (Box<Content>)?
grep -n "body: Box<Content>" 01_core/src/entities/content.rs | grep -i footnote

# 5. Regions struct existe (P243 infraestrutura multi-region)?
grep -n "struct Regions" 01_core/src/entities/region.rs

# 6. FrameItem::Group com clip_mask existe (P242 infraestrutura)?
grep -n "clip_mask" 01_core/src/entities/layout_types.rs
```

**Output esperado**:
1. ≥1 hit em `content.rs` com `Footnote { body }`
2. ≥1 hit em `layout/mod.rs` com `footnote_counter: u32` ou similar
3. ≥2 hits (`fn new_page` e `fn finish`)
4. ≥1 hit confirmando `body: Box<Content>` no variant Footnote
5. ≥1 hit com `struct Regions`
6. ≥1 hit com `clip_mask: Option<ShapeKind>`

**Se (1) ou (4) falhar** → P295 não foi feito; reclassificar para XL (reconstruir footnote do zero).  
**Se (2) falhar** → marker-only não existe; reclassificar para L (infraestrutura de contador ausente).  
**Se (3) falhar** → pipeline de página não existe; reclassificar para XL.  
**Se (5) ou (6) falhar** → infraestrutura de região/clip ausente; reclassificar para L (reabrir P243/P242).

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/content.md` + `layout.md`):

### A.1.1 — Decisão arquitetural: honestidade epistêmica ADR-0113

P295 (Fase 1) aplicou **stub transparente** per ADR-0113: o elemento existia no pipeline, mas o consumer emitia apenas `[N]` superscript inline.  
P416 **promove** o stub para semantic real: o body do footnote é renderizado no rodapé da página onde o marker aparece.

**Princípio**: não há fallback software — o body é renderizado realmente, ou o footnote continua como marker-only. Não implementar "quase funciona" (ex.: body renderizado inline após o parágrafo).

### A.1.2 — Estratégia de implementação (2 opções)

| Opção | Descrição | Magnitude | Risco |
|-------|-----------|-----------|-------|
| **α** — Rodapé como região separada | `Regions` ganha `footer: Option<Region>`; Layouter reserva espaço no fundo da página; body renderizado em `footer` via `layout_sub_frame_with_width` | L | Alto — refactor Regions + cursor.y tracking + collision com floats (P245) |
| **β** — Rodapé como deferred frame | `Layouter` acumula `Vec<DeferredFootnote>`; em `finish()` / `new_page()`, emite `FrameItem::Group` com items do rodapé posicionados no fundo da página atual | M | Baixo — reusa pattern P245 (`DeferredFloat`) + P251 (`DeferredCellTail`) |

**Decisão recomendada**: **Opção β** — pattern "DeferredX buffer + flush em ponto canônico" já validado em P245 (floats) e P251 (cell tails). Paridade arquitetural cristalina; zero refactor de `Regions`.

### A.1.3 — Estrutura de dados

```rust
// Novo struct local no Layouter (paridade P245 DeferredFloat)
struct DeferredFootnote {
    number: u32,
    body_items: Vec<FrameItem>,
    body_height: f64,
    body_width: f64,
}
```

**Layouter +2 fields**:
- `footnotes_pending: Vec<DeferredFootnote>` — buffer de notas pendentes na página atual
- `footnote_area_height: f64` — espaço reservado no fundo da página (cumulativo)

### A.1.4 — Algoritmo de renderização

1. **Marker inline** (preservado P295): `Content::Footnote { body }` → emite `[N]` superscript via `footnote_counter`
2. **Body capture**: durante `layout_content` do Footnote, `layout_sub_frame_with_width(body, max_width)` captura items + altura
3. **Buffer push**: `DeferredFootnote { number, body_items, body_height, body_width }` → `footnotes_pending`
4. **Flush em `finish()` e `new_page()`** (paridade P245):
   - `cursor_y_bottom_reserve` já existe (P245) — reusa ou estende
   - Posiciona cada footnote no fundo: `target_y = page_h - margin - footnote_area_height - accumulated`
   - Separação entre notas: `FOOTNOTE_SEP_LENGTH` (~0.5em)
   - Linha separadora opcional: `FrameItem::Line` horizontal (scope-out ADR-0054 graded — pode ser adiada)
5. **Reset**: `footnotes_pending.clear()` + `footnote_area_height = 0.0` em `new_page`

### A.1.5 — Paridade vanilla

- Vanilla Typst: footnote body renderizado no rodapé da página do marker; numeração contínua por documento; linha separadora 0.5pt acima do primeiro footnote da página
- Cristalino P416: paridade estrutural (body no rodapé, numeração contínua); linha separadora scope-out (adiada para P416.X se necessário)

### A.1.6 — Scope-out explícito

- `numbering` cosmético customizado (continua `u32` sequencial; pattern customizado tipo `[a]`, `[*]` — scope-out)
- `FootnoteBody::Reference(Label)` (P295 scope-out) — continua scope-out
- Linha separadora decorativa — scope-out ADR-0054 graded (pode ser adicionada em P416.X sem breaking change)
- Multi-column footnotes (quando `columns` ativo) — scope-out; renderiza na página principal

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `content.md` + `layout.md`
2. Sonda A.0 produziu 6/6 OK
3. Decisão β validada (pattern DeferredX reutilizável)

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities (zero alteração)

`Content::Footnote { body: Box<Content> }` já existe (P295). Nenhum variant novo. Nenhum tipo novo.

### B.2 — Layouter (`rules/layout/mod.rs`)

1. **Adicionar `DeferredFootnote` struct local** (paridade P245 `DeferredFloat`)
2. **Adicionar 2 fields ao Layouter**:
   - `footnotes_pending: Vec<DeferredFootnote>`
   - `footnote_area_height: f64` (default 0.0)
3. **Modificar arm `Content::Footnote`** (~50 LOC):
   - Preservar marker inline: emite `[N]` superscript (código P295 existente)
   - Adicionar body capture: `layout_sub_frame_with_width(body, ...)` → captura items/height
   - Push ao buffer
   - Atualizar `footnote_area_height += body_height + sep`
4. **Adicionar `flush_pending_footnotes()` method** (~40 LOC):
   - Paridade P245 `flush_pending_floats`
   - Posiciona no fundo da página
   - Emite `FrameItem::Group` por footnote (ou items directamente)
   - Reset buffer
5. **Modificar `new_page()` e `finish()`**:
   - Chamar `flush_pending_footnotes()` antes de commit da página
   - Reset `footnote_area_height` em `new_page`

### B.3 — Consumer math/overflow

- **Collision com floats (P245)**: `cursor_y_bottom_reserve` já existe para floats bottom. Footnotes competem pelo mesmo espaço. Decisão: footnotes têm precedência sobre floats bottom (paridade vanilla aproximada); ou empilham (floats bottom acima de footnotes). **Recomendação**: footnotes no fundo absoluto; floats bottom acima de footnotes.
- **Overflow de página**: se footnote body é maior que página inteira, emitir em página própria (paridade vanilla "overlong footnote"). Implementação: se `body_height > safe_available`, forçar `new_page()` antes do marker.

### B.4 — Tests

**Mínimo 12 tests**:
- 3 unit: `DeferredFootnote` struct + partial_eq + clone
- 4 unit Layouter: marker inline preservado + body capture + flush vazio + flush com 1 nota
- 3 E2E layout: 1 footnote na página + 2 footnotes empilhadas + footnote em página 2
- 2 E2E regression: backward compat P295 (marker-only sem body real quando layout falha) + float+footnote collision

---

## FASE C — Validação

```bash
crystalline-lint .
# → 0 drift
# → 0 violations
# → tests: +12 verdes
# → workspace total: <baseline> + 12 verdes
```

**Critério de fecho**:
- [ ] 12 tests verdes
- [ ] Lint zero
- [ ] Marker inline `[N]` preservado (backward compat P295)
- [ ] Body renderizado no rodapé da página correta
- [ ] Numeração contínua por documento
- [ ] `footnote_area_height` resetado por página
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Padrão reutilizado**: "DeferredX buffer + flush em ponto canônico" — P245 (floats) + P251 (cell tails) + **P416 (footnotes)**. N=3 cumulativo atinge limiar formalização.
- **Honestidade epistêmica**: P295 foi stub transparente (body armazenado, não renderizado). P416 é a **promoção real** scope-out → semantic concreta, paridade P242/P245/P248.
- **Sem alteração ao enum Content** — evolução por adição pura no consumer (ADR-0113 §stub).
- **Próximo passo P417**: `Selector::Where` (A, infraestrutura) ou `bibliography/cite` CSL (C, XL) ou `repr()` completo (D, S).
