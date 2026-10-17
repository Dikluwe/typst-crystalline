# Prompt L0 — `entities/elements/footnote_entry` — `FootnoteEntryElem`
Hash do Código: 941e45c4

**Estado**: MATERIALIZADO (confirmação humana 2026-09-21, posterior ao commit
causal somente-L0 `3ac60a578`; execução segregada A/B/C — ver
`typst-passo-1473.md` §8).

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/footnote_entry.rs`
**Origem**: **P1473** (`typst-passo-1473.md`), família das oito rotas públicas não
matemáticas restantes. Owner do consumer: `compiler/stdlib/structural/footnote.md`
(construção); layout/realização permanecem fora desta fatia — ver lá.
**Vanilla ratificado**: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`typst-library/src/model/footnote.rs:223-296`.

---

## Escopo reduzido — construção sem realização

Ver `compiler/stdlib/structural/footnote.md` §P1473 para a medição completa e a
justificação do corte de fase. Esta entidade só existe para dar forma pública a
`footnote.entry(...)`; não participa do layout de notas de rodapé nesta fatia.

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteEntryElem {
    pub note:      Content,          // FootnoteElem real (não validado além de Content)
    /// Vanilla default: uma linha (LineElem) de 30% de largura, traço 0.05em.
    /// Layout real é scope-out; o campo é aceite e preservado sem efeito visual.
    pub separator: Option<Content>,  // None = default do vanilla (não replicado aqui)
    /// Vanilla default 1em. Layout real é scope-out.
    pub clearance: Option<Length>,
    /// Vanilla default 0.5em. Layout real é scope-out.
    pub gap:       Option<Length>,
    /// Vanilla default 1em. Layout real é scope-out.
    pub indent:    Option<Length>,
}
```

`Content::FootnoteEntry(Arc<FootnoteEntryElem>)`.

Construtor: `Content::footnote_entry(note: Content, separator: Option<Content>,
clearance: Option<Length>, gap: Option<Length>, indent: Option<Length>) ->
FootnoteEntryElem`. `None` nos quatro últimos = "usar o default do vanilla
quando/se a realização real for implementada"; não resolve nem materializa
o default aqui (não há layout consumidor nesta fatia).

## `Element` — mínimo viável

`plain_text()` delega a `note.plain_text()`. `is_empty()` = `note.is_empty()`.
`map_content`/`map_text` recursam só em `note` (os outros campos ou são `None`
ou `Content` opaco sem filhos relevantes ao mapeamento nesta fatia — se
`separator` vier `Some`, recursar nele também, por simetria com os demais
elementos de conteúdo do crate).

## `eq`/`Hash`

`PartialEq` deriva direto. **`Hash` não deriva** — `Length` carrega `f64`,
que não implementa `Hash` (mesma causa já registada em
`entities/elements/list_item.md` para `indent`/`body_indent`/`tight`).
`Hash` manual via `format!("{self:?}")` (`Hash`-via-`Debug`), mesmo padrão
de `ListItemElem`/`EnumItemElem` (`entities/elements/list_item.rs`).

## Não-objetivos

Não implementa `.realize()` (link/superscript via `Counter::of(FootnoteElem)`,
`note.location()`), não implementa `ShowSet` (leading/tamanho automáticos), não
altera `compiler/layout/footnote.rs`. Reabertura de qualquer um destes exige
medição própria e gate ADR-0127 (mudança de fase do pipeline).

## Materialização

Agente A (`01a82593d`) criou este ficheiro e a entidade
`FootnoteEntryElem` (struct, `Hash` manual via `Debug` — `Length` carrega
`f64`, mesma causa de `ListItemElem` —, `Element`, `Content::
FootnoteEntry`/constructor) sem ver os testes. O ficheiro não tinha nenhum
teste próprio até a integração (papel C, 2026-09-21) — `crystalline-lint`
V2 exige verificação simultânea por módulo do núcleo; acrescentados 5
testes locais (mesma cobertura de `figure_caption.rs`, adaptada aos cinco
campos). Suíte: GREEN.
