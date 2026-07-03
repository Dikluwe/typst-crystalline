# Relatório de Paridade de Produção — P538d

**Data:** 2026-07-03  
**Passo:** 538d  
**Prompt L0:** `00_nucleo/prompts/rules/layout.md` (hash `12536b5c`)  
**Dependências:** P483 (correcção equivalente para texto normal), P532 (introdução da numeração automática de página)

## Objectivo

Corrigir o `style.font` ausente no texto de numeração automática de página (`#set page(numbering: ...)`), que era construído com `TextStyle::regular(self.font_size_pt)` e deixava o campo `font` a `None`.

## Causa

Em `01_core/src/rules/layout/mod.rs:1173`, o texto da numeração de página usava:

```rust
style: TextStyle::regular(self.font_size_pt),
```

`TextStyle::regular` deriva de `Self::default()`, onde `font` é `None`. Isto coloca o texto de numeração na mesma classe de problema que P483 corrigiu para o texto normal do documento: sem `font` definida, o shaper não aplica a lógica de fonte correcta.

## Implementação

Substituída a construção por:

```rust
let mut style = TextStyle::from(&self.chain);
style.size = self.font_size_pt;
items.push(FrameItem::Text {
    pos: Point { x: Pt(x), y: Pt(y) },
    text: text.into(),
    style,
});
```

A fonte e os restantes atributos de texto agora vêm da `StyleChain` activa (ponto único de resolução ADR-0039); o tamanho é forçado para `self.font_size_pt` para preservar o comportamento anterior de P532.

## Ficheiros alterados

- `01_core/src/rules/layout/mod.rs` — numeração automática de página usa `TextStyle::from(&self.chain)`.
- `01_core/src/rules/layout/tests.rs` — teste `p538d_page_numbering_text_tem_font_definida`.

## Testes automáticos

Comando executado:

```bash
cargo test -p typst-core p538d_page_numbering_text_tem_font_definida -- --nocapture
```

Resultado: `test rules::layout::tests::tests_set_rule_integration::p538d_page_numbering_text_tem_font_definida ... ok`

O teste verifica directamente que existe um `FrameItem::Text` com texto `"1"` e `style.font.is_some()`.

## Teste manual

Script:

```typst
#set page(numbering: "1")
Página.
```

Comando:

```bash
./target/release/typst /tmp/num-test.typ /tmp/num.pdf
pdftotext /tmp/num.pdf -
```

Resultado:

```text
Página.

1
```

A numeração continua a aparecer correctamente no PDF.

## Validação arquitetural

```bash
crystalline-lint .
```

Resultado: zero violations.

Também executado:

```bash
cargo test --workspace
```

Resultado: todos os testes passam.

## Conclusão

P538d está concluído. O texto de numeração de página agora tem `style.font` preenchido, alinhado com a correcção de P483 para texto normal. Não houve regressões.
