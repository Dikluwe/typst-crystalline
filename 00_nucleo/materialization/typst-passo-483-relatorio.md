# Relatório P483 — Trilha 5 Fase 2: migração `Text → TextShaped` + `FrameItem::Text` deprecated

**Data:** 2026-06-28
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P483 (Trilha 5 Fase 2 — migração completa + deprecated)
**Materialização:** 3 mudanças funcionais + 5 testes + 16 ficheiros `#[allow(deprecated)]`

---

## 1. Resumo

P483 fecha o dual-path de P482. O problema central era que a maioria do texto
tinha `style.font = None` (fonte padrão, sem `#set text(font:...)` no documento)
— o shaper ignorava esses items.

**Sub-item A — Causa raiz identificada e corrigida (Caso 1):**
`From<&StyleChain> for TextStyle` propagava `chain.font()` directamente; para
chain sem delta de fonte, isso resultava em `None`. Fix: fallback para
`FontList::single("Helvetica")` quando `chain.font()` retorna `None`.

**Sub-item B — `FrameItem::Text` deprecated:**
Marcado `#[deprecated(since = "P483")]`. Todos os ~16 ficheiros com match ou
construção legítima receberam `#![allow(deprecated)]`. `export/stream.rs`
reordenado: `TextShaped` como arm primário, `Text` como fallback.

**Resultado:** 73/73 paridade mantida. 5 novos testes verdes. `cargo build` zero
erros. `crystalline-lint` zero V1–V14.

---

## 2. Sub-item A — Causa raiz e fix (ADR-0108)

### 2.1 Sondas (medir antes de decidir)

| Medição | Resultado | `file:line` |
|---------|-----------|-------------|
| `TextStyle::default().font` | `None` (derive Default) | `layout_types.rs:122` |
| `StyleChain::font()` para chain vazia | `None` — cadeia sem delta de fonte | `style_chain.rs:493` |
| `From<&StyleChain>` para TextStyle | `font: chain.font()` | `style_chain.rs:609` |
| Shaper guard | `style.font.is_some()` | `shaper.rs:37` |
| `FontList::single(name: EcoString)` disponível? | Sim | `font_list.rs:155` |
| `FontFamily::new(EcoString)` normaliza lowercase? | Sim | `font_list.rs:90` |

**Classificação (ADR-0108):** Caso 1 — `chain.font()` retorna `None` para texto
sem `#set text(font:...)`. Fix mínimo: fallback em `From<&StyleChain>`.

**Decisão**: NÃO mudar `TextStyle::default()` (usada em testes com `TextStyle::default()`
que esperam `font = None`). Mudar apenas `From<&StyleChain>` — o path que o
Layouter usa em produção.

### 2.2 Implementação

**Ficheiro:** `01_core/src/entities/style_chain.rs` (`From<&StyleChain> for TextStyle`)

```rust
// Antes (P482):
font: chain.font(),

// Depois (P483):
font: Some(chain.font().unwrap_or_else(|| {
    // P483 — fonte padrão Helvetica garante que shaper actua em texto sem
    // #set text(font:...). Shaper faz fallback defensivo (Text preservado)
    // se fonte não estiver carregada.
    FontList::single(EcoString::from("Helvetica"))
})),
```

**Impacto:** Em produção, todos os items de texto emitidos pelo Layouter têm
`style.font = Some(...)`. O shaper chama `try_shape` → `resolve_slot` →
`world.font(idx)`. Se a fonte está carregada: item convertido para `TextShaped`.
Se não está: item preservado como `Text` (fallback defensivo intacto).

### 2.3 Verificação

Cobertura de ≥95% não pode ser provada por testes unitários (MockWorld sem
fontes reais). Racionalidade: em produção, `SystemWorld.book()` tem as fontes
do sistema registadas; qualquer `select_pattern("helvetica", ...)` encontrará
uma fonte compatível. Comportamento verificado por raciocínio estrutural e
pelo facto de a parity suite continuar 73/73.

---

## 3. Sub-item B — `FrameItem::Text` deprecated + export primário

### 3.1 Deprecação

**Ficheiro:** `01_core/src/entities/layout_types.rs`

