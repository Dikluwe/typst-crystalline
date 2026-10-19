# typst-passo-538b.md

## Objetivo

Fechar os itens de alto impacto que **não** foram confirmados em P538:

- `00_nucleo/diagnosticos/confirmacao-p538.md`

## Itens a corrigir

### 1. P532 — `style.font` do texto de numeração de página

**Medição (P538):** `01_core/src/engine/layout/mod.rs:1154-1158` insere `FrameItem::Text` com `style: TextStyle::regular(self.font_size_pt)`. `TextStyle::regular` usa `..Self::default()`, logo `style.font` é `None`.

**Comportamento esperado:** `style.font` deve conter a fonte activa da página, como qualquer outro texto renderizado.

**Teste de aceitação:**

```bash
cat > /tmp/num-test.typ <<'EOF'
#set page(numbering: "1")
Página.
EOF
./target/release/typst /tmp/num-test.typ /tmp/num-test.pdf && pdftotext /tmp/num-test.pdf -
```

A confirmação de `style.font` preenchido é feita por inspecção directa do código (`style.font.is_some()` no ponto onde o `FrameItem::Text` da numeração é criado).

### 2. P534 — Fallback de fonte por carácter

**Medição (P538):**

```bash
cat > /tmp/fb-test.typ <<'EOF'
Hello 你好 مرحبا
EOF
./target/release/typst /tmp/fb-test.typ /tmp/fb-crystalline.pdf && pdftotext /tmp/fb-crystalline.pdf -
/usr/local/bin/typst compile /tmp/fb-test.typ /tmp/fb-vanilla.pdf && pdftotext /tmp/fb-vanilla.pdf -
```

**Comportamento esperado:** extração do PDF cristalino deve conter `Hello 你好 مرحبا`, como o vanilla.

**Teste de aceitação:** `pdftotext` do PDF cristalino exibe os três scripts e o diff contra o vanilla é aceitável.

### 3. P536 — Metadados não-ASCII

**Medição (P538):**

```bash
cat > /tmp/meta-test.typ <<'EOF'
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
EOF
./target/release/typst /tmp/meta-test.typ /tmp/meta-crystalline.pdf && pdfinfo /tmp/meta-crystalline.pdf
```

Resultado: `RelatÃ…Â³rio de JosÃ…Â©` / `JoÃ…Â£o ConceiÃ…Â§Ã…Â£o`.

**Comportamento esperado:** `pdfinfo` mostra `Relatório de José` e `João Conceição`.

**Teste de aceitação:** `pdfinfo` exibe os caracteres acentuados correctamente (sem diferença visível contra `Relatório de José` / `João Conceição`).

### 4. P537/P537b — Documento longo em colunas

**Medição (P538):**

```bash
cat > /tmp/cols-test.typ <<'EOF'
#set page(columns: 2)
#lorem(1200)
EOF
./target/release/typst /tmp/cols-test.typ /tmp/cols-crystalline.pdf && pdfinfo /tmp/cols-crystalline.pdf | grep Pages
/usr/local/bin/typst compile /tmp/cols-test.typ /tmp/cols-vanilla.pdf && pdfinfo /tmp/cols-vanilla.pdf | grep Pages
```

Resultado: cristalino `Pages: 9`; vanilla `Pages: 2`.

**Comportamento esperado:** número de páginas próximo do vanilla (idealmente `2`, ou justificado por diferença de layout se houver); `pdfinfo` não deve emitir `Syntax Error: Suspects object is wrong type (boolean)`.

**Teste de aceitação:**

- `pdfinfo` não reporta erro de sintaxe.
- `Pages` difere do vanilla no máximo 1 página, ou a diferença é explicitamente documentada e aceite.

## Ordem sugerida

1. **P536** — metadados não-ASCII (provavelmente fix simples de codificação UTF-16/UTF-8 no writer de PDF metadata).
2. **P532** — preencher `style.font` na numeração de página.
3. **P534** — fallback de fonte por carácter.
4. **P537/P537b** — paginação de colunas em documentos longos.

## Restrições

- Não iniciar P539 (reorganização) até que todos os itens acima estejam fechados.
- Cada correção segue o Protocolo de Nucleação: existe L0? → testes primeiro → implementação → `cargo build && crystalline-lint .`.
- O scope-out de overflow de nota grande em colunas não faz parte deste passo; permanece registado em `00_nucleo/diagnosticos/paridade-producao-p537.md`.
