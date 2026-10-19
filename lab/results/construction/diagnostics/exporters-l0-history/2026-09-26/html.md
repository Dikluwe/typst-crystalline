# Prompt L0 — export HTML: fachada
Hash do Código: 87bdc16f

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html.rs`
**Atualizado em:** 2026-09-26
**ADRs:** ADR-0107, ADR-0109, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Medição anterior à decisão

O snapshot corrente possui 1355 linhas em `03_infra/src/export/html.rs`.
Entrypoints, modo e resolvedor estão em :73-76 e :141-188; composição em
:190-248/:417-512; pretty em :250-415; inline em :518-610; tabela em
:631-704; encoding em :612-629/:706-735/:900-918; whitespace em :737-898;
tags em :14-68; testes em :78-139 e :920-1355. Há adaptações locais já
existentes a Content::Styled(styled), que devem ser preservadas.
SHA-256 do L0 anterior: 068194eebf6067f84a942bdfd59ef64abbad6c6747cacd177083b187810f3878.
A divisão é mecânica L3, sem mudança de API, default, compatibilidade ou
fase de pipeline; segue fluxo contínuo ADR-0127.

## Ownership da fachada

Possui HtmlSerializationMode (Crystalline/Vanilla), HtmlFrameResolver e
export_html, export_html_with_serialization,
export_html_with_serialization_and_pretty, export_html_with_resolved_frames
e HtmlSerializationMode::export_with_resolved_frames. Preserva assinaturas
e caminho público typst_infra::export::html. Os entrypoints delegam ao
encoder documental interno; default continua Crystalline/pretty=false.

HTML continua target semântico (ADR-0128), não conversão de PagedDocument.
O exporter não faz I/O nem chama World/layout: frames já resolvidos são
recebidos da pipeline pelo contrato existente. Não expandir o subconjunto
suportado nem alterar fallback de variante não suportada nesta atomização.

## Owners descendentes

- `03_infra/src/export/html/document.rs` → `00_nucleo/prompts/infra/export/html/document.md`.
- `03_infra/src/export/html/inline.rs` → `00_nucleo/prompts/infra/export/html/inline.md`.
- `03_infra/src/export/html/table.rs` → `00_nucleo/prompts/infra/export/html/table.md`.
- `03_infra/src/export/html/whitespace.rs` → `00_nucleo/prompts/infra/export/html/whitespace.md`.
- `03_infra/src/export/html/encoding.rs` → `00_nucleo/prompts/infra/export/html/encoding.md`.
- `03_infra/src/export/html/pretty.rs` → `00_nucleo/prompts/infra/export/html/pretty.md`.
- `03_infra/src/export/html/tags.rs` → `00_nucleo/prompts/infra/export/html/tags.md`.
- `03_infra/src/export/html/tests.rs` → `00_nucleo/prompts/infra/export/html/tests.md`.
- `03_infra/src/export/html/p1360_ab_tests.rs` → `00_nucleo/prompts/infra/export/html/p1360_ab_tests.md`.

Módulos são privados. Sem estado global, despacho dinâmico novo, mudança
de entidade ou aumento de visibilidade para pub(crate). Cada arquivo
possui owner próprio; regras históricas foram transferidas por
responsabilidade aos prompts descendentes, não compartilhadas 1:N.

## Verificação

Caracterizações podem nascer no módulo de testes inline antes da extração.
Gate estrutural RED→GREEN prova existência dos descendentes e fachada magra;
testes de comportamento já devem passar no snapshot anterior, sem inventar
mudança semântica para forçar RED num refactor. Build, testes integrados de
frame e lint validam consumidores e linhagem.
