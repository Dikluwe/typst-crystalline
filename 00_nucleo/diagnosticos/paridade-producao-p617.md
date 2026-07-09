# Paridade de produção — P617: Bandeira opcional para `DocumentID` externo

**Data:** 2026-07-08  
**Hash do commit de fecho:** `d1ad7e540`  
**Materialization:** `00_nucleo/materialization/typst-passo-617.md`

---

## 1. Contexto

Em P615, o cristalino reverteu `DocumentID`/`InstanceID` para valores
aleatórios por compilação, alinhado com o vanilla 0.15.0. Esse
comportamento é correcto por defeito, mas não serve utilizadores ou
ferramentas externas que precisam de manter a mesma identidade de
documento entre compilações sucessivas (por exemplo, sistemas de gestão
documental).

P617 introduz uma bandeira explícita para fornecer um `DocumentID`
externo, sem tornar o compilador interactivo e sem alterar o
comportamento por defeito.

## 2. Sonda

### 2.1 Existe mecanismo equivalente no vanilla?

```bash
lab/typst-original/target/release/typst compile --help | grep -iE 'id|metadata|document'
```

Resultado: nenhuma flag relacionada com identificadores de documento.
Esta é uma **capacidade nova do cristalino**, não paridade com o vanilla.

### 2.2 Forma de entrada decidida

- **Bandeira de linha de comandos:** `--document-id <UUID>`.
- **Variável de ambiente:** `CRYSTALLINE_DOCUMENT_ID=<UUID>`.
- **Prioridade:** `--document-id` > `CRYSTALLINE_DOCUMENT_ID` > ausente
  (comportamento por defeito aleatório de P615).

## 3. Decisão

Aceitar um UUID externo de 16 bytes e usá-lo como `xmpMM:DocumentID` no
pacote XMP. O `xmpMM:InstanceID` continua sempre aleatório (ou fixo em
ambiente de testes), porque cada compilação é uma instância diferente do
documento.

## 4. Implementação

### 4.1 Ficheiros alterados

- `00_nucleo/prompts/shell/cli.md`:
  - Documentada a flag `--document-id` e a env var
    `CRYSTALLINE_DOCUMENT_ID`.
  - Adicionado campo `RunIntent.document_id: Option<[u8; 16]>`.
- `02_shell/src/cli.rs`:
  - Adicionado argumento `document_id: Option<String>`.
  - Implementado `parse_uuid_bytes` para converter UUID textual nos 16
    bytes correspondentes (aceita formato canónico e 32 hex sem hífenes).
  - Validação em `parse()`: UUID inválido termina com mensagem clara e
    exit 2.
  - Testes unitários para parsing válido e inválido.
- `00_nucleo/prompts/wiring.md`:
  - Actualizado uso e pipeline para reflectir `document_id`.
- `04_wiring/src/main.rs`:
  - Consome `document_id` do `RunIntent` e passa-o às novas variantes do
    pipeline.
- `00_nucleo/prompts/infra/pipeline.md`:
  - Documentadas as variantes `_with_document_id` das funções de
    compilação.
- `03_infra/src/pipeline.rs`:
  - Adicionadas `compile_to_pdf_bytes_full_error_and_document_id` e
    `compile_to_pdf_bytes_with_timings_full_error_and_document_id`.
  - `compile_to_pdf_bytes_impl` recebe `document_id: Option<[u8; 16]>` e
    propaga-o ao exportador.
- `03_infra/src/export/mod.rs`:
  - Adicionadas variantes `_with_document_id` de `export_pdf`,
    `export_pdf_with_font`, `export_pdf_with_font_and_timings`,
    `export_pdf_multifont` e `export_pdf_multifont_and_timings`.
  - As funções originais permanecem como wrappers que passam `None`,
    preservando a API existente.
- `00_nucleo/prompts/infra/export/builder.md`:
  - Actualizada secção §P611 para descrever o `DocumentID` externo.
- `03_infra/src/export/builder.rs`:
  - Adicionado campo `document_id` a `PdfBuilder` e método
    `with_document_id`.
  - `xmp_instance_and_document_id` aceita `external_id` e usa-o quando
    presente; `InstanceID` mantém a lógica de P615.
- `04_wiring/tests/cli.rs`:
  - Teste de integração que confirma `DocumentID` fixo e `InstanceID`
    diferente entre duas compilações com o mesmo `--document-id`.
  - Teste que confirma `DocumentID` aleatório sem a bandeira.
  - Teste que confirma erro claro para UUID inválido.

### 4.2 Design

- **Sem dependências novas:** o parsing de UUID é feito manualmente com
  `u8::from_str_radix`, evitando adicionar crates.
- **Sem alteração da API pública existente:** as funções antigas de
  export e pipeline mantêm as mesmas assinaturas; as novas variantes
  têm sufixo `_with_document_id`.
- **Comportamento por defeito inalterado:** quando `document_id` é
  `None`, o fluxo é idêntico a P615.

## 5. Validação

### 5.1 Testes automáticos

```bash
cargo test --workspace
```

Resultado: todos os testes passam, incluindo os novos testes de CLI e
os testes de parsing de UUID.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 5.2 Verificação manual

```bash
cat > /tmp/p617-teste.typ <<'EOF'
Texto.
EOF
./target/release/typst compile --document-id "f81d4fae-7dec-11d0-a765-00a0c91e6bf6" /tmp/p617-teste.typ /tmp/p617-1.pdf
./target/release/typst compile --document-id "f81d4fae-7dec-11d0-a765-00a0c91e6bf6" /tmp/p617-teste.typ /tmp/p617-2.pdf
./target/release/typst compile /tmp/p617-teste.typ /tmp/p617-3.pdf
```

Os dois primeiros PDFs partilham o mesmo `DocumentID` (base64 do UUID
fornecido) e têm `InstanceID` diferentes. O terceiro PDF tem `DocumentID`
aleatório, distinto dos anteriores.

## 6. Conclusão

P617 está concluído. O cristalino passa a suportar `--document-id`
`<UUID>` (e `CRYSTALLINE_DOCUMENT_ID`) para fixar o `DocumentID` entre
compilações, sem afectar o comportamento por defeito e sem paridade com
o vanilla — trata-se de uma capacidade nova. `InstanceID` continua
sempre aleatório, preservando a semântica de "cada compilação é uma
instância diferente".

---

## 7. Ligações

- Commit de fecho: `d1ad7e540`
- Prompts L0:
  - `00_nucleo/prompts/shell/cli.md`
  - `00_nucleo/prompts/wiring.md`
  - `00_nucleo/prompts/infra/pipeline.md`
  - `00_nucleo/prompts/infra/export/builder.md`
- Materialization: `00_nucleo/materialization/typst-passo-617.md`
