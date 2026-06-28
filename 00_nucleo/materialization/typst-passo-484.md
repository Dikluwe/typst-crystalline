---

# P484 — Trilha 5 Fase 3: RTL básico (`unicode-bidi`) + remoção de `FrameItem::Text`

> **Passo:** 484
> **Data:** 2026-06-28
> **Foco:** (A) RTL básico — adicionar `unicode-bidi` ao workspace; re-order de runs bidirectionais antes do shaping; (B) Remover `FrameItem::Text` (agora deprecated desde P483) depois de RTL estabilizado.
> **Trilha:** 5 — Shaping / rustybuzz — Fase 3.
> **Tipo:** Materialização L.
> **Tamanho:** L (~3–4h).
> **ADR-0120 ACEITE** (P482). **ADR-0029 EM VIGOR** — L1 sem I/O. **ADR-0039 EM VIGOR** — TextStyle struct resolvido.

---

## Contexto

P482 e P483 fecharam as Fases 1 e 2 do épico de shaping:
- `FrameItem::TextShaped` existe e é produzido pelo shaper.
- `shaper.rs` está integrado no pipeline entre layout e export.
- `export/stream.rs` usa `TextShaped` como path primário.
- `FrameItem::Text` está deprecated e serve apenas como fallback.

A Fase 3 fecha dois itens em aberto:

**Sub-item A — RTL básico:** `unicode-bidi` está ausente do workspace (confirmado P481). `rustybuzz::UnicodeBuffer::guess_segment_properties()` detecta automaticamente LTR/RTL via Unicode Bidirectional Algorithm, mas não re-ordena os runs — o texto árabe/hebraico seria shaped correctamente ao nível do glifo mas apresentado na ordem visual errada. Este sub-item adiciona `unicode-bidi` e implementa re-order de runs antes de `shape()`.

**Sub-item B — Remoção de `FrameItem::Text`:** com RTL a funcionar e shaping cobrindo ≥95% do texto (P483), `FrameItem::Text` pode ser removido do enum. Os ~16 ficheiros com `#![allow(deprecated)]` são actualizados; o arm de fallback em export é substituído por emissão directa de `TextShaped`.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `unicode-bidi` disponível no workspace? | `Cargo.toml` raiz; `03_infra/Cargo.toml` | ✅ ausente (P481) |
| Versão `unicode-bidi` compatível com Rust edition 2021? | crates.io `unicode-bidi` latest | 🟡 |
| `rustybuzz::UnicodeBuffer::set_direction(Direction::RightToLeft)` existe? | doc rustybuzz 0.20 | 🟡 |
| `UnicodeBuffer::push_str` preserva ordem visual vs lógica? | doc rustybuzz | 🟡 |
| `unicode_bidi::BidiInfo::new(text, None)` disponível? | doc unicode-bidi | 🟡 |
| `BidiInfo::reorder_visual(para, level, range)` disponível? | idem | 🟡 |
| Sites de construção de `FrameItem::Text` que ainda existem após P483? | grep `FrameItem::Text {` em todos os ficheiros | 🟡 |
| Ficheiros com `#![allow(deprecated)]` após P483 | listados no relatório P483 (16 ficheiros) | ✅ |
| Testes que constroem `FrameItem::Text` directamente | `export/tests.rs`, `integration_tests.rs` | ✅ (P483) |

**Sondas 🟡 com `grep`/`file:line` antes de qualquer código.**

---

## Sub-item A — RTL básico

### A.1 — Adicionar `unicode-bidi` ao workspace

**Ficheiro:** `Cargo.toml` raiz (workspace dependencies)

```toml
[workspace.dependencies]
# ... existentes ...
unicode-bidi = "0.3"   # P484 — bidi algorithm para re-order de runs RTL
```

**Ficheiro:** `03_infra/Cargo.toml`

```toml
[dependencies]
unicode-bidi = { workspace = true }
```

**Nota:** verificar versão exacta no crates.io antes de escrever. `unicode-bidi 0.3.x` é o ramo estável. Se `lab/typst-original` usa uma versão diferente, verificar compatibilidade.

### A.2 — Pipeline de runs bidirectionais em `shaper.rs`

O texto latino LTR é um único run. Texto misto (ex: "Hello مرحبا world") tem múltiplos runs com direcções diferentes. `unicode-bidi` resolve os runs e a sua ordem visual.

