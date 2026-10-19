# Correção — PDF/UA-1 dentro de bundle

**Estado:** 2026-09-30T03:01:50Z, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. `initial.json`, `closure.json`, `git-diff-stat-{initial,final}.txt` e `git-status-{initial,final}.txt` registram proveniência e SHA-256 de fontes, prompts e binário. Binário release final SHA-256 `2b366c2b7349c196c38bca83e00293ff0e11bb7979a5537b7ab5116ee6bee971`.

## Causa medida

A sonda anterior em `../auditoria-composicao-construcao-2026-09-29/probe.json` mostrou o mesmo corpo PDF/UA-1 sem título rejeitado pelo PDF isolado, mas publicado pelo bundle. `pdf.rs:404-426` validava título, alt e caminhos de anexos duplicados; `bundle.rs:463-513` chamava o exporter sem esses gates. A regressão integrada em `red.log` falhou porque o bundle devolveu `one.pdf`.

Depois de compartilhar os gates, o primeiro controle com título ainda falhou (`green.log`): o bundle descartava os metadados do `Content::Document`, enquanto o PDF isolado transportava `module.document_info()` ao documento paginado. O RED com `title:` nomeado está em `red-metadata.log`. Esta segunda causa foi adicionada ao L0 antes da respectiva implementação.

## Mudança

O owner `pdf.rs` concentra as três validações pré-exportação num helper privado ao pipeline, preservando a ordem e os diagnósticos vigentes. O bundle chama esse helper depois de resolver o standard e antes de resolver fontes/exportar, abortando a coleção inteira ao encontrar erro. Cada entrada `Content::Document` agora projeta seu próprio `title`, `author` e `keywords` no `DocumentInfo` do PDF correspondente; o título de uma entrada não contamina outra. A data segue no carrier existente e não foi acrescentada ao `DocumentInfo` nesta fatia. Não houve nova API pública, flag, modo padrão ou mudança de fase; os três L0 owners foram atualizados primeiro e ressellados conforme `scope.json`/`scoped-seals.json` (fluxo contínuo ADR-0127 para correção de invariante PDF já vigente).

## Verificação

- RED causal: bundle sem título devolvia bytes PDF; GREEN: devolve `PDF/UA-1 error: missing document title` com o hint vigente, sem publicação parcial.
- Bundle com `#document("one.pdf", title: [Okay])[...]` compila PDF/UA-1, com `pdfuaid:part=1` e `<dc:title>` no XMP. Documento seguinte sem título falha, sem herdar o primeiro. Sem UA-1, título segue opcional.
- `probe-final.json` usa o binário release final: PDF isolado inválido exit 1; bundle inválido exit 1; bundle com título exit 0. Entradas e artefato PDF estão neste diretório.
- `cargo test -p typst-infra --lib`: 1139 passaram, zero falhas. Suíte integrada `pipeline::tests`: 117 passaram. `cargo build`, build release e `crystalline-lint .`: exit 0. Nenhum V5 no escopo, V15 ou V26; 171 avisos V5 históricos fora do escopo permanecem.

O hint de erro herdado do PDF isolado sugere `set document(title: [...])`; no bundle, o título por entrada é fornecido por `title:` no construtor `document`. Alterar o hint é uma decisão de superfície de diagnóstico separada e não foi incluída na correção do gate. A equivalência da seleção de fontes PDF isolado/bundle continua para a auditoria seguinte.
