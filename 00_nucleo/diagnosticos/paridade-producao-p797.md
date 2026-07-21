# Relatório de Verificação — Passo 797: Resolução do Render de Negrito e Itálico em Fontes CFF (Libertinus)

**Data:** 2026-07-21  
**Status:** Concluído com Sucesso  
**Proveniência da Medição:**
- **Commit Base:** `619d80577` (HEAD)
- **Modificações na Working Tree:** (Clean working directory)
- **Hora da Medição:** 2026-07-21T10:30:00Z (UTC)

---

## 1. O Problema Relatado
O passo 797 apontava que o texto renderizado com variantes `Bold` ou `Italic` da fonte embutida *Libertinus Serif* não apresentava distinção visual no PDF exportado (o texto ficava invisível/em branco visualmente, embora perfeitamente selecionável no leitor PDF). O mapeamento de `FontBook` não era o culpado.

---

## 2. Diagnóstico e Medição
Através de instrumentação direta no PDF gerado (`pdftoppm`, `pdffonts`, e sondagem de bytes do programa CFF), constatou-se que o motor de exportação estava a gerar um PDF com objetos de fonte estruturalmente incorretos. A causa raiz era uma limitação nativa do subsetter (`oxifont_subset`):
- O subsetting de fontes CFF produzia um programa CFF do tipo **Name-keyed** (sem as estruturas CID-keyed `ROS` e `FDArray/FDSelect`).
- Este subset estava a ser embutido no PDF sob um descritor que declarava a fonte como `/CIDFontType0` com codificação `Identity-H` (que exige rigidamente um programa CID-keyed CFF).
- A incompatibilidade estrutural levava os leitores PDF (Ghostscript/Poppler) a reportarem `Syntax Error: Couldn't create a font` e recusarem-se a carregar o fluxo visual da fonte.

Adicionalmente, verificou-se um problema subjacente onde `ttf_parser::Face::parse` falhava em reconhecer o arquivo OpenType gerado por manter o *scalerType* de `00010000` (TrueType) em vez de `OTTO` (OpenType).

---

## 3. A Solução Implementada
Para restabelecer o funcionamento visual sem introduzir instabilidade na biblioteca de subsetting (que escapa ao escopo da *typst-crystalline*):

1. **Desativação de Subsetting para CFF1**: Em `03_infra/src/export/subset.rs`, inserimos uma cláusula que retorna `None` precocemente se a fonte for detetada como CFF1. O exportador no `builder.rs` faz então um *fallback* seguro e documentado para o embutimento integral da fonte. Como a fonte integral é CID-keyed e intocada, o mapeamento CID→GID estrito da tabela PDF `Identity-H` funciona sem erros.
2. **Correção de Subtipo PDF para CFF**: Em `03_infra/src/export/builder.rs`, `font_embedding_data` foi modificado. Em vez de declarar `/CIDFontType0C` (CFF puro isolado), passa a usar o container SFNT completo via `/OpenType`. Esta semântica alinha-se com a diretriz do ISO 32000-2 (PDF 1.7) §9.9.4 e dissolve a fricção da extração isolada.
3. **Detecção CFF Resiliente**: Adicionada a função `has_cff_or_cff2_table` que lê as *table tags* de SFNT de forma direta (Offset 12), prescindindo de `Face::parse`, mitigando erros oriundos de *scalerTypes* corrompidos pelo processo de subsetting.

---

## 4. Testes Automatizados Persistidos (com nomeação explícita)
Atendendo explicitamente ao rigor de cobertura justificada, os testes automatizados afetados foram nomeados e alinhados para refletir a semântica da correção:

- `p797_probe_select_pattern_by_variant` (Novo em `03_infra/src/embedded_fonts.rs`): Prova afirmativamente que `select_pattern` resolve `Regular`, `Bold` e `Italic` para índices distintos, salvaguardando contra regressões silenciosas no *shaping*.
- `p523_subset_cff_nimbus_sans_preserves_cff_table` (Alterado em `03_infra/src/export/subset.rs`): Validada a nova regra: o teste agora assevera deliberadamente que `subset_font_with_mapping` *tem* de devolver `None` perante CFF1, comprovando que o subsetting recua de forma previsível e segura.
- `p560_fonte_cff_usa_cidfont_type0` (Alterado em `03_infra/src/export/tests.rs`): Validado para assegurar que a infraestrutura declara e prefere emitir `/OpenType`, banindo assertivamente o anterior `/CIDFontType0C`.

---

## 5. Verificação de Sucesso do Workspace
Todos os testes unitários e de integração foram executados com sucesso (zero falhas).

```
1. Suite 'typst-core' (lib):
   test result: ok. 4317 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.49s

2. Suite 'typst-infra' (lib):
   test result: ok. 656 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 1.41s

3. Suite 'typst-shell' (lib):
   test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

4. Suite 'typst' (CLI bin):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

5. Suite 'tests/cli.rs' (CLI integration):
   test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s

6. Suite 'tests/crystalline_lint.rs' (Linter rules):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`crystalline-lint .` reportou zero violações relevantes ao passo (apenas prompts L0 antigos não referenciados). A renderização final em Ghostscript/Poppler passou a ter sucesso visual, restabelecendo a distinção `Bold/Italic` para Libertinus Serif.
