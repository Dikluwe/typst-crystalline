# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/bidi.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `1092bf644a190c4d65021832a70ca472489b3134384f52577fc0d999b1976773`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 4524c646
prompt: infra/shaper/bidi
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper/bidi

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:1059-1122` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Produzir parágrafos em ordem lógica, cada um com runs em ordem visual, texto, direção e byte_start originais. Usar unicode-bidi sem alterar base direction. Preservar parágrafos vazios e remover separadores finais de classe B. Não selecionar fontes nem deslocar posições.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## P484 — Fase 3: RTL básico via unicode-bidi

**Data:** 2026-06-28

`unicode-bidi = "0.3"` adicionado ao workspace. `try_shape` agora divide o texto
em runs bidirectionais antes de shape via `bidi_runs(text) -> Vec<BidiRun>`.

### `BidiRun`

```rust
struct BidiRun {
    text:       String,  // substring do texto original para este run
    rtl:        bool,    // true se run RTL (árabe, hebraico, etc.)
    byte_start: usize,   // offset byte no string original (para cluster abs)
}
```

### `bidi_runs`

```rust
fn bidi_runs(text: &str) -> Vec<BidiRun>
```

Usa `unicode_bidi::BidiInfo::new(text, None)` + `visual_runs(para, line)`
para obter runs na ordem visual correcta. Para texto LTR puro: 1 run.
Para texto misto/RTL: múltiplos runs na ordem correcta de renderização.

### `try_shape` pós-P484

Substitui `buffer.guess_segment_properties()` por:

```rust
for run in bidi_runs(text) {
    buffer.set_direction(if run.rtl { Direction::RightToLeft }
                         else       { Direction::LeftToRight });
    // shape + ajustar abs_cluster = run.byte_start + info.cluster
}
```

### Scope-out P484

- ~~Múltiplos parágrafos (`paragraphs[0]` assume 1 parágrafo por `FrameItem::Text`).~~ **Revogado em P845** — ver secção P845 abaixo.
- Texto vertical (CJK rotated).
- Corpus RTL no lab/parity — testes unitários L3 cobrem.
- Remoção de `FrameItem::Text` — requer ADR nova (colisão ADR-0029).

### Testes adicionados P484

- `p484_bidi_runs_ltr_unico_run`: texto inglês → 1 run LTR.
- `p484_bidi_runs_vazio_zero_runs`: texto vazio → 0 runs.
- `p484_bidi_runs_arabico_rtl`: árabe → run RTL detectado.
- `p484_try_shape_rtl_sem_fonte_nao_panic`: árabe sem fonte → sem panic.
- `p484_bidi_runs_misto_ingles_arabico`: texto misto → ≥2 runs.
- `p484_bidi_runs_byte_start_correcto`: byte_start correcto.

---
