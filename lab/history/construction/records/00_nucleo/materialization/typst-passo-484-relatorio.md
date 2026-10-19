# Relatório P484 — Trilha 5 Fase 3: RTL básico via unicode-bidi

**Data:** 2026-06-28
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P484 (Trilha 5 Fase 3: RTL + decisão remoção FrameItem::Text)
**Materialização:** unicode-bidi no workspace + bidi_runs em shaper.rs + decisão de preservação de FrameItem::Text

---

## 1. Resumo

**Sub-item A — RTL básico:**
`unicode-bidi = "0.3"` adicionado ao `[workspace.dependencies]` e a
`03_infra/Cargo.toml`. `shaper.rs` recebe `BidiRun` + `bidi_runs(text)`
usando `BidiInfo::visual_runs`. `try_shape` substituí
`buffer.guess_segment_properties()` por iteração de runs com
`set_direction(Direction::LeftToRight | RightToLeft)`.

**Sub-item B — Decisão de remoção de `FrameItem::Text`:**
Remoção **adiada indefinidamente**. Os emit sites de L1 (`cursor.rs`,
`list_item.rs`, etc.) emitem `FrameItem::Text`; migrar estes sites para emitir
`FrameItem::TextShaped` directamente violaria ADR-0029 (bytes de fonte em L3).
`FrameItem::Text` é o tipo de pré-shaping permanente da arquitectura cristalina.

**Resultado final:** 6 testes novos verdes; 73/73 paridade mantida;
`crystalline-lint` 0 erros V1–V14. **Trilha 5 COMPLETA.**

---

## 2. Sub-item A — RTL básico (ADR-0108)

### 2.1 Sondas com `file:line`

| Pergunta | Resultado | `file:line` |
|----------|-----------|-------------|
| `unicode-bidi` no workspace cristalino pré-P484 | **Ausente** | `Cargo.toml` (confirmado P481) |
| Versão em `lab/typst-original` | `"0.3.18"` | `lab/typst-original/Cargo.toml.original:1` |
| `rustybuzz::Direction` variantes | `LeftToRight`, `RightToLeft` (+ TopToBottom, BottomToTop) | `lab/krilla-reference/crates/krilla/src/text/shape.rs:20-23` |
| API `BidiInfo::reorder_line` (hipótese) | Retorna `Cow<str>` — **incompatível com Vec<Range>** | Erro de compilação E0599 |
| API `BidiInfo::visual_runs` (real) | `(Vec<Level>, Vec<LevelRun>)` onde `LevelRun = Range<usize>` | `lab/typst-original/crates/typst-layout/src/inline/line.rs:273` |
| `Level::is_rtl()` disponível | ✅ | `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:21` |
| `buffer.set_direction(Direction::RightToLeft)` disponível | ✅ | `lab/krilla-reference/crates/krilla/src/text/shape.rs:21` |

### 2.2 Decisão de API

A hipótese inicial (`reorder_line` → `Vec<Range>`) estava errada — `reorder_line`
retorna `Cow<str>` (texto já reordenado). A API correcta para obter ranges com
levels é `BidiInfo::visual_runs(para, line) -> (Vec<Level>, Vec<LevelRun>)`.

Fonte: `lab/typst-original/crates/typst-layout/src/inline/line.rs:273`.

### 2.3 Implementação

**`Cargo.toml` workspace** (`file:20`):
```toml
unicode-bidi = "0.3"   # P484 — bidi algorithm para re-order de runs RTL
```

**`03_infra/Cargo.toml`** (`file:26`):
```toml
unicode-bidi = { workspace = true }  # P484 — re-order de runs RTL
```

**`03_infra/src/shaper.rs`** — `BidiRun` + `bidi_runs` + `try_shape` actualizado:

