# Paridade de produção — P612: DocumentID/InstanceID do XMP

**Data:** 2026-07-08  
**Hash do commit de fecho:** `d0c7b76c4`  
**Materialization:** `00_nucleo/materialization/typst-passo-612.md`  

---

## 1. Verificação do problema

### 1.1 Documentos de teste

```typst
// /tmp/p612-doc-a.typ
Documento A.
```

```typst
// /tmp/p612-doc-b.typ
Documento B, completamente diferente.
```

### 1.2 Resultado antes da correcção

```bash
./target/release/typst /tmp/p612-doc-a.typ /tmp/p612-a.pdf
./target/release/typst /tmp/p612-doc-b.typ /tmp/p612-b.pdf
```

```text
=== /tmp/p612-a.pdf ===
DocumentID: dHlwc3QtY3J5c3QtZG9jdQ==
=== /tmp/p612-b.pdf ===
DocumentID: dHlwc3QtY3J5c3QtZG9jdQ==
```

**Problema confirmado:** dois documentos com conteúdo completamente diferente produziam o mesmo `DocumentID`, anulando o propósito do campo.

### 1.3 Causa

Em P611, `xmpMM:InstanceID` e `xmpMM:DocumentID` eram constantes fixas no código, independentemente do modo de execução:

```rust
const INSTANCE_ID: &str = "dHlwc3QtY3J5c3QtaW5zdA==";
const DOCUMENT_ID: &str = "dHlwc3QtY3J5c3QtZG9jdQ==";
```

---

## 2. Decisão

Seguir o mesmo padrão de P601 (`CRYSTALLINE_PDF_FIXED_EPOCH`):

- **Em produção:** gerar IDs únicos por documento/compilação.
- **Em testes:** manter valores fixos para snapshots deterministas.

### 2.1 Estratégia de geração

| ID | Fonte |
|----|-------|
| `DocumentID` | Hash determinístico de 128 bits do conteúdo do documento: metadados (`title`, `author`, `keywords`), número de páginas, dimensões de cada página e texto plano dos `FrameItem`. |
| `InstanceID` | Hash de 128 bits de `DocumentID` + timestamp de compilação, para ser único por compilação. |

Ambos são apresentados em base64, como no vanilla.

---

## 3. Implementação

### 3.1 Ficheiros alterados

- `03_infra/src/export/builder.rs`:
  - `base64_encode_16` — codificação base64 de 16 bytes.
  - `document_fingerprint` — fingerprint determinístico do documento.
  - `collect_xmp_fingerprint_text` — recolha recursiva de texto dos `FrameItem`.
  - `xmp_instance_and_document_id` — geração dos IDs, com valores fixos em testes.
  - `emit_xmp_metadata` passa a usar `xmp_instance_and_document_id(doc)`.
- `00_nucleo/prompts/infra/export/builder.md` — regra 8 da secção §P611 actualizada para reflectir P612.

### 3.2 Modo teste

Quando `CRYSTALLINE_PDF_FIXED_EPOCH` está definida, os IDs são fixos:

```text
InstanceID: dHlwc3QtY3J5c3QtaW5zdA==
DocumentID: dHlwc3QtY3J5c3QtZG9jdQ==
```

Isto garante que os snapshots P307b e outros testes binários permaneçam deterministas.

---

## 4. Validação

### 4.1 Documentos diferentes em produção

```bash
./target/release/typst /tmp/p612-doc-a.typ /tmp/p612-a-depois.pdf
./target/release/typst /tmp/p612-doc-b.typ /tmp/p612-b-depois.pdf
```

```text
=== /tmp/p612-a-depois.pdf ===
InstanceID: SGxgLPNR3QxpvnaVmsQwCg==
DocumentID: af0FJw8TrQ8C3BvJwhvuPQ==
=== /tmp/p612-b-depois.pdf ===
InstanceID: 11i6ctUsnWY+ga8PTV5t7Q==
DocumentID: kHGzpOR3dWrTdd4KOpZa+Q==
```

**Resultado:** documentos diferentes produzem `DocumentID` e `InstanceID` distintos.

### 4.2 Testes automáticos

```bash
cargo test --workspace
```

Resultado: todos os testes passam; snapshots P307b permanecem deterministas com a variável de teste activa.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 5. Conclusão

P612 está concluído. O problema de `DocumentID`/`InstanceID` fixos em produção foi corrigido: os IDs agora são únicos por documento/compilação em ambiente normal, e mantêm valores fixos durante testes para preservar a determinização dos snapshots.

---

## 6. Ligações

- Commit de fecho: `d0c7b76c4`
- Prompt L0: `00_nucleo/prompts/infra/export/builder.md`
- Materialization: `00_nucleo/materialization/typst-passo-612.md`
