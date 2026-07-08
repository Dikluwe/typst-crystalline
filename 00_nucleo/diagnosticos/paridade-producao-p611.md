# Paridade de produção — P611: Stream de metadados XMP

**Data:** 2026-07-08  
**Hash do commit de fecho:** `a59acf07a`  
**Materialization:** `00_nucleo/materialization/typst-passo-611.md`  

---

## 1. Sonda

### 1.1 Documento de teste

```typst
#set document(title: "Documento de Teste", author: "Autor Teste", keywords: ("chave1", "chave2"))
Texto.
```

### 1.2 Conteúdo XMP do vanilla 0.15.0

```xml
<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?><x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="xmp-writer"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"  xmlns:xmp="http://ns.adobe.com/xap/1.0/"  xmlns:xmpMM="http://ns.adobe.com/xap/1.0/mm/"  xmlns:xmpTPg="http://ns.adobe.com/xap/1.0/t/pg/"  xmlns:pdf="http://ns.adobe.com/pdf/1.3/" ><dc:title><rdf:Alt><rdf:li xml:lang="x-default">Documento de Teste</rdf:li></rdf:Alt></dc:title><pdf:Keywords>chave1, chave2</pdf:Keywords><dc:creator><rdf:Seq><rdf:li>Autor Teste</rdf:li></rdf:Seq></dc:creator><xmp:CreatorTool>Typst 0.15.0</xmp:CreatorTool><dc:language><rdf:Bag><rdf:li>en</rdf:li></rdf:Bag></dc:language><xmp:ModifyDate>2026-07-08T08:39:18-03:00</xmp:ModifyDate><xmp:CreateDate>2026-07-08T08:39:18-03:00</xmp:CreateDate><xmpTPg:NPages>1</xmpTPg:NPages><dc:format>application/pdf</dc:format><xmpMM:InstanceID>q7a5Evax/4zCi9aVbPFo9w==</xmpMM:InstanceID><xmpMM:DocumentID>1BVIT9GyOwq6nXK/mnf3Og==</xmpMM:DocumentID><xmpMM:RenditionClass>proof</xmpMM:RenditionClass><pdf:PDFVersion>1.7</pdf:PDFVersion></rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end="r"?>
```

### 1.3 Vanilla sem metadados de utilizador

Documento:

```typst
Texto sem metadados.
```

O vanilla 0.15.0 também emite XMP, omitindo apenas os campos `dc:title`, `pdf:Keywords` e `dc:creator`:

```xml
<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?><x:xmpmeta ...><rdf:Description ...><xmp:CreatorTool>Typst 0.15.0</xmp:CreatorTool><dc:language><rdf:Bag><rdf:li>en</rdf:li></rdf:Bag></dc:language><xmp:ModifyDate>...</xmp:ModifyDate><xmp:CreateDate>...</xmp:CreateDate><xmpTPg:NPages>1</xmpTPg:NPages><dc:format>application/pdf</dc:format><xmpMM:InstanceID>...</xmpMM:InstanceID><xmpMM:DocumentID>...</xmpMM:DocumentID><xmpMM:RenditionClass>proof</xmpMM:RenditionClass><pdf:PDFVersion>1.7</pdf:PDFVersion></rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end="r"?>
```

**Conclusão da sonda:** o XMP é **sempre emitido**, independentemente de haver metadados de utilizador — a mesma regra já estabelecida para `/Info` em P601.

### 1.4 Estado anterior do cristalino

Antes de P611, o PDF gerado pelo cristalino não continha `<?xpacket` nem referência `/Metadata` no catálogo.

---

## 2. Implementação

### 2.1 Ficheiros alterados

- `03_infra/src/export/builder.rs` — implementação do stream XMP.
- `00_nucleo/prompts/infra/export/builder.md` — secção §P611 adicionada ao Prompt L0.
- `03_infra/fixtures/p307b/reference/*.pdf` — snapshots regenerados (crescimento esperado devido ao stream XMP).
- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` — XMP movido de scope-out para corrigido.
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` — XMP marcado como fechado em P611.

### 2.2 Decisões tomadas