```rust
struct BidiRun {
    text:       String,
    rtl:        bool,
    byte_start: usize,  // offset byte no string original
}

fn bidi_runs(text: &str) -> Vec<BidiRun> {
    if text.is_empty() { return vec![]; }
    let bidi = BidiInfo::new(text, None);
    if bidi.paragraphs.is_empty() {
        return vec![BidiRun { text: text.to_owned(), rtl: false, byte_start: 0 }];
    }
    let para           = &bidi.paragraphs[0];
    let (levels, runs) = bidi.visual_runs(para, para.range.clone());
    runs.into_iter().map(|run_range| {
        let rtl = levels.get(run_range.start)
            .map(|l: &unicode_bidi::Level| l.is_rtl()).unwrap_or(false);
        BidiRun { text: text[run_range.clone()].to_owned(), rtl, byte_start: run_range.start }
    }).collect()
}
```

`try_shape` substitui `guess_segment_properties()` por:
```rust
for run in bidi_runs(text.as_str()) {
    buffer.set_direction(if run.rtl { Direction::RightToLeft }
                         else       { Direction::LeftToRight });
    // ... shape + abs_cluster = run.byte_start + info.cluster
}
```

---

## 3. Sub-item B — Decisão de preservação de `FrameItem::Text`

### 3.1 Análise (ADR-0108)

| Questão | Resultado |
|---------|-----------|
| Sites de emit de `FrameItem::Text` em L1 | `cursor.rs:86`, `equation.rs:75+149`, `list_item.rs:25`, `enum_item.rs:33`, `link.rs:123`, `math/layout/mod.rs:535` |
| Possibilidade de emitir `TextShaped` em L1 | **Impossível** — bytes de fonte vivem em L3 (ADR-0029/ADR-0030) |
| Alternativa: tipo intermediário `TextRaw`/`TextPending` | Exige ADR nova (renomeação + ripple em todos os ~19 sites) |
| Custo de remover `Text` via ADR nova | L magnitude — não justificado neste passo |

### 3.2 Decisão

`FrameItem::Text` permanece como tipo de **pré-shaping** da arquitectura:

- Emitido por L1 (cursor, equation, list, link, math).
- Convertido para `TextShaped` pelo shaper L3 em pipeline.
- `#[deprecated]` (P483) mantido como aviso de não usar directamente em L3+.
- Remoção futura requer ADR nova antes de avançar.

**Zero código adicional** neste sub-item — apenas documentação na ADR-0120 e
`prompts/entities/layout_types.md`.

---

## 4. Testes (6 novos)

| Teste | Ficheiro | Resultado |
|-------|----------|-----------|
| `p484_bidi_runs_ltr_unico_run` | `shaper.rs` | ✅ ok |
| `p484_bidi_runs_vazio_zero_runs` | `shaper.rs` | ✅ ok |
| `p484_bidi_runs_arabico_rtl` | `shaper.rs` | ✅ ok |
| `p484_try_shape_rtl_sem_fonte_nao_panic` | `shaper.rs` | ✅ ok |
| `p484_bidi_runs_misto_ingles_arabico` | `shaper.rs` | ✅ ok |
| `p484_bidi_runs_byte_start_correcto` | `shaper.rs` | ✅ ok |

**Total acumulado typst-infra:** 522 testes (6 novos de P484).

---

## 5. Arquivos alterados

