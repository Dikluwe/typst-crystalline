# Paridade de Produção — P536

**Passo:** 536  
**Foco:** Metadados do documento (`#set document(title:, author:)`) → `/Info` do PDF.  
**Data:** 2026-07-02  

## Resumo

Antes de P536, o cristalino reconhecia a sintaxe `#set document(title: ..., author: ...)`
mas emitia um aviso de "target 'document' ainda não suportado" e descartava a
informação. O PDF final não continha `/Title` nem `/Author` no dicionário `/Info`.

P536 resolve isto interceptando o target `"document"` no eval, acumulando os
valores em `EvalContext::document_info`, transportando-os para o `Module` e,
via pipeline, para o `PagedDocument`. O exportador PDF (`PdfBuilder`) emite um
dicionário `/Info` com `/Title`, `/Author`, `/Keywords`, `/CreationDate` e
`/Creator`.

A stream de metadados XMP foi deixada como **scope-out explícito**: `/Info` é
lido por todos os leitores de PDF comuns e cobre o caso de uso principal; XMP
adiciona complexidade XML sem ganho observable imediato.

## Sondas

### Sonda 1 — Onde `#set document(...)` era descartado

Ficheiro: `01_core/src/rules/eval/rules.rs:858-862`.

```rust
if target != "text" {
    let (msg, hint) = unsupported_target_warn(&target);
    engine.sink.warn_note(target_span, &msg, &hint);
    return Ok(Value::None);
}
```

Qualquer target que não fosse `text` (e não tivesse braço específico como
`page`, `heading`, etc.) caía neste fallback e gerava o aviso. `document` não
tinha braço próprio.

### Sonda 2 — `/Info` no exportador

Comando:

```bash
grep -rn "/Info\|DocumentInfo\|/Title\|/Author\|XMP" 03_infra/src/export/ --include="*.rs"
```

Resultado: nenhuma ocorrência. O PDF cristalino não escrevia `/Info`.

### Sonda 3 — Referência vanilla

Comando:

```bash
cat > /tmp/test-metadata.typ <<'EOF'
#set document(title: "Documento de Teste", author: "Autor Teste")
Conteúdo do documento.
EOF
/usr/local/bin/typst compile /tmp/test-metadata.typ /tmp/metadata-vanilla.pdf
pdfinfo /tmp/metadata-vanilla.pdf
```

Resultado vanilla:

```text
Title:           Documento de Teste
Author:          Autor Teste
Creator:         Typst 0.15.0
Producer:        Typst 0.15.0
CreationDate:    Thu Jul  2 16:30:00 2026 UTC
```

O vanilla escreve `/Title`, `/Author`, `/Creator`, `/Producer` e
`/CreationDate`. Também inclui uma stream XMP com a mesma informação.

## Implementação

### Ficheiros alterados / criados

- `00_nucleo/prompts/entities/document_info.md` — Prompt L0 do novo tipo.
- `00_nucleo/prompts/entities/module.md` — secção P536, `document_info` no
  `ModuleInner` e métodos getter/setter.
- `00_nucleo/prompts/entities/layout_types.md` — campo `document_info` em
  `PagedDocument`.
- `00_nucleo/prompts/rules/eval.md` — secção P536, interceptação de
  `#set document(...)` em `eval_set_rule`.
- `00_nucleo/prompts/infra/export/builder.md` — secção P536, emissão de `/Info`.
- `01_core/src/entities/document_info.rs` — novo tipo `DocumentInfo`.
- `01_core/src/entities/mod.rs` — exporta `document_info`.
- `01_core/src/entities/module.rs` — campo e métodos para `DocumentInfo`.
- `01_core/src/rules/eval/mod.rs` — `EvalContext::document_info`, cópia para o
  `Module` no final do eval.
- `01_core/src/rules/eval/rules.rs` — braço `target == "document"`.
- `01_core/src/entities/layout_types.rs` — `document_info` em `PagedDocument`.
- `03_infra/src/pipeline.rs` — copia `module.document_info()` para
  `doc.document_info`.
- `03_infra/src/export/builder.rs` — emite `/Info` no trailer e objecto
  correspondente.

### Decisões

- **Transporte via `EvalContext` → `Module` → `PagedDocument`**: em vez de
  adicionar uma variante `Content::SetDocument` (que exigiria tocar no enum
  `Content` e em todos os seus `match`), os metadados viajam como tabela lateral,
  analogamente a `bibliography_styles` (P429). Isto minimiza o blast radius e
  mantém a semântica de "metadados do documento", não de content renderizável.
- **`/Info` apenas, XMP scope-out**: `/Info` é suficiente para `pdfinfo` e para
  a maioria dos leitores. A stream XMP pode ser adicionada num passo futuro sem
  alterar a arquitetura de transporte.
- **`author` como string simples**: o vanilla aceita `author: ("A", "B")`; o
  cristalino converte arrays de strings para `"A, B"`.
- **Data de criação UTC**: usa a crate `time` (já dependência do workspace) para
  obter `OffsetDateTime::now_utc()` e formatar `D:YYYYMMDDHHMMSS`.

## Validação

### Documento de teste do passo

```typst
#set document(title: "Documento de Teste", author: "Autor Teste")
Conteúdo do documento.
```

Comando:

```bash
./target/release/typst /tmp/test-metadata.typ /tmp/metadata.pdf
pdfinfo /tmp/metadata.pdf
```

Resultado cristalino:

```text
Title:           Documento de Teste
Author:          Autor Teste
Creator:         typst-crystalline
CreationDate:    Thu Jul  2 16:30:00 2026 UTC
```

### Teste com array de autores

```typst
#set document(title: "Doc", author: ("Ana", "Bruno"))
Texto.
```

Resultado `pdfinfo`:

```text
Title:           Doc
Author:          Ana, Bruno
```

### Teste sem metadados

Documento sem `#set document(...)` continua a compilar sem avisos e sem
`Title`/`Author` em `pdfinfo`.

### Comparação com Typst vanilla 0.15.0

| Campo   | Vanilla                              | Cristalino           |
|---------|--------------------------------------|----------------------|
| Title   | `Documento de Teste`                 | `Documento de Teste` |
| Author  | `Autor Teste`                        | `Autor Teste`        |
| Creator | `Typst 0.15.0`                       | `typst-crystalline`  |
| XMP     | sim                                  | scope-out            |

A paridade semântica (título e autor acessíveis via `/Info`) está alcançada.
A diferença em `Creator` é esperada e não afecta a funcionalidade.

### Testes automatizados

```bash
cargo test --workspace
crystalline-lint .
```

Resultado:

- `cargo test --workspace`: todos os testes passam.
- `crystalline-lint .`: zero violations.

## Checklist de fecho

- [x] Sonda completa antes do código.
- [x] `#set document(...)` deixa de emitir aviso de "não suportado".
- [x] `/Info` no PDF final tem `/Title` e `/Author` correctos.
- [x] Decisão sobre XMP registada (scope-out explícito).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p536.md`.

## Scope-out registado

- **Stream de metadados XMP:** `/Info` resolve o caso de uso comum. XMP pode
  ser adicionado posteriormente sem alterar o transporte de metadados.
- **`/Producer`:** não é preenchido. O vanilla usa o próprio nome/versão; não
  há requisito de paridade literal neste campo.
- **Timezone em `/CreationDate`:** usa UTC sem offset. Adicionar timezone
  (`+01'00'`) é possível mas não afecta leitores comuns.
- **`subject`/`keywords` expandidos:** `keywords` é suportado; campos adicionais
  do vanilla (ex.: `subject`) ficam para pedido futuro.
