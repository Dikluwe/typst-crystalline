# Paridade de Produção — P601

**Data do relatório:** 2026-07-07
**Passo:** 601
**Foco:** Emitir `/Info` sempre, mesmo em documentos sem metadados do utilizador.

---

## Resumo executivo

O cristalino só emitia o dicionário `/Info` quando o documento tinha pelo menos um de `title`, `author` ou `keywords`. O vanilla 0.15.0 emite `/Info` sempre, com `/Creator`, `/CreationDate` e `/ModDate`, independentemente de metadados.

Este passo alterou `emit_info` em `03_infra/src/export/builder.rs` para:

- Sempre construir e emitir o dicionário `/Info`.
- Adicionar `/ModDate` (o vanilla também o inclui).
- Manter `/Title`, `/Author` e `/Keywords` opcionais (só aparecem se definidos).

Como `/CreationDate` e `/ModDate` mudam a cada segundo, os snapshots binários P307b tornaram-se não-determinísticos. Para preservar a comparabilidade byte-a-byte dos snapshots, o teste `p307b_snapshot` passou a definir `CRYSTALLINE_PDF_FIXED_EPOCH=0`, congelando as datas num valor fixo apenas durante a geração/comparação dos snapshots. Os 9 snapshots P307b foram regenerados.

---

## Proveniência

- **Hash base:** `45ae11b8c44414b5778ad4288453f56435acac15`
- **Data/hora:** 2026-07-07T20:59:06-03:00
- **Binário usado:** `./target/release/typst` (crate `typst-wiring`, release)
- **Ferramentas auxiliares:** `mutool show`, `pdfinfo`

---

## Implementação

### `03_infra/src/export/builder.rs`

A função `emit_info` foi alterada:

- Removida a condição `if doc.document_info.is_empty() { return; }`.
- Removida a condição `if parts.is_empty() { return; }`.
- `/CreationDate` e `/ModDate` são sempre adicionados.
- `/Creator (typst-crystalline)` é sempre adicionado.
- Campos de utilizador (`/Title`, `/Author`, `/Keywords`) continuam condicionais.
- Adicionado suporte à variável de ambiente `CRYSTALLINE_PDF_FIXED_EPOCH` para permitir testes determinísticos (usada pelos snapshots P307b).

### `00_nucleo/prompts/infra/export/builder.md`

Secção §P536 actualizada para reflectir que `/Info` é sempre emitido e inclui `/ModDate`.

### `03_infra/src/p307b_snapshot_tests.rs`

`compile_fixture` define `CRYSTALLINE_PDF_FIXED_EPOCH=0` para manter os snapshots determinísticos face às datas no `/Info`.

### Snapshots P307b

Todos os 9 snapshots foram regenerados com `UPDATE_P307B_SNAPSHOTS=1`:

- `01-markup-plain.pdf`
- `02-markup-heading.pdf`
- `03-text-styling.pdf`
- `04-shapes.pdf`
- `05-gradient-linear.pdf`
- `06-gradient-conic.pdf`
- `07-multi-feature.pdf`
- `08-image-jpeg.pdf`
- `09-cidfont.pdf`

---

## Validação

### Documento sem metadados

```typst
Texto sem metadados.
```

Objecto `/Info` no cristalino:

```text
10 0 obj
<<
  /CreationDate (D:20260707235510)
  /ModDate (D:20260707235510)
  /Creator (typst-crystalline)
>>
endobj
```

O trailer referencia `/Info` mesmo sem metadados do utilizador.

### Documento com metadados

```typst
#set document(title: "Título", author: "Autor")
Texto com metadados.
```

Objecto `/Info`:

```text
10 0 obj
<<
  /Title <FEFF005400ED00740075006C006F>
  /Author <FEFF004100750074006F0072>
  /CreationDate (D:20260707235511)
  /ModDate (D:20260707235511)
  /Creator (typst-crystalline)
>>
endobj
```

### `cargo test --workspace`

Todos os crates passaram:

| Crate | Resultado |
|-------|-----------|
| `typst-core` | 3581 passed; 0 failed |
| `typst-infra` | 598 passed; 0 failed; 5 ignored |
| `typst-shell` | 24 passed; 0 failed |
| `typst-wiring` | 21 + 2 passed; 0 failed |
| Doc-tests | 0 failed; 3 ignored |

### Corpus geral

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1
 done
```

Resultado: 37 documentos compilados, 0 falhas.

### `crystalline-lint .`

```text
✓ No violations found
```

---

## Decisão

A disparidade foi corrigida: o cristalino agora emite `/Info` em todos os PDFs, tal como o vanilla. `/ModDate` foi adicionado para alinhar com o vanilla.

A variável `CRYSTALLINE_PDF_FIXED_EPOCH` é uma medida de teste interna; não afecta o comportamento normal do compilador, onde as datas continuam a refletir o momento real da compilação.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/paridade-producao-p600.md` — observação sobre `/Info` só ser emitido com metadados fica desactualizada; P601 corrige-a.

Não havia item específico para esta disparidade nas listas consolidadas, uma vez que só foi identificada como observação lateral em P600.

---

## Critérios de fecho do passo

- [x] `/Info` presente em documentos sem metadados, com `/Creator`, `/CreationDate` e `/ModDate`.
- [x] `/ModDate` adicionado.
- [x] Documentos com metadados sem regressão.
- [x] Corpus geral sem falhas novas.
- [x] Snapshots P307b regenerados e testes passando.
- [x] `cargo test --workspace` sem regressões.
- [x] `crystalline-lint .` limpo.
- [x] Relatório escrito com hash do commit e proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-601.md` — passo que originou esta implementação.
- `00_nucleo/diagnosticos/paridade-producao-p600.md` — onde a diferença foi observada.
- `03_infra/src/export/builder.rs` — implementação de `emit_info`.
- `00_nucleo/prompts/infra/export/builder.md` — Prompt L0 actualizado.
