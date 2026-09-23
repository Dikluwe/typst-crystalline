# Passo 1480 — PDF/UA-1: identificação XMP e preferência de título

**Estado:** MATERIALIZADO — identificação/apresentação PDF/UA-1 e validação
de título concluídas; P1481 continua responsável pela língua estrutural e
pelo conteúdo marcado.

## 0. Achado de integração — gate ADR-0127 (2026-09-22)

Medição independente do §1 (reproduzida bit-a-bit a partir deste ficheiro,
commit `90aad760b`) confirmou os números citados (vanilla 106/0; cristalino
100/6/12-checks) e as duas regras desta fatia (ISO 14289-1 §5 test 1;
§7.1 test 10). Ao seguir o item 3 do contrato ("medir vanilla com e sem
`document(title:)`"), a medição revelou que o vanilla **recusa compilar**
sob `--pdf-standard ua-1` quando falta `document(title:)`:

```
$ typst compile --pdf-standard ua-1 sem-titulo.typ x.pdf
error: PDF/UA-1 error: missing document title
 = hint: set the title with `set document(title: [...])`
```

Fonte: `krilla::configure::validate::ValidationError::NoDocumentTitle`,
convertido em diagnóstico por `typst-pdf/src/convert.rs:748-752` do vanilla
ratificado (`prefix` = `"{validators} error:"`,
`typst-pdf/src/convert.rs:535`). Cristalino hoje (`target/release/typst`,
mesmo commit) compila o mesmo ficheiro com exit 0, produzindo um PDF
não-conforme sem qualquer aviso — comportamento divergente confirmado por
medição direta, não inferido.

Isto excede o contrato original (que previa só dois observáveis aditivos
em `builder.rs`): é um caminho de erro novo, visível ao utilizador, para
uma flag já existente (`--pdf-standard ua-1`) — critério ADR-0127 (2)
mudança de comportamento por defeito da flag, e (4) quebra de
compatibilidade para quem hoje depende do sucesso silencioso. Gate
disparado; humano perguntado explicitamente em chat 2026-09-22: **incluir
a validação no escopo de P1480** (opção escolhida, com a alternativa de
scope-out explícito rejeitada). Registro desta confirmação **precede**
qualquer edição de L0/código, mesma disciplina de P1477 (`typst-passo-1477.md`).

Escopo agora cobre dois owners:
1. `infra/export/builder.md` ↔ `builder.rs` — os dois observáveis aditivos
   originais (XMP `pdfuaid`, `/ViewerPreferences /DisplayDocTitle`).
2. `infra/pipeline.md` ↔ `pipeline.rs` — nova validação: `PdfAccessibility::
   Ua1` selecionado + `doc.document_info.title.is_none()` ⇒ erro de
   compilação, mensagem e hint byte-idênticos ao vanilla medido acima.
   Inserida em `resolve_pdf_pages_tags_and_standards`/
   `compile_to_pdf_bytes_impl` (mesma família que já resolve `standards` e
   já devolve `Result<Vec<u8>, Vec<SourceDiagnostic>>` — não é mudança de
   assinatura pública de `export_pdf*`, que continua infalível).

**Baseline da medição:** 2026-09-22, commit
`90aad760ba472cffb70063baa684c461a240e447`, working tree limpa;
cristalino `target/release/typst` SHA-256
`b70537dbcfa4184d900892623c440c01b92ae5bfee32b752ea9f7921eaf64fba`;
vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## 1. Medição anterior à decisão

Compilar bilateralmente com `--pdf-standard ua-1` esta fixture (SHA-256
`93b3d9d54a454cf7d3f86ca780f3d30244759125ea4cb90e95ae300142f547d1`):

```typst
#set document(title: "PDF/UA parity audit", author: "typst-crystalline")
#set page(width: 12cm, height: 8cm)
#set text(lang: "en")

= Accessible heading

This is a short tagged paragraph with *emphasis*.
```

`veraPDF 1.30.2 -f ua1` mediu vanilla conforme (106 regras aprovadas,
zero falhas) e cristalino não conforme (100 aprovadas, seis regras e 12
checks falhados). Duas regras pertencem a esta fatia: falta a identificação
PDF/UA no XMP (ISO 14289-1, §5, test 1) e falta
`/ViewerPreferences /DisplayDocTitle true` (ISO 14289-1, §7.1, test 10).
As outras quatro regras pertencem a P1481. Estes números são de **uma**
fixture, não de todo o espaço PDF/UA.

Fonte causal: `03_infra/src/export/builder.rs:2924-3034` emite XMP e só
acrescenta `pdfaid` a partir do eixo archival; `builder.rs:3509-3559`
emite a árvore de tags, sem o preference referido. O L0 vigente
`00_nucleo/prompts/infra/export/builder.md` §P1469/§P1475 declara
PDF/UA estrutural como scope-out, não como funcionalidade já selada.

Classificação: identificação e preferência são observáveis do PDF público,
não igualdade de bytes/IDs de objetos. Inferência: os dois pontos de emissão
explicam estas duas regras; refutação: inserir os campos e a validação ainda
falhar nas mesmas regras.

## 2. Contrato da fatia

1. Antes do RED, medir no PDF vanilla a forma e o namespace XMP de
   `pdfuaid` e a posição semântica de `DisplayDocTitle`; registar `file:line`
   da fonte pinada e a inspeção estrutural do PDF. Não copiar whitespace nem
   a ordem de objetos.
2. Estender **somente quando `PdfStandards` inclui `Ua1`** o XMP com a
   identificação PDF/UA-1 correta e o catálogo com preferência de exibir o
   título do documento. Conservar XMP `pdfaid` quando PDF/A e PDF/UA são
   selecionados em conjunto; não tratar um eixo como substituto do outro.
3. Não inventar título se a fonte não o define. Medir vanilla com e sem
   `document(title:)` para separar regra de perfil de valor fornecido pelo
   autor. Não alterar o default dos PDFs não-UA.
4. Não declarar `ua-1` conforme ao fim desta fatia: P1481 é dependência
   explícita e as quatro regras restantes continuam RED esperado.

## 3. Sequência L0 e testes

Owner inicial: `00_nucleo/prompts/infra/export/builder.md` ↔
`03_infra/src/export/builder.rs`. Auditar `infra/export/mod.md` e os
prompts de teste se seus consumers mudarem; manter ownership 1:1.
Retificar o scope-out histórico de PDF/UA no L0 **antes** de qualquer código,
registrando que P1480 é parcial e que P1481 completa esta fixture. Se a
medição exigir novo contrato público, assinatura ou default, parar no gate
ADR-0127; emissão interna de paridade segue fluxo contínuo após L0.

RED unitário: perfil `ua-1` não contém ainda os dois observáveis; controle
`a-4f`, PDF sem standard e `a-2u+ua-1` (se a combinação for aceita pela
interseção de versões). GREEN: os dois observáveis aparecem
somente onde requeridos e veraPDF deixa de reportar **essas duas** regras na
fixture bilateral. Guardar relatório completo para provar que nenhuma regra
PDF/A antes aprovada regrediu. Depois, `cargo build`, testes afetados,
`crystalline-lint .`, V5/V15/V26 e `git diff --check`.

O relatório final deve registrar commit/tree ou lista exata de arquivos
alterados se working tree não commitada, hora quando houver builds sucessivos,
hashes dos dois binários e comandos/versão do validador. Não usar a string
`--version` como única prova da proveniência.

## 4. Execução final e proveniência (2026-09-23)

**Estado medido:** HEAD `e18fd39c4e32e417192f83a6a67dfcc26227af18`,
working tree não commitada, medição final às
`2026-09-23T13:43:43-03:00`. Ficheiros P1480 alterados nesse estado:

- `00_nucleo/materialization/typst-passo-1480.md`;
- `00_nucleo/prompts/infra/export/builder.md`;
- `00_nucleo/prompts/infra/export/tests/mod.md`;
- `00_nucleo/prompts/infra/export/tests/p1480_pdfua_identification_tests.md`;
- `03_infra/src/export/builder.rs`;
- `03_infra/src/export/tests/mod.rs`;
- `03_infra/src/export/tests/p1480_pdfua_identification_tests.rs`;
- `03_infra/src/pipeline.rs`.

Os ficheiros não rastreados `typst-passo-1481.md` até
`typst-passo-1488.md` já existiam na working tree e não foram alterados por
esta execução.

**RED→GREEN.** O quinto teste novo reproduziu a regressão combinada:
`cargo test -p typst-infra --lib p1480_pdfua_identification_tests` terminou
primeiro com 4 aprovados/1 falhado pela ausência de `pdfaExtension`; após a
declaração mínima do schema PDF/UA, terminou com 5/5. O conjunto afetado
`cargo test -p typst-infra --lib export::tests` terminou com 343/343.
`cargo build`, `crystalline-lint .`,
`crystalline-lint --checks v5,v15,v26 .` e `git diff --check` passaram.

**Proveniência bilateral.** Fixture com título SHA-256
`93b3d9d54a454cf7d3f86ca780f3d30244759125ea4cb90e95ae300142f547d1`;
cristalino release final SHA-256
`cdbd7da3465d846eab862c5c1299e8e390d480487d7a488cebf5b2ec19f469a4`;
vanilla ratificado `/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
O cristalino foi reconstruído com `cargo build --release -p typst-wiring`.

`qpdf 11.9.0 --check` não encontrou erros de sintaxe ou stream nem no UA-1
isolado nem no A2u+UA-1. `veraPDF 1.30.2 -f ua1` mediu o cristalino final
com 102 regras aprovadas, quatro falhadas, 172 checks aprovados e dez
falhados: são exatamente idioma do outline, conteúdo não marcado, idioma
dos metadados e idioma do texto, todos reservados a P1481. As duas regras
desta fatia desapareceram.

A medição de não-regressão `veraPDF 1.30.2 -f 2u` sobre A2u+UA-1 terminou
conforme, com 146 regras aprovadas/zero falhadas e 1106 checks
aprovados/zero falhados. Antes da retificação L0 e do segundo RED, o mesmo
cristalino media 144/2 por `pdfuaid:part` sem schema de extensão; o vanilla
combinado media 146/0. A declaração mínima restaurou a conformidade PDF/A
sem adicionar a extensão a UA-1 isolado ou PDF/A isolado.

Por fim, compilar a fixture sem `document(title:)` com
`--pdf-standard ua-1` terminou com exit 1 e o diagnóstico:

```text
error: PDF/UA-1 error: missing document title
 = hint: set the title with `set document(title: [...])`
```

A fatia P1480 está fechada sem declarar conformidade PDF/UA-1 completa;
as quatro regras restantes são o RED esperado de P1481.