### Specs L0 (3 actualizadas)

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/infra/shaper.md` | §P484: `BidiRun`, `bidi_runs`, `try_shape` com RTL; scope-out |
| `00_nucleo/prompts/entities/layout_types.md` | §P484: `FrameItem::Text` como tipo pré-shaping permanente; ADR-0029 justificação |
| `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md` | §P484: unicode-bidi, bidi_runs, decisão de preservação, Trilha 5 COMPLETA |

### Código L3 (3 ficheiros)

| Ficheiro | `@prompt-hash` pós-P484 | Alteração |
|----------|------------------------|-----------|
| `03_infra/src/shaper.rs` | `6fbff5fe` | `BidiRun`, `bidi_runs`, `try_shape` com bidi + 6 testes P484 |
| `03_infra/Cargo.toml` | — | `unicode-bidi = { workspace = true }` |
| `Cargo.toml` (workspace) | — | `unicode-bidi = "0.3"` em `[workspace.dependencies]` |

### Código L1 (1 ficheiro — hash actualizado por fix-hashes)

| Ficheiro | `@prompt-hash` pós-P484 | Alteração |
|----------|------------------------|-----------|
| `01_core/src/entities/layout_types.rs` | `d33f7884` | Hash regenerado (prompts/entities/layout_types.md actualizado) |

### Documentação

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/materialization/typst-passo-484-relatorio.md` | CRIADO (este ficheiro) |

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 2 files:
    ./01_core/src/entities/layout_types.rs  → d33f7884
    ./03_infra/src/shaper.rs               → 6fbff5fe
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ 0 erros V1–V14.
  Warnings V7 pré-existentes (prompts órfãos não relacionados com P484).
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Múltiplos parágrafos** | `bidi_runs` usa `paragraphs[0]` — texto com `\n` tratado como 1 parágrafo. Shaper actua palavra a palavra (FrameItem::Text por palavra), por isso o impacto é nulo. |
| **Texto vertical (CJK rotated)** | `Direction::TopToBottom`/`BottomToTop` não implementados. |
| **Corpus RTL em lab/parity** | Sem ficheiros árabe/hebraico no corpus. Testes unitários L3 cobrem o comportamento. |
| **Remoção de `FrameItem::Text`** | Adiada — requer ADR nova (colisão ADR-0029). Documentada como permanente. |
| **Font fallback multi-família para RTL** | shaper usa apenas fonte primária da FontList. |
| **OpenType features explícitas** | `features = &[]` — GSUB/GPOS por defeito. |

---

## 8. Critério de fecho

- [x] Sondas: versão `unicode-bidi` = 0.3.18; `visual_runs` API confirmada via `line.rs:273`; `set_direction` via `shape.rs:20-23`.
- [x] `unicode-bidi = "0.3"` adicionado ao workspace e a `03_infra/Cargo.toml`.
- [x] `bidi_runs(text) -> Vec<BidiRun>` implementado em `shaper.rs`.
- [x] `try_shape` usa `bidi_runs` em vez de `guess_segment_properties()`.
- [x] `set_direction(Direction::RightToLeft)` usado para runs RTL.
- [x] 6 testes RTL verdes.
- [x] Decisão sub-item B documentada: `FrameItem::Text` preservado como tipo pré-shaping.
- [x] ADR-0120 anotada com §P484 (Fase 3 executada; Trilha 5 COMPLETA).
- [x] `cargo build --workspace` verde (0 errors).
- [x] `cargo test -p typst-infra` verde (522 passed, 0 failed).
- [x] `crystalline-lint .` zero erros V1–V14.
- [x] **Trilha 5: Fase 3 FECHADA.** RTL básico implementado.

**Nota:** `recursao_infinita_retorna_err_sem_crash` (typst-core) causa SIGABRT
por stack overflow — pré-existente antes de P484 (verificado via `git stash`).
Não relacionado com P484.

---

## 9. Estado pós-P484

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, **5**, 7, 8 |
| Trilhas pendentes | 6 (4/5) |
| Paridade | 73/73 matches |
| ADR-0120 | **ACEITE** — Fases 1 + 2 + 3 executadas |
| Trilha 5 | **COMPLETA** (P482 + P483 + P484) |
| `FrameItem::Text` | `#[deprecated]` — tipo pré-shaping permanente |
| `unicode-bidi` | `"0.3"` no workspace |
| **P484** | **FECHADO** |

---

## 10. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P485-A** | Fechar Trilha 6 (5ª funcionalidade pendente) | M |
| **P485-B** | `emit_shaped_pdf` com `x_advance` em pt — posicionamento preciso por glifo | M |
| **P485-C** | OpenType features explícitas (`liga`, `kern`) via `rustybuzz::Feature` | M |