```rust
use unicode_bidi::{BidiInfo, Level};

/// **P484** — Divide texto em runs bidirectionais e retorna-os na ordem visual.
/// Para texto puramente LTR, retorna um único run com o texto original.
fn bidi_runs(text: &str) -> Vec<BidiRun> {
    let bidi = BidiInfo::new(text, None);   // None = auto-detect base direction
    let para = &bidi.paragraphs[0];
    let line  = para.range.clone();

    bidi.reorder_line(para, line)
        .into_iter()
        .map(|range| BidiRun {
            text:  text[range.clone()].to_owned(),
            rtl:   bidi.levels[range.start] == Level::rtl(),
        })
        .collect()
}

struct BidiRun {
    text: String,
    rtl:  bool,
}
```

### A.3 — Integração em `try_shape`

**Ficheiro:** `03_infra/src/shaper.rs` — função `try_shape`

Antes de chamar `rustybuzz::shape()`, dividir o texto em runs:

```rust
fn try_shape(text: &str, rb_face: &RbFace) -> Vec<ShapedGlyph> {
    let runs = bidi_runs(text);
    let mut all_glyphs = Vec::new();

    for run in runs {
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(&run.text);
        if run.rtl {
            buffer.set_direction(rustybuzz::Direction::RightToLeft);
        } else {
            buffer.set_direction(rustybuzz::Direction::LeftToRight);
        }
        // NÃO chamar guess_segment_properties() — direcção já foi definida
        let output = rustybuzz::shape(rb_face, &[], buffer);
        // ... construir ShapedGlyph como antes ...
        all_glyphs.extend(glyphs_from_output(&output, &run.text));
    }

    all_glyphs
}
```

**Nota sobre direcção:** se `run.rtl = true`, a ordem dos glifos no `GlyphBuffer` já está na ordem correcta de renderização (da direita para a esquerda). O `emit_shaped_pdf` itera os glyphs na ordem do Vec — para RTL isso é correcto sem inversão adicional.

### A.4 — Fallback para textos sem runs RTL

Para texto puramente LTR (a maioria dos documentos), `bidi_runs` retorna um único run. O overhead de `BidiInfo::new` é O(n) mas aceitável dado que shaping já é O(n) com rustybuzz.

### A.5 — Testes RTL

- **L3:** `p484_bidi_runs_ltr_unico_run` — texto inglês → 1 run, `rtl = false`.
- **L3:** `p484_bidi_runs_rtl_arabico` — texto árabe → 1 run, `rtl = true`.
- **L3:** `p484_bidi_runs_misto_dois_runs` — "Hello مرحبا" → 2 runs com direcções diferentes.
- **L3:** `p484_try_shape_rtl_nao_panic` — shape de texto árabe sem fonte → sem panic (fallback Text).

---

## Sub-item B — Remoção de `FrameItem::Text`

### B.1 — Condição de remoção

A remoção só é segura se:
1. RTL estabilizado (sub-item A).
2. Nenhum teste constrói `FrameItem::Text` directamente (ou é actualizado).
3. Todos os emit sites em L1 (`cursor.rs`, `list_item.rs`, etc.) são migrados para `FrameItem::TextShaped` directo **ou** permanece claro que o shaper os converterá.

**Decisão importante:** os emit sites de `cursor.rs` (e outros) continuam a emitir `FrameItem::Text` — o shaper em L3 é quem converte. A remoção de `FrameItem::Text` do enum implicaria que esses sites emitam `FrameItem::TextShaped` directamente de L1 — o que viola ADR-0029/ADR-0030 (L1 sem acesso a bytes de fonte).

**Conclusão da análise:** a remoção completa de `FrameItem::Text` requer que os emit sites em L1 sejam migrados para um novo tipo neutral (`FrameItem::TextRaw` ou `FrameItem::TextPending`) que não é `deprecated` mas que o shaper converte para `TextShaped`. Esta é uma mudança arquitectural maior do que o esperado.

**Decisão para P484:** **Não remover `FrameItem::Text` neste passo.** Em vez disso:
- Sub-item B passa a ser: **renomear** `FrameItem::Text` para `FrameItem::TextRaw` e remover `#[deprecated]` — explicitando que é um tipo "pré-shaping" válido, não um tipo obsoleto.
- OU: preservar `#[deprecated]` e manter a remoção para um épico de arquitectura separado.

