# ⚖️ ADR-0022: `FontBook` real em L3

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-27

---

## Contexto

`FontBook` em L1 é actualmente `FontBook(())` — stub opaco.
`world.book()` retorna este stub, o que significa que o compilador
não tem metadados de fontes para selecção tipográfica.

`FontBook` no original (`typst-library/src/text/font/book.rs`)
contém `Vec<FontInfo>` com metadados de cada face de fonte:
família, estilo, peso, largura, flags. É usado pelo engine de
layout para seleccionar a fonte correcta para cada texto.

A questão é onde `FontBook` e `FontInfo` pertencem: se os campos
de `FontInfo` são primitivos, é domínio puro e vai para L1.
Se usam tipos de `ttf_parser` directamente, fica em L3.

---

## Decisão (após diagnóstico)

### Opção A — FontBook e FontInfo como tipos de domínio em L1

Se `FontInfo` tem apenas campos primitivos (String, u16, u32, bool):

```rust
// 01_core/src/entities/font_book.rs
pub struct FontBook { infos: Vec<FontInfo> }
pub struct FontInfo {
    pub family:  String,
    pub variant: FontVariant,
    pub flags:   FontFlags,
    // coverage: confirmar se existe e de que tipo
}
```

A *extracção* de `FontInfo` a partir de bytes de fonte (via `ttf_parser`)
fica em L3 — `fonts.rs` já existente. `FontBook` em L1 é apenas a
colecção com métodos de pesquisa; não sabe de bytes nem de `ttf_parser`.

Remover stub `FontBook(())` de `world_types.rs` após criar o tipo real.

### Opção B — FontBook permanece em L3

Se `FontInfo` usa tipos de `ttf_parser` nos campos, ou se a pesquisa
de fontes (ex: `select_font` com lógica de scoring complexa) tem
dependências externas — `FontBook` fica em L3 como tipo interno.

O stub em L1 mantém-se. Nesse caso, `world.book()` retorna uma
referência a um wrapper mínimo que L1 pode usar sem ver os tipos de L3.
Esta opção é mais trabalhosa — preferir Opção A se os campos permitirem.

**A decisão entre A e B depende exclusivamente do diagnóstico.**

---

## Impacto no pipeline

Com `FontBook` real (qualquer opção):
- `world.book()` retorna metadados reais de fontes
- O engine de layout pode seleccionar fontes correctamente
- `#set text(font: "...")` deixa de silenciosamente usar fonte de fallback

---

## Relação com Font real

`FontBook` e `Font` são independentes e podem migrar em ordem diferente:
- `FontBook` tem *metadados* (para selecção)
- `Font` tem *dados* (para shaping e rendering via `rustybuzz`)

Este ADR cobre apenas `FontBook`. `Font` real é trabalho posterior.

---

## Consequências

**Positivas**: `world.book()` passa a retornar informação real;
selecção de fontes funciona no pipeline.

**Negativas**: Se Opção A, adiciona novos tipos a L1 com prompts e
testes; a extracção de `FontInfo` via `ttf_parser` em L3 adiciona
lógica a `fonts.rs`. Se Opção B, stub mantido com impacto no layout.

**Neutras**: `FontSlot::get()` em L3 já carrega bytes de fonte —
a extracção de `FontInfo` é um passo adicional sobre dados disponíveis.

---

## Referências

**Diagnóstico prévio**: ver
`00_nucleo/diagnosticos/diagnostico-adr-0022-fontbook.md` —
verificações executadas antes desta decisão.

- ADR-0019 — `ttf-parser` e `rustybuzz` em L3
- Passo 8 — `FontSlot` e `discover_fonts` em `03_infra/src/fonts.rs`
- `lab/typst-original/crates/typst-library/src/text/font/book.rs`
