# Prompt L0 — `compiler/stdlib/structural/footnote` — `footnote` (construção)
Hash do Código: 5a4c737b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/footnote.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/footnote.rs` — ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. `native_footnote` tem **3** commits de corpo
reais (`c823fca5b`, `dfef736af`, `fac57c4d1`) e nenhum toca `par`; com `quote` partilha só
`c4978547e` e `e1f09cc24` — os dois lotes transversais. Critério 3 mudo, critério 4 separa.

**Fronteira de fase — verificada, não presumida**: existe um `compiler/layout/footnote.rs`
(L0 próprio: `compiler/layout/footnote.md`). **Não há duplicação de responsabilidade**:

| Nível | Ficheiro | O que faz |
|---|---|---|
| construção (este nó) | `stdlib/structural/footnote.rs` | valida argumentos e emite `Content::footnote_with_numbering(body, numbering)` |
| render | `compiler/layout/footnote.rs` | `layout(...)`, estado `pending_footnote_bodies`, marcador inline via `format_pattern` + `Content::superscript` |

A nativa não sabe numerar nem posicionar; o layout não valida argumentos. A fronteira é a
fase do pipeline, e mexer nela é gate ADR-0127.

---

## Contexto

`footnote(...)` marca o ponto de chamada e transporta o corpo da nota. Quem decide o
número, o marcador e a posição na página é o layout — este nó só constrói o elemento.

## Instrução

`footnote(body, numbering: ?)` — 1 posicional `Content | Str` obrigatório.

- `Content` → usado directamente; `Str` → `Content::text(s)`.
- `numbering: Str | none` → pattern de numeração; `none` remove-o. Tipo diferente de
  string → `footnote(numbering:): espera string, recebeu {tipo}`.
- Emite `Content::footnote_with_numbering(body, numbering)`.
- Sem body → `footnote() exige body como argumento posicional`; tipo errado →
  `footnote() espera content ou string, recebeu {tipo}`.
- **Qualquer outro nomeado** → erro que cita o scope-out graded (ADR-0054), não
  "argumento inesperado" genérico: os cosméticos do vanilla estão deliberadamente fora.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados. Em especial: o
  contador de notas **não** vive aqui (é estado do layout), e introduzi-lo neste nó seria
  mudança de fase do pipeline — paragem obrigatória (ADR-0127).
- Nenhum helper privado.

## Critérios de Verificação

```
#footnote[x]                   → Content::Footnote com body, numbering None
#footnote("x")                 → body coagido de Str
#footnote(numbering: "a")[x]   → Footnote com pattern "a"
#footnote(numbering: none)[x]  → Footnote sem pattern
#footnote(numbering: 1)[x]     → Err "footnote(numbering:): espera string, recebeu integer"
#footnote()                    → Err "footnote() exige body como argumento posicional"
#footnote(1)                   → Err "footnote() espera content ou string, recebeu integer"
#footnote(zz: 1)[x]            → Err citando o scope-out graded (ADR-0054)
```

## P1473 — `footnote.entry` (ESCOPO REDUZIDO — construção sem realização) (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

Sonda bilateral (2026-09-21, vanilla ratificado `586e1bd43fae`):
`type(footnote.entry)` → `function` no vanilla, `function footnote does not
contain entry` no cristalino (`footnote` já é `Value::Func` bare).

Fonte pinada
(`lab/typst-original/crates/typst-library/src/model/footnote.rs:223-296`):

```rust
pub struct FootnoteEntry {
    #[required] pub note: Packed<FootnoteElem>,
    #[default(LineElem::new()....pack())] pub separator: Content,
    #[default(Em::new(1.0).into())] pub clearance: Length,
    #[default(Em::new(0.5).into())] pub gap: Length,
    #[default(Em::new(1.0).into())] pub indent: Length,
}
```

`note` exige um `FootnoteElem` com `Location` real (`self.note.location()`,
usado por `.realize()` para consultar `Counter::of(FootnoteElem::ELEM)` e
construir o link/superscript de volta à chamada) — **inerentemente ligado
ao pipeline de posicionamento de página**, não construível com sentido
fora dele. `.realize()`/`ShowSet` (leading 0.5em, tamanho 0.85em) vivem
inteiramente na fase de layout (`compiler/layout/footnote.rs`, owner
próprio) — **não podem mover-se para cá** (regra já vigente acima nesta
secção, "A nativa não sabe numerar nem posicionar").

### Decisão — escopo reduzido

Registrar somente a **construção**: `footnote.entry(note, separator:,
clearance:, gap:, indent:)`. `note` é `Content` posicional obrigatório
(mesma coerção `Str → Content::text` de `footnote`); os quatro nomeados
são opcionais, tipados `Content`(`separator`)/`Length`(demais), aceites e
**preservados sem efeito de layout** — mesmo padrão já vigente em
`entities/elements/list_item.md` para `marker_align`/`indent`/`tight`
("campo aceite e preservado sem efeito visual", P504/P505). Constrói
`Content::FootnoteEntry(FootnoteEntryElem { note, separator, clearance,
gap, indent })`, nova entidade em
`entities/elements/footnote_entry.rs` (owner 1:1 próprio,
`entities/elements/footnote_entry.md`, a criar).

**Não implementa nesta fatia:** `.realize()` (link/superscript/número via
`Counter`), `ShowSet` (leading/tamanho automáticos), qualquer efeito
visual dos quatro nomeados, nem a substituição do renderer atual de
`compiler/layout/footnote.rs` por este constructor — o layout continua a
gerar a lista de notas de rodapé exatamente como hoje, sem consultar
`footnote.entry`. `#show footnote.entry: ...` pode não interceptar nada
até essa integração existir; isso é medido e registado no RED, não
resolvido aqui. Reabertura futura exige medir a integração `note.location()`
↔ pipeline de posicionamento antes de qualquer layout real.

### Gate ADR-0127

Nova rota qualificada + nova entidade pública. Requer L0 causal (este e o
novo `entities/elements/footnote_entry.md`), hashes ressealados, V5/V15/
V26, commit documental e **confirmação humana explícita posterior**.