**Recomendação:** preservar `#[deprecated]` e não renomear neste passo. A complexidade de migrar os emit sites L1 sem violar ADR-0029 requer uma ADR nova antes de avançar. Sub-item B de P484 é reduzido a: remover `#![allow(deprecated)]` dos ficheiros que **apenas** fazem match (não constroem) e podem usar `TextShaped` exclusivamente — i.e., `export/stream.rs` remove o arm `Text` do path primário (já feito em P483 como arm secundário; P484 pode simplesmente removê-lo se shaper garantir 100% de cobertura).

### B.2 — Acção reduzida de B em P484

**Se shaper garante cobertura total** (toda fonte carregada → TextShaped; fallback só para fontes ausentes):

Remover o arm `#[allow(deprecated)] FrameItem::Text { .. }` de `export/stream.rs` path principal:

```rust
// ANTES (P483):
FrameItem::TextShaped { .. } => { /* path primário */ }
#[allow(deprecated)]
FrameItem::Text { .. } => { /* fallback */ }

// DEPOIS (P484) — se cobertura total:
FrameItem::TextShaped { .. } => { /* único path */ }
// Text preservado como fallback APENAS em modo de erro/debug
```

**Se cobertura não é total** (fontes do sistema podem falhar): manter arm `Text` em export. Sub-item B fica como documentação da razão de preservação.

A sonda deve verificar em que condições `resolve_slot` retorna `None` em produção.

---

## Spec L0

### Sub-item A

- `infra/shaper.md` — §P484: `bidi_runs`, `BidiRun`, `try_shape` alargada com RTL.
- ADR-0120 — anotação P484: Fase 3 RTL executada.

### Sub-item B (acção reduzida)

- ADR-0120 — anotação P484: decisão de preservar `FrameItem::Text` como tipo de pré-shaping; justificação ADR-0029.
- `entities/layout_types.md` — actualização: `FrameItem::Text` é tipo de pré-shaping válido; `#[deprecated]` é sinalização de não usar directamente em L3+; arquitectura permanente pós-P484.

---

## Scope-out explícito

- **Remoção completa de `FrameItem::Text`** — requer ADR nova (colisão com ADR-0029). Scope-out declarado permanente até revisão arquitectural.
- **OpenType features explícitas** (liga, kern, smcp) — `features = &[]` usa padrão da fonte; feature knobs são P485+.
- **Bidi nível de parágrafo múltiplo** — `BidiInfo.paragraphs[0]` assume um único parágrafo; textos com múltiplos parágrafos são tratados como um único bloco. Suficiente para o caso de uso real (shaper actua palavra a palavra).
- **Texto vertical (CJK rotated)** — scope-out.
- **Fallback de fonte multi-família** — shaping usa apenas a fonte primária da `FontList`. P485+.
- **Corpus RTL no lab/parity** — sem ficheiros de corpus árabe/hebraico; testes unitários L3 cobrem o comportamento.

---

## Critério de fecho

- [ ] Sondas: versão `unicode-bidi`; `BidiInfo::new` API confirmada; `UnicodeBuffer::set_direction` disponível — todos com `file:line`.
- [ ] `unicode-bidi` adicionado ao workspace e a `03_infra/Cargo.toml`.
- [ ] `bidi_runs(text) -> Vec<BidiRun>` implementado em `shaper.rs`.
- [ ] `try_shape` usa `bidi_runs` antes de `rustybuzz::shape()`.
- [ ] `set_direction(RTL)` usado para runs RTL.
- [ ] 4 testes RTL verdes (sub-item A).
- [ ] Decisão de sub-item B documentada e justificada (preservar `Text` ou reduzir arm).
- [ ] ADR-0120 anotada com Fase 3.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 5: Fase 3 FECHADA** (RTL básico; `Text` preservado como tipo de pré-shaping).

---

## Próximo passo (P485)

Com Fase 3 fechada:

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P485-A** | `emit_shaped_pdf` com `x_advance` em pt — posicionamento preciso | M |
| **P485-B** | OpenType features: `liga` (ligatures), `kern` (kerning explícito) | M |
| **Trilha 6** | Fechar a 5ª funcionalidade pendente de Trilha 6 | M |

---

## Estado pós-P483 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| ADR-0120 | ACEITE — Fases 1 + 2 executadas |
| Trilha 5 | Fase 1 ✅; Fase 2 ✅; **Fase 3 em preparação** |
| `FrameItem::Text` | `#[deprecated]` — fallback ativo |
| **P484** | Trilha 5 Fase 3 — RTL + decisão remoção Text | 🔄 EM PREPARAÇÃO |