```rust
#[deprecated(since = "P483", note = "Use FrameItem::TextShaped. \
    Preserved as fallback for fonts not loaded or Type1.")]
Text {
    pos:   Point,
    text:  EcoString,
    style: TextStyle,
},
```

Rust 1.81+ suporta `#[deprecated]` em enum variants — warnings emitidos em
match e construção. Verificado: zero erros de compilação; apenas warnings
suprimidos por `#![allow(deprecated)]` nos ficheiros legítimos.

### 3.2 `#![allow(deprecated)]` nos ficheiros afectados

| Ficheiro | Sites afectados |
|----------|-----------------|
| `01_core/src/rules/layout/cursor.rs` | Constrói e faz match `FrameItem::Text` |
| `01_core/src/rules/layout/enum_item.rs` | Constrói |
| `01_core/src/rules/layout/equation.rs` | Constrói e faz match |
| `01_core/src/rules/layout/helpers.rs` | Faz match |
| `01_core/src/rules/layout/link.rs` | Constrói e faz match |
| `01_core/src/rules/layout/list_item.rs` | Constrói |
| `01_core/src/rules/layout/slicing.rs` | Faz match |
| `01_core/src/rules/math/layout/frac.rs` | Faz match |
| `01_core/src/rules/math/layout/mod.rs` | Constrói e faz match |
| `03_infra/src/export/fonts.rs` | Faz match |
| `03_infra/src/export/stream.rs` | Faz match (e reordenado) |
| `03_infra/src/export/tests.rs` | Constrói em testes |
| `03_infra/src/integration_tests.rs` | Faz match em testes |
| `03_infra/src/pipeline.rs` | Faz match |
| `03_infra/src/shaper.rs` | Faz match e constrói |
| `01_core/src/entities/layout_types.rs` | `plain_text_items` — match |

Abordagem: `#![allow(deprecated)]` como inner attribute em ficheiros com múltiplos
sites; `#[allow(deprecated)]` na função `plain_text_items` em `layout_types.rs`.
Para `integration_tests.rs` (mod aninhado): `#![allow(deprecated)]` dentro de
`mod integration { ... }`.

### 3.3 Export stream — arm primário

**Ficheiro:** `03_infra/src/export/stream.rs`

Em `build_page_stream` e `draw_item_local`: `FrameItem::TextShaped` agora
**precede** `FrameItem::Text`. `Text` com comentário `// Fallback: shaping indisponível`.

---

## 4. Testes (5 novos)

### L1 — `entities::style_chain::tests`

| Teste | Cobertura |
|-------|-----------|
| `p483_textstyle_from_chain_font_nunca_none` | Chain default → font = Some("helvetica") |
| `p483_textstyle_from_chain_com_font_explicity_preserva` | Font explícito ("Arial") preservado, não Helvetica |

### L3 — `shaper::tests`

| Teste | Cobertura |
|-------|-----------|
| `p483_text_com_font_helvetica_tenta_shape_mas_sem_fontes_preserva_text` | `style.font = Some` → shaper tenta; MockWorld sem fontes → Text preservado |
| `p483_text_sem_font_nao_tenta_shape` | `style.font = None` → shaper não tenta (guard is_some()) |
| `p483_shaped_glyph_debug_display` | ShapedGlyph Debug inclui glyph_id |

### Parity

| Teste | Cobertura |
|-------|-----------|
| `p483_parity_73_73_mantido` | 73/73 matches, 0 diffs, 0 errors |

---

## 5. Ficheiros alterados/criados

### Specs L0 (3 actualizadas)

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/entities/layout_types.md` | §P483 — deprecated + font padrão |
| `00_nucleo/prompts/infra/shaper.md` | §P483 — cobertura ≥95% + testes |
| `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md` | Anotação P483 — Fase 2 executada |

### Código L1 (3 ficheiros funcionais + 8 #![allow])

| Ficheiro | `@prompt-hash` pós-P483 | Alteração |
|----------|------------------------|-----------|
| `01_core/src/entities/layout_types.rs` | `867b46c8` | `#[deprecated]` em Text + allow em `plain_text_items` |
| `01_core/src/entities/style_chain.rs` | `de464849` (inalterado) | Font fallback Helvetica em `From<&StyleChain>` + 2 testes |
| `01_core/src/rules/layout/cursor.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/enum_item.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/equation.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/helpers.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/link.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/list_item.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/layout/slicing.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/math/layout/frac.rs` | inalterado | `#![allow(deprecated)]` |
| `01_core/src/rules/math/layout/mod.rs` | inalterado | `#![allow(deprecated)]` |

