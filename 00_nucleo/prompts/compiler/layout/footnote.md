# Prompt L0 — `compiler/layout/footnote` — layout do marcador de nota de rodapé
Hash do Código: 6f0fb64a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/layout/footnote.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/layout.md`
**ADRs**: ADR-0107 (paridade linguagem), ADR-0109 (atomização, forma B), ADR-0127 (gate de mudança de output visual)

---

## Contexto

Layout do elemento `footnote`: emite o marcador inline e difere o corpo para
o rodapé (`pending_footnote_bodies`, flush em `new_page`/`finish`). O arquivo
foi extraído do monólito `layout/mod.rs` no âmbito da atomização (P381,
ADR-0109).

## Instrução

### `layout(layouter, e)`

1. **Obter o número**: via `layouter.introspector.flat_counter_at("footnote", loc)`
   (P1016). `loc` é `layouter.current_location`, actualizado no topo de
   `layout_content` porque Footnote é locatable.

2. **Aplicar `numbering` do `FootnoteElem`**:
   - `FootnoteElem::numbering` é `Option<EcoString>`.
   - Se `Some(pattern)`, formatar o número com `format_pattern` (reusar
     `compiler/stdlib/numbering.rs::format_pattern`, P793/P844).
   - Se `None`, usar o default vanilla `"1"` (comportamento actual de
     `#footnote[...]` sem `#set footnote(numbering:)`).

3. **Renderizar em superscript**:
   - Envolver o texto formatado em `Content::superscript(...)` (P448), de
     forma a elevar a baseline e reduzir o corpo — paridade visual com o
     vanilla.
   - **Não** envolver em colchetes literais (`[N]`); o vanilla usa apenas o
     número formatado (ex.: `¹`, `*`).

4. **Diferir o body**: empurrar `(n, Box::new(e.body.clone()))` para
   `layouter.pending_footnote_bodies`, como hoje.

### Exemplos

```typst
A#footnote[nota] B#footnote[outra]
```

- Sem `#set footnote(numbering:)`: marcadores `¹` e `²` em superscript.
- Com `#set footnote(numbering: "*")`: marcadores `*` e `**` em superscript.

## Restrições estruturais

- Manter acesso ao estado privado do `Layouter` por descendência de módulo
  (forma B da ADR-0109).
- Não introduzir import reverso `entities → layout`.

## Gatilhos de reabertura

- Mudança na forma do marcador no vanilla.
- Suporte a `footnote.entry` ou links do marcador para a nota.

## Critérios de verificação

```
A#footnote[uma] B#footnote[duas]
  marcadores no corpo → "A¹ B²" (superscript, sem colchetes)
  notas no rodapé     → "1 uma / 2 duas"  // inalterado

#set footnote(numbering: "*")
A#footnote[a] B#footnote[b] C#footnote[c]
  marcadores → "A* B** C***"
```

Não-regressão: `p552_footnote_counter_avanca_em_set_page_columns` e todos os
testes de posicionamento/numeração de nota do Passo 1016 — a numeração em si
não pode mudar, só a forma de desenho.

Aplicação final: `cargo build && crystalline-lint .` — zero violations.
