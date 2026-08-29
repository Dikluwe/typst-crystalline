# P1247 — fronteiras de links internos SVG

**Veredito:** `L0_WRITTEN_ARCHITECTURE_APPROVED_AWAITING_SEGREGATED_PRESEAL`.

## Medição antes da decisão

A aresta interna não está perdida: `FrameItem::Link` transporta
`LinkTarget::Destination(Label)` até L3. O exporter distingue essa variante,
mas a deixa em scope-out. O layout final também produz
`PagedDocument.extracted_label_pages` e `extracted_label_positions`.

A API `export_svg(page, ...)` recebe somente uma `Page`; não recebe o registro
de destinos do documento. Assim, não consegue emitir um nó `id` na posição
correta nem provar que um fragmento resolve. Gerar apenas `href="#label"`
criaria uma aresta potencialmente pendente.

Existe um segundo gap independente: o construtor público `link()` aceita URL
string, mas não label, location ou dicionário page/x/y. Embora referências
internas já produzam `Destination(Label)`, a superfície completa da linguagem
não está representada.

## Opções

A matriz em `p1247-link-options.tsv` compara cinco opções. O dono aprovou em
2026-08-28 injetar em L3 um contexto imutável de destinos derivado do
`PagedDocument`:

- fecha links label já existentes dentro da página;
- não duplica o registro como novo `FrameItem`;
- deixa roteamento cross-page sob política explícita do wiring/bundle;
- não mistura automaticamente a expansão pública de `link()`.

Expandir `link()` para label/location/page-position é uma segunda decisão
ADR-0127 e exige owners L0 próprios. Sintetizar fragmentos sem nós é rejeitado.

## Decisão e L0

O invariant compartilhado está no Núcleo Tekt
`prompts/_nuclei/export/svg-destination-context.toml`, pinado pelos dois prompts
proprietários. `infra/export/svg` define o contexto page-local, os wrappers
compatíveis e o grafo fechado `href ↔ id`; `infra/pipeline` deriva o contexto
dos mapas do `PagedDocument` e o entrega ao exporter. Política de bundle e rota
cross-page permanece no caller de composição.

## Execução

O auditor validou oito fronteiras e cinco opções duas vezes com saída
byte-idêntica. Resultado: uma aresta interna representada, zero grafos SVG
internos fechados, zero oráculos e zero mutações executadas. Não há mutation
score, contrato selado ou alteração produtiva.

O runner confirmou a aprovação e os pins L0. Nenhum código foi escrito. Próximo
gate: contrato, oráculos e ataques segregados, seguidos de preseal válido antes
da implementação RED→GREEN.