| Aspecto | Decisão | Base |
|---------|---------|------|
| Emitir sempre? | Sim | Sonda do vanilla: XMP presente mesmo sem metadados. |
| Estrutura XML | Copiada do vanilla (namespaces, ordem de campos) | Paridade morfológica com o vanilla. |
| `x:xmptk` | `xmp-writer` | Igual ao vanilla (detalhe mecânico). |
| `CreatorTool` | `typst-crystalline` | Divergência de branding permitida (a paridade é com a linguagem, não com o literal do vanilla). |
| `dc:language` | `en` fixo | O cristalino ainda não propaga a linguagem do documento para `PagedDocument`. |
| Datas | UTC (`Z`) via `time::OffsetDateTime::now_utc` | Alinhado com `/Info` (P601) e determinismo em testes via `CRYSTALLINE_PDF_FIXED_EPOCH`. |
| IDs (`InstanceID`, `DocumentID`) | Base64 de 16 bytes, valores fixos | Determinismo; `dHlwc3QtY3J5c3QtaW5zdA==` e `dHlwc3QtY3J5c3QtZG9jdQ==`. |
| `NPages` | Número real de páginas de `PagedDocument` | Paridade semântica. |
| Escape de caracteres | Entidades XML (`&amp;`, `&lt;`, etc.) | Reaproveitamento da disciplina de P538b, adaptado a XML. |
| Alocação de ID | Após todos os outros objectos, incluindo `/Info` | O stream é referenciado por `/Metadata` no catálogo. |

### 2.3 Código

A função `PdfBuilder::emit_xmp_metadata` foi adicionada a `03_infra/src/export/builder.rs` e é chamada após `emit_info` nos três caminhos de export (`build_helvetica`, `build_cidfont`, `build_multifont`).

---

## 3. Validação

### 3.1 Testes manuais

Documento com metadados:

```bash
./target/release/typst /tmp/p611-teste.typ /tmp/p611-depois.pdf
python3 -c "
data = open('/tmp/p611-depois.pdf', 'rb').read()
print('Tem xpacket:', b'<?xpacket' in data)
print('Tem /Metadata:', b'/Metadata' in data)
"
```

Resultado:

```text
Tem xpacket: True
Tem /Metadata: True
```

Documento sem metadados:

```bash
./target/release/typst /tmp/p611-sem-metadados.typ /tmp/p611-depois-sem.pdf
```

Resultado: XMP emitido sem `dc:title`, `pdf:Keywords` nem `dc:creator`.

Documento com acentos:

```typst
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
```

Resultado: carateres acentuados preservados no XML (`Relatório de José`, `João Conceição`).

### 3.2 Testes automáticos

```bash
cargo test --workspace
```

Resultado: todos os testes passam (604 passed; 9 snapshots P307b regenerados).

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 4. Diferenças em relação ao vanilla

| Item | Vanilla 0.15.0 | Cristalino após P611 | Nota |
|------|----------------|----------------------|------|
| `CreatorTool` | `Typst 0.15.0` | `typst-crystalline` | Divergência de branding intencional. |
| Fuso horário das datas | Offset local (`-03:00`) | UTC (`Z`) | Alinhado com `/Info`; ambos válidos. |
| Espaçamento entre atributos `xmlns:*` | Dois espaços | Um espaço | Detalhe mecânico de serialização; sem impacto semântico. |
| IDs | Aleatórios / baseados em hash estável | Fixos | Determinismo em testes. |

Nenhuma diferença afecta a leitura do XMP por ferramentas standard.

---

## 5. Conclusão

P611 está concluído. O cristalino passa a emitir um stream de metadados XMP equivalente ao do vanilla 0.15.0: mesma estrutura de namespaces e campos, sempre presente, correctamente referenciado no catálogo do PDF, e com tratamento de carateres especiais. A disparidade foi removida das listas de estado.

---

## 6. Ligações

- Commit de fecho: `a59acf07a`
- Prompt L0: `00_nucleo/prompts/infra/export/builder.md`
- Materialization: `00_nucleo/materialization/typst-passo-611.md`
