---
# P785a — Syntax highlighting em blocos `raw`

> **Passo:** 785a
> **Data:** 2026-07-20
> **Foco:** P785 confirmou que blocos ` ```rust `/` ```typ ` renderizam monocromáticos (`DeviceGray 0`) no cristalino, enquanto o vanilla aplica cores por token (`DeviceRGB`, palavras-chave/identificadores/strings diferenciados) via `typst_syntax::highlight`. Este passo implementa o mecanismo de highlighting.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** L — nova capacidade de tokenização + mapeamento de cores, toca no layout de `raw`.
> **ADR-0108 EM VIGOR** — confirmar a paleta de cores exata e a linguagem/linguagens suportadas antes de implementar.
> **Dependências:** P785 (achado, evidência de cores RGB específicas).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn highlight\b\|Tag::" lab/typst-original/crates/typst-syntax/src/highlight.rs 2>/dev/null | head -40
```

Confirmar:
1. Quais categorias de token existem (`Tag` enum — keyword, identifier, string, comment, etc.) e a paleta de cores associada a cada uma no exportador (`typst-library`/tema padrão).
2. Que linguagens são suportadas nativamente — o vanilla usa tokenização própria só para Typst (` ```typ `), ou também para outras linguagens via alguma dependência (ex: `syntect`)? Confirmar antes de assumir escopo amplo.
3. Como o highlighting é acionado — automático para todo bloco `raw` com `lang:` reconhecido, ou requer configuração explícita.

```bash
cat > /tmp/p785a-test.typ <<'EOF'
```rust
fn main() {
    let x = 42;
}
```
EOF
lab/typst-original/target/release/typst compile /tmp/p785a-test.typ 2>&1
```

Confirmar quais linguagens o vanilla realmente coloriza (pode ser só um subconjunto, não qualquer `lang:` arbitrário).

---

## Decisão de âmbito

| Cenário | Decisão |
|---|---|
| Vanilla só coloriza Typst nativo (` ```typ `), usando o próprio lexer/parser já existente no compilador | Implementável diretamente, reaproveitando o lexer já existente |
| Vanilla coloriza múltiplas linguagens via dependência externa de tokenização | Avaliar peso da dependência (mesmo padrão de decisão usado em P772k/P781 para SVG/PDF) antes de comprometer |

Registar com base no que a sonda encontrar.

---

## Implementação (conforme decisão)

1. Se for só Typst nativo: reutilizar o lexer/parser já existente (`01_core/src/engine/lexer`, `parse`) para tokenizar o conteúdo do bloco `raw`, mapear tokens para categorias, aplicar cores no layout de texto do bloco.
2. Se envolver outras linguagens: confirmar a dependência necessária, medir peso (mesmo processo de P781), decidir se cabe.
3. L0 antes do código, cobrindo a paleta de cores e o mecanismo de tokenização.

---

## Validação

```bash
./target/release/typst compile /tmp/p785a-test.typ /tmp/p785a-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p785a-test.typ /tmp/p785a-vanilla.pdf
mutool trace /tmp/p785a-vanilla.pdf | grep -o 'color="[^"]*"' | sort -u
mutool trace /tmp/p785a-cristalino.pdf | grep -o 'color="[^"]*"' | sort -u
```

Confirmar que as mesmas cores (ou próximas) aparecem para as mesmas categorias de token.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Escopo de linguagens suportadas pelo vanilla confirmado.
- [x] Paleta de cores por categoria de token confirmada.
- [x] Decisão de âmbito registrada.
- [x] Highlighting implementado e validado por cores no PDF exportado.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p785a.md`.

---

## Próximo passo

P785b (campos nativos) ou P785c (offset UTF-16).
