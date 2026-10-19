# Relatório de produção — P538b — Metadados `/Info` em UTF-16BE

**Passo:** 538b  
**Data:** 2026-07-03  
**Foco:** Corrigir a codificação do dicionário `/Info` do PDF para caracteres não-ASCII.  
**Binário cristalino:** `./target/release/typst` (reconstruído em release após a correção).  
**Prompt L0:** `00_nucleo/prompts/infra/export/builder.md` (hash `bf679d81`).

---

## Problema

`#set document(title: "Relatório de José", author: "João Conceição")` produzia:

```text
Title:           RelatÃ…Â³rio de JosÃ…Â©
Author:          JoÃ…Â£o ConceiÃ…Â§Ã…Â£o
```

O padrão de corrupção indicava dupla codificação: texto UTF-8 era escrito como string literal PDF (interpretada como Latin-1/PDFDocEncoding) e depois relido como UTF-8.

---

## Sonda

**Ficheiro:** `03_infra/src/export/builder.rs`

| Linha | Conteúdo |
|---|---|
| 1148-1156 | `emit_info` escreve `/Title`, `/Author`, `/Keywords` via `escape_pdf_literal`. |
| 1225-1239 | `escape_pdf_literal` itera sobre `s.bytes()` e assume string de um byte. |
| 1261-1269 | Já existia `utf16be_hex_string` (usada para `/Title` em bookmarks). |

**Conclusão da sonda:** não havia dupla chamada, mas a função de escape assumia codificação de um byte sobre texto UTF-8. A solução é reutilizar `utf16be_hex_string` para os campos `/Info`.

---

## Implementação

Em `03_infra/src/export/builder.rs:1148-1156`, `escape_pdf_literal` foi substituído por `utf16be_hex_string` para `/Title`, `/Author` e `/Keywords`.

```rust
if let Some(title) = &doc.document_info.title {
    parts.push(format!("/Title {}", utf16be_hex_string(title.as_str())));
}
if let Some(author) = &doc.document_info.author {
    parts.push(format!("/Author {}", utf16be_hex_string(author.as_str())));
}
if let Some(keywords) = &doc.document_info.keywords {
    parts.push(format!("/Keywords {}", utf16be_hex_string(keywords.as_str())));
}
```

O `/Info` agora contém strings hex UTF-16BE com BOM (`<FEFF...>`), que cobrem qualquer carácter Unicode.

### Prompt L0

A secção §P536 de `00_nucleo/prompts/infra/export/builder.md` foi atualizada para especificar UTF-16BE com BOM. Hash do código resultante: `bf679d81`.

### Testes

Adicionados em `03_infra/src/export/tests.rs`:

- `pdf_info_utf16be_com_acentos` — verifica `"Relatório de José"` e `"João Conceição"`.
- `pdf_info_utf16be_caracter_nao_latino` — verifica `"日本語"`, `"用户"` e `"你好 مرحبا"`.

Ambos decodificam o valor hex do PDF e comparam com a string original.

---

## Validação

### Testes automáticos

```bash
cargo test --workspace
crystalline-lint .
```

Resultado:

- `cargo test --workspace` — passou.
- `crystalline-lint .` — zero violations.

### Teste manual — acentos latinos

```bash
cat > /tmp/meta-test.typ <<'EOF'
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
EOF
./target/release/typst /tmp/meta-test.typ /tmp/meta-crystalline.pdf
pdfinfo /tmp/meta-crystalline.pdf | grep -E "Title|Author"
```

Resultado:

```text
Title:           Relatório de José
Author:          João Conceição
```

### Teste manual — caracteres não-latinos

```bash
cat > /tmp/meta-test-cjk.typ <<'EOF'
#set document(title: "日本語", author: "用户", keywords: "你好 مرحبا")
Texto.
EOF
./target/release/typst /tmp/meta-test-cjk.typ /tmp/meta-cjk.pdf
pdfinfo /tmp/meta-cjk.pdf | grep -E "Title|Author|Keywords"
```

Resultado:

```text
Title:           日本語
Keywords:        你好 مرحبا
Author:          用户
```

---

## Critérios de fecho

- [x] Sonda completa antes de código.
- [x] `pdfinfo` mostra os acentos correctos.
- [x] Testado com carácter fora do latim.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p538b.md`.

---

## Estado final

**P538b fechado.** O item P536 da tabela de P538 está resolvido. Os restantes itens (P532, P534, P537/P537b) permanecem em aberto para passos subsequentes.
