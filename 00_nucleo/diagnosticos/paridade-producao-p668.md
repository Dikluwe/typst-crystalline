# Relatório de Paridade de Produção — Passo 668

| Campo | Valor |
|-------|-------|
| Passo | P668 |
| Foco | Caminho single-font nunca instancia fontes variáveis |
| Data | 2026-07-10 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Corrigido |
| Commit de medição inicial | `79f050ada4c6e5ffa2afe6689633562768523e8a` |

---

## Sonda

### Objetivo

P666 verificou que o caminho multi-font instancia correctamente fontes variáveis. P667 adicionou um erro claro quando Python/fontTools falta. P668 investiga o caminho single-font (`build_cidfont`), usado quando o documento inteiro resolve numa única fonte — o caso mais comum em documentos simples.

### Documento de teste

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
cat > /tmp/p668-single-weight.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700, size: 40pt)
Só este peso, nada mais no documento.
EOF
```

### Resultado antes da correcção

```bash
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668.pdf
pdffonts /tmp/p668.pdf
```

- Uma só fonte CID TrueType embutida (`AAAAAA+CrystallineFont`).
- Visualmente o texto aparece em Regular, apesar de `weight: 700`.
- Tamanho do PDF: ~278 KB (fonte VF completa, não instanciada).

### Condição de dispatch single vs multi-font

Em `03_infra/src/pipeline.rs:456`:

```rust
let (pdf, subset_ms) = match resolved.as_slice() {
    [] => (export_pdf_with_document_id(&doc, document_id), 0.0),
    [((_, _), bytes)] => {
        export_pdf_with_font_and_timings_and_document_id(&doc, bytes, document_id)
    }
    many => export_pdf_multifont_and_timings_and_document_id(&doc, many, document_id),
};
```

A condição é puramente numérica: um único font resolvido usa o caminho single-font (`export_pdf_with_font` → `PdfBuilder::build` → `build_cidfont`). Documentos simples com uma só família de fonte — o caso mais comum — caem aqui.

### Verificação P667 no caminho single-font

```bash
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-sem-python.pdf
echo "Exit code: $?"
```

Resultado **após P667 e antes de P668**:

```text
/tmp/p668-single-weight.typ:<detached>: error: fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível
Exit code: 1
```

A verificação de P667 (erro quando Python/fontTools indisponível) já se aplicava a este caminho porque vive em `pipeline.rs`, antes do dispatch. O problema era apenas quando a dependência **estava** disponível: o single-font não a usava.

---

## Implementação

### Decisão

Em vez de duplicar a lógica de instanciação no `build_cidfont`, alterou-se o dispatch em `03_infra/src/pipeline.rs`: quando há uma única fonte resolvida mas ela é variável e requer eixos não-default, o documento é encaminhado para o caminho multi-font, que já instancia correctamente (P530/P666).

### Alteração em `03_infra/src/pipeline.rs`

```rust
[single @ ((_, font_variant), bytes)] => {
    // P668 — se a única fonte resolvida for uma VF com eixos
    // não-default, usar o caminho multi-font, que já instancia
    // correctamente (P530/P666). O caminho single-font
    // (`build_cidfont`) não faz instanciação.
    if is_variable_font(bytes) && !axis_variations_for_font_variant(font_variant).is_empty() {
        export_pdf_multifont_and_timings_and_document_id(&doc, std::slice::from_ref(single), document_id)
    } else {
        export_pdf_with_font_and_timings_and_document_id(&doc, bytes, document_id)
    }
}
```

Fontes não-variáveis ou VF com instância default continuam a usar o caminho single-font original, preservando o comportamento e PDFs existentes.

---

## Validação

### Com Python/fontTools

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-depois.pdf
pdffonts /tmp/p668-depois.pdf
```

Resultado:

- Uma fonte CID TrueType embutida (`AAAAAA+CrystallineFont1`).
- Visualmente o texto aparece agora em Bold (700).
- Tamanho do PDF: ~7,6 KB (instância estática subsetada, sem tabelas de variação).

### Sem Python/fontTools

```bash
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-erro.pdf
```

Resultado:

```text
/tmp/p668-single-weight.typ:<detached>: error: fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível
Exit code: 1
```

Nenhum PDF gerado.

### Regressão no caminho multi-font

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p666-vf-pesos.typ /tmp/p668-multi-regression.pdf
pdffonts /tmp/p668-multi-regression.pdf
```

Resultado: 3 fontes CID TrueType, pesos visuais distintos. Sem regressão.

### Sanity checks

| Check | Comando | Resultado |
|-------|---------|-----------|
| Testes workspace | `cargo test --workspace` | 606 passed; 0 failed; 5 ignored |
| Linter | `crystalline-lint .` | ✓ No violations found |

---

## Tabela final de classificação

| Item | Resultado |
|------|-----------|
| Caminho single-font produz contornos errados | ✅ Confirmado antes da correcção |
| Este é o caso mais comum (documento com uma fonte) | ✅ Confirmado |
| Verificação P667 aplica-se ao single-font | ✅ Confirmado |
| Correcção implementada | ✅ Single VF com eixos não-default encaminhada para multi-font |
| Peso correcto no single-font | ✅ Confirmado visualmente |
| Erro sem Python/fontTools | ✅ Confirmado |
| Caminho multi-font sem regressão | ✅ Confirmado |
| Sem regressão nos testes | ✅ 606 passed |
| Linter limpo | ✅ |
| Relatório com hash do commit | ✅ |
| Pendência de P525 fechada nos dois caminhos | ✅ |

---

## Actualização do relatório de P525

A pendência 1 de `00_nucleo/diagnosticos/paridade-producao-p525.md` foi actualizada para reflectir que o fecho completo da regressão P525 envolveu P666 (caminho multi-font) **e** P668 (caminho single-font).

---

## Reprodução

```bash
# Documento com peso único numa fonte variável
cat > /tmp/p668-single-weight.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700, size: 40pt)
Só este peso, nada mais no documento.
EOF

# Com fontTools — deve produzir bold visual
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-ok.pdf

# Sem fontTools — deve falhar com erro claro
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-erro.pdf

# Sanity checks
cargo test --workspace
crystalline-lint .
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/pipeline.md`, `00_nucleo/prompts/infra/font_variant.md`
- Código: `03_infra/src/pipeline.rs`
- Relacionados: `00_nucleo/diagnosticos/paridade-producao-p525.md`, `00_nucleo/diagnosticos/paridade-producao-p666.md`, `00_nucleo/diagnosticos/paridade-producao-p667.md`
- ADR-0107: paridade é com a linguagem — `weight: 700` deve produzir contornos de negrito.
- ADR-0108: medir antes de decidir; o bug foi confirmado visualmente antes da correcção.