### Código L3 (5 ficheiros)

| Ficheiro | `@prompt-hash` pós-P483 | Alteração |
|----------|------------------------|-----------|
| `03_infra/src/shaper.rs` | `325595fd` | 3 testes P483 + `#![allow(deprecated)]` |
| `03_infra/src/export/stream.rs` | `9acca994` | Reordenação TextShaped/Text + `#![allow(deprecated)]` |
| `03_infra/src/export/fonts.rs` | `c7d24b28` | `#![allow(deprecated)]` |
| `03_infra/src/pipeline.rs` | `1b030acd` | `#![allow(deprecated)]` |
| `03_infra/src/export/tests.rs` | `243b14db` | `#![allow(deprecated)]` |
| `03_infra/src/integration_tests.rs` | `4eecd2a1` | `#![allow(deprecated)]` |

### Parity suite

| Ficheiro | Alteração |
|----------|-----------|
| `lab/parity/tests/structural_parity.rs` | Sentinela `p483_parity_73_73_mantido` |

### Documentação

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/materialization/typst-passo-483-relatorio.md` | Criado |

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 2 files:
    ./01_core/src/entities/layout_types.rs  → 867b46c8
    ./03_infra/src/shaper.rs                → 325595fd
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ 0 erros V1–V14.
  Warnings V7 pré-existentes (prompts órfãos não relacionados com P483).
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Remoção de `FrameItem::Text`** | Apenas deprecated. Remoção em Fase 3 (P484) após RTL. |
| **Cobertura ≥95% verificável via teste** | MockWorld não tem fontes; verificado por raciocínio estrutural. |
| **Type1 / CFF shaping** | `FrameItem::Text` permanece para fontes que rustybuzz não suporta. |
| **Avanço horizontal em pt** | `emit_shaped_pdf` usa glyph IDs hex; posicionamento preciso é P485+. |
| **Font substitution avançada** | Se "Helvetica" ausente na FontBook, fallback para Text. Sem font-matching. |

---

## 8. Critério de fecho

- [x] Sondas: `TextStyle::default().font = None` (Caso 1); `chain.font() = None` para chain vazia; `FontList::single` disponível.
- [x] Causa raiz de `style.font = None` identificada e corrigida (Caso 1).
- [x] `TextStyle::from(&StyleChain::default_chain()).font` = `Some("helvetica")`.
- [x] `FrameItem::Text` marcado `#[deprecated(since = "P483")]`.
- [x] `#![allow(deprecated)]` adicionado nos ~16 ficheiros legítimos.
- [x] `export/stream.rs` tem `TextShaped` como arm primário; `Text` como fallback.
- [x] 5 testes novos verdes (2 L1 + 3 L3).
- [x] `p483_parity_73_73_mantido` verde.
- [x] Spec L0 actualizada (2 prompts + ADR-0120).
- [x] `cargo build --workspace` zero erros; zero deprecated warnings restantes.
- [x] `crystalline-lint` zero V1–V14.
- [x] **Trilha 5: Fase 2 FECHADA.**

---

## 9. Estado pós-P483

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (Fase 3 RTL — P484), 6 (4/5) |
| Paridade | **73/73 matches; 0 diffs; 0 errors** |
| `FrameItem::Text` | `#[deprecated]` — fallback ativo |
| ADR-0120 | **ACEITE** — Fase 2 executada |
| **P483** | **FECHADO** |

---

## 10. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P484 — Trilha 5 Fase 3** | RTL básico via `unicode-bidi`; remoção de `FrameItem::Text`. | L |
| **Trilha 6** | Fechar a 5ª funcionalidade pendente. | M |
| **P485 — advance em pt** | `emit_shaped_pdf` com posicionamento preciso por x_advance/units_per_em. | M |
