---
# P612 — `DocumentID`/`InstanceID` do XMP: fixos sempre, ou só em testes?

> **Passo:** 612
> **Data:** 2026-07-05
> **Foco:** P611 deixou `InstanceID` e `DocumentID` do XMP como "valores fixos", sem esclarecer se isto é uma medida de teste (como `CRYSTALLINE_PDF_FIXED_EPOCH` de P601) ou um valor permanente em qualquer documento gerado. Se for sempre fixo, todos os PDFs do cristalino têm o mesmo `DocumentID`, o que anula o propósito do campo — identificar um documento de forma única. Este passo confirma e corrige se necessário.
> **Tipo:** Verificação directa + correcção, se confirmado o problema.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P611 (onde os valores fixos foram introduzidos), P601 (precedente de mecanismo de teste isolado, `CRYSTALLINE_PDF_FIXED_EPOCH`).

---

## Verificação

### Confirmar se dois documentos diferentes produzem o mesmo `DocumentID`

```bash
cat > /tmp/p612-doc-a.typ <<'EOF'
Documento A.
EOF
cat > /tmp/p612-doc-b.typ <<'EOF'
Documento B, completamente diferente.
EOF
./target/release/typst /tmp/p612-doc-a.typ /tmp/p612-a.pdf
./target/release/typst /tmp/p612-doc-b.typ /tmp/p612-b.pdf

for f in /tmp/p612-a.pdf /tmp/p612-b.pdf; do
  echo "=== $f ==="
  python3 -c "
import re
data = open('$f', 'rb').read()
m = re.search(rb'DocumentID>([^<]+)<', data)
print('DocumentID:', m.group(1) if m else 'não encontrado')
"
done
```

Sem definir nenhuma variável de ambiente de teste. Se os dois documentos, com conteúdo completamente diferente, produzirem o mesmo `DocumentID`, o problema está confirmado.

### Localizar no código

```bash
grep -n "InstanceID\|DocumentID" 03_infra/src/export/builder.rs | head -20
```

Confirmar se o valor vem de uma constante fixa no código, ou se depende de `CRYSTALLINE_PDF_FIXED_EPOCH` (só fixo durante testes).

---

## Decisão

Se for confirmado que o valor é sempre fixo, mesmo em produção: corrigir para gerar um valor único por documento (por exemplo, um hash do conteúdo do documento, ou um UUID gerado no momento da compilação), reservando o valor fixo só para quando a variável de teste estiver activa — o mesmo padrão já estabelecido em P601 para as datas.

### Implementação, se necessário

```rust
// Esboço, a confirmar contra a estrutura real:
let document_id = if std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH").is_ok() {
    FIXED_DOCUMENT_ID.to_string()
} else {
    generate_unique_document_id(&doc)  // hash do conteúdo, ou UUID
};
```

---

## Validação

```bash
./target/release/typst /tmp/p612-doc-a.typ /tmp/p612-a-depois.pdf
./target/release/typst /tmp/p612-doc-b.typ /tmp/p612-b-depois.pdf
```

Confirmar que os dois documentos produzem `DocumentID` diferentes agora.

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar que os snapshots P307b continuam deterministas com a variável de teste activa.

---

## Critério de fecho do passo

- [ ] Confirmado se o problema existe (dois documentos diferentes com o mesmo `DocumentID`, sem variável de teste).
- [ ] Se confirmado: corrigido, com o valor fixo reservado só para testes, seguindo o padrão de P601.
- [ ] Documentos diferentes produzem `DocumentID` diferentes em produção.
- [ ] Snapshots de teste continuam deterministas.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p612.md`, com hash do commit.
